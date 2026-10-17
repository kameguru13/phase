# Phase 1 implementation review, round 1 (etb-review-fixes, phase index 1)

Mode: `review-engine-impl` phase mode on top of checkpoint mode. Charter `.git/engine-implementer-runs/etb-review-fixes/phase-charter`, phase 1. Deferral allowlist: all C1 content is `DEFERRED(phase 2)`. Reviewed span: `git diff addbc884ee251794a64f2e1da0487cd329f8ce90 d687e9bc5cb3e2237c13e76f573c29018a4a9ab6`, read with git from `/home/user/phase-personal` (HEAD d687e9b, clean, not changed). No builds were run.

```text
Review Head: d687e9bc5cb3e2237c13e76f573c29018a4a9ab6
Completion Gate: FAIL
Maintainer-Simulation Gate: PASS
```

- **Review Head.** `git rev-list addbc88..d687e9b` holds exactly one commit, d687e9b. The diff reproduces from PHASE_BASE_SHA..CANDIDATE_SHA. START_SHA equals PHASE_BASE_SHA (addbc88; round 1).
- **Completion Gate: FAIL, because evidence is missing (no defect found).** The Step 5 completion run in `/home/user/wt/complete` (`scratchpad/p1-complete.sh`, log `scratchpad/p1-complete.log`) is still in progress. As read:
  - fmt, the parser-combinator gate, authorities, test-card-data, prelowered, skill-doc, CR anchors and frames have each finished.
  - `draw_replacement_census.py --corpus --check` and `cost_walk_carrier_census.py --check` exited rc=2, because `/home/user/wt/complete/data/card-data.json` is absent. The input is missing; the censuses did not report a failure.
  - `check-interaction-bindings.sh --check`, CI clippy and the workspace tests were still compiling or not yet started.
  - `cargo coverage` / `cargo semantic-audit` on the candidate are not in the script. Step 4 measured byte-identical card data (base vs candidate), which the orchestrator may cite for them.

  Re-grade the gate once the run finishes and the two censuses have run against generated card data.
- **Maintainer-Simulation Gate: PASS.** The executor's own matrix artifact was not among my inputs, so I graded the reviewed plan's Verification Matrix (`rf-p1-plan-r3.md`) row by row against the diff.
  - V1, V2, V3, V4, V5, V5b, V5c, V6c, V6d and V19 are in `conditional_entry_gate_projection.rs`.
  - V6a (`layers.rs`) and V6b, V7, V8, V11 (`filter.rs`) are lib tests.
  - V9 is in `keywords.rs`.
  - V9b, V10, V20 and V20b are in `etb_trigger_entry_time_characteristics.rs`.
  - V18 is in `oracle_effect/tests.rs` and `conditions.rs`. V18b is in `sequence.rs` (test module only). V19b is in `oracle_effect/tests.rs`.
  - The C1 rows are `DEFERRED(phase 2)`. No phase-1 code forecloses them: the observation matcher and the D1 predicate were not touched beyond the C3 fallback, and nothing adds a dependency on an onset field.
  - Every negative assertion I checked has a paired positive reach guard in the same test: V6a (c) with (a)/(b), V8 FirstStrike with the Flying positives, V10 with `WithKeyword`, V11 with its live-arm reach, V19 case 3 with its zone assertion, V20 with V20b, V18b's "put" strings with the represented list.

Scope and addenda:
- The diff touches exactly the 9 materialized SCOPE_PATHS.
- `change_zone.rs` adds only `///` lines (verified: no non-comment `+`/`-` line).
- The `sequence.rs` hunk sits inside `#[cfg(test)] mod tests` (lines 10171–16405). That matches the decision-6 addendum (V18b).
- The decision-7 addendum (assert the postcondition rather than a break flag) is what `drive_to_empty_within` implements.

CR citations: every CR number the diff adds or moves was grep-verified in `docs/MagicCompRules.txt` and describes the annotated code: 614.12, 614.1c, 700.5, 109.5, 110.2a, 613.1/613.1a, 707.2, 603.10, 603.6a, 608.2i, 400.7, 400.3, 702.34a, 508.1a, 113.6, 114.4, 702.26b.

## Findings

