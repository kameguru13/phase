# Plan: evaluate every member's entry gates against the board before the event (CR 603.6a + CR 614.12)

Ordinary mode, run etb-review-fixes, final-review fix round. Base and start are 0eb777b8a. Implementation worktree: /mnt/data/dev/wt/etb. Everything below was gathered read-only. Nothing in the repo was edited.

## Step 0: Premise check

Oracle text was read from `/mnt/data/dev/phase-personal/data/mtgjson/AtomicCards.json` with jq.

- **Rusted Relic (Artifact):** "Metalcraft — This artifact is a 5/5 Golem artifact creature as long as you control three or more artifacts."
- **Ornithopter (Artifact Creature — Thopter):** "Flying"
- **Recommission:** "Return target artifact or creature card with mana value 3 or less from your graveyard to the battlefield. If a creature enters this way, it enters with an additional +1/+1 counter on it."
- **What Must Be Done, mode 2:** "Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature."
- **March of the Machines:** "Each noncreature artifact is an artifact creature with power and toughness each equal to its mana value."

The finding's premise holds. Every printed gated card returns a single object, so the defect is latent. It is real for any multi-object gated `ChangeZone`, and the PR already asserts multi-object gated projection.

CR numbers were verified by grep in `/mnt/data/dev/phase-personal/docs/MagicCompRules.txt`:

| Rule | Line | What it says |
|---|---|---|
| 603.6a | 2605 | "Each time an event puts one or more permanents onto the battlefield…" — one event, several permanents |
| 603.10 | 2642 | — |
| 611.2e | 2921 | Cited only for the out-of-scope C1 follow-up |
| 613.1 | 2964 | — |
| 614.1c | 3066 | — |
| 614.12 | 3104 | "check the characteristics of the permanent as it would exist on the battlefield … continuous effects that already exist" |
| 614.12a | 3109 | The choice is made before the permanent enters |
| 614.12b | 3111 | Choices for replacements on multiple permanents entering simultaneously may not make their combined costs unpayable (cited only as evidence that such entry is one event) |

Heliod, Sun-Crowned ruling (2020-01-24), already quoted in `conditional_entry_gate_projection.rs`: replacement effects are considered before the permanent is on the battlefield.

## Step 1: Applicable skills

- **`/add-replacement-effect`:** Phase 5 tests apply: pipeline test with a revert-failing assertion, accept/decline through `GameAction::ChooseReplacement`, and a serialized-state test. Phases 1–4 are N/A: no new `ReplacementEvent`, condition, registry entry or parser change.
- **`/casting-stack-conditions` and `/add-interactive-effect`:** the paused-continuation and WaitingFor round-trip pattern. Phase 8 tests apply. No new WaitingFor, GameAction, AI, frontend or multiplayer routing, so those phases are N/A.
- **`/card-test`:** read for the foot-guns, especially paired positive reach-guards. The new rows are constructed-ability unit tests in `change_zone.rs`, because no printed card makes a multi-object gated move. This is allowed by the brief and follows `paused_multi_object_resume_projects_the_paused_move`.
- **`/add-engine-variant`:** N/A. No enum variant is added, only two structs: one serialized and one borrowed and non-serialized.
- **`/oracle-parser`:** N/A. No parser change.

## Step 2: Analogous traces

**Trace A — the conditional-counter path today:**
- `Effect::ChangeZone.conditional_enter_with_counters` / `enters_modified_if` (`types/ability.rs`)
- → `change_zone::resolve`: single-pick branches at about 1077 and 1171, and the targeted loop at about 1304–1500
- → `enter_with_counters_for_object` → `enter_with_counters_for_entry` → `entry_gate_projection`
- → `filter::BattlefieldEntrantProjection::new` (`filter.rs`, about 4856). This clones the live `state` as the scratch and calls `layers::pin_entry_population_conditions(&mut scratch, state, …)` (`layers.rs`, about 1736) with the live state as `pre_entry`.
- → `ChangeZoneIterationCtx` → `process_one_zone_move_with_terminal`, which also calls `effective_enter_mods` against the live state at move time
- → `zone_pipeline::execute_zone_move_with_terminal_and_controller`.

On a pause the loop builds `PendingChangeZoneIteration` (`types/game_state.rs`, about 4319), which carries the raw gate spec. It is parked in `ChangeZoneFrame.pending` (`types/resolution.rs`, about 389) via `GameState::push_change_zone_iteration` or `push_change_zone_iteration_after_child`. It is resumed in one of two places:
- `effects/mod.rs::drain_pending_change_zone_iteration`, which recomputes with `enter_with_counters_for_pending_object` against the live state, and
- `engine_resolution_choices.rs`, EffectZoneChoice `ChangeZone | BounceAll` arm (about 6627–6950), which computes per object inside its own move loop.

The PayCost arm (about 7221) and `search_outside_game.rs` (about 181) also build the context.

