# Phase D implementation review r3 (8f18b5e)

Review Head: 8f18b5e (primary diff `618fb57..8f18b5e`, reproduced exactly; Phase D span `4483f4b..8f18b5e` read only where it meets the r2 fixes)
Completion Gate: not assessed. This was a read-only review with no builds or probes. Under the deferred-verification policy the executor ran fmt, the CR grep and targeted red/green only. Full `test-engine` is still owed. The new `debug_assert!` in `commit_activation_announcement`'s `Unowned` arm (`announcement_triggers.rs:380-386`) turns a previously silent path into a debug panic, so the deferred activation and announcement suites must actually run.
Maintainer-Simulation Gate: not assessed (no matrix supplied). T10f is still `#[ignore]`d, and gate (a) is still undiscriminated.

Verdict: **behavior=6 text=0 machinery=0**.
- **Blocking:** F1 (MED), F2 (MED) and F3 (MED). F3 can instead be named as a residual, but only if the orchestrator does that explicitly and T10f′ lands.
- **Ride along:** F4, F5 and F6 (LOW).

CR numbers grep-verified for this review: 101.4, 117.3c, 117.5, 118.12, 118.12a, 601.2h, 601.2i, 602.2b, 603.2c, 603.3, 603.3b, 603.7c, 605.1b, 605.3a, 605.3b, 605.4a, 608.2c, 608.2e, 616.1g, 704.3. Every CR annotation the diff adds or moves resolves and describes its site. The site-1 rewrite is correct. `603.7c` is carried over unchanged from `push_ability_entry`.

---

## A. The r2-fix diff

### Custody semantics: correct

- **`announcement_owned_from` (`announcement_triggers.rs:145-151`).**
  - It is exactly the stage's precondition, and `stage_announcement_trigger_events` now reads it (`:193`), so the two cannot diverge.
  - It returns `max(start, scope_start)`, which is ≥ `start`.
- **`take_inline_trigger_span` (`:169-180`).** It returns `start..owned_from.min(len)`, which is empty when the announcement owns the whole span and is the unowned prefix otherwise.
  - The stage claims `start..len` (`claim_ordinary`, `:240`) and advances the scope (`:207`).
  - So `end_action`'s stage-from-0 and the pipeline's stage (`engine_priority.rs:156`) cannot take the same occurrences twice.
  - Nothing is dropped. The owned part is in the carrier, and the commit (`commit_announcement_span`) collects only what is unclaimed from the advanced scope.
  - Delayed triggers on the staged span stay visible to the pipeline's delayed scan, because the claim is `OrdinaryCollectorsOnly`.
- **Retained events.** Retained (`retained_by_cost_owners`) events are claimed by the stage but not collected. Their owners settle them from their own carried copies (`casting_costs.rs:3790-3810` carries `deferred_cost_events` by value), so the claim is harmless.
- **`resolve_tap_mana_triggers_inline` ordering.** It runs only in the custody branch, before the stage. It resolves the `TapsForMana` and `ManaAbilityProduced` triggered mana abilities (CR 605.4a) and flips their events, so the carrier's collector skips them. The non-custody branch is byte-identical.
- **`commit_activation_announcement`.**
  - The `None` gate is preserved: `mem::take` leaves the scope `Unowned`, exactly as `end_activation_announcement` did, and nothing is collected.
  - The `Some` + `Unowned` case asserts and starts at 0, per C3(b).
  - The shared body `commit_announcement_span` is not duplicated.
- **Routed sites.** All routed sites in the fix-design census are present:
  - the mana arm;
  - B1/B2 of `batch_or_drain_observer_triggers`;
  - `defer_observer_triggers_for_paused_choice`, plus its callers `engine_replacement.rs:1271/1281` and `engine_resolution_choices.rs:5986`, which pass the action buffer;
  - `collect_search_observer_triggers`;
  - the discard pair loop;
  - the four B2 parks;
  - the change-zone iteration;
  - `copy_spell`.
- **`copy_spell` routing is right.** `SpellCopied` is pushed into `events` (`copy_spell.rs:284`) and staged by index. `park_observer_triggers_if_paused` already excludes `SpellCopied`.

The census itself missed three collectors (F1). The "announced" predicate is also imprecise (F6).

---

