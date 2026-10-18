# Review: PR sync design, upstream main d3f002d into #10039 (review-engine-plan, findings only)

Subject: `pr-sync-main-d3f002d-plan.md` and the staged trial merge in `/home/user/wt-merge` (HEAD c909249 + MERGE_HEAD d3f002d, base 872c4e311).
Constraints honoured: read-only, no cargo/pnpm. The one command run was `node scripts/check-protocol-version.mjs` in wt-merge (exit 0). No test ran, so every runtime claim below comes from reading the code. Where a test would settle a point, this review names the test.

**Verdict: APPROVE-WITH-CHANGES**

`behavior=2 text=4 machinery=1`

The reconciliation is sound. After the merge there is one custody authority. Upstream's stand-down and our scope/claim logic do not double-collect anything or lose a trigger on any path I traced: conflict hunks, auto-merged park functions, settle, the push, `end_action`, and the action-boundary resume ordering. The changes required are small:
- one missing discriminating row (F1);
- one unsafe contingency instruction in the plan (F2);
- wording and run-list fixes (F3 to F6).

---

## Blocking findings

### F1 [MED] [behavior] No row covers the one new interaction the merge creates on our path (stand-down inside a resumed counter-cost root). Blocking before push; it can land as a commit after the merge commit.

**Evidence.**
- The plan claims (§1.1 U1, §1.2 item 2) that a residual cost step paid after our root resumes now stands down, and that its events reach the carrier through `end_action`. Magosi's selected return is the example: `resume_counter_cost_removal_payment` → `CompleteSelectedReturnToHand` → `finish_selected_return_to_hand_after_automatic` → `move_cost_objects` → `park_cost_payment_triggers_if_paused` (casting_costs.rs:2652).
- At c909249 the same park routed into the carrier through `take_pending_activation_trigger_collection`. a180ef9 deleted that branch, so the route on our path really is new.
- None of the listed rows reaches it:
  - `magosi_deferred_counter_leg_completes_its_selected_return_after_the_choice` and `geometric_nexus_…` end the return on a Priority result. `park_cost_payment_triggers_if_paused` returns at its first line for Priority, before the stand-down conjunct, so the stand-down never runs.
  - Upstream's n1/n2 rows have no counter root.

**Required revision.** Write the plan's proposed row, with these corrections so that it discriminates:
- **Batch assertion.** "Exactly one Witness trigger, in the activation's batch, above the activation" does not tell one batch from a split batch when both halves end up above the activation. Use upstream's instrument (`n2_untargeted_discard_trigger_is_in_the_activation_batch`):
  - add a second observer with the same controller, firing on the push itself (an `AbilityActivated` observer such as Crackdown Construct);
  - assert that a single `OrderTriggers` prompt holds exactly {Witness, Construct}, then that the stack is `[Activated, …]` with `stack_triggers_from(witness) == 1`.
- **Reach guard at the pause.**
  - The selected return is actually paused on its replacement: wait `ReplacementChoice`, with `PendingCostMoveResume::Cast { pending: Some(_) }` holding the activation.
  - `state.deferred_triggers` holds no Witness context; `stack` is empty.
  - The activation's carrier is present and its `pending_contexts` holds the Witness, read the way upstream's `replacement_resumed_activation_discard_stages_triggers_into_the_carrier_when_mana_payment_remains` reads it.
- **Revert column.** Removing the `activation_announcement_is_live` conjunct makes the park public, so the single-prompt assertion fails: two prompts, or Witness alone first.
- Use the verbatim Oracle text of the cost card, through `/card-test`.

### F2 [MED] [text] The R1 contingency in §4 is unsafe. Running it would hide staged events from every later delayed scan.

**Evidence.**
- The fallback widens the commit's `AllCollectors` claim to `0..len` while keeping the scope start for collection.
- Events in `[0..watermark)` that an inline take staged earlier in the same action carry only an `OrdinaryCollectorsOnly` claim, and they have had no delayed scan. `stage_announcement_trigger_events` collects ordinary triggers only, and `PendingActivationTriggerCollection::collect` uses `LogicalZoneTriggerCollection::Ordinary`.
- On the common route the reducer's own post-action pipeline runs after the push. Its delayed scan (engine_priority.rs core, filtered by `ConsumedTriggerEventScope::consumes(Delayed)`) is what collects those events. An `AllCollectors` claim over them, with no delayed scan at the commit, removes them from that pipeline. A one-shot delayed trigger such as Earthbend's return would then never fire (CR 603.7b).