**Trace B — the existing pre-event authority (the model):** the Devour co-entry snapshot.
- `change_zone::resolve` targeted loop, about 1322: "Captured before any member enters so every co-arriver … is excluded regardless of iteration order".
- → `GameState::push_devour_change_zone_snapshot` → `ChangeZoneFrame.devour_eligible_snapshot`
- → preserved across re-park by `push_change_zone_iteration` and `replace_active_change_zone_iteration` (`game_state.rs`, about 26356–26440)
- → read via `active_devour_eligible_snapshot` (`zone_pipeline.rs`, about 3779; `engine_resolution_choices.rs`, about 6294).

This is the established pattern: capture once before the first member moves, carry it in the ChangeZone frame through every pause, and never recompute.

**Mechanism of the defect (traced, not probed):**
1. Targeted loop: member N's counters are computed inside the loop after members 1..N-1 have moved (change_zone.rs, about 1383–1402).
2. The tapped/attacking riders are computed inside `process_one_zone_move_with_terminal`, also after earlier members moved.
3. The drain and EffectZoneChoice paths behave the same way.
4. `BattlefieldEntrantProjection::new` treats the live state as both the scratch base and `pre_entry`. Co-entrants therefore count toward pinned population leaves, and their statics apply as "existing" effects.
5. Example: two anchor artifacts, targets [Ornithopter, Rusted Relic]. The live board at Relic's turn holds three artifacts, so metalcraft pins true, the Relic projects as a creature, and it gets a counter. The correct result is none.

## Step 3: Files read

- `crates/engine/src/game/effects/change_zone.rs`: resolve branches at 1000–1560; gate helpers and context at 1590–1960; `resolve_all` pause sites around 2440–2620; tests around 4380–4720, 7528–7640 and 9990–10180, plus `add_shock_in_library_for_test` at about 10622.
- `crates/engine/src/game/effects/mod.rs`: `drain_pending_change_zone_iteration`, about 2161–2470.
- `crates/engine/src/game/engine_resolution_choices.rs`: EffectZoneChoice destructure at about 6060–6095; `ChangeZone` arm at 6600–6960; PayCost at about 7215–7380.
- `crates/engine/src/game/filter.rs`: `BattlefieldEntrantProjection`, 4780–4990.
- `crates/engine/src/game/layers.rs`: `pin_entry_population_conditions` doc and body, about 1707–1800.
- `crates/engine/src/types/game_state.rs`: `PendingChangeZoneIteration` at 4290–4420; frame helpers at 26316–26460; serde test at about 45170–45280; test literal at about 35496.
- `crates/engine/src/types/resolution.rs`: `ChangeZoneFrame` at 386–398; `RESOLUTION_STATE_WIRE_VERSION` doc at about 3799; test literal at about 6027.
- `crates/engine/src/game/elimination.rs`: retain at about 907; test helper at about 2028.
- `crates/engine/src/game/effects/search_outside_game.rs`: about 170–205.
- `crates/engine/src/game/zone_pipeline.rs`: `EntryMods` at 144. It is not serializable and its scope is broader, so it is not reusable as the carrier.
- `crates/engine/tests/integration/conditional_entry_gate_projection.rs`: header and helpers.
- `scripts/check-protocol-version.mjs`, `crates/lobby-broker/src/protocol.rs` (about 1080–1112), and the client protocol pins.

## Step 3.5: Probes

**No probe ran.** This environment has no Rust toolchain (`cargo` is not on PATH), and Tilt is down (`tilt get uiresource clippy` exits 1). A scratch copy at `/mnt/data/dev/probe-etb-planner` was set up and then deleted after the build failed with "cargo: No such file or directory". It never touched any checkout.

Unprobed assertions, all traced from source:
- (P1) At 0eb777b8a, targets [Ornithopter, Relic] with two anchors give the Relic 1 counter.
- (P2) A co-entering March makes a co-entering noncreature artifact take the counter, and also the tapped rider.
- (P3) The drain resume at 0eb777b8a gives the Relic a counter when an earlier paused member is an artifact.
- (P4) A graveyard-origin MayCost shock replacement parks the targeted loop with `ReplacementChoice`, as the library-origin shock tests do.

The implementer must run each new test against 0eb777b8a semantics (see the REVERT notes) before treating it as discriminating. The first implementation verification is the probe.

## Design decision

**Precompute each member's resolved entry modifications before any member moves, and carry them as the members themselves.** `PendingChangeZoneIteration.remaining` changes from `Vec<ObjectId>` to `Vec<ChangeZoneMemberEntry>`.

Rejected alternatives:
- **A parallel results vector or map** keyed by id: this is parallel bookkeeping. Elimination's `retain` would have to prune two collections.
- **Carrying a "pre-event basis"** such as a Devour-style battlefield id set: a set of ids cannot rebuild the pre-event board. Co-entrants are already placed. Earlier members' as-enters work, such as Devour sacrifices, has already changed the board. `pin_entry_population_conditions` needs a full pre-event `GameState`.
- **Storing a cloned `GameState`** in the frame: unbounded size and a heavy serialized payload.

