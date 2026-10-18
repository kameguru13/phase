# Phase D implementation review r1 (59247b7)

Review Head: 59247b7 (diff `4483f4b..59247b7`, reproduced exactly)
Completion Gate: not assessed. This was a read-only review with no build, and no completion-check evidence was supplied.
Maintainer-Simulation Gate: not assessed (no matrix supplied).

Out of scope: the amendment's items (T4/T10 `#[ignore]`, D7(a)/(b), T4b, U3b).

Verdict: **behavior=2 text=0 machinery=0** (1 MED blocking, 1 LOW riding along)

---

## Findings

**[MED] [behavior] F1. Once the subtree has retired, nothing stops the drain, so outer resolution work runs before a mana root parked mid-resolution resumes.**

Evidence:
- `engine_replacement.rs:1361-1373`: the post-delivery rider drain now runs under an `Unfinished` mana root.
- `engine_replacement.rs:1663-1666` (D4(b) `Unfinished`) and `engine.rs:8890-8903 / 8950-8961`: the D5 pass.
- `effects/mod.rs:1386-1399`: the post-retire cascade inside `drain_pending_continuation`.
- `effects/mod.rs:1782-1792`: the `PostReplacement` arm of `resume_resolution_frames`.

Why it matters:
- At the base, a mana root was resumed **before** any continuation drain. The pre-delivery drain at 1280-1287 skips under `ManaAbilityPayment` ("a mana-source cost pause owns the unpaid cost suffix"), and 1345 resumed the root first.
- 59247b7 defers that root until its subtree retires. It retires the subtree with `drain_pending_continuation` and the full common-boundary pass, and neither stops at `replacement_subtree`.

Traced at 59247b7, for a mana root parked mid-resolution (the class the plan claims: "a mana ability paying a prompt inside a paused post-effect"):

1. **Outer tail `[P, C]`: Execute/DECLINE route, D6(a).**
   - Setup: the outer post-effect is `X.unless_pay(..).sub_ability(Y)`, so `Y` is stashed as C above P while `UnlessPayment` is up. The Cove is activated there, and the child nests into P.
   - At the 1360 drain, R runs. `finish_active_paused_post_replacement_dispatch` pops the child (P is the paired parent). Then `active_ability_continuation()` is C, so `drain_pending_continuation` runs `Y` (`effects/mod.rs:1397-1399`).
   - The cascade then retires P's outer dispatch as well, all **before** D6(a) resumes the Cove and re-offers `UnlessPayment`.
   - At the base, C ran only after the unless payment was answered.
   - This breaks CR 608.2c order and the CR 605.3b "mana ability completes first" contract.
2. **Plain T10 shape `[P]`: last child entry prevented instead of declined.**
   - The flow goes D4(b) `Unfinished` → queue drain → `resume_pending_continuation_if_priority` with `waiting_for` forced to `Priority`.
   - In pass 1, `drain_pending_continuation` runs R and retires the child dispatch, so the subtree has now retired.
   - In the same pass, `resume_resolution_frames` sees top = P with a `Paused` resident and calls `finish_active_paused_post_replacement_dispatch`. That retires the **outer** dispatch, whose `UnlessPayment` is still unanswered (`effects/mod.rs:1782-1792`).
   - Only then does the PriorityBoundary cost drain (`engine.rs:9021`) resume the Cove.
   - The same applies to a non-nested outer continuation below the boundary, for example a resolving spell's stashed tail under an `UnlessPayment` (`waits_for_resolution_choice` includes `UnlessPayment`, `effects/mod.rs:4664`). The child's `Pc` pops, then the cascade or the pass drains that tail.
3. **Untested.** T10 has no outer tail and declines its last entry. T9 pays while casting with an empty resolution stack. The stack-route rows activate at depth 0. So no row has frames below the boundary.

D6(a)'s comment ("resumes as soon as the child's rider has retired that subtree") does not describe what the code does.

This is reachable only through a synthetic RemoveCounter replacement, the same as every row. Per the skill's calibration that is still MED: it is a regression against the base for a sub-class the change claims to cover.

Suggested fix:
- Bound the subtree drain at the root's boundary. Once `counter_cost_root_subtree()` reads `Retired`, resume the root (PriorityBoundary or ReplacementDelivered) before any further frame work.
- Concretely:
  - in the D5 loop, check for retirement between `drain_pending_continuation` and `resume_resolution_frames`. The `DeferredLifeCostResume` seam already gates its drains on `len > boundary` (`engine.rs:8950-8961`), so a typed `replacement_subtree` gate there is the house idiom;
  - gate `drain_pending_continuation`'s post-retire cascade (`effects/mod.rs:1386-1399`) on "a parked root that withholds priority has a retired subtree".
