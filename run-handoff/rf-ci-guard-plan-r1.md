## Plan: PR #10036, round fixes (A) v1 change-zone migration moves to the single v1-reader boundary, and (B) a reach guard for `put_counter_if_its_a_type_stays_later_instruction`

BASE = START = `3012091c424380b33b0b569767431961be10c574` (worktree `/mnt/data/dev/wt/etb`, branch `engine/etb-entry-snapshot`). The plan is read-only. The probe worktree `/mnt/data/dev/wt/plan6-probe` was created and then removed. The probe target was `/mnt/data/targets/review`. No other worktree or target dir was touched. The untracked `RESUME.md` and `data/mtgjson/{decks,sets}` in `/mnt/data/dev/wt/etb` were there before this run and are not part of this work.

### Step 0: Premise
- **Item (A)** is a persistence-boundary refactor that keeps behavior the same. No card's rules behavior changes.
- **Item (B)** hardens a parser test. Its fixture string is synthetic ("Return target creature card from your graveyard to the battlefield. Put two +1/+1 counters on it if it's a creature.") and is not a card's Oracle text.
- No card premise is involved, so no Scryfall check was needed. Elemental Bond's behavior is untouched in this round.
- Both CI findings were reproduced at BASE:
  - `./scripts/check-resolution-frame-boundaries.sh` at BASE exits 1 with `crates/engine/src/types/resolution.rs:4281: legacy resolution input key 'pending_change_zone_iteration' is outside the v1 reader or a fixture`.
  - The test at `crates/engine/src/parser/oracle_effect/tests.rs` (`put_counter_if_its_a_type_stays_later_instruction`) has only the negative `!matches!(sub.condition, Some(ZoneChangedThisWay{..}))`.

### Step 1: Skills
- `/casting-stack-conditions` covers the resolution-state persistence boundary (`ResolutionStateWire`).
- `/oracle-parser` is consulted for the item (B) test conventions only. No parser code changes.
- `/card-test` foot-gun #6 (vacuous negative assertions) is exactly what item (B) fixes.
- `/add-engine-variant`: N/A, because no variant is added.

### Step 2: Analogous trace
I traced how the v1 reader arm already consumes every other v1 key. For example, `pending_repeat_iteration` and `draw_sequences` go `ResolutionStateWire::from_value` (`crates/engine/src/types/resolution.rs`) → decode-mode dispatch → `GameStateDecode::prepare_resolution_wire` (`crates/engine/src/types/game_state.rs`) → `migrate_legacy_zone_change_trigger_provenance` (game_state.rs) → `GameStateDecodeMode::ResolutionWireV1 => { ... }` arm. That arm:
- reads `draw_sequences` for `validate_draw_sequence_delivery_owner_fields`;
- runs `reconcile_persisted_zone_change_occurrences`;
- refuses a v1 payload that contains `resolution_frames` / `resolution_stack`;
- runs the `Legacy*Wire::from_value` decodes, the `legacy_object.remove(..)` list, `materialize_prepared` and the `push_*` frame installs.

`draw_sequences` is the direct precedent: a v1 key that is read and validated inside the arm before its `Legacy*Wire` decode.

I also traced the guard (`scripts/check-resolution-frame-boundaries.sh`):
- It scans **string literals only**, so comments are not flagged.
- It allows the 39 keys in:
  - the block span of the first regex match `\bGameStateDecodeMode::ResolutionWireV1\s*=>\s*\{`, which is the single main v1 arm. `grep -c` finds 1 occurrence in resolution.rs. The `=> GameStateDecodeMode::ResolutionWireV1,` and `ResolutionWireV1 => LEGACY_LIVE_ZONE_CHANGED_EVENT_ROOTS,` lines do not match, because they have no `{`;
  - `legacy_resolution_wire_fields`;
  - the `LEGACY_LIVE_ZONE_CHANGED_EVENT_ROOTS` census;
  - `struct Legacy\w+Wire` bodies;
  - `#[cfg(test)]` modules;
  - `migrate_legacy_trigger_firing_carriers` in game_state.rs.