## B. Deviation 3 (T10f): the cause is a positional frame swap on draw completion, not the splice

**The executor's splice hypothesis is refuted.**
- `append_to_pending_continuation` can splice into P1's leg only while P1's `AbilityContinuation` is the **top** frame. Both `active_ability_continuation()` and `active_ability_continuation_frame_mut()` are top-only (`resolution.rs:1030-1045`; `effects/mod.rs:2795-2828`).
- When the child's `RemoveCounter` pauses, the top frame is the child's `CounterRemovals` queue, so the generic pause path parks the Draw rider parent-of-active (`effects/mod.rs:18936-19032`, then `prepend_to_pending_continuation_with_producer` at `:2912-2917`).
- Stack: `[AC(P1), PR(child, Paused), AC(rider), CounterRemovals]`.

**Actual cause (high-confidence trace, unprobed):**
1. DECLINE, DECLINE: the queue completes. The rider's frame is drained (`effects/mod.rs:1260-1290`), leaving `[AC(P1), PR(Paused)]`. The rider's `Draw 2` pushes a `MultiDraw` child above the paused PR.
2. On completion, `draw.rs:858-860` calls `take_completed_multi_draw_frame` (`game_state.rs:27691-27733`).
   - Because `[PR(Paused), MultiDraw]` is a "draw pair", it calls `promote_ability_continuation_after_post_replacement_draw` (`resolution.rs:3295-3316`).
   - That function swaps **any** `AbilityContinuation` directly below a paused PR above it, so `[AC(P1), PR]` becomes `[PR, AC(P1)]`. It assumes that continuation is the dispatch's own tail. Here it is P1's outer `player_scope` leg.
   - `draw.rs:865` then skips the paused-dispatch retire, because a continuation is now on top.
3. Back in the rider's drain, at `effects/mod.rs:1373-1410`:
   - The root is not due: `ReplacementSubtreeBoundary::is_retired` is `stack.len() <= resolution_depth`, and 2 > 1 (`game_state.rs:6092-6097`).
   - `finish_active_paused_post_replacement_dispatch` (`:1404`) finishes the PR's drain through the paired-parent slot (`resolution.rs:2845-2860`). It cannot pop the PR, because the PR is not the top (`game_state.rs:remove_empty_active_post_replacement_frame`).
   - `:1408` then recurses into `drain_pending_continuation`.
   - Gate (a) (`:1213`) still reads `Unfinished` (len 2), so it **runs P1's leg**, which raises P1's `UnlessPayment`.
   - Now `[PR(empty)]`, len 1 ≤ 1, reads `Retired`. The Cove root resumes and restores P0's `UnlessPayment` over P1's prompt.
4. Every observed symptom matches:
   - frames `[PostReplacement]`;
   - the AC is gone;
   - P1 is never asked and P1's life is unchanged;
   - `ManaAdded{cove}` is present;
   - two `CardDrawn` (the rider).

Probe to confirm: at `draw.rs:858`, log `resolution_stack` frame kinds before and after `take_completed_multi_draw_frame`. Expected: `[AC, PR(Paused), MultiDraw]` becomes `[PR, AC]`.

**What CR requires.**
- P0 activates the Cove while paying P0's unless cost (CR 605.3a), and the mana ability completes immediately (CR 605.3b).
- The resolving Taxer ability then continues in written order (CR 608.2c). Its "each player … unless" step asks each player in APNAP order (CR 608.2e + CR 101.4; CR 118.12a: unless means that player may pay).
- P1 must be offered P1's own unless choice after P0's. Dropping it is a rules violation.

**Classification: (i).**
- The swap function is pre-existing upstream (`upstream/main` 39fb1eb `game_state.rs:27513`, the same `draw.rs:859`).
- Without a parked root, its positional assumption is harmless: the outer continuation it lifts is the next written instruction anyway.
- It becomes a CR-visible defect only because Phase D's depth boundary and gate (a) are meant to hold outer frames below a parked counter-cost root, and the swap moves a subtree frame below the outer frame without changing the stack length.
- This is Phase D's claimed configuration: T10f exists to prove it. It is not a non-counter flow.

