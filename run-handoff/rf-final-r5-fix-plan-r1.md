# Plan: version the parked ChangeZone member-entry carrier (persisted v7) and add an optional Starfield-grant row

BASE_SHA = START_SHA = 1097f3e7f17f68dd54c97c8812bfc93d1b6e0223. IMPLEMENTATION_WORKTREE = /mnt/data/dev/wt/etb. Budget: standard.

Under the updated engine-implementer (eddee97), build-backed checks are deferred and batched at the final combined candidate, after the planned upstream merge (0a853dbb3) and the protocol reallocation. This round's executor writes the code and the tests, then runs only `cargo fmt --all`.

## Step 0: Premise (checked)

**The finding is real and has not been fixed.** I re-probed it at 1097f3e. The probe used a real cast-driven paused board: a three-target graveyard→battlefield ChangeZone parked on a shock-style `MayCost` replacement. The resolution stack held exactly one frame, `[ChangeZone]`, and the resolving spell stack entry was present.

- **Current writer:** declares `resolution_state_version` 6 on both the Raw and the trusted wire.
- **Current round-trip:** passes through `PersistedGameState` (Raw and Trusted) → `prepare_for_restore(Immediate)` → `finalize_immediately`. Declining the replacement afterwards applies each member's carried entry: tapped, plus two +1/+1 counters.
- **Downgraded carrier:** rewriting the same wire's carrier to the historical shape (`remaining` as bare ids, plus carrier-level `enters_attacking`, `enter_tapped: true`, `enter_with_counters: [["P1P1",2]]`) fails on both readers with `Err("invalid type: integer `2`, expected struct ChangeZoneMemberEntry")`. The same failure appears on a v1 projection (carrier at top-level `pending_change_zone_iteration`, `resolution_frames` removed, version 1).
- **Empty `remaining`:** a legacy carrier with an empty `remaining` restores Ok on both readers, at v6 and at v1.

No version-aware transformation or refusal runs before the carrier decode. On the traced paths, `ResolutionStateWire::from_value` (types/resolution.rs) goes from the version `match` straight to `LegacyChangeZoneWire::from_value` (v1) or `serde_json::from_value::<ResolutionStack>` (v2–v6).

**The historical carrier shape is confirmed from base 51adf5568** (`git diff 51adf5568 HEAD -- crates/engine/src/types/game_state.rs`):

| Field | Old serde shape |
|---|---|
| `remaining` | `Vec<ObjectId>`, required |
| `enter_tapped` | `#[serde(default, with = "etb_tap_bool_compat", skip_serializing_if = is_unspecified)]`, so it was written as a JSON bool |
| `enters_attacking` | `bool`, required, always written |
| `enter_with_counters` | `Vec<(CounterType,u32)>`, default / skip-empty |
| `conditional_enter_with_counters` | `Vec<(TargetFilter,CounterType,QuantityExpr)>`, default / skip-empty |
| `enters_modified_if` | `Option<TargetFilter>`, default / skip-none |

**How the old resume read a member** (base `effects/mod.rs::drain_pending_change_zone_iteration` + `change_zone.rs::process_one_zone_move_with_terminal`): for each id it computed `enter_with_counters_for_pending_object(state, source, id, enter_with_counters, conditional)`. That function returns `base` when `conditional` is empty. The riders came from `effective_enter_mods(.., enter_tapped, enters_attacking, enters_modified_if)`, which returns the base riders when the gate is `None`. Both gates were re-read against the live board at resume, which is exactly the CR 603.6a / 614.12 defect this PR fixes.

**No card is premised.** This is a persistence-boundary contract. Step 0 for the secondary unit: Starfield of Nyx and Sanctum of Tranquil Light Oracle text is already verbatim in the test file's constants; the grant host is explicitly synthetic.

## Step 1: Applicable skills

- `/casting-stack-conditions` covers resolution-stack and persistence wire changes. Checklist items that apply: the persisted wire version, the decode path for every historical version, and validation before the typed decode.
- `/card-test` applies to Unit B, which uses the existing cast recipe `cast_release_juno`.
- `/add-engine-variant`: no enum variant is added. The new types are private serde boundary structs and one private boundary enum used for detection only; none is an engine AST enum. Not triggered.
- No parser files change, so `/oracle-parser` and the Nom Compliance section do not apply.

## Step 2: Analogous trace

**Traced the v3→v4 draw `delivery_owner` versioning.** Each layer has an analogue here:

| Layer | Delivery-owner precedent | This plan's analogue |
|---|---|---|
| Version constants | `LEGACY_RESULT_OWNERSHIP_RESOLUTION_STATE_WIRE_VERSION`, `LEGACY_DEALERLESS_RESOLUTION_STATE_WIRE_VERSION` | the new v6 const and the bump to 7 |
| Version-keyed validation | `validate_draw_sequence_delivery_owner_fields(value, version, ..)` | the version-keyed migrate/refuse pass |
| Typed boundary probe | `DrawSequenceBoundaryFrame` / `ResolutionBoundaryStack` (serde types used only to inspect shape) | a ChangeZone member boundary |
| Unversioned inference | `declare_raw_resolution_wire` declares 4 when `has_draw_sequence_delivery_owner(resolution_stack)`, else 3 | a member-record → v7 inference |
| Refusal string | `reject_ambiguous_legacy_paused_draw_result` returns a precise `legacy resolution_state_version {v} cannot safely restore …` | the new gated-carrier diagnostic |

**Trace path:**
- `types/resolution.rs`: `RESOLUTION_STATE_WIRE_VERSION` → `ResolutionStateWire::to_value` (writer) → `ResolutionStateWire::from_value` (version `match` → `validate_draw_sequence_delivery_owner_fields` → `reject_ambiguous_legacy_paused_draw_result` → `GameStateDecode::prepare_resolution_wire` → V1 branch `LegacyChangeZoneWire::from_value` / typed branch `serde_json::from_value::<ResolutionStack>`) → `declare_raw_resolution_wire`.
- `types/game_state.rs`: `GameStateDecode::decode_persisted_resolution_state` (PersistedRaw → `declare_raw_resolution_wire`; TrustedEnvelope passes through) → `impl Deserialize for TrustedGameStateEnvelope` / `impl Deserialize for PersistedGameState` → `PersistedGameState::prepare_for_restore` → `PreparedPersistedGameState::finalize_*`.