### Step 3: Files read
- `crates/engine/src/types/resolution.rs`: the version constants near `LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION`, `reject_ambiguous_legacy_paused_draw_result`, `LEGACY_CHANGE_ZONE_ENTRY_FIELDS`, `LegacyChangeZoneMembers`, `LegacyChangeZoneEntrySpec`, `migrate_legacy_change_zone_carrier`, `migrate_legacy_change_zone_member_entries`, `LEGACY_LIVE_ZONE_CHANGED_EVENT_ROOTS`, `ResolutionStateWire::from_value` (whole), `LegacyChangeZoneWire`.
- `crates/engine/src/types/game_state.rs`: `prepare_resolution_wire` and each migration it calls (`migrate_legacy_tap_effects`, `migrate_legacy_exploited_trigger_roles`, `reject_legacy_exploit_event_evidence`, `migrate_legacy_trigger_firing_carriers`), `migrate_legacy_zone_change_trigger_provenance`, `visit_persisted_live_zone_changed_records`, `serialized_zone_changed_record_mut`, `reconcile_persisted_zone_change_occurrences` (head), `decode_persisted_resolution_state`, and the bare-id test comment near the `ChangeZoneMemberEntry` rejection.
- `crates/engine/src/game/effects/change_zone.rs`: test helpers (`both_wires`, `downgrade_change_zone_carrier`, `legacy_change_zone_carrier`, `change_zone_carrier_mut`) and the three named tests.
- `crates/engine/src/parser/oracle_effect/tests.rs`: the named test, its neighbours, and the existing `RevealedHasCardType` equality style. The `RevealTop` test around line 31063 asserts `sub.condition == Some(AbilityCondition::RevealedHasCardType { card_types: vec![CoreType::Land], additional_filter: None, subtype_filter: None })`. `CoreType` is already imported (line 25).
- `crates/engine/src/types/ability.rs`: the `AbilityCondition::RevealedHasCardType { card_types: Vec<CoreType>, additional_filter: Option<FilterProp>, subtype_filter: Option<Box<TargetFilter>> }` shape.

### Step 3.5: Probes (all compiled and ran to a clean exit in the throwaway worktree at 3012091c4)

**P1. Guard on the proposed placement.** After applying the exact edits in Step 5 (A1/A2), `./scripts/check-resolution-frame-boundaries.sh` printed `Resolution-frame boundary guard PASS`, rc=0.
- Decision this settled: an arm-top placement inside the existing v1 arm satisfies the guard with no change to the guard or its allowlist.
- REVERT control: the same script at BASE (the old placement) exits 1 and flags `resolution.rs:4281 'pending_change_zone_iteration'`.

**P2. Behavior with the proposed edits** (A1 + A2 + B1 applied), running `cargo test -p phase-engine --lib -- put_counter_if_its_a_type_stays_later_instruction legacy_ungated_parked_change_zone_migrates_through_both_readers legacy_gated_parked_change_zone_refuses_with_the_exact_diagnostic unversioned_raw_parked_change_zone_declares_its_member_shape`: **4 passed, 0 failed**.
- This includes every v1 row in both tests. Those tests have their own reach guards: the ungated restore equals the current restore and then resumes; the gated rows assert the exact constant and the ungated sibling restores.
- It also confirms the exact parser shape for (B): `Some(AbilityCondition::RevealedHasCardType { card_types: vec![CoreType::Creature], additional_filter: None, subtype_filter: None })`. The test used `assert_eq!` on that full value.

**P3. REVERT of the moved call.** I removed only the new in-arm block (A2) and kept A1, so v2–v6 still migrate. Both change_zone tests then FAIL on their v1 rows:
- the ungated test with `raw v1: legacy restore: invalid type: integer `2`, expected struct ChangeZoneMemberEntry`;
- the gated test with `raw v1 conditional counters: the exact diagnostic` mismatch.

This shows that the in-arm call is load-bearing and that A1's range correctly excludes v1. Build+run took about 7 minutes (rebuild only, `CARGO_INCREMENTAL=0`).

**Not probed (argued from code, labelled):**
- Multi-defect error precedence: a v1 payload that both carries a gated or malformed carrier and fails one of `prepare_resolution_wire` or the provenance migration. No existing fixture has that shape.
- The commutation argument in the Identity / Provenance section below.

### Step 4: Architectural sections

