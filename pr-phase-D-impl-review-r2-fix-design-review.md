# Fix-design review: Phase D impl-review r2 (F1/F2/F3/T1)

- Head: 618fb57. Input: `pr-phase-D-impl-review-r2.md`.
- Mode: a read-only design review. No builds and no probes were run (caller constraint). Every claim below comes from reading code at 618fb57.
- CR numbers were grep-verified against `docs/MagicCompRules.txt`: 117.3c, 117.5, 601.2h, 601.2i, 602.2, 602.2b, 603.2c, 603.3, 603.3b, 605.1b, 605.3a, 605.3b, 605.4, 605.4a, 704.3, 704.4.

**Verdict: APPROVE-WITH-CHANGES.**
- The diagnosis in r2 is right. The fix shape is also right in direction: the announcement carrier is the custodian, and the activation commit mirrors the spell commit.
- Three issues must change before implementation:
  1. (i) as written has a hole: it inline-scans the retained remainder.
  2. (i) must be generalized now, not after P2.
  3. (ii) needs three specifics (C3).
- Q3, Q4 and Q5 are answered below. Of these, only Q3 changes the design.

---

## Q1. Is (i)+(ii) the single authority? Should it be generalized now?

### The actual authority

The one authority already in force is not the carrier as such. It is the occurrence ledger protocol, which every current same-buffer collector follows:

- **The rule:** a collector that collects a span of the action buffer records those occurrences in `consumed_before_priority_trigger_events`.
- **Who follows it:**
  - `stage_announcement_trigger_events`;
  - `claim_ordinary_collected_span`, used by `park_cost_payment_triggers_if_paused` (`casting_costs.rs:2580-2617`) and `park_deferred_cost_triggers_if_paused` (`:3510-3539`);
  - the completed mana frame (`mana_abilities.rs:3705-3763`);
  - sacrifice-cost settlement (`casting_costs.rs:3800-3833`);
  - `collect_copy_cast_observers` (`engine.rs:10549-10565`);
  - the logical zone-change owner (`triggers.rs:~4285`).
- **What the claims do:**
  - The announcement stage reads them through `unclaimed_events` → `filter_already_collected_trigger_events_from` (`triggers.rs:11925`).
  - That filter honours claims, plus a value-equality witness for queued `ZoneChanged` events **only**.

The `WaitingForWithInlineTriggers` collectors and the `ResolvingEffect` mana arm break this protocol:
- They collect into `deferred_triggers`, or dispatch, and claim nothing.
- Their only dedupe is `triggers_processed_inline`, which suppresses the pipeline **scan** only.
- `stage_announcement_trigger_events` runs before `run_post_action_pipeline_core` regardless of `skip_trigger_scan` (`engine_priority.rs:156`). `end_action` stages from 0 on every non-Priority result (`announcement_triggers.rs:258-266`).

So under any live announcement, every **non-`ZoneChanged`** event these collectors take is staged again. This holds for `LifeChanged`, `CounterRemoved`, `Discarded` and `PlayerPerformedAction`. It is a static certainty, not a probe question. P2 cannot come back clean for a collector whose slice includes the rider's events.

**Answer:** generalize now, with one helper. P2's T12c and T12d become regression rows (C4), not decision gates.

### Class-level shape (required)

**One ownership predicate**, shared with the stage, so the two cannot diverge. Add it to `announcement_triggers.rs`:

```rust
/// CR 601.2h + CR 602.2b + CR 603.3: whether a live announcement owns
/// `events[start..]` of the current action buffer — exactly the condition
/// under which `stage_announcement_trigger_events` would take custody.
fn announcement_owned_from(state: &GameState, start: usize) -> Option<usize>
    // Some(max(start, owned_from)) iff scope == From{owned_from} && pending_trigger_carrier_owner().is_some()

/// Inline collectors' entry: stage the owned part of the span into the
/// carrier (claimed), and return the sub-range the caller still owns
/// (empty when the announcement took all of it; the whole span when none is live).
pub(crate) fn take_inline_trigger_span(state: &mut GameState, events: &[GameEvent], start: usize) -> Range<usize>
```