**Production entry points traced:**
- `crates/engine-wasm/src/lib.rs::prepare_restored_game_state`: `serde_json::from_value::<PersistedGameState>`, error wrapped as `Failed to deserialize GameState: {error}`.
- `crates/server-core/src/persist.rs::PersistedSession.state: PersistedGameState`, loaded by `crates/phase-server/src/persistence.rs` `load_active_full_sessions`: logs `Failed to deserialize Full session row {code}: {error}` and skips the row.
- `crates/phase-ai/src/saved_state.rs::load_saved_game_state` (through `PersistedGameState`).

All of them funnel into `ResolutionStateWire::from_value`, so one fix there covers every reader. No adapter changes.

**House pattern for reader-level regressions:** `legacy_choose_dungeon_prompt_migrates_through_both_envelopes` (game_state.rs mod tests). It serializes `PersistedGameState::Raw` and `PersistedGameState::capture`, downgrades the JSON, and restores through `serde_json::from_value::<PersistedGameState>`.

## Step 3: Files read

- `types/resolution.rs`: wire constants (~3798–3823); `declare_raw_resolution_wire`; `ResolutionStateWire::{to_value, from_value}`; `LegacyChangeZoneWire`; `ChangeZoneFrame` / `ResolutionFrame` serde (`#[serde(tag="type", content="data")]`, so a frame is `{"type":"ChangeZone","data":{"pending":{..}}}`); tests `resolution_wire_contract_pins_v5_and_the_legacy_readers`, `restore_v1_change_zone_fixture`.
- `types/game_state.rs`: `PendingChangeZoneIteration`, `ChangeZoneMemberEntry`, the trusted/raw readers, `prepare_for_restore`, and test `change_zone_iteration_modern_shape_roundtrips` (bare-id carrier rejection at :45258–45269).
- `game/effects/change_zone.rs`: `resolve_change_zone_member_entry`, `enter_with_counters_for_entry`, `effective_enter_mods`, and the test helpers `add_graveyard_rusted_relic`, `add_graveyard_ornithopter`, `set_core_types`, `plus_one_counters`, `parked_change_zone_iteration`, `add_shock_in_graveyard_for_test`, `typed`.
- `game/effects/mod.rs`: drain at base and at HEAD.
- `types/zones.rs::etb_tap_bool_compat`.
- `engine-wasm/src/lib.rs` restore and the `LEGACY_CASTING_VARIANT_FACE_RESTORE_ERROR` precedent.
- `phase-server/src/persistence.rs` :770–805.
- `tests/integration/conditional_entry_gate_projection.rs` (whole file), `exploit_object_filter.rs` :155–170, `resolution_progress_contract.rs` :110–125.
- **Fixture census:** a script scanned every committed `crates/**/fixtures/**/*.json*`. No committed fixture carries a parked ChangeZone carrier with a non-empty `remaining`. The only ChangeZone frame found is a Devour-only frame (`pending: null`) in `issue_8024_devour_rest_turn10.json.gz`.

## Step 3.5: Probes

Probes ran in a throwaway worktree at 1097f3e with target `/mnt/data/targets/review`; the worktree has been removed.

- **P1, synthetic Draw-root board** (`set_draw_root`): unusable for persistence. Both readers refuse it with `resolving triggered entry has no firing carrier`. **Design impact:** the regression must use a cast-driven board, not `set_draw_root`.
- **P2, cast-driven board with `enter_tapped: Tapped` on the move:** the shock-style `MayCost` does not pause (the shock enters tapped with no prompt). **Design impact:** a Tapped member entry cannot be produced by a pausing real move in this fixture. The fixture therefore sets Tapped on the parked members through `replace_active_change_zone_iteration`, labelled synthetic.
- **P3, cast-driven board** (`enters_attacking: true`, `enter_with_counters: +1/+1 ×2`):
  - Shock first: pauses on `ReplacementChoice`, frames `[ChangeZone]`, `remaining` = 2 member records.
  - Shock last: `remaining` = `[]`.
  - The current round-trip restores Ok through both readers, with an identical parked carrier.
  - Declining applies Tapped and 2 counters to each remaining member.
  - Outside combat, `combat` is `None`, so attacking is not observable at resolution. The carrier equality covers it.
  - `CounterType::Plus1Plus1` serializes as `"P1P1"`.
  - Legacy v6 downgrade: Err (integer `2`) on both readers. Shock last: Ok.
- **P4, v1 projection of the same board:**
  - Current-shaped carrier: Ok, and `GameState` is equal to the current-wire restore on both readers. **Design impact:** whole-state equality is a valid faithful-migration oracle for v1 and v6.
  - Legacy-shaped carrier: Err when `remaining` is non-empty, Ok when empty.
- **P5, unversioned raw payload** (derived `serde_json::to_value(&GameState)`): `declare_raw_resolution_wire` declares **v3** for a board whose stack carries member records. It decoded Ok on the cast board at 1097f3e. **Design impact:** once v1–v6 are migrated, a current-writer unversioned payload (from phase-ai `saved_state` or a bare-object WASM restore) carrying member records would be misread as legacy unless the inference also dates member records to v7. Hence Step 5.3.
- **P6, grant host parse:** `Each enchantment has "As long as you control five or more enchantments, this enchantment is a creature in addition to its other types and has base power and base toughness each equal to its mana value."` parses to `GrantStaticAbility { definition: { affected: SelfRef, modifications: [AddType Creature, SetPowerDynamic ObjectManaValue(Recipient), SetToughnessDynamic ObjectManaValue(Recipient)], condition: QuantityComparison(ObjectCount(Enchantment, controller You, InZone Battlefield) GE 5) } }`.

**Not probed:**
- Exact equality of the new diagnostic string after the Trusted path's two `serde::de::Error::custom` hops. Evidence it holds: the `reject_viewer_projection_as_authority` doc states those hops "add no text of their own", and P3/P4 errors arrive with no added text.
- Reader behaviour for versions 2–5. They share the typed branch with v6, and the board has no draw frame, so the delivery-owner validation never fires.
- Unit B outcomes. They are derived from CR and Oracle text below, and run at batched verification.

## Contract decision: hybrid (migrate where exact, refuse where the authority is gone)