**Pattern Coverage.**
- Item (A) covers every v1 save paused inside a multi-object ChangeZone: the whole class of `pending_change_zone_iteration` carriers (ungated ones migrate, gated ones refuse). The v2–v6 typed-frame carriers stay on their current path.
- Item (B) guards the parser class "later instruction gated by an anaphoric type check after a battlefield move" against silently dropping the clause.
- No card-count change, and no coverage-status impact.

**Sizing.**
- Units: **1**. Unit A: "the v1 parked-ChangeZone carrier migration runs inside the single v1-reader arm." It uses one checklist pass and two `.rs` code sites in one file.
  - Registration surfaces: `migrate_legacy_change_zone_member_entries` (version range) and the `GameStateDecodeMode::ResolutionWireV1 => {` arm.
  - Discriminating tests: the guard script, plus the v1 rows of `legacy_ungated_parked_change_zone_migrates_through_both_readers` and `legacy_gated_parked_change_zone_refuses_with_the_exact_diagnostic`.
- Item (B) is a test-only assertion with no behavior and no registration surface, so I count it as a rider, not a unit. If the orchestrator counts it as a second unit, T1 fires but T2 does not (4 < 13), so the phase-fit gate still does not fire. It would only decide whether the small-change lane applies.
- No dependency edges: A and B are independent.
- Expected counted scope paths: **4** (or 3 if the test-only `oracle_effect/tests.rs` is treated as a fixture; counted conservatively here).
- **New variant: none. `WaitingFor` / `GameAction`: unchanged. Serialized surface:** the wire shape and `RESOLUTION_STATE_WIRE_VERSION` are unchanged, and every input decodes to the same result or the same refusal. The only difference is which refusal message wins when one payload carries several defects (see below). No protocol bump.
- **Verdict: one unit, ≤4 counted paths, no new variant, no serialized-shape and no WaitingFor/GameAction change. The small-change lane applies** (subject to the orchestrator's reading of "serialized surface" for a reordered decode step that preserves semantics, flagged here for transparency).

**Building Blocks.** The plan reuses `migrate_legacy_change_zone_carrier` unchanged; it stays the single per-carrier authority for the migrate/refuse decision. It also reuses `migrate_legacy_change_zone_member_entries` for v2–v6 frames, `RangeInclusive::contains` over the existing version constants (`LEGACY_TYPED_FRAME_RESOLUTION_STATE_WIRE_VERSION ..= LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION`), and `Value::get_mut(..).filter(..)`. No new helper.

**Logic Placement.** Everything stays in the engine's persistence boundary (`types/resolution.rs`).
- The v1 key literal moves into the v1 reader arm. This is the "single v1-reader boundary" the maintainer asked for, and the same arm already consumes the other 38 v1 keys.
- The v2–v6 frame walk stays where it is: before `prepare_resolution_wire`, after `reject_ambiguous_legacy_paused_draw_result`. Its literals (`resolution_frames`, `ChangeZone`, `pending`) are not v1 keys.

**Rust Idioms.**
- A range `contains` instead of two `if` early-returns.
- An `if let Some(carrier) = value.get_mut(..).filter(|carrier| !carrier.is_null())` in the arm, the same filter shape as the existing frame loop.
- No bools and no wildcards.

**Nom Compliance.** No file under `crates/engine/src/parser/` changes except the test module, where no parser code changes. N/A.

**Extension vs Creation.** This extends the v1 arm, which already consumes v1 keys. No new pattern.

**Analogous Trace.** `draw_sequences` / `pending_repeat_iteration`: `ResolutionStateWire::from_value` → `GameStateDecode::prepare_resolution_wire` → `migrate_legacy_zone_change_trigger_provenance` → `GameStateDecodeMode::ResolutionWireV1 => {` arm (`validate_draw_sequence_delivery_owner_fields` over `draw_sequences`, `Legacy*Wire::from_value`, `legacy_object.remove`, `materialize_prepared`, `push_*`), all in `crates/engine/src/types/resolution.rs` + `crates/engine/src/types/game_state.rs`.

**Variant Discoverability.** N/A (no variant).

**Identity / Provenance Contract (ordering and semantics of item A).**

Current v1 order:
1. version → decode_mode
2. (`validate_draw_sequence_delivery_owner_fields` is skipped for v1)
3. `reject_ambiguous_legacy_paused_draw_result` (a no-op for v1; it matches only versions 2 and 3)
4. **CZ migration (v1 top-level carrier)**
5. `prepare_resolution_wire`
6. `migrate_legacy_zone_change_trigger_provenance`
7. arm: v1 delivery-owner validation → `reconcile_persisted_zone_change_occurrences` → `resolution_frames` / `resolution_stack` refusals → `Legacy*Wire::from_value` (… `LegacyChangeZoneWire` …)

New v1 order: steps 1, 2, 3, 5 and 6 unchanged, then the arm with **the CZ migration as the arm's first statement**, followed by every arm step in its current order.

Kept exactly:
- The migration still follows `reject_ambiguous_legacy_paused_draw_result`.
- It still precedes every in-arm validation: the v1 delivery-owner check, reconcile, the `resolution_frames` / `resolution_stack` refusals, and every `Legacy*Wire` decode including `LegacyChangeZoneWire::from_value`.
- A v1 payload's bogus `resolution_frames` is still never walked by the frame migration. A1's range excludes v1, so the arm's "v1 resolution state must not contain resolution_frames" refusal is still the one that fires. If v1 fell through to the frame loop instead, that would change the result.
- v2–v6 are byte-for-byte unchanged in position and code path. v7 and later still skip.

What moves: `prepare_resolution_wire` and the provenance migration now run on the un-migrated v1 carrier. They commute with the CZ migration:
- No function in either names `pending_change_zone_iteration`. `grep` finds the literal only in resolution.rs (the migration, the census, `legacy_object.remove`, `LegacyChangeZoneWire`, the inventory, tests) and in game_state.rs tests.
- The recursive walkers act only on their own sub-objects:
  - `migrate_legacy_tap_effects`: objects under an `effect` key with type Tap / Untap / TapAll / UntapAll;
  - `migrate_legacy_exploited_trigger_roles`: exploited trigger definitions;
  - `reject_legacy_exploit_event_evidence`: `CreatureExploited` events;
  - `visit_persisted_live_zone_changed_records` (the carrier is a census root): `ZoneChanged` records only.
- The CZ migration touches only the five `LEGACY_CHANGE_ZONE_ENTRY_FIELDS` and `remaining`, which hold ids or the expanded member records.
- The gate contents (`conditional_enter_with_counters`, `enters_modified_if`) are read only as `IgnoredAny` / for presence and are never copied to the output. `enter_tapped`, `enters_attacking` and `enter_with_counters` match no walker.
- So for every payload a writer can produce (the gates hold a `TargetFilter` and `(TargetFilter, CounterType, QuantityExpr)` triples, which contain no shape a walker refuses), the accepted output is identical and every refused input is still refused. A hand-built payload whose inert gate holds a shape a walker refuses was accepted at BASE and is now refused. An inert gate is one the migration drops: an empty `remaining`, or `enters_modified_if` over default riders. Example shapes: `{"TurnFaceUp": 1}` (`migrate_legacy_turn_face_up_resume`), or a `CreatureExploited` object without `data` (`reject_legacy_exploit_event_evidence`). The walkers now see the gate before the migration drops it. This change is stricter, cannot be reached from any writer, and needs no test.

The one observable difference: a payload carrying **both** a gated or malformed v1 carrier **and** a defect refused by `prepare_resolution_wire` or the provenance migration now reports the latter refusal first. This is unavoidable under the single-boundary rule, because the guard anchors the allowlist on the one v1 arm, which is reached only after those two steps. It is not probed (no fixture).

**Verification Matrix.**

| Claim | Seam | Entry | Test | Revert-failing assertion | Sibling / negative and reach guard |
|---|---|---|---|---|---|
| The v1 key literal lives only inside the v1 reader | `from_value` v1 arm, `migrate_legacy_change_zone_member_entries` | `./scripts/check-resolution-frame-boundaries.sh` | guard script (cheap, run directly) | BASE placement: exit 1, `resolution.rs:4281 'pending_change_zone_iteration'` (P1 control) | Positive: PASS line printed (P1) |
| An ungated v1 carrier migrates through raw and trusted envelopes and resumes | the in-arm call → `migrate_legacy_change_zone_carrier` | `restore_persisted` → `decode_persisted_resolution_state` → `ResolutionStateWire::from_value` | `legacy_ungated_parked_change_zone_migrates_through_both_readers` (existing v1 row) | P3: `raw v1: legacy restore: invalid type: integer` | v2–v6 rows (frame path) still pass; v6 no-carrier row unchanged |
| A gated v1 carrier refuses with the exact constant; the ungated sibling and the inert gate migrate; an empty `remaining` drops the gate | same | same | `legacy_gated_parked_change_zone_refuses_with_the_exact_diagnostic` (v1 rows) | P3: `raw v1 conditional counters: the exact diagnostic` mismatch | Each refused row's de-gated sibling restores (reach); v3 and v6 rows exercise the unmoved path |
| Unversioned raw payload is unaffected (declares v3 / v7) | `declare_raw_resolution_wire` + v2–v6 path | `restore_persisted` | `unversioned_raw_parked_change_zone_declares_its_member_shape` | (unchanged code path; passes in P2) | gated v3 refusal row |
| The parser keeps the "if it's a creature" gate on the later instruction | none (test only) | `parse_effect_chain` | `put_counter_if_its_a_type_stays_later_instruction` + new `assert_eq!` | dropping the clause makes `condition` `None` and fails the `assert_eq!` | The positive `assert_eq!` is the reach guard for the existing `!matches!(.., ZoneChangedThisWay)` |

- Hostile rows: the v1 `resolution_frames`-present refusal stays reachable and first, because A1 excludes v1 from the frame walk. That is proven from code; no new test is planned (an existing refusal path, no change).
- Multi-defect precedence: described above, unprobed, no test.
- Coverage impact: none. No Oracle text acceptance changes.

**Reference Readings.** The ungated test's "migrated equals current restore" rows are parity rows. Their expected value (each member enters with the carrier's own counters/tapped/attacking, CR 614.12, with no gate verdict involved) was derived and accepted in earlier rounds (r5–r7), and this round does not change it. No new reference reading.

