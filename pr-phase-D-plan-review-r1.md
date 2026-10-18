# Phase D plan review, round 1

Plan: `pr-phase-D-plan.md`. Supporting trace: `coderabbit-r6-owner-trace.md`.
Code read at 2d20628 (the plan's PHASE_BASE) and diffed against the new implementation base 4483f4b (the merge of upstream 872c4e3, #9728), as the coordinator asked. The review was read-only: no cargo or pnpm, and no probes. Every finding below comes from static reading, and each one names its evidence.

## Verdict: REVISE

The plan is the right shape. It reuses the `DeferredLifeCostResume::resume_at_resolution_depth` seam. It classifies ownership by depth instead of identity, and T6 shows that identity cannot separate the cases. It extracts the typed tail and stays inside the scope of the three findings. Most file:line claims check out (list at the end).

Two behavior defects block it:
- **B1.** The new gate creates an ordering inversion in the Execute arm on the mana route.
- **B2.** The premise "exactly when `len` is back to depth" is false in one frame configuration. The unrequested D5(a) change then turns that gap into a regression against HEAD.

There are also two text findings (the rebase coordinates and the protocol numbers) and several tightenings.

---

## Blocking

### B1 (behavior). D3's gate defers a mana root in the Execute arm, and `resume_interrupted_cost_payment` then finishes the outer cast before the mana ability that is paying for it

**Evidence (2d20628; engine_replacement.rs is unchanged at 4483f4b):**
- `engine_replacement.rs:1276–1296`: the continuation drain is skipped while a `ManaAbilityPayment` root is parked.
- `:1345` runs the ReplacementDelivered cost drain.
- `:1360–1369` runs an ungated continuation drain.
- `:1371–1386`:
  ```rust
  if matches!(waiting_for, WaitingFor::Priority { .. }) && (state.pending_cast.is_some() || state.pending_discard_for_cost.is_some())
  ```
  This calls `casting_costs::resume_interrupted_cost_payment`. With a `ManaAbilityPayment` root parked, that function falls through to its last branch (`casting_costs.rs:3112–3126`): `state.pending_cast.take()` then `finish_activated_ability_at_payment_boundary` or `finish_pending_cost_or_cast`.
- On the nested mana route, `pending_cast` is `Some`. A storage land is tapped while a spell or activation sits in `WaitingFor::ManaPayment`, which is the shape of the existing `a_mana_ability_activated_while_paying_does_not_take_the_pending_count` / `nested_composite_counter_cost_mana_ability_adds_per_counter_removed`.

**At HEAD:**
- `:1345` resumes the mana root.
- `resume_mana_ability_cost_move` returns the outer `ManaPayment` prompt, so `:1360` and `:1371` are skipped because `waiting_for` is not Priority.
- (HEAD's own defect here is that the child's rider is left behind. That is the CR-owner class.)

**With D3:** take a child under the mana root, either the T3 shape (queue plus rider) or any child whose rider or paused dispatch sits above depth.
- At the child's Execute, `:1150` finishes the queue and `:1296` is skipped (mana root).
- `:1345` reads `Unfinished` and returns `Ok(None)`.
- `:1360` drains the rider and retires the dispatch.
- `:1371` sees `pending_cast.is_some()` and finishes the **outer** cast or activation while the mana ability paying for it is still parked.
- The mana root resumes only afterwards, at `:1415`.

That is the inversion the plan's "one rule" exists to prevent, moved one level out. It is on the "nested mana activation during a cast payment" route that this review was asked to check. No row covers it: T2 and T8 activate the Cove at priority with no `pending_cast`.

**Required revision:**
1. Make the Execute arm re-offer the root before `:1371` once the subtree has retired. For example, after the `:1360` drain, re-run `drain_pending_cost_move_resume(ReplacementDelivered{..})` (now `Retired`), or gate `:1371` on `counter_cost_root_subtree() != Unfinished`.
2. State which option is chosen and why it keeps the Execute order (queue, then subtree, then root, then outer payment).
3. Add a row: Saprazzan Cove's counter leg activated while paying a spell's `ManaPayment`, with the T3-shaped child (an execute `RemoveCounter` with ≥ 2 entries plus a rider; entry 1 prevented, entry 2 declined). Assert:
   - reach: `pending_cast.is_some()` and `parked_mana_cursor(..).counter_removal.is_some()` at the child prompt;
   - after the decline: mana added to the pool before the spell's payment resumes. Use `event_index` ordering: the child's `EffectResolved` < the `ManaAdded`-class event < `SpellCast`/stack placement. Or assert that the spell is still in `ManaPayment` with the blue mana present.
   - It should be red with only D1–D4 applied, and green with the fix.

### B2 (behavior). P1 "the subtree has retired exactly when `resolution_stack.len()` is back to that depth" is false when the root parks under an active PostReplacement frame. D5(a) then regresses that case against HEAD

**Evidence:**
- `GameState::install_post_replacement_drain` (`game_state.rs:27665–27690` at 2d20628; ~27734 at 4483f4b) **installs into the existing frame** whenever `active_post_replacement_drains_mut()` is `Some`. It pushes a new frame only otherwise.
- `active_post_replacement_parent_slot` (`resolution.rs:2845–2860`) returns `Some` when the top frame **or the frame immediately below it** is a `PostReplacement`.
- `PostReplacementDrainStack::install(.., Replace)` (`game_state.rs:24926–24955`) evicts only a `Ready` resident. Under a `Paused`/`Dispatching` resident it pushes a nested drain into the same frame.
- So if the counter-cost payer runs while the stack top is `[.., PostReplacement(paused)]` or `[.., PostReplacement(paused), AbilityContinuation]`, the accepted replacement's post-effect nests inside that frame and `len` does not grow. That configuration is a mana ability activated to pay a prompt raised inside a post-replacement dispatch, the typical shape of a paused outer chain.
- If the child then pauses (the T1 Scry), `counter_cost_root_subtree()` reads `Retired` while the child's dispatch is live.

**Consequence:**
- D3: the root becomes eligible at the replacement boundaries before its child retires. That is HEAD-equivalent, because HEAD has no gate.
- D5(a): this is the regression. It bounds the priority-boundary `drain_pending_continuation`/`resume_resolution_frames` at `len > depth`, so after the child's answer **neither runs**. At HEAD they are ungated, and `resume_resolution_frames`' PostReplacement arm (`effects/mod.rs:1783–1792`) retires the paused nested dispatch before the cost drain. With D5(a), the mana root resumes first and leaves a `Paused` nested drain inside the outer frame. The outer dispatch's later retirement (`finish_active_paused_post_replacement_dispatch`) then meets the wrong resident.

D5(a) is also outside the findings' scope and has no row:
- Its stated purpose, "the paying ability's own rider below it is not run before the mana ability settles" for a mana cursor parked mid-resolution, is a HEAD behavior change that no reviewer raised.
- Every mana row in the plan (T2, T8) activates at priority with depth 0, where D5(a) is a no-op.
- The plan's claim that HEAD runs that rider early is itself unprobed. `drain_pending_continuation`'s continuation step is top-only, and with the PostReplacement frame on top `active_ability_continuation()` is `None`.

**Required revision.** Do both:
1. **Restate P1 with its precondition.** The child begins on a new frame only when no PostReplacement frame is at the top or immediately below it when the payer records the depth. Then choose one:
   - (a) make the boundary exact: record the active PostReplacement frame's drain count with the depth, and treat a larger count at that frame as `Unfinished`; or
   - (b) name the configuration as a residual, with `debug_assert!(state.active_post_replacement_drains().is_none())` at the record site so the gap cannot pass silently. If the assertion is unsafe because the configuration is reachable on a printed path, (a) is required.
2. **Drop D5(a)** (keep HEAD's `deferred_life_boundary`-only bounding), or justify it with a mid-resolution mana row that is red at the base. Without D5(a), the nested case stays HEAD-equivalent at the priority boundary, and B2 reduces to the residual in (1).

D5(b), the post-root re-drain, is inert at depth 0 and has no row. Either drop it or give it a reach.

---

## Text

### X1 (text). The base and its coordinates moved with #9728 (4483f4b)

- `PHASE_BASE = 2d20628…` → `PHASE_BASE = 4483f4b` (merge of upstream 872c4e3). The "Red/green is stated against PHASE_BASE" sentence and Verification step 3 then refer to 4483f4b.
- Delete "Upstream main moved to 872c4e3 … and is merged in this same round." The merge is done.
- engine.rs anchors, +109 at 4483f4b:

  | At 2d20628 | At 4483f4b |
  |---|---|
  | `drain_pending_cost_move_resume` 8623–8686 | 8732–8795 |
  | PriorityBoundary arm 8674–8682 | 8783–8791 |
  | doc 8619–8622 | 8728–8731 |
  | `resume_pending_continuation_if_priority` 8765 | 8874 |
  | `deferred_life_boundary` gates 8778/8783 | 8887/8892 |
  | deferred-life re-drain 8803–8813 | 8912–8922 |
  | PriorityBoundary cost drain 8843–8852 | 8952–8961 |
  | `cost_move_drain_priority_boundary_tests` 26758, doc 26767–26770 | 27038, doc ~27047–27050 |

- game_state.rs anchors:

  | At 2d20628 | At 4483f4b |
  |---|---|
  | `DeferredLifeCostResume` 7897–7960 | 7895–7958 |
  | `ManaAbilityCostCursor.counter_removal` 8112 | 8110 |
  | `CounterCostRemovalPayment` 8294 | 8292 |
  | `PendingCostMoveResume` impl 8360+ | 8358+ |
  | `install_post_replacement_drain` 27667–27690 | ~27734 |

  `PausedCounterCostRemoval` stays at 6043.
- engine_replacement.rs, costs.rs, mana_abilities.rs, counters.rs, effects/mod.rs, engine_resolution_choices.rs and engine_payment_choices.rs are unchanged by the merge. casting_costs.rs changed only in tests (≥17587), so its 5015–5020 anchors hold.

**#9728's semantic delta for this plan:** `resume_pending_continuation_if_priority` lost its `deliver_deferred_spell` tail (2d20628 engine.rs:8864–8871). A resolving instant's or sorcery's final part now runs at the action-boundary convergence (`owed_spell_final_part_is_due` → `run_spell_final_part_if_due`) and in the priority pass. No plan step depends on the removed tail, and D5's insertion point (before the liminal/phase/cost drains) is unaffected. The one interaction is noted under T5 below.

### X2 (text). Protocol numbers in "Serialized surface" are stale at 4483f4b

At 4483f4b this PR's entries read **151 / wire 133**, after #10004's 150/132:
- `crates/lobby-broker/src/protocol.rs:63` `/// 151 —`, `PROTOCOL_VERSION = 151` at :1147;
- `crates/server-core/src/protocol.rs:3476` `/// v151:`;
- `client/src/adapter/ws-adapter.ts:218` `151 —`;
- `client/src/network/protocol.ts:109` `133 —`.

Replacements:
- "both introduced by this PR under its v150 / wire-132 bump" → "both introduced by this PR under its v151 / wire-133 bump".
- "No further bump. Do not renumber anything; main's 149/131 merge happens this round." → "No further bump; the merge to 4483f4b already renumbered this PR's entries to 151/133."
- "the existing v150/132 entries" → "the existing v151/133 entries".
- "(PROTOCOL_VERSION's `/// 150` doc)" → "(PROTOCOL_VERSION's `/// 151` doc, :63)".
- "`client/src/network/protocol.ts` ~111" → "~109–115 (the `133 —` entry)".

---

## Tightening (non-blocking)

**T1.** Residual 2 says the root is "never stranded" because the other Prevented-arm child blocks (life-total, counter additions/moves, copy token, batch) resume it "at the next priority boundary". That is inaccurate.
- Those blocks return Priority with the root still parked.
- The next `resume_pending_continuation_if_priority` on that path is the priority pass's (2d20628 engine.rs:9291), which runs **after** `handle_priority_pass`.
- So priority is given mid-activation, which contradicts `PendingCostMoveResume::withholds_priority` (true for `CounterCostRemovalPayment`/`ManaAbilityPayment`; `game_state.rs:8367`; CR 601.2h/602.2b).

Reword it as a remaining defect, not a resolution: "resumes only after the next priority pass — priority is surfaced mid-activation; follow-up".

**T2.** The admission at `PriorityBoundary` routes `resume_counter_cost_removal_payment`'s `Err`s into callers that `.expect`.
- `finish_with_continuation` (`engine_resolution_choices.rs:9403–9411`, the T1 path) and 13 other `.expect("a settled … must resume its continuation")` sites in that file call `resume_pending_continuation_if_priority`.
- The resume can return `Err` from `require_locked_activation_cost`, a `Failed` residual payment (`casting_costs.rs` `finish_counter_cost_payment`), or the `InvalidAction` arm.
- A child post-effect can change the board before the residual is paid.
- `ManaAbilityPayment` already has this exposure. The plan should still state the error contract for the newly admitted root, either as an argument that a residual cannot fail after the subtree or as a named residual, rather than leave a latent panic.

**T3. CR citations.**
- D2's doc "CR 118.3 + CR 616.1" does not fit: 118.3 is "can't pay without the necessary resources". Use "CR 601.2h + CR 616.1".
- The extracted tail keeps "CR 614.12a" (permanents entering the battlefield) and "CR 118.12" ("[do something]. If you do…" costs). Both are inherited from HEAD's comment, but CLAUDE.md requires checking existing annotations on modified code. Neither is in Step 0's grep-verified list, and neither fits a generic cost-root tail. Drop them or justify them; 601.2h/602.2b/605.3b/616.1 cover it.
- Add 601.2h/602.2b to the Step 0 verified list as cited.

**T4.** `#[serde(default)]` on `resume_at_resolution_depth: usize` defaults to 0.
- For a mana root parked mid-resolution, 0 is not neutral: `len > 0` reads `Unfinished` for as long as the paused outer frames exist, so the root could never resume.
- There is no legacy payload: the record is new in this PR's unreleased 151 bump.
- Make the field required (no default), so a malformed state fails decode rather than strands, or justify the default.

**T5.** The `Unfinished` arms (D4(b) and D4(c)) assume one `resume_pending_continuation_if_priority` call retires the whole subtree, but `resume_resolution_frames` handles one top frame per call.
- The rows' shapes satisfy this: T1 retires PostReplacement via the frames step; T3, T5 and T6 retire it inside `drain_pending_continuation` at `effects/mod.rs:~1389`.
- A deeper subtree returns Priority with the root parked. That is the same mid-activation priority as T1. With #9728 it also lets the action-boundary convergence run a resolving spell's final part, because `resolution_instructions_are_done` checks only the top-only continuation accessor.
- State the single-pass precondition, or make the arm loop while `Unfinished` and making progress.

**T6.** The T3 multi-entry rider asserts "life +1" reading `applied_total` 1 (P4).
- Keep the fallback named in P4.
- Also add the ordering assertion's reach for the rider: `active_ability_continuation().is_some()` beneath the queue at entry 2's prompt (P2). As written, P2's guard is only implied by T5.

---

## Verified (no finding)

**Locations:**
- Prevented-arm guard at `engine_replacement.rs:1624–1635`.
- Copy-token/batch blocks at 1636–1653.
- Tail at 1656–1686.
- CR 608.3e teardown with `clear_active_ability_continuation` at 1688–1727.
- Execute order at 1150/1276–1296/1345/1360/1371/1415.
- `pending_was_counter_removal` at 232–235.

**Code claims:**
- `pay_counter_cost_removals` (`costs.rs:786–822`) is the single constructor of `PausedCounterCostRemoval`. Its callers are costs.rs:1894, casting_costs.rs:4910/5033 and mana_abilities.rs:4146/4628, so recording in the payer covers both routes and every re-park.
- The compiler-forced destructures at `casting_costs.rs:5020` and `mana_abilities.rs:4138` are exhaustive (no `..`).
- The deferred-life precedent (`costs.rs:913/938`; `engine.rs:8774–8813`) matches the plan's description.
- `resume_counter_cost_removal_payment` reads the boundary only as `action_event_start`, and `pending: None` returns Priority (`casting_costs.rs:5015–5019`). U1's paired positive holds.
- D2's 13-arm exhaustive match matches the enum. `withholds_priority` and `unattached_activation_slot_mut` are the precedent for exhaustive classifiers on `PendingCostMoveResume`.

**Reachability of the rows:**
- A child `RemoveCounter` starts with an empty `applied` set (`counters.rs:1476–1500`). No RemoveCounter path reads `ResolvedAbility.replacement_applied`, so the same optional replacement can re-prompt on the child (CR 614.5: a new event). T3, T5 and T6 are reachable.
- `resolve_remove` with `counter_type: None, Fixed(-1)` queues one entry per counter type (`counters.rs:3190–3239`), so T3 has two entries.
- The root's own removal is applied before the post-effect (`engine_replacement.rs:567–579`), so T1's "elixir 0" and T6's measured 3 are correct.
- T1 is red at the base: the PriorityBoundary arm excludes the root.
- T3, T4 and T6 are red at the base: the HEAD guard falls to the tail, places the activation, and leaves the queue `in_flight`. The `event_index` panic is the reach.
- No parser or card-data RemoveCounter replacement exists (`git grep` 0; card-data.json 0).

**CR numbers:** 616.1, 616.1f, 608.2c, 118.11, 601.2h, 602.2b, 605.3b, 614.6, 120.3c, 122.1 and 614.5 are present in `docs/MagicCompRules.txt` with fitting text. The exceptions are in T3.

## Pre-existing (untagged, not blocking)

- `DeferredLifeCostResume::resume_at_resolution_depth` has the same nested-PostReplacement-frame gap as B2: the drain nests into an active frame without growing `len`. It predates this PR.