- `stage_announcement_trigger_events` itself uses `announcement_owned_from`.
- The caller's residual range is `start..owned_from` in the rare case where the announcement began part-way through the span. Otherwise it is empty.
- When the announcement takes custody, the caller does **not** inline-scan, does **not** collect into `deferred_triggers`, and does **not** set `triggers_processed_inline`.
- Retained events (`retained_by_cost_owners`) are left to their owner. They are neither staged nor scanned (see C2).
- Callers keep their B1 `drain_deferred_trigger_queue` call unchanged. It is barrier-gated while a cost root is parked, and it still releases earlier queued work.

### Site census

- **Command:**
  ```
  rg -n "collect_triggers_into_deferred\(|process_triggers\(" crates/engine/src/game --glob '!*_tests.rs'
  ```
  Then exclude `#[cfg(test)]` modules.
- **Predicate:** the site collects a slice of the **current action buffer**, records no ledger claim, and can run while `pending_trigger_carrier_owner().is_some()`. That is a child of a parked cost root, or a resolution-time step under a cast's payment.

| Site | Disposition |
|---|---|
| `engine.rs:13256-13287` ChooseManaColor `ResolvingEffect` arm | **Route** (with C2 ordering) |
| `engine_resolution_choices.rs` `batch_or_drain_observer_triggers` B1 (`:789`) and B2 (`:817`) | **Route** |
| `engine_resolution_choices.rs:836` `defer_observer_triggers_for_paused_choice` | **Route** |
| `engine_resolution_choices.rs:870` `collect_search_observer_triggers` (Search; the maintainer's example) | **Route**. It already filters claims, but it still does not claim, so the stage re-collects it. |
| `engine_resolution_choices.rs:6036-6062` discard + continuation pair loop | **Route** |
| `engine_resolution_choices.rs:8394, 8510, 8594, 8685` B2 parks | **Route** |
| `effects/mod.rs:2545/2548` `drain_pending_change_zone_iteration` | **Route** |
| `engine_priority.rs:364-375, 438-474` (pipeline scan and SBA loop) | **No change.** Already behind the stage and ledger. The SBA mid-activation timing is site 1's job (Q4). |
| `engine.rs:10554` `collect_copy_cast_observers`, `casting_costs.rs:2608/3536/3804`, `mana_abilities.rs:3705` | **No change.** These already claim. |
| `engine_replacement.rs:2946` and `engine_resolution_choices.rs:5714` (replay `deferred_entry_events`) | **Pre-existing, out of class.** These events are not in the buffer, so they cannot be staged by index. They dispatch immediately under a live announcement. Listed under Pre-existing; no Phase D row reaches them. |
| `effects/copy_spell.rs:290` (`SpellCopied`) | Verify whether the event is also pushed to `events`. If it is, **route**. Otherwise it is out-of-buffer, as above. |

`collect_triggers_into_deferred`/`process_triggers` are **not** the place for the guard:
- Their slices include out-of-buffer events: `deferred` ledgers, `final_part_events` and replayed entries.
- A blanket "skip under announcement" there would drop triggers.

---

## Q2. Does (ii) change ordinary activations?

### Funnel

- `push_ability_entry` has two callers, both inside `push_activated_ability_to_stack` (`casting_costs.rs:7158, 7171`).
- That wrapper has 12 callers: 11 in `casting_costs.rs`, 1 in `casting.rs`. Every stack-using activation goes through it.
- Mana abilities and the plot special action never reach it (`:7131-7138`).

### Activations with no carrier (the common path): no change, provided (ii) keeps the gate

- `activation_trigger_collection` is `Some` only if a carrier was installed. That happens:
  - at targeting (`casting_targets.rs:47`);
  - for modes (`engine_modes.rs:316`);
  - on paused payments (`casting_costs.rs:1994/2029/6932/6960/6986`, `casting.rs:27746`);
  - by a stage (`announcement_triggers.rs:191`).
- An untargeted, unpaused activation has `None`. Today it only calls `end_activation_announcement`, and its cost triggers are collected by the pipeline scan that follows: sacrifice-as-cost dies, tap, Treasure.
- `commit_activation_announcement` must do exactly that for `None`: take the scope to `Unowned` and return. It must not create a carrier or collect anything.

### Carrier-bearing activations (targeted, modal, paused): five deltas

These are common, for example "{T}, Sacrifice X: … any target".

1. **Start moves from 0 to the scope start.**
   - `activate_with_cost_carrier` calls `begin_announcement(events.len())` (`casting.rs:26640`). That is 0 for an `ActivateAbility` action. `begin_action` gives `From{0}` for a resumed activation.
   - Equal to today, except after an in-action stage or pipeline pass. Those advance the scope, and that is exactly the copy-#13 fix.
2. **Claimed occurrences are skipped.**
   - The ledger writers listed in Q1 each pair a claim with a collection into the carrier, `deferred_triggers`, a mana fixed point or an owner pass.
   - **No writer claims without collecting** (checked: every `consumed_before_priority_trigger_events` writer in non-test code; `visibility.rs` only reads).
   - So the filter can only remove re-collections, never lose a trigger.
   - The intersection where this changes counts today is a carrier-bearing activation whose push shares a buffer with an auto-paid mana frame (Treasure) or a replacement-paused sacrifice settlement. The current unfiltered `collection.collect(state, events)` re-collects occurrences those collectors already claimed and queued. That is a probable pre-existing duplicate: unprobed, listed under Pre-existing.
3. **`retained_by_cost_owners` is excluded from collect and claim.**
   - Today such events are collected by the carrier and claimed `AllCollectors`.
   - This mirrors the spell path, and the owner settles them.
   - It would lose a trigger only if a retaining owner is live at push time and never settles. No such route was found, but the suites below must confirm it.
4. **Delayed triggers.**
   - `collect_delayed_triggers_into_deferred(state, events)` (no claim filter) becomes the filtered `Delayed`-requester span with `collect_delayed_trigger_contexts_into_deferred`.
   - Staged spans are claimed `OrdinaryCollectorsOnly`, so delayed triggers still see them (Earthbend's dies-or-exiled return is preserved).
   - The only difference is that already-consumed delayed occurrences no longer fire twice.
5. **The claim is narrowed** from the full buffer to `start..len` minus retained. Events before `start` stay visible to the shared pipeline, so they are collected once there.

**Ordering:** carrier contexts are still appended to `deferred_triggers` at the same point, before `drain_deferred_triggers_after_stack_object_announcement`. No reordering.

**Batch triggers** ("one or more …"): a batch that today spans a claimed collector and the carrier is already split, and so fires twice. After (ii) it fires once per collector batch. This is not a regression.

### Suites most likely to catch a regression

- **Integration** (`crates/engine/tests/integration/`):
  - this PR: `cost_removed_counter_amount`;
  - targeting and announcement: `activation_cost_target_settlement`, `announced_target_set_placement`, `becomes_target_paused_cast_triggers`, `avalanche_of_sector_7_activation_trigger`, `cr733_resolved_trigger_collection`;
  - delayed triggers (Earthbend, per the comment at `casting_costs.rs:7328-7333`): `delayed_trigger_continuation`, `integration_bending`;
  - sacrifice and Treasure costs: `issue_5252_additional_sacrifice_after_mana_abilities`, `issue_5997_malcolm_treasure_trigger`, `issue_6157_gold_token_auto_mana_payment`, `issue_6924_moldervine_cost_sacrifice_trigger`, `issue_2415_rottenmouth_viper_sacrifice_cost`, `sacrifice_cost_context_identity`, `cost_zone_pipeline`, `random_discard_cost_replacement_resume`;
  - other activation flows: `search_delivery_observer_dedup`, `loyalty_ability_activated_trigger`, `keyword_activation_cancel`, `cost_x_carrier_runtime`, `special_action_x_runtime`, `stack_ability_kind_axis`.
- **Unit:** `engine_tests.rs` (it touches the carrier and the ledger) and `triggers_dedup_regression_tests.rs`.

---

## Q3. The CR 605.4a residual

CR 605.4a (verified): "A triggered mana ability doesn't go on the stack … Rather, it resolves immediately after the mana ability that triggered it, without waiting for priority." CR 605.1b covers triggers "from the activation or resolution of an activated mana ability … or from mana being added to a player's mana pool".

**Do not split the span.** A span split is the wrong shape, because one event (a `ManaAdded` or a `TappedForMana`) can trigger both a triggered mana ability and an ordinary trigger. The correct partition is by context, not by event. That is what `partition_collected_mana_frame_contexts` (`triggers.rs:10107`) does for mana frames.

**Narrow the residual with existing machinery (required).**
- In the custody branch, call `triggers::resolve_tap_mana_triggers_inline(state, &mut events, events_before)` **before** staging.
- That is the existing CR 605.4a authority. It resolves `TapsForMana` and `ManaAbilityProduced` triggered mana abilities now, and flips the events to `FromTapTriggersResolved` and `InlineResolved`.
- Every collector skips the flipped events through the guard in `collect_matching_triggers_inner` (`triggers.rs:3172-3200`), and the carrier's `collect_pending_triggers_with_collection` funnels through that guard. So the staged copy cannot resolve them twice.
- It is idempotent with auto-tap's own call.
- Call it only in the custody branch. The no-announcement branch stays byte-identical (`process_triggers`).

**Residual to record.** What remains is **`ManaAdded`-mode** triggered mana abilities triggered by the child's mana under a live announcement:
- They are carried by the carrier and resolve at the activation's drain instead of immediately.
- This matches the engine's existing deferred-scan convention for `ManaAdded`-mode abilities (comment at `triggers.rs:3176-3177`).
- It is unreachable by every Phase D row.
- Record it as a residual naming the follow-up route: classify the `ResolvingEffect` span's contexts through `partition_collected_mana_frame_contexts`, or route it through `collect_completed_mana_frame_events`. That needs a resume variant for an accepted mana ability that pauses, which is new machinery.
- Acceptable to record.

---

## Q4. Site 1's rationale and P3

**The rationale is correct, and it is stronger than r2 states.**
- CR 704.3 (verified): SBAs are checked "whenever a player would get priority". CR 117.5 says the same.
- During an activation nobody receives priority until it is complete (CR 602.2b with CR 601.2h/601.2i). The activator receives priority afterward (CR 117.3c).
- Without site 1, the pipeline runs with the root parked and `waiting_for = Priority`, so the SBA loop runs mid-activation. Its trigger branch is `engine_priority.rs:463-474`:
  - It is `process_triggers` unless `pending_resolution_completion`, `announced_cast_batch` or `final_part_ran` holds, and none of those holds for a parked activation root.
  - `process_triggers` checks only `resolving_spell_owes_final_part` (`triggers.rs:7027`).
  - So an SBA-caused trigger goes on the stack **immediately, below the activation**. That violates CR 603.3.
- The pipeline's delayed scan and deferred drain are not observable here. The drain is barrier-gated, and (ii)'s commit filters by `Delayed`-consumed.
- **Replacement comment for site 1** (`engine.rs:16380-16383`):
  > CR 704.3 + CR 117.5 + CR 117.3c: a counter-cost root this handler made due resumes before the pipeline below, so the activation is on the stack before the game checks state-based actions — no player receives priority mid-activation (CR 602.2b + CR 601.2h), and triggers those actions cause go on the stack above the activation (CR 603.3). Same authority as the action boundary's call in `apply_action_boundary_core`.

**P3 is a sound discriminator.**
- With site 1: push → pipeline → SBA kills the 1/1 → dies trigger above B. Stack [B, D].
- Without site 1: SBA → `process_triggers` → D on the stack, then site 2 pushes B. Stack [D, B]. **Red.**
- The `.sub_ability(lose(1))` rider is unnecessary for P3; drop it so the row isolates the SBA path.
- Assertions:
  - stack == [B, D];
  - exactly one D;
  - the 1/1 is in the graveyard;
  - assert directly after the answer, with no `drive` (F2's rule).
- Required reach guard: before the answer, the 1/1 has 1 damage marked and is still on the battlefield, and the root is parked.

If P3 were green without site 1, deletion is correct under the plan's rule (plan line 1075, gate↔row map line 1092). Site 2 alone still resumes every due root. I expect P3 to be red, so site 1 stays.

---

## Q5. T10f: `player_scope` + `unless_pay`

**It parks a frame.** The reach is valid, with one fixture precondition.

- The `player_scope` driver (`effects/mod.rs:16087-16100`) runs **before** the unless intercept (`:17016`).
  - `split_player_scope_chain` (`:7374`) keeps `unless_pay` on the scoped template.
  - `bind_scoped_seat` (`:19751`) sets `controller = seat`, so `payer: TargetFilter::Controller` (`lose_unless_two`, test `:2729-2736`) resolves to that seat through `resolve_unless_payer`.
- Seat 0's step sets `UnlessPayment` and returns pending.
- `:16391-16422` then builds P1's leg as the tail and calls `append_to_pending_continuation`.
  - With no active ability-continuation frame (Taxer resolving from the stack), it reaches `park_ability_continuation` (`:2838-2840`), which is a new `AbilityContinuation` frame.
  - If a continuation frame **were** active, the tail would be spliced into that frame's chain (`:2815-2828`) and no new frame would exist.
- **Precondition:** P0 must be first in APNAP order (P0 is the active player), so P0's seat pauses and P1's leg is the parked frame. Otherwise P1 is prompted first and the row asserts the wrong seat.
- The row's reach assertion (frame present, `cove_removal_enclosing(..).is_none()`) is the right guard. Keep r2's fallback rule: a different producer, never deleting (a).

---

## Required changes

**C1 [behavior]. Generalize (i) now.**
- Add `announcement_owned_from` (shared by `stage_announcement_trigger_events`) and `take_inline_trigger_span` to `announcement_triggers.rs`.
- Route every **Route** row in the Q1 census through it.
- In the custody branch: no inline scan, no `deferred_triggers` collection, no `triggers_processed_inline`.
- The no-announcement branch stays unchanged.

**C2 [behavior]. Replace r2's "stage, then inline-scan the unclaimed remainder".**
- Why: after a stage, the remainder is exactly the `retained_by_cost_owners` events, which the stage excludes but does **not** claim. In T12d this is the `ManaAbilityPayment` cursor's `deferred_cost_events`, and the same holds for an active change-zone or batch owner. Inline-scanning them would dispatch them now, and their owner would collect them again.
- Required shape: `resolve_tap_mana_triggers_inline` → `take_inline_trigger_span`, and no scan of what the announcement took. Record the `ManaAdded`-mode residual (Q3).

**C3 [behavior]. Specifics for (ii) `commit_activation_announcement(state, collection: Option<Box<_>>, events)`.**
- (a) `None`: take the scope (to `Unowned`) and return. This replaces `end_activation_announcement` at `:7344`.
- (b) `Some` with an `Unowned` scope: add `debug_assert!` and use start 0 (today's behavior), **not** `events.len()` as the spell commit does. A carrier-bearing activation the owner enumeration missed would otherwise silently drop its payment triggers.
- (c) Share the collect, delayed and claim body with `commit_spell_announcement` as one private function over `(collection, events, start)`. Do not copy it.
- (d) Claim `AllCollectors` over `start..len` minus retained.
- (e) Put the Q2 no-loss argument (every ledger writer pairs a claim with a collection) in the function's doc comment.

**C4 [behavior]. P2 becomes rows, not a gate.**
- Add T12c (Scry child + `lose(1)` + Witness) and T12d (the T12 child under a Cove root paying a sorcery's `ManaPayment` + Witness).
- Each asserts with no `drive`: exactly one Witness `TriggeredAbility`, and the expected position (T12c: [B, W]; T12d: W in the cast's batch, above the sorcery).
- Each has a revert-check: drop C1's routing at the exercised site → red.

**C5 [text].** Site 1's comment is replaced with the Q4 text. P3 is added as site 1's discriminating row in the gate↔row map, replacing T12b in that role. T12b stays as F1's row.

**F2, F3, T1:** agree as r2 wrote them. For T10f, add the APNAP precondition (Q5).

## Pre-existing (not blocking)

- **Out-of-buffer replays dispatch immediately under a live announcement.**
  - Sites: `replay_deferred_entry_events` (`engine_replacement.rs:2946`) and the `ChooseBranch` replay (`engine_resolution_choices.rs:5714`). They call `process_triggers` on events outside the action buffer, so they would place triggers below an activation that is still being announced.
  - Fix route: a carrier-collect entry (no claim) when an announcement is live.
- **`push_ability_entry`'s unfiltered full-buffer collect** probably double-collects occurrences that a same-buffer mana frame or sacrifice settlement already claimed, for carrier-bearing activations. Unprobed; (ii) removes it.
- **Site 2's PassPriority route** runs `pass_priority_once_with_pipeline` before site 2. Only a root made due by a PassPriority action could see the SBA mid-activation effect there, and that is reachable only through residual 2's `Unfinished` exits.