### Step 5: Implementation steps

**A1. `crates/engine/src/types/resolution.rs`, `migrate_legacy_change_zone_member_entries`.** Replace:
```rust
    if version > LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION {
        return Ok(());
    }
    if version == LEGACY_RESOLUTION_STATE_WIRE_VERSION {
        return match value.get_mut("pending_change_zone_iteration") {
            Some(carrier) if !carrier.is_null() => migrate_legacy_change_zone_carrier(carrier),
            _ => Ok(()),
        };
    }
```
with:
```rust
    // v1 carries its carrier at a top-level v1 key, which only the v1 reader
    // arm of `ResolutionStateWire::from_value` may read; it migrates it there.
    if !(LEGACY_TYPED_FRAME_RESOLUTION_STATE_WIRE_VERSION
        ..=LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION)
        .contains(&version)
    {
        return Ok(());
    }
```
Rewrite its doc comment, currently "The single v1–v6 → v7 authority … v7 and later decode directly.", to say it is the v2–v6 typed-frame half. The v1 top-level carrier is migrated by the v1 reader arm through the same `migrate_legacy_change_zone_carrier`, which stays the single per-carrier authority. Keep the existing `LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR` link. Do **not** quote the v1 key as a string literal anywhere outside the arm. Comments are fine, because the guard skips them.

