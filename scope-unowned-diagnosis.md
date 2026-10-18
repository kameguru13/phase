# Diagnosis: "a carrier-bearing activation committed with no announcement scope"

This was a read-only trace of `/home/user/wt-merge` at 89113b3, the merge of d3f002d. Nothing was built or run.

The failing row is `activation_cost_target_settlement::every_target_settlement_route_prices_the_committed_targets`, segment **R3**, the modal auto-target segment (line 1813). The other segments of that test (R2, R5, R6, R9) do not reach the arm (see §1.4).

## 1. Why the scope is `Unowned` at the commit

### 1.1 The R3 sequence

**Action 1: `ActivateAbility`**
- `apply_action_boundary_core` calls `begin_action` (engine.rs:1590). No owner is live, so the scope is `Unowned`.
- `activate_with_cost_carrier` calls `begin_announcement(state, events.len())` (casting.rs:26681), which sets the scope to `From{0}`.
- The ability is modal, so it returns `WaitingFor::AbilityModeChoice { is_activated: true, ability_index: Some(_), ability_cost, activation_cost_snapshot, mode_abilities, .. }` (casting.rs:27064).
- **No `PendingCast` exists yet.** The activation is held only in the fields of that prompt: `ability_cost`, `activation_cost_snapshot` and `mode_abilities`. There is no carrier anywhere.
- `end_action` stages nothing, because `pending_trigger_carrier_owner()` is `None`.

**Action 2: `SelectModes{[0]}`**
- `begin_action` (announcement_triggers.rs:294) runs `state.live_pending_cast_owners()`, and the result is **empty**:
  - External: `state.pending_cast` is `None`.
  - Prompt: `WaitingFor::pending_cast_ref()` (game_state.rs:18532) has no arm for `AbilityModeChoice`, so it returns `None`.
  - DeferredLifeCost, CostMove and DiscardForCost are all `None`.
- So the scope becomes `AnnouncementEventScope::Unowned`.
- `handle_ability_mode_choice` calls `handle_activated_mode_choice` (engine_modes.rs:163). Nothing on this path calls `begin_announcement`. Its only callers are the spell announcement (casting.rs:18618) and `activate_with_cost_carrier` (casting.rs:26681).
- The mode's only legal target is auto-selected (engine_modes.rs:288), and `emit_targeting_events` runs (`BecomesTarget`, :296).
- **The carrier is installed** at engine_modes.rs:316 (`pending.begin_activation_trigger_collection()`), on a local `PendingCast` that is handed straight to `finish_target_selected_activated_ability_at_payment_boundary`.
- From there the call chain is:
  1. `run_settled_activation_tail`
  2. `finalize_automatic_mana_payment`: R3's `{2}+{2}` is paid from the pool automatically.
  3. `finish_activation_after_automatic_mana_payment`
  4. `finish_activated_ability_at_payment_boundary`
  5. `push_ability_entry` (casting_costs.rs:7395)
  6. `commit_activation_announcement(Some(carrier))`
- At step 6 the scope is still `Unowned`, so the `debug_assert!` fires (announcement_triggers.rs:408-413).

### 1.2 Root cause

`begin_action` uses "some `PendingCastOwner` is live" to decide whether an announcement is already live. Exactly one announcement step does not satisfy that test: the **activated** `AbilityModeChoice`.
- CR 602.2a announces the activation in action 1.
- CR 700.2a + CR 601.2b (via CR 602.2b): the modes are chosen as part of activating it.
- The engine builds the activation's `PendingCast` only after the modes are known.

So the announcement is live at the start of action 2, but `begin_action` cannot see it, and no later authority calls `begin_announcement`. C3(b) assumed that the owner enumeration catches every carrier-bearing activation, and this case shows the assumption is false.

### 1.3 The defect is not only the assert

