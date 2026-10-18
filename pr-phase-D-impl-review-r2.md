# Phase D implementation review r2 (618fb57)

Review Head: 618fb57 (diff `4483f4b..618fb57`; fix round `fad0bf7..618fb57`, reproduced exactly)
Completion Gate: not assessed. The review was read-only with no builds. The runner reports fmt, the CR grep and the targeted rows only (deferred-verification policy). T12b is `#[ignore]`d.
Maintainer-Simulation Gate: not assessed (no matrix supplied). Plan row T12b is blocked.

Verdict: **behavior=3 text=1 machinery=0**. F1 (MED) and F2 (MED) block; F3 (LOW) and T1 (LOW, text) ride along.

---

## 1. T12b root-cause trace (mandatory)

### Flow at 618fb57

The row: Bottle activation → ACCEPT → `ChooseManaColor{ResolvingEffect}` → `ChooseManaColor(Green)`. At the prompt, the activation is a live announcement:
- `pending_cost_move_resume = CounterCostRemovalPayment{pending: Some(..)}` is a `PendingCastOwner::CostMove` (`types/game_state.rs:9081-9086`).
- So `announcement_triggers::begin_action` sets `announcement_event_scope = From{start: 0}` for this action (`announcement_triggers.rs:225-233`).
- The previous action's `end_action` already staged its events, which installed the carrier on that `PendingCast` (`announcement_triggers.rs:186-196`).

Every pass that sees the child's single `LifeChanged(-1)`:

1. **Copy #10, below the activation, with no `StackPushed`.** This is the ChooseManaColor `ResolvingEffect` inline scan.
   - `engine.rs:13187` records `events_before`.
   - `effects/mana.rs:432` (`handle_choose_mana_effect`) calls `drain_pending_continuation`, which runs the child's `LoseLife` rider. So `events[events_before..]` holds the mana events **and** the rider's life loss.
   - `engine.rs:13259-13262` then calls `process_triggers(state, &mana_events)` over that whole span.
   - `process_triggers` (`triggers.rs:7022-7034`) collects and **dispatches straight to the stack** into a local `events_out` that it then discards. That is why no `StackPushed` for #10 reaches the action's events.
   - It honours neither of the two existing authorities:
     - the announcement's custody (`announcement_event_scope`, the consumed ledger);
     - the release barrier `resolution_completion_can_settle`, which refuses while `pending_cost_move_resume.is_some()` (`triggers.rs:10573`). `process_triggers` checks only `resolving_spell_owes_final_part`.
   - It also claims nothing, so the carrier later collects the same occurrence again.
   - It then sets `triggers_processed_inline = true` (`engine.rs:13285-13287`).
2. **Copy #12, above the activation. This is the correct copy, and it is the one site 1 produces.**
   - Site 1 (`engine.rs:16388`) → `resume_pending_continuation_if_priority` → `resume_due_counter_cost_root` → `resume_counter_cost_removal_payment` → `finish_counter_cost_payment` → `finish_pending_cost_or_cast` (`casting_costs.rs:1630`) → `push_ability_entry`.
   - `push_ability_entry` runs `collection.collect(state, events)` over the **full** action buffer (`casting_costs.rs:7323-7345`) and commits into `deferred_triggers`.
   - `drain_deferred_triggers_after_stack_object_announcement` (`casting_costs.rs:1631-1634`) then puts it above B#11.
   - The shared pipeline that follows skips its scan (`triggers_processed_inline`), and the announcement has ended (`end_activation_announcement`), so it adds nothing.
