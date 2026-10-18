# Phase D plan (r3): the counter-cost root resumes only after its replacement subtree retires

PHASE_BASE = 4483f4be4f371e0a572ab951ba4187ae44fddb62 (branch `engine/cost-removed-counters`, PR phase-rs/phase#10039).
- This base is the merge of upstream 872c4e3 (#9728). This PR's protocol entries are 151 / wire 133 there.
- Every file:line below is cited at 4483f4b. That includes the `game_state.rs` and `resolution.rs` anchors: those files shifted non-uniformly against 2d20628, so no offset rule applies.
- Unchanged since 2d20628: `engine_replacement.rs`, `costs.rs`, `mana_abilities.rs`, `engine_resolution_choices.rs`, `engine_payment_choices.rs`, `cost_removed_counter_amount.rs`, and `effects/*` except `end_combat_phase.rs`, `end_phase.rs`, `end_the_turn.rs` and `perpetual.rs` (none of which this plan touches).
- `casting_costs.rs` changed only in tests (≥17587).

Planner constraints: read-only planning, no cargo. Every runtime claim below comes from tracing, not from a probe. Each unprobed claim is labelled **(unprobed)**, and the test that settles it has a reach guard.

#9728's semantic delta for this plan: `resume_pending_continuation_if_priority` lost its `deliver_deferred_spell` tail. A spell's final part now runs at the action-boundary convergence (`owed_spell_final_part_is_due`, `engine.rs` 1702 → `run_spell_final_part_if_due`, 9098) and in the priority pass. No step here depends on the removed tail, and nothing in D5's loop runs a final part.

---

## Findings closed (one design)

- **CR-owner** (CodeRabbit, Major, `engine_replacement.rs` 1624–1635). Problem: the Prevented arm sends a prevented `RemoveCounter` to the effect-queue drain only when `pending_cost_move_resume.is_none()`. That condition means "no root exists", where it should mean "the root owns this removal". As a result, a child removal under a parked cost root lets the root settle first and leaves the child's `CounterRemovals` frame stuck with `in_flight` set. This was introduced by 1cbd2df; base 3b828ac drained unconditionally and stranded a `Cast`/`ActivationMillPayment` root.
- **M-F1** (maintainer, MED). Problem: `drain_pending_cost_move_resume`'s `PriorityBoundary` arm (`engine.rs` 8783–8791) admits only `Cast{FinalizeDelvedCast}` and `ManaAbilityPayment`. Witness: an optional RemoveCounter replacement with `execute = Scry(1)` surfaces `ScryChoice` at `engine_replacement.rs` 1037, so the delivery drain at 1345 does not run. Answering the Scry goes through `engine_resolution_choices.rs` 2017–2082, then `finish_with_continuation` (9403–9411), then `resume_pending_continuation_if_priority` (`engine.rs` 8874), then the PriorityBoundary drain (8952–8961). That drain skips the counter-cost root, so the activation is left off the stack.
- **M-F2** (maintainer, MED): the same defect as CR-owner. Extra asks:
  - inspect the exact owner, including the mana cursor and the child resolution frame;
  - finish the child's removal queue and its rider (`counters.rs` 3446–3472) before the parent resumes;
  - add a multi-entry child test with a trailing rider;
  - add a prevented-child control with an unrelated outer root parked.

**The one rule:** a counter-cost root resumes only when its whole replacement subtree has retired, wherever that subtree finishes. That can be:
- the replacement boundary;
- the priority boundary after a child choice, reached from any of the ~50 callers of `resume_pending_continuation_if_priority`;
- a prevented child.

It also resumes before any outer payment it is paying for (B1).

---

## Step 0: premise (verified)

- Essence Bottle and Saprazzan Cove (`data/mtgjson/AtomicCards.json`) match the file's constants verbatim.
- Divination is `{2}{U}`, "Draw two cards." (AtomicCards). It is used verbatim in T9.
- No card has a `RemoveCounter` replacement: `rg ReplacementEvent::RemoveCounter crates/engine/src/parser` returns nothing, and `client/public/card-data.json` has 0 `"event":"RemoveCounter"`. Every row is therefore a typed-input witness, as in the PR's R2 rows.
- The holder is "Steward" (`install_optional_counter_removal_replacement`, ~:1183). Prevention is staged by `stage_prevented_counter_removal` (~:1608) because `remove_counter_applier` (`replacement.rs` 4697) always returns `Modified`.
- The mill witness is synthetic. AtomicCards' single-mill-cost lines don't isolate the `ActivationMillPayment` root:
  - Charmed Pendant is a mana ability.
  - The Warring Triad targets and asks for a colour.
  - Sinister Concoction adds interactive discard and sacrifice legs.

  So `MILL_COST_CONTROL = "{T}, Mill a card: You gain 1 life."` is a labelled typed control, following the file's `EFFECT_PATH_CONTROL` / `X_READER` precedent. `DRAW_CONTROL = "{T}: Draw a card."` (T10) is labelled synthetic on the same precedent.

CR numbers cited below were grep-verified in `docs/MagicCompRules.txt`: 118.12, 601.2h, 602.2b, 605.3b, 608.2c, 614.5, 614.6, 616.1, 616.1f, 616.1g, 118.11, 120.3c, 122.1.

## Step 1: skills

- `/add-replacement-effect`: the post-replacement drain lifecycle and the Execute/Prevented arms.
- `/casting-stack-conditions`: cost-move roots and resume boundaries.
- `/add-engine-variant`: no variant is added to a listed engine enum. These additions get the same gate:
  - two new serialized structs (`ReplacementSubtreeBoundary` and `EnclosingPostReplacementFrame`), which are field types parameterizing the existing record;
  - one private, unserialized classification enum;
  - one private, unserialized loop measure.
- `/card-test`: public `GameAction` flows only.

## Step 2: verified trace (with r1/r2 corrections)

### Ownership records

- Effect queue: `resolve_remove` → `push_counter_removals` → `drain_pending_counter_removals` (`counters.rs` 3446). `active_counter_removals()` is top-only (`resolution.rs` 1651).
- Cost payer: `pay_counter_cost_removals` (`costs.rs` 786–822) is the single constructor of `PausedCounterCostRemoval` (`game_state.rs` 6043). Its callers are `costs.rs` 1894, `casting_costs.rs` 4910/5033 and `mana_abilities.rs` 4146/4628. The record is held by:
  - the stack route: `CounterCostRemovalPayment.removal` (`game_state.rs` 8292);
  - the mana route: `ManaAbilityCostCursor.counter_removal` (`game_state.rs` 8110).
- Unrecorded removals: damage (`deal_damage.rs` 703/716), resolution-scope PayCost (`costs.rs` 1918–1923), negative loyalty (`costs.rs` ~1791), unless-payments (`engine_payment_choices.rs` 1718/1733).

### Post-replacement subtree, and the boundary's precondition

- An accepted replacement's post-effect is a drain. `GameState::install_post_replacement_drain` (`game_state.rs` 27734) installs it **into the existing frame** when `active_post_replacement_drains_mut()` is `Some`. That is the case when the top frame or the frame immediately below it is `PostReplacement` (`resolution.rs` 2845–2860, `active_post_replacement_parent_slot`). Otherwise it pushes a new `PostReplacement` frame.
- `PostReplacementDrainStack::install(.., Replace)` (`game_state.rs` 24988–25010) evicts only a `Ready` resident. Under a `Dispatching`/`Paused` resident the new drain nests above it in the same frame (CR 616.1g).
- A finished drain is removed from the frame:
  - `finish_dispatch` → `drains.remove` (25067–25077);
  - `finish_paused_dispatch` → `drains.pop`, innermost `Paused` only (25082–25087). An outer paused drain therefore cannot be popped while a nested child drain sits above it.
- `apply_pending_post_replacement_effect` (`engine_replacement.rs` 3261–3375):
  - a completed effect retires its dispatch and pops an empty frame (3351–3361);
  - a paused effect parks its dispatch (3345–3350).
- A parked dispatch is retired inside `drain_pending_continuation` (`effects/mod.rs` ~1389) or by the `PostReplacement` arm of `resume_resolution_frames` (`effects/mod.rs` 1783–1792).
- **Frame identity.** A `PostReplacement` frame is stamped with a `PostReplacementFrameId` at its first dispatch (`begin_active_post_replacement_dispatch`, `resolution.rs` 2883–2913; `stamp_frame_id`, `game_state.rs` 25155). That is the only production `begin_dispatch` caller (the other, `resolution.rs` 6069, is a test fixture). So every frame holding a `Dispatching` or `Paused` drain is stamped, except a frame restored from a pre-identity payload (`id` is `serde(default)`, `game_state.rs` 24912). `ResolutionStack::post_replacement_frame(id)` (`resolution.rs` 2939, private) finds a frame by identity through `FrameVec::by_id`, wherever it has moved. The allocator `last_post_replacement_frame_id` is serialized and rewinds only together with the whole state on a failed action (`resolution.rs` ~780–810), so an id recorded in state cannot alias a later frame.

So a root parked with no PostReplacement frame among the top two frames has its subtree wholly above the depth `len` recorded at park. Nothing below that depth is consumed while those frames live: the stack is LIFO, and the one pre-existing exception (the ungated `drain_pending_continuation` that can run an outer continuation C above a paired PostReplacement parent, review r2 "Pre-existing") needs such a parent.

When the root parks under such a frame P (for example, a mana ability activated to pay a prompt raised inside a paused post-effect: row T10), the child's drain **nests into P** without growing `len`. If C sits above P (`[.., P, C]`) and is consumed early, `len` drops below the recorded depth while the child is still nested in P. Neither `len` alone (r1 B2) nor a positional drain count with `Less → retired` (r2 T-A) is exact there. The boundary therefore records the depth **and P by identity, with P's drain count** (D1).

### Analogous feature (the existing typed seam)

`DeferredLifeCostResume::*::resume_at_resolution_depth` (`game_state.rs` 7895–7958):
- recorded at `costs.rs` 913/938;
- consumed by `resume_pending_continuation_if_priority` (`engine.rs` 8883–8922).

Trace path: `types/game_state.rs` → `game/costs.rs` → `game/engine.rs::resume_pending_continuation_if_priority` → `effects/mod.rs::{drain_pending_continuation, resume_resolution_frames}`.

That seam has the same nested-frame gap (r1 "pre-existing"). This plan does not change it.

### Execute arm order (`engine_replacement.rs`)

1. Queue (1150).
2. Continuation, skipped under a mana root (1275–1296).
3. Delivery cost drain (1345–1356).
4. Continuation (1360–1369).
5. `resume_interrupted_cost_payment` when `pending_cast`/`pending_discard_for_cost` (1371–1386; `casting_costs.rs` 3112–3126 finishes the outer cast).
6. `resume_pending_continuation_if_priority` (1415).

### The common boundary and its callers

- `resume_pending_continuation_if_priority` (`engine.rs` 8874–8979) has about 50 call sites:
  - 18 in `engine_resolution_choices.rs`, including `finish_with_continuation` (9403–9411), which is the M-F1 route, and its 13 `.expect` siblings;
  - 12 in `engine_payment_choices.rs`, 17 in `engine.rs`, 4 in `engine_replacement.rs`;
  - plus `engine_stack.rs`, `contest.rs` and `effects/choose_from_zone.rs`.
- It is already re-entrant. `drain_pending_cost_move_resume` → `resume_random_discard_unless_payment` (`engine_payment_choices.rs` 2595) → `finish_successful_unless_payment` (1970, via 2658) → `resume_pending_continuation_if_priority` (2040).
- Its tail `settle_resolving_stack_entry_after_continuation_resume` (8981) runs on every call.

### Corrections recorded

- Identity matching (object, counter type, count) cannot separate a same-subject child from the root's own removal (T6), so ownership is by subtree boundary.
- The r6 fall-through passed through the copy-token and batch blocks, so the typed tail is called directly instead (D4).

## Step 3: files read (at 4483f4b)

- `engine_replacement.rs`: 180–320, 330–620, 1000–1730, 3261–3375.
- `engine.rs`: 1700–1720, 8718–8795, 8874–9030, 13420–14070, 27038–27095.
- `effects/counters.rs`: 1448–1525, 3149–3260, 3380–3492.
- `effects/mod.rs`: 1209–1400, 1655–1800, 2716–2753; the `unless_pay` paths at ~16995–17100.
- `costs.rs`: 774–822, 900–945, 1780–1930.
- `casting_costs.rs`: 3112–3126, 4950–5074, 6460–6531, 6722–6739.
- `mana_abilities.rs`: 2767–2792, 3235–3270, 3365–3390, 4120–4160, 4218–4228, 4600–4670.
- `types/game_state.rs`: 6018–6065, 7580–7590, 7895–7958, 8062–8125, 8188–8433, 24697–25205, 26310, 26407, 27530–27760, 42095–42115.
- `types/resolution.rs`: 407–411, 755–860, 1151–1230, 1651–1700, 2800–2990, 3091, 3316.
- `types/ability.rs`: 15639–15649 (`UnlessPayModifier`), 27937 (`AbilityDefinition::unless_pay`).
- `engine_resolution_choices.rs`: 2017–2082, 9403–9411.
- `engine_payment_choices.rs`: 1700–1750, 1970–2050, 2515–2575, 2595–2660.
- `replacement.rs`: 4683–4704, 5534–5580, 9361–9410, 12370–12560.
- The whole test harness of `cost_removed_counter_amount.rs` (1–410, 780–945, 1170–1760).
- The protocol entries: `lobby-broker/src/protocol.rs` 63, `server-core/src/protocol.rs` 3476, `ws-adapter.ts` 218, `protocol.ts` 109.

## Step 3.5: probes

None were run (no-build constraint). Unprobed assertions, each with a row reach:

- **P1.** A root parked with no PostReplacement frame among the top two frames sees its child's frames above `resolution_depth`. The rest of the stack returns to `resolution_depth` exactly when they retire. Rows T1, T3, T5, T6, T9.
- **P1'.** A root parked under such a frame sees its child's drain nest into that frame. The frame's drain count, found by identity, rises, then drops back when the child retires, even if a frame above it is consumed first. Rows U2, U3 (helper level); T10 (flow reach).
- **P2.** A post-effect `RemoveCounter` with a `sub_ability` rider parks the rider as an `AbilityContinuation` under the queue. Rows T3, T9 and T10 (frame-iteration reach), and T5.
- **P3.** `MILL_COST_CONTROL` parses and parks `ActivationMillPayment` on an optional Mill replacement. Row T4.
- **P4.** `QuantityRef::PreviousEffectCount` in the rider reads the queue's `applied_total` stamp. Row T3. Fallback: use the ref `EFFECT_PATH_CONTROL` parses to for "removed this way" (row `effect_path_removal_still_reads_its_own_count` already reads the queue stamp).
- **P5.** In `CastPaymentMode::Manual`, Divination waits at `ManaPayment`, and the Cove's mana ability can be activated there (as in `a_mana_ability_activated_while_paying_does_not_take_the_pending_count`). Row T9.
- **P6.** A post-replacement `execute` carrying `unless_pay` raises `WaitingFor::UnlessPayment` with its drain `Paused` in a frame that is top or top-1. Mana abilities are legal there (`engine.rs` 13424–13456, CR 118.12). Row T10. Its reach guard is the first assertion; if it fails, T10 is dropped and U2/U3 stand as the evidence (see T-C in the dispositions).

---

## Design

### D1. Exact subtree boundary on the shared removal record, keyed by frame identity

This adds a serialized field, and the field is required.

In `crates/engine/src/types/game_state.rs`, next to `PausedCounterCostRemoval` (6043):

```rust
/// CR 616.1 + CR 608.2c: where a counter removal's replacement subtree begins.
/// Everything the removal's replacement installs (its post-effect drain, an
/// effect removal queue, a rider) is above this position: on frames above
/// `resolution_depth`, or nested (CR 616.1g) into the post-replacement frame
/// that was the drain owner when the removal was proposed
/// (`install_post_replacement_drain`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplacementSubtreeBoundary {
    pub resolution_depth: usize,
    pub enclosing: Option<EnclosingPostReplacementFrame>,
}

/// The post-replacement frame a removal's own post-effect drain nests into, and
/// how many drains it held when the removal was proposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnclosingPostReplacementFrame {
    /// The frame's identity. Every frame holding a dispatched (`Paused`) drain
    /// is stamped; `None` only for a frame restored from a pre-identity payload,
    /// which is then read positionally.
    pub frame: Option<PostReplacementFrameId>,
    pub drains: usize,
}

impl ReplacementSubtreeBoundary {
    /// The current position of `stack`, as the payer records it.
    pub(crate) fn at(stack: &ResolutionStack) -> Self {
        Self {
            resolution_depth: stack.len(),
            enclosing: stack.active_post_replacement_or_paired_parent().map(|drains| {
                EnclosingPostReplacementFrame { frame: drains.frame_id(), drains: drains.len() }
            }),
        }
    }

    /// Whether nothing the subtree installed remains: no frame above the
    /// recorded depth, and the enclosing frame (found by identity, wherever it
    /// now sits) holds no more drains than it did.
    pub(crate) fn is_retired(&self, stack: &ResolutionStack) -> bool {
        stack.len() <= self.resolution_depth
            && self.enclosing.is_none_or(|enclosing| enclosing.is_retired(stack))
    }
}

impl EnclosingPostReplacementFrame {
    fn is_retired(self, stack: &ResolutionStack) -> bool {
        let drains_now = match self.frame {
            Some(id) => stack.post_replacement_frame_drain_count(id),
            None => stack
                .active_post_replacement_or_paired_parent()
                .map(PostReplacementDrainStack::len),
        };
        // A consumed frame took its nested drains with it.
        drains_now.is_none_or(|drains| drains <= self.drains)
    }
}
```

Supporting edits:
- Add `pub fn len(&self) -> usize { self.drains.len() }` to `PostReplacementDrainStack` (next to `is_empty`, `game_state.rs` 24942). `is_empty` already exists, so clippy is satisfied. `frame_id()` (25133, `pub(super)`) is reachable here because both types live in `types/game_state.rs`.
- Add to `ResolutionStack` in `crates/engine/src/types/resolution.rs`, next to `post_replacement_frame` (2939):
  ```rust
  /// CR 616.1g: the drain count of the frame `id` names, if it is still
  /// resident. Identity-addressed, like the dispatcher's own cleanup.
  pub(crate) fn post_replacement_frame_drain_count(&self, id: PostReplacementFrameId) -> Option<usize> {
      self.post_replacement_frame(id).map(PostReplacementDrainStack::len)
  }
  ```
  It is read-only and goes through the existing `FrameVec::by_id` authority, so the type-enforced "no positional search" contract (`resolution.rs` 757–767) holds.
- `PausedCounterCostRemoval` gains `pub replacement_subtree: ReplacementSubtreeBoundary`. It is **required**: no `#[serde(default)]`. The record is new in this PR's unreleased v151, so no legacy payload exists. A missing field fails decode instead of defaulting to a depth 0 that would strand a mid-resolution root (r1 T4). The inner `enclosing`/`frame` options decode a missing key as `None` under serde's derive, which is the correct "no enclosing frame" meaning.
- `costs.rs::pay_counter_cost_removals` (786) records `let replacement_subtree = ReplacementSubtreeBoundary::at(&state.resolution_stack);` as its first statement and sets it in the literal (~809). A re-park from either resume calls the payer again and re-records the boundary.
- **No debug assertion** (r2 T-B). The `Ready`-resident eviction it guarded is residual 6.
- Compiler-forced exhaustive destructures: `casting_costs.rs` 5020 and `mana_abilities.rs` 4138 add `replacement_subtree: _`.

**Why exact, and why by identity:**
- r1 B2 option (b) (`debug_assert!(active_post_replacement_drains().is_none())`) is unsafe. The nested configuration (a mana ability paying a prompt raised inside a paused post-effect) is reachable through public actions (T10's flow, P6), so the assertion would fire.
- r2's positional count was inexact in the paired-parent shape (T-A). The identity key costs one `Option` field and one read-only, identity-addressed accessor over an existing private lookup, and makes the `Less` case exact.
- The only positional fallback is an unstamped frame, which needs a pre-identity payload: residual 7.
- Equality: the recorded id participates in `GameState` equality through `PendingCostMoveResume`'s derived `PartialEq`. The exposure note at `game_state.rs` 42104–42114 calls that direction fail-safe for frame ids already in the stack.

### D2. One typed query for the root's subtree

In `crates/engine/src/types/game_state.rs`, inside the `impl PendingCostMoveResume` block (8358):

```rust
/// CR 601.2h + CR 616.1: the counter-cost removal this root holds paused on
/// its replacement choice — the stack route's own record, or the mana cursor's.
/// Exhaustive so a new root is classified.
pub(crate) fn paused_counter_removal(&self) -> Option<&PausedCounterCostRemoval> {
    match self {
        Self::CounterCostRemovalPayment { removal, .. } => Some(removal),
        Self::ManaAbilityPayment { cursor, .. } => cursor.counter_removal.as_deref(),
        Self::Cast { .. } | Self::SacrificeForCost { .. } | Self::WardSacrificePayment { .. }
        | Self::ReplacementMayCost { .. } | Self::Foretell { .. }
        | Self::CollectEvidencePayment { .. } | Self::UnlessBouncePayment { .. }
        | Self::ActivationMillPayment { .. } | Self::LoyaltyActivation { .. }
        | Self::CounterAdditionUnlessPayment { .. } | Self::RandomDiscardUnlessPayment(_) => None,
    }
}
```

Then:

```rust
/// CR 616.1 + CR 608.2c: where a parked counter-cost root's own replacement
/// subtree stands. Private plumbing; not serialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CounterCostRootSubtree { Absent, Unfinished, Retired }

impl GameState {
    pub(crate) fn counter_cost_root_subtree(&self) -> CounterCostRootSubtree {
        match self.pending_cost_move_resume.as_ref()
            .and_then(PendingCostMoveResume::paused_counter_removal)
        {
            None => CounterCostRootSubtree::Absent,
            Some(paused) if paused.replacement_subtree.is_retired(&self.resolution_stack) =>
                CounterCostRootSubtree::Retired,
            Some(_) => CounterCostRootSubtree::Unfinished,
        }
    }
}
```

### D3. Cost-move eligibility: one gate at every boundary, plus the priority-boundary admission (M-F1)

In `crates/engine/src/game/engine.rs::drain_pending_cost_move_resume` (8732–8795):

- Return `Ok(None)` first when `state.counter_cost_root_subtree() == CounterCostRootSubtree::Unfinished`, with the comment:

  > CR 616.1 + CR 608.2c: a root whose replacement subtree is still live is not resumable at any boundary.

  Only `CounterCostRemovalPayment`, and `ManaAbilityPayment` with a counter leg, can read `Unfinished`.
- `PriorityBoundary` arm (8783–8791): add `| PendingCostMoveResume::CounterCostRemovalPayment { .. }`.
  - `resume_counter_cost_removal_payment` reads the boundary only as `action_event_start` (`None` → `events.len()` for `CompleteSelectedReturnToHand`, `casting_costs.rs` 5064).
  - `CounterAdditionUnlessPayment` stays unadmitted, so its `unreachable!` and the existing unit row hold.
- Update the doc at 8728–8731, the module doc of `cost_move_drain_priority_boundary_tests` (~27047–27050), and `engine_payment_choices.rs` 2526–2530. New text: Delve, mana cursors, and a counter-cost root whose subtree has retired. These are comment-only edits.
- Mana route (the M-F1 question): `ManaAbilityPayment` is already admitted at `PriorityBoundary`, so the Scry shape does not strand it at the base (row T2). D3 adds only the subtree condition.

### D4. Prevented arm: classify by the subtree (CR-owner / M-F2)

All in `crates/engine/src/game/engine_replacement.rs`.

**(a) Extract the typed tail** (1656–1686, Priority reset → `drain_pending_cost_move_resume(ReplacementPrevented)` → the mana-rider drain) into:
```rust
/// CR 601.2h + CR 602.2b + CR 605.3b + CR 616.1: resume the parked cost-move
/// root at the Prevented boundary; a mana root's rider drains after it.
fn resume_cost_move_root_after_prevention(
    state: &mut GameState, events: &mut Vec<GameEvent>, action_event_start: usize,
) -> Result<Option<WaitingFor>, EngineError>
```
- The body is today's code, returning `Some(state.waiting_for.clone())` where it returns now.
- The inherited inner annotations "CR 614.12a" (1655) and "CR 118.12" (1675) are replaced:
  - 614.12a concerns a permanent entering the battlefield;
  - 118.12 concerns "unless/if you do" costs;
  - neither fits a generic cost-root tail (r1 T3).

  Use "CR 601.2h + CR 602.2b + CR 616.1" for the prevented cost move, and "CR 605.3b + CR 616.1" for the mana-rider ordering.
- The general tail calls it with `if let Some(wf) = ...? { return Ok(wf); }`, which is behaviour-identical.

**(b) Replace the counter-removal block** (1624–1635). It always returns:
```rust
// CR 616.1 + CR 608.2c: a prevented counter removal belongs to whatever owns
// the work it interrupted. Work the parked counter-cost root's replacement
// installed is that root's subtree (its post-effect, an effect removal queue,
// a rider): it finishes first, and the root resumes once it retires. With the
// subtree retired the removal is the root's own (CR 118.11: the cost is still
// paid). With no counter-cost root the owner is the resolving work (an effect
// queue, damage, a resolution-time cost) and any other parked root resumes after
// it, in the Execute arm's order.
if pending_was_counter_removal {
    state.waiting_for = WaitingFor::Priority { player: state.active_player };
    match state.counter_cost_root_subtree() {
        CounterCostRootSubtree::Unfinished => {
            effects::counters::drain_pending_counter_removals(state, events);
            super::engine::resume_pending_continuation_if_priority(state, events)?;
        }
        CounterCostRootSubtree::Retired => {
            resume_cost_move_root_after_prevention(state, events, replacement_action_event_start)?;
        }
        CounterCostRootSubtree::Absent => {
            effects::counters::drain_pending_counter_removals(state, events);
            if matches!(state.waiting_for, WaitingFor::Priority { .. })
                && !matches!(state.pending_cost_move_resume, Some(PendingCostMoveResume::ManaAbilityPayment { .. }))
            {
                effects::drain_pending_continuation(state, events);
            }
            if matches!(state.waiting_for, WaitingFor::Priority { .. }) {
                resume_cost_move_root_after_prevention(state, events, replacement_action_event_start)?;
            }
        }
    }
    return Ok(state.waiting_for.clone());
}
```
- `Unfinished` covers T3, T5, T6, T9 and T10: the child queue, the unrecorded damage child, the same-subject child, and the nested child.
  - The explicit queue drain finishes the child's removals and the queue finalization (`last_effect_count` stamp plus `EffectResolved`).
  - The common boundary (looping per D5) finishes the rest of the subtree and resumes the root.
  - A next entry that re-parks leaves a `ReplacementChoice`, and the boundary is inert.
- `Retired` is the root's own removal on either route. It behaves as at the base (T7, T8).
- `Absent` is the base's rootless path (T7 damage row). It adds the parent-root resume that fixes 3b828ac's stranding (T4). A rootless tail call is inert (`Ok(None)`).

**(c) Tail guard.** Insert this just before the extracted-tail call in the general Prevented path, after the copy-token and batch blocks at 1636–1653:
```rust
// CR 616.1 + CR 608.2c: any other prevented event inside a parked counter-cost
// root's replacement subtree finishes that subtree first; the root resumes once
// it retires.
if state.counter_cost_root_subtree() == CounterCostRootSubtree::Unfinished {
    state.waiting_for = WaitingFor::Priority { player: state.active_player };
    super::engine::resume_pending_continuation_if_priority(state, events)?;
    return Ok(state.waiting_for.clone());
}
```
Without this guard, D3 makes the root ineligible at the tail. Control would then reach the CR 608.3e teardown (1688–1727), which clears the child's continuation.

### D5. The common boundary runs a live counter-cost subtree to retirement (r1 T5, r2 X3 + T-D)

The loop lives **inside** `resume_pending_continuation_if_priority`. There is no parallel entry point, so every one of its ~50 callers is covered, including the M-F1 route through `finish_with_continuation` and its 13 `.expect` siblings (T-D).

In `crates/engine/src/game/engine.rs`:

1. Rename today's body of `resume_pending_continuation_if_priority` (8874–8979, including the trailing `settle_resolving_stack_entry_after_continuation_resume` call) to a private `fn resume_pending_continuation_pass(state, events) -> Result<(), EngineError>`. Move it verbatim; its comments and CR annotations stay.
2. The public function becomes:

```rust
/// CR 605.3b + CR 616.1 + CR 608.2c: the common priority boundary. One pass
/// for every state except a parked counter-cost root whose replacement subtree
/// is still live: `resume_resolution_frames` advances one top frame per pass,
/// so a subtree several frames deep needs several, and the root resumes in the
/// pass that retires the last of it (D3 admits it at `PriorityBoundary`). A
/// repeat needs the position to strictly decrease, so the loop ends.
pub(super) fn resume_pending_continuation_if_priority(
    state: &mut GameState,
    events: &mut Vec<GameEvent>,
) -> Result<(), EngineError> {
    loop {
        let before = SubtreePosition::at(&state.resolution_stack);
        resume_pending_continuation_pass(state, events)?;
        let after = SubtreePosition::at(&state.resolution_stack);
        if !matches!(state.waiting_for, WaitingFor::Priority { .. })
            || state.counter_cost_root_subtree() != CounterCostRootSubtree::Unfinished
            || !after.is_strictly_inside(&before)
        {
            return Ok(());
        }
    }
}

/// The loop's progress measure: stack depth, then the drain count of the active
/// post-replacement frame. Positional on purpose: it only has to be
/// well-founded, not to name the subtree (that is `ReplacementSubtreeBoundary`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubtreePosition {
    resolution_depth: usize,
    enclosing_post_replacement_drains: usize,
}

impl SubtreePosition {
    fn at(stack: &ResolutionStack) -> Self {
        Self {
            resolution_depth: stack.len(),
            enclosing_post_replacement_drains: stack
                .active_post_replacement_or_paired_parent()
                .map_or(0, PostReplacementDrainStack::len),
        }
    }

    /// Strictly less of the subtree remains: the stack is lower, or as high
    /// with fewer drains in the enclosing frame (lexicographic on ℕ×ℕ).
    fn is_strictly_inside(&self, before: &Self) -> bool {
        (self.resolution_depth, self.enclosing_post_replacement_drains)
            < (before.resolution_depth, before.enclosing_post_replacement_drains)
    }
}
```

`is_strictly_inside` and the exit condition are the review r2 X3 replacement code, used verbatim. The only change is the receiver: a private `SubtreePosition` measure instead of `ReplacementSubtreeBoundary`, because the boundary is now identity-keyed (T-A) and is not a total order.

**Termination:** a repeat requires `(resolution_depth, enclosing drains)` to strictly decrease lexicographically. ℕ×ℕ under that order is well-founded, so the loop ends. A pass that does not strictly decrease ends it, and any subtree left is residual 3.

**Why moving the loop inside is sound (T-D):**
- **Inert elsewhere.** With no `Unfinished` counter-cost root, the first pass exits on the second condition. Every other flow runs exactly today's body once.
- **No new state shape.** Each extra pass is a full call of today's body (drains, liminal entry, phase transition, cost-move drain, deferred step triggers, settle). That is what the next boundary would run; the loop only runs it in the same action.
- **Re-entrancy is unchanged.** The body already recurses through the random-discard root (Step 2). A nested call runs its own bounded loop and returns. The outer loop then measures the position again and needs its own strict decrease, so each invocation terminates and the recursion depth is today's.
- **Errors propagate unchanged** (`?`).
- **Callers that act afterwards** (`finish_successful_unless_payment`'s post-action pipeline at 2041+, `pass_priority_once_with_pipeline` at 9441, the 1415 epilogue) see the state a finished subtree leaves. That is the state they would see had the subtree finished in one pass.
- **Pre-existing exposure.** A repeat pass can run the pre-existing ungated outer-C drain (review r2 "Pre-existing") just as the next boundary would. The loop does not create it, and it is unaffected by D1's identity key.

As a result, D4(b), D4(c) and the Execute epilogue (1415) call `resume_pending_continuation_if_priority` unchanged. r2's separate helper `resume_pending_continuation_through_counter_cost_subtree` is gone, and so are r1's D5(a)/(b).

### D6. Execute arm: the root resumes before the outer payment it is paying for (B1)

In `crates/engine/src/game/engine_replacement.rs`:

**(a)** After the 1360–1369 continuation drain, insert:
```rust
// CR 601.2h + CR 605.3b + CR 608.2c + CR 616.1: a counter-cost root deferred
// at the delivery drain above because its replacement subtree was live resumes
// as soon as the child's rider has retired that subtree — at this same
// replacement boundary, before any outer cast or activation it is paying for
// (a mana ability paying a spell's cost) is finished below.
if matches!(waiting_for, WaitingFor::Priority { .. })
    && state.counter_cost_root_subtree() == CounterCostRootSubtree::Retired
{
    if let Some(resumed) = super::engine::drain_pending_cost_move_resume(
        state, events,
        super::engine::CostMoveDrainBoundary::ReplacementDelivered {
            action_event_start: replacement_action_event_start,
        },
    )? {
        waiting_for = resumed;
    }
}
```
A root still parked and `Retired` at this point was necessarily deferred at 1345: a `Retired` root at 1345 is drained there, and a root re-parked by its own resume leaves a non-Priority prompt.

**(b)** Gate 1371–1386 by adding `&& state.counter_cost_root_subtree() != CounterCostRootSubtree::Unfinished` to its condition. While the subtree is still live (no rider retired it, e.g. a paused dispatch with no continuation), the outer `resume_interrupted_cost_payment` is not run. Control reaches 1415, where `resume_pending_continuation_if_priority` (looping per D5):
- retires the dispatch (`resume_resolution_frames`);
- resumes the root at `PriorityBoundary`, after which the mana root returns the outer `ManaPayment` prompt.

**Why both:**
- (a) keeps the Execute order exact at the replacement boundary when the rider retires the subtree (T9): queue → subtree → root → outer payment.
- (b) is the invariant that the outer payment never finishes under a live subtree. The remaining case falls to the common boundary instead of re-implementing it.
- Rejected alternative: (b) alone. It would also be correct, but it moves every rider-retired resume to the 1415 boundary with `action_event_start = None`. (a) keeps the delivery boundary's event start for `CompleteSelectedReturnToHand`.

### Ownership / provenance contract

- **Authority:** the parked root's `PausedCounterCostRemoval`, read through `paused_counter_removal`: the stack root's `removal` or the cursor's `counter_removal`. Its `replacement_subtree` is compared with the live stack, with the enclosing frame found by identity (D1).
- **Bound when:** the payer proposes the removal (CR 601.2h payment step). It is never re-derived.
- **Consumed by:** `counter_cost_root_subtree` in D3, D4, D5 and D6.
- **Invalidated:** when the resume consumes the record. A re-park records a fresh one.
- **Multi-authority hostiles:**

  | Case | Classification | Row |
  |---|---|---|
  | Child queue plus rider | `Unfinished` | T3 |
  | Unrecorded damage child | `Unfinished` | T5 |
  | Same-subject child | `Unfinished` (identity cannot separate it) | T6 |
  | Nested-into-frame child | `Unfinished` | U2, T10 |
  | Nested child after the frame above it was consumed (`[.., P, C]`, C drained early) | `Unfinished` (by frame identity) | U3 |
  | Mana root paying an outer cast | `Unfinished`, then outer payment | T9 |
  | Unrelated non-counter root | `Absent` | T4 |
  | Root's own removal | `Retired` | T7, T8 |

### Residuals (named; no code)

1. **Non-counter roots carry no boundary** (`Cast`, `ActivationMillPayment`, `ManaAbilityPayment` without a counter leg). At the base and after this phase, they resume after the child's queue and rider but before a paused PostReplacement dispatch of their own replacement retires. The Execute arm does the same (1345 before 1415). Giving every root the boundary means a field on every variant, which is a follow-up. T4 asserts only what is guaranteed.
2. **Other Prevented-arm child blocks under a counter-cost root** (connive, life-total, counter additions/moves at 1534–1621, copy token, batch at 1636–1653) return Priority with the root still parked. The root then resumes only after the next priority pass's `resume_pending_continuation_if_priority`, which runs after `handle_priority_pass`. **Priority is surfaced mid-activation**, contrary to `PendingCostMoveResume::withholds_priority` (`game_state.rs` 8365; CR 601.2h/602.2b). This is a remaining defect and a follow-up (fold these blocks into D4(c)). Every route is synthetic: no printed RemoveCounter replacement exists.
3. **A subtree the common boundary cannot advance.** A pass whose position does not strictly decrease ends D5's loop with the root parked under `Priority`. Effects:
   - The same mid-activation Priority as residual 2, with the same follow-up.
   - **#9728 interaction (T-F).** At the action boundary, `owed_spell_final_part_is_due` (`engine.rs` 1702) → `resolution_instructions_are_done` (9022–9029) checks only top-only accessors and `pending_cast`, not `pending_cost_move_resume`. So with a mana root parked mid-resolution (paying a resolution-time prompt such as T10's `UnlessPayment` inside a resolving instant), the instant's final part could run under the parked payment. The cast route is unaffected because `pending_cast` is set. The loop itself runs no final part (#9728 removed `deliver_deferred_spell` from the boundary). The T-D move reduces this to the no-progress case but does not close it. Follow-up: make `resolution_instructions_are_done` refuse while `pending_cost_move_resume.as_ref().is_some_and(PendingCostMoveResume::withholds_priority)`.

   No row reaches a no-progress subtree; it needs a resolution frame that re-parks without prompting (e.g. a `RepeatFor`/`RepeatUntil` re-park, `effects/mod.rs` 2716–2734) inside a synthetic RemoveCounter replacement.
4. **Error contract at the new admission (r1 T2).** `resume_counter_cost_removal_payment` can return `Err`:
   - `require_locked_activation_cost`;
   - a `Failed` residual in `finish_counter_cost_payment` (`casting_costs.rs` 4985–4987);
   - the `InvalidAction` completion arm.

   A child post-effect can change the board before the residual is paid (for example, by tapping the source).
   - At the replacement boundaries the `Err` rejects the `ChooseReplacement` action, as at the base.
   - At `PriorityBoundary`, `finish_with_continuation` (`engine_resolution_choices.rs` 9403–9411) and its 13 sibling `.expect("a settled … must resume its continuation")` sites would **panic**. This is the same exposure `ManaAbilityPayment` already has there.

   It is reachable only through a synthetic replacement whose post-effect invalidates the residual. The follow-up is for resolution-choice handlers to propagate that `Result`. It is named here rather than claimed impossible.
5. Resolution-scope PayCost and negative-loyalty removals stay out of run (charter).
6. **A `Ready` enclosing resident (r2 T-B).** If a removal is proposed while the enclosing frame's innermost drain is `Ready` (an undispatched CR 616.1g `KeepResident` stash above a paused drain, `game_state.rs` 24988–24996), the child's `Replace` install evicts that drain (pre-existing loss of the stashed post-effect). The count therefore does not rise, and the boundary reads `Retired` while the child is live, in release builds too. Neither a debug assertion nor this phase changes it. It needs a stash that has not dispatched while a prompt allowing mana abilities is up, plus a synthetic RemoveCounter replacement. The follow-up belongs with the eviction itself: `Replace` should nest above a `Ready` drain that is not this event's.
7. **An unstamped enclosing frame** (one restored from a pre-identity payload while a dispatch was paused) is read positionally, as in r2. That is exact unless the pre-existing early-C drain also fires. Every frame dispatched in this build is stamped (Step 2).

---

## Serialized surface

- **New required field:** `PausedCounterCostRemoval.replacement_subtree: ReplacementSubtreeBoundary { resolution_depth: usize, enclosing: Option<EnclosingPostReplacementFrame { frame: Option<PostReplacementFrameId>, drains: usize }> }`. There is no serde default on `replacement_subtree`.
  - `PostReplacementFrameId` is already a serialized newtype (`game_state.rs` 25199) on the resolution stack's wire.
  - It reaches the wire through `PendingCostMoveResume::CounterCostRemovalPayment.removal` and `ManaAbilityCostCursor.counter_removal`, both introduced by this PR under its v151 / wire-133 bump.
  - **No further bump.** The merge to 4483f4b already renumbered this PR's entries to 151/133.
- **Comment-only:** append "; `PausedCounterCostRemoval` carries its `replacement_subtree` boundary" to the existing v151/133 entries:
  - `crates/lobby-broker/src/protocol.rs` (PROTOCOL_VERSION's `/// 151` doc, :63);
  - `crates/server-core/src/protocol.rs` (`/// v151:`, ~3476);
  - `client/src/adapter/ws-adapter.ts` (~218, the `151 —` entry);
  - `client/src/network/protocol.ts` (~109–115, the `133 —` entry).
- `CounterCostRootSubtree` and `SubtreePosition` are not serialized.
- No visibility redaction is needed: the field holds counts and a frame id, not hidden information.

---

## Tests

All tests use public `GameAction` flows in `crates/engine/tests/integration/cost_removed_counter_amount.rs`, except U1–U3. Red/green is stated against PHASE_BASE 4483f4b; the executor records the evidence.

### Harness refactor (reuse, no near-duplicates)

- `install_replacement_on_steward(runner, def) -> ReplacementId` holds the push currently duplicated at ~:1183 and ~:1608. Both call it. T10 also uses it for its Draw replacement.
- `optional_replacement(event, execute: Option<AbilityDefinition>) -> ReplacementDefinition` is the `Optional { decline: None }` builder.
- `event_index(events, pred) -> usize` panics with the event list when nothing matches. That panic is the reach of every ordering assertion.
- `parked_child_rider(state) -> bool` returns `state.resolution_stack.iter().any(|f| matches!(f, ResolutionFrame::AbilityContinuation(_)))`. The top-only `active_ability_continuation()` cannot see a rider under the queue (r1 T6). Import `engine::types::resolution::ResolutionFrame`.
- `t3_child_execute(battery) -> AbilityDefinition` builds the T3 execute once. T3, T9 and T10 share it.
- New constants: `MILL_COST_CONTROL` and `DRAW_CONTROL` (both labelled synthetic), and `DIVINATION` (verbatim).
- A `Battery` artifact carrying the child's counters.
- Imports: `EffectKind`, `ResolutionFrame`, `DrainStatus`, `UnlessPayModifier`.

### Rows

| # | Row | Flow | Assertions | At 4483f4b |
|---|---|---|---|---|
| T1 | `a_counter_cost_root_resumes_after_its_replacement_scry_child` (M-F1) | Essence Bottle (3 elixir), P0 library ≥ 2, RemoveCounter optional `execute = Scry { count: 1, target: Controller }`. Activate → `ChooseReplacement(ACCEPT)` → `ScryChoice` → `SelectCards { cards: [top] }`. | Reach at Scry: `ScryChoice`, root is `CounterCostRemovalPayment`, stack empty, elixir 0. After: stack 1, `top_stack_count == Some(3)`, `payment_ownership_is_clear`, queue none. Resolve, then life +6. | **red** (stranded) |
| T2 | `a_mana_cost_root_resumes_after_its_replacement_scry_child` | Saprazzan Cove (3 storage), `Answers::pay(2)`, same Scry execute. | Reach: `parked_mana_cursor(..).counter_removal.is_some()` at Scry. After: blue 2, storage 1, ownership clear. | green (already admitted); guards D1/D3 on the cursor |
| T3 | `a_prevented_child_removal_queue_and_rider_finish_before_the_cost_root` (M-F2) | Bottle 3. Battery with charge 1 and +1/+1 1. Execute = `t3_child_execute`: `RemoveCounter { counter_type: None, count: Fixed(-1), target: SpecificObject(battery) }` `.sub_ability(GainLife { amount: Ref(PreviousEffectCount), player: Controller })` (P4 fallback named). Activate → ACCEPT → entry 1 prompt → stage prevention → choose 0 → entry 2 prompt → `ChooseReplacement(DECLINE)`. | At entry 1 and entry 2 prompts: `parked_child_rider` true, queue `in_flight.is_some()`, root still `CounterCostRemovalPayment`, stack empty. After entry 2's decline: Battery has one counter left, life +1 (`applied_total` 1, not `total` 2 or the root's 3; order-independent), stack 1 with count 3, queue none, no rider frame. In the decline action's events: `EffectResolved{RemoveCounter}` < P0 life gain < `AbilityActivated{bottle}`. Ownership clear. Resolve, then +6. | **red** (activation placed after entry 1) |
| T4 | `a_prevented_child_removal_under_a_mill_cost_root_resumes_that_root` (control: unrelated outer root) | `MILL_COST_CONTROL`, library ≥ 2, Battery charge 1. Mill optional `execute = RemoveCounter(charge, Fixed 1, SpecificObject battery)`, RemoveCounter optional. Activate → Mill prompt → ACCEPT → child prompt → stage prevention → choose 0. | Reach: root is `ActivationMillPayment` at both prompts; queue `in_flight.is_some()` at the child prompt. After: queue none, charge 1, stack 1, `EffectResolved{RemoveCounter}` < `AbilityActivated{miller}`, ownership clear. Resolve, then life +1. | **red** (order, stale queue); 3b828ac red (stranded) |
| T5 | `a_prevented_damage_child_finishes_its_rider_before_the_cost_root` | Bottle 3. P1 planeswalker "Walker" loyalty 5 (as ~:1648). Execute = `DealDamage { amount: Fixed 1, target: SpecificObject(walker) }` `.sub_ability(GainLife Fixed 1)`. Activate → ACCEPT → loyalty prompt → stage prevention → choose 0. | Reach at the prompt: `parked_child_rider`, `active_counter_removals().is_none()`, root parked. After: loyalty 5, life +1, stack 1 with count 3, gain < `AbilityActivated{bottle}`, no rider frame. | **red** |
| T6 | `a_same_subject_child_removal_is_still_the_roots_child` | Bottle 3. Execute = `RemoveCounter(elixir, Fixed 3, SpecificObject(bottle))`. Activate → ACCEPT → prompt → stage prevention → choose 0. | Reach: queue `in_flight.removal` equals the root's `removal.in_flight.removal`. After: queue none, stack 1, count 3, `EffectResolved{RemoveCounter}` < `AbilityActivated`. | **red** |
| T7 | existing `a_prevented_cost_counter_removal_resumes_its_cost_root`, `a_prevented_damage_counter_removal_resumes_the_resolving_spell`, every R2 row | unchanged | unchanged | green (`Retired` / `Absent`) |
| T8 | `a_prevented_mana_cost_counter_removal_resumes_its_cursor` | Cove 3, pay 2, stage prevention, choose 0. | Reach: `cursor.counter_removal.is_some()`. After: blue 0, storage 3, ownership clear. | green |
| T9 | `a_child_under_a_mana_cost_root_paying_a_spell_settles_before_the_spell_resumes` (B1) | Divination in hand, empty pool, Cove 3 storage, Battery as in T3, `t3_child_execute`. `CastSpell { payment_mode: Manual }` → `ManaPayment`. Activate the Cove (`Answers::pay(2)`) → ACCEPT → entry 1 prompt → stage prevention → choose 0 → entry 2 → DECLINE. | Reach at both child prompts: `pending_cast.is_some()`, `parked_mana_cursor(..).counter_removal.is_some()`, `parked_child_rider`. After the decline: `EffectResolved{RemoveCounter}` < P0 life gain < first `ManaAdded{source_id: cove}`. No `SpellCast`/`StackPushed` for Divination in that action. `waiting_for` is `ManaPayment` for Divination. Blue in pool is 2, or Divination's payment has absorbed 2 (assert whichever P5 shows, with the other as reach). Storage 1, queue none. | **red** at the base (the mana root resumes after entry 1; the queue is stale). **Red with D1–D5 only** (1371 finishes the outer cast before the mana root). Green with D6 |
| T10 | `a_child_under_a_mana_root_paying_inside_a_paused_post_effect_settles_first` (T-C: the nested configuration through public actions) | Battery as in T3. P0 artifact "Drawer" with `DRAW_CONTROL`, library ≥ 2, Cove 3 storage, empty pool. Two Steward replacements via `install_replacement_on_steward`: an optional `Draw` replacement whose execute is `GainLife { amount: Fixed 1, player: Controller }` with `unless_pay = Some(UnlessPayModifier { cost: Mana({2}), payer: Controller })`, and the RemoveCounter optional replacement with `t3_child_execute`. Activate Drawer → pass priority to resolve → draw replacement prompt → ACCEPT → `UnlessPayment` → activate the Cove (`Answers::pay(2)`) → ACCEPT → entry 1 → stage prevention → choose 0 → entry 2 → DECLINE → `PayUnlessCost { pay: true }`. | **Reach (P6) at `UnlessPayment`:** `state.resolution_stack.active_post_replacement_or_paired_parent()` is `Some`, with resident status `DrainStatus::Paused`. At entry 1: the parked cursor's `counter_removal.replacement_subtree.enclosing.is_some_and(\|e\| e.frame.is_some())` (the nested shape), plus `parked_child_rider`. After the decline: `EffectResolved{RemoveCounter}` < P0 rider life gain < first `ManaAdded{source_id: cove}`. `waiting_for` is `UnlessPayment`. Storage 1, queue none. After paying: no unless-gain, `resolution_stack.is_empty()`, ownership clear. | **red** at the base (T3 mechanism: the mana root resumes after entry 1). Exactness against `len`-only is shown by U2/U3, not by T10 (see T-C) |
| U1 | `engine.rs` `cost_move_drain_priority_boundary_tests::a_counter_cost_root_waits_for_its_subtree_at_every_boundary` | Root `CounterCostRemovalPayment { pending: None, removal: ..boundary at(&empty stack).. }`. Push a `CounterRemovals` frame, then drain at each boundary. Pop it, then drain at `PriorityBoundary`. | With the frame: `Ok(None)`, root parked, no events (×3). Without: `Ok(Some(_))`, root taken (`pending: None` → Priority, `casting_costs.rs` 5015–5019): the paired positive. The `CounterAdditionUnlessPayment` row stays green. | n/a at base (new field). Revert-check: drop the D3 gate → negatives red; drop the admission → positive red |
| U2 | `types/game_state.rs` `#[cfg(test)]`: `a_child_nested_into_the_enclosing_frame_keeps_the_subtree_live` | `install_post_replacement_drain(PostReplacementDrain::ready(..), Replace)`; `begin_post_replacement_dispatch()` (stamps the frame, `Dispatching`); `pause_post_replacement_dispatch(handle)`. Record `b = ReplacementSubtreeBoundary::at(..)`. Install a second drain (nests; `len` unchanged), then **begin it explicitly** to obtain its handle. | Reach: `b.enclosing.is_some_and(\|e\| e.frame.is_some() && e.drains == 1)`. `!b.is_retired(..)` with `resolution_stack.len() == b.resolution_depth` (count 2 > 1). After `finish_post_replacement_dispatch(nested_handle)`: `b.is_retired(..)`. A `len`-only check would read retired at the first assertion. | n/a (new type). Revert-check: replace `is_retired` by `len <= depth` → red |
| U3 | same module: `a_nested_child_stays_live_after_the_frame_above_its_parent_is_consumed` (T-A) | As U2 up to the pause; then `push_ability_continuation(C)` with C a trivial `AbilityContinuationFrame { pending, choose_zone_trigger_context: None }`, so P is the paired parent at top-1. Record `b`. Install a second drain (nests into P), begin it, then `take_active_ability_continuation()` to consume C. | Reach: `resolution_stack.len() == b.resolution_depth - 1` and P's count 2. `!b.is_retired(..)` (r2's positional `Less → true` read retired here). After finishing the nested dispatch: `b.is_retired(..)`. | n/a (new type). Revert-check: the positional fallback for `Some(id)` → red |

Constraints:
- Each negative is paired with a positive in the same row.
- Ordering assertions are on the `ActionResult.events` of the single deciding `runner.act`.

---

## Steps (executor)

1. `types/game_state.rs`:
   - `ReplacementSubtreeBoundary` and `EnclosingPostReplacementFrame` with their methods;
   - `PostReplacementDrainStack::len`;
   - the `PausedCounterCostRemoval` field (required);
   - `paused_counter_removal`;
   - `CounterCostRootSubtree` and `counter_cost_root_subtree`;
   - U2 and U3.
2. `types/resolution.rs`: `ResolutionStack::post_replacement_frame_drain_count` (D1).
3. `game/costs.rs`: record the boundary (D1). No assertion.
4. `game/casting_costs.rs` 5020 and `game/mana_abilities.rs` 4138: compiler-forced destructures.
5. `game/engine.rs`:
   - the D3 gate, admission and docs;
   - D5: rename the body to `resume_pending_continuation_pass`, add the looping `resume_pending_continuation_if_priority`, and add `SubtreePosition`;
   - U1;
   - the module-doc update.
6. `game/engine_replacement.rs`:
   - D4(a) extraction with the CR fix;
   - D4(b) and D4(c);
   - D6(a) and D6(b);
   - update `pending_was_counter_removal`'s comment (228–231) to name D4.

   The 1415 call is unchanged.
7. `game/engine_payment_choices.rs`: comment only (2526–2530).
8. Protocol comment lines (Serialized surface).
9. Tests: the harness refactor, then T1–T6, T8, T9 and T10.

## Verification

1. `cargo fmt --all`.
2. With Tilt up: `./scripts/tilt-wait.sh clippy test-engine`. Otherwise, with an isolated target and only if disk allows:
   ```
   CARGO_TARGET_DIR=<worktree>/target-phase-d cargo test -p phase-engine --test integration cost_removed_counter_amount
   cargo test -p phase-engine --test integration activation_cost_work_census activation_cost_target_settlement
   cargo test -p phase-engine --lib cost_move_drain_priority_boundary_tests replacement_subtree
   cargo clippy -p phase-engine --all-targets -- -D warnings
   ```
3. Red evidence:
   - At 4483f4b, before the source diff, run T1, T3, T4, T5, T6, T9 and T10. Record each failure and its reason. Expect T2, T7 and T8 green.
   - If T10's P6 reach fails, drop T10 and record why in the run notes. U2 and U3 then stand as the nested-configuration evidence (T-C).
   - Then apply D1–D5 without D6 and confirm T9 red. That is the B1 discriminator.
   - U1, U2 and U3 get the revert-checks named in their rows.
4. Full `test-engine` and `wasm` before marking fixed. D5 changes the body of the shared common boundary, so the whole engine suite is the regression net for its inertness claim. `check-frontend` covers only the TS comment edits.

## Sizing (phase-fit)

**Units: 1.** "A counter-cost root resumes only once its replacement subtree retires, and before any outer payment it is paying for."
- It has one seam (D1/D2), consumed at every resume site (D3 gate and admission, D4 Prevented, D5 common boundary, D6 Execute).
- No leg is coherent alone:
  - the admission without the gate resumes above a live subtree;
  - the gate without D6 inverts the nested mana route (B1);
  - D4 without D3 and D5 has no resumer.
- Discriminating tests: T1, T3, T4, T5, T6, T9, T10, U1, U2, U3.

**Scope paths (counted):**
- `crates/engine/src/types/game_state.rs`
- `crates/engine/src/types/resolution.rs` (one read-only accessor)
- `crates/engine/src/game/costs.rs`
- `crates/engine/src/game/engine.rs`
- `crates/engine/src/game/engine_replacement.rs`
- `crates/engine/src/game/casting_costs.rs` (compiler-forced)
- `crates/engine/src/game/mana_abilities.rs` (compiler-forced)
- `crates/engine/src/game/engine_payment_choices.rs` (comment only)
- `crates/engine/tests/integration/cost_removed_counter_amount.rs`

That is 8 production paths plus 1 test path = 9. The protocol comment surfaces are excluded.

**Verdict:** T1 = 1, so the conjunction does not fire.

## Pattern coverage

- **Class:** every activation or mana-ability counter-removal cost (the PR's printed set: Essence Bottle, Saprazzan Cove, Ooze Flux, Geometric Nexus, Magosi and the rest) under any replacement whose post-effect does further work: a choice, an effect removal, damage or a rider. That includes:
  - a mana ability paying an outer cast;
  - a mana ability paying a prompt inside a paused post-effect.
- The class is covered at every boundary that calls the common priority boundary.
- `Absent` covers every rootless or other-rooted prevented removal (damage, CR 120.3c).
- The RemoveCounter-replacement leg is typed-input only today.

---

## Review r1 dispositions

| Finding | Resolution |
|---|---|
| **B1** (the Execute-arm gate lets 1371 finish the outer cast before the mana root) | **D6:** (a) re-offer the delivery drain after 1360 once the subtree has retired; (b) gate 1371 on `!= Unfinished`, leaving the rest to the 1415 common boundary. The order is justified in D6. New row **T9** (Saprazzan Cove paying Divination's `ManaPayment` with the T3-shaped child), red at the base and red with D1–D5 only. |
| **B2** (`len`-only boundary inexact under a PostReplacement frame; D5(a) regression) | P1 restated with its precondition (Step 2). **Option (a), exact** (D1), with the choice justified: (b)'s assertion would fire on a reachable configuration. In r3 the enclosing frame is keyed by identity (r2 T-A). Rows **U2**, **U3**, **T10**. **D5(a) and D5(b) dropped.** |
| **X1** (rebase coordinates) | PHASE_BASE = 4483f4b. Every anchor is cited at 4483f4b. The #9728 delta is noted. |
| **X2** (protocol numbers) | Serialized surface cites v151 / wire 133, with lobby-broker :63, server-core ~3476, ws-adapter ~218 and protocol.ts ~109–115. No further bump. |
| **T1** (residual 2 wording) | Residual 2 is a remaining defect: priority is surfaced mid-activation; follow-up. Residual 3 covers no-progress subtrees. |
| **T2** (error contract at the new admission) | Residual 4 states the `Err` sources, the `.expect` panic path at `PriorityBoundary`, its parity with `ManaAbilityPayment`, reachability (synthetic only) and the follow-up. |
| **T3** (CR citations) | D2 cites CR 601.2h + 616.1. The extracted tail drops 614.12a and 118.12 in favour of 601.2h/602.2b/605.3b/616.1. |
| **T4** (`serde(default)`) | The field is required, with no default (D1, Serialized surface). |
| **T5** (single-pass assumption) | **D5**: the common boundary loops while the subtree is unfinished and the position strictly decreases. In r3 the loop is inside `resume_pending_continuation_if_priority` (r2 T-D). |
| **T6** (rider reach) | `parked_child_rider` frame-iteration reach at both T3 prompts and in T5, T9 and T10. The P4 fallback is kept. |

## Review r2 dispositions

| Finding | Resolution |
|---|---|
| **X3** (D5 loop termination not well-founded) | Adopted the review's replacement code verbatim: the exit condition `!after.is_strictly_inside(&before)` and the lexicographic `is_strictly_inside` on (depth, enclosing drains). It now sits on a private `SubtreePosition` measure in `engine.rs`, because `ReplacementSubtreeBoundary` is identity-keyed after T-A and has no total order. The termination paragraph is the review's text. Residual 3 owns the stop case. |
| **T-D** (M-F1 route still makes one pass) | **The loop moved inside `resume_pending_continuation_if_priority`** (D5). Today's body becomes the private `resume_pending_continuation_pass`. All ~50 callers are covered, including `finish_with_continuation` and its 13 `.expect` siblings, with no parallel entry point. r2's helper is deleted, and D4(b), D4(c) and 1415 call the boundary directly. Soundness is argued in D5: inert unless a counter-cost root is `Unfinished`; each repeat is a full call of today's body; the existing re-entrancy (random-discard root → `finish_successful_unless_payment`) gets a bounded loop per invocation; `Err` propagation is unchanged. Verification step 4 runs the full engine suite for the inertness claim. |
| **T-A** (`Less → true` misreads a nested child after C is consumed) | **Keyed by `PostReplacementFrameId`** (D1). This is cheap: one `Option<EnclosingPostReplacementFrame { frame: Option<PostReplacementFrameId>, drains }>` field and one read-only, identity-addressed accessor, `post_replacement_frame_drain_count`, over the existing private `post_replacement_frame` / `FrameVec::by_id`. It is exact for every frame dispatched in this build (stamped at first dispatch, the only production dispatch path). `is_retired` is now `len ≤ depth ∧ enclosing count by identity ≤ recorded`, so the `Less` case no longer short-circuits. New row **U3** reproduces the `[.., P, C]` shape. An unstamped frame (pre-identity payload) falls back to positional: residual 7. The early-C drain itself remains the review's pre-existing finding. |
| **T-B** (unsupported `debug_assert!`) | **Assertion removed.** Replaced by named residual 6 (a `Ready` enclosing resident is evicted by `Replace`, so the count does not rise and the boundary reads `Retired` early, in release too), with its trigger and a follow-up at the eviction site. |
| **T-C** (nested configuration has only U2) | **Flow named and row added:** **T10**. A synthetic `Draw` replacement's post-effect with `unless_pay` pauses at `UnlessPayment`, and P0 activates Saprazzan Cove there (mana abilities are legal at `UnlessPayment`, `engine.rs` 13424–13456, CR 118.12), so the Cove's removal parks nested in that frame. Its first assertion is the P6 reach guard. T10 is red at the base via the T3 mechanism. It shows the configuration is reachable (supporting D1's rejection of option (b)) and that the flow settles in order. It is not claimed to discriminate exactness against `len`-only: retiring a completed paused dispatch emits no event, so U2/U3 are the discriminating evidence at helper level, per check 9's helper-only rule. If P6's reach fails, the executor drops T10 and records U2/U3 as the accepted evidence for that reason (Verification step 3). |
| **T-E** (header coordinates) | The header now says every anchor, including `game_state.rs` and `resolution.rs`, is cited at 4483f4b, with no offset rule. It lists `perpetual.rs` (and the end-phase files) as changed under `effects/`. |
| **T-F** (#9728 final-part interaction) | **Added to residual 3.** At the action boundary, `owed_spell_final_part_is_due` → `resolution_instructions_are_done` (`engine.rs` 9022–9029) does not consult `pending_cost_move_resume`, so a mid-resolution mana root left parked by a no-progress stop could see a resolving instant's final part run under it. The cast route is protected by `pending_cast`. T-D narrows this to the no-progress case but does not close it. Follow-up named: gate `resolution_instructions_are_done` on `withholds_priority`. |

---

## Phase D amendment (post-implementation)

Implemented as 59247b7 on `engine/cost-removed-counters`; every anchor below is cited at 59247b7. Read-only planning, no builds. Implementation deviations accepted as recorded by the coordinator:
- T5 uses `Effect::DamageAll { target: SpecificObject(walker) }`, because a template `DealDamage` binds no target.
- `removal_queue(state)` iterates the whole stack (`cost_removed_counter_amount.rs` 1801).
- T10's `Draw` replacement carries `DrawReplacementScope::InstructionCount`.

### A. T4: a batch parked above the child's queue

**Diagnosis (traced at 59247b7):**
1. The mill cost proposes `ProposedEvent::Mill` (`casting_costs.rs` 6481–6533). The optional Mill replacement's ACCEPT goes through the Execute arm's Mill arm (`engine_replacement.rs` 767–775) → `apply_mill_after_replacement` (`effects/mill.rs` 110–186) → `move_objects_simultaneously_then` (`zone_pipeline.rs` 1223) → `deliver_batch` (1344).
2. The milled card's move drains the mill replacement's post-effect in its **delivery tail** (`zone_pipeline.rs` 2133–2180, `PostReplacementDrainOwner::DeliveryTail`). That post-effect is the child `RemoveCounter`. It pushes `CounterRemovals` and pauses on the Steward's RemoveCounter choice.
3. `move_object_with_terminal` returns `NeedsChoice`. `deliver_batch` then stashes the batch owner with `BatchDeliveryParking::NewChild` (`stash_batch_tail`, 1493–1497), **after** the child pushed its queue. `move_objects_simultaneously_then` hangs the `MilledDeliveryComplete` completion on that frame (1265–1275). The result is `[PostReplacement, CounterRemovals, BatchDelivery]`.
4. That is the engine's established shape for a batch whose member's delivery tail paused in a post-effect child. `resume_resolution_frames`' `BatchDelivery` arm (`effects/mod.rs` 1685–1695) drains the batch first and then "continue[s] that child" it exposes.
5. The Prevented arm's counter-removal block (`engine_replacement.rs` 1650–1694) always returns. Its queue drain is top-only (`counters.rs` 3447), so it is a no-op under the batch. The Absent arm then resumes the mill root over both frames. The arm's own batch block (1707–1713), which would have settled the batch, is **shadowed** by that early return. At the base the counter-removal condition fell through to the batch block, which drained the batch and returned Priority with the root parked, so T4 was red there too.
6. The Execute arm has the same order hazard. The queue drain (1153–1160) runs before the batch drain (1187–1194), so a DECLINE of the child entry settles the batch, exposes the queue, and the delivery cost drain (1345) resumes the mill root before the exposed queue finishes (row T4b).

**P3 correction.** P3 assumed the child queue is the top frame when its removal is prevented. That holds for counter-cost roots (the payer proposes the removal directly, not inside a move). It fails when the removal is proposed inside a batch member's delivery tail, as under the mill root.

**CR order (decided):**
- CR 614.6 (verified): "If an event is replaced, it never happens. A modified event occurs instead." The accepted Mill replacement's modified event is the mill plus its post-effect, the child removal. The child removal, including its prevented entry (CR 616.1 / 614.6) and its queue finalization, is therefore part of the mill cost's payment.
- CR 601.2h + CR 602.2b (verified): the activation continues only after that cost is paid. So the root resumes only after **both** the batch (`Milled` events, logical zone-change trigger collection) and the child queue have settled.
- Between the batch and the child, the engine's existing convention applies: the batch settles first and exposes the child (`effects/mod.rs` 1685–1695). This changes no rules outcome, because settlement only collects triggers, and CR 603.2/603.3 (verified) put triggered abilities on the stack the next time a player would receive priority, after both.
- T4/T4b assert root-after-both. They deliberately do not pin `Milled` against `EffectResolved{RemoveCounter}`.

**D7. The fix (two edits, `crates/engine/src/game/engine_replacement.rs` only):**

(a) Prevented arm. In the `if pending_was_counter_removal {` block (1658), immediately after the `state.waiting_for = WaitingFor::Priority { .. }` reset and **before** `match state.counter_cost_root_subtree()`, insert:
```rust
// CR 614.6 + CR 616.1: a batch zone move whose member's delivery-tail
// post-effect proposed this removal was parked above the removal's own work on
// the way out of that pause (`zone_pipeline::deliver_batch`, `NewChild`). It is
// part of the same modified event: settle it first, as `resume_resolution_frames`'
// BatchDelivery arm does, which exposes the removal's queue for the owner below.
if state.active_batch_delivery().is_some() {
    crate::game::zone_pipeline::drain_pending_batch_deliveries(state, events);
    if !matches!(state.waiting_for, WaitingFor::Priority { .. }) {
        return Ok(state.waiting_for.clone());
    }
}
```
- **Why a top `BatchDelivery` here is always this removal's enclosing batch.** The live prompt was this `RemoveCounter` choice. A batch frame is pushed only on the way out of the pause that raised the live prompt: `stash_batch_tail` in `deliver_batch`'s `NeedsChoice` arm, or `defer_completion_on_pause` for a single-object pause. A batch parked on an earlier, different prompt would have that prompt as the live one. So no gate on queue presence is needed. The same reasoning covers a delivery-tail child whose removal is not queued (damage, a resolution-time cost): the batch is still that child's enclosing owner. This restores what the shadowed batch block (1707–1713) did at the base.
- The three subtree arms are unchanged after it:
  - **Absent** (T4): queue → continuation → root.
  - **Unfinished:** queue → common boundary.
  - **Retired:** cannot see a batch, because a counter-cost removal is proposed by the payer, not inside a move.

(b) Execute arm. Directly after the batch block (1187–1194), insert:
```rust
// CR 614.6 + CR 616.1: settling a batch parked above its member's delivery-tail
// child exposes that child's counter-removal queue. Finish it here, in the same
// order as the queue drain above, before any parked cost root resumes below.
if matches!(waiting_for, WaitingFor::Priority { .. })
    && state.active_counter_removals().is_some()
{
    effects::counters::drain_pending_counter_removals(state, events);
    if !matches!(state.waiting_for, WaitingFor::Priority { .. }) {
        waiting_for = state.waiting_for.clone();
    }
}
```
This only fires when a queue is the top frame after the batch drain. Every flow where the queue was already drained at 1153–1160, or where none exists, is unchanged.

**Rejected alternatives:**
- *Locate the child queue by identity under the batch and drain it first.* That finishes the child while the batch is still parked, which inverts `resume_resolution_frames`' shipped order for the same shape. `ResolutionStack` also deliberately exposes no positional mutation (`resolution.rs` 757–767), so it would need a new buried-frame mutator.
- *Insert the batch below the child in `deliver_batch`* (`insert_parent_at_child_boundary`, the ChangeZone precedent at `resolution.rs` 1382). That changes the parking of every batch whose delivery-tail post-effect pauses, against the `BatchDelivery` arm's shipped convention and its existing rows. Not minimal, and no rules outcome requires it.
- *Is the batch the root's own work, to wait until after the root?* No. The batch is the mill event itself, which is the cost being paid (CR 601.2h). It must settle before the root, not after it.

**Residual 1, refined.** Non-counter roots still carry no subtree boundary. After D7 they resume once the batch, the queue and the continuation drain have run. A paused `PostReplacement` dispatch of their own replacement may still retire after the root (unchanged, follow-up).

**Test edits (`crates/engine/tests/integration/cost_removed_counter_amount.rs`):**
- **T4** (`a_prevented_child_removal_under_a_mill_cost_root_resumes_that_root`, 2067): remove the `#[ignore]`.
  - Add a reach at the child prompt: `runner.state().resolution_stack.last()` is `ResolutionFrame::BatchDelivery(_)` (the shape D7(a) handles). Keep the existing `removal_queue(..).in_flight` reach.
  - In the prevention action's events, add `event_index(.., Milled)` < `activated` beside the existing `resolved < activated`. Do not order `Milled` against `resolved`.
  - After the existing `drive(.., priority_with_empty_stack, ..)`, add `assert!(runner.state().resolution_stack.is_empty())` paired with the existing life +1.
  - Expected: red at 59247b7, green with D7(a). Revert-check: delete D7(a) → red.
- **T4b** (new) `a_declined_child_removal_under_a_mill_cost_root_resumes_that_root`: T4's setup, but answer the child prompt with `ChooseReplacement(DECLINE)` (index 1, as in T3).
  - Same reach as T4.
  - After the decline: battery charge 0 (the removal happened), `removal_queue` none, stack 1, `resolved < activated`, `Milled < activated`, ownership clear.
  - After driving to an empty stack: life +1 and `resolution_stack.is_empty()`.
  - Expected: **(unprobed)** red at 59247b7 (1345 resumes the mill root before the exposed queue drains at 1415, so `resolved < activated` fails), green with D7(b).

### B. T10 panics on a paired multi-draw child (pre-existing)

**Confirmed out of scope as a defect.** The validator (`debug_assert_runtime_resolution_invariants`, `resolution.rs` 4876–4882, run after every public action) rejects a `MultiDraw` child of a draw's `PostReplacement` drain that is buried below frames other than the active direct-choice owner. Activating a mana ability whose cost raises frames (the Cove's child queue and rider) at a draw post-effect's `UnlessPayment` buries it. That reproduces at 4483f4b and at 59247b7, independent of D1–D7.

**Residual 8 (new).** A mana ability whose payment raises resolution frames, activated at an `UnlessPayment` raised by a draw replacement's post-effect, buries the paired `MultiDraw` child and trips the CR 616.1g pairing invariant. Only a synthetic counter-removal replacement gets a mana payment to raise frames; a plain land tap raises none. Follow-up: owner of the draw-sequence pairing (`effects/draw.rs` / `resolution.rs` pairing validation).

**T10 re-flow (proposed), same nested configuration without a draw:**
- Rename to `a_child_under_a_mana_root_paying_inside_a_paused_life_gain_post_effect_settles_first`.
- Replace `DRAW_CONTROL` with a labelled synthetic `GAIN_CONTROL = "{T}: You gain 1 life."` on artifact "Gainer".
- Replace the Draw replacement with `optional_replacement(ReplacementEvent::GainLife, Some(lose))`, where `lose = AbilityDefinition::new(AbilityKind::Spell, Effect::LoseLife { amount: Fixed 1, target: None }).unless_pay(UnlessPayModifier { cost: AbilityCost::Mana { cost: ManaCost::generic(2) }, payer: TargetFilter::Controller })`. The post-effect is a life loss, so the GainLife replacement cannot re-apply to it.
- The RemoveCounter replacement with `t3_child_execute(battery)`, the Cove, the Battery and every remaining step and assertion are unchanged from the 59247b7 row, including both P6 reaches (`UnlessPayment`, and the resident drain `Paused` in the active frame).
- Final assertions: life is `life_at_unless` + rider (+1) and no −1 (the unless cost was paid). `resolution_stack.is_empty()` after paying.
- **(unprobed)** A life-gain post-effect has no paired `MultiDraw` child, so the T3-shaped `[PostReplacement(P, nested child), rider, queue]` stack is the one T3 already validates. If the re-flow still trips validation, or P6 fails, keep the row `#[ignore]`d citing residual 8 and record U2/U3 as the accepted nested-configuration evidence (T-C).

### C. U3: a case that discriminates identity from position

U3 as written cannot catch the "positional fallback for `Some(id)`" mutation: once C is consumed, P is the top again, so the positional read finds the same frame. Add **U3b** in the same module (`types/game_state.rs` `#[cfg(test)]`): `a_nested_child_is_read_in_its_own_frame_not_the_active_one`.
1. Build U3's state up to "consume C": P (stamped, one `Paused` drain), C pushed, `b` recorded (`enclosing == Some { frame: Some(P.id), drains: 1 }`, depth N), nested child drain begun in P (P count 2), C taken (len N−1).
2. Push a fresh, unstamped `PostReplacement` frame P2 directly through `ResolutionStack::push_post_replacement` (`resolution.rs` 3091), holding one `Ready` drain. The length is N again, and P2 is now the active frame.
3. Assert:
   - reach: `resolution_stack.len() == b.resolution_depth`, `active_post_replacement_or_paired_parent().map(len) == Some(1)`, and P's count by id is 2;
   - `!b.is_retired(..)`.

   The positional mutation reads P2's 1 ≤ 1 → retired, so it fails here. The identity read sees P's 2 > 1.
4. Pop P2 (`ResolutionStack::take_active_post_replacement`, `resolution.rs` 3080) and finish the nested dispatch. Then `b.is_retired(..)` (the paired positive).
5. This test exercises `ResolutionStack` directly, so the public-action validator does not run. The adjacent-frame shape is a type-level contract fixture, not a claimed production shape.

Revert-check: `EnclosingPostReplacementFrame::is_retired` with `Some(id)` routed to the positional read → U3b red, U3 green.

### Scope and sizing after the amendment

- Production: `engine_replacement.rs` only (already counted).
- Tests: T4 un-ignored and tightened; T4b new; T10 re-flowed (or left ignored under residual 8); U3b new.
- Paths stay at 9; units stay at 1.

### Verification for the amendment

- `cargo fmt --all`.
- Then, through Tilt or the isolated target: `cost_removed_counter_amount` (T4, T4b, T10 re-flow) and the lib tests holding U2/U3/U3b.
- Run full `test-engine` because the Execute arm changed. The existing batch rows (`cost_zone_pipeline`, the mill/RIP double-pause rows) are the regression net for D7(b)'s "inert unless a queue is exposed" claim.
- Red evidence:
  - T4 red at 59247b7 and green with D7(a);
  - T4b red at 59247b7 and green with D7(b);
  - U3b red under the positional mutation.
