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
- The network protocol moves to 150 (wire 132).

**Scope:** the onset deferral for "gains/gets/becomes" riders (CR 611.2e, the "C1" CodeRabbit item) moves to a separate follow-up PR. This PR keeps the existing conservative deferral.

The Zimone "create a token, then put N counters on it" lowering is untouched; that fix is in #10004.

## Files changed

- `crates/engine/src/game/filter.rs`: `BattlefieldEntrantProjection` (CR 614.12 projection with the pre-entry population pin), `matches_target_filter_on_entry_observation`, incarnation-checked fallback, keyword-kind reads through `HeldObject`
- `crates/engine/src/game/layers.rs`: `pin_entry_population_conditions`, including per-recipient pinning of `GrantStaticAbility` conditions
- `crates/engine/src/game/keywords.rs`: `HeldObject { Live, Detached }`, `effective_keyword_kind_of`
- `crates/engine/src/game/effects/change_zone.rs`: `EntryGateContext`, `ChangeZoneEntrySpec`, `resolve_change_zone_member_entries`
- `crates/engine/src/game/effects/mod.rs`: instruction-boundary settlement and its deferral predicate; the paused ChangeZone drain applies carried member entries
- `crates/engine/src/game/engine_resolution_choices.rs`: the EffectZoneChoice and PayCost resumes precompute member entries
- `crates/engine/src/game/effects/search_outside_game.rs`, `crates/engine/src/game/elimination.rs`: carrier retype
- `crates/engine/src/game/zones.rs`: `settle_battlefield_entry_observations`
- `crates/engine/src/game/game_object.rs`: `overlay_source_projection`
- `crates/engine/src/game/trigger_matchers.rs`: enters observers read the observation
- `crates/engine/src/types/game_state.rs`: `battlefield_entry_observations`; `ChangeZoneMemberEntry`
- `crates/engine/src/types/resolution.rs`: resolution state v7; v1–v6 carrier migration/refusal (`migrate_legacy_change_zone_member_entries`); unversioned v7 inference
- `crates/engine/src/game/turns.rs`, `crates/engine/src/analysis/resource.rs`, `crates/engine/src/game/visibility.rs`: clear or strip the table
- `crates/engine/src/parser/oracle_effect/{mod,conditions,sequence,tests}.rs`: "it enters with … if it's a [type] (or [type])" as a conditional entry counter
- `crates/engine/tests/integration/conditional_entry_gate_projection.rs` (new), `etb_trigger_entry_time_characteristics.rs` (new), `main.rs`, `exploit_object_filter.rs` and `resolution_progress_contract.rs` (v7 pins)
- `crates/engine/tests/fixtures/cr733/authority_matrix.json.gz`, `blocked_write_sites.json.gz`: register the new field
- `scripts/post-replacement-continuation-baseline.txt`
- Protocol bump to full 150 / wire 132, after main's 148/130 and 149/131:
  - `crates/lobby-broker/src/protocol.rs`
  - `crates/server-core/src/protocol.rs`
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