**Recommendation: fix it in this PR. It is small and uses the existing boundary.**
- **Fix.** In `take_completed_multi_draw_frame` (or as a predicate passed to the promote), do not promote when a counter-cost root is parked and the continuation below the PR sits below its boundary (`index < paused_counter_removal().replacement_subtree.resolution_depth`). Such a continuation is outer work and never rises above the root's subtree.
  - A legitimate tail parked "outside the pair" during the subtree sits at or above the boundary, so it is still promoted.
  - With the guard, `draw.rs:865` retires the paused PR, the stack is `[AC(P1)]`, gate (a) fires and T10f passes.
  - Cost: about 15 lines plus a CR annotation (CR 608.2c + CR 605.3b), with T10f (Draw rider) as its discriminating row.
- **Fallback.** If the orchestrator will not take another fix, name it **residual 9** ("a replacement child's paired draw lifts an outer continuation above a parked counter-cost root"), with the same guard as the follow-up. Then T10f′ below becomes mandatory.

**Gate (a) can be discriminated now, independent of the draw defect.**
- Add **T10f′**: T10f's board with `t3_child_execute` (GainLife rider) instead of `t10_child_execute`. A GainLife rider raises no `MultiDraw`, so there is no swap.
- Trace: the rider runs at `:1260+`. `:1404` retires the PR as the top, leaving `[AC(P1)]`. The recursion at `:1408` hits gate (a) (`:1213`, len 1 ≤ 1, due) and returns. The root resumes, and P0's prompt is re-offered with P1's leg intact.
- With gate (a) removed, the recursion runs P1's leg and the row goes red.
- So gate (a) remains justified and should not be deleted. Deleting it reintroduces r1-F1 for this class, as r2 §2 said.

---

## C. Deviation 4 (T12e rider-free child): a Phase D defect in the F1 class

**Confirmed.**
- `handle_choose_mana_effect` (`effects/mana.rs:427-432`) calls only `drain_pending_continuation`.
- In `drain_pending_continuation`, the paused post-replacement dispatch is retired only inside the continuation-drain block (`effects/mod.rs:1260-1410`, retire at `:1404`). A child whose **last** instruction is the mana effect has no continuation, so the PR stays `Paused` and the root reads `Unfinished`.
- The FD1 authority `resume_due_counter_cost_root_at_action_boundary` (`engine.rs:1741-1752`) returns at `:1746` unless the root is already **due**. So neither site 1 (`engine.rs:16401-16409`) nor site 2 (`engine.rs:1659`) ever runs the D5 loop.
- The D5 loop (`resume_pending_continuation_if_priority`, `engine.rs:8931-8946`) would advance this exact subtree:
  - `resume_resolution_frames`' `PostReplacement` arm retires a `Paused` resident (`effects/mod.rs:1806-1815`);
  - `resume_due_counter_cost_root` then resumes the root.
- The Scry route works only because its handler calls `finish_with_continuation`, which runs the D5 loop (`engine_resolution_choices.rs:9456-9465`). T1, the rider-free Scry child, places the activation in the same action.

**Why this is not residual 2 or 3.**
- Residual 2 is the Prevented-arm blocks.
- Residual 3 is a subtree the loop cannot advance.
- Here the subtree is advanceable. The authority's `is_due` precondition simply never lets the loop run.

**Consequences.** The pipeline then runs SBAs with the root parked (CR 704.3 / 117.5 mid-activation). The activation is placed only at the next priority pass, contrary to CR 602.2b + 601.2h/601.2i and CR 117.3c. A rider-free "add one mana of any color" child is the plainest interactive child, so this is the maintainer's F1: "resume the counter-cost owner after a replacement's interactive child".

**Minimal fix (the authority seam, with no handler patch, per FD1).**
- Change the precondition at `engine.rs:1746` from `counter_cost_root_is_due()` to "a counter-cost root is parked" (`counter_cost_root_subtree() != CounterCostRootSubtree::Absent`) under a `Priority` result, and run the existing D5 loop.
- Return `Some` only if the loop changed the wait or the root left the slot, so site 2 does not run a convergence pass for a no-progress (residual 3) subtree.
- This also narrows residual 2's "Priority surfaced mid-activation" for every advanceable subtree. Update the residual 2 text.