**A2. Same file, `ResolutionStateWire::from_value`, `GameStateDecodeMode::ResolutionWireV1 => {` arm.** Insert as the first statement, before `let legacy_draw_sequence_frames = value`:
```rust
                // CR 603.6a + CR 614.12: a v1 save paused inside a multi-object
                // ChangeZone wrote bare member ids with carrier-level entry
                // modifications. Migrate (or refuse, when a gate decides the
                // entries) before this arm's validation or the `LegacyChangeZoneWire`
                // decode reads it. Unlike the v2–v6 frame migration, this runs after
                // `prepare_resolution_wire` and the provenance migration, neither of
                // which reads the carrier's `remaining` or entry fields.
                if let Some(carrier) = value
                    .get_mut("pending_change_zone_iteration")
                    .filter(|carrier| !carrier.is_null())
                {
                    migrate_legacy_change_zone_carrier(carrier)?;
                }
```
(CR 603.6a and CR 614.12 are verified in `docs/MagicCompRules.txt` and already used by the adjacent code.)

Leave the `migrate_legacy_change_zone_member_entries(&mut value, version)?;` call in `from_value` exactly where it is. Update `from_value`'s doc sentence "A v1–v6 parked ChangeZone carrier is migrated or refused first by `migrate_legacy_change_zone_member_entries`." to: "A v2–v6 parked ChangeZone frame carrier is migrated or refused first by `migrate_legacy_change_zone_member_entries`; the v1 top-level carrier, by the v1 reader arm before that arm's validation and legacy decodes."

