# Phase D plan review, round 2

Plan: `pr-phase-D-plan.md` (r2, PHASE_BASE 4483f4b). Prior round: `pr-phase-D-plan-review-r1.md`.
Code read at 4483f4b, the working tree. The review was read-only: no cargo, no probes.

## Verdict: no behavior gap. Close the round by applying one blocking text fix (X3)

r1's B1, B2, X1, X2 and T1–T6 are resolved in the plan text and check out against 4483f4b. The details are below.

The new material carries one blocking defect: the D5 loop's termination measure is not well-founded. The fix is fully specified, so it is tagged `text` and closes on application.

The rest are non-blocking tightenings:
- the `Less → true` arm of `is_retired`;
- the safety of the new debug assertion;
- that U2 is the only nested-frame row and is a helper-level test;
- that the M-F1 route still makes a single pass;
- one coordinate sentence.

---

## r1 findings: resolution check

| r1 | Status | Evidence |
|---|---|---|
| B1 | **Resolved** | See "B1 check" below. |
| B2 | **Resolved for the reported configuration.** See T-A for a residual inexactness in the `Less` arm. | `ReplacementSubtreeBoundary::at` reads `active_post_replacement_or_paired_parent()` (resolution.rs 2809, the same slot `install_post_replacement_drain` at game_state.rs 27734 installs into), so a nested child raises exactly the counted frame's `drains.len()`. `install(Replace)` (24988) evicts only `Ready`. `finish_dispatch` (25067) and `finish_paused_dispatch` (25082) remove the entry, so the count returns. D5(a) and D5(b) are dropped, and `resume_pending_continuation_if_priority`'s body is unchanged. |
| X1 | **Resolved** | engine.rs anchors +109: drain at 8732, PriorityBoundary arm at 8783, `resume_pending_continuation_if_priority` at 8874, `resume_interrupted_cost_payment` dispatch at 8808. game_state.rs: 6043 / 7895 / 8110 / 8292 / 8358 / 24988 / 27734 confirmed. The header sentence on game_state.rs is inexact (T-E). |
| X2 | **Resolved** | lobby-broker `protocol.rs:63/1147` (151), server-core `:3476` (v151), `ws-adapter.ts:218` (151), `protocol.ts:109` (133). |
| T1 | **Resolved** | Residual 2 is now stated as a remaining defect, citing `withholds_priority` (game_state.rs 8365; CR 601.2h/602.2b). |
| T2 | **Resolved** | Residual 4 names the `Err` sources, the `.expect` panic, its parity with `ManaAbilityPayment`, and the follow-up. |
| T3 | **Resolved** | D2 cites 601.2h + 616.1. The extracted tail's 614.12a and 118.12 are replaced. Step 0 lists 601.2h and 602.2b. |
| T4 | **Resolved** | The field is required, with no serde default. The construction and consumption sites are complete: `costs.rs` 809 is the only constructor; the destructures are `casting_costs.rs` 5020 and `mana_abilities.rs` 4138; the test helper at :1233 only borrows. |
| T5 | **Partially resolved.** See T-D. | The D5 helper covers D4(b), D4(c) and the Execute epilogue. The M-F1 route (resolution-choice handlers → `finish_with_continuation`) still makes a single pass. |
| T6 | **Resolved** | `parked_child_rider` iterates frames through `ResolutionStack::iter` (resolution.rs 3316), and `ResolutionFrame` is public through `types::resolution`. |

### B1 check

The only callers of `resume_interrupted_cost_payment` are `engine.rs:8808`, the cost-move dispatcher for `Cast`/`SacrificeForCost` roots, and `engine_replacement.rs:1380`. D6 covers both. Its two halves:

**D6(a)'s premise holds.** Between 1345 and the insertion point, only the 1360 drain runs, and it can only move `waiting_for` off Priority. So a root that is `Retired` while `waiting_for` is Priority at D6(a) was `Unfinished` at 1345. A root that resumed at 1345 and re-parked leaves a `ReplacementChoice`.

**D6(b) is enough.** Once D6(a) has run, `waiting_for` is Priority at 1371 only if the root is `Absent` or `Unfinished`.