3. **Copy #13, only when site 1 is removed.**
   - The shared pipeline's `stage_announcement_trigger_events` (`engine_priority.rs:156`) runs regardless of `skip_trigger_scan`. With the root still parked and the scope `From{0}`, it stages the unclaimed `LifeChanged` into the carrier and claims it (`OrdinaryCollectorsOnly`).
   - Site 2 (`engine.rs:1659-1676`) then resumes the root **after** that pipeline. `push_ability_entry`'s full-buffer `collection.collect(state, events)` (`casting_costs.rs:7324`) does not filter claimed occurrences, so the carrier holds the event twice.
   - Result: [W#10, B#11, W#12, W#13]. This matches the runner's observation exactly.

### Root cause

- **Primary.** `engine.rs:13259-13262`: the `ResolvingEffect` inline scan puts the child's ordinary triggers on the stack while the activation is still being announced. It bypasses the announcement carrier, which `announcement_triggers.rs:12-15` declares the single authority for those events.
- **Secondary.** `casting_costs.rs:7323-7324`: the activation's commit collects the whole buffer and ignores earlier same-buffer claims. Its spell counterpart `commit_spell_announcement` (`announcement_triggers.rs:301-352`) does filter: it reads `unclaimed_events` from the scope start, minus `retained_by_cost_owners`.
  - Before 618fb57, nothing staged into the carrier earlier in the **same** buffer as the push.
  - Site 2 (resume after the shared pipeline) is the first route where a stage precedes the push in one buffer.

### What the rules require (grep-verified)

- CR 603.3: a triggered ability is put on the stack "the next time a player would receive priority".
- CR 602.2b + CR 601.2h/601.2i: activating an ability is one process. The activation is not complete until its cost is paid, and nobody receives priority mid-activation. CR 117.3c: priority comes after the activation.
- CR 117.5 + CR 704.3: SBAs, and then waiting triggers, run each time a player would get priority.
- CR 605.4a: a *triggered mana ability* resolves immediately, without waiting for priority. That is the legitimate role of the inline scan, and it does not extend to ordinary triggers.

Required outcome: exactly one Witness trigger, on top of the Bottle activation. Stack = [B, W].

### Is it introduced by 618fb57?

- **Copies #10 + carrier: no.**
  - The pair arises whenever a replacement child's `ResolvingEffect` mana choice is answered while a counter-cost root (an announcement) is parked.
  - At fad0bf7 the T12b flow already reaches it: the inline scan pushes W, the shared pipeline stages W into the carrier, and the root stays parked. T12 is red at fad0bf7, so the activation is placed later with the second W above it.
  - The same two collectors would also fire at 4483f4b, where the root is stranded (maintainer F1).
  - Phase D makes the shape reachable as a claimed, tested flow. 618fb57's sites change only where the second copy lands.
- **Copy #13: yes, through site 2's ordering** (stage, then push in one buffer). With site 1 present, site 2 resumes a root only for direct-return handlers and the PassPriority route. There it is latent unless such a handler ran a pipeline first.
- **T12:** no observer, so nothing is visible.
- **ManaAbilityPayment root (unproven, needs a probe):** a Mana-choice child under a mana root that pays a cast's `ManaPayment` has a live outer announcement (`External`). The inline scan pushes W mid-cast, and `end_action` (non-Priority result) stages the same occurrence into the outer carrier. That gives a duplicate, plus a trigger placed before CR 601.2i.
- **Sibling inline collectors (unproven, needs a probe):** the `WaitingForWithInlineTriggers` producers in `engine_resolution_choices.rs` (`:775-795`, `:826-842`, `:6036-6062`) call `collect_triggers_into_deferred`. Their drain is barrier-gated, so nothing lands *below* the activation, but they do not claim.
  - `push_ability_entry`'s unfiltered collect then re-collects the same events: every event kind, because the push does not filter at all. The carrier's stage re-collects non-zone-change events, because `filter_already_collected_trigger_events_from` dedupes only zone changes through the queued witness.
  - This is the maintainer's own F1 example class (an interactive Scry/Search/Dig child).
- **Pre-Phase-D ReplacementDelivered resume (`engine_replacement.rs`, Execute arm):** the push happens inside the replacement-choice handler before any same-buffer stage. No duplicate was traced there, but none is proven absent either, because no T1–T9 row has an observer.

### Fix (minimal; one authority, the announcement carrier)

- **(i) `engine.rs` ChooseManaColor `ResolvingEffect` arm (13256-13287).**
  - Before the inline scan, call `announcement_triggers::stage_announcement_trigger_events(state, &events, events_before)`. It is inert unless a live announcement owns the buffer, and it claims what it stages.
  - Build `mana_events` from `triggers::filter_already_collected_trigger_events_from(state, &events, events_before, &state.consumed_before_priority_trigger_events)`, so only the unclaimed remainder is inline-scanned.
  - Set `triggers_processed_inline` only when that remainder was non-empty.
  - Residual to name: under a live announcement, a `TapsForMana` triggered mana ability on the child's mana is then carried by the carrier instead of resolving at once (CR 605.4a). This shape is unreachable before this PR. Record it, or split the span so triggered mana abilities still resolve inline.
- **(ii) Move the activation commit into `announcement_triggers`** as `commit_activation_announcement(state, collection, events)`, mirroring `commit_spell_announcement`:
  - collect `unclaimed_events(state, events, scope_start)` minus `retained_by_cost_owners`;
  - collect the delayed scan;
  - claim `AllCollectors`.
  - `push_ability_entry` (`casting_costs.rs:7323-7345`) calls it. This removes copy #13 and every future push-after-stage duplicate.
- **Site 1's rationale changes.** With (i)+(ii), T12b is green with or without site 1, so it no longer discriminates site 1.
  - Site 1 remains CR-motivated: it keeps the shared pipeline's SBA loop and delayed scan (CR 704.3 / CR 117.5) from running mid-activation.
  - Its comment ("so triggers raised while paying are collected after the activation exists") must say that instead.
  - Discriminate site 1 with probe P3 below. If nothing observable discriminates it, delete site 1 and its comment per the plan's own rule, and keep site 2 as the only call site.

### Probes for the kg-dev runner

- **P1: confirm the trace at 618fb57.**
  - Add temporary `eprintln!`s:
    - (a) `engine.rs:13261`: `state.stack.len()` before and after `process_triggers`, `mana_events` discriminants, and `state.pending_cost_move_resume.is_some()`;
    - (b) `casting_costs.rs:7324`: `events.len()`, the count of `LifeChanged{amount<0}` in `events`, and `state.deferred_triggers.len()` before and after `commit_into`;
    - (c) `engine_priority.rs:156`: `state.announcement_event_scope` and `state.pending_trigger_carrier_owner()`.
  - Run `cargo test -p phase-engine --test integration a_trigger_from_the_childs_payment -- --ignored --nocapture`, once as is and once with site 1 (`engine.rs:16384-16389`) commented out.
  - Expected:
    - (a) stack 0→1 with the root parked;
    - with site 1: (b) deferred +1, and (c) never logs a `From` scope with a carrier owner after the push;
    - without site 1: (c) logs `From{start: 0}` with owner `CostMove` before the push, and (b) deferred +2.
- **P2: class probes** (after fix (i)+(ii); assert directly after the answer, with **no `drive`**):
  - T12c: `Effect::Scry{1}.sub_ability(lose(1))` as the RemoveCounter post-effect, plus Witness. Answer the Scry. Expect stack == [B, W] with exactly one Witness entry.
  - T12d: the T12 child under a Cove mana root paying a sorcery's `ManaPayment`, plus Witness. Expect one Witness, collected with the cast batch.
  - If either duplicates, the fix must extend to the `WaitingForWithInlineTriggers` collectors. When `pending_trigger_carrier_owner().is_some()`, they should stage through `stage_announcement_trigger_events` instead of `collect_triggers_into_deferred`.
- **P3: a site-1 discriminator** (after the fix).
  - Setup: T12's board plus a P0 1/1 "When this creature dies, you gain 1 life". The child is `DealDamage{1, SpecificObject(that creature)}.sub_ability(lose(1))`, or a Mana-choice child with that rider.
  - With site 1: the dies trigger is above B.
  - With site 1 removed: check whether the shared pipeline's SBA kills it before B is placed and its trigger lands below B.
  - If it is red without site 1, that is T12b's replacement as site 1's discriminator. If it is green, delete site 1.

---

## 2. Gate (a)

**Reachable where (b) and (c) do not stop the cascade first. Keep it, and add a row.**

- (b)'s block (`effects/mod.rs:1401-1412`) calls `finish_active_paused_post_replacement_dispatch()`. In the non-nested case, that retires the child's own post-replacement frame, which is the subtree's last element, so the root becomes due **inside** the block.
- The block then runs its own recursive `drain_pending_continuation` (1408-1411) whenever `active_ability_continuation().is_some()`.
- Gate (c) (1418) sits after the block, so it cannot stop that call. Only (a) (1213) can.
- The same is true of the authority entry: site 1 or site 2 → the pass's first `drain_pending_continuation` (`engine.rs:9008-9012`) runs with the root already due, and only (a) stops it.
- No current row reaches it because nothing below the boundary is an `AbilityContinuation` frame. Deviation 1 showed that the unless tail rides in `UnlessPayment.pending_effect.sub_ability`.
- A producer that does park a frame below an unless prompt is a `player_scope` clause paused on one seat. `effects/mod.rs:16400-16429` → `append_to_pending_continuation` parks the remaining seats as an `AbilityContinuation`.

**Row T10f** `an_outer_player_scope_seat_waits_for_the_mana_root`:
- Board: T10c's, with Taxer's typed ability `{T}: lose_unless_two(1)` given `player_scope` over all players (each seat loses unless that seat pays).
- Reach at P0's `UnlessPayment`: an `AbilityContinuation` frame (P1's clause) is on the resolution stack, and `cove_removal_enclosing(..).is_none()`.
- Flow: Cove → T3-shaped child → entry 1 DECLINE → entry 2 DECLINE.
- Assert:
  - `waiting_for` is `UnlessPayment` for **P0**;
  - the P1 continuation frame is still parked;
  - `ManaAdded{cove}` is in the action's events.