**Row T12f** `a_rider_free_mana_choice_child_places_the_activation_in_the_same_action`:
- Child: `mana_choice_child()` without `.sub_ability(lose(1))`.
- After `choose_green`, with no `drive`, assert:
  - `stack.len() == 1`;
  - `top_stack_count == Some(3)`;
  - pool G == 1;
  - `payment_ownership_is_clear`;
  - no `PostReplacement` frame.
- Revert check: restore the `is_due` precondition. The row goes red (empty stack, root parked).
- Optionally, T12e can return to the plan's rider-free child once T12f is green.

---

## Findings

**[MED] [behavior] F1. The custody census missed three same-buffer collectors that do not claim, so the r2 duplicate still occurs for the maintainer's own "interactive Scry child" class.**
- Predicate: a collector that reads a slice of the current action buffer into `deferred_triggers`, records no claim in `consumed_before_priority_trigger_events`, and can run while `pending_trigger_carrier_owner().is_some()`.
- Command:
  ```
  rg -n "park_observer_triggers_if_paused\(|collect_and_drain_observer_triggers_if_settled\(|defer_observer_triggers_from\(" crates/engine/src/game --glob '!*_tests.rs'
  ```
- Matching sites:
  1. `triggers.rs:8400` `park_observer_triggers_if_paused`. Callers:
     - `engine_resolution_choices.rs:2096` (ScryChoice continuation): **route**;
     - `engine_payment_choices.rs:251` (OptionalEffectChoice): **route**;
     - `morph.rs:713`: **route** (inert under an announcement today; routing is free).
  2. `triggers.rs:8437` `collect_and_drain_observer_triggers_if_settled`. Callers `engine_payment_choices.rs:420` and `:567` (OpponentMayChoice): **route**.
     - This handler returns directly. Site 2 then pushes the activation, and `commit_announcement_span` re-collects the same occurrences as "unclaimed".
     - `engine.rs:1650-1656` already documents that this handler collects without claiming.
  3. `effects/discard.rs:204` `park_discard_batch` via the new `defer_observer_triggers_from`: **route**.
     - Its `paused_events` are `events[events_before_self..].to_vec()` (`discard.rs:467, 526, 637, 677`). That is a copy of the action-buffer suffix, not out-of-buffer events.
     - So the split's premise, and `defer_observer_triggers_from`'s doc comment (`engine_resolution_choices.rs:833-835`: "events that are not part of the current action buffer"), are false.
- Evidence of the duplicate:
  - Child `scry_one().sub_ability(lose(1)).sub_ability(scry_one())` plus Witness.
  - Answering the first Scry: the rider's `LifeChanged` is parked by `park_observer_triggers_if_paused` without a claim. Then `end_action` (non-Priority) stages it from 0 into the carrier. `filter_already_collected_trigger_events_from` (`triggers.rs:11925-11958`) dedupes only queued `ZoneChanged` events.
  - After the second answer, the stack is [B, W, W].
- Why it matters: this is the F1 duplicate (CR 603.2c, CR 603.3) on the routes the fix claims to cover.
- Suggested fix:
  - Call `take_inline_trigger_span` inside both `triggers.rs` helpers. `park_observer_triggers_if_paused` would then take `&mut Vec<GameEvent>`.
  - Pass `events` and `events_before_self` into `park_discard_batch`, and call `defer_observer_triggers_for_paused_choice` there.
  - Fold `defer_observer_triggers_from` back into the routed function, or correct its doc comment.
  - Add a double-Scry row (stack == [B, W], exactly one W). Revert check: unroute `triggers.rs:8400`, and the row goes red.

**[MED] [behavior] F2. A rider-free interactive mana child strands the counter-cost root under Priority. The activation is not placed in the answering action, and SBAs run mid-activation.**
- Evidence:
  - `effects/mana.rs:432`;
  - `effects/mod.rs:1260-1410` (the only paused-dispatch retire, at `:1404`, is inside the continuation block);
  - `engine.rs:1746` (`is_due` precondition);
  - `effects/mod.rs:1806-1815` (the D5 loop would retire it);
  - executor deviation 4.
- Why it matters: this is the maintainer's F1 class (CR 602.2b + 601.2h, CR 117.3c, CR 704.3). It is not residual 2 or 3, because the subtree is advanceable.
- Suggested fix: let the authority run the D5 loop for any parked counter-cost root under a Priority result (§C). Add row T12f.

