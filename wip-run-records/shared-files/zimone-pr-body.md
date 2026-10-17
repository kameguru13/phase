<!-- claude-projects-attribution: {"github_login":"kameguru13"} -->
_Requested by **Jakob** · [project thread](https://claude.ai/code/project/chan_015qLA6HiCz3fdjbq412agV8?thread=cmsg_015qLA6HiCz3fdjbq412agV8WKRZYgnopiKDCM3v3gp2ns)_

## Summary

Implements Zimone, All-Questioning: a new `NumberQuality` (prime / odd / even) condition predicate on static, trigger and ability conditions, the "you control a/an prime|even|odd number of <type>" grammar, and binding of "that many" to a positive count gate. Also fixes a pre-existing parser fallthrough: "Cast this spell only <condition>" lines whose condition can't be represented now become an honest `Unimplemented` gap instead of a `CastFromZone` self-recast effect.

Review round 1 (F1), fixed class-wide: the imperative "create … token, then put N counters on it" (and its two-sentence form "Create … token. Put … counter(s) on it.") no longer folds into the token's entry counters. It lowers to the existing `Token` followed by `PutCounter { target: LastCreated }` as a `ContinuationStep`, so the token enters as printed and then gets its counters (CR 122.6, CR 608.2c). This moves 27 cards, including Zimone. Entry counters stay reserved for real "enters with" text (Printlifter Ooze is unchanged). The phase-ai readers were updated first so the moved cards keep their tactical verdicts.

## Files changed

- `crates/engine/src/types/ability.rs`, `types/ability_visit.rs`, `types/mod.rs`: `NumberQuality`, `QuantityQuality` condition arms (incl. upstream's new `is_casting_condition` match)
- `crates/engine/src/game/{ability_rw,ability_scan,casting,coverage,filter,layers,quantity,triggers}.rs`, `game/effects/{effect,token}.rs`: evaluation and walker arms
- `crates/engine/src/game/effects/mod.rs`: walker arms; `is_player_scope_local_continuation` keeps a `Token` → `PutCounter{LastCreated}` pair inside each player's iteration (Oversimplify)
- `crates/engine/src/parser/oracle_nom/condition.rs`: `parse_number_quality`, `parse_control_count_quality`
- `crates/engine/src/parser/oracle_trigger.rs`, `oracle_trigger_tests.rs`: static→trigger bridge, `positive_count_gate_qty` (renamed from `positive_had_counter_gate_qty`), binding after condition remaps
- `crates/engine/src/parser/oracle_ir/ast.rs`: `ContinuationAst::PutCountersOnLastCreated`
- `crates/engine/src/parser/oracle_effect/{mod,sequence,conditions,lower}.rs`: `CountPrefix::ThatMany`, the create-then-put-counters continuation (fails closed without a `Token` antecedent), ability bridge
- `crates/engine/src/parser/oracle_effect/snapshots/…assembly_create_token_put_counter.snap`: updated snapshot
- `crates/engine/src/parser/{oracle,oracle_casting}.rs`: cast-only frame predicate and honest gap
- `crates/engine/src/parser/{oracle_condition,oracle_replacement,swallow_check}.rs`, `oracle_static/{shared,static_helpers}.rs`: fail-closed and walker arms
- `crates/engine/tests/integration/{main,number_quality_gate_reach,zimone_all_questioning,created_token_counter_sequence}.rs`: new tests
- `crates/engine/tests/integration/{gimbal_gremlin_prodigy,oversimplify_per_player_fractal,where_x_totality_guard}.rs`: updated for the new shape
- `crates/phase-ai/src/ability_chain.rs`: shared `created_token_placement_source` predicate and `AbilityWalk`
- `crates/phase-ai/src/policies/context.rs`: `ability_walks`, `effects_without_created_token_placements`
- `crates/phase-ai/src/{cast_facts,features/plus_one_counters}.rs`, `policies/{anti_self_harm,plus_one_counters,redundancy_avoidance,x_cast_gate,x_reference}.rs`, `policies/mulligan/plus_one_counters_keepables.rs`: readers moved onto the shared predicate
- `crates/phase-ai/src/policies/tests/{created_token_placement_parity,mod}.rs`: registry-wide verdict parity test
- Protocol bump 144→145, wire 126→127 (on top of upstream's v144 boons bump): `crates/lobby-broker/src/protocol.rs`, `crates/server-core/src/protocol.rs`, `client/src/adapter/ws-adapter.ts`, `client/src/network/protocol.ts`, `client/src/network/__tests__/protocol.test.ts`, `client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`, `client/src/services/__tests__/openPhaseSocket.test.ts`, `scripts/check-protocol-version.mjs`

## Track

Developer

## LLM

Model: claude-opus-5-5
Tier: Frontier
Thinking: medium (session effort setting; reported as-is)

## Implementation method (required)

Method: /engine-implementer

Original change: a chartered run in 2 phases (predicate types + runtime; grammar + anaphor + card). Review fix F1: a second chartered run in 2 phases (phase-ai readers; parser lowering + tests). Each phase had plan review, implementation review and Step 5 checks, then a run-level integration review (clean), then a full-PR review at the final head.

## CR references

CR 107.1, 107.1b, 107.3, 111.1, 111.4, 115.1, 115.10a, 119.4, 122.1, 122.2, 122.6, 122.6a, 201.2, 301.5b, 508.1, 508.5, 513.1, 601.3, 602.2b, 602.5, 603.4, 603.6a, 603.7a, 603.10a, 603.12, 607.2a, 608.2c, 608.2h, 608.2i, 611.3a, 614.1a, 614.1c, 616.1, 700.2, 701.57c, 704.4, 704.5f (all grep-verified in docs/MagicCompRules.txt).

## Verification

- [x] Required checks ran clean, or the exact CI-owned alternative is stated below.
- [x] Gate A output below is for the current committed head.
- [x] Final review-impl below is clean for the current committed head.
- [x] Both anchors cite existing analogous code at the same seam.

All at head dfc1a50 (branch merged with upstream main 33aa23a). Tilt was not running, so commands ran directly:

- `cargo fmt --all --check` — clean
- `./scripts/check-parser-combinators.sh` — Gate A PASS (below)
- `cargo clippy -p phase-engine --features test-support --lib --tests -- -D warnings` — clean
- `cargo clippy -p phase-ai --all-targets -- -D warnings` — clean
- `cargo clippy -p server-core -p lobby-broker --lib --tests -- -D warnings` — clean
- (clippy runs per crate set instead of workspace all-targets, which runs out of sandbox memory)
- `cargo test -p phase-engine --features test-support --lib` — 24161 passed
- `cargo test -p phase-engine --features test-support --test integration` — 11569 passed
- `cargo test -p phase-ai` — 2958 passed
- `cargo test -p server-core -p lobby-broker --lib` — 777 passed
- `./scripts/check-interaction-bindings.sh --check`, `node scripts/check-protocol-version.mjs` — pass
- `pnpm --dir client run type-check`, `pnpm --dir client lint` (0 errors), vitest full run — pass
- `./scripts/gen-card-data.sh` — ok; `cargo coverage` — 31957 / 35923 supported, equal to upstream 33aa23a; `cargo semantic-audit` — 198 cards with findings
- `./scripts/ai-gate.sh` — 0 FAIL, 1 WARN (affinity-mirror, L→W = 2, p = 0.125). The same WARN was measured at the base before the F1 work, so it is pre-existing baseline drift; no baseline refresh.
- Parse diff vs upstream 33aa23a card data (same MTGJSON vintage): exactly the 30 cards below; card key sets identical

## Gate A

Gate A PASS head=dfc1a500f285e808fe6b6d70bc96d4e4adf55a65 base=b5d4fde6021b9219e2325321ebdf60c97f4038a1

## Anchored on

- crates/engine/src/types/ability.rs:1819 — `Parity`, the existing mana-value parity predicate `NumberQuality` reuses; `QuantityComparison` (ability.rs:12522) is the sibling condition arm `QuantityQuality` mirrors across the three condition enums
- crates/engine/src/parser/oracle_effect/sequence.rs:5327 — `ContinuationAst::SuspectLastCreated` / `GoadLastCreated`, the existing continuations that act on the just-created token through `TargetFilter::LastCreated`; `PutCountersOnLastCreated` is their sibling
- crates/engine/src/game/effects/mod.rs:6942 — `is_player_scope_local_continuation`, the existing per-player continuation scope, extended with the `Token` → `PutCounter{LastCreated}` pair

## Final review-impl

Final review-impl PASS head=dfc1a500f285e808fe6b6d70bc96d4e4adf55a65

## Claimed parse impact

From the original change:

- Zimone, All-Questioning: unsupported → supported (also moves to the new token shape below)
- Even the Odds: was parsed as a `CastFromZone` self-recast; now an honest `casting_restriction` gap (supported → unsupported)
- Suffocation: same gap change as Even the Odds (unsupported before and after)
- Checks and Balances: still unsupported; gap renamed from `unknown` to `casting_restriction`

From F1, 27 cards change shape from `Token { enter_with_counters }` to `Token` + `PutCounter { target: LastCreated }` (one placement each, two for Lattice Library). Support is unchanged for all of them (Kasmina, Enigma Sage was already unsupported for another clause):

Additive Evolution, Berta, Wise Extrapolator, Body of Research, Deekah, Fractal Theorist, Emil, Vastlands Roamer, Fractal Anomaly, Fractal Summoning, Fractal Tender, G'raha Tia, Scion Reborn, Geometric Nexus, Gimbal, Gremlin Prodigy, Hiveheart Shaman, Kasmina, Enigma Sage, Lattice Library, Leyline Invocation, Manifestation Sage, Oversimplify, Paradox Zone, Primo, the Unbounded, Recon Craft Theta, Rot Like the Scum You Are, Sequence Engine, Serpentine Curve, Slime Against Humanity, Wild Hypothesis, Zaxara, the Exemplary, Zimone, All-Questioning

Net coverage is unchanged against upstream (31957): Zimone +1, Even the Odds −1.

## Scope Expansion

- The "Cast this spell only …" fix is outside Zimone's card scope. A Zimone test (a prime-count casting gate must fail closed) exposed that any such line with an unrepresentable condition fell through to the spell-effect parser and shipped as a self-recast effect. Leaving it would have marked a hypothetical prime casting gate as supported and keeps Even the Odds and Suffocation silently wrong.
- F1 covers the whole imperative class (27 cards), not only Zimone, per the review: fixing only Zimone would leave the same enters-with-counters defect on the other 26.

## Validation Failures

- Full-DB `FORGE_TEST_FULL_DB=1 cargo test -p phase-engine --lib ordering_parity_sweep` failed at the earlier head b4685f2 with 25 unexplained diffs and stale `DOCUMENTED_OVER_PROMPT` entries, identically against upstream d259a48's card data (card-data drift, not this PR). It was not rerun at dfc1a50.
- Process deviations: clippy ran per crate set instead of workspace all-targets (sandbox memory); one checkout served as implementation and completion worktree (disk limits).
- `crates/engine/tests/integration/dandan_scoped_counts.rs` (Body of Research) still reads the stale `integration_cards.json.gz` fixture; the Oversimplify test moved to a verbatim parse. Refreshing that fixture is a follow-up.

Follow-ups (pre-existing, not changed here):
- Enter-trigger timing (CR 603.10a): an enter trigger checks the live object after resolution, so Elemental Bond sees a token's later counters. Reproduces with either token shape; fix in progress on `kameguru13/phase-personal` branch `engine/etb-entry-snapshot`, PR to follow.
- Counters removed as an activation cost read 0 for "removed this way" / "for each counter removed" (Geometric Nexus, Essence Bottle, Molten Hydra, Relic Amulet, Blademane Baku, the Mana Batteries, the storage lands; 27 cards). Identical under both token shapes; tracked as its own follow-up.
- CodeRabbit #1 / #2: identical behavior on the sibling `QuantityCheck` / `TriggerCondition::QuantityComparison` paths; unreachable from any grammar today.
- phase-ai: Fractal Harness's `Attach{LastCreated}` still sets the ETB-target flag; `analysis/ability_graph.rs` doesn't count declarative `enter_with_counters` as counter production; Kianne with nothing studied loses its zero-count penalty although its 0/0 dies.
- A quality gate on an event trigger would also capture a "that much" anaphor (no printed card; assumption documented at `positive_count_gate_qty`).
- `companion.rs` computes parity inline instead of `Parity::matches`.
- Suffocation's damage parses as `DealDamage { target: Any }`.
- Vazi drops its Treasure condition; Shoreline Scout drops "or an Island"; the engine names Gimbal's token "Gremlin" rather than "Gremlin Token" (CR 111.4).

## CI Failures

None.

```text
Pipeline-reviewed head: f2733a35fcff896864107a84fa13d7181fcfbb4d
Correction commit: none
Current branch head: dfc1a500f285e808fe6b6d70bc96d4e4adf55a65
Pipeline status: historical — merged upstream main 33aa23a after acceptance
Current-head review: clean at dfc1a500f285e808fe6b6d70bc96d4e4adf55a65
```

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_0142hFNYJJM5M1dvVyQVsp9G