- Trace with (a) deleted:
  1. The recursive call runs P1's clause, which raises P1's `UnlessPayment`.
  2. D6(a) is skipped (not Priority), so the Cove root is stranded and P0's prompt is lost → red.
- Fallback: if `player_scope` with `unless_pay` does not park the frame (the reach fails), use any other producer that parks a frame below the prompt. Deletion is **not** the fallback: the trace shows (a) guarding a reachable cascade, so deleting it would reintroduce r1-F1's defect for that class.

---

## Findings

**[MED] [behavior] F1. A trigger raised by a counter-cost child's payment is put on the stack below the activation, and is collected twice: three times without site 1.**
- Evidence: `engine.rs:13259-13262` with `effects/mana.rs:432` and `triggers.rs:7022-7034` (copy below B, no `StackPushed`); `casting_costs.rs:7323-7324` (unfiltered full-buffer commit); `engine_priority.rs:156` with site 2 at `engine.rs:1659-1676` (stage, then push, in one buffer).
- Why it matters: it violates CR 603.3, CR 602.2b + 601.2h and CR 117.5. Plan row T12b cannot pass, and site 1's comment (`engine.rs:16380-16383`) states a behavior the code does not have.
- Suggested fix: (i) stage the inline span through `announcement_triggers::stage_announcement_trigger_events` and inline-scan only the unclaimed remainder; (ii) move the activation commit into `announcement_triggers::commit_activation_announcement`, filtering as `commit_spell_announcement` does. Then run probes P2/P3 and re-derive site 1's discriminator, or delete site 1.