**T9 discriminates.**
- At the base, entry 1's Prevented arm falls to the tail and resumes the mana root. The queue is left stale and entry 2 never prompts.
- With D1–D5 only, at entry 2's Execute arm:
  1. 1345 reads `Unfinished`.
  2. 1360 drains the rider and retires the paused dispatch (`effects/mod.rs` ~1389).
  3. 1371 sees `pending_cast` and finishes Divination before the Cove's mana arrives.
- With D6, `ManaAdded{source_id: cove}` (events.rs:1052) precedes any spell progress, and `waiting_for` returns to ManaPayment.

---

## Blocking

### X3 (text). The D5 loop's termination measure is not well-founded

**Old text at D5:**
```rust
        if !matches!(state.waiting_for, WaitingFor::Priority { .. })
            || state.counter_cost_root_subtree() != CounterCostRootSubtree::Unfinished
            || ReplacementSubtreeBoundary::at(&state.resolution_stack) == before
        {
            return Ok(());
        }
```

> Termination: every repeat requires the position `(len, enclosing drains)` to have changed and the subtree to be unfinished. Each pass that changes the position retires frames or drains the subtree pushed. A pass that pushes new subtree work raises a prompt, which ends the loop on `waiting_for`. **(unprobed; rows T3 and T9 reach it)**

**Why it is wrong:**
- "Changed" is not progress. A pass that grows the stack without prompting (a top frame whose drain parks deeper children, e.g. a `RepeatFor`/`RepeatUntil` re-park, `effects/mod.rs` 2716–2734) followed by a pass that shrinks it would alternate between positions forever.
- The claim "a pass that pushes new subtree work raises a prompt" is unprobed, and `resume_resolution_frames`' per-frame arms do not guarantee it.
- An unbounded loop in a reducer hangs the session.

**Replacement:**
```rust
        let after = ReplacementSubtreeBoundary::at(&state.resolution_stack);
        if !matches!(state.waiting_for, WaitingFor::Priority { .. })
            || state.counter_cost_root_subtree() != CounterCostRootSubtree::Unfinished
            || !after.is_strictly_inside(&before)
        {
            return Ok(());
        }
```
Add to `impl ReplacementSubtreeBoundary`:
```rust
    /// Strictly less of the subtree remains: the stack is lower, or as high
    /// with fewer drains in the enclosing frame (lexicographic on ℕ×ℕ).
    fn is_strictly_inside(&self, before: &Self) -> bool {
        (self.resolution_depth, self.enclosing_post_replacement_drains)
            < (before.resolution_depth, before.enclosing_post_replacement_drains)
    }
```
Replace the termination paragraph with:

> Termination: a repeat requires `(resolution_depth, enclosing drains)` to strictly decrease lexicographically. ℕ×ℕ under that order is well-founded, so the loop ends. A pass that does not strictly decrease ends it, and any subtree left is residual 3.

---

## Tightening (non-blocking)

**T-A. `is_retired`'s `Less → true` arm can misread a nested child.** When the enclosing frame is the paired parent at top-1 (`[.., P(paused), C]`, with C the outer chain's parked rest, resolution.rs 2854–2859), the child nests into P below the recorded top.
- If C is consumed while the child is still `Paused` in P, `len` drops to `N-1` and the arm returns `Retired` with P's count still `k+1`. Draining C early is a pre-existing ungated `drain_pending_continuation` (r1 B2's dropped D5(a) territory).
- Exactness would key the enclosing frame by identity, not position: record `Option<(PostReplacementFrameId, usize)>`. The frame id is stamped once at dispatch (game_state.rs 24912 / 25133), and a paused enclosing frame has one.
- Alternatively, state this as a residual next to the pre-existing early-C drain.