The resolved values are small, typed and exactly what the move consumes: counters, tap state, attacking. They are fixed at the CR 614.12 decision point, which is before the single CR 603.6a event in which every member enters. Only continuous effects that already exist then apply, and a co-entrant's statics are not among them. The population is the pre-entry one, per the Heliod ruling.

Pauses: every member's results are computed before the loop starts. A pause after k members therefore stores `members[k+1..]` with their pre-event results unchanged. The drain and EffectZoneChoice resumes apply them and never recompute. This is Devour's capture-once, carry-through pattern applied per member.

**Serialized shape:** yes, it changes.
- `PendingChangeZoneIteration.remaining` is retyped.
- Five fields become dead and are removed: `enter_tapped`, `enters_attacking`, `enter_with_counters`, `conditional_enter_with_counters`, `enters_modified_if`. Keeping them would leave two authorities.
- New struct `ChangeZoneMemberEntry`.
- No `#[serde(default)]` on the members. A legacy carrier with bare ids is rejected, because a default would resume a legacy pause with no counters or riders. This follows the precedent in `change_zone_iteration_modern_shape_roundtrips`, which already rejects legacy carriers missing `logical_zone_change_group` / `immediately_before_latched`.
- No repo-owned fixture contains a paused ChangeZone frame. A grep of `tests/fixtures/**/*.json.gz` and `tests/integration/fixtures/*.json` for `paused_current` / `enters_modified_if` found nothing.

**Protocol:**
- A full-game protocol parse bump is required, 147 → 148, with the P2P wire moving in lockstep, 129 → 130. Old and new peers cannot parse each other's `GameState` while a ChangeZone frame is parked. Precedent: lobby-broker doc v18 ("paused logical zone-change owners are now wire-visible") and v23/v25, and #10047 bumped for a frame-shape change.
- `RESOLUTION_STATE_WIRE_VERSION` stays at 6. Its history (3–6) bumps only for migrated required shapes. The logical-group carrier requirement was added with only a protocol bump, and legacy carriers are rejected rather than migrated. Migration is impossible here anyway: a v6 pause has no pre-event results to recover.
- If main has passed 147/129 at integration time, use the next free pair.

**C1 (CR 611.2e onset deferral) stays open.** It will add the effect's own "is [characteristic]" onset to the projection. That fits in the one precompute helper by extending `ChangeZoneEntrySpec`, and no resume site needs changing.

## Pattern coverage