**Ungated carriers migrate exactly.** A carrier is ungated when `conditional_enter_with_counters` is empty and `enters_modified_if` is absent. Every unprocessed id becomes `ChangeZoneMemberEntry { object_id: id, enter_with_counters: carrier.enter_with_counters, enter_tapped: carrier.enter_tapped, enters_attacking: carrier.enters_attacking }`. This is exactly what three things produce:
- the HEAD resolver `resolve_change_zone_member_entry`: `enter_with_counters_for_entry` returns `base` when `conditional` is empty, and `effective_enter_mods` returns the base riders for `gate: None`;
- the old v6 drain;
- an uninterrupted move under either engine.

Nothing is reconstructed or defaulted: the carrier's own fields are the full authority.

**Inert gates also migrate exactly.** If `enters_modified_if` is present but both riders are default (`enter_tapped` absent/false and `enters_attacking: false`), `effective_enter_mods` yields `(Unspecified, false)` on both a match and a mismatch. The gate's verdict cannot change any member, so this case migrates exactly as ungated.

**Gated carriers are refused with an exact, typed-constant diagnostic.** A carrier is gated when `conditional_enter_with_counters` is non-empty, or `enters_modified_if` is present with a non-default rider, and it has a non-empty `remaining`. Each member's entry is then a CR 614.12 verdict against the board before the event's first member moved (CR 603.6a: one event). That board is gone. Re-reading it at restore is the defect this PR fixes, and the maintainer forbids fabricating it.

**Gate fields on an empty `remaining` are dropped.** They were only ever consumed inside the `remaining` loop (base drain), so nothing is lost.