On this route `Unowned` also disables `end_action`'s staging, because `announcement_owned_from` returns `None`. If the `SelectModes` action **pauses** after declaring targets or paying part of the cost, its events are never collected:
- Pause points include the `PayCost` (sacrifice) prompt and a `ManaPayment` prompt that cannot be auto-paid.
- The lost events include `BecomesTarget` from the auto-declared target and any non-interactive cost already paid on the untargeted `settle_activation_cost` path.
- The mechanism:
  - `end_action` stages nothing.
  - The action returns a non-Priority wait, so no post-action pipeline runs (engine.rs ~16436).
  - The next action's `begin_action` sets `From{0}` over a **new** buffer.
  - The events are therefore never staged, claimed or scanned. For example, Hojo's "becomes the target of an activated ability" trigger is lost, which violates CR 603.2.
- R3 does not pause (the pool auto-pays), so it shows only the assert. The paused variant of the same route loses triggers silently in release.

### 1.4 Routes that share the shape

The shape is: an activation whose announcement continues in a later action, while no `PendingCastOwner` holds it.

| Prompt the activation pauses on | Where the activation lives | Scope at next `begin_action` | Hits the arm? |
|---|---|---|---|
| **`AbilityModeChoice{is_activated:true}`** | prompt fields only, no `PendingCast` | **Unowned** | **Yes.** Auto-target (:288) and random-target (:282) modes reach `commit(Some)` in the same action. The target-prompt mode (:331, `TargetSelection` carries it), the X modes (:220/:246, `state.pending_cast`) and the untargeted `settle_activation_cost` mode (:352) do not reach the arm, but the action's own events are not staged if it pauses (§1.3). |
| `ChooseXValue` | prompt `pending_cast` | `From{0}` | no (R6) |
| `TargetSelection` | prompt `pending_cast` (carrier installed in `begin_activated_target_selection`) | `From{0}` | no (R2) |
| `DistributeAmong` | `state.pending_cast` (External), casting_targets.rs:330 | `From{0}` | no (R9) |
| `OptionalCostChoice`, `OrderCostReductions`, `ActivationCostOneOfChoice`, `CostTypeChoice`, `ChooseAnnouncingOpponent` | prompt `pending_cast` | `From{0}` | no |
| `PayCost{CostResume::Spell}` | prompt `pending_cast` | `From{0}` | no |
| `ManaPayment`, variable-speed `NamedChoice` | `state.pending_cast` | `From{0}` | no |
| Replacement-paused cost moves: `Cast`, `SacrificeForCost`, `CounterCostRemovalPayment`, `ActivationMillPayment`, `DiscardForCost`, `DeferredLifeCost::Cast` | CostMove / DiscardForCost / DeferredLifeCost owners | `From{0}` | no |
| `PendingCostMoveResume::LoyaltyActivation` | resume root, no `PendingCast`, **no carrier field** | Unowned | No: commit sees `None`. This is a separate gap: the loyalty-replacement pause stages none of its events. It is out of scope here. |
| `EquipTarget`, `CrewVehicle`, `StationTarget`, `SaddleMount` | prompt, no `PendingCast` | Unowned | No: `push_keyword_action` does not go through the carrier or the commit, and it returns Priority, so the pipeline collects. |
| Activation begun in the same action (R5, and auto-target from `ActivateAbility`) | `begin_announcement` at casting.rs:26681 | `From{start}` | no |

The mode prompt is the only route that reaches the assert. The deferred integration run may also trip it in other modal-activation auto-target tests. Candidates to check in the failing-run log: `stack_entry_declared_targets`, `announced_counter_recipient_set`, `become_prepared_declared_target`, `period_confirm_rows`, `x_mana_sacrifice_residual`, `cost_zone_pipeline` and `activation_cost_reduction_election`, which all combine `SelectModes` with `ActivateAbility`.

## 2. Is start 0 correct for this route?