**[MED] [behavior] F3. T10f: an outer `player_scope` leg runs inside the Cove's activation, and P1's unless choice is lost. The cause is the draw-pair promote swapping the outer continuation above the child's paused PR, which defeats the depth boundary and gate (a).**
- Evidence:
  - `draw.rs:858-867`;
  - `game_state.rs:27691-27733`;
  - `resolution.rs:3295-3316`;
  - `game_state.rs:6092-6097`;
  - `effects/mod.rs:1213`, `:1404-1410`;
  - §B trace.
- Why it matters: it violates CR 101.4 / 608.2e / 118.12a inside Phase D's claimed configuration, and it leaves gate (a) without a discriminating row.
- Suggested fix:
  - Guard the promote against a continuation below the parked root's boundary. Keep T10f (Draw rider) as its row.
  - Add T10f′ (GainLife rider) as gate (a)'s row.
  - Fallback: name it residual 9, but T10f′ is still required.

**[LOW] [behavior] F4. T10f's reach and final "P1's leg is still parked" assertions cannot tell P1's leg from any other continuation.**
- Evidence: `parked_child_rider` is true for any `AbilityContinuation` (`cost_removed_counter_amount.rs:1800-1805`). It is used at `:3611-3614` and `:3629-3632`, and through `assert_mana_root_waits_with_child_rider` (`:2477-2487`).
- Why it matters: a stranded rider, or a spliced leg, satisfies it. This is the vacuous-negative class.
- Suggested fix:
  - Assert exactly one `AbilityContinuation` frame whose chain is the `LoseLife` bound to P1.
  - Or, after `PayUnlessCost{pay:false}`, assert `UnlessPayment{player: P1}`.
  - Apply the same to T10f′.

**[LOW] [behavior] F5. Of the routed sites, only the mana arm has a discriminating row. T12c reaches no routed collector.**
- Evidence:
  - T12c is green at base, head and every revert. Answering the Scry runs `finish_with_continuation`, then the D5 loop, then `push`, and `park_observer_triggers_if_paused` is a no-op at Priority.
  - So C4's "revert-check: drop C1's routing at the exercised site → red" is unmet.
  - No row reaches `batch_or_drain_observer_triggers`, `collect_search_observer_triggers`, the discard pair loop, the B2 parks, the change-zone iteration or `copy_spell`.
- Why it matters: the generalized custody (C1) ships mostly untested. This is a fixture path-divergence.
- Suggested fix:
  - Replace T12c with F1's double-Scry row.
  - Add one batch/sacrifice row: a child "sacrifice a creature" (EffectZoneChoice), then `batch_or_drain_observer_triggers` B1, with a dies-Witness. Assert [B, W] once; the row goes red with that site unrouted.

**[LOW] [behavior] F6. `announced = owned.end != events.len()` also holds when the announcement owns only a suffix. The B1 returns then give up the inline-scan suppression for the prefix the caller just collected without claiming.**
- Evidence: the B1 return `WaitingFor` arms at:
  - `engine_resolution_choices.rs:770` and `:799-803` (`batch_or_drain_observer_triggers`);
  - `:871` and `:887-889` (`collect_search_observer_triggers`);
  - `:6060` and `:6081-6083` (the discard pair loop).

  Compare the mana arm's correct `scanned_inline = !owned.is_empty()` (`engine.rs:13265-13270`).
- Why it matters: the pipeline would re-scan the unowned prefix (CR 603.2c). It is latent: partial custody under a Priority result is unreachable today, because a Priority-live announcement is a parked root whose scope is `From{0}`. It is still wrong for the case the code handles.
- Suggested fix: return `WaitingFor` only when `owned.is_empty() && owned.end < events.len()`, matching the mana arm.

## Pre-existing

- The draw-pair promote (`resolution.rs:3295-3316`, from `draw.rs:858`) is positional. With no parked root, it lifts whatever continuation lies below a paused PR. It leaves that PR, now empty, below the lifted frame, and `remove_empty_active_post_replacement_frame` (top-only) cannot remove it. The ordering is benign there, because the lifted frame is the next written instruction. Upstream 39fb1eb has the same code. F3 covers the Phase D consequence.