- Add two rows:
  - T10 with an outer tail (`LoseLife.unless_pay({2}).sub_ability(GainLife 5)`), asserting that the tail has not run when `UnlessPayment` is re-offered and runs after paying;
  - a T10 variant that prevents the last child entry, asserting that P's outer drain is still resident and `Paused` at the re-offered `UnlessPayment`.
- Coordinate with the amendment's A1 revision, which also edits the `Unfinished` arm.

**[LOW] [behavior] F2. The D4(c) tail guard has no row.**

Evidence: `engine_replacement.rs:1714-1723`.

Why it matters:
- This guard is the only thing that keeps a prevented **non-counter** event inside a live counter-cost subtree from reaching the CR 608.3e teardown. That teardown clears the child's continuation.
- No row reaches it: T1-T9 prevent only RemoveCounter events, which return at 1655. The guard could therefore be reverted with the suite staying green.
- The plan assigned no row either. This does not report wrong behavior.

Suggested fix: add a T3-shaped row whose child post-effect proposes a non-counter event, for example a life gain or a draw, prevented by `stage_prevented_*`. Assert that the rider runs and the activation is placed afterwards, with a revert-check on D4(c).

---

## Checked, no finding

- **Plan fidelity.** D1, D2, D3 (gate, admission and docs), D4(a)/(b)/(c), D5, D6(a)/(b), the protocol comment lines and the `pending_was_counter_removal` comment all match the plan text.
  - The `EnclosingPostReplacementFrame.frame` doc adopts r3's text correction ("never dispatched, journal-replayed, or from a pre-identity payload").
  - No debug assertion was added (r2 T-B).
- **`ReplacementSubtreeBoundary` / `is_retired`.**
  - It is recorded once, before the payer's loop (`costs.rs:793-795`). A re-park from either resume re-enters the payer and re-records it.
  - `pause_mana_ability_cost_payment` pushes no frame, so the root's own parking does not inflate `len`.
  - The identity read goes through `post_replacement_frame` → `FrameVec::by_id`, and the positional fallback is used only for `frame: None` (residuals 6/7).
- **D5 loop.**
  - It terminates: the lexicographic ℕ×ℕ measure must strictly decrease for another pass to run.
  - With no `Unfinished` root it makes exactly one pass. A root parked during a pass leaves a `ReplacementChoice` and exits on the first disjunct.
  - Recursion through a resumed root reads `Absent` after `take()`.
- **D3 admission.** `CounterAdditionUnlessPayment` stays unadmitted, so its `unreachable!` holds. U1 discriminates both the gate and the admission.
- **D4(a).** The extraction is behaviour-identical: `Some(state.waiting_for.clone())` matches the old return. The 614.12a and 118.12 citations were replaced.
- **D4(b), other arms.**
  - The `Absent` arm with no root is inert, so the R2 rows and the damage → resolving-spell row are unchanged.
  - Every root is eligible at ReplacementPrevented, so no rooted `Absent` case falls to a teardown.
  - Copy-token owners park **below** their child (`token_copy.rs:566-583`), so the new early return shadows only the batch block. That is the amendment's D7(a).
- **D6.**
  - (a) fires only for a root that was `Unfinished` at 1345 and has since retired.
  - (b) changes nothing unless such a root is still `Unfinished`.
  - Cast, ActivationMillPayment, ManaAbilityPayment-without-counter-leg and random-discard unless roots never read `Unfinished`.
- **#9728.** The loop runs no final part. The no-progress interaction is residual 3 (T-F).
- **No conflict with the amendment.**
  - D7(a)'s anchor, just after the Priority reset and before the `match` at `engine_replacement.rs:1659-1662`, is intact.
  - D7(b)'s anchor, after the batch block at 1187-1194, is intact.
  - F1's fix touches the same `Unfinished` arm as A1's gating, so sequence them.
- **Serde.**
  - `replacement_subtree` has no default. The inner `Option`s decode a missing key as `None`.
  - `git grep` finds no JSON, snap, ron or TS fixture that carries `PausedCounterCostRemoval`, `counter_removal` or `pending_cost_move_resume`. The only hits are the protocol comments.
  - v151/133 is this PR's unreleased bump.
- **CR numbers.** 601.2h, 602.2b, 605.3b, 608.2c, 614.6, 616.1, 616.1f, 616.1g, 118.11, 118.12, 120.3c and 701.17a were all grep-verified in `docs/MagicCompRules.txt`, and each describes its site.
- **Executor deviations: acceptable.**
  - T5's `DamageAll { SpecificObject(walker) }`: a template binds no targets, and the comment says so.
  - `removal_queue` iterating the whole stack: test-only, documented, and it makes its `is_none()` negatives stronger.
  - T10's `InstructionCount`: moot while the amendment re-flows T10.