**Old text (§4 R1, last bullet):**
> If any fails, the fallback is to extend `commit_announcement_span`'s claim to `0..len` minus retained (keeping the scope start for collection), not to restore upstream's body.

**Replacement:**
> If any fails, do not widen only the claim. A wider `AllCollectors` claim hides ordinary-only-staged events from the later pipeline's delayed scan (CR 603.7b). The delayed scan and the claim must share one start. The fallback is to run both the delayed scan and the claim of `commit_announcement_span` from the same start (0, as upstream does), with ordinary collection still from the scope start. Return to design review with the failing row before applying it.

---

## Non-blocking findings (apply or list as PR-body residuals)

### F3 [LOW] [text] A CR citation that exists but does not fit is carried at the push commit.
- `casting_costs.rs` ~7397 (merged): `// CR 602.2b + CR 603.3b + CR 603.7c: the activation transaction owns its cost-payment events' trigger batch, delayed triggers included (Earthbend's dies-or-exiled return); the announcement authority commits it.`
- CR 603.7c (grep-verified, line 2624) is about a delayed trigger that refers to a particular object and how it tracks that object at resolution. It does not cover when a delayed trigger is collected. Upstream corrected this same site in a180ef9 to `CR 603.2c + CR 603.7b`.
- **Replacement:** `// CR 602.2b + CR 603.3b + CR 603.7b: the activation transaction owns its cost-payment events' trigger batch, delayed triggers included (Earthbend's dies-or-exiled return, which triggers only once); the announcement authority commits it.`
- Plan §1.2 item 1: change "CR 603.7c verified" to "CR 603.7b verified".

### F4 [LOW] [text] The U3 claim is broader than the code supports.
- §1.1 U3 says: "Every suffix root rebuilt after a counter cost now refuses `CancelCast`."
- That holds for the selected path: `pay_selected_counter_cost` calls `mark_activation_cost_committed()` before it parks, and again on Complete. It is not established for the automatic counter leg:
  - that leg parks `CounterCostRemovalPayment { pending: None }`;
  - the activation is attached unmarked (`pending_activation_after_cost_pause` at casting.rs:23715; push_activated_ability_to_stack marks only when `activation_cost_committed` was already true);
  - `resume_counter_cost_removal_payment` never marks.
- No row covers either path for cancel.
- **Replacement:** "A selected counter cost marks the activation committed before it parks and when it completes, so every suffix root that `push_activated_ability_to_stack` rebuilds after it refuses `CancelCast` (upstream's threading). An automatic counter leg paused at its first component is not marked on resume. That is the same pattern as upstream's automatic self-move root and is listed as a follow-up."
- Do not put the broader sentence in the PR body. The automatic-leg gap is in the Pre-existing list below.

### F5 [LOW] [machinery] The run list leaves out the suites whose files the merge edited outside the engine.
- The resolution edits:
  - Rust protocol tests: `lobby-broker` `protocol_version_tracks_full_game_wire_additions`, and `server-core` `protocol_version_is_153_for_counter_cost_removal_payment` / `full_game_floor_is_current_only_not_a_rollout_window`;
  - three vitest files: `openPhaseSocket.test.ts`, `network/__tests__/protocol.test.ts`, `p2p-adapter-multiplayer.test.ts` ("P2P wire-protocol version gate" and the two Bestow rows);
  - two TS sources.
- §3 lists only engine suites, clippy and the script.
- **Close by substituting existing gates:** the `check-frontend` and `test-frontend` Tilt resources (or `pnpm` type-check and vitest on those three files when Tilt is down), and the lobby-broker / server-core unit tests through the standard runner. Nothing new is needed.

