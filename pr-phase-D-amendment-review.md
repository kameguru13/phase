# Phase D amendment review (post-implementation, 59247b7)

Scope: the "Phase D amendment (post-implementation)" section of `pr-phase-D-plan.md`, checked against the code at 59247b7. The review was read-only: no builds, no probes. Only blocking findings are listed.

## Verdict: REVISE

D7(b), the CR order, and the T4, T4b and U3b rows are sound. Two findings block:
- **A1:** D7(a)'s premise is false on the mana route.
- **A2:** the T10 re-flow cannot pass as written.

---

### A1 (behavior). D7(a)'s premise "a top `BatchDelivery` here is always this removal's enclosing batch" fails on the mana route

**The plan's argument (amendment §A, D7(a)):** a batch parked on an earlier prompt "would have that prompt as the live one", and the Retired arm "cannot see a batch, because a counter-cost removal is proposed by the payer, not inside a move". A mana ability activated at that earlier prompt (CR 605.3a, while paying) breaks both claims.

**How the batch gets parked:**
- A delivery-tail post-effect parks the batch on **any** non-Priority prompt, not only on a replacement choice (`zone_pipeline.rs` 2155–2176).
- `apply_pending_post_replacement_effect` returns any `wf` that is not Priority, then `replacement_pause_delivery_result`. That produces `NeedsChoice`, and `deliver_batch` then calls `stash_batch_tail(.., NewChild)`, which pushes the batch on top (1494).
- So a Mill replacement whose `execute` carries `unless_pay` (T10's construction, put on T4's Mill replacement) leaves `[.., PostReplacement(paused), BatchDelivery]` at `WaitingFor::UnlessPayment`.

**How the premise breaks:**
1. P0 activates Saprazzan Cove there.
2. The payer records the boundary with the batch on top, and the Cove's own removal raises the RemoveCounter choice. That is now the live prompt, while the batch belongs to the outer, still-unanswered `UnlessPayment`.
3. When that removal is prevented, D7(a) runs `drain_pending_batch_deliveries` (`zone_pipeline.rs` 1573) **before** the subtree match. That:
   - delivers the batch's remaining members (a multi-card mill or a mass move);
   - runs its completion (`MilledDeliveryComplete` → `complete_mill_delivery`, `engine_resolution_choices.rs` 9469);
   - pops the batch.

   All of this happens mid-payment, before the outer post-effect's unless cost is answered. That inverts CR 614.6's modified event, in which the post-effect is part of the mill.
4. If a later member pauses, D7(a) returns that prompt, and the Cove's own prevented removal is never settled in this action.

**At 59247b7 (without D7):** the Retired arm resumes the Cove (`engine_replacement.rs` 1665–1671), which returns `UnlessPayment`, and the batch stays parked. D7(a) therefore regresses a working path.

The same mis-ownership reaches the Unfinished arm whenever the batch sits at or below the root's recorded depth.

**Required revision:**
- Drain the batch only when it is part of this removal's own work:
  - **Absent:** keep D7(a) (T4).
  - **Unfinished:** drain only if the batch is above the root's boundary (`resolution_stack.len() > replacement_subtree.resolution_depth` with the batch on top).
  - **Retired:** never.
- Equivalently, move the batch drain into the Absent arm, and into the Unfinished arm behind that depth check.
- Correct the two premise sentences.
- Add a row: T4's Mill replacement with `execute = LoseLife { Fixed 1 }.unless_pay({2})`, the Cove activated at the `UnlessPayment`, and its own removal prevented. Assert:
  - reach: the top frame is `BatchDelivery` at the Cove's prompt;
  - after the prevention: `waiting_for` is `UnlessPayment`, the batch frame is still parked, and no `Milled` event is in the action's events;
  - after paying: the batch settles.

  This row is red with D7(a) as written and green with the gated form.

### A2 (behavior; the test cannot pass as written). The T10 re-flow's rider re-triggers its own GainLife replacement

The amendment re-flows T10 with `optional_replacement(ReplacementEvent::GainLife, Some(lose-unless))` on Steward. It also keeps `t3_child_execute(battery)` "unchanged", and that execute's rider is `Effect::GainLife { PreviousEffectCount, Controller }` (`cost_removed_counter_amount.rs` 1829–1834).

- The rider's gain is a new GainLife event for P0. The optional GainLife replacement applies to it again (CR 614.5 forbids re-application only to the same event).
- So after entry 2's DECLINE, the rider pauses on a fresh `ReplacementChoice`:
  - the Cove's subtree stays `Unfinished`;
  - no `ManaAdded` follows;
  - `waiting_for` is not `UnlessPayment`.
- The row's assertions then fail ("rider gain < first `ManaAdded`", "`waiting_for` is `UnlessPayment`", the life total).
- The amendment's own fallback would wrongly re-`#[ignore]` the row under residual 8. This failure is neither a validation trip nor a P6 failure.

**Fix (choose one):**
- give T10 a rider that is not a life gain, for example a second `t3`-style execute whose rider is `Effect::Draw` or a `PutCounter`;
- scope the GainLife replacement so the rider cannot match it;
- or have the row answer the rider's prompt explicitly (DECLINE) and move the ordering and `waiting_for` assertions to that action.

State which one, and keep the P6 reaches.

---

## Checked, no finding

**D7(b) is safe for every existing Execute-arm flow.**
- It fires only when a `CounterRemovals` frame is on top immediately after the 1187–1194 batch drain.
- Every frame-based drain between 1196 and 1415 is top-only (`active_copy_token`, `active_ability_continuation`, `active_each_player_copy_chosen`), so it was blocked behind the exposed queue anyway. At 59247b7 the queue drained only at 1415, via `drain_pending_continuation`'s leading `drain_pending_counter_removals`. The only reorders are against the delivery cost drain (1345, the intended T4b fix) and 1371.
- **Unaffected flows:**
  - Saprazzan Cove, Essence Bottle and the R2 rows push no batch.
  - Damage removals push no queue.
  - An effect queue under a resolving spell (Rhys/Tetravus) is drained at 1153 while it is on top.
- **A queue exposed below a batch** is either the batch member's delivery-tail child (T4b) or an outer queue whose own entry raised the batch. Either way it is the next work at this boundary and precedes any parked root, which matches the plan's order.

**The CR order is right.** The batch settles first and exposes the child, the same convention as `resume_resolution_frames`' `BatchDelivery` arm (`effects/mod.rs` 1685–1695). The root resumes after both (CR 614.6 + CR 601.2h/602.2b). Settlement only collects triggers, which are put on the stack at the next priority (CR 603.3).

**T4 and T4b discriminate.**
- **T4:** the child queue sits under the batch, and the Absent arm resumes the mill root over it at 59247b7. The `event_index(EffectResolved{RemoveCounter})` reach panics (red). D7(a) makes it green. The new reach (top frame is `BatchDelivery`) pins the shape.
- **T4b:**
  - at 59247b7, the 1153 drain skips (the batch is on top), 1187 exposes the queue, and 1345 resumes the `ActivationMillPayment` root before the queue drains at 1415, so `resolved < activated` fails (red);
  - D7(b) makes it green;
  - `Milled < activated` holds either way, so it is a reach, not a discriminator.

**U3b discriminates identity from position.**
- With C taken and the unstamped P2 pushed, `len == depth` and the active frame is P2, which holds 1 drain. A positional `Some(id)` read gives 1 ≤ 1 and reports retired (red). The identity read gives P = 2 > 1 (green).
- The paired positive (pop P2, finish the nested drain) reads retired.