Batched verification ran in a clean worktree at e16fe3b6f, the merge with upstream main 39fb1eb74:
- `cargo fmt --all -- --check`, `./scripts/check-parser-combinators.sh`, `node scripts/check-protocol-version.mjs`, `./scripts/check-interaction-bindings.sh --check`, `./scripts/gen-card-data.sh`: all clean
- `cargo clippy --locked --workspace --exclude phase-tauri --all-targets --features engine/proptest -- -D warnings`: clean
- `cargo test --locked --workspace --exclude phase-tauri --features engine/proptest --no-fail-fast`: 41380 passed, 1 failed. The failure was `types::resolution::tests::the_dealerless_version_still_decodes_and_the_next_one_is_refused`, which pinned the pre-v7 version list. It was fixed in 3012091c4; on the head, `cargo test -p phase-engine --lib -- types::resolution change_zone::tests` gives 165 passed, 0 failed. Every other result carries over from e16fe3b6f, because 3012091c4 changes only that test assertion.
- `pnpm run type-check`: clean. `pnpm lint`: 0 errors. `vitest run`: 641 files, 9827 passed.
- Parser measurement (`oracle-gen` + `coverage-report` + `coverage-parse-diff`): base 39fb1eb74 vs head, same pinned MTGJSON. Only What Must Be Done changes, 2 signatures; supported 31946 → 31947, 0 regressions.
- `oracle-gen semantic-audit`: 208 findings at base and at head, identical, 0 new.
- Revert discrimination:
  - The Match the Odds, Fractal Harness, Welcoming Vampire, Miraculous Recovery, library-search resume, hand-put, living weapon and departed-token rows fail with the matcher routed back to the live object.
  - The Heliod four/five-devotion rows fail without the population pin.
  - The granted Shrine/Starfield rows fail with the grant arm removed or keyed to the host's controller.
  - The multi-object metalcraft (Ornithopter + Rusted Relic), co-entering March of the Machines and paused-drain rows fail when gates are resolved after earlier members moved.
  - The persisted-reader rows (raw and trusted envelopes, v1–v6) fail without the v7 migration or the gated refusal.
- Pre-push hook passed on push: workspace clippy, card-data validate, parser tests, phase-ai lib, oracle-gen, coverage regression check, client lint and type-check.

## Gate A

Gate A PASS head=3012091c424380b33b0b569767431961be10c574 base=51adf55688e0a8dff26f50864e812dddfd87d988

## Anchored on

- crates/engine/src/game/filter.rs:5156 — `matches_target_filter_on_battlefield_entry`: the existing CR 614.12 "as it would exist on the battlefield" read for static entry replacements. The projection extends it to effect-carried entry gates.
- crates/engine/src/game/filter.rs:5486 — `matches_target_filter_on_lki_snapshot`: the existing filter evaluation on a recorded snapshot, which the entry-observation evaluator follows.
- crates/engine/src/types/game_state.rs:26425 — `push_devour_change_zone_snapshot`: the existing capture-once-before-any-member-moves pattern for a ChangeZone event, carried through pauses. Pre-event member entries follow it.
- crates/engine/src/types/resolution.rs:4094 — `reject_ambiguous_legacy_paused_draw_result`: the existing version-keyed refusal of a legacy paused carrier with a precise diagnostic. The v7 migration/refusal follows it.

## Final review-impl

Final review-impl PASS head=3012091c424380b33b0b569767431961be10c574

## Claimed parse impact

What Must Be Done

## Scope Expansion

- **Multi-object entry gates.** Every member's gated counters and tapped/attacking riders are resolved before the first member moves, and carried through pauses (`ChangeZoneMemberEntry`). Without this, the PR's own projection let later members of one event see earlier co-entrants (CR 603.6a / CR 614.12). No printed card makes a multi-object gated move today, but leaving it would ship a wrong reading. A paused multi-object resume also projects from the paused move's own entry descriptor rather than the resolving stack root.
- **Granted statics.** The pre-entry population pin covers conditions inside statics granted by another effect, per recipient (CR 109.5). This was raised in review.
- **Persisted resolution state v7.** The carrier change required versioning the saved-game boundary: v1–v6 migration or refusal, plus reader regressions. Protocol moves to 150/132.

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
- CodeRabbit's two parser-test nitpicks. On the current parse the suggested assertions fail: the later `PutCounter` carries `RevealedHasCardType`, and the token rider is `Unimplemented`. Neither shape comes from this PR, and the tests carry positive reach guards.
- CodeRabbit's `is_permanent_type` filter on the any-of gate. It would drop Kindred (CR 110.4), and a non-permanent leg can never match a battlefield entrant.

Known, outside this PR (to be filed separately):
- The intervening-if recheck (CR 603.4) and evolve's comparison (CR 702.100a) still read the live object.
- Hunting Wilds' unkicked "become 3/3" behavior.
- Semester's End ("Each of them enters with … if it's a creature") isn't covered by the anaphoric rider grammar.

## CI Failures

None.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EJVTJ7tgdqKRAN45icTrEE
