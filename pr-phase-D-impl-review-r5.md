# Phase D implementation review r5 (c909249)

Review Head: c909249. The primary diff `384bb92..c909249` was reproduced exactly. It touches 4 files: `game/engine.rs`, `game/engine_resolution_choices.rs`, `types/game_state.rs` and `tests/integration/cost_removed_counter_amount.rs`.

Completion Gate: not assessed. This review was read-only: no builds, tests or probes were run, and the executor's red/green claims were not re-run.
- Under the deferred-verification policy, the full `test-engine` run is still owed.
- The brief's item 6 asked for engine clippy to be run before committing. The executor's report does not mention it, so that evidence is missing.

Maintainer-Simulation Gate: not assessed, because no matrix was supplied.

Verdict: **behavior=1 text=0 machinery=0**.
- **R5-F1 (MED, latent).** The R4-F1 ledger filter was not applied to one resume-then-collect site: the CategoryChoice keeper arm (Cataclysm class). That site matches R4-F1's own predicate, but the r4 disposition table wrongly listed it as "no collector follows", and the brief inherited the omission. The executor followed the brief and flagged the site as "first sacrifice park unchanged".
- **Blocking:** yes, as the completion of R4-F1. The fix is the same five-line change already made at the two sibling keeper arms. It needs no new row: the siblings have none either (residual 3 / latent).

CR numbers grep-verified for this review against `docs/MagicCompRules.txt`: 101.4, 117.3c, 117.5, 118.12a, 601.2h, 602.2b, 603.2, 603.2c, 603.3, 605.3b, 608.2c, 608.2e, 704.3. Every CR number in the diff resolves, and each describes the site it annotates.

---

## A. Verification of the r4 fixes

### R4-F1 (resume-before-collect duplicate): correct where applied, but one site was missed (R5-F1)