The class is every multi-object `ChangeZone` with an effect-carried entry gate, for every member:
- `conditional_enter_with_counters` (Recommission, What Must Be Done mode 2, the Winter Soldier rider class), and
- `enters_modified_if` tapped/attacking riders (Summoner's Grimoire class).

This applies across all four multi-member production paths: targeted loop, EffectZoneChoice resume, mid-loop pause drain, and PayCost (ungated). Every gate shape (type, subtype, controller, mana value) and every population-pinned condition gets the same treatment. Printed cards are single-object today, so 0 card verdicts flip. The defect is latent, and the fix makes the machinery rules-correct for the whole class.

Single-object branches, where pre-event equals live, route through the same helper so the resolver has one authority. `ChangeZoneAll` has no gate fields; its members get unconditional entries that are identical by construction.

## Sizing

- **Units (T1): 1.**
  - U1 is "pre-event resolution of every ChangeZone member's effect-carried entry modifications, carried through pauses". It is one mechanic, one CR 603.6a + 614.12 decision, one checklist pass.
  - Registration surfaces: `ChangeZoneMemberEntry` and the `PendingChangeZoneIteration` retype; `ChangeZoneEntrySpec` and `resolve_change_zone_member_entries`; the `process_one_zone_move_with_terminal` signature; the five producer/consumer sites; elimination `retain`; and the protocol pair.
  - Discriminating tests: V1, V1b, V2 and V3 below. V4 and V5 cover machinery.
  - No inter-unit edges.
- **Counted scope paths (T2): 8.**
  1. `crates/engine/src/types/game_state.rs`
  2. `crates/engine/src/game/effects/change_zone.rs`
  3. `crates/engine/src/game/effects/mod.rs`
  4. `crates/engine/src/game/engine_resolution_choices.rs`
  5. `crates/engine/src/game/effects/search_outside_game.rs`
  6. `crates/engine/src/game/elimination.rs`
  7. `crates/engine/src/types/resolution.rs` (test-module literal only; counted conservatively)
  8. `crates/engine/src/game/filter.rs` (doc comment only)
- **Excluded protocol-version surfaces:** `crates/lobby-broker/src/protocol.rs`, `crates/server-core/src/protocol.rs` (test pin), `scripts/check-protocol-version.mjs`, `client/src/adapter/ws-adapter.ts`, `client/src/network/protocol.ts`, `client/src/network/__tests__/protocol.test.ts`, `client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`, `client/src/services/__tests__/openPhaseSocket.test.ts`.
- **Phase-fit:** T1 = 1 and T2 = 8 < 13, so it is not tripped.

## Building blocks

Reused:
- `enter_with_counters_for_entry`: counter merge with a caller-supplied descriptor, uses `FilterContext::from_ability` and `resolve_quantity_with_targets`.
- `effective_enter_mods`: the rider gate.
- `entry_gate_projection` / `BattlefieldEntrantProjection::new` / `pin_entry_population_conditions`: unchanged, now always called on the pre-event state.
- `EntryGateContext`: built from the already-resolved `dest_zone`, `enters_under_player` and `enter_transformed`.
- `resolving_stack_ability_for_source`: still binds the EffectZoneChoice filters and quantities.
- `EtbTapState` and `EtbTapState::is_unspecified`.
- The frame API, which is unchanged: `push_change_zone_iteration(_after_child)` and `replace_active_change_zone_iteration(_after_child)`.

New helpers, justified:
- `resolve_change_zone_member_entry` / `resolve_change_zone_member_entries`. They replace the two per-member compute sites (counters in loops, riders inside the mover) with one authority, called before the first move.
- `enter_with_counters_for_pending_object` and `EntryGateContext::from_iteration` become dead and are deleted.

## Logic placement

- `types/game_state.rs`: the serialized carrier (`ChangeZoneMemberEntry`, next to `PendingChangeZoneIteration`).
- `game/effects/change_zone.rs`: resolution of gate results, an effect-layer rules decision next to the existing gate helpers.
- Callers: the targeted loop, the EffectZoneChoice handler, PayCost, the drain, `resolve_all` and `search_outside_game`. They only build the spec and pass members through.
- The mover (`process_one_zone_move_with_terminal`) becomes a pure executor of resolved entry modifications.
- No frontend, AI, parser or transport logic.

## Rust idioms

- Typed struct over parallel vectors. `EtbTapState`, not a bool, for tap state.
- `ChangeZoneEntrySpec<'a>` borrows slices and filters (no clones) and is `Clone + Copy`-free plain data.
- `Option<&ResolvedAbility>` for the binding; `None` is documented as the EffectZoneChoice no-root case and keeps today's base-only behavior.
- Iterator `map(...).collect()`; `members[i + 1..].to_vec()` for re-park.
- No wildcard matches are added.
- The `#[allow(dead_code)] conditional_enter_with_counters` on the context is removed together with the field.

## Nom compliance

N/A. No file under `crates/engine/src/parser/` changes.

## Extension vs creation

This extends the existing ChangeZone pause machinery (`ChangeZoneFrame` / `PendingChangeZoneIteration` / drain) and the existing gate helpers. Its shape copies the Devour pre-entry snapshot authority. No new pattern, frame kind or WaitingFor.

## Variant discoverability

No enum variant is added. `cargo engine-inventory` and the `/add-engine-variant` gate are not triggered. The new types are a serialized struct and a borrowed spec struct.

## Identity / provenance contract

| Aspect | Contract |
|---|---|
| Source phrase | "If a creature enters this way" / "It enters with … if it's a creature" / gated "enters tapped and attacking" |
| Rules concept | CR 614.12 entry replacement, decided before the single CR 603.6a event |
| Authority | `ChangeZoneMemberEntry { object_id, enter_with_counters, enter_tapped, enters_attacking }`, one per member. `object_id` has the same identity semantics as today's `remaining` ids. The current paused member is still `paused_current`, keyed by `ObjectIncarnationRef`. |
| Binding time | Targeted loop: after targets are fixed, before the first member moves. EffectZoneChoice: on `SelectCards`, before the first move. Single-pick: before its one move. PayCost: before the first move. |
| Semantics | Latched values, never recomputed. A member that later fails to move (origin mismatch, CR 400.7) just leaves its entry unused, and the others are unaffected. |
| Storage | `ChangeZoneFrame.pending.remaining[*]` on the resolution stack |
| Consumer | `process_one_zone_move_with_terminal(state, ctx, member, events)` |
| Invalidation | Consumed when its member is processed. Removed by `elimination::abandon_pending_zone_change_member_for_player_left` (`retain` by `object_id`). Retired with the frame. |
| Multi-authority fixture | V3 row (b): non-move root (Draw) vs the move's own descriptor. The existing `paused_multi_object_resume_projects_the_paused_move` covers the EffectZoneChoice descriptor. The existing `conditional_entry_counters_project_under_entering_controller` covers owner vs `enters_under` controller (single-pick path, now via the helper). |

## Reference readings

Each expected value is derived from Oracle text and CR. No row asserts parity with another reading.

- **V1 [Relic, Ornithopter] order row:** derived independently: Relic 0, Ornithopter 1. It is not "same as the other order".
- **Positive controls:** an existing March gives a Sol-Ring-class artifact the counter (March is an existing continuous effect, so it would be a creature). Three anchors give the Relic 1 (metalcraft already holds before the event).
- **Kept existing test `paused_multi_object_resume_projects_the_paused_move`** (preservation): both noncreature artifacts on a board where March already exists would be creatures, so each gets 1. The derivation agrees with the current reading.
- **Kept single-object integration rows** in `conditional_entry_gate_projection.rs`: single-member events, where pre-event equals live. The derived values are already in that file.
- **Rewritten drain machinery test (V5):** card-independent. The expected values are the constructed carried entries, labeled non-derivable from any card.

## Verification matrix

Every new test lives in `crates/engine/src/game/effects/change_zone.rs` `mod tests`. They use the existing helpers `graveyard_noncreature_artifact`, `typed`, `MARCH_OF_THE_MACHINES`, `conditional_creature_counter_move` (it already accepts `targets`) and `add_shock_in_library_for_test` (adapt a graveyard-origin sibling `add_shock_in_graveyard_for_test`).

Test setup:
- Rusted Relic is built as `GameScenario::add_creature_to_graveyard(P0, "Rusted Relic", 0, 0).with_mana_cost({4}).from_oracle_text(RUSTED_RELIC)` with verbatim text. After `build`, its core types are set to `[Artifact]` and printed P/T is cleared, as `conditional_entry_gate_projection.rs::set_core_types` does.
- Ornithopter: `from_oracle_text("Flying")`, core types `[Artifact, Creature]`, 0/2.
- March as a graveyard card: `from_oracle_text(MARCH_OF_THE_MACHINES)` with core types `[Enchantment]`.
- Anchors: `add_artifact_from_oracle`.

**V1 — `multi_object_gated_move_reads_the_pre_event_population`**
- Path: targeted loop, change_zone.rs at about 1384. Entry: `resolve(state, &move)` with two `TargetRef::Object`s, gate Creature +1/+1.
- Rows:
  - (a) two anchors, [Ornithopter, Relic]: Ornithopter counter 1 (reach guard: the gate ran); Relic counter 0; Relic is a 5/5 creature after the event (reach: four artifacts).
  - (b) two anchors, [Relic, Ornithopter]: same derived values (hostile order sibling).
  - (c) three anchors, [Ornithopter, Relic]: Relic 1 (positive control).
- REVERT: computing counters inside the loop per member, as `enter_with_counters_for_object` does at 0eb777b8a, gives row (a) Relic = 1.

**V1b — `multi_object_gated_choice_reads_the_pre_event_population`**
- Path: EffectZoneChoice arm. Set up like the existing paused test, including its Draw root set as `state.resolving_stack_entry` with the move as `sub_ability`: `up_to`, `MultiTargetSpec::fixed(0, 2)`, `TargetChoiceTiming::Resolution`, then `apply_as_current(SelectCards [orni, relic])`, two anchors.
- Expect Ornithopter 1, Relic 0, Relic a creature after.
- REVERT: per-object computation inside the handler's move loop gives Relic 1.
- Empty/decline sibling: `SelectCards []` with `up_to` moves nothing, no panic, frame cleared.

**V2 — `co_entering_static_is_not_an_existing_effect_for_entry_gates`** (finding test 2)
- Counters arm: targets [graveyard March, graveyard noncreature artifact with mana value 1], no March on the battlefield.
  - Artifact counter 0.
  - Reach guards: both on the battlefield; the artifact is a 1/1 artifact creature after the event (March applies once it exists); March counter 0.
  - Positive control: an extra March already on the battlefield gives the artifact 1.
- Rider arm (`enters_modified_if`): same targets with `enters_modified_if: Some(typed(Creature))` and `enter_tapped: Tapped`.
  - The artifact enters untapped.
  - Positive control with an existing March: tapped.
- REVERT: per-iteration counters give the counters arm 1. `effective_enter_mods` left inside `process_one_zone_move_with_terminal` reads the live board with March present and leaves the artifact tapped.

**V3 — `paused_multi_object_drain_applies_pre_event_entries_under_a_non_move_root`** (LOW finding; path `drain_pending_change_zone_iteration`)
- Root is `Draw` with the gated move as `sub_ability`, set as `state.resolving_stack_entry`, as in the existing paused test. The move is resolved directly.
- Row (a), live-state discriminator:
  - Setup: two anchors; targets [shock-replacement artifact land (core types `[Artifact, Land]`), Relic, Ornithopter].
  - `resolve` parks on `ReplacementChoice`.
  - Structural: the active frame's `pending.remaining` equals [Relic with counters `[]`, Ornithopter with `[(Plus1Plus1, 1)]`]. This proves the pre-event results are carried.
  - `apply_as_current(ChooseReplacement { index: 1 })` (decline) resumes through the drain: Relic 0, Ornithopter 1 (reach), Relic a creature after, frame retired.
- Row (b), root discriminator:
  - Setup: March on the battlefield; targets [shock land with `[Land]`, noncreature artifact]. After the decline, the artifact has 1 counter.
- REVERT:
  - At 0eb777b8a the test does not compile, because of the carrier retype. With the structural assertion removed, row (a) fails behaviorally: the drain recomputes against the live board, which holds the shock artifact land, so the Relic gets 1.
  - Recomputing in the drain from the root descriptor (the pre-0eb777b8a reading, `EntryGateContext::from_change_zone(root)` = `None`) gives row (b) 0. This replaces the drain-caller guard the LOW said was missing.
- The accept path is covered by the existing shock tests.
- The resolving_stack_entry must be set; without it the 0eb777b8a drain binds None and row (a) is not discriminating.

**V4 — serde**, in `types/game_state.rs` `change_zone_iteration_modern_shape_roundtrips`:
- Add a non-empty `remaining` member with counters, `Tapped`, `enters_attacking: true`; round-trip equality.
- A legacy carrier whose `remaining` is `[7]` must fail, and the error must name `ChangeZoneMemberEntry`.
- Removed fields are absent from the JSON.

**V5 — rewrite `pending_change_zone_drain_recomputes_conditional_entry_counters_per_object`** as `pending_change_zone_drain_applies_each_members_carried_entry`:
- Push a carrier whose members hold different entries: Hero `[(P1P1, 1)]`, Soldier `[]`.
- The drain applies each member's own entry: Hero 1, Soldier 0, so one member's vector is not reused.
- REVERT: applying `remaining[0]`'s entry to every member gives Soldier 1.

**Must stay green:**
- `paused_multi_object_resume_projects_the_paused_move`: EffectZoneChoice descriptor. Its REVERT note now refers to the precompute's `EntryGateContext`.
- `conditional_entry_counters_project_only_on_battlefield_entry` and `conditional_entry_counters_project_under_entering_controller`.
- `effective_enter_mods_reads_projected_types` and the V-P7b production-path test.
- The shock and face-down paused tests (`paused_face_down_change_zone_resumes_face_down_with_profile`, the issue #535 rows).
- The Devour co-entry tests, the elimination player-left tests, and all of `tests/integration/conditional_entry_gate_projection.rs` and `etb_trigger_entry_time_characteristics.rs`.

**Coverage status impact:** none. No parser change, and no supported or unsupported verdict moves.

## Step 5: Implementation steps

**1. `crates/engine/src/types/game_state.rs`**
- (a) Add next to `PendingChangeZoneIteration`:
  ```rust
  /// CR 603.6a + CR 614.12: one member of a ChangeZone event with the effect-carried modifications it enters with — its gated entry counters and gated tapped/attacking riders — resolved against the board as it was before the event's first member moved. A co-entrant is not on the battlefield then, so neither its statics nor its presence affect another member's gates (Heliod, Sun-Crowned ruling 2020-01-24).
  #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
  pub struct ChangeZoneMemberEntry {
      pub object_id: ObjectId,
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      pub enter_with_counters: Vec<(CounterType, u32)>,
      #[serde(default, skip_serializing_if = "EtbTapState::is_unspecified")]
      pub enter_tapped: EtbTapState,
      pub enters_attacking: bool,
  }
  ```
  `object_id` is required. No legacy bool compat: a new type has no legacy writer.
- (b) In `PendingChangeZoneIteration`:
  - `remaining: Vec<ChangeZoneMemberEntry>`, with doc "unprocessed members with their pre-event entry modifications (CR 603.6a)".
  - Delete `enter_tapped`, `enters_attacking`, `enter_with_counters`, `conditional_enter_with_counters`, `enters_modified_if` and their docs.
  - Update the struct doc: the carrier stores resolved members, not the raw gate spec.
- (c) Tests: update the literal at about 35496 and the serde test (V4).

**2. `crates/engine/src/game/effects/change_zone.rs`**
- (a) Add next to `EntryGateContext`:
  - `pub(crate) struct ChangeZoneEntrySpec<'a> { entry: EntryGateContext, source_id: ObjectId, enter_with_counters: &'a [(CounterType, u32)], conditional_enter_with_counters: &'a [(TargetFilter, CounterType, QuantityExpr)], enter_tapped: EtbTapState, enters_attacking: bool, enters_modified_if: Option<&'a TargetFilter> }`
  - `pub(crate) fn resolve_change_zone_member_entry(state: &GameState, binding: Option<&ResolvedAbility>, spec: &ChangeZoneEntrySpec<'_>, object_id: ObjectId) -> ChangeZoneMemberEntry`:
    - counters = `binding.map_or_else(|| spec.enter_with_counters.to_vec(), |ability| enter_with_counters_for_entry(state, ability, object_id, Some(spec.entry), spec.enter_with_counters, spec.conditional_enter_with_counters))`
    - (tap, attacking) = `effective_enter_mods(state, object_id, spec.entry, spec.source_id, spec.enter_tapped, spec.enters_attacking, spec.enters_modified_if)`
  - `pub(crate) fn resolve_change_zone_member_entries(state, binding, spec, members: &[ObjectId]) -> Vec<ChangeZoneMemberEntry>`, which maps the single-member function.
  - Doc both: "CR 603.6a + CR 614.12: call before any member of the event moves; `state` is the pre-event board every member's projection reads". Note the `None` binding: EffectZoneChoice with no resolvable root keeps the unconditional counters, as today.
- (b) `ChangeZoneIterationCtx`: remove `enter_tapped`, `enters_attacking`, `enter_with_counters`, `conditional_enter_with_counters` (and its `allow(dead_code)`), `enters_modified_if`, with their docs. Update the struct doc: it mirrors `PendingChangeZoneIteration` minus `remaining`, `effect_kind` and the logical-group fields.
- (c) `process_one_zone_move` / `process_one_zone_move_with_terminal`:
  - New signature `(state, ctx, member: &ChangeZoneMemberEntry, events)`; `obj_id = member.object_id`.
  - Delete the `effective_enter_mods` call.
  - Pass `member.enter_tapped`, `member.enters_attacking`, `&member.enter_with_counters` to the pipeline.
  - Keep the CR 400.7/114.5/603.7c guards.
  - Comment: "CR 614.12: entry modifications were resolved before the event (`resolve_change_zone_member_entries`)".
- (d) Single-pick branches (about 1077 and 1171):
  - Build `ChangeZoneEntrySpec` from `dest_zone`, `enters_under_player`, `effect_enter_transformed`, `ability.source_id`, the effect slices, `effect_enter_tapped`, `effect_enters_attacking`, `effect_enters_modified_if.as_ref()`.
  - Replace the paired `enter_with_counters_for_object` + `effective_enter_mods` calls with `resolve_change_zone_member_entry(state, Some(ability), &spec, chosen)`, and feed its fields to `execute_zone_move_with_controller`.
  - Hoist the spec construction once above the branches; it borrows the effect slices.
- (e) Targeted loop:
  - After the Devour snapshot block and before the loop: `let members = resolve_change_zone_member_entries(state, Some(ability), &spec, &targeted_objects);` with comment "CR 603.6a + CR 614.12: one event — every member's gates read the board before any member moves."
  - Loop `for (i, member) in members.iter().enumerate()` with `let obj_id = &member.object_id`.
  - Delete `per_obj_ctx`; pass `&ctx` and `member`.
  - Both pause sites: `remaining: members[i + 1..].to_vec()` and drop the five removed fields.
  - `ctx` construction at about 1304: drop the five fields.
  - `allocate_logical_zone_change_group` still takes `&targeted_objects`.
- (f) `resolve_all` pause sites (about 2520 and 2584):
  - `remaining: matching[i + 1..].iter().map(|&object_id| ChangeZoneMemberEntry { object_id, enter_with_counters: enter_with_counters.clone(), enter_tapped, enters_attacking }).collect()`.
  - Keep the CR 122.1h comment.
  - Comment "ChangeZoneAll carries no entry gate, so every member's entry is the unconditional one".
  - Drop the five fields.
- (g) Delete `enter_with_counters_for_pending_object` and `EntryGateContext::from_iteration`, both dead. Keep `enter_with_counters_for_object` and `from_change_zone`: `cast_from_zone.rs` and tests use them.
- (h) Update the `entry_gate_projection` doc's consumer list.
- (i) Tests: add V1, V1b, V2, V3 and the graveyard shock helper. Rewrite V5. Update the REVERT text of `paused_multi_object_resume_projects_the_paused_move` to point at the EffectZoneChoice precompute's descriptor.

**3. `crates/engine/src/game/effects/mod.rs`**, `drain_pending_change_zone_iteration`:
- The destructure loses the five fields.
- Loop `for (i, member) in remaining.iter().enumerate()`.
- Delete the `enter_with_counters_for_pending_object` call; the context loses the five fields.
- `before_zone` / `anticipated_pause` / `record_delivery_completion` / moved-count use `member.object_id`.
- Call `process_one_zone_move_with_terminal(state, &ctx, member, events)`.
- Both re-parks: `remaining: remaining[i + 1..].to_vec()`, drop the five fields.
- Comment: "CR 603.6a + CR 614.12: each member applies the entry modifications resolved before the event's first member moved; never recompute against the partly-moved board."

**4. `crates/engine/src/game/engine_resolution_choices.rs`**
- `ChangeZone | BounceAll` arm:
  - After `allocate_logical_zone_change_group` and before `let logical_group_event_start`:
    ```rust
    let binding = effects::change_zone::resolving_stack_ability_for_source(state, source_id);
    let spec = ChangeZoneEntrySpec { entry: EntryGateContext { destination: dest_zone, controller_override: enters_under_player, enter_transformed }, source_id, enter_with_counters: &enter_with_counters, conditional_enter_with_counters: &conditional_enter_with_counters, enter_tapped, enters_attacking, enters_modified_if: enters_modified_if.as_ref() };
    let members = resolve_change_zone_member_entries(state, binding.as_ref(), &spec, &chosen_ids);
    ```
    Same CR comment as 2(e).
  - Iterate members; the per-member `origin` lookup uses `member.object_id`.
  - Delete the per-object `enter_with_counters_for_pending_object` call; the context loses the five fields.
  - Both pause sites: `remaining: members[i + 1..].to_vec()`, drop the five fields.
- PayCost arm:
  - The context loses the five fields.
  - Precompute `members` with the same helper (`binding: None`, empty counter slices, the round-trip `enter_tapped` / `enters_attacking` / `enters_modified_if`), which preserves today's values.
  - Iterate members; both pause sites use `members[i + 1..]`.

**5. `crates/engine/src/game/effects/search_outside_game.rs`**
- The context loses the five fields.
- Call `process_one_zone_move(state, &ctx, &ChangeZoneMemberEntry { object_id, enter_with_counters: Vec::new(), enter_tapped: EtbTapState::Unspecified, enters_attacking: false }, events)`.
- Comment: search-from-outside carries no entry modifications.

**6. `crates/engine/src/game/elimination.rs`**
- About line 907: `pending.remaining.retain(|member| member.object_id != identity.object_id)`. The batch-delivery `retain` at about 927 is a different type and stays.
- The test helper `pending_change_zone_iteration`: map `remaining: Vec<ObjectId>` to unconditional members and drop the five fields.

**7. `crates/engine/src/types/resolution.rs`**: test `change_zone_frame` literal (about 6027) drops the five fields. Keep `remaining: Vec::new()`.

**8. `crates/engine/src/game/filter.rs`**: `BattlefieldEntrantProjection` doc gains one sentence: "`state` must be the board before the entry event's first member moved (CR 603.6a); multi-object callers resolve every member first (`change_zone::resolve_change_zone_member_entries`)."

**9. Protocol bump**, full-game 148 / P2P wire 130, mirroring #10047's file set:
- `crates/lobby-broker/src/protocol.rs`: constant plus a `/// 148 — PendingChangeZoneIteration members carry their pre-event entry modifications (CR 603.6a + CR 614.12); old peers cannot parse a parked ChangeZone frame.` entry; the test pin at about 2369.
- `crates/server-core/src/protocol.rs`: test pin at about 3708.
- `client/src/adapter/ws-adapter.ts`: constant and history.
- `client/src/network/protocol.ts`: `WIRE_PROTOCOL_VERSION` and history.
- `client/src/network/__tests__/protocol.test.ts`.
- `client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`: previous/own wire.
- `client/src/services/__tests__/openPhaseSocket.test.ts`: accept 148, reject the previous version.
- `scripts/check-protocol-version.mjs`: v148 note, `+77` full, `+76` wire.
- Verify with `node scripts/check-protocol-version.mjs`.

**10. Verification**
- `cargo fmt --all`.
- Tilt is down in the planning environment. In the implementation environment, if Tilt is up, use `./scripts/tilt-wait.sh --timeout 600 clippy test-engine check-frontend test-frontend`. Otherwise run directly: `cargo clippy --all-targets -- -D warnings`, `cargo test -p phase-engine` (lib, plus integration `conditional_entry_gate_projection` / `etb_trigger_entry_time_characteristics` / `cr733*` / `committed_dump_walk`), and `pnpm --dir client run type-check && pnpm --dir client test -- protocol openPhaseSocket p2p-adapter-multiplayer`.
- Run each new test's REVERT mutation once locally to confirm it goes red. This also discharges the unprobed P1–P4.

## SCOPE_PATHS

```
crates/engine/src/types/game_state.rs
crates/engine/src/game/effects/change_zone.rs
crates/engine/src/game/effects/mod.rs
crates/engine/src/game/engine_resolution_choices.rs
crates/engine/src/game/effects/search_outside_game.rs
crates/engine/src/game/elimination.rs
crates/engine/src/types/resolution.rs
crates/engine/src/game/filter.rs
crates/lobby-broker/src/protocol.rs
crates/server-core/src/protocol.rs
scripts/check-protocol-version.mjs
client/src/adapter/ws-adapter.ts
client/src/network/protocol.ts
client/src/network/__tests__/protocol.test.ts
client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts
client/src/services/__tests__/openPhaseSocket.test.ts
```

Standing inclusion classes apply: compiler-forced sites from the `ChangeZoneIterationCtx` / `PendingChangeZoneIteration` / `process_one_zone_move*` signature changes, and comment-only edits. A grep found no `ChangeZoneIterationCtx`, `PendingChangeZoneIteration` or `process_one_zone_move` use outside these files except one doc comment in `tests/integration/issue_4235_cloak_and_dagger_entwined.rs`, which needs no change.

## Risks to flag to the orchestrator

- No probe could run here (no cargo, Tilt down). P1–P4 are traced, not measured. In particular, confirm that a graveyard-origin MayCost shock parks the targeted loop (P4) before relying on V3. If it does not, use the external tap-collision pair from `paused_face_down_change_zone_resumes_face_down_with_profile` as the pause source.
- Saves that hold a parked ChangeZone frame from before the bump are rejected, per the logical-group carrier precedent. The protocol bump keeps peers from cross-parsing.
