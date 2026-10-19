<!-- claude-projects-attribution: {"github_login":"kameguru13"} -->
_Requested by **Jakob** · [project thread](https://claude.ai/code/project/chan_015qLA6HiCz3fdjbq412agV8?thread=cmsg_015qLA6HiCz3fdjbq412agV894K2BagV3yTWUnR6phraGL)_

## Summary

**Before:** triggers of the form "Whenever a [filtered] creature enters" were matched against the entering permanent only after the whole spell had finished resolving. So Elemental Bond ("Whenever a creature you control with power 3 or greater enters, draw a card.") drew a card for Match the Odds' 1/1 Ally token, because the next instruction put +1/+1 counters on it. Fractal Harness behaved the same way. Separately, What Must Be Done's "It enters with two additional +1/+1 counters on it if it's a creature" was lowered to a later `PutCounter` instead of an entry replacement.

**After:** enter triggers read the entrant as it was immediately after it entered (CR 603.10, CR 603.6a, CR 608.2c). Counters it entered with, and continuous effects already in force, count. Changes made by later instructions of the same resolution do not. Match the Odds and Fractal Harness X=3 no longer draw with Elemental Bond; a 3/3 token, or a 2/2 boosted by an anthem, still does.

**Entry gates:** effect-carried gates ("If a creature enters this way, it enters with an additional +1/+1 counter", and gated tapped/attacking riders) now read the permanent as it would exist on the battlefield (CR 614.12), on the board as it was before the entry:
- Population conditions use the pre-entry population. This covers devotion, metalcraft and enchantment counts, including conditions inside statics granted by another effect, where each one is evaluated with its recipient's controller as "you" (CR 109.5). This follows the Heliod, Sun-Crowned ruling of 2020-01-24: Recommission returning Heliod at four white devotion gives no counter.
- Keyword-kind filters read the saved entry view.
- In a multi-object move, every member's gates are resolved before any member moves (CR 603.6a, one event). Those results are carried through pauses as `ChangeZoneMemberEntry`.

**Saved games:** the parked multi-object carrier changed shape, so persisted resolution state moves to version 7:
- A v1–v6 save paused inside an ungated multi-object move migrates exactly.
- A gated one is refused with `LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR`, because its pre-event board is not in the save.
- The v1 carrier is migrated inside the v1 reader arm, so the resolution-frame compatibility boundary holds.
- The network protocol moves to 155 (wire 137). Main holds 152/134 (#10098), and 153/135 and 154/136 are reserved for #10004 and #10039.

**Integration with main:** at the chain walker's sub-ability transition, main's cast-metadata splice (#9477) first selects the next written instruction. Entries are then settled before that instruction (CR 603.10 + CR 608.2c). A pure-metadata chain finishes without settling, because no later written instruction can change an entrant there.

**Scope:** the onset deferral for "gains/gets/becomes" riders (CR 611.2e, the "C1" CodeRabbit item) moves to a separate follow-up PR. This PR keeps the existing conservative deferral.

The Zimone "create a token, then put N counters on it" lowering is untouched; that fix is in #10004.

## Files changed

- `crates/engine/src/game/filter.rs`: `BattlefieldEntrantProjection` (CR 614.12 projection with the pre-entry population pin), `matches_target_filter_on_entry_observation`, incarnation-checked fallback, keyword-kind reads through `HeldObject`
- `crates/engine/src/game/layers.rs`: `pin_entry_population_conditions`, including per-recipient pinning of `GrantStaticAbility` conditions
- `crates/engine/src/game/keywords.rs`: `HeldObject { Live, Detached }`, `effective_keyword_kind_of`
- `crates/engine/src/game/effects/change_zone.rs`: `EntryGateContext`, `ChangeZoneEntrySpec`, `resolve_change_zone_member_entries`
- `crates/engine/src/game/effects/mod.rs`: instruction-boundary settlement (after main's cast-metadata splice) and its deferral predicate; the paused ChangeZone drain applies carried member entries
- `crates/engine/src/game/engine_resolution_choices.rs`: the EffectZoneChoice and PayCost resumes precompute member entries
- `crates/engine/src/game/effects/search_outside_game.rs`, `crates/engine/src/game/elimination.rs`: carrier retype
- `crates/engine/src/game/zones.rs`: `settle_battlefield_entry_observations`
- `crates/engine/src/game/game_object.rs`: `overlay_source_projection`
- `crates/engine/src/game/trigger_matchers.rs`: enters observers read the observation
- `crates/engine/src/types/game_state.rs`: `battlefield_entry_observations`; `ChangeZoneMemberEntry`
- `crates/engine/src/types/resolution.rs`: resolution state v7; carrier migration/refusal (`migrate_legacy_change_zone_carrier`, reached from the v1 reader arm and, for v2–v6 frames, from `migrate_legacy_change_zone_member_entries`); unversioned v7 inference
- `crates/engine/src/game/turns.rs`, `crates/engine/src/analysis/resource.rs`, `crates/engine/src/game/visibility.rs`: clear or strip the table
- `crates/engine/src/parser/oracle_effect/{mod,conditions,sequence,tests}.rs`: "it enters with … if it's a [type] (or [type])" as a conditional entry counter
- `crates/engine/tests/integration/conditional_entry_gate_projection.rs` (new), `etb_trigger_entry_time_characteristics.rs` (new), `main.rs`, `exploit_object_filter.rs` and `resolution_progress_contract.rs` (v7 pins)
- `crates/engine/tests/fixtures/cr733/authority_matrix.json.gz`, `blocked_write_sites.json.gz`: register the new field
- `scripts/post-replacement-continuation-baseline.txt`
- Protocol bump to full 155 / wire 137, after main's 152/134 (#10098), with 153/135 and 154/136 reserved for #10004 and #10039:
  - `crates/lobby-broker/src/protocol.rs`
  - `crates/server-core/src/protocol.rs`
  - `crates/phase-server/src/main.rs` (hello-gate pin)
  - `scripts/check-protocol-version.mjs`
  - `client/src/adapter/ws-adapter.ts`
  - `client/src/network/protocol.ts`
  - `client/src/network/__tests__/protocol.test.ts`
  - `client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`
  - `client/src/services/__tests__/openPhaseSocket.test.ts`

## Track

Developer

## LLM

Model: Claude Opus 5.5
Tier: Frontier
Thinking: high

## Implementation method (required)

Method: /engine-implementer

## CR references

Added or touched; each was checked against `docs/MagicCompRules.txt`: CR 101.4, CR 107.3m, CR 109.5, CR 110.2a, CR 111.8, CR 113.6, CR 114.4, CR 115.1d, CR 115.10, CR 118.3, CR 122.1, CR 122.1h, CR 202.3, CR 303.4f, CR 400.3, CR 400.4a, CR 400.7, CR 508.1a, CR 508.4, CR 601.2c, CR 603.6a, CR 603.10, CR 603.10a, CR 608.2c, CR 608.2i, CR 611.2e, CR 611.3c, CR 613.1, CR 613.1a, CR 613.1b, CR 613.7d, CR 614.1c, CR 614.1d, CR 614.12, CR 614.12a, CR 700.5, CR 700.6, CR 701.27a, CR 702.26b, CR 702.34a, CR 702.92a, CR 704.3, CR 707.2, CR 707.5, CR 708.5, CR 712.14a.

## Verification

- [x] Required checks ran clean, or the exact CI-owned alternative is stated below.
- [x] Gate A output below is for the current committed head.
- [x] Final review-impl below is clean for the current committed head.
- [x] Both anchors cite existing analogous code at the same seam.

Latest merge (4a0e09df2, upstream main 57c74815f): only protocol constants, history entries and their tests changed (152/134 → 155/137). Checked locally: `node scripts/check-protocol-version.mjs`, `./scripts/check-resolution-frame-boundaries.sh`, Gate A, `cargo check --workspace --all-targets`, lobby-broker/server-core protocol tests (30 + 96), phase-server handshake tests (30), client type-check and the three client protocol test files (332 passed), plus the pre-push hook. Everything else is left to required CI on this head.

Batched verification for the previous head ran in a clean worktree at 425e0dc17, the merge with upstream main d3f002dae. It mirrors every step of the CI "Rust lint" job:
- `cargo fmt --all -- --check`, `./scripts/check-interaction-bindings.sh --check`, `./scripts/check-parser-combinators.sh <base>`, `./scripts/check-prelowered-ratchet.sh`, `./scripts/check-skill-doc.sh`, `./scripts/check-cr-citation-anchors.sh`, `./scripts/check-engine-authorities.sh <base>`: all clean.
- `./scripts/check-resolution-frame-boundaries.sh`, `./scripts/check-test-card-data-load.sh <base>`, `node scripts/check-protocol-version.mjs`: all clean.
- The CI lint job's Python checker tests: all clean, except one timing failure in `preview_data_wait_tests.py` under local load. It passed 3/3 on rerun, and this PR does not touch it.
- `./scripts/gen-card-data.sh`: clean.
- `cargo clippy --locked --workspace --exclude phase-tauri --all-targets --features engine/proptest -- -D warnings`: clean
- `cargo test --locked --workspace --exclude phase-tauri --features engine/proptest --no-fail-fast`: 41558 passed, 1 failed. The failure was `phase-server` `handshake_tests::bestow_payload_full_game_refuses_previous_and_accepts_current`, a protocol pin from main. It was fixed in 8a4618688, and `cargo test -p phase-server` on the head gives 429 passed, 0 failed. Every other result carries over from 425e0dc17, because 8a4618688 changes only that server test.
- `pnpm run type-check`: clean. `pnpm lint`: 0 errors. `vitest run`: 9837 passed.
- Parser measurement (`oracle-gen` + `coverage-report` + `coverage-parse-diff`): base d3f002dae vs head, same pinned MTGJSON. Only What Must Be Done changes, 2 signatures; supported 31952 → 31953, 0 regressions.
- `oracle-gen semantic-audit`: 208 findings at base and at head, identical, 0 new.
- Revert discrimination:
  - The Match the Odds, Fractal Harness, Welcoming Vampire, Miraculous Recovery, library-search resume, hand-put, living weapon and departed-token rows fail with the matcher routed back to the live object.
  - The Heliod four/five-devotion rows fail without the population pin.
  - The granted Shrine/Starfield rows fail with the grant arm removed or keyed to the host's controller.
  - The multi-object metalcraft (Ornithopter + Rusted Relic), co-entering March of the Machines and paused-drain rows fail when gates are resolved after earlier members moved.
  - The persisted-reader rows (raw and trusted envelopes, v1–v6) fail without the v7 migration, the gated refusal, or the v1 reader arm's migration call.
  - `cast_metadata_rider_boundary_is_the_next_written_instruction` fails if entries are settled before main's cast-metadata splice.
- Pre-push hook passed on push: workspace clippy, card-data validate, parser tests, phase-ai lib, oracle-gen, coverage regression check, client lint and type-check.

## Gate A

Gate A PASS head=4a0e09df2b2023a08090fa5d4f265b7463be6912 base=57c74815f6f5e6ba14cde1cc298e3a5afaa27a3c

## Anchored on

- crates/engine/src/game/filter.rs:5156 — `matches_target_filter_on_battlefield_entry`: the existing CR 614.12 "as it would exist on the battlefield" read for static entry replacements. The projection extends it to effect-carried entry gates.
- crates/engine/src/game/filter.rs:5486 — `matches_target_filter_on_lki_snapshot`: the existing filter evaluation on a recorded snapshot, which the entry-observation evaluator follows.
- crates/engine/src/types/game_state.rs:26445 — `push_devour_change_zone_snapshot`: the existing capture-once-before-any-member-moves pattern for a ChangeZone event, carried through pauses. Pre-event member entries follow it.
- crates/engine/src/types/resolution.rs:4095 — `reject_ambiguous_legacy_paused_draw_result`: the existing version-keyed refusal of a legacy paused carrier with a precise diagnostic. The v7 migration/refusal follows it.

## Final review-impl

Final review-impl PASS head=4a0e09df2b2023a08090fa5d4f265b7463be6912

## Claimed parse impact

What Must Be Done

## Scope Expansion

- **Multi-object entry gates.** Every member's gated counters and tapped/attacking riders are resolved before the first member moves, and carried through pauses (`ChangeZoneMemberEntry`). Without this, the PR's own projection let later members of one event see earlier co-entrants (CR 603.6a / CR 614.12). No printed card makes a multi-object gated move today, but leaving it would ship a wrong reading. A paused multi-object resume also projects from the paused move's own entry descriptor rather than the resolving stack root.
- **Granted statics.** The pre-entry population pin covers conditions inside statics granted by another effect, per recipient (CR 109.5). This was raised in review.
- **Persisted resolution state v7.** The carrier change required versioning the saved-game boundary: v1–v6 migration or refusal (the v1 carrier inside the v1 reader arm), plus reader regressions. Protocol moves to 155/137.

## Validation Failures

Follow-ups, moving to the C1 PR (CR 611.2e onset):
- The instruction-boundary deferral (`entry_observation_deferred`) still treats every `GenericEffect` / `Animate` / `ReturnAsAura` / `ApplyPerpetual` later in the chain as an "is/has" rider, because the AST doesn't record the rider's onset. Reason: C1 adds the typed onset. Until then those chains keep today's live reading, which is no regression.
- `effect_may_apply_with_entry` and `effect_enters_battlefield` are `matches!` subset classifiers over `Effect`. Reason: they follow the file's ~50 sibling classifiers, and C1 replaces the former. C1 will convert or justify both.

Residuals (review LOW, no wrong behavior):
- An entrant gated on both counters and tapped/attacking builds two projections. No printed card carries both gates.
- `pinned_static_condition` encodes true/false with `StaticCondition::None` / `Not { None }` (scratch-only).
- `overlay_source_projection` copies `TriggerSourceContext` field by field.
- `EntryGateContext::from_change_zone` maps an already-surfaced `enters_under` error to "no projection".
- `is_anaphoric_entry_counters_clause` repeats the parser's stripper order.
- `settle_entries_before_instruction` runs its deferral check before knowing whether an entry is pending.
- The deferral predicate's `TopCardContestRoundComplete` arm (added when merging main) has no dedicated test. It only makes deferral more conservative and is shadowed by the per-seat deferral; no printed card is affected.
- The entry-gate projection doesn't apply a face-down profile. This is pre-existing: base read the face-up card too, and no printed card puts a gated rider on a face-down move.

Declined review suggestions:
- CodeRabbit's first parser-test nitpicks, as originally worded. A `TargetMatchesFilter` assertion and a not-`Unimplemented` guard both fail on the current parse: the later `PutCounter` carries `RevealedHasCardType`, and the token rider is `Unimplemented`. Neither shape comes from this PR. Its later re-review asked for the condition the parser emits today, and that positive assertion is now in `put_counter_if_its_a_type_stays_later_instruction`.
- CodeRabbit's `is_permanent_type` filter on the any-of gate. It would drop Kindred (CR 110.4), and a non-permanent leg can never match a battlefield entrant.

Known, outside this PR (to be filed separately):
- The intervening-if recheck (CR 603.4) and evolve's comparison (CR 702.100a) still read the live object.
- Hunting Wilds' unkicked "become 3/3" behavior.
- Semester's End ("Each of them enters with … if it's a creature") isn't covered by the anaphoric rider grammar.

## CI Failures

None.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EJVTJ7tgdqKRAN45icTrEE
