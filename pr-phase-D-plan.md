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

---

## Phase D impl-review r1 fixes

Sequenced **on top of** the amendment as revised by A1/A2 (the kg-dev executor's next commit). Anchors are cited at 59247b7; the amendment touches only `engine_replacement.rs` 1187–1194 and 1658–1662 plus tests, so these anchors carry over unchanged. Read-only planning, no builds.

### F1. The subtree drain does not stop at the root's boundary

**Confirmed at 59247b7.** The reviewer's three traces hold. The mechanism is that nothing that drains frames knows that a parked counter-cost root became due mid-drain:
- `drain_pending_continuation` (`effects/mod.rs` 1209–1436):
  - its leading counter drains (1210–1212);
  - its post-chain retire of the active paused dispatch (1386–1390), which retires an **outer** paused dispatch when the child's drain is already gone;
  - its cascade into the exposed continuation (1394–1399);
  - its later vote, repeat-for, choose-one-of and repeat-until steps (1404–1430).
- `resume_resolution_frames` (1655): the `PostReplacement` arm retires the top `Paused` dispatch (1782–1792) and drains an exposed continuation (1816–1827).
- The common-boundary pass (`engine.rs` 8939): it runs `drain_pending_continuation` and then `resume_resolution_frames` before its cost-move drain.

The `DeferredLifeCostResume` seam guards only the *entry* of the first two steps (`len > boundary`, `engine.rs` 8953–8961). Its own mid-cascade gap is pre-existing and unchanged here.

**Rule (CR 601.2h + CR 602.2b + CR 605.3b + CR 608.2c, verified):** a cost being paid is the innermost unfinished work. A mana ability "resolves immediately after it is activated" (CR 605.3b), and the paid activation or outer payment resumes before any later instruction is followed (CR 608.2c). So once the root's replacement subtree has retired, **no** further resolution work may run before the root resumes.

**One predicate, three stops, one resume:**

1. `crates/engine/src/types/game_state.rs`, next to `counter_cost_root_subtree` (8436):
   ```rust
   /// CR 601.2h + CR 605.3b + CR 616.1: a parked counter-cost root whose
   /// replacement subtree has retired is due: it is the innermost unfinished
   /// work and resumes before any other resolution work.
   pub(crate) fn counter_cost_root_is_due(&self) -> bool {
       self.counter_cost_root_subtree() == CounterCostRootSubtree::Retired
   }
   ```

2. `crates/engine/src/game/effects/mod.rs::drain_pending_continuation` (1209), three gates with one shared comment:
   ```rust
   // CR 601.2h + CR 605.3b + CR 608.2c: a parked counter-cost root whose
   // replacement subtree has retired resumes before any further resolution
   // work; every frame left on top of it is outer work.
   ```
   - (a) The first statement of the body, before the leading counter drains: `if state.counter_cost_root_is_due() { return; }`.
   - (b) The post-chain block at 1386 becomes `if !waits_for_resolution_choice(&state.waiting_for) && !state.counter_cost_root_is_due() {`. If the subtree retired with the continuation that just ran, the top paused dispatch belongs to an outer owner and must stay paused. While the subtree is unfinished, the paused dispatch on top is the child's own: either its own frame above the depth, or nested in the enclosing frame and raising that frame's count.
   - (c) Directly after that `if` block closes (before the vote-ballot step at ~1404): `if state.counter_cost_root_is_due() { return; }`. The child's dispatch retiring at (b) is exactly when the root becomes due. This stops the cascade at 1394–1399, whose recursive call would also stop at (a), and the later loop steps.

   The skipped tail cleanups (`clear_post_replacement_token_choice_seed_if_resolution_drained`, `clear_return_result_frames_if_idle`) are idle-guarded. The next boundary pass runs them once the root has resumed.

3. `crates/engine/src/game/effects/mod.rs::resume_resolution_frames` (1655): directly after the `if !matches!(state.waiting_for, WaitingFor::Priority { .. }) { .. return; }` block, add `if state.counter_cost_root_is_due() { return; }` with the same comment. This covers the `PostReplacement` arm retiring P's outer dispatch (trace 2) and every other frame arm, for every caller.

4. `crates/engine/src/game/engine.rs`, next to `resume_pending_continuation_pass` (8939):
   ```rust
   /// CR 601.2h + CR 605.3b + CR 608.2c + CR 616.1: once a parked counter-cost
   /// root's replacement subtree has retired, resume that root at this boundary
   /// before any further frame, liminal-entry or phase-transition work.
   fn resume_due_counter_cost_root(
       state: &mut GameState,
       events: &mut Vec<GameEvent>,
   ) -> Result<(), EngineError> {
       if matches!(state.waiting_for, WaitingFor::Priority { .. })
           && state.counter_cost_root_is_due()
       {
           super::life_safety::observe_boundary_carrier(state);
           let _ = drain_pending_cost_move_resume(
               state,
               events,
               CostMoveDrainBoundary::PriorityBoundary,
           )?;
       }
       Ok(())
   }
   ```
   Call it twice inside `resume_pending_continuation_pass`:
   - right after the first `drain_pending_continuation` block (8953–8956);
   - right after the `resume_resolution_frames` block (8957–8961).

   The existing PriorityBoundary cost drain later in the pass is unchanged and still serves the other admitted roots. After a resumed mana root, `waiting_for` is the outer prompt, so every later step of the pass and the D5 loop's first exit condition stop. After a resumed stack root, `waiting_for` is Priority with the root gone, so the remaining outer work runs after it, as CR 608.2c orders.

**Why this is sufficient:**
- **Trace 1** (Execute/DECLINE, `[P, C]`, nested):
  1. 1153 finishes the queue.
  2. 1345: `Unfinished` → skip.
  3. 1360 `drain_pending_continuation` runs R. Gate (b) passes (the child's drain is still nested in P), so the child's paused dispatch retires and the root becomes due. Gate (c) returns before the cascade.
  4. D6(a) resumes the Cove at `ReplacementDelivered`, which re-offers `UnlessPayment`. C and P's outer `Paused` drain remain.
- **Trace 2** (last entry prevented, `[P]`):
  1. D4(b) `Unfinished`: queue → common boundary.
  2. In the pass, `drain_pending_continuation` runs R and retires the child's dispatch; gate (c) returns.
  3. `resume_due_counter_cost_root` resumes the Cove, which returns `UnlessPayment`.
  4. `resume_resolution_frames` is never reached in this pass. The `Priority` guard also blocks its own entry, and gate 3 would too. P's outer dispatch stays `Paused`.
- **Non-nested outer tail below the boundary** (a resolving ability's `sub_ability` parked under its `UnlessPayment`): the child's frames sit above the depth. When they retire, gates (a)–(c) and 3 stop, and the pass helper resumes the root.
- **Recursion:** the `Queued` re-drain at 1381–1384 calls `drain_pending_continuation`/`resume_resolution_frames` recursively. Both stop at their entry gates, and control unwinds to a site that resumes the root.

**Is the same stop needed at the Execute arm's 1360 drain and in D4(b)'s `Unfinished` arm?** Yes, and both get it from the gates above with no new code at those sites:
- **1360** is a `drain_pending_continuation` call, stopped by (b)/(c). D6(a), which follows it, is the resume.
- **D4(b) `Unfinished`** reaches the boundary only through `resume_pending_continuation_if_priority` → the pass, which stops at (a)–(c)/3 and resumes through `resume_due_counter_cost_root`. Its direct `drain_pending_counter_removals` drains only the child's own queue: the subtree is `Unfinished` there by construction, and the queue finalization runs no outer work.
- **D4(c)** is the same as D4(b).
- **The Execute arm's 1276–1296 pre-delivery drain** now stops at (a) when the root is due at entry. That happens for a stack-route root whose own removal was just delivered, so its continuation now drains at 1360 after 1345 resumes the root. That is root-first, matching the mana route's existing skip there. It is inert for every existing row, which activate at depth 0 with nothing parked.

**Why this is minimal:**
- The stops sit in the two functions every frame drain goes through, so no caller list has to be audited or kept in sync.
- The resume sits only in the common-boundary pass, plus D6(a), which exists already.
- With no counter-cost root, or an `Unfinished` one, every gate is inert. The predicate is one comparison on the existing typed seam.
- A single check between the two pass steps (the reviewer's first suggestion) would not stop the cascade *inside* `drain_pending_continuation` (traces 1 and 2), so gates (b)/(c) are required. Gate 3 is what keeps a non-pass caller of `resume_resolution_frames` from doing the same.

**Comment fixes:**
- **D6(a)** (`engine_replacement.rs` 1374–1379): "resumes as soon as the child's rider has retired that subtree" becomes "the continuation drain above stops at the moment the subtree retires (`counter_cost_root_is_due`), and the root resumes here, before any outer frame work and before any outer cast or activation it is paying for".
- **D5's doc** (`engine.rs` 8890): add "a due root resumes mid-pass through `resume_due_counter_cost_root`".

~~New residual 9~~: deleted (fix-design review FD1). Its claim that "every site where a subtree can retire is followed by a resume" is false:
- the resolution `PayCost{TapCreatures}` handler (`engine.rs` 12996–13006);
- the ChooseMana `ResolvingEffect` handler (`engine.rs` 13175 → `effects/mana.rs` 431–432);
- the Prevented counter-move block (`engine_replacement.rs` 1640–1648) and residual 2's other blocks.

All of these drain directly and never reach the common boundary. Replaced by the action-boundary authority below.

### F1, part 2 (FD1): one resume authority at the action boundary

**Problem.** A handler that drains directly and returns `Priority` leaves a due root parked. Under the new gates (a) and 3, every later `drain_pending_continuation`/`resume_resolution_frames` is then inert until something resumes the root. That includes the next action's stack resolution, because `pass_priority_once_with_pipeline` runs `handle_priority_pass` (`engine.rs` 9461) before its boundary (9506).

**Authority.** In `crates/engine/src/game/engine.rs`, next to `owed_spell_final_part_is_due` (1704):
```rust
/// CR 601.2h + CR 602.2b + CR 605.3b + CR 608.2c: the action-boundary resume
/// authority for a parked counter-cost root whose replacement subtree retired
/// during this action. A handler may finish the subtree by draining directly
/// (a resolution-time cost choice, a mana-colour choice, a Prevented-arm child
/// block) without reaching the common boundary; the root is still the innermost
/// unfinished work and resumes before the action returns. Inert unless a root is
/// due under a Priority result.
fn resume_due_counter_cost_root_at_action_boundary(
    state: &mut GameState,
    waiting_for: &WaitingFor,
    events: &mut Vec<GameEvent>,
) -> Result<Option<WaitingFor>, EngineError> {
    if !matches!(waiting_for, WaitingFor::Priority { .. }) || !state.counter_cost_root_is_due() {
        return Ok(None);
    }
    sync_waiting_for(state, waiting_for);
    resume_pending_continuation_if_priority(state, events)?;
    Ok(Some(state.waiting_for.clone()))
}
```
It enters the common boundary, so the root itself resumes through the pass's `resume_due_counter_cost_root`, and the outer work after it runs in CR 608.2c order. One function, one gate, and two call sites. That follows the precedent of `realize_settled_token_battlefield_entry`, which is called both before the shared pipeline and at the core convergence ("Same gate as that boundary call, one authority", `engine.rs` 16331–16338; `sync_waiting_for` is `public_state.rs` 83). No individual handler is patched.

- **Site 1, the shared tail of `apply_non_priority_pass_action`** (`engine.rs` ~16326). This is the route of every handler that falls through to the shared post-action pipeline, including the TapCreatures, ChooseMana and replacement-choice arms. Immediately before `if matches!(waiting_for, WaitingFor::Priority { .. }) {`, rebind:
  ```rust
  let waiting_for =
      resume_due_counter_cost_root_at_action_boundary(state, &waiting_for, &mut events)?
          .unwrap_or(waiting_for);
  ```
  It must run **before** that pipeline ("SBAs, triggers, layers", 16323). Otherwise triggers raised during the payment are collected while the activation is still unplaced. CR 603.3 (verified) puts them on the stack the next time a player would receive priority, which is after the activation completes (CR 602.2b + 601.2h).
  - A resumed stack root returns `Priority` with the activation placed, and the pipeline then collects its triggers.
  - A resumed mana root returns the outer prompt, and the pipeline is skipped, as for any non-Priority result.
- **Site 2, `apply_action_boundary_core`** (`engine.rs` 1658–1673), for handlers that return their `ActionResult` directly and the `PassPriority` route (`engine.rs` 11549–11559). Move `let scan_from = result.events.len();` above a new first step:
  ```rust
  let resumed_due_root = match resume_due_counter_cost_root_at_action_boundary(
      state, &result.waiting_for, &mut result.events,
  ) {
      Ok(Some(waiting_for)) => { result.waiting_for = waiting_for; true }
      Ok(None) => false,
      Err(err) => { *state = boundary_snapshot; return Err(err); }
  };
  ```
  Then widen the existing convergence condition to `(resumed_due_root && matches!(result.waiting_for, WaitingFor::Priority { .. })) || realized_token_entry || owed_spell_final_part_is_due`. `run_boundary_convergence_pipeline(.., scan_from)` then collects the resumed root's events, and `owed_spell_final_part_is_due` is evaluated **after** the resume. That also narrows T-F: a final part is never run under a root that this action made due.

**Runs after every action.** `apply`, `apply_as_current` and `apply_as_current_for_simulation` all go through `apply_action_boundary_core` (`engine.rs` 1306/1335/1398/8648). That calls `apply_action` (11072), which ends in either `pass_priority_once_with_pipeline` (11551) or `apply_non_priority_pass_action` (11562). Site 1 covers the second route's shared tail; site 2 covers everything else before the core returns. A due root therefore never outlives the action that made it due.

**Why the ordering at 9461 vs 9506 is safe.** `handle_priority_pass` (9461) can only meet a due root left by an earlier action, and site 2 has resumed that root before the earlier action returned. Within a `PassPriority` action, no root becomes due before 9461, because roots park only during payments, which end at prompts that are answered by non-pass actions.

**Residual 2, narrowed.** Its Prevented-arm blocks, and any other direct-drain handler, now resume a **due** root in the same action, at site 1 or site 2. What remains of residual 2 is a block that returns while the subtree is still `Unfinished`, which surfaces Priority mid-activation (unchanged, follow-up).

**No new residual 9.** There is no remaining caller-coverage gap: the authority sits on the two routes every action takes.

### F2. A row for the D4(c) tail guard

**T11** `a_prevented_non_counter_child_event_finishes_its_rider_before_the_cost_root`:
- **Board:** Essence Bottle with 3 elixir (stack-route root, depth 0).
- **Steward replacements:**
  - an optional RemoveCounter replacement with `execute = GainLife { Fixed 2, Controller }.sub_ability(LoseLife { amount: Fixed 1, target: None })`;
  - an optional `GainLife` replacement with no execute.
- **Harness:** generalize `stage_prevented_counter_removal` (1637) into `stage_prevented_pending_event(runner, expected: fn(&ProposedEvent) -> bool)`. Its body is unchanged except for the asserted predicate. Keep `stage_prevented_counter_removal` as a one-line wrapper so existing rows don't change.
- **Flow:** activate → `ChooseReplacement(ACCEPT)` → the GainLife prompt → `stage_prevented_pending_event(.., |p| matches!(p, ProposedEvent::LifeGain { .. }))` → choose 0.
- **Assertions:**
  - Reach at the GainLife prompt: the root is `CounterCostRemovalPayment`, `parked_child_rider` is true, the stack is empty, and `pending_replacement.proposed` is `LifeGain`.
  - After: life −1 (the gain was prevented; the rider ran). In that action's events, the P0 `LifeLost` comes before `AbilityActivated{bottle}`. Stack 1 with `top_stack_count == Some(3)`, no rider frame, ownership clear.
  - Resolve: +6.
- **Expected:**
  - **(unprobed)** green at 59247b7.
  - **Revert-check:** delete the D4(c) block (`engine_replacement.rs` 1714–1723). The CR 608.3e teardown (1746–1750) then clears the rider, so life is 0 → red.
- **P7 (unprobed):** the `BeginTurn` staging candidate prevents a `LifeGain` as it does a `RemoveCounter`. If it does not, stage with a GainLife-scoped prevention definition from the same helper.

### New rows for F1, on the A2 re-flowed T10

All three use A2's T10 board: Gainer (`GAIN_CONTROL`), the Steward `GainLife` replacement with the `LoseLife { 1 }.unless_pay({2}, Controller)` post-effect, the Cove with 3 storage, the Battery, and the RemoveCounter replacement with A2's child execute (whatever non-GainLife rider the executor chose; call it `t10_child_execute`). Every new life-moving effect in these rows is a life **loss**, so the GainLife replacement cannot re-apply (A2).

- **T10a** `an_outer_tail_waits_for_the_mana_root_paying_its_unless_cost` (trace 1). The outer post-effect is `LoseLife { 1 }.unless_pay({2}, Controller).sub_ability(LoseLife { Fixed 3 })`. Flow: activate Gainer → resolve → ACCEPT → `UnlessPayment` → activate the Cove → ACCEPT → entry 1 → DECLINE → entry 2 → DECLINE.
  - Reach at `UnlessPayment`, before the Cove: both P6 reaches, and an `AbilityContinuation` frame is parked (the outer tail C, via `parked_child_rider`).
  - Reach at each child prompt: `parked_mana_cursor(..).counter_removal` has `enclosing.frame.is_some()`.
  - After entry 2's DECLINE:
    - `waiting_for` is `UnlessPayment`;
    - life equals `life_at_unless` plus the child rider's own delta, and **not** −3 (the tail has not run);
    - the outer drain is still resident and `Paused` in the active frame;
    - an `AbilityContinuation` (C) is still parked;
    - in events, `EffectResolved{RemoveCounter}` < first `ManaAdded{cove}`.
  - Then `PayUnlessCost { pay: true }`: life −3 (the tail ran and the unless loss did not), `resolution_stack.is_empty()`, ownership clear.
  - Expected: **red** at 59247b7 (the cascade runs C before the re-offer) and green with F1. Revert-check: drop gate (a). The cascade's recursive `drain_pending_continuation` (`effects/mod.rs` 1397–1399) then runs C → red. Corrected by FD2: T10a does not discriminate (b) or (c). At the post-chain block the child is still nested in P, so the root is not yet due there, and T10a has no outer vote, repeat or choose-one-of frame for (c) to guard. T10d and T10e carry those gates.
- **T10b** `a_prevented_last_child_entry_leaves_the_outer_unless_post_effect_paused` (trace 2). The A2 T10 flow, but entry 1 is DECLINEd (removes one, so the count-driven rider sees 1) and entry 2 is prevented (`stage_prevented_counter_removal`, choose 0).
  - Reach at entry 2: the queue's `in_flight` is set, and the enclosing frame reach is as in T10a.
  - After the prevention:
    - `waiting_for` is `UnlessPayment`;
    - `active_post_replacement_or_paired_parent().and_then(resident)` is `DrainStatus::Paused` (the outer dispatch was not retired);
    - `EffectResolved{RemoveCounter}` < first `ManaAdded{cove}`;
    - storage 1, queue none.
  - After paying: no unless loss, and `resolution_stack.is_empty()`.
  - Expected: **red** at 59247b7 (`resume_resolution_frames` retires the outer dispatch first) and green with F1. Revert-check: drop gate 3 **and** `resume_due_counter_cost_root` → red.
- **T10c** `an_outer_tail_below_the_boundary_waits_for_the_mana_root` (the non-nested case, no replacement on the outer effect). Artifact "Taxer" gets a typed activated ability `{T}: LoseLife{1}.unless_pay({2}, Controller).sub_ability(LoseLife{3})`. Push it onto the object's ability definitions through `runner.state_mut()`, the way `install_replacement_on_steward` (1185) pushes a replacement. Flow: activate Taxer → resolve → `UnlessPayment` → the Cove → T3-shaped child → entry 1 DECLINE → entry 2 DECLINE.
  - Reach at the child prompts: `enclosing.is_none()` (the root parked with no PostReplacement frame among the top two), and C parked below.
  - After the second DECLINE: `waiting_for` is `UnlessPayment`, the tail has not run, and C is still parked. After paying: −3.
  - Expected: **red** at 59247b7 and green with F1.
- **T10d** `a_queued_child_rider_leaves_the_outer_unless_post_effect_paused` (FD2, discriminates gate (b)). The A2 T10 flow, but the child's rider is `Effect::Clash` with `.sub_ability(LoseLife { Fixed 1 })` in place of the A2 rider. Both players' libraries are empty, so Clash queues its tail without a placement prompt (P10). That is the `Queued` shape pinned by `step_distinguishes_complete_queued_waiting_and_keeps_outer_authority` (`effects/mod.rs` ~46610–46640). Entry 1 and entry 2 are both DECLINEd.
  - Reach: both libraries are empty at the Cove activation; the enclosing frame reach is as in T10a; no Clash placement prompt is ever raised.
  - After entry 2's DECLINE:
    - `waiting_for` is `UnlessPayment`;
    - the queued tail ran: P0 lost 1 in that action;
    - `active_post_replacement_or_paired_parent().and_then(resident)` is `DrainStatus::Paused` (P's outer dispatch survived);
    - `EffectResolved{RemoveCounter}` < first `ManaAdded{cove}`.
  - After paying: no unless loss, `resolution_stack.is_empty()`.
  - **Trace with (b) deleted:**
    1. In `drain_pending_continuation`, R's `Queued` branch (`effects/mod.rs` 1381–1384) runs the tail recursively. The recursive post-chain block retires the child's paused dispatch, so the root becomes due.
    2. Back in the outer call, the post-chain block's `finish_active_paused_post_replacement_dispatch` retires **P's outer** dispatch → red.
  - Expected: **red** at 59247b7, green with F1, red with gate (b) deleted.
  - **If P10 fails** (Clash prompts, or returns `Complete`), or the A2 flow cannot reach: **delete gate (b) and T10d**, and record "a `Queued` child rider under a nested mana root retires the outer paused dispatch" as residual 9 (a remaining defect with a follow-up), not an unexercised gate.
- **T10e** `an_outer_repeat_waits_for_the_mana_root_paying_its_unless_cost` (FD2, discriminates gate (c); non-nested, needs no outer replacement). Taxer's typed ability (as T10c) is `{T}: LoseLife{1}.unless_pay({2}, Controller)` with `repeat_for = Some(QuantityExpr::Fixed { value: 2 })` (`AbilityDefinition::repeat_for`, CR 608.2c) and no `sub_ability`. Flow:
  1. activate Taxer → resolve → `UnlessPayment` (iteration 1);
  2. snapshot `let repeat = runner.state().resolution_stack.iter().find(|f| matches!(f, ResolutionFrame::RepeatFor(_))).cloned()`;
  3. the Cove → T3-shaped child → entry 1 DECLINE → entry 2 DECLINE.
  - Reach: `repeat.is_some()`, and the child prompts' cursor has `enclosing.is_none()`.
  - After entry 2's DECLINE:
    - `waiting_for` is `UnlessPayment`;
    - the `RepeatFor` frame is still present and **equal to `repeat`** (iteration 2 has not started);
    - `ManaAdded{cove}` is in that action's events; storage 1.
  - Then `PayUnlessCost { pay: true }` (iteration 1 paid) → `UnlessPayment` again (iteration 2) → `PayUnlessCost { pay: false }`: P0 lost exactly 1 overall, and `resolution_stack.is_empty()`.
  - **Trace with (c) deleted:** after R retires the child's own dispatch frame, the post-block `drain_active_repeat_for` (`effects/mod.rs` ~1415) starts iteration 2, which raises a fresh `UnlessPayment`. D6(a) never runs (`waiting_for` is not Priority), so the Cove root stays parked → red. The `RepeatFor` frame differs from `repeat`, and there is no `ManaAdded`.
  - Expected: **red** at 59247b7, green with F1, red with gate (c) deleted.
  - **P9 (unprobed):** `repeat_for` with `unless_pay` on a resolving activated ability parks a `RepeatFor` frame under iteration 1's `UnlessPayment`. If it does not (for example, the unless prompt owns the whole repeat), use the Oracle form "{T}: Repeat the following process twice. You lose 1 life unless you pay {2}." if it parses; otherwise use `RepeatUntil` / `VoteBallot` / `ChooseOneOf` shapes, whichever reaches first. If none reaches, **delete gate (c)** and record its shape as a residual, as for (b).
- **T12** `a_mana_choice_child_finishes_and_places_the_activation_in_the_same_action` (FD1). Essence Bottle with 3 elixir. A Steward optional RemoveCounter replacement with `execute = Mana(AnyOneColor, count 1).sub_ability(LoseLife { Fixed 1 })`: the typed `Effect::Mana` whose production is `ManaProduction::AnyOneColor { count: Fixed 1, .. }` (`types/ability.rs` 3442). Flow: activate → ACCEPT → `WaitingFor::ChooseManaColor { context: ManaChoiceContext::ResolvingEffect(_), .. }` → `GameAction::ChooseManaColor { choice: ManaChoice::SingleColor(ManaType::Green), count: 1 }` (the answer shape used by `triggered_ability_mana_ledger.rs` 228–235).
  - Reach at the prompt: the root is `CounterCostRemovalPayment`, `parked_child_rider`, stack empty.
  - After the answer, **in that same action**:
    - green 1 in P0's pool and life −1;
    - `LifeLost` < `AbilityActivated{bottle}` in the action's events;
    - stack 1 with `top_stack_count == Some(3)`, no rider frame, ownership clear.
  - Resolve: +6.
  - Expected:
    - **red** at 59247b7: the handler (`effects/mana.rs` 431–432) drains and returns, so nothing resumes the root in this action;
    - **red** with F1 part 1 alone: the gates stop and nothing resumes;
    - **green** with the authority.
  - Revert-check: remove both authority call sites → red. Remove only site 1 → still green (site 2 catches it), which is why T12b exists.
- **T12b** `a_trigger_from_the_childs_payment_goes_on_the_stack_above_the_activation` (discriminates site 1). T12's board plus a P0 artifact "Witness" with the labelled synthetic Oracle text `"Whenever you lose life, you gain 1 life."` (reach: Witness has one trigger definition). Same flow.
  - After the answer: stack 2, the **top** entry is Witness's trigger, the entry below it is the Bottle activation with count 3.
  - Expected: green with site 1. With site 1 removed (site 2 alone), the shared pipeline collects Witness's trigger before the activation exists, so the activation lands above it → red. **(unprobed)**
  - If T12b is green with site 1 removed (the pipeline does not place triggers while a root is parked), **delete site 1 and T12b** and keep site 2 as the only call site.

**If the A2 flow cannot reach** (P6 fails, or the public-action validator rejects the nested shape): keep T10a/T10b `#[ignore]`d, with the reason quoting the failed reach. T10c (which needs no outer replacement) and T11 then carry F1's and F2's evidence, and the nested-configuration gate stays covered by U2/U3/U3b. If T10c's typed ability cannot be installed through `state_mut()`, express it in Oracle text instead ("{T}: You lose 1 life unless you pay {2}. You lose 3 life.") and assert the parsed shape (`unless_pay.is_some()`, a `sub_ability` present) as its first reach. If that does not parse either, record T10c as blocked; F1 then rests on T10a/T10b alone.

### Scope, sizing and verification

- **Production paths:** `types/game_state.rs`, `game/effects/mod.rs` (**new path**), `game/engine.rs` (the pass helper and the action-boundary authority with its two call sites) and `game/engine_replacement.rs` (comment only).
- **Tests:** T10a–T10e, T11, T12, T12b, and the staging generalization in `cost_removed_counter_amount.rs`.
- **Gate ↔ row map** (every gate has a discriminating row, or it is deleted):

  | Gate | Row | If the row cannot reach |
  |---|---|---|
  | (a) | T10a | — |
  | (b) | T10d | delete (b) and T10d, residual 9 |
  | (c) | T10e | delete (c), residual |
  | 3 + `resume_due_counter_cost_root` | T10b (paired) | — |
  | Authority, site 2 | T12 | — |
  | Authority, site 1 | T12b | delete site 1 |
  | D4(c) | T11 | — |
- Paths go from 9 to 10, still 1 unit (the same rule, now enforced where frames drain). This does not trip the phase-fit gate, so no second unit is needed.
- **Verification:**
  1. `cargo fmt --all`.
  2. Full `test-engine` (the gates sit in the shared drains) and `clippy`.
  3. Red evidence at 59247b7 plus the amendment commit for T10a–T10e and T12, then green after.
  4. Run each gate's revert-check from the table above, and record the deletions the table prescribes if a row cannot reach.
  5. T11 green, with its D4(c) revert-check red.
  6. All earlier rows (T1–T9, T4b, U1–U3b) stay green. T3 and T9 are the closest regressions: their R runs and then the child retires, after which the stop is a no-op, because no outer work exists at depth 0.

### Fix-design review dispositions

| Finding | Resolution |
|---|---|
| **FD1** (residual 9 false; direct-drain handlers leave a due root parked behind the gates) | Residual 9 deleted. One authority, `resume_due_counter_cost_root_at_action_boundary`, sits next to `owed_spell_final_part_is_due`. It is called at the core (site 2, every action) and before the shared post-action pipeline (site 1, trigger order, CR 603.3), following the token-realization precedent; no handler is patched. Verified that it runs after every action (all entry points → `apply_action_boundary_core`), and that 9461 cannot meet a due root. Residual 2 is narrowed to `Unfinished` exits. Rows **T12** (ChooseMana child, same-action placement) and **T12b** (site 1, trigger above the activation, with a delete-if-undiscriminated fallback). |
| **FD2** (gates (b)/(c) undiscriminated; T10a revert-checks wrong) | T10a's revert-check corrected to gate (a). **T10e** (outer `RepeatFor` under the unless prompt) discriminates (c). **T10d** (a `Queued` Clash rider) discriminates (b). Each comes with an explicit "delete the gate and record a residual" fallback if its fixture cannot reach, so no gate ships undiscriminated. |

---

## Phase D impl-review r2 fixes (approved design: impl-review-r2 + fix-design review APPROVE-WITH-CHANGES)

Sources: pr-phase-D-impl-review-r2.md (F1 MED, F2 MED, F3 LOW, T1 text) and pr-phase-D-impl-review-r2-fix-design-review.md (C1–C5). P1 probe at 618fb57 confirmed r2's trace exactly (phase-fit 92). Where the two documents differ, the fix-design review wins.

1. **One custody authority for inline trigger collectors (C1 + C2).** In `announcement_triggers`, add one ownership predicate `announcement_owned_from` and make `stage_announcement_trigger_events` use it. Add one entry point, `take_inline_trigger_span(state, events, start) -> Range`, which returns the part the caller still owns. Under a live announcement it stages and claims, and the caller does NO inline scan, NO deferred collection and does not set `triggers_processed_inline`. Before taking custody, call the existing `resolve_tap_mana_triggers_inline` (CR 605.4a; the residual is ManaAdded-mode triggered mana abilities only — record it in a comment and in the PR body, with `partition_collected_mana_frame_contexts` as the follow-up route). Do NOT inline-scan "the unclaimed remainder" after staging: that remainder is cost-owner-retained events (C2). Route every same-buffer inline collector through it:
   - engine.rs ChooseManaColor ResolvingEffect arm (~13256-13287);
   - engine_resolution_choices.rs at ~789, ~817 (`batch_or_drain_observer_triggers`), ~836, ~870 (search collector), ~6036-6062, ~8394, ~8510, ~8594, ~8685;
   - effects/mod.rs ~2545/2548.
   Do NOT put the guard in `collect_triggers_into_deferred` or `process_triggers` (their slices include events outside the action buffer). Line numbers are from 618fb57; re-locate by symbol.
2. **`commit_activation_announcement` (fix (ii) + C3).** It lives in `announcement_triggers` and shares one body with `commit_spell_announcement` (no copied code). Behaviour:
   - `None` collection → end the announcement scope only. This is today's behaviour for every ordinary activation and must stay unchanged.
   - `Some` → collect `unclaimed_events(state, events, scope_start)` minus `retained_by_cost_owners`, plus the filtered delayed scan, and claim `AllCollectors` over start..len minus retained events.
   - `Some` with an `Unowned` scope → `debug_assert!` and start at 0, not `events.len()`.
   - The doc comment carries the no-loss argument: no ledger writer claims without collecting, so the filter only removes re-collections, and the commit point is unchanged, so there is no reordering.
   `push_ability_entry` (casting_costs.rs ~7323-7345) calls it.
3. **Site 1 (C5).** Keep site 1 with the fix-design review's replacement comment: CR 704.3 / 117.5 / 117.3c — the shared pipeline's SBA loop and its `process_triggers` branch must not run mid-activation, or SBA-caused triggers land below the activation (CR 603.3). Discriminator row P3 → **T12e** `an_sba_trigger_from_the_childs_payment_goes_above_the_activation`:
   - board: T12's, plus a P0 1/1 "When this creature dies, you gain 1 life";
   - child: `DealDamage{1, SpecificObject(that creature)}` with no lose rider;
   - reach guard: before the answer, the 1/1 has damage marked and is still on the battlefield;
   - assert straight after the answer with no drive: stack is [B, D].
   Expected red without site 1 ([D, B]). If it is green without site 1, delete site 1 and say so.
4. **Tests.**
   - F2: T12b is un-ignored and asserts straight after the colour answer with NO `drive`: exactly one Witness entry, above B.
   - C4: T12c (Scry child + Witness: `Effect::Scry{1}.sub_ability(lose(1))` as the RemoveCounter post-effect; answer the Scry; stack == [B, W] with one W) and T12d (T12's child under a Cove mana root paying a sorcery's ManaPayment, plus Witness; exactly one Witness, collected with the cast batch). Both are asserted with no drive, and each gets a revert check against item 1.
   - F3: T10f `an_outer_player_scope_seat_waits_for_the_mana_root` as in r2 §2, with P0 as the active player (APNAP), so P0's seat is the one that pauses. Reach: an AbilityContinuation frame (P1's leg) is parked at P0's UnlessPayment. Expected red with gate (a) removed.
   - T1: replace T10c's doc comment with the text in r2.
5. **Verification under the deferred policy:**
   - fmt, CR grep, and targeted red/green only. The base for red/green is 618fb57 plus the new tests; one-revert columns: −item 1, −item 2, −site 1, −gate (a).
   - Rows: T12b, T12c, T12d, T12e, T10f, plus the existing cost_removed_counter_amount file.
   - The regression suites the fix-design review lists (activation/announcement, delayed triggers, Treasure/sacrifice costs, search dedup, engine_tests, triggers_dedup_regression_tests) are deferred to the final batched check after the upstream-main merge, where they are covered by the engine integration and lib runs.

---

## Phase D impl-review r3 fixes (Jakob approved the expansion case "Fix all three", 2026-10-18)

The source is pr-phase-D-impl-review-r3.md, section "Findings" F1–F6. Its suggested fixes are the brief. Use these constraints:

1. **F1 (missed custody collectors).**
   - Route `park_observer_triggers_if_paused` (triggers.rs ~8400) and `collect_and_drain_observer_triggers_if_settled` (~8437) through `take_inline_trigger_span`, changing signatures to `&mut Vec<GameEvent>` as needed. Their callers are Scry continuation, OptionalEffectChoice, morph, and OpponentMayChoice ×2.
   - Route `park_discard_batch` (effects/discard.rs ~204) by passing the action buffer and `events_before_self`, then calling `defer_observer_triggers_for_paused_choice`. Fold `defer_observer_triggers_from` back in (its premise was false), and fix or remove its doc comment.
2. **F2 (rider-free mana child strands the root).** At the action-boundary authority (`resume_due_counter_cost_root_at_action_boundary`, engine.rs ~1746), run the D5 resume loop for ANY parked counter-cost root under a Priority result, not only `is_due`. Return `Some` only on progress. Annotate with CR 602.2b + 601.2h / 117.3c / 704.3 (grep-verify). Note in the plan's residual 2 text that this narrows it; do NOT widen to the other Prevented-arm blocks.
3. **F3 (draw-pair promote lifts an outer continuation).** In `take_completed_multi_draw_frame` / `promote_ability_continuation_after_post_replacement_draw` (draw.rs ~858, resolution.rs ~3295-3316), do not promote when a counter-cost root is parked and the continuation below the PR sits below the root's boundary (`index < paused_counter_removal().replacement_subtree.resolution_depth`). Prefer passing a predicate or boundary from the caller over reading root state inside resolution.rs, if that keeps the layering clean. CR 101.4 / 608.2e / 118.12a — grep-verify.
4. **Rows.**
   - T10f un-ignored (Draw rider) as F3's row, with F4's real assertions: exactly one AbilityContinuation whose chain is P1's bound LoseLife, and/or `UnlessPayment{player: P1}` after P0 declines.
   - T10f′ (GainLife rider via `t3_child_execute`, no draw) as gate (a)'s row, with the same F4 assertions.
   - T12f: the rider-free mana child. The activation is on the stack in the same action, counter count 3. Red with the `is_due` precondition restored.
   - Double-Scry row: `scry_one().sub_ability(lose(1)).sub_ability(scry_one())` + Witness; stack == [B, W] with exactly one W. Red with triggers.rs ~8400 unrouted. It replaces T12c (delete T12c, or keep it only if it adds something the double-Scry row doesn't).
   - Sacrifice-child row (F5): a child "sacrifice a creature" (EffectZoneChoice) → `batch_or_drain_observer_triggers` B1, with a dies-Witness; [B, W] once. Red with that site unrouted. If the fixture cannot reach that site, stop and report rather than improvise.
5. **F6.** In the B1 returns of `batch_or_drain_observer_triggers`, `collect_search_observer_triggers` and the discard pair loop, return plain `WaitingFor` only when `owned.is_empty() && owned.end < events.len()`, matching the mana arm.
6. **Verification (deferred policy).**
   - fmt, CR grep, scope/diff check, and targeted red/green only. Base is 8f18b5e plus the new tests.
   - One-revert columns: −F1 routing (triggers.rs 8400), −F2 (restore `is_due`), −F3 guard, −gate (a), −sacrifice-site routing.
   - Run the whole cost_removed_counter_amount file at head.
   - No workspace clippy/tests, coverage, audits or parser comparison.

Note (r3 fix F2): the action-boundary authority now runs the D5 loop for ANY parked counter-cost root under a Priority result, which narrows residual 2: a Prevented-arm child block that returns Priority with an advanceable subtree is now resumed in the same action. Residual 2 remains only for subtrees the loop cannot advance (residual 3's no-progress case).

---

## Phase D impl-review r4 fixes (Jakob approved expansion case "Fix all four", 2026-10-18)

The brief is pr-phase-D-impl-review-r4.md (findings R4-F1 to R4-F5 and their suggested fixes). Constraints:

1. **R4-F1 (resume-before-collect duplicate).** Use the ledger filter, not collect-before-resume. In the `batch_or_drain_observer_triggers` body (~769-793), filter the slice with `filter_consumed_trigger_events_from(&events[..end], start, Ordinary, ledger)`, the idiom `collect_search_observer_triggers` already uses. That one change covers its callers (~6365, 6630, 7542/7566, 7991/7995, 8024/8029, 6016, 7423). Apply the same filter to the DiscardChoice pair loop (~6047-6066) and the keeper-choice B2 arms (~8622, ~8719). Rows:
   - un-ignore the sacrifice row (exactly one W, above B);
   - add a discard-child row (a "discard a card" child with a discard observer such as a "whenever a player discards" trigger). It must go red with the filter removed.
2. **R4-F2 (second promote caller).** Move the F3 guard into one `GameState` helper and call it from both `take_completed_multi_draw_frame` and `finish_active_paused_post_replacement_dispatch`. Add a row only if a reachable fixture exists. If not, say so: the shared helper makes the existing T10f revert check cover both callers.
3. **R4-F3 (T10f fixture payer).** Switch the T10f / T10f′ fixtures' unless payer to `ScopedPlayer` (the real "unless that player pays" class) and assert `UnlessPayment.player == P1` for P1's leg.
4. **R4-F4 (orphaned events on partial progress).** When the action-boundary authority returns `None` after the loop appended events, stage the appended span, as the review specifies. A row is optional; it lives inside residual 3.
5. **R4-F5 (text).** Rename `resume_due_counter_cost_root_at_action_boundary` / `resumed_due_root`, and fix the comments at engine.rs ~1659 and ~16411, using the review's replacement wording.
6. **Verification (deferred policy).**
   - fmt, CR grep, scope check, targeted red/green (base 384bb92 plus the new tests).
   - One-revert columns: −ledger filter in batch_or_drain, −discard-loop filter, −shared promote guard (both callers), −F4 staging.
   - Run the whole cost_removed_counter_amount file at head.
   - Run engine clippy before committing.

## Phase D impl-review r5 fix (R5-F1, expansion case 104)

1. `engine_resolution_choices.rs` CategoryChoice keeper arm (the B2 `else` branch after `resume_with_error_propagation` at ~:8413, raw slice ~:8429-8440): replace the raw `events[events_before_sacrifice..events_after_sacrifice.min(owned.end)]` slice with `unclaimed_observer_events(state, events, events_before_sacrifice, events_after_sacrifice.min(owned.end))`, keeping the `PhaseChanged` filter, exactly as the two sibling keeper arms (~:8648, ~:8749) do. Comment: same CR 603.2c / 603.3 note as the siblings.
2. No new test row (latent, residual-3 shape, like its siblings); add this site to the residual note listing the keeper arms without a discriminating row.
3. Check: `rg -n "resume_with_error_propagation\(state, events\)\?;" -A30 crates/engine/src/game/engine_resolution_choices.rs` shows no raw `events[..]` -> `collect_triggers_into_deferred` after a resume.
4. Verify: cargo fmt --all; engine clippy (`-p phase-engine --all-targets --features engine/proptest -D warnings`); counter-cost integration suite (expect 72/0/0). Commit on top of c909249, push to engine/cost-removed-counters-wip only.