**F1 [LOW] [text] The revert-discriminator sentence for the pre-move read now misstates which cases fail.** Evidence: `crates/engine/tests/integration/conditional_entry_gate_projection.rs:38-41` (module doc, first REVERT DISCRIMINATORS bullet).
- Under the restored pre-move `matches_target_filter` read, Heliod is read in the graveyard. There its own "isn't a creature" static does not function (CR 113.6), and the fixture sets its core types to Enchantment Creature, so it matches the creature gate in every case.
- So the (0, 0) and (2, 0) cases get 1 counter (expected 0). The base text of this same bullet said exactly this for the no-anchor case ("gets 1 counter (expected 0)").
- The new text says the (0, 0) case "gets 0" and implies the test discriminates only through (2, 1). That is false.

Why it matters: the bullet tells a maintainer which assertion guards the projection, and it now names the wrong case.

Suggested fix: comment-only replacement. Replace exactly:
```
//!   `recommission_reads_entrants_own_isnt_a_creature_static` gets 1 counter in
//!   its (2, 1) case (expected 1 only because devotion is already five before
//!   entry; its (0, 0) case gets 0), and the What Must Be Done March /
//!   Opalescence rows get 0 counters (expected 2).
```
with:
```
//!   `recommission_reads_entrants_own_isnt_a_creature_static` gets 1 counter in
//!   its (0, 0) and (2, 0) cases (expected 0: the graveyard card is a creature),
//!   and the What Must Be Done March / Opalescence rows get 0 counters
//!   (expected 2).
```

**F2 [LOW] [text] A production doc comment cites a run-local plan artifact.** Evidence: `crates/engine/src/game/effects/change_zone.rs:1647-1649` (doc on `entry_gate_projection`).
- "(card-data census, phase-1 plan P7)" points at a scratchpad plan that is not in the repository.
- The comment ships in production code, unlike the `V-P*` labels that live in test modules.

Why it matters: a reader cannot resolve the citation, and the claim reads as sourced when the source is unreachable.

Suggested fix: comment-only replacement. Replace exactly:
```
/// costs at most two projections. No printed card carries both gates on one
/// entrant (card-data census, phase-1 plan P7), so one shared projection is not
/// threaded through the two call paths.
```
with:
```
/// costs at most two projections. No printed card carries both gates on one
/// entrant, so one shared projection is not threaded through the two call paths.
```

## Checked and not reported (for the orchestrator's record)

- **B1 seam.** The pin is one `layers.rs` authority, `pin_entry_population_conditions`. Its only caller is `BattlefieldEntrantProjection::new`, which runs after placement, transform and the controller override, and before `mark_layers_full`/`flush_layers`. The projection's only production user is `change_zone::entry_gate_projection`, behind both gate consumers.
  - **Leaf rule.** A leaf pins when it is not recipient-context and is either population-reading or reads the origin zone. Its truth is evaluated on `pre_entry` with the carrier's scratch "you" through `ConditionContext::ability_holder`.
  - **Carriers.** Every scratch object's `base_static_definitions` and `static_definitions` (index-aligned through `Definitions::as_slice`/`get_mut`). The `CopyValues` payload statics, keyed to the recipient's controller. Transient conditions, keyed to `tce.controller`.
  - **Copy-on-write.** `Arc::ptr_eq` is asserted for untouched lists.
  - **Combinators.** `And`/`Or`/`Not` are the only nested `StaticCondition` shapes.
  - **Pinned values.** `StaticCondition::None` evaluates true (`layers.rs:2863`), and every classifier treats it as inert. The live state is never written.
  - **Pre-event state and multi-entrant moves.** Both consumers build the projection before the move. V1 being red at base and green at the candidate confirms it. Multi-entrant `ChangeZone` stays residual R5(g) as the plan establishes.
- **B2 doors.** All 19 `filter_inner_for_object` call sites pass a `HeldObject` matching the plan's disposition table.
  - `Live`: `filter_inner_with_lookup`, including-phased-out, both owner-zone arms, and the unprojected live staged entrant.
  - `Detached`: observations, records, the event snapshot, entry projections, face-down, enter-as-copy, liminal and token objects.
  - The kind arms were the only id-keyed keyword reads in `matches_filter_prop`. The record matchers already read `record.keywords`.
  - `object_has_effective_keyword_kind` keeps its answer for every id-keyed caller (`casting.rs`).
- **C3.** The fallback uses the same `entered_incarnation.is_none_or(..)` convention as `GameState::battlefield_entry_observation`.
- **C4.** `RevealedHasCardType` evaluates any-of (`effects/mod.rs:20704-20706`), so `TypeFilter::AnyOf` preserves the condition's meaning. Its `[]` → `None` arm keeps the strict path. The fold and the swallow recognizer share the one converter.
- **L1/C5.** The postcondition is checked on the state after the loop, and the existing callers delegate with 40 steps.

## Pre-existing

None reported.