Update the `LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION` doc's parenthetical `(`migrate_legacy_change_zone_member_entries`)` to name both: `migrate_legacy_change_zone_member_entries` for v2–v6 frames and the v1 reader arm for v1, both through `migrate_legacy_change_zone_carrier`.

**A3. `crates/engine/src/game/effects/change_zone.rs`, comment-only.** In the doc of `legacy_ungated_parked_change_zone_migrates_through_both_readers`, replace "REVERT: removing the `migrate_legacy_change_zone_member_entries` call in `ResolutionStateWire::from_value` makes every legacy row fail with serde `invalid type: integer`;" with: "REVERT: removing the `migrate_legacy_change_zone_member_entries` call in `ResolutionStateWire::from_value` makes every v2–v6 row fail, and removing the v1 reader arm's `migrate_legacy_change_zone_carrier` call makes the v1 rows fail, each with serde `invalid type: integer`;". The second half was measured in P3.

The gated test's "without the migration the error is the serde message" stays true as written. No change to the third test.

**A4. `crates/engine/src/types/game_state.rs`, comment-only.** In the test comment "A persisted v1–v6 carrier is migrated or refused before this decode by `migrate_legacy_change_zone_member_entries` (`types/resolution.rs`).", change the tail to: "… by `migrate_legacy_change_zone_carrier` (`types/resolution.rs`), reached from the v1 reader arm and from `migrate_legacy_change_zone_member_entries` for v2–v6." This avoids a false sentence for v1.

**B1. `crates/engine/src/parser/oracle_effect/tests.rs`, `put_counter_if_its_a_type_stays_later_instruction`.** Between the `PutCounter` assertion and the existing negative assertion (keep the negative unchanged), insert:
```rust
    // Reach guard: the "if it's a creature" clause survives as the later
    // instruction's type gate, so the negative check below cannot pass by the
    // clause being dropped.
    assert_eq!(
        sub.condition,
        Some(AbilityCondition::RevealedHasCardType {
            card_types: vec![CoreType::Creature],
            additional_filter: None,
            subtype_filter: None,
        }),
        "{sub:#?}"
    );
```
`CoreType` and `AbilityCondition` are already in scope. The shape was verified in P2. No parser change.

**Verification**
- Run directly:
  - `cargo fmt --all`;
  - `./scripts/check-resolution-frame-boundaries.sh`, which must print PASS. The REVERT note: BASE fails at the old placement, P1 control.
- Deferred to the final combined-candidate batch, after the upstream merge (`test-engine`):
  - the four tests above;
  - the full change_zone / resolution persistence suites;
  - clippy.
- The planned upstream merge (protocol 151/133 → ours 152/134, the `effects/mod.rs` ordering conflict) is out of scope. Nothing here touches protocol surfaces or `effects/mod.rs`.

### SCOPE_PATHS
```
crates/engine/src/types/resolution.rs
crates/engine/src/game/effects/change_zone.rs
crates/engine/src/types/game_state.rs
crates/engine/src/parser/oracle_effect/tests.rs
```
- `change_zone.rs` and `game_state.rs` get comment-only edits.
- `scripts/check-resolution-frame-boundaries.sh` is **not** in scope and is not edited.

### Lane
Sizing gives 1 unit, 4 counted paths (3 if the test module counts as a fixture), no new variant, no `WaitingFor` / `GameAction` change, and no wire-shape or version change. **The small-change lane applies.** The one judgment for the orchestrator is whether a reordered decode step that keeps the same semantics counts as a "serialized surface" change.

Probe artifacts: the probe log is `/tmp/claude-1000/-mnt-data-dev-phase-personal--claude-worktrees-bridge-cse-01WLHDBFt86Lo3sv4YVbytHb/329270ab-229a-55c0-99a0-1f7c602b125a/scratchpad/probe1.log`.