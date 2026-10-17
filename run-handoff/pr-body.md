<!-- claude-projects-attribution: {"github_login":"kameguru13"} -->
_Requested by **Jakob** · [project thread](https://claude.ai/code/project/chan_015qLA6HiCz3fdjbq412agV8?thread=cmsg_015qLA6HiCz3fdjbq412agV894K2BagV3yTWUnR6phraGL)_

## Summary

Before: "Whenever a [filtered] creature enters" triggers were matched against the entering permanent after the whole spell finished resolving. Elemental Bond ("Whenever a creature you control with power 3 or greater enters, draw a card.") drew a card for Match the Odds' 1/1 Ally token, because the next instruction put +1/+1 counters on it. Fractal Harness behaved the same way. Separately, What Must Be Done's "It enters with two additional +1/+1 counters on it if it's a creature" was lowered to a later `PutCounter` instead of an entry replacement.

After: enter triggers read the entrant as it was immediately after it entered (CR 603.10, CR 603.6a, CR 608.2c). Counters it entered with and continuous effects already in force count; counters and changes from later instructions of the same resolution do not. Match the Odds and Fractal Harness X=3 no longer draw with Elemental Bond; a 3/3 token and an anthem-boosted 2/2 still do.

How: two steps, one commit each.
1. Conditional entry gates (`enter_with_counters_for_object`, `effective_enter_mods`) read a CR 614.12 projection of the permanent as it would exist on the battlefield (`BattlefieldEntrantProjection`, threaded through `EntryGateContext`). What Must Be Done's rider now parses as a conditional entry counter.
2. `GameState::battlefield_entry_observations` records each battlefield entrant's characteristics, keyed by its zone-change record, at the instruction boundary after it enters (`zones::settle_battlefield_entry_observations`, called from `resolve_chain_body` and `drain_pending_continuation`). Settlement is deferred while a same-resolution "is/has" rider (CR 611.2e), a following attach (CR 303.4f) or a per-seat leg is still pending. The enters arm of `zone_change_clause_matches` evaluates its filter on the observation through `filter::matches_target_filter_on_entry_observation` and falls back to the live object when there is none. The table is cleared at turn start and in the resource-projection and visibility clones, and is registered in the CR 733 census fixtures.

The Zimone "create a token, then put N counters on it" lowering is untouched; that fix lives in #10004.

## Files changed

- `crates/engine/src/game/filter.rs`: `BattlefieldEntrantProjection`, `matches_target_filter_on_entry_observation`
- `crates/engine/src/game/effects/change_zone.rs`: `EntryGateContext`, entry-gate projection helpers
- `crates/engine/src/game/effects/mod.rs`: instruction-boundary settlement and its deferral predicate
- `crates/engine/src/game/zones.rs`: `settle_battlefield_entry_observations`
- `crates/engine/src/game/game_object.rs`: `overlay_source_projection`
- `crates/engine/src/game/trigger_matchers.rs`: enters observers read the observation
- `crates/engine/src/types/game_state.rs`: `battlefield_entry_observations` field and lookup
- `crates/engine/src/game/turns.rs`, `analysis/resource.rs`, `game/visibility.rs`: clear the table
- `crates/engine/src/parser/oracle_effect/{mod,conditions,sequence,tests}.rs`: "it enters with … if it's a [type]" as a conditional entry counter
- `crates/engine/tests/integration/conditional_entry_gate_projection.rs` (new), `etb_trigger_entry_time_characteristics.rs` (new, 26 rows), `main.rs`
- `crates/engine/tests/fixtures/cr733/authority_matrix.json.gz`, `blocked_write_sites.json.gz`: register the new field

## Track

Developer

## LLM

Model: claude-opus-5-5
Tier: Frontier
Thinking: medium (session effort setting; reported as-is)

## Implementation method (required)

Method: /engine-implementer

## CR references

Added: CR 101.4, CR 110.2a, CR 111.8, CR 122.1, CR 303.4f, CR 400.4a, CR 400.7, CR 508.4, CR 603.6a, CR 603.10, CR 603.10a, CR 608.2c, CR 611.2e, CR 611.3c, CR 613.1, CR 613.7d, CR 614.1c, CR 614.1d, CR 614.12, CR 614.12a, CR 701.27a, CR 702.26b, CR 702.92a, CR 707.5, CR 708.5, CR 712.14a. Each was checked against `docs/MagicCompRules.txt`.

## Verification

- [x] Required checks ran clean, or the exact CI-owned alternative is stated below.
- [x] Gate A output below is for the current committed head.
- [x] Final review-impl below is clean for the current committed head.
- [x] Both anchors cite existing analogous code at the same seam.

- `cargo fmt --all --check` — clean
- `./scripts/check-parser-combinators.sh` — Gate G PASS, Gate A PASS
- `./scripts/check-interaction-bindings.sh --check` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test --workspace --no-fail-fast` — 40694 passed, 2 failed: `phase-server` `data_bootstrap::tests::a_directory_that_cannot_be_synced_fails_no_step_that_otherwise_succeeded` and `a_copy_that_could_not_be_held_is_not_replaced`. Both fail identically on `main` (b1be411) in this sandbox, which runs as uid 0 so the permission checks they rely on don't apply. No `phase-server` file is touched.
- Parser measurement (`oracle-gen` + `coverage-report` + `coverage-parse-diff`, base vs head, same pinned MTGJSON): only What Must Be Done changes; supported 31956 → 31957.
- `oracle-gen semantic-audit` — 199 flagged cards at base and head, identical findings.
- Revert discrimination: the Match the Odds, Fractal Harness, Welcoming Vampire, Miraculous Recovery, library-search resume, hand-put, living weapon and departed-token rows fail with the matcher routed back to the live object.
- Merged with current upstream `main` (33aa23a, merges cleanly): `cargo clippy -p phase-engine --lib --test integration --features test-support -- -D warnings` clean; integration suite 11576 passed, 0 failed.

## Gate A

Gate A PASS head=17eb94d4b739779c63816f2400d3fb0704ba474d base=f4080d2364b5a454dffa0498b432a0d338065de3

## Anchored on

- crates/engine/src/game/filter.rs:5118 — `matches_target_filter_on_battlefield_entry`, the existing CR 614.12 "as it would exist on the battlefield" read for static entry replacements; the projection extends the same idea to effect-carried entry gates.
- crates/engine/src/game/filter.rs:5439 — `matches_target_filter_on_lki_snapshot`, the existing filter evaluation on a recorded object snapshot, used by `trigger_matchers.rs` for last-known-information triggers; the entry-observation evaluator follows it.

## Final review-impl

Final review-impl PASS head=17eb94d4b739779c63816f2400d3fb0704ba474d

## Claimed parse impact

What Must Be Done

## Scope Expansion

None.

## Validation Failures

- Residual (review LOW, no wrong behavior): `settle_entries_before_instruction` runs its deferral check on every sub-instruction before it knows whether any entry is waiting to be observed. A cheap "no pending entry" early return would skip that work on the common path.
- Not covered here, filed separately: the intervening-if recheck (CR 603.4) and evolve's comparison (CR 702.100a) still read the live object, so an evolve creature still evolves off Match the Odds' token.

## CI Failures

None.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EJVTJ7tgdqKRAN45icTrEE