**T-B. The D1 `debug_assert!` (no `Ready` enclosing resident) is asserted safe without evidence.** That is the same objection the plan raises against option (b).
- It sits in `pay_counter_cost_removals`, which also serves resolution-scope PayCost (`costs.rs` 1918–1923) and dry runs. Neither creates a root.
- `KeepResident` stacks a `Ready` drain above a `Dispatching` one (game_state.rs ~24988; the CR 616.1g re-entrant stash).
- Either scope the assertion to the activation-scope park sites (`costs.rs` 1906 and `casting_costs.rs` 4910/5033), or replace it with a named residual. In release builds the eviction leaves the count unchanged and reads `Retired` early, so a debug-only check does not protect the classification.

**T-C. The nested-frame exactness (P1′) has only the helper-level row U2.**
- The plan rejects option (b) because the nested configuration is "reachable on ordinary resolution paths" but names no flow.
- Name one, or add a `GameAction` row for it: a mana ability paying a prompt raised inside a paused post-effect, with the Scry child.
- Otherwise record U2 as the accepted evidence level, with that reason, per check 9's helper-only rule.

**T-D. The M-F1 route still makes one pass.**
- The resolution-choice handlers (`engine_resolution_choices.rs` 9403–9411 `finish_with_continuation`, plus 13 sibling `.expect` sites) call the plain `resume_pending_continuation_if_priority`, not the D5 helper.
- A subtree that needs more than one `resume_resolution_frames` pass after a child choice leaves the root parked at Priority.
- Either move the loop inside `resume_pending_continuation_if_priority`. It is inert (one pass) unless a counter-cost root is `Unfinished`, and it covers every caller without a parallel entry point. Or extend residual 3 to name these callers.

**T-E. Header coordinates.** "`game_state.rs` is −2 below line 7895" is wrong past line 12820: the shift is +31 there and about +67 at 27734. The plan cites the correct absolute lines (24988, 27734), so replace the sentence with "game_state.rs anchors are cited at 4483f4b". Also, `effects/perpetual.rs` changed too, not only the end-phase files.

**T-F. #9728 interaction (residual 3).** When the loop exits with the root still parked at Priority, the action-boundary convergence (`owed_spell_final_part_is_due` → `resolution_instructions_are_done`, engine.rs 9022–9029) checks only top-only accessors and not `pending_cost_move_resume`. In a mid-resolution mana-root case, a resolving instant's final part could run under the parked payment. Add one clause to residual 3. Nothing in the loop itself runs a final part, because #9728 removed `deliver_deferred_spell` from `resume_pending_continuation_if_priority`.

---

## Verified (no finding)

**Termination via `waiting_for`:**
- A re-park of a second removal re-records the boundary from the payer at a position that is `Retired`/equal.
- Every resume site exits the loop on the non-Priority `ReplacementChoice`.

**Copies:** Rings and Bracers copy after stack placement and make no cost payment. The copy-token and batch Prevented blocks are residual 2.

**U1:** `at(&empty)` = (0, 0). A pushed `CounterRemovals` frame reads `Unfinished`, and popping it reads `Retired`. U1 discriminates.

**U2:** the APIs exist:
- `PostReplacementDrain::ready` at 24919;
- `begin_post_replacement_dispatch` at 27614;
- `finish_post_replacement_dispatch` at 27646.

  With a `Dispatching` resident, a second `Replace` install nests and leaves `len` unchanged. The nested drain must be begun to obtain its handle before `finish`. The plan's wording implies this; the executor should do it explicitly.

**T9:**
- `ManaAdded { source_id }` exists.
- Its negative ("no `SpellCast`/`StackPushed` for Divination") is paired with the `ManaAdded` `event_index` reach and with `waiting_for == ManaPayment`.

**D4(b) and D4(c) under B1's shape:** neither path reaches `resume_interrupted_cost_payment`.

**CR numbers** in the new text (601.2h, 602.2b, 605.3b, 608.2c, 614.5, 614.6, 616.1, 616.1f, 118.11, 120.3c, 122.1) are present in `docs/MagicCompRules.txt`, with fitting text.

## Pre-existing (untagged)

- An ungated `drain_pending_continuation` at the priority boundary can run an outer chain's parked continuation that sits above a paired PostReplacement parent while a mana root it is paying for is parked. This is the T-A trigger. It reproduces at 4483f4b and is not introduced by this plan.