**Uniform refusal was rejected.** It would refuse every ordinary parked multi-object move (the issue #535 Skyshroud Claim / shock-land class, ungated and by far the common case) with no fidelity gain. The maintainer explicitly prefers migration where the authority is available.

**The migration never decodes the discarded gate payloads.** It reads `conditional_enter_with_counters` as `Vec<serde::de::IgnoredAny>` and `enters_modified_if` as `Option<IgnoredAny>`. Only their presence decides, so a historical `TargetFilter` / `QuantityExpr` shape cannot turn the precise refusal into a serde error.

**Version bump:** `RESOLUTION_STATE_WIRE_VERSION` 6 → 7. The writer declares 7. Readers for v1–v6 run the migrate/refuse pass; v7 decodes directly and strictly (bare ids fail serde, as the existing carrier test asserts).

## Pattern Coverage

The class is every persisted state paused inside a multi-object ChangeZone, all routed through `PendingChangeZoneIteration`:
- targeted `ChangeZone`, `ChangeZoneAll`, and `EffectZoneChoice` multi-card resumes (`engine_resolution_choices.rs`, `elimination.rs`, `search_outside_game.rs`);
- the pauses: shock lands, check-land reveals, copy-as-enters, Aura host choices, face-down returns, library placement.

That covers any card returning, searching for, or putting two or more permanents at once where one member raises a per-permanent replacement choice: hundreds of cards, every v1–v6 save of one, and both public readers plus all three production loaders. Unit B is a test-only coverage row for the granted-static population pin (a class already fixed in 92a98bcc1).

## Sizing

| Unit | Description | Registration surfaces | Discriminating tests |
|---|---|---|---|
| **A** | Persisted ChangeZone member-entry wire versioning: v7 writer, v1–v6 migrate/refuse, unversioned-raw v7 inference, exact diagnostic | `types/resolution.rs` (constants, `from_value`, `declare_raw_resolution_wire`, new private helpers, `pub const` diagnostic); literal version pins in `tests/integration/exploit_object_filter.rs` and `tests/integration/resolution_progress_contract.rs` (version-number-only edits); comment-only edit in `types/game_state.rs` | three reader-level tests in `game/effects/change_zone.rs` mod tests (§5.5) |
| **B** (optional, recommended) | Starfield-shaped granted-static rows | `tests/integration/conditional_entry_gate_projection.rs` only | two tests (§5.6) |

- **Dependencies:** none between A and B; they are independent and may land in either order.
- **Phase-fit path count:** 3 counted paths — `crates/engine/src/types/resolution.rs`, `crates/engine/src/game/effects/change_zone.rs`, `crates/engine/tests/integration/conditional_entry_gate_projection.rs`. `crates/engine/src/types/game_state.rs` is comment-only (standing inclusion). The two version-pin test files are wire-version-number-only surfaces, excluded from the count. 2 units.

## SCOPE_PATHS

```
crates/engine/src/types/resolution.rs
crates/engine/src/game/effects/change_zone.rs
crates/engine/src/types/game_state.rs
crates/engine/tests/integration/exploit_object_filter.rs
crates/engine/tests/integration/resolution_progress_contract.rs
crates/engine/tests/integration/conditional_entry_gate_projection.rs
```

## Building Blocks

- **Reused version constants:** `LEGACY_RESOLUTION_STATE_WIRE_VERSION`, `LEGACY_DEALERLESS_RESOLUTION_STATE_WIRE_VERSION`, `LEGACY_RESULT_OWNERSHIP_RESOLUTION_STATE_WIRE_VERSION`.
- **Precedents for the new code:** the typed-boundary shape probe follows `has_draw_sequence_delivery_owner` / `ResolutionBoundaryStack`; the refusal follows `reject_ambiguous_legacy_paused_draw_result`.
- **Reused serde pieces:** `crate::types::zones::etb_tap_bool_compat` (the exact historical `enter_tapped` codec), `ChangeZoneMemberEntry`'s own `Serialize`, `ObjectId`'s transparent serde.
- **Reused test infrastructure:**
  - change_zone.rs: `add_graveyard_rusted_relic`, `add_graveyard_ornithopter`, `set_core_types`, `add_shock_in_graveyard_for_test`, `parked_change_zone_iteration`, `plus_one_counters`, `typed`.
  - Scenario and engine: `GameScenario::add_spell_to_hand_from_oracle`, `GameRunner::cast(..).target_objects(..).commit()`, `CastCommit::{state_mut, act, state}`, `GameState::replace_active_change_zone_iteration`, `apply_as_current`.
  - Persistence: `PersistedGameState::{Raw, capture}`, `prepare_for_restore(PersistedRestoreFinalization::Immediate)`, `finalize_immediately`.
  - conditional_entry_gate_projection.rs: `board`, `add_what_must_be_done`, `add_graveyard_sanctum`, `add_enchantment_anchors`, `cast_release_juno`, `set_core_types`, `is_creature`, `put_counter_resolved`, `generic_and_white`.
- **New helpers, justified:** `migrate_legacy_change_zone_member_entries` (the single version-boundary authority), `migrate_legacy_change_zone_carrier` (one carrier), `carries_resolved_change_zone_members` (the inference probe). No existing helper handles the ChangeZone carrier shape.

## Logic Placement

- **All logic lives in the engine persistence seam, `types/resolution.rs`.** `ResolutionStateWire::from_value` is the single reader every public reader reaches, and `declare_raw_resolution_wire` is documented as "the only place in the engine permitted to infer a wire version".
- **No adapter changes.** The engine-wasm, server-core, phase-server and phase-ai paths pass the engine string through unchanged. Unlike the WASM-side `legacy_casting_variant_choice_lacks_face` pre-check, this keeps the logic in the engine.
- **No runtime or effect-handler change.** The carrier type and the drain stay as they are at HEAD.

## Rust Idioms

- Typed serde boundary structs instead of ad-hoc `Value` probing for the legacy spec: `LegacyChangeZoneMembers { remaining: Vec<ObjectId> }` and `LegacyChangeZoneEntrySpec`. The latter uses `#[serde(default, with = "crate::types::zones::etb_tap_bool_compat")] enter_tapped: EtbTapState`, `enters_attacking: bool` (required, as historically), `#[serde(default)] enter_with_counters: Vec<(CounterType, u32)>`, `#[serde(default)] conditional_enter_with_counters: Vec<serde::de::IgnoredAny>`, `#[serde(default)] enters_modified_if: Option<serde::de::IgnoredAny>`.
- Version comparisons go against named constants; no magic numbers.
- The inference boundary is an internally tagged private enum with `#[serde(other)] Other`, mirroring `ResolutionBoundaryFrame`.
- No bools are added to engine types; the gate predicate is a method `LegacyChangeZoneEntrySpec::entry_depends_on_gate(&self) -> bool` on a private serde struct.

## Extension vs Creation

This extends the established wire-version pattern: the version constants, version-keyed validation, unversioned inference by typed boundary, and precise refusal strings. No new pattern is introduced.

## Variant Discoverability

N/A: no engine enum variant is added. The private boundary enums are serde detection types, as `ResolutionBoundaryFrame` already is.

## Identity / Provenance Contract

| Aspect | Contract |
|---|---|
| Source concept | The parked ChangeZone member's resolved entry (CR 603.6a one event; CR 614.12 entry read before the event). |
| Authority | A historical carrier's own uniform fields (`enter_with_counters`, `enter_tapped`, `enters_attacking`), plus each bare `ObjectId` in `remaining`. |
| Binding time | Restore decode (persisted-version boundary), before any typed carrier decode. |
| Storage | `PendingChangeZoneIteration.remaining: Vec<ChangeZoneMemberEntry>`. Snapshotted, never re-read; unchanged runtime semantics. |
| Member identity | The same bare id the v6 drain used. The incarnation authority remains the carrier's `logical_zone_change_group`, which is unchanged. |
| Consumer | The HEAD `drain_pending_change_zone_iteration` / `process_one_zone_move_with_terminal`, unchanged. |
| Invalidation | None; the migrated record is the final authority. |
| Gated authority | Absent (the pre-event board is gone), so it is refused. |
| Multi-authority hostile fixtures | Gate present with an empty `remaining` (migrates); inert gate (migrates); live gate with non-empty `remaining` (refused); current member records in an unversioned raw payload (dated v7, never treated as legacy). |

## Reference Readings

1. **"Migrated legacy v1–v6 restore == current-wire restore of the same pause" (whole-`GameState` equality).** Derived independently: the HEAD resolver and the old drain both give each ungated member exactly the carrier's uniform fields (Contract decision). The expected entries are `{id, [(P1P1,2)], Tapped, true}` for each remaining member. Measured reference: P3/P4 showed the current-wire restore carries exactly those entries, and the v1 projection equals the current restore.
2. **"Unaffected old states restore as before."** The derivation: no ChangeZone carrier (or an empty `remaining`) means no member to transform. Measured: P3/P4 empty-`remaining` rows are Ok. Existing committed rows already exercise this through both readers: the `p1_9354_oracle_owner_contract_30.rs` v3 raw/trusted fixtures, the `resolution_progress_contract.rs` v5 Clash row (main's line ~1143 after merge), and the `mycoloth_devour_drain_strand.rs` v2 projection. They run unchanged at batched verification.
3. **Unit B.** No parity rows; every expected value is derived (§5.6).

No reference disagreed with its derivation.

## Verification Matrix

**Claim A1: v1–v6 ungated carriers migrate faithfully through both readers.**
- Seam: `ResolutionStateWire::from_value` → `migrate_legacy_change_zone_member_entries`.
- Entry: `serde_json::from_value::<PersistedGameState>` for Raw and Trusted, then `prepare_for_restore(Immediate)` → `finalize_immediately`.
- Test: `legacy_ungated_parked_change_zone_migrates_through_both_readers`.
- Revert-failing: delete the migration call and every legacy row fails with serde `invalid type: integer` (measured P3/P4).
- Positive reach: the decline applies Tapped and 2 counters (measured P3).
- Siblings: versions 1, 2, 3, 4, 5, 6. The inert-gate row migrates.

**Claim A2: gated carriers refuse with the exact constant at both readers.**
- Test: `legacy_gated_parked_change_zone_refuses_with_the_exact_diagnostic`.
- Revert-failing: without the gate predicate the row migrates (Ok), so the assertion fails. Without the migration, the error is the serde message, not the constant.
- Paired positive reach: the same fixture with only the gate keys removed restores Ok.
- Hostile rows: conditional gate; rider gate with Tapped; rider gate with attacking only; gate with an empty `remaining` (Ok, equals current); inert gate (Ok).
- First branch reached: `remaining.is_empty()` early return, then `entry_depends_on_gate`.

**Claim A3: the current writer declares 7 and round-trips.**
- Asserted in A1's test: `wire["resolution_state_version"] == RESOLUTION_STATE_WIRE_VERSION`, and a current restore whose parked `remaining` equals the mutated entries.
- Revert-failing: keep the writer at 6 and the migration then decodes member records as bare ids, so the current round-trip errs.

**Claim A4: an unversioned raw payload carrying member records is dated v7; a legacy unversioned payload is migrated or refused.**
- Test: `unversioned_raw_parked_change_zone_declares_its_member_shape`.
- Revert-failing: without the inference, `declare_raw_resolution_wire` declares 3 (measured P5), the migration rejects the records, and decode errs.
- Hostile rows: legacy bare-id unversioned (declares 3, migrates); a gated legacy unversioned payload (exact diagnostic).

**Claim A5: version pins move to 7.** `resolution_wire_contract_pins_v5_and_the_legacy_readers`, `exploit_object_filter.rs:165`, `resolution_progress_contract.rs:116`.

**Claim B: granted Starfield-shaped population conditions count the population before the entry, with each recipient's controller as "you".** Two tests (§5.6). Revert-failing: drop `EntryPopulationPin::grants` and the zero-counter rows get 2. Key the pin to the host's controller and the opposite-controller rows flip.

- **Coverage impact:** none; no parser change, so coverage status is unchanged.
- **Accepted-but-deferred Oracle text:** none.

## Step 5: Implementation steps

### 5.1 `crates/engine/src/types/resolution.rs`: constants and doc

- Replace `/// Version 6 retains deferred Clash bottom choices until the final answer.` with:
  ```
  /// Version 6 retains deferred Clash bottom choices until the final answer.
  /// Version 7 carries each unprocessed member of a parked ChangeZone with the
  /// entry modifications resolved before the event's first member moved
  /// (`ChangeZoneMemberEntry`) in place of a bare id and the raw entry gates.
  ```
- Change `pub const RESOLUTION_STATE_WIRE_VERSION: u64 = 6;` to `7`.
- After `LEGACY_DEALERLESS_RESOLUTION_STATE_WIRE_VERSION`, add:
  ```rust
  /// Last wire version whose parked ChangeZone carrier held bare member ids
  /// with carrier-level entry modifications and gates. Its readers migrate or
  /// refuse that carrier before the typed decode
  /// (`migrate_legacy_change_zone_member_entries`).
  const LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION: u64 = 6;
  ```
- After it, add the diagnostic:
  ```rust
  /// Stable, machine-recognizable restore refusal for a v1–v6 save paused inside
  /// a multi-object ChangeZone whose unprocessed members' entry modifications
  /// depend on an entry gate. CR 603.6a + CR 614.12: each member's gate is read
  /// against the board as it was before the event's first member moved, and that
  /// board is not in the save, so no member entry can be reconstructed. Player-
  /// visible copy (see `reject_viewer_projection_as_authority` for the frames
  /// that wrap it): it neither repeats "could not restore" nor announces a new
  /// game.
  pub const LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR: &str = "RESTORE_INCOMPATIBLE_GATED_CHANGE_ZONE_ENTRY: This saved game was paused while several cards were entering the battlefield at once, and whether each remaining card enters with counters, tapped, or attacking depends on the battlefield as it was before any of them entered. That battlefield is not part of the save, so the move cannot be finished as the rules require.";
  ```
  CR 603.6a and CR 614.12 were verified in `docs/MagicCompRules.txt`.

### 5.2 `crates/engine/src/types/resolution.rs`: the version-boundary migration

Place this beside `reject_ambiguous_legacy_paused_draw_result`. Add `use crate::types::counter::CounterType;`, `use crate::types::game_state::ChangeZoneMemberEntry;` and `use crate::types::zones::EtbTapState;` where they are not already imported.

```rust
/// The carrier fields a v1–v6 parked ChangeZone wrote and v7 replaced with
/// per-member records.
const LEGACY_CHANGE_ZONE_ENTRY_FIELDS: &[&str] = &[
    "enter_tapped", "enters_attacking", "enter_with_counters",
    "conditional_enter_with_counters", "enters_modified_if",
];

#[derive(Deserialize)]
struct LegacyChangeZoneMembers { remaining: Vec<ObjectId> }

/// The v1–v6 carrier-level entry modifications. The gates are read for presence
/// only: their content is the pre-event authority this migration never consults.
#[derive(Deserialize)]
struct LegacyChangeZoneEntrySpec {
    #[serde(default, with = "crate::types::zones::etb_tap_bool_compat")]
    enter_tapped: EtbTapState,
    enters_attacking: bool,
    #[serde(default)]
    enter_with_counters: Vec<(CounterType, u32)>,
    #[serde(default)]
    conditional_enter_with_counters: Vec<serde::de::IgnoredAny>,
    #[serde(default)]
    enters_modified_if: Option<serde::de::IgnoredAny>,
}

impl LegacyChangeZoneEntrySpec {
    /// CR 614.12: whether a member's entry depends on a gate verdict. Conditional
    /// counters always do. A rider gate matters only when a rider is set:
    /// `effective_enter_mods` yields `(Unspecified, false)` on a mismatch, so a
    /// gate over default riders gives every member the same entry either way.
    fn entry_depends_on_gate(&self) -> bool {
        !self.conditional_enter_with_counters.is_empty()
            || (self.enters_modified_if.is_some()
                && (!self.enter_tapped.is_unspecified() || self.enters_attacking))
    }
}
```

`migrate_legacy_change_zone_carrier(carrier: &mut Value) -> Result<(), String>`:
1. `carrier.as_object_mut()`, or Err `"legacy ChangeZone carrier must be a JSON object"`.
2. Remove each `LEGACY_CHANGE_ZONE_ENTRY_FIELDS` key into a fresh `Map`.
3. Decode `LegacyChangeZoneMembers` from `{"remaining": object["remaining"]}`. A missing `remaining` is a serde error, as before. Map errors to `format!("legacy ChangeZone carrier is malformed: {error}")`.
4. If `remaining` is empty: leave `object["remaining"]` as `[]` and return Ok. Gate fields only drove the member loop, so dropping them loses nothing.
5. Otherwise decode `LegacyChangeZoneEntrySpec` from the removed-fields map (same error mapping). If `entry_depends_on_gate()`, return `Err(LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR.to_string())`, annotated `// CR 603.6a + CR 614.12: the gate reads the board before the event's first member moved, which the save does not hold.`
6. Otherwise insert `"remaining"` = `serde_json::to_value(members)`, where each id maps to `ChangeZoneMemberEntry { object_id, enter_with_counters: spec.enter_with_counters.clone(), enter_tapped: spec.enter_tapped, enters_attacking: spec.enters_attacking }`. Annotate it with a `// CR 614.12:` sentence: with no gate, the entry is the carrier's own modifications, as `resolve_change_zone_member_entry` would resolve them.

`migrate_legacy_change_zone_member_entries(value: &mut Value, version: u64) -> Result<(), String>`, with a doc comment naming it the single v1–v6 → v7 authority:
- If `version > LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION`, return `Ok(())`.
- If `version == LEGACY_RESOLUTION_STATE_WIRE_VERSION`, migrate `value.get_mut("pending_change_zone_iteration")` when it is present and non-null.
- Otherwise iterate `value["resolution_frames"]["frames"]` (when it is an array). For every element with `type == "ChangeZone"`, migrate `data.pending` when it is present and non-null.

In `ResolutionStateWire::from_value`, immediately after `reject_ambiguous_legacy_paused_draw_result(&value, version)?;`, insert `migrate_legacy_change_zone_member_entries(&mut value, version)?;`.

Version `match`:
- Change the arm to `LEGACY_DEALERLESS_RESOLUTION_STATE_WIRE_VERSION | 5 | LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION | RESOLUTION_STATE_WIRE_VERSION => GameStateDecodeMode::ResolutionWireV4`.
- Update the unsupported-version message to `expected 1, 2, {…3}, {…4}, 5, {LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION}, or {RESOLUTION_STATE_WIRE_VERSION}`.
- Update the `from_value` doc: versions 4–7 share the frame layout; v1–v6 ChangeZone carriers are migrated or refused first by `migrate_legacy_change_zone_member_entries`.

### 5.3 `crates/engine/src/types/resolution.rs`: unversioned inference

Add a private boundary beside `ResolutionBoundaryStack`:

```rust
#[derive(Deserialize)]
#[serde(untagged)]
enum ChangeZoneBoundaryMember {
    /// A v1–v6 bare member id; only the shape is inspected.
    BareId(#[allow(dead_code)] ObjectId),
    /// A v7 `ChangeZoneMemberEntry`; only the shape is inspected.
    Resolved(#[allow(dead_code)] Map<String, Value>),
}
#[derive(Deserialize)]
struct ChangeZoneBoundaryCarrier { #[serde(default)] remaining: Vec<ChangeZoneBoundaryMember> }
#[derive(Deserialize)]
struct ChangeZoneBoundaryData { #[serde(default)] pending: Option<ChangeZoneBoundaryCarrier> }
#[derive(Deserialize)]
#[serde(tag = "type")]
enum ChangeZoneBoundaryFrame { ChangeZone { data: ChangeZoneBoundaryData }, #[serde(other)] Other }
#[derive(Deserialize)]
struct ChangeZoneBoundaryStack { #[serde(default)] frames: Vec<ChangeZoneBoundaryFrame> }
```

Add `fn carries_resolved_change_zone_members(stack: &Value) -> Result<bool, String>`. It decodes `ChangeZoneBoundaryStack`, maps errors as `typed resolution frame boundary is malformed: {e}` (same text as `typed_draw_sequence_frames`), and returns whether any ChangeZone carrier holds a `Resolved` member.

In `declare_raw_resolution_wire`:
- Compute `unversioned_stack_has_resolved_change_zone_members` next to `unversioned_stack_has_delivery_owner`, the same way (`.map(..).transpose()?.unwrap_or(false)`).
- In the `Some(frames)` arm, choose the version: `RESOLUTION_STATE_WIRE_VERSION` if resolved members are present, otherwise the existing owner-based 4/3.
- Add a comment: a member record is written only by the v7 carrier, so it dates the payload exactly as `delivery_owner` dates v4. A stack written that late also carries `delivery_owner` on every draw frame, which v7 validation still requires.

### 5.4 `crates/engine/src/types/resolution.rs` tests

In `resolution_wire_contract_pins_v5_and_the_legacy_readers`:
- `assert_eq!(RESOLUTION_STATE_WIRE_VERSION, 7);`
- Add `assert_eq!(LEGACY_UNRESOLVED_CHANGE_ZONE_MEMBER_RESOLUTION_STATE_WIRE_VERSION, 6);`

### 5.5 `crates/engine/src/game/effects/change_zone.rs` mod tests (Unit A)

Placement: directly after `paused_multi_object_drain_applies_pre_event_entries_under_a_non_move_root`. Add imports inside the test module as needed: `PersistedGameState`, `PersistedRestoreFinalization`, `crate::types::resolution::{LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR, RESOLUTION_STATE_WIRE_VERSION}`.

**Helpers:**

`const MASS_RETURN: &str = "Return up to three target permanent cards from your graveyard to the battlefield.";`, documented as *Synthetic (no printed card): a multi-target graveyard return cast through the real stack, so a paused carrier has a resolving spell entry.*

`fn paused_mass_return(mods: impl FnOnce(&mut Effect), shock_first: bool) -> (GameState, ObjectId, ObjectId, ObjectId)`:
- Build the scenario: `at_phase(PreCombatMain)`, `add_spell_to_hand_from_oracle(P0, "Mass Return", false, MASS_RETURN)`, `add_graveyard_rusted_relic`, `add_graveyard_ornithopter`, then `build()`.
- Set core types: relic `[Artifact]`, orni `[Artifact, Creature]`. Add the shock with `add_shock_in_graveyard_for_test(state, 701, P0, &[CoreType::Land])`.
- Targets: `[shock, relic, orni]` when `shock_first`, else `[relic, orni, shock]`. Then `runner.cast(spell).target_objects(&targets).commit()`.
- Apply `mods` to the top stack entry's `ability_mut().effect`, using `stack.get_mut(stack.len()-1)` because `im::Vector` has no `last_mut`.
- Submit `GameAction::PassPriority` until `WaitingFor::ReplacementChoice`, at most 6 times.
- Premises: assert `ReplacementChoice` and that `resolution_stack` kinds == `[FrameKind::ChangeZone]`.
- Return the cloned state and `(shock, relic, orni)`.

`fn restore_persisted(wire: Value) -> Result<GameState, String>`: `serde_json::from_value::<PersistedGameState>` → `prepare_for_restore(PersistedRestoreFinalization::Immediate)` → `finalize_immediately()`, with errors converted via `to_string()`.

`fn both_wires(state: &GameState) -> [(&'static str, Value); 2]`: Raw `serde_json::to_value(PersistedGameState::Raw(Box::new(state.clone())))` and trusted `serde_json::to_value(PersistedGameState::capture(state.clone()))`.

`fn downgrade_change_zone_carrier(wire: &Value, version: u64, legacy: &Value) -> Value`:
- Operates on `wire["state"]` when present, else on the wire.
- Rewrites the ChangeZone carrier (`resolution_frames.frames[i].data.pending` where `type == "ChangeZone"`) to its v1–v6 shape: `remaining` = the members' `object_id`s, and every key of the `legacy` object inserted at carrier level.
- Sets `resolution_state_version` = `version`.
- For `version == 1`, it also removes `resolution_frames` and moves the carrier to top-level `pending_change_zone_iteration` (the P4 projection).
- Doc: it reproduces what the v1–v6 writer wrote (`enter_tapped` as a bool, `enters_attacking` always present, other gate fields only when non-default).

**Test `legacy_ungated_parked_change_zone_migrates_through_both_readers`**

Doc: CR 603.6a + CR 614.12, plus the Contract-decision rationale. REVERT: removing the `from_value` migration call makes every legacy row fail with `invalid type: integer`; keeping the writer at 6 fails the current round-trip.

Setup:
- `mods`: `enters_attacking = true`, `enter_with_counters = vec![(CounterType::Plus1Plus1, QuantityExpr::Fixed { value: 2 })]`, shock first.
- Labelled synthetic step: set `enter_tapped = EtbTapState::Tapped` on every parked member via `replace_active_change_zone_iteration`. Doc why: with Tapped on the move itself the shock decline is moot and no pause occurs (probe P2). This models an ungated v6 carrier written with `enter_tapped: true`.
- `expected_members` = `[{relic, [(P1P1,2)], Tapped, true}, {orni, [(P1P1,2)], Tapped, true}]`.

For each `(label, current)` in `both_wires(&paused)`:
- Assert `state(current)["resolution_state_version"] == RESOLUTION_STATE_WIRE_VERSION` and that every carrier member is a JSON object.
- `let expected = restore_persisted(current.clone()).unwrap()`, then assert the parked `remaining == expected_members` (current round-trip).
- For `version in [1, 2, 3, 4, 5, 6]`:
  - `legacy = downgrade(&current, version, json!({"enter_tapped": true, "enters_attacking": true, "enter_with_counters": [["P1P1", 2]]}))`
  - `let migrated = restore_persisted(legacy).unwrap_or_else(..panic with label/version)`
  - `assert_eq!(migrated, expected)` (whole `GameState`, faithful).
- For `version in [1, 6]`, on the migrated state, `apply_as_current(ChooseReplacement { index: 1 })`, then assert:
  - shock on the battlefield and tapped (reach);
  - relic and orni on the battlefield, tapped, `plus_one_counters == 2`;
  - `parked_change_zone_iteration(..).is_none()`.

Unaffected-old row:
- Take the post-decline state from the v6 row (no frames). Its current wire set to `resolution_state_version: 6` restores Ok and equals the restore of its unmodified current wire.
- Both readers.

**Test `legacy_gated_parked_change_zone_refuses_with_the_exact_diagnostic`**

Board: `paused_mass_return(no mods, shock_first: true)`. The current members are default: `{id, [], Unspecified, false}`.

`creature = serde_json::to_value(typed(TypeFilter::Creature)).unwrap()` is the gate payload. `conditional = json!([[creature, "P1P1", serde_json::to_value(QuantityExpr::Fixed { value: 1 }).unwrap()]])`.

Refused rows, for each reader in `both_wires` and each `version in [1, 3, 6]`:
- (a) `{"enters_attacking": false, "conditional_enter_with_counters": conditional}`
- (b) `{"enters_attacking": false, "enter_tapped": true, "enters_modified_if": creature}`
- (c) `{"enters_attacking": true, "enters_modified_if": creature}`

Each asserts `restore_persisted(legacy).unwrap_err() == LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR`, plus `starts_with("RESTORE_INCOMPATIBLE_GATED_CHANGE_ZONE_ENTRY:")`. Paired positive reach guard for each: the same row with only `conditional_enter_with_counters` / `enters_modified_if` removed restores Ok.

Inert-gate row: `{"enters_attacking": false, "enters_modified_if": creature}` restores Ok and equals `restore_persisted(current)`. REVERT: dropping the rider-default clause of `entry_depends_on_gate` refuses this row.

Empty-`remaining` row: board `paused_mass_return(no mods, shock_first: false)` (`remaining` `[]`), with the legacy `{"enters_attacking": false, "conditional_enter_with_counters": conditional, "enters_modified_if": creature, "enter_tapped": true}`. Restores Ok and equals `restore_persisted(current)`; versions 1 and 6, both readers.

**Test `unversioned_raw_parked_change_zone_declares_its_member_shape`**

Board: the A1 board, before the Tapped mutation is fine.

- `raw = serde_json::to_value(&paused)` (derived; no version; has `resolution_stack`).
- On a clone, `crate::types::resolution::declare_raw_resolution_wire(&mut clone)` yields version `RESOLUTION_STATE_WIRE_VERSION`. REVERT: the inference removed gives 3 (P5).
- `restore_persisted(raw)` is Ok, and its parked `remaining` equals the paused carrier's `remaining`.
- Legacy unversioned (ungated): rewrite `raw["resolution_stack"]` frame `data.pending` the same way (bare ids plus `{"enters_attacking": true, "enter_with_counters": [["P1P1",2]]}`). `declare_raw_resolution_wire` declares 3; the restore is Ok and the parked `remaining` equals the paused one.
- Legacy unversioned (gated): adds `conditional_enter_with_counters`, and the restore error equals the constant.

### 5.6 `crates/engine/tests/integration/conditional_entry_gate_projection.rs` (Unit B, separate unit)

Add after `release_juno_granted_condition_binds_each_recipients_controller`:

```rust
/// Synthetic (no printed card): a host granting every enchantment Starfield of
/// Nyx's animation static, scoped to its recipient. Starfield's own text grants
/// nothing ("each other non-Aura enchantment you control"); this carrier gives the
/// same population condition and characteristics to each enchantment through
/// `GrantStaticAbility`, so the pin's grant arm reads it.
const STARFIELD_GRANT_HOST: &str = "Each enchantment has \"As long as you control five or more \
     enchantments, this enchantment is a creature in addition to its other types and has base \
     power and base toughness each equal to its mana value.\"";
```

`fn release_juno_under_starfield_grant(host_controller: PlayerId, p0_anchors: usize, p1_anchors: usize) -> (CastOutcome, ObjectId, ObjectId)` mirrors `release_juno_under_shrine_grant`:
- `board()`, `add_what_must_be_done`, `add_graveyard_sanctum`.
- The host via `add_enchantment_from_oracle(host_controller, "Starfield Grant Host", STARFIELD_GRANT_HOST).with_mana_cost(generic_and_white(4, 1))`. Mana value 5 keeps the animated host alive as a 5/5.
- `add_enchantment_anchors(P0, "Enchantment Anchor", p0_anchors)` and `add_enchantment_anchors(P1, "P1 Anchor", p1_anchors)`.
- Build, then `set_core_types(sanctum, [Enchantment])`.
- Premise: the host carries `GrantStaticAbility { definition }` with `definition.condition.is_some()` (P6 measured this exact shape).
- `cast_release_juno(runner, spell, sanctum)`.

**`release_juno_granted_starfield_condition_counts_pre_entry_enchantments`** (CR 614.12 + CR 109.5). Rows `(p0_anchors, counters, pt)`:
- `(3, 0, (1,1))`: host plus three anchors means P0 controls four enchantments before Sanctum enters as the fifth, so it is not a creature as it would enter: zero counters. After entry, five enchantments, so Sanctum is a creature with base P/T equal to its mana value 1.
- `(4, 2, (3,3))`: already true before entry.

Each row asserts zone Battlefield, the counters, `is_creature(sanctum)` (live-pass reach), the P/T, and `!put_counter_resolved`.

**`release_juno_granted_starfield_condition_binds_the_recipients_controller`** (host P1; "you" is Sanctum's controller P0). Rows `(p0_anchors, p1_anchors, counters, pt)`:
- `(4, 4, 0, (1,1))`: P0 controls four before entry. A host-keyed pin would read P1's five and give 2.
- `(5, 0, 2, (3,3))`: a host-keyed pin would read P1's one and give 0.

Each row also asserts `controller == P0` and creature-after-entry.

Module doc: in the REVERT DISCRIMINATORS bullet for `EntryPopulationPin::grants`, append that these two tests fail too: zero-counter rows get 2, and with a host-keyed pin the opposite-controller rows flip.

**Expected values derived** (unprobed outcome; runs at batched verification):
- CR 614.12: continuous effects that already exist apply, so the grant applies; the entrant is not yet counted, so the population comes from the pre-entry pin.
- CR 109.5: "you" means the object's controller.
- Sanctum's mana value 1 is from `generic_and_white(0, 1)`. The counters are +2 from What Must Be Done.

### 5.7 Version pins (wire-version-number-only edits)

- `crates/engine/tests/integration/exploit_object_filter.rs:165`: `assert_eq!(RESOLUTION_STATE_WIRE_VERSION, 6);` → `7`.
- `crates/engine/tests/integration/resolution_progress_contract.rs:116`: `assert_eq!(json["resolution_state_version"], 6);` → `7`.

### 5.8 `crates/engine/src/types/game_state.rs` (comment-only)

In `change_zone_iteration_modern_shape_roundtrips`, replace

```
        // A legacy carrier whose members are bare ids has no pre-event entry
        // modifications to resume with, so it must reject rather than default.
```
with
```
        // The carrier itself never accepts bare-id members. A persisted v1–v6
        // carrier is migrated or refused before this decode by
        // `migrate_legacy_change_zone_member_entries` (`types/resolution.rs`).
```

## Verification (risk-scaled, per eddee97)

**This round:** `cargo fmt --all` only. No cargo build, test or clippy.

**Deferred to the final combined candidate**, after merging upstream 0a853dbb3, reallocating protocol 150/132 and reconciling CR733:
- `cargo test -p phase-engine --lib -- change_zone::tests::legacy_ change_zone::tests::unversioned_raw_parked types::resolution types::game_state::tests::change_zone_iteration`
- `cargo test -p phase-engine --test integration -- conditional_entry_gate_projection exploit_object_filter resolution_progress_contract p1_9354_oracle_owner_contract_30 mycoloth_devour_drain_strand raw_resolution_stack_restore eagles_delayed_result_phase3`
- clippy for `phase-engine` (`-D warnings`).

No parser change, so no parser-output measurement. Report these checks as deferred, not passed.

## Interactions to carry into the next round (not planned here)

- **Upstream main 0a853dbb3:**
  - It keeps `RESOLUTION_STATE_WIRE_VERSION = 6`; it does not bump the version.
  - Its only `resolution.rs` change (872c4e311) adds `validate_owed_spell_final_part_coherence` calls in both `from_value` branches and `resolving_spell_final_part = None` in `normalize_legacy_completed_resolution_carrier`. Those are adjacent to, but not overlapping, this plan's edits (constants, the version `match`, one inserted call after `reject_ambiguous_legacy_paused_draw_result`, `declare_raw_resolution_wire`).
  - 0a853dbb3 does not change `exploit_object_filter.rs` or `resolution_progress_contract.rs`; the §5.7 edits to 7 carry through the merge unchanged. Main's v5 Clash row there (`json["resolution_state_version"] = 5`) is an unaffected-old-state row and must keep passing.
- **Protocol:** when reallocating to 150/132, the protocol history doc lines (`crates/lobby-broker/src/protocol.rs` / `crates/server-core/src/protocol.rs`) should also state `RESOLUTION_STATE_WIRE_VERSION` 6 → 7, following the v116 precedent ("4 to 5 … A v115 peer refuses the version-5 resolution state").
- **CR733 fixtures:** this plan adds no `GameState` write sites (the migration rewrites JSON before decode), so no expected census change. Confirm during the scheduled reconciliation.

## Reply mapping for the maintainer

| Request | Where it is answered |
|---|---|
| "Version the shape" | v7 writer; v1–v6 readers migrate or refuse (§5.1–5.2) |
| "Migration where the original authority is available" | ungated and inert-gate carriers expanded exactly |
| "Refuse … with a precise diagnostic" | gated carriers with a non-empty `remaining` → `LEGACY_GATED_CHANGE_ZONE_ENTRY_RESTORE_ERROR` |
| "Raw and trusted-envelope reader regressions with non-default tap/attack/counter/conditional settings, current round-trips, unaffected old states" | the three change_zone.rs tests (§5.5), all through `PersistedGameState` Raw and Trusted with `prepare_for_restore` → `finalize_immediately` |
| Starfield-shaped grant rows | the two new tests in §5.6 |