**[MED] [behavior] F2. T12b's assertions pass with a duplicate trigger above the activation.**
- Evidence: `cost_removed_counter_amount.rs:3320-3325`. `drive(.., stack.len() == 2, ..)` passes priority (`drive` at 281-289), which resolves the top entry until two remain. So [B, W, W] resolves one W and passes, and [W, B, W] only fails by accident of order.
- Why it matters: after a partial fix (for example fix (i) without (ii)), the row goes green with the duplicate still present.
- Suggested fix: drop the `drive`. Right after `choose_green`, assert `stack.len() == 2`, `stack[0]` is the Bottle activation with count 3, `stack[1]` is Witness's trigger, and the action's events hold exactly one `StackPushed` per entry. Pair this with a count of Witness `TriggeredAbility` entries == 1.

**[LOW] [behavior] F3. Gate (a) is reachable and undiscriminated. T10a/T10c cannot reach it, because the outer tail is never a frame.**
- Evidence: `effects/mod.rs:1213`, `1401-1412`, `1418`; `engine.rs:9008-9012`; runner table (no row red under -(a)); deviation 1.
- Why it matters: the gate↔row map requires a discriminating row, and the plan's (a) evidence (T10a) cannot fail.
- Suggested fix: add T10f (section 2). Do not delete (a).