Yes. Every event in a `SelectModes` action answering an activated mode prompt belongs to the activation's announcement:
- The announcement began in the earlier `ActivateAbility` action (CR 602.2a).
- The mode answer is an announcement step (CR 700.2a "as part of ... activating that ability", CR 601.2b via CR 602.2b).
- Nothing emits before the handler:
  - Between `begin_action` and the reducer match, `apply_action` emits nothing for this action. The only pre-match emitters are the debug, permission and pass-priority arms.
  - The reducer arm (engine.rs:15611) passes the empty buffer to the handler.
- Inside the handler, in order: the announcement record, the targeting events (CR 601.2c), cost and mana payment (CR 601.2h), the stack push, and `AbilityActivated`.
- The commit claims `[0..len]` at commit time. The post-action pipeline that follows sees the scope already taken (`mem::take` sets it to `Unowned`), so stage/commit does not take later events.

The fallback therefore produces the correct span for R3. The defect is that the scope was **never established**, which also turns off staging on the paused variant.

## 3. Recommended fix

### Option evaluation

- **(a) Drop the assert and document start 0.** Rejected.
  - It hides the symptom only at the commit.
  - It leaves §1.3 in place: a paused `SelectModes` action still loses its targeting and cost events.
  - It also removes the guard that C3(b) added specifically to catch owner-enumeration misses.