### F6 [LOW] [text] Run-list identifiers and counts.
- "`triggers_dedup_regression_tests`" does not exist as a module name. The module is `game::triggers::dedup_regression_tests` (`#[path = "triggers_dedup_regression_tests.rs"]`, triggers.rs:51988). Use that filter.
- "(72 rows; 73 once b78ff37 lands, which adds no row)" contradicts itself, and the figure is a snapshot. Replace it with the durable form: "every row in `cost_removed_counter_amount.rs`".
- b78ff37 is not reachable in `/home/user/phase-personal`'s object store (`git show b78ff37` fails). Add an explicit step: fetch the branch head that holds it, then apply it before the merge commit. Its "applies cleanly" claim is unverified here.

### F7 [LOW] [behavior] The delayed scan at the commit starts from the watermark, not the inception (R1 residual). Non-blocking; list it in the PR body and fold it into the R2 follow-up.
- `commit_announcement_span` scans for delayed triggers from `start` = the advanced watermark. Upstream's body scanned from 0, over events not claimed for the Delayed requester.
- Events staged as ordinary-only earlier in the same action therefore get their delayed scan from the reducer's later pipeline, not from the commit. That covers every route that runs the pipeline after the push.
- One route has no delayed scan for that span: a handler that returns `ActionResult` directly and stages inline (`collect_and_drain_observer_triggers_if_settled`, `park_observer_triggers_if_paused`), followed in the same action by the action-boundary resume that pushes the activation.
  - The commit starts at the watermark.
  - `run_boundary_convergence_pipeline` starts at `scan_from`, which is after the span.
- I found no card-level reproduction: it needs such a handler under our counter root and a delayed observer of its events. It does not reproduce at c909249 any differently; the merge only makes the contrast with upstream visible.
- The R2 split (`inception` / `staged_through`) closes it if the commit's delayed scan and claim both use `inception`.

---

## Rulings on the plan's questions

**R1: our scope-watermark claim against upstream's whole-buffer claim.** Keep ours. Not a problem for ordinary collection or for the claim:
- Every resumed action enters with `From { start: 0 }` (`begin_action`). The fresh `ActivateAbility` action begins at `events.len()` ≈ 0. So `[0..start)` is empty or pre-announcement on every route I traced, and leaving pre-announcement events to the priority pipeline is the more correct reading.
- The retained-exclusion difference is theoretical. `finish_sacrifice_for_cost` takes the root before the settle and the push, so `retained_by_cost_owners` is empty at the push.
- The matcher-claim publication in upstream's `collect_unclaimed_delayed_triggers_into_deferred` is subsumed by both sides' index claims.
- The only real difference is the delayed scan start (F7, residual).
- The fallback text must change (F2).

**R2: the watermark could skip stood-down events.** Acceptable residual; list it in the PR body. Do not apply the split in this merge.
- The skip mechanism predates both sides: `advanced_to` and pipeline staging at engine_priority.rs:156/169 exist at 872c4e311 and on main. Our PR adds callers.
- The merge adds no path where a stood-down span is followed, in the same action, by a staging call that starts after it:
  - The stand-down fires only for a non-Priority next wait. That wait ends its caller chain: every `park_*` caller returns the wait immediately.
  - Every later-start stager is gated on a Priority result: F4 in `resume_parked_counter_cost_root_at_action_boundary`, the convergence pipeline, and the resume itself (`!matches!(waiting_for, Priority)` → `None`).
  - The inline takes reachable after a stood-down park (engine_payment_choices.rs:251/420/567, the ChooseManaColor arm) capture their start before the paying call, so they stage the stood-down span.
- The split changes the residual contract of all 13 inline sites and needs its own design round and rows. Pair it with F7.

**Q1: two ownership predicates.** Option (a), leave as merged.
- In engine flows they agree:
  - `begin_action` sets `From { 0 }` whenever any owner is live, including the `Prompt` owner read from `waiting_for`;
  - `activate_with_cost_carrier` calls `begin_announcement` before any cost;
  - the commit is the only place that returns the scope to `Unowned`, and it runs after the last cost.
- Upstream main relies on the same invariant.
- Options (b) and (c) both break upstream's direct-handler lib tests (`activation_discard_cost_events_are_left_to_the_action_boundary`, `activation_discard_for_cost_leaves_graveyard_triggers_…`). They run on a fresh `GameState`, where the scope is `Unowned`, and a `debug_assert!` would fire in them.
- Mention the shared invariant in the PR body; it does not need to be a follow-up.