**[LOW] [text] T1. T10c's doc comment describes a frame that does not exist.**
- Evidence: `cost_removed_counter_amount.rs` (T10c header), "The non-nested case: a resolving ability's later instruction is parked below the root's boundary under its own unless payment."
- Why it matters: deviation 1 shows the tail rides in the prompt, so the row is a control, not evidence for a frame below the boundary.
- Suggested fix: replace it with "The non-nested control: the root parks with no post-replacement frame on top, and the ability's later instruction rides in the unless prompt's pending effect, so it can only run after the prompt is answered."

---

## 3. Rest of the Phase D diff: checked, no further finding

- **CR numbers in the diff:** 118.11, 120.3c, 601.2h, 602.2b, 603.3, 605.3b, 608.2c, 614.5, 614.6, 616.1, 616.1g and 701.17a were all grep-verified, and each describes its site. The exception is site 1's 603.3 rationale, which F1 covers.
- **Termination.**
  - `resume_due_counter_cost_root` takes the root, which then reads `Absent`, or re-parks it with a non-Priority wait. Either way, the D5 loop's exit conditions (`engine.rs:8940-8944`) hold.
  - The authority calls the loop once, and that cannot recurse into the authority.
  - The gates read `Retired` only while a root is parked, so they are inert for every flow without one.
- **Wedge risk.**
  - A due root under a Priority result is resumed by site 2 on every `apply_action_boundary_core` route.
  - A due root under a non-Priority result (for example the inline scan's `OrderTriggers` direct return, `engine.rs:13269-13271`) resumes at the answering action's site 1/2.
  - Error paths restore `boundary_snapshot`.
- **Exhaustive matches:** `paused_counter_removal` and `counter_cost_root_subtree` have no wildcard arms.
- **Building blocks:** the `sync_waiting_for` + `resume_pending_continuation_if_priority` reuse follows the token-realization precedent. Fix (ii) above is the one place where an existing authority (`commit_spell_announcement`'s filter) was not mirrored.
- **Deviations.**
  - **1:** the replaced reach (`unless_prompt_holds_its_tail`) is honest. It makes T10a's "tail has not run" assertion structurally true, so T10a's evidence is now `active_drain_is_paused`, which gate (b) discriminates. The plan's (a) claim moves to T10f (F3).
  - **2:** declining in place of paying is fine for the rows. Whether paying should skip the `sub_ability` tail is listed under Pre-existing.
  - **3:** T10c as a control is fine (T1).
  - **4:** T11's DECLINE answer is fine. The D4(c) revert goes red.
- **T10b, T10d and T10e** each discriminate their gate per the runner table. Their negatives are paired with reaches: the prompt kind, `active_drain_is_paused`, the frame equality, and `ManaAdded{cove}`.
- **Phase-fit entry 89 gaps do not depend on this PR.**
  - (ii) Both slot writes are unconditional on upstream main 39fb1eb: `mana_abilities.rs:2711` (`ManaAbilityPayment`) and `casting_costs.rs:6437` (`ActivationMillPayment`).
  - (i) `ActivationMillPayment` is not `PriorityBoundary`-eligible upstream (`engine.rs:8779-8787`), and Phase D does not change its eligibility. D7(a) and (b) touch only the batch settlement under a counter-cost root.
  - No Phase D row (T4, T4b, A1) puts a mill root and a mana root in the slot together, or puts an unless prompt under a mill root.

## Pre-existing

- **Paying an unless cost skips the effect's `sub_ability` tail** (runner deviation 2). This is the runner's observation and was not verified here. No row now depends on it, because T10a/c decline.