- **Tests.**
  - T1, T3, T5 and T6 each pair their negatives with reach guards: prompt, rider, queue `in_flight`, root kind, and an identity-equal child in T6.
  - T3's life +1 discriminates `applied_total` from `total` and from the root's 3.
  - U2 and U3 match the plan. U3's non-discrimination is the amendment's U3b.

---

## Fix-design review

This reviews the "Phase D impl-review r1 fixes" section of `pr-phase-D-plan.md` against the code at 59247b7. It was read-only, with no builds. Only blocking findings are listed.

**Verdict: REVISE.** There are two blocking findings, both `behavior`.

The gates do close the three traced shapes:
- **Trace 1** (`[P, C]`, DECLINE): the cascade's recursive call stops at gate (a), and D6(a) resumes the Cove.
- **Trace 2** (`[P]`, prevented last entry): drain → (c) → `resume_due_counter_cost_root` → `UnlessPayment`. `resume_resolution_frames` is never reached.
- **Resolving-ability stashed tail** (non-nested `[C, Pc, R]`): `Pc` pops at (b), the recursive call into C stops at (a), and D6(a) resumes the Cove.

Each gate reads `Retired` only while a counter-cost root is parked, which happens only during its own payment. So the gates are inert for every flow that has no such root. The #9728 final-part convergence (`resolution_instructions_are_done`, `engine.rs:9087`) and the life-cost seam are no worse than residuals 3/T-F already state.

### FD1. [MED] [behavior] Residual 9 is false: some handlers stop at the new gates and nothing resumes the root

Evidence:
- `engine.rs:12996-13006`: the resolution `PayCost{TapCreatures}` handler runs `drain_pending_continuation` and then returns. It never calls `resume_pending_continuation_if_priority`. The prompt is raised as a resolution-scope PayCost at `costs.rs:1284/1323`.
- `engine.rs:13175` → `effects/mana.rs:431-432`: the ChooseMana `ResolvingEffect` handler does the same.
- `engine_replacement.rs:1640-1648`: the Prevented counter-move block, one of residual 2's blocks, does the same.
- None of these is followed by a cost-move drain. `run_post_action_pipeline` does not call the boundary either (`engine_priority.rs`).

What happens:
- When a child post-effect raises one of these prompts and answering it retires the subtree, gate (b)/(c) stops the drain. The action then returns `Priority` with the root parked and due.
- At 59247b7 this was already a gap in the maintainer's M-F1 finding: the plan's claim that "all ~50 callers" are covered misses handlers that never call the boundary.
- The new gates make it worse. Until something resumes the root, gates (a) and 3 block **every** later `drain_pending_continuation` and `resume_resolution_frames`. That includes the next stack resolution, because `pass_priority_once_with_pipeline` runs `handle_priority_pass` (`engine.rs:9461`) before its boundary (9506).
- The plan's statement that "every site where a subtree can retire is followed by a resume" is contradicted by these sites, and by residual 2's own blocks.

Suggested fix:
- Add one resume authority at the action boundary, beside `owed_spell_final_part_is_due` (`engine.rs:1668`): when the result is `Priority` and `counter_cost_root_is_due()`, run `resume_pending_continuation_if_priority`. Alternatively, route each listed handler through the boundary.
- Add a row: a T3-shaped child whose post-effect raises a resolution `PayCost{TapCreatures}` or a ChooseMana, answered, with the activation placed in that same action.

### FD2. [MED] [behavior] No row discriminates gates (b) and (c), and T10a's revert-checks claim otherwise

Evidence:
- `effects/mod.rs:1390-1400`. With (c) deleted, T10a stays green: the cascade's recursive `drain_pending_continuation` (1397-1399) returns at gate (a), and T10a has no outer vote, repeat or choose-one-of frame for the post-block steps (`effects/mod.rs:~1404-1430`) to run.
- With (b) deleted, T10a is also green. At the post-chain block the child is still nested in P (count 2 > 1), so the root is not yet due and (b) cannot change anything.
- Gate (b) is reachable only through the `Queued` re-drain (`effects/mod.rs:1381-1388`): its `resume_resolution_frames` retires the child's dispatch, and (b) then has to spare P's outer paused dispatch.
- Gate (c) is reachable only when the outer top is a `RepeatFor`, `VoteBallot`, `ChooseOneOf` or `RepeatUntil` frame.
- So T10a discriminates only gate (a), not "(c) → red; (b)+(c) → red".

Suggested fix:
- Correct T10a's revert-checks to gate (a).
- Add a row whose outer work below the boundary is a `RepeatFor`, for example Taxer's unless instruction iterated by a "for each". Assert that the next iteration has not run when `UnlessPayment` is re-offered; this discriminates (c).
- Add a row whose child rider returns `Queued`, to discriminate (b). If no reachable fixture exists, delete (b) as unexercised.

Not blocking: T10b's paired revert-check (drop gate 3 **and** `resume_due_counter_cost_root`) and T11's D4(c) revert-check both hold as traced.