- **(b) as phrased (read `next_wait.pending_cast_ref()` like upstream's `activation_announcement_is_live`).** This does not work.
  - At `begin_action` the incoming wait is `AbilityModeChoice`, and its `pending_cast_ref()` is `None`. That is exactly why the owner list is empty.
  - `next_wait` is the reducer's *output* prompt, which `begin_action` does not have.
  - The principle behind (b) is right, though: `begin_action` should recognise every announcement that is live at entry. That is the recommendation below, as (b′).
- **(c) Call `begin_announcement` in the continuation handler.** This works, and only one handler needs it: the top of `handle_activated_mode_choice`. However:
  - It records the mode answer as the announcement's *inception*, which is wrong in the documented model. `AnnouncementEventScope::From` says `start` is "the announcement's inception offset (0 when it was already live at action entry)".
  - It places scope custody in a reducer handler rather than in `announcement_triggers`.
  - It is numerically identical, because `events.len()` is 0 at handler entry. It is an acceptable fallback if (b′) is refused.

### Recommendation (b′): one authority, `begin_action`'s liveness predicate

In `crates/engine/src/game/announcement_triggers.rs`, add a private predicate next to `begin_action` and use it in place of the bare owner check:

```rust
/// CR 602.2a + CR 602.2b + CR 700.2a: whether an announcement is already under
/// way as an action begins. Every announcement step holds a live `PendingCast`
/// except one: an activated ability's mode choice. The activation was announced
/// (CR 602.2a) and its modes are chosen as part of activating it (CR 700.2a,
/// CR 601.2b), but its `PendingCast` is built only once the modes are known.
fn announcement_live_at_action_entry(state: &GameState) -> bool {
    !state.live_pending_cast_owners().is_empty()
        || matches!(
            state.waiting_for,
            WaitingFor::AbilityModeChoice {
                is_activated: true,
                ..
            }
        )
}
```

and in `begin_action` (announcement_triggers.rs:299):

```rust
    state.announcement_event_scope = if announcement_live_at_action_entry(state) {
        AnnouncementEventScope::From { start: 0 }
    } else {
        AnnouncementEventScope::Unowned
    };
```

- **Keep** the `debug_assert!` and the start-0 release fallback in `commit_activation_announcement`. With the predicate fixed, the C3(b) premise holds again, so the assert once more guards against a future owner that the enumeration misses.
- Optionally extend `begin_action`'s doc comment: "An announcement is live at entry when a `PendingCast` owner holds it, or when the action answers an activated ability's mode prompt."
- `is_activated: false` (a triggered ability's mode prompt) must stay `Unowned`. That choice belongs to trigger construction (CR 603.3c), not to an announcement.

All CR numbers above were checked with grep against `/home/user/phase-personal/docs/MagicCompRules.txt`:
- 601.2b: modal announce
- 601.2c: targets
- 602.2a: announce activation
- 602.2b: rest of 601.2b–i applies
- 603.2: triggers on event
- 603.2c: once per occurrence
- 603.3 and 603.3b: placed when a player would receive priority, APNAP batch
- 700.2a: modes chosen as part of activating

### Rows

- **Failing row, red to green.** `every_target_settlement_route_prices_the_committed_targets` R3. The scope becomes `From{0}` at entry, so the commit takes the same `[0..len]` span the fallback used, and the assert no longer fires. The R3 price and settlement assertions (`open_settlements == 1`, paid 4) are unaffected, because the scope does not touch the cost lock.
- **The arm's own discriminator, new row (recommended).** Neither `cost_removed_counter_amount` nor `announcement_trigger_collection` has a row with a modal activation, so no existing row distinguishes (a) from (b′). Proposed row, in `announcement_trigger_collection.rs` or alongside R3:
  - P0 controls Professor Hojo (HOJO const) as its only creature.
  - The modal ability is `{2}, Sacrifice an artifact: Choose one — • Tap target creature you control. • You gain 1 life.`
  - P0 has two artifacts available for the sacrifice.
  - `SelectModes{[0]}` auto-targets Hojo and should pause on `PayCost`. **Reach guard:** the wait after `SelectModes` is `PayCost`, not Priority.
  - Then pay with `SelectCards`.
  - **Assert:** exactly one Hojo draw trigger on the stack, above the activation, in its batch (CR 603.3b).
  - **Revert-checks:**
    - Without (b′) the trigger is missing, because the `BecomesTarget` event was never staged.
    - With (a) only, it is still missing.
    - With (b′) it is green.
- **Regression net.** The Breya rows in the same file, which pass through the interactive `TargetSelection` and must stay green:
  - RT-1: `a_modal_activations_open_carrier_survives_a_restore_at_mode_choice`, including the `round_trip` at the mode choice; the predicate reads the restored `waiting_for`.
  - M1: `a_deferred_modal_activation_whose_chosen_mode_has_no_target_locks_at_mode_choice`.
  - The `{X}: Choose one` row (~2124).
  - `issue_2852_breya_sacrifice_cost`.

### Risk to F4 staging and the watermark

The risk is low.
- **Before a carrier exists.** `From{0}` at entry changes nothing until a carrier owner exists. `announcement_owned_from` and `stage_announcement_trigger_events` both require `pending_trigger_carrier_owner()`. Until `handle_activated_mode_choice` stores or returns a `PendingCast`, `take_inline_trigger_span` returns the whole span and nothing is staged.
- **Once one exists.** End-of-action staging now runs for this action, which is the intended F4 custody. `retained_by_cost_owners` still excludes events that cost roots own, and the claim stays `OrdinaryCollectorsOnly`.
- **Watermark.** `advanced_to` behaves exactly as on every other `From{0}` continuation, such as `ChooseXValue` and `TargetSelection`. The pipeline's advance (engine_priority.rs:169) and `mem::take` at the commit still end the scope.
- **`CancelCast` at the activated mode prompt.** The scope becomes `From{0}` with no owner and a Priority result: there is no staging, and the next `begin_action` resets it. The scope was already left at `From{0}` after action 1's `begin_announcement`, so no new stale state is introduced.
- **Spell carrier.** `live_spell_announcement` is unchanged (it uses the owner list), so the in-flight spell carrier logic is untouched.

### Separate observations (not part of this fix)

- **`LoyaltyActivation` cost-move root.** It holds no `PendingCast` and no carrier, so a replacement-paused loyalty payment stages none of its events (Unowned on resume). This is the same class of liveness gap, but it does not reach the assert because the commit sees `None`. Record it as a residual or follow-up.