**The helper.** `unclaimed_observer_events(state, events, start, end)` (`engine_resolution_choices.rs:745-760`) does three things:
- It returns empty when `end <= start`. That guard matters: `filter_consumed_trigger_events_from(&events[..end], start, ..)` would panic slicing `[start..]` past `end`.
- It filters `events[..end]` from `start` against the ledger, with `Ordinary` as the requester.
- The occurrence identities are computed against a prefix of the full action buffer, so they agree with every writer that claims against that buffer (the commit's `claim_indices`, the logical-owner marks and staging).

**`batch_or_drain_observer_triggers` (`:792-813`).**
- The move slice and the `PlayerPerformedAction` completion chain are both filtered.
- `completion_start = min(event_slice_end, owned.end) <= owned.end`, so the second call never inverts its range.
- The PhaseChanged and `zone_changes_are_logically_owned` filters still apply after the ledger filter.
- This one change covers every caller in the r4 table (`:6036`, `:6386`, `:6651`, `:7443`, `:7563`→`:7592`, `:8016`, `:8050`).

**DiscardChoice pair loop (`:6068-6090`).** The filter is applied to each segment. Both segments end at or before `owned.end`, so they cannot overlap the span that custody staged and claimed.

**Keeper B2 arms.**
- Changed: KeepWithinTotalPower (`:8634` → `:8643-8656`) and the exact-cardinality keeper (`:8735` → `:8744-8757`).
- Not changed: CategoryChoice (`:8413` → `:8429-8440`). See R5-F1.

**Can the filter now drop an event that was legitimately unclaimed (CR 603.2c / 603.3)?** No. I checked every production writer of `consumed_before_priority_trigger_events`:

| Writer | Claim scope | Paired collection |
|---|---|---|
| `commit_announcement_span` | `AllCollectors` | Collects ordinary triggers into the carrier and commits them to deferred; collects delayed contexts. Cost-owner-retained events are left unclaimed. |
| `stage_announcement_trigger_events` → `claim_ordinary` | `OrdinaryCollectorsOnly` | Carrier collection. |
| `mark_logical_zone_events_consumed_before_priority` (`zone_pipeline.rs:1249`, `:1657`; `effects/mod.rs:2498`) | — | Follows `complete_logical_zone_trigger_collection`. |
| `collect_delayed_triggers_into_deferred` | `AllCollectors` | Only at `engine.rs:10576`, `casting_costs.rs:3805` and the SBA path. In each place an ordinary collection over the same events comes first. |
| `apply_resolved_trigger_collection` | — | Run-batch claims, after ordinary and delayed collection. |
| `stack.rs` bulk paths | — | Collected on the clone. |

Every claim is therefore paired with an ordinary collection, so the filter removes only re-collections. Both `collect_triggers_into_deferred` and the touched collectors are ordinary-only, so using `Ordinary` as the requester is correct: an `OrdinaryCollectorsOnly` claim should suppress them.

**Retained events.** Cost-owner-retained events stay unclaimed after a commit, so these collectors still see them, exactly as before the change. A filter can only narrow, so it cannot newly collect a retained event.

**Plain main paths (no counter-cost root).** The ledger is cleared at action entry (`clear_player_action_transients`) and at `finish_action_boundary_with_lifecycle`. Within a resolution-choice action, the only claims the filter can meet are claims that action's own collectors already made: logical-owner marks, staging, or a commit. So on a main path the change only removes a second collection of an event that was already collected.
- The one theoretical cause of a wrong drop is a mark recorded with sub-slice ordinals (`zone_pipeline.rs:1249` marks against `&events[event_start..]`). It would need two byte-identical `ZoneChanged` events in one buffer, which their records make practically impossible. This is not a finding.

**The two arms the executor left "unchanged per brief".**

| Arm | Lines | Is it safe? |
|---|---|---|
| Token park (EachPlayerCopyChosen) | `:8553-8562` | **Yes.** `each_player_copy_chosen::advance_to_next_player` has no resume (grep: no `resume_pending_continuation_if_priority` / `drain_pending_cost_move_resume` / `drain_pending_continuation` in `each_player_copy_chosen.rs`). Custody is therefore evaluated before any commit could happen. |
| First sacrifice park (CategoryChoice) | `:8413` → `:8429-8440` | **No.** It is the same resume-then-unfiltered-B2 shape as the two keeper arms that were fixed (R5-F1). |

**Scan for other resume-then-collect sites.** I scanned every `resume_pending_continuation_if_priority(state, events)` / `resume_with_error_propagation(state, events)` / `finish_with_continuation(state, _, events)` site in `crates/engine/src/game` for a following custody call or `collect_triggers_into_deferred` within 45 lines. Only four sites match: `:6049` (fixed), `:8413` (**unfixed**), `:8634` (fixed) and `:8735` (fixed).
- In `engine_replacement.rs`, the `defer_observer_triggers_for_paused_choice` calls at `:1271` and `:1281` run before the `ReplacementDelivered` resume at `:1366`/`:1399`. No collector follows that resume. So they are unaffected.

### R4-F2 (second promote caller): correct

**The guard.** `GameState::promote_paused_drain_tail_within_counter_cost_subtree` (`game_state.rs:27688-27713`) is a private method, and both callers use it:
- `take_completed_multi_draw_frame` (`:27754-27756`);
- `finish_active_paused_post_replacement_dispatch` (`:27828`).

`grep` finds no other caller of the raw `ResolutionStack::promote_ability_continuation_after_post_replacement_draw`.

**The second caller's stack shapes.**
1. **The frame is not empty after the finish.** The stack is `[.., C, PR]` and the guard's `len < depth + 2` check is exact, as at the draw caller.
2. **The frame was emptied and removed.** The top is whatever lay below.
   - If the top is the enclosing PR at `depth-1`, then `len = depth < depth+2`. The guard skips, which is correct, because anything below the enclosing frame is outer work.
   - If the top is an AC, the guard either skips (when `len < depth+2`) or the raw promote returns `Ok(false)` or an error, which the caller ignores as before.

**No legitimate promote is blocked.** A tail parked by the child's dispatch sits at index `>= depth`, which gives `len >= depth + 2`.

### R4-F3 (T10f payer): correct

- **Fixture.** `outer_player_scope_seat_waits_for_the_mana_root` now builds `lose(1).unless_pay(.. payer: TargetFilter::ScopedPlayer)` with `player_scope = All` (`cost_removed_counter_amount.rs:3562-3570`). That is the shape the parser emits for "unless that player pays".
- **Assertion.** The post-decline match now requires `player == P1` as well as `pending_effect.controller == P1` (`:3614-3624`), so a prompt sent to the wrong seat now fails the row.
- **Reach.** The first prompt's `P0` reach assertion is unchanged.
- **Coverage.** The helper is parameterized, so both T10f (Draw rider) and T10f′ (GainLife rider) pick up the fixture and the assertion.

### R4-F4 (orphaned events on partial progress): correct; double staging is not possible

`resume_parked_counter_cost_root_at_action_boundary` (`engine.rs:1747-1773`) records `events_before` and, when `!progressed`, calls `stage_announcement_trigger_events(state, events, events_before)`.

**No double staging.**
- Staging reads `unclaimed_events`, claims the span `OrdinaryCollectorsOnly`, and advances `announcement_event_scope` to `events.len()` (`announcement_triggers.rs:203-207`, `:239`).
- **Site 1.** `run_post_action_pipeline` stages again at the start of its pass. It finds the scope already advanced and the events already claimed, so it adds nothing. The generic ordinary scan skips those events because of the claim, and the delayed scan still sees them. This is the same outcome as before the change.
- **Site 2.** `end_action` stages only for non-Priority waits, so F4 is the only stager on this path. That was the point of the finding. When `realized_token_entry` or an owed final part triggers the convergence pass, the pipeline's own staging is also a no-op there.

**Remaining case.** With no live announcement, staging is a no-op. This stays inside residual 3, as the brief says.

### R4-F5 (text): correct

- The function and the local are renamed. Both call-site comments use the review's replacement wording.
- `grep` finds no remaining `resume_due_counter_cost_root_at_action_boundary`, `resumed_due_root` or "made due resumes" anywhere in `crates/`.

### Executor's unrequested text

The added sentences are:
- the `unclaimed_observer_events` doc;
- the shared guard's doc;
- the F4 comment;
- the T10f fixture comment;
- the new row's doc.

All of them are accurate. The F4 comment says "the loop advanced part of the subtree", but the same branch also runs when nothing advanced, where staging is a no-op. "Whatever it emitted" covers that case, so this is not a finding.

---

## B. New findings

**[MED] [behavior] R5-F1. The CategoryChoice keeper arm resumes the parked root and then parks its sacrifice slice unfiltered, so the R4-F1 duplicate remains at the one resume-then-collect site the r4 table missed.**

**Evidence.**
- `engine_resolution_choices.rs:8413` (`resume_with_error_propagation(state, events)?`) is followed by the B2 arm at `:8429-8440`:
  ```rust
  let trigger_events: Vec<GameEvent> = events
      [events_before_sacrifice..events_after_sacrifice.min(owned.end)]
      .iter()
      .filter(|ev| !matches!(ev, GameEvent::PhaseChanged { .. }))
      .cloned()
      .collect();
  ```
  This is the same shape as the KeepWithinTotalPower and exact-cardinality arms, which were fixed at `:8648` and `:8749`.
- At 384bb92 the site was at `:8392` → `:8408`. It appears in the r4 predicate command's output, but the r4 table marked it "all other resume sites … no collector follows | unaffected", which is false for this site.

**Failure scenario.** This is the same as R4-F1's keeper arms:
1. A counter-cost replacement's child reaches a Cataclysm-class "choose one of each category, sacrifice the rest" resolution choice.
2. The sacrifice retires the subtree. `resume_with_error_propagation` resumes the root, and `commit_activation_announcement` collects the dies trigger W and claims `0..len` `AllCollectors`, ending the scope.
3. The resumed work then leaves a non-Priority wait, for example an outer `player_scope` leg's prompt or an ordering prompt.
4. The B2 arm finds no live announcement, so `take_inline_trigger_span` returns the whole span. The arm then collects W into `deferred_triggers` a second time.
5. At the next priority window W goes on the stack twice, violating CR 603.2c and CR 603.3.

It is latent and synthetic, like the whole Phase D class. Under the calibration rule it is MED, the same rating the sibling arms got in R4-F1.

**Why it matters.** The R4-F1 fix is incomplete against its own predicate. One of three structurally identical keeper arms still double-collects.

**Proposed fix.** Replace the slice with the same call the siblings use:
```rust
unclaimed_observer_events(state, events, events_before_sacrifice, events_after_sacrifice.min(owned.end))
    .into_iter()
    .filter(|ev| !matches!(ev, GameEvent::PhaseChanged { .. }))
    .collect()
```
- No row is needed: the sibling keeper arms have no discriminating row either (latent, residual 3 shape). Say so in the residual note.
- Verify with `rg -n "resume_with_error_propagation\(state, events\)\?;" -A30 engine_resolution_choices.rs`. Every hit followed by a raw `events[..]` → `collect_triggers_into_deferred` should be gone.

**Blocking:** yes. It completes R4-F1 at the same seam with the same helper.

No other new defect was found in the diff.

## Pre-existing

None new. The r4 items (`resolve_unless_payer` `Controller => active_player`; the delve `Cast{FinalizeDelvedCast}` resume-then-collect on upstream) are not repeated.
- The delve item is now closed by the R4-F1 filter wherever the filter is applied. The CategoryChoice site (R5-F1) is the remaining exception.

---

## C. CR check

The CR annotations added or changed in the diff are below. Every one was grep-verified against `docs/MagicCompRules.txt`.

| Site | CR | Rule text (head) | Fits |
|---|---|---|---|
| `engine.rs:1659` | 601.2h + 602.2b + 605.3b | pay total cost; activation follows 601.2b–i; mana ability resolves immediately | yes (unchanged numbers, reworded) |
| `engine.rs:1762` (F4) | 603.2 + 603.3 | abilities trigger on events; put on the stack the next time a player would receive priority | yes: events wait for the announcement's carrier |
| `engine.rs:16422` | 704.3 + 117.5 + 117.3c + 602.2b + 601.2h + 603.3 | SBAs when a player would get priority; priority after activation; trigger placement | yes (unchanged numbers, reworded) |
| `engine_resolution_choices.rs:740` | 603.2c | triggers only once each time its trigger event occurs | yes: the dedupe |
| `game_state.rs:27688` | 608.2c + 608.2e + 101.4 + 118.12a | instructions in order; multi-player steps APNAP; simultaneous choices APNAP; "unless" | yes (moved from the first caller) |

The renamed comments keep their numbers. None of them use an old format.

---

## D. Tests

**Sacrifice row (`a_dies_trigger_from_a_sacrifice_child_goes_on_the_stack_once_above_the_activation`, `:3778`).**
- It is un-ignored.
- **Reach:** `EffectZoneChoice` containing the fodder, plus `assert_root_is_counter_cost`.
- **Positive shape:** the stack is exactly `[Bottle activation reading 3, W]` (`assert_activation_then_one_trigger`: `len == 2`, bottom is the activation with `cost_removed_counter_amount == Some(3)`, top is the witness trigger), and `witness_entries == 1`.
- A duplicate cannot hide in `deferred_triggers`, because B1 drains the queue to the stack before returning.
- This is not vacuous.

**Discard row (`a_discard_trigger_from_a_discard_child_goes_on_the_stack_once_above_the_activation`, `:3721`).**
- The child is `Effect::Discard { selection: Chosen, count: 1 }` with two cards in hand, so it prompts `DiscardChoice` (the reach is asserted).
- It is an optional replacement post-effect, not a MayCost payment, so `pending_replacement.may_cost_paid` is false. The completion therefore takes `finish_with_continuation` and then the pair loop, which is the site under test.
- It makes the same strong positive assertions as the sacrifice row.
- The executor reports it RED at base and RED with the pair-loop filter reverted. I did not re-run either.

**T10f / T10f′.** The assertions are now discriminating (§A, R4-F3).

**Rows the executor reports as missing.**
- **F2's second caller.** There is no dedicated row. The brief allowed this, because reverting the shared guard affects both callers through one body. A future edit that bypasses the helper at `finish_active_paused_post_replacement_dispatch` would go undetected. That is accepted under the brief and is not a finding.
- **F4 staging and the keeper B2 arms.** There is no row for either. The executor disclosed this, and both are latent (residual 3).

**Counts.** The executor reports the counter-cost suite at 72 passed, 0 failed, 0 ignored. That is consistent with the un-ignore and the one added row. I did not re-run it.
