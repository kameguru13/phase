# PR sync plan: merge upstream main d3f002d into #10039 (engine/cost-removed-counters-wip)

- Ours: c909249. b78ff37 (the r5 CategoryChoice keeper-arm fix) is not in this checkout.
- Theirs: upstream/main d3f002d. Merge base: 872c4e311. 13 upstream commits.
- Trial merge: `/home/user/wt-merge`, detached at c909249, with `git merge --no-commit upstream/main`. MERGE_HEAD is set. The resolutions are staged. One follow-up edit is unstaged (`announcement_triggers.rs`, see §2). Nothing is committed and nothing is pushed.
- Saved diff: `pr-sync-main-d3f002d.diff`, which is `git -C /home/user/wt-merge diff HEAD`. It shows everything the merge brings in relative to c909249, including the resolutions.
- No cargo or pnpm command was run. `rustfmt --check` passes on the three Rust files edited by hand. `node scripts/check-protocol-version.mjs` exits 0.

## 0. Upstream commits that touch the overlap

| commit | overlap |
|---|---|
| a180ef9 (#9941) | One custodian for cost-span triggers. `activation_announcement_is_live`. Park stand-down in `park_cost_payment_triggers_if_paused` and `park_deferred_cost_triggers_if_paused` (both in casting_costs.rs; **triggers.rs is not touched**). Removes `take_pending_activation_trigger_collection`. Threads `activation_cost_committed` through `push_activated_ability_to_stack`. The `settle_sacrifice_for_cost_events` carrier branch now sends ordinary to the carrier and the delayed group to the public queue. `push_ability_entry` scans `unclaimed_events(0)` plus a delayed full-buffer-claim scan. `unclaimed_events` becomes `pub(crate)`. |
| af4b1d8 (#9886) | casting_costs: `stamp_spell_cast_cost_paid_object` / `settled_public_discard_referent`, spell casts only. triggers.rs: removes the dead `*_refs_cost_paid_object` walkers. No trigger-collection change. |
| bf867d0 (#9477) | Large: engine_resolution_choices, triggers, effects/mod, game_state, casting_costs, protocol v150 / wire 132. Grepped: **adds no new inline trigger collector** (no new `process_triggers`, `collect_triggers_into_deferred`, `park_observer_*`, `defer_observer_*`, `run_post_action_pipeline` or `collect_delayed_*` call), so nothing new has to be routed through `take_inline_trigger_span`. |
| cec1c3c (#9953) | Protocol v151 / wire 133 (Bestow). |
| others | Parser, quantity and docs only. No custody overlap. |

## 1. Reconciliation design: one custody authority

**Verdict.**
- Our announcement-custody design and upstream's #9941 are complementary.
- At exactly one site, `push_ability_entry`'s commit, ours supersedes theirs.
- Everywhere else upstream's mechanisms are kept as they are and coexist with ours. That works because they act at different collector sites and hand off through the same stager (`announcement_triggers::stage_announcement_trigger_events`, reached from `end_action`).
- After the merge, `announcement_triggers` is still the only module that stages, commits and claims announcement events. Two exceptions predate this PR: the carrier-collect in `settle_sacrifice_for_cost_events`, and `claim_ordinary_collected_span` in the public-park path. Both are listed as follow-ups.

### 1.1 Upstream mechanisms, one by one

| # | Upstream mechanism | Relation to ours | Resolution |
|---|---|---|---|
| U1 | `activation_announcement_is_live` plus the park stand-down in the two casting_costs park functions. An activation's cost events are left in the action buffer, and `end_action` stages them into the carrier. | **Coexist.** It covers cost-payment parks, which our design never routed. Ours (`take_inline_trigger_span`) covers resolution-time inline collectors. The two hand off to the same stager. Our counter-cost root is a `PendingCastOwner::CostMove` owner (`pending_cast_at`, game_state.rs ~9076), so the predicate also sees an activation parked in `CounterCostRemovalPayment`. A residual cost step paid after our root resumes (for example Magosi's selected return) therefore stands down too. Its events reach the carrier through `end_action`, which runs **after** our action-boundary resume (`apply_action_boundary_core`: resume at ~1662, convergence, then `end_action` at ~1703). | Keep theirs unchanged (auto-merged). Two predicates remain, answering two questions: "who stages, and when" versus "does the announcement own this span". See Q1 for unifying them. |
| U2 | `take_pending_activation_trigger_collection` removed. | Our branch never added a caller (2 refs at base and at ours, both base). | Auto-merged; 0 refs remain. |
| U3 | `activation_cost_committed` threaded through `push_activated_ability_to_stack` and its 13 call sites. | **Theirs extends ours.** `pay_selected_counter_cost` already calls `mark_activation_cost_committed()` before it parks the root, and on Complete. `resume_counter_cost_removal_payment` → `finish_counter_cost_payment` → `finish_pending_cost_or_cast` → `finish_activated_ability_at_payment_boundary` passes `pending.activation_cost_committed`. On the selected-counter path, the suffix root rebuilt after the counter cost refuses `CancelCast` (CR 601.2h + CR 602.2b). Not on the automatic counter leg (`pending: None`): it is attached unmarked and `resume_counter_cost_removal_payment` never marks it (design review F4); do not claim the broad version in the PR body. | Auto-merged. Every call site has the new arity: counted 14 top-level commas at all 13 calls. |
| U4 | `settle_sacrifice_for_cost_events`: the ordinary group goes to the carrier (or the deferred queue), the complete delayed group (retained plus current) goes through `collect_delayed_trigger_contexts_into_deferred`, and the current fragment is claimed `AllCollectors` (CR 603.2c, 603.7b, 603.10a). | **Coexist.** Our `retained_by_cost_owners` excludes `SacrificeForCost.deferred_cost_events` from `stage_announcement_trigger_events` and from `commit_announcement_span`. The settle is therefore the single collector for retained fragments, and its `AllCollectors` claim on the current fragment hides it from our commit's ordinary and delayed scans. No double collection. | Auto-merged (we did not touch settle). |
| U5 | `push_ability_entry` body: `collection.collect(unclaimed_events(state, events, 0))`, commit, delayed scan via `unclaimed_trigger_events_from(.., Delayed, ..)` + `collect_unclaimed_delayed_triggers_into_deferred` (matcher claims published at full-buffer positions), claim the **whole buffer** `AllCollectors`, then `end_activation_announcement`. | **Ours supersedes.** It is the only textual conflict in engine code. `commit_activation_announcement` → `commit_announcement_span` (shared with the spell commit, approved in impl-review r2 C3) does the same work with three refinements. (a) It starts at the announcement's scope, not 0: events before an announcement that began mid-action stay with the priority pipeline. (b) It excludes events a cost root still retains (CR 603.2c: collected once, by that owner). Upstream would collect a live root's retained events into the carrier and claim them, and the root's later settle reads only the queued-context witness for carried fragments, so it could double-collect. (c) The delayed scan uses the same requester-scoped filter (`filter_consumed_trigger_events_from(.., Delayed, ..)`): ordinary-only claims stay visible to delayed observers, and terminal `AllCollectors` claims (settle's) hide them. The owned span is then claimed `AllCollectors` index by index, which subsumes the matcher's own claims. The "complete delayed activation cost groups" fix survives entirely: its group logic lives in the settle (U4), and its "do not hide later occurrences" property holds because claims stay full-buffer identities of this buffer only. | **Resolved to ours** (`casting_costs.rs` ~7392). `end_activation_announcement` stays deleted. `unclaimed_events` goes back to private (§2). `unclaimed_trigger_events_from` and `collect_unclaimed_delayed_triggers_into_deferred` remain live through `engine_priority.rs` (~628, ~734) and the triggers.rs lib test, so there is no dead code. |
| U6 | Spell scope: upstream deliberately keeps a spell's paid-cost events parked publicly so they survive a cancel. | **Coexist, but see Q3.** Our `take_inline_trigger_span` also stages a spell announcement's inline (resolution-time / mana-ability child) events into the spell carrier, which is dropped on cancel. That matches the base `end_action` staging model for spells; upstream's note is scoped to cost parks. | No code change. |

### 1.2 Where the two designs meet after the auto-merge (re-check each on the build machine)

1. **`casting_costs.rs::push_ability_entry` (~7392).** Resolved to `commit_activation_announcement`. CR 602.2b + 603.3b + 603.7b comment (603.7c corrected to 603.7b per design review F3; upstream made the same correction); verified in MagicCompRules.txt.
2. **`casting_costs.rs::park_cost_payment_triggers_if_paused` (~2638) and `park_deferred_cost_triggers_if_paused` (~3582).** Upstream stand-down. Callers on our paths:
   - `move_cost_objects` (installs the Cast root with `pending` *before* parking, so the stand-down sees an owner);
   - `finish_selected_return_to_hand_after_automatic` (reached from `resume_counter_cost_removal_payment`'s `CompleteSelectedReturnToHand`).
   - Our `pay_counter_cost_removals` path calls no park, so its CounterRemoved events always go to `end_action` staging.
3. **`engine.rs::apply_action_boundary_core` (~1659-1705).** Order: `resume_parked_counter_cost_root_at_action_boundary`, then `run_boundary_convergence_pipeline` (only for a Priority result after progress), then `end_action`.
   - Events from a stood-down park inside the resumed root are staged by `end_action`, on a non-Priority result.
   - On a Priority result the root was pushed and `commit_activation_announcement` collected them.
   - Our F4 partial-progress staging (`stage_announcement_trigger_events(.., events_before)`) only runs under a Priority result, where no stood-down park can be pending.
4. **`engine.rs::apply_non_priority_pass_action` (~16429).** The same resume authority runs before `run_post_action_pipeline`, whose first step stages from its `event_start`; `end_action` is the backstop.
5. **`announcement_triggers::stage_announcement_trigger_events` / `end_action`.** Unchanged by both sides. It is now the custodian for upstream's stood-down spans **and** for our inline collectors (via `take_inline_trigger_span`). The scope watermark (`advanced_to(len)`) is shared, which is the source of R2.
6. **`announcement_triggers::retained_by_cost_owners`.** Our `CounterCostRemovalPayment` arm (retains nothing) sits alongside the base `SacrificeForCost` / `ManaAbilityPayment` arms that U4's settle consumes.
7. **triggers.rs `park_observer_triggers_if_paused` (~8512) / `collect_and_drain_observer_triggers_if_settled` (~8553), effects/discard.rs `park_discard_batch`, engine_resolution_choices.rs inline sites (~792, ~850, ~882, ~6078, ~8439, ~8563, ~8653, ~8754), effects/mod.rs ~2537, copy_spell.rs ~294, engine.rs ChooseManaColor arm ~13288.** All ours, routed through `take_inline_trigger_span`. Upstream changed none of them (only bf867d0 edited these files, with no collector calls). The r5 site (~8439) is still a raw slice in this merge, because b78ff37 is not in c909249. Apply b78ff37 before or after the merge; its target region auto-merged unchanged, so it should apply cleanly.
8. **game_state.rs.** Upstream deleted `take_pending_activation_trigger_collection` and kept `restore_pending_activation_trigger_collection`, which `stage_announcement_trigger_events` uses. Our `PendingCastOwner::CostMove` arm for `CounterCostRemovalPayment` auto-merged in both `pending_cast_at` and `pending_cast_at_mut`.

### 1.3 CR check for the reconciled behaviour (all grep-verified in docs/MagicCompRules.txt)
- **CR 603.2c** (one trigger per occurrence): every span has exactly one collector.
  - Retained sacrifice fragments go to the settle (U4).
  - Stood-down cost spans and inline spans go to the stager, which claims `OrdinaryCollectorsOnly`.
  - The finishing fragment goes to the commit, which claims `AllCollectors`.
  - The ledger filter (`unclaimed_events`, the requester-scoped filter) makes stage-then-commit idempotent.
- **CR 603.3b** (one batch at next priority): ordinary activation-cost triggers ride the carrier into the push batch. Delayed ones go to the public deferred queue at the settle or the commit, which is upstream's documented batched-delayed limit and unchanged.
- **CR 603.7b** (delayed triggers fire once): the delayed group is scanned at the settle (retained plus current) and at the commit (unclaimed-for-Delayed). Both are claimed `AllCollectors` afterwards.
- **CR 601.2h + CR 602.2b**: payment completes before the activation is placed. `activation_cost_committed` now survives every rebuilt suffix root, including after our counter-cost root.

## 2. Conflict resolutions applied in /home/user/wt-merge

There were 7 conflicted files and 13 hunks.

1. `crates/engine/src/game/casting_costs.rs` (1 hunk, after `record_activated_ability_placed`): **ours**, `commit_activation_announcement(state, activation_trigger_collection, events)`.
   - Follow-up edit (unstaged): `announcement_triggers::unclaimed_events` goes back from `pub(crate)` to private. Upstream widened it only for the `push_ability_entry` body this resolution drops, and keeping it private keeps the occurrence filter behind the custody API.
2. `crates/lobby-broker/src/protocol.rs` (1 hunk):
   - our doc entry is renumbered to **153** ("a v152 peer cannot parse…", "wire 135", "(152/134 is reserved for the Zimone PR, #10004.)", which follows the file's existing "(119/101 is reserved for the Legends of Jidoor PR.)" convention);
   - placed above main's untouched 151 (Bestow) and 150 (gains rider) entries;
   - `PROTOCOL_VERSION = 153`, test `assert_eq!(PROTOCOL_VERSION, 153)`, `assert_eq!(MIN_SUPPORTED_PROTOCOL, 152)`.
3. `crates/server-core/src/protocol.rs` (3 hunks):
   - v153 doc entry with the v152 reservation note, then main's v151 line, then v149…;
   - test renamed `protocol_version_is_153_for_counter_cost_removal_payment` (the script requires the current numeral and refuses `_152`), asserting 153;
   - the REVERT-PROBE doc reference is renamed to match.
4. `scripts/check-protocol-version.mjs` (3 hunks):
   - main's v147/v150/v151 comments are kept;
   - added: a v153 header line, "(+81, v152, is reserved for the Zimone PR, #10004.)", and "+82: v153 …" → `EXPECTED_PROTOCOL_VERSION = 71 + 82` (153);
   - for the wire: "(+80, wire 134, is reserved …)" and "+81: wire 135 …" → `EXPECTED_WIRE_PROTOCOL_VERSION = 54 + 81` (135).
5. `client/src/adapter/ws-adapter.ts` (2 hunks): the 153 doc entry is placed above main's 151/150 entries, and `PROTOCOL_VERSION = 153`. The second hunk takes main's extra blank line after the constant, to match main.
6. `client/src/network/protocol.ts` (1 hunk): the wire 135 doc entry is placed above main's 133/132, and `WIRE_PROTOCOL_VERSION = 135`.
7. `client/src/services/__tests__/openPhaseSocket.test.ts` (2 hunks):
   - main's titles are taken for the v149 reject and the v150 reject;
   - main's "accepts the literal Full v151 populated Bestow protocol" becomes "rejects Full v151 before it can read a deferred counter-cost removal";
   - "accepts the literal Full v153 deferred counter-cost removal and populated Bestow protocol" is added.
   - No v152 row: #10004 adds its own accept-152 row, which our rebase then turns into a reject.

Files that auto-merged but still carried 151/133 literals (edited):
- `client/src/network/__tests__/protocol.test.ts`: the title pin is "v135", `toBe(135)`.
- `client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`:
  - the earlier-history list gains 132 and 133;
  - the gate row is "refuses the previous wire protocol (v134) and admits its own (v135)" with `setupFrameAt(134)` / `setupFrameAt(135)` and its revert-probe comment renumbered;
  - in main's two Bestow reconnect/host rows the **admitted** literal goes 133 → 135 and the refused literal stays 132. That 132 is the genuinely pre-Bestow peer those rows are about, and the exact-match gate still refuses it.

Main's own 150/151 entries are untouched everywhere.

## 3. Tests to run first on the build machine

Order: the lib tests, the overlap integration rows, every row in the `cost_removed_counter_amount` file, then the full `test-engine`, clippy and the protocol script (already green here).

**Upstream (a180ef9), integration `announcement_trigger_collection.rs`: all 20 rows**
- n1_targeted_discard_trigger_survives_a_later_cost_prompt
- n1_three_player_opponent_discard_observer_joins_the_batch
- n2_untargeted_discard_trigger_is_in_the_activation_batch
- n2b_discard_triggers_exactly_once
- n1_cancel_after_a_paid_discard_is_refused_and_keeps_its_triggers
- n1_cancel_before_any_payment_is_accepted
- v21_activation_sacrifice_group_yields_one_observer_trigger
- v21_reference_unpaused_group_yields_one_observer_trigger
- v21b_delayed_observer_of_a_retained_sacrifice_fragment_triggers_once
- api_batched_delayed_sacrifice_group_is_one_firing (and `_targeted`)
- api_delayed_sacrifice_batch_controls
- api_one_shot_delayed_filters_keep_retained_current_and_unmatched_authority
- api_delayed_group_keeps_source_and_member_authority
- api_delayed_group_uses_creating_controller_despite_different_source_owner
- api_delayed_sacrifice_group_survives_raw_and_trusted_restore
- api_delayed_group_claims_do_not_hide_activation_or_later_occurrences
- v25_spell_with_a_paid_discard_across_a_manual_payment_pause
- guard_untargeted_exile_cost_trigger_is_in_the_stack_push_batch
- guard_mana_ability_sacrifice_is_not_an_announcement

The api_* and v21b rows are the ones that discriminate the U5 resolution (our commit in place of upstream's push body).

**Upstream lib tests**
- casting_costs.rs `mod tests`:
  - discard_for_cost_parks_triggers_when_later_discard_pauses_on_replacement
  - activation_discard_cost_events_are_left_to_the_action_boundary
  - discard_for_cost_resume_parks_triggers_when_next_discard_pauses_on_replacement
  - discard_for_cost_resume_can_pause_on_each_remaining_discard
  - activation_discard_for_cost_leaves_graveyard_triggers_to_the_boundary_when_mana_payment_remains
  - replacement_resumed_activation_discard_stages_triggers_into_the_carrier_when_mana_payment_remains
  - every `push_activated_ability_to_stack` row (new `false` argument, ~28711-29829)
- casting.rs `sacrifice_cost_context_identity_tests::direct_mana_leg_and_none_entries_pay_both_selected_costs`
- triggers.rs `unclaimed_delayed_scan_publishes_full_buffer_occurrences`

**Related existing suites** (named in the r2 deferred list): `delayed_trigger_continuation.rs`, `integration_bending.rs` (Earthbend's delayed return through a cost), `activation_cost_target_settlement.rs`, `activation_cost_work_census.rs`, `mana_payment_class_rows.rs`, `game::triggers::dedup_regression_tests`, and the engine.rs `cost_move_drain_priority_boundary_tests`.

**Our rows that exercise the overlap** (`cost_removed_counter_amount.rs`)
- Commit path (U5), plus a carrier staged earlier in the action:
  - a_trigger_from_the_childs_payment_goes_on_the_stack_above_the_activation
  - an_sba_trigger_from_the_childs_payment_goes_above_the_activation
  - a_trigger_between_two_scry_children_goes_on_the_stack_once_above_the_activation
  - a_discard_trigger_from_a_discard_child_goes_on_the_stack_once_above_the_activation
  - a_dies_trigger_from_a_sacrifice_child_goes_on_the_stack_once_above_the_activation
  - a_mana_choice_child_finishes_and_places_the_activation_in_the_same_action
  - a_rider_free_mana_choice_child_places_the_activation_in_the_same_action
- Spell carrier (U6): a_trigger_from_a_child_under_a_mana_root_is_collected_with_the_cast
- Counter root as a CostMove owner, residual cost step after resume (U1 stand-down on our path): magosi_deferred_counter_leg_completes_its_selected_return_after_the_choice, geometric_nexus_deferred_residual_counter_leg_counts_after_the_choice
- `ActivationMillPayment` root with batch delivery (U1 park / `end_action` staging): a_prevented_child_removal_under_a_mill_cost_root_resumes_that_root, a_declined_child_removal_under_a_mill_cost_root_resumes_that_root
- Root carried across `end_action` (staging into the carrier while the root is parked): deferred_cost_removal_is_counted_after_its_replacement_choice, deferred_removals_from_among_settle_one_at_a_time, a_prevented_cost_counter_removal_resumes_its_cost_root, a_counter_cost_root_resumes_after_its_replacement_scry_child

**Coverage gap (proposed row, not written).** No row combines our counter-cost root with a *cost* park stand-down and a trigger observer. Suggested row, Magosi-shaped:
- a counter leg paused on a RemoveCounter replacement, then a selected cost move that itself pauses on a replacement;
- plus an observer of the moved object ("whenever a permanent is returned to a hand" or a leaves-battlefield Witness);
- assert exactly one Witness trigger, in the activation's batch, above the activation;
- revert check: delete the stand-down → the Witness is parked publicly (split batch, still one).

## 4. Risks and open questions for design review

**R1 (riskiest point): the U5 resolution changes upstream's push semantics.** Upstream claims the whole action buffer `AllCollectors` at the push. Ours claims only `[scope watermark..len)` minus cost-root-retained events.
- Events in `[0..watermark)` stay visible to a later pipeline pass in the same action. Such events were already staged with ordinary-only claims, or already scanned by a pipeline pass.
- Delayed observers of staged events are collected by that later pass (single collection).
- Re-scan of events a pass already scanned would require a later pass that starts below an earlier pass's end. None is known: `run_boundary_convergence_pipeline` scans from `scan_from` = len before our resume.
- This is the approved r2/C3 design, now competing directly with #9941's version. Upstream's api_* rows are the discriminating evidence. If any fails, do not widen only the claim. A wider `AllCollectors` claim hides ordinary-only-staged events from the later pipeline's delayed scan (CR 603.7b). The delayed scan and the claim must share one start. The fallback is to run both the delayed scan and the claim of `commit_announcement_span` from the same start (0, as upstream does), with ordinary collection still from the scope start. Return to design review with the failing row before applying it.

**R2: the shared scope watermark between upstream's stand-down and our mid-action staging.**
- `stage_announcement_trigger_events(state, events, start)` stages `[max(start, watermark)..len)` and then advances the watermark to `len`.
- Upstream's stand-down leaves cost events unclaimed in the buffer for `end_action`.
- If a later staging call in the **same action** starts *after* those events (`take_inline_trigger_span(start)`, our F4 `events_before`, or a pipeline pass `event_start`), the watermark jumps over them and `end_action` never stages them: a lost trigger.
- Today this needs a stood-down park (which implies a non-Priority wait) followed by more collector work in the same action. No such path was found: a paused prompt ends the action, and our resume and F4 run only under Priority. The protection is an invariant, not a construction.
- Proposed hardening (needs design review, not applied): split `AnnouncementEventScope::From { start }` into `{ inception, staged_through }`.
  - Staging always covers `[staged_through..len)`, whatever the caller's slice start, with claims keeping it idempotent.
  - `take_inline_trigger_span`'s residual becomes `start..max(start, inception)`, so only genuinely pre-announcement events are returned.
- This also closes a pre-existing nested-residual double-collect for raw-slice inline callers. The engine.rs ChooseManaColor arm (`events[owned]` → `process_triggers`) and keeper arm ~8439 (fixed by b78ff37) are the raw ones; the others are ledger-filtered after r4.

**Q1: two custody predicates.**
- `activation_announcement_is_live` is owner-based, activation-only, and falls back to `next_wait`.
- `announcement_owned_from` is scope-based and covers spells and activations.
- In engine flows they agree for activations: `begin_announcement` in `activate_with_cost_carrier`, `begin_action` for live owners.
- Making the stand-down also require `AnnouncementEventScope::From` would make the two agree by construction: never stand down when the stager would not take custody. However, upstream's direct-handler unit tests (`activation_discard_cost_events_are_left_to_the_action_boundary`, `activation_discard_for_cost_leaves_graveyard_triggers_…`) run on a fresh `GameState` whose scope is `Unowned`. They would need a scope fixture. Decide: (a) leave as merged, which is what was applied; (b) add the conjunct plus fixture updates; or (c) a `debug_assert!` in the stand-down.

**Q2: carrier writers outside `announcement_triggers`.** These predate this PR and are left in place for the merge; decide whether to fold them into the custody module afterwards.
- `settle_sacrifice_for_cost_events` collects into `pending.activation_trigger_collection` directly.
- The park functions' public path calls `claim_ordinary_collected_span`.
- Follow-up candidate: `announcement_triggers::collect_into_announcement(state, pending, events)`.

**Q3: spell scope consistency.** Upstream keeps a spell's paid-*cost* events public so they survive a cancel. Our inline custody stages a spell's resolution-time / mana-ability-child events into the spell carrier, which `end_action` drops on cancel (`cancelled_casts`).
- CR 733.1 lets a player reverse mana abilities; mana abilities that are not reversed still happened, so their triggers arguably should survive.
- This matches the base model for `end_action` spell staging (unchanged by both sides), but the PR body should state it.
- Covered by row T12d (`a_trigger_from_a_child_under_a_mana_root_is_collected_with_the_cast`) for the non-cancel case only.

**Q4: unattached-slot window.** While a root's `pending` slot is `None` (`unattached_activation_slot_mut`, used by our automatic counter leg and upstream's self-move roots), a park inside `pay_ability_cost_for_activation` sees no owner. It then parks publicly instead of standing down. The result is a split batch (CR 603.3b ordering only), with no loss or double: the public park claims the span ordinary-only. The pattern predates this PR; accept or route.

**Q5: protocol reservation.**
- Every surface states 152/134 as reserved for #10004 (Zimone), using the file's existing "reserved for the … PR" convention.
- If #10004 lands at a different number, or after us, renumber: the four Rust/TS constants, the script's +82/+81, the server-core test name, the protocol.test.ts title, the p2p gate row, and the openPhaseSocket accept/reject rows.
- When #10004 lands first, its "accepts the literal Full v152" row and its p2p gate (133→134) will conflict with ours. Take theirs as the new previous version, then re-apply ours on top.

**Q6: b78ff37.** It is not in this checkout. Land it on the branch before merging (or right after); its hunk (engine_resolution_choices.rs keeper arm, ~8439 in the merged tree) auto-merged unchanged.


## Design review dispositions (pr-sync-main-d3f002d-review.md: APPROVE-WITH-CHANGES, behavior=2 text=4 machinery=1)

- F2, F3, F4, F6 text changes applied above (F3 also applied in /home/user/wt-merge casting_costs.rs ~7392).
- F6: b78ff37 (R5-F1) is not in the cloud object store yet; the build machine performs the merge on top of b78ff37 (fetch it first), taking this resolution for the conflicted files.
- F5: the run list adds the existing gates for files the merge edited: lobby-broker and server-core unit tests (protocol tests), the frontend gates `check-frontend` (type-check + lint) and `test-frontend` (covers the three vitest files: network/__tests__/protocol.test.ts, adapter/__tests__/p2p-adapter-multiplayer.test.ts, services/__tests__/openPhaseSocket.test.ts), plus scripts/check-protocol-version.mjs.
- F1 (blocking before push, may follow the merge commit): new row in cost_removed_counter_amount.rs: a counter-cost root whose resumed selected return (Magosi class) reaches a cost park that stands down (`activation_announcement_is_live`), with two same-controller observers (the Witness and Crackdown Construct). Reach guard at the pause: wait is `ReplacementChoice` with `Cast{pending: Some}`, `deferred_triggers` empty, the Witness context in the carrier. Assert one `OrderTriggers` prompt holding both observers' triggers (as n2 does). Revert check: removing the `activation_announcement_is_live` conjunct breaks the single-prompt assertion.
- F7 (non-blocking): PR-body residual. The commit's delayed scan starts at the watermark, not the announcement start; on the ActionResult-direct inline-stage + same-action boundary-resume route no delayed scan covers that span. No reproducing card found. Follow-up paired with R2's inception/staged-through split.
- Rulings: R1 keep ours. R2 PR-body residual (follow-up with F7). Q1 leave as merged; residual note on the shared invariant. Q2 follow-up only. Q3 PR-body residual plus a follow-up cancel row. Q4 PR-body residual. Protocol 153 / wire 135 consistent; legacy MIN_SUPPORTED_PROTOCOL=152 names an unshipped version (note only).