**Q2: settle's direct carrier writes.** Not a problem for the merge; follow-up only.
- `settle_sacrifice_for_cost_events` writes the same carrier the stager would.
- Its `AllCollectors` claim on the current fragment, and our `retained_by_cost_owners` exclusion, give each fragment exactly one collector.
- A spell's carrier is in flight during the action, so a spell settle falls through to the public queue, which matches upstream's spell policy.

**Q3: a spell's mana-ability child events in a cancellable carrier.** Acceptable residual; state it in the PR body. The merge did not introduce it.
- `end_action` already staged every event of a non-Priority action into the spell carrier at base.
- Upstream's "spell paid-cost events stay public" only covers costs that call the park functions. A counter removal, for example, never parked, even at base.
- Our inline sites extend the same model to resolution-time child events. Row T12d covers only the case without a cancel.
- Add a follow-up row: cancel after a spell's mana-child event, deriving the expected outcome from CR 733.1's "may reverse mana abilities" clause.

**Q4: an empty pending slot parks publicly.** Acceptable residual for the PR body.
- Nothing is lost or doubled:
  - the public park claims `OrdinaryCollectorsOnly`, so both the stager and the commit skip the span;
  - the span stays visible to delayed requesters, so it gets exactly one delayed scan.
- The window is not new: the pre-a180ef9 carrier branch also needed a findable owner.

---

## Protocol renumbering (153 / wire 135; 152/134 reserved for #10004)

Consistent. No finding.
- lobby-broker: the doc entry; `PROTOCOL_VERSION = 153`; tests at 153, and `MIN_SUPPORTED_PROTOCOL == 152` (derived as `saturating_sub(1)`).
- server-core: the v153 doc entry; the test renamed `_153_`; the REVERT-PROBE reference updated.
- check-protocol-version.mjs: +82 / +81 with reservation comments; it requires `protocol_version_is_153` and refuses `_152`; exit 0.
- ws-adapter: `PROTOCOL_VERSION = 153`. `MIN_SUPPORTED_SERVER_PROTOCOL` is derived.
- protocol.ts: `WIRE_PROTOCOL_VERSION = 135`.
- protocol.test.ts: pins 135.
- p2p gate row: refuses 134, admits 135. The history list gains 132 and 133.
- Bestow rows: admit 135 and refuse 132, the genuinely pre-Bestow peer that the exact-match gate still refuses.
- openPhaseSocket: rejects 151, accepts 153, and keeps main's 149 and 150 reject titles.
- No stale 151/133 admit literal remains: grepped client/src, server-core, lobby-broker, phase-server and lobby-worker.
- Note, not a finding: the legacy lobby window `MIN_SUPPORTED_PROTOCOL = 152` names a version that never shipped. That window serves only clients that predate `LOBBY_PROTOCOL_VERSION`, so it is inert, and the file's earlier 119 reservation set the precedent.

## Tests to run first

The list is right in substance. The upstream api_* and v21/v21b rows are the correct discriminators for U5. Changes:
- add the row from F1;
- add the non-engine gates from F5;
- correct the identifiers from F6.

## CR verification (grep of docs/MagicCompRules.txt)

- Present and fitting: 118.11 (l.1030), 601.2h (2478), 601.2i (2481), 602.2a (2535), 602.2b (2537), 603.2c (2573), 603.3b (2592), 603.7b (2622), 603.10a (2644), 605.3b (2700), 605.4a (2706), 616.1 (3179), 733.1 (6415).
- Present but not fitting at its site: 603.7c (2624), at the push commit (F3).

## Pre-existing (at c909249; untagged; not blocking)

- **The automatic counter leg is never marked committed.** `CounterCostRemovalPayment { pending: None }` is attached unmarked (casting.rs:23715 `pending_activation_after_cost_pause`), and `resume_counter_cost_removal_payment` never calls `mark_activation_cost_committed`. A suffix prompt after a settled automatic counter removal may therefore still accept `CancelCast`. Upstream's automatic self-move root has the same shape.
- **A nested residual can be collected twice by raw-slice inline callers** (the plan's R2 note). This affects the ChooseManaColor arm, engine.rs ~13288 (`events[owned]` → `process_triggers`), when an inner stage advanced the watermark past the outer `events_before`. The keeper arm is fixed by b78ff37.
