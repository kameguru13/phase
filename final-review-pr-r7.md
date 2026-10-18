# Final PR review r7: phase-rs/phase#10039 (engine/cost-removed-counters-wip)

Review Head: 375a58f8189652b46da5a503bfd0f1aa755d90f5
Base: upstream/main d3f002d. The merge-base equals d3f002d.
Scope: `git diff d3f002d...375a58f` (42 files, +7415/−443).

This review focuses on the work since r6 (2d20628):
- Phase D (59247b7 → fad0bf7 → 618fb57 → 8f18b5e → 384bb92 → c909249 → b78ff37);
- the merge b133e67 (#9941 / a180ef9 reconciliation);
- the F1 stand-down row fa71624;
- the final-check fix 375a58f.

Mode: read-only. No cargo or pnpm was run. The one script run was `node scripts/check-protocol-version.mjs`, which returned rc=0. The completion evidence is the orchestrator's (phase-fit 117–119).

## Verdict

behavior=0 text=0 machinery=0

No findings. The test-adequacy notes in §6 are observations, not findings.

---

## 1. Maintainer F1 / F2 (CHANGES_REQUESTED at 2d20628)

**F1: resume the counter-cost owner after a replacement's interactive child, such as Scry.**

One typed seam carries the boundary:
- `ReplacementSubtreeBoundary` is recorded on `PausedCounterCostRemoval` (`costs.rs:pay_counter_cost_removals`, the first statement).
- It is read through `PendingCostMoveResume::paused_counter_removal`, which is exhaustive. It covers the stack root's `removal` and the mana cursor's `counter_removal`.
- `GameState::counter_cost_root_subtree` classifies the root as Absent, Unfinished or Retired.

The boundaries that consume it:
- `drain_pending_cost_move_resume` refuses an Unfinished root at every boundary. It admits `CounterCostRemovalPayment` at `PriorityBoundary` (engine.rs ~8819/8860).
- `resume_pending_continuation_if_priority` now loops while the root is Unfinished and the lexicographic `SubtreePosition` strictly decreases, so the loop terminates (ℕ×ℕ).
- Every one of its ~50 callers gets the loop, including `finish_with_continuation` (engine_resolution_choices.rs:9492). That is the Scry route.

Rules check:
- CR 601.2h + 602.2b: the activation is placed only once its cost is paid.
- CR 616.1 / 614.6: the replacement's modified event, which here is the post-effect, is part of that payment.
- CR 605.3b: a mana root resumes immediately.
- So the root resumes only once its replacement subtree retires, and before any outer work (CR 608.2c).

That order is enforced by the due-root stops:
- `drain_pending_continuation` gates (a), (b) and (c);
- the `resume_resolution_frames` gate;
- `resume_due_counter_cost_root`, which is called twice inside the pass.

The action-boundary authority `resume_parked_counter_cost_root_at_action_boundary` closes the handlers that drain directly:
- Site 1 sits before `run_post_action_pipeline`: the activation is on the stack before the SBA check (CR 704.3 / 117.5), and triggers go above it (CR 603.3).
- Site 2 sits in `apply_action_boundary_core`, so `owed_spell_final_part_is_due` is evaluated after the resume.

Rows:

| Route | Row |
|---|---|
| Stack | T1 `a_counter_cost_root_resumes_after_its_replacement_scry_child`: a ScryChoice reach. After the answer, the stack holds one item with count 3, the queue is gone and ownership is clear. |
| Mana | T2 `a_mana_cost_root_resumes_after_its_replacement_scry_child` |
| Other interactive children | mana colour (T12 / T12f), double Scry, discard, sacrifice, unless/outer tails (T10a–f), SBA (T12e) |

**F2: tell a prevented child removal apart from the outer payment's own removal; finish the child's queue and rider first.**

The Prevented arm (engine_replacement.rs, the `pending_was_counter_removal` block) is always classified by the subtree:

| Class | What it does |
|---|---|
| Unfinished | Settles the in-subtree parked batch only above the root's recorded depth, drains the child queue (its finalisation stamps `applied_total`), then takes the common boundary. |
| Retired | The removal is the root's own (CR 118.11): `resume_cost_move_root_after_prevention`. |
| Absent | The rootless/effect owner. The batch settles, then the queue, then the continuation, then any unrelated parked root (the 3b828ac stranding is fixed). |

- The D4(c) tail guard covers any other prevented event under an Unfinished root.
- The Execute arm re-offers the delivery drain after the continuation once the root is Retired (D6(a)).
- It also refuses `resume_interrupted_cost_payment` while the root is Unfinished (D6(b)), so a mana root resumes before the outer cast it pays for.

Rows:

| Case | Row |
|---|---|
| Multi-entry child with a trailing rider; the rider reads its own queue's 1, not the root's 3; order queue < rider < activation | T3 |
| Unrelated outer mill root, prevented and declined | T4 / T4b |
| Damage child (an unrecorded removal) | T5 |
| Same-subject child (identity cannot separate it, the subtree can) | T6 |
| Root's own removal (Retired) | T7 / T8 |
| Unrelated batch under a prevented mana root | `a_mana_root_prevented_above_an_unrelated_parked_batch_leaves_the_batch` |
| Prevented last entry with a rider (mana route) | T10b |
| Non-counter child event (D4(c)) | T11 |

Both asks are met, and each class has a discriminating public-action row.

## 2. Single custody authority after the #9941 merge

`announcement_triggers` is still the only stager and committer of announcement events.

- **Push commit.** `push_ability_entry` → `commit_activation_announcement` → `commit_announcement_span`, the body shared with the spell commit. Upstream's whole-buffer body is superseded, as the merge design resolved. `unclaimed_events` is private again.
- **Upstream's stand-down is kept** (`activation_announcement_is_live` in both park functions). Stood-down spans reach the carrier through `end_action` → `stage_announcement_trigger_events`. On our path that covers the `CompleteSelectedReturnToHand` → `move_cost_objects` park; `pay_counter_cost_removals` calls no park.
- **Upstream's settle carrier branch is kept.** It sends ordinary triggers to the carrier and the delayed group publicly, and claims the current fragment `AllCollectors`.
  - Our commit excludes `retained_by_cost_owners`, and its `AllCollectors` claim hides the settled fragment from both scans.
  - So each fragment has one collector (CR 603.2c), and the delayed group is scanned once (CR 603.7b).
- **Upstream adds no new collector.** Between 872c4e3 and d3f002d the only added collector calls are #9941's own park and settle sites (`git diff 872c4e3 d3f002d | grep` collector calls), so no new site needed routing through `take_inline_trigger_span`.
- **R5-F1 holds at head.** Every `resume_with_error_propagation(state, events)?` that is followed by a collector uses `unclaimed_observer_events` (:8444, :8661, :8762). The one raw slice left (:7593) is `mark_simultaneous_departures` stamping, not a collector.

I traced no double collection and no lost trigger on the merged paths. The known residuals (R2 / F7 / Q3 / Q4) are listed in §7.

## 3. 375a58f (`announcement_live_at_action_entry`)

**The fix is correct.**
- `begin_action` now treats `WaitingFor::AbilityModeChoice { is_activated: true, .. }` as a live announcement. CR 700.2a + 602.2b: modes are chosen as part of activating, and the `PendingCast` is built only afterwards.
- The paused variant (SelectModes → PayCost) now stages its `BecomesTarget` span through `end_action` into the carrier. It agrees with upstream's stand-down predicate, which reads `next_wait.pending_cast_ref()`.
- The completing variant commits with `From{0}`.
- The C3(b) `debug_assert!` stays as the owner-enumeration sentinel, and the full debug integration run is green.
- A triggered mode prompt (`is_activated: false`) stays Unowned. That is correct: CR 603.3c is trigger construction, not an announcement.

**The fix is complete for the class.** I checked every prompt that holds an in-flight activation without a `PendingCast`:

| Prompt | Why it needs nothing |
|---|---|
| `AbilityModeChoice{is_activated:true}` | Fixed. It is the only one whose answer installs a carrier (engine_modes.rs:316) with no owner visible at entry. |
| `EquipTarget`, `StationTarget`, `CrewVehicle`, `SaddleMount` | They go through `push_keyword_action`. There is no carrier and no commit, and they return Priority, so the pipeline collects. |
| `PayCost{CostResume::ManaAbility}`, `TapCreaturesForManaAbility`, ManaAbility `ChooseManaColor` | Mana abilities are not announcements (CR 605.3b) and have no carrier. |
| `PendingCostMoveResume::LoyaltyActivation` | No `PendingCast` and no carrier, so the commit sees `None` and the assert cannot fire. This liveness gap reproduces at d3f002d; see Pre-existing. |
| `CounterCostRemovalPayment{pending: None}` | The paused payment is attached to the root by its caller before the action returns (`unattached_activation_slot_mut`). The next `begin_action` therefore sees a CostMove owner. |

**The row is sound.** `a_target_trigger_from_an_activated_mode_choice_survives_its_cost_prompt`:
- Professor Hojo's Oracle text is verbatim against AtomicCards.json.
- The modal ability is labelled synthetic.
- Reach guards: `AbilityModeChoice{is_activated:true}`, then `PayCost` after SelectModes, with an empty stack.
- Positive: the activation is at the bottom and there is exactly one Hojo entry.
- The runner's red/green record is: no fix RED, assert-only removal RED, fix GREEN.

## 4. Protocol 153 / wire 135

The numbers are consistent across every surface (152/134 reserved for #10004):
- lobby-broker: `PROTOCOL_VERSION = 153`, the test asserts 153/152, and the `/// 153` entry carries the reservation note.
- server-core: the v153 doc entry and `protocol_version_is_153_for_counter_cost_removal_payment`, with the REVERT-PROBE reference renamed.
- ws-adapter: 153.
- protocol.ts: wire 135; protocol.test.ts pins v135.
- p2p gate: refuses 134 and admits 135. The history list gains 132 and 133. The Bestow rows admit 135 and refuse 132.
- openPhaseSocket: rejects v151 and accepts v153, and keeps main's v149 and v150 reject rows.
- check-protocol-version.mjs: +82 / +81 with reservation comments, rc=0.
- `replacement_subtree` is new inside the unreleased v153 record, so it needs no further bump.

## 5. CR annotations

I extracted all 45 distinct `CR NNN(.Na)` numbers added in the PR diff and grep-verified each against `docs/MagicCompRules.txt`. All 45 resolve:

101.4, 106.5, 107.1, 107.1c, 117.3c, 117.5, 118.3, 118.11, 118.12, 118.12a, 120.3c, 122.1, 122.2, 201.5b, 500.7, 601.2f, 601.2g, 601.2h, 601.2i, 602.2a, 602.2b, 603.2, 603.2c, 603.3, 603.3b, 603.7b, 605.1a, 605.3a, 605.3b, 605.4a, 608.2c, 608.2d, 608.2e, 608.2h, 608.3e, 614.5, 614.6, 616.1, 616.1g, 700.2a, 700.7, 701.17a, 702.49a, 704.3, 707.10.

The new annotations each fit their site:
- 700.2a, 602.2a and 602.2b on `announcement_live_at_action_entry`;
- 704.3, 117.5 and 117.3c at site 1;
- 101.4, 608.2e and 118.12a on the promote guard;
- 614.6 and 616.1 on the batch settle;
- 603.7b at the push commit. Merge-review F3's 603.7c was corrected to 603.7b. The only 603.7c left in engine_replacement.rs is upstream text outside this diff.

The compound `+` / `/` forms follow the house convention.

## 6. Test quality

No vacuous negatives found in the Phase D and post-merge rows. Each negative is paired with a positive in the same flow:
- deferred empty → one Witness in the final prompt;
- the batch is not settled mid-payment → `Milled` on the pay;
- no rider frame → the life delta.

Ordering asserts use `event_index`, which panics when the event is absent, on the deciding action's events. Nothing is `#[ignore]`d in `cost_removed_counter_amount.rs` (74/0/0).

**The F1 stand-down row is adequate.** The row is `a_stood_down_cost_park_inside_a_resumed_counter_root_keeps_one_activation_batch`.
- On this shape the stand-down does not change the rules outcome. With the conjunct reverted, the public park claims the span `OrdinaryCollectorsOnly` and the deferred queue drains into the same CR 603.3b batch at the push. Both versions yield one `OrderTriggers` prompt {Witness, Rings} above the activation.
- The merge review's premise (a split batch) does not hold here, so the single-prompt assertion cannot discriminate, and no public-action outcome would.
- What the conjunct does change is custody during the pause. The row pins that with `deferred_triggers.is_empty()` at the `ReplacementChoice` reach, and the runner observed it go RED under revert.
- That assertion is not vacuous. Its paired positive is that the Witness (an earlier action's event) is collected exactly once in the final prompt. If the stand-down had lost it instead, the prompt would hold Rings alone.
- The reach guards confirm the paused `Cast{pending: Some}` and the `CompleteSelectedReturnToHand` completion.
- Magosi and Rings of Brighthearth are verbatim. The Witness is a labelled typed control.
- Suggestion, not a finding: the PR body should say that the stand-down is outcome-neutral on this shape and that the row pins custody.

**Other observations, not findings:**
- The Hojo row tests modal-activation announcement custody, not counter costs, but it lives in `cost_removed_counter_amount.rs`. `announcement_trigger_collection.rs` would be the natural home. This does not affect correctness.
- These sites still have no dedicated discriminating row, as disclosed by the executor and accepted in impl-review r5:
  - F4's partial-progress staging;
  - the three keeper B2 arms;
  - `finish_active_paused_post_replacement_dispatch`'s promote guard. That guard is covered only through the shared helper's T10f revert.

## 7. Residuals the PR body must disclose (each confirmed at 375a58f)

From the Phase D plan:
1. **Non-counter roots carry no subtree boundary** (`Cast`, `ActivationMillPayment`, `ManaAbilityPayment` without a counter leg). `paused_counter_removal` returns `None` for them. They resume after the batch, the queue and the continuation, but a paused `PostReplacement` dispatch of their own replacement may retire after the root. The unless payment inside a mill cost root is also outside the A1 row.
2. **No-progress subtree (old residual 3; old residual 2 is folded into it).** If a pass cannot strictly decrease the position, the loop stops with the root parked under `Priority`. Priority is then surfaced mid-activation (CR 601.2h / 602.2b).
   - The Prevented-arm blocks (connive, life, counter add/move, copy token, batch) now resume an advanceable subtree in the same action through the action-boundary authority.
   - `resolution_instructions_are_done` (engine.rs:9168) still does not consult `pending_cost_move_resume`. A spell's final part can therefore run under a root left parked by a no-progress stop. The follow-up is to gate it on `withholds_priority`.
   - No row reaches this residual: it is synthetic only.
3. **Error contract at the new admission.** `resume_counter_cost_removal_payment` can return `Err`. At `PriorityBoundary`, `finish_with_continuation` and its 16 sibling `.expect("… must resume its continuation")` sites (17 in total) would panic. This is the same exposure `ManaAbilityPayment` already has, and it is synthetic only.
4. **Resolution-scope PayCost and negative-loyalty counter removals** keep no root and no count.
5. **A `Ready` enclosing resident.** A `Replace` install evicts an undispatched `KeepResident` stash, so the drain count does not rise and the boundary reads `Retired` early (release too).
6. **An unstamped enclosing frame is read positionally.** The doc at game_state.rs:6069 now names three sources: never dispatched, journal-replayed, and pre-identity payload. Correct the plan's "pre-identity payload only" wording to include journal-replayed frames.
7. **`ManaAdded`-mode triggered mana abilities under a live announcement** wait in the carrier until the drain instead of resolving immediately (CR 605.4a residual; follow-up via `partition_collected_mana_frame_contexts`). This is documented on `take_inline_trigger_span`.

From the merge design review:

8. **R2, watermark skip.** A same-action stager that starts after a stood-down span would advance the watermark past it. No path was found: stand-downs end on a non-Priority wait, and every later-start stager is Priority-gated. The protection is an invariant, not a construction. The follow-up is the `inception` / `staged_through` split.
9. **F7, delayed scan start.** `commit_announcement_span`'s delayed scan starts at the advanced watermark. On the route where an `ActionResult`-direct inline stage is followed by a same-action boundary resume, no delayed scan covers the earlier staged span. No card reproduces it. It folds into the R2 follow-up.
10. **Q1, two predicates.** `activation_announcement_is_live` (owner based) and `announcement_owned_from` (scope based) agree only by the shared `begin_action` / `begin_announcement` invariant. 375a58f extends that invariant to the activated mode prompt.
11. **Q3, spell carrier on cancel.** Resolution-time and mana-ability-child events staged into a spell carrier are dropped if the spell is cancelled (CR 733.1). This matches the base `end_action` model. T12d covers only the non-cancel case.
12. **Q4, unattached-slot window.** A park while a root's `pending` slot is `None` goes public, which can split the batch (CR 603.3b ordering only). Nothing is lost or doubled.
13. **Automatic counter leg not marked committed** (merge F4). A selected counter cost marks the activation committed, but the automatic counter leg (`pending: None`) is never marked on resume, so a later suffix prompt may still accept `CancelCast`. Do not claim the broad "every suffix root refuses CancelCast" sentence.
14. **Protocol reservation.** 152 / wire 134 are reserved for #10004. If #10004 lands at other numbers, or after this PR, renumber all four constants, the script's +82/+81, the server-core test name, the protocol.test.ts title, the p2p gate row and the openPhaseSocket rows. No literal v152 frame row exists.

Also state in the PR body: the parser output differs from d3f002d for exactly one card, Sage of Hours ("for each five counters removed this way" → `DivideRounded`, floor). Every other printed card in scope keeps an unchanged parse; the PR changes runtime reading only.

## Pre-existing (reproduces at d3f002d; untagged; not blocking)

- **`LoyaltyActivation` cost-move root has no `PendingCast` and no carrier.** An action that pauses on a loyalty-cost replacement starts Unowned, and its events are not staged (scope-unowned-diagnosis §"Separate observations"). It is the same liveness class as 375a58f, but it never reaches the commit assert.
- **`resolve_unless_payer` `Controller => active_player`** (r4). T10f was moved to the `ScopedPlayer` payer to avoid it.
- **Residual 8 of the plan.** A mana ability whose payment raises frames, activated at an `UnlessPayment` raised by a draw replacement's post-effect, buries the paired `MultiDraw` child and trips the CR 616.1g pairing validator. T10 was re-flowed onto a life-gain post-effect to avoid it.
- **Raw-slice ChooseManaColor arm** (engine.rs, `events[owned]` → `process_triggers`). It can re-collect a nested residual when an inner stage advanced the watermark past the outer `events_before` (merge review, Pre-existing).

## Evidence owned by the orchestrator

- I could not reach GitHub from this session: GraphQL and REST both return 403, "access not enabled". That leaves four items unchecked:
  - the CI runs bound to 375a58f;
  - the parse-diff sticky comment;
  - any external or maintainer comments after 2d20628;
  - #10004's actual numbers.

  Confront the sticky against the local parser comparison (only Sage of Hours) once CI posts it for this head.
- Completion checks on 375a58f are as reported (phase-fit 117–119): fmt, engine lib/integration, phase-ai, server-core, lobby-broker, clippy, protocol script, client type-check/lint/vitest, the counter-cost suite (74/0/0), parser comparison, and Gate A/G PASS.

Final review-impl PASS head=375a58f8189652b46da5a503bfd0f1aa755d90f5
