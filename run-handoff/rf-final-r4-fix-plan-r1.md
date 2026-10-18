# Plan: pin granted-static inner conditions per recipient in the CR 614.12 entry projection (r4, CodeRabbit finding)

BASE_SHA = START_SHA = a0e023a78. IMPLEMENTATION_WORKTREE /mnt/data/dev/wt/etb, branch engine/etb-entry-snapshot.

## Step 0: premise check
- **The finding is real and the PR introduced it.** At a0e023a78, `pin_entry_population_conditions` (layers.rs) rewrites:
  - object `base_static_definitions` / `static_definitions` conditions;
  - transient conditions;
  - CopyValues payload static conditions.
- It never walks into `ContinuousModification::GrantStaticAbility { definition }` bodies.
- `expand_granted_static_effects` clones `inner.condition` (`retained_inner_condition`) into each synthesized child with `source_id = recipient`. `apply_continuous_effect_filtered` then evaluates that condition on the scratch state through `active_effect_condition_controller`, i.e. with the recipient's current controller (CR 109.5).
- There are three expansion sites:
  - the printed static gather (`active_continuous_effects_from_static_definitions`);
  - the transient gather (`gather_transient_continuous_effects_with`);
  - `append_late_granted_static_effects`.
- **Probed red at a0e023a78** (throwaway worktree, since removed). Host text: `Each enchantment has "As long as you control a Shrine, this enchantment is a 1/1 creature in addition to its other types."`. It parses to a `GrantStaticAbility` whose inner has `AddType Creature`, `SetPower`/`SetToughness` 1 and condition `IsPresent{Shrine, You}`. Release Juno returns Sanctum of Tranquil Light (the only Shrine). Rows are (counters, creature after entry, P/T):
  - A: host P0, no other Shrine → `(2, true, 3/3)`. Expected 0.
  - B: host P0, P0 has a Shrine → `(2, true, 3/3)`. Expected 2.
  - C: host P1, only P1 has a Shrine → `(2, true, 3/3)`. Expected 0.
  - D: host P1, only P0 has a Shrine → `(2, true, 3/3)`. Expected 2.
  - E: host P1, no Shrines → `(2, true, 3/3)`. Expected 0.
- **Card text verified** (AtomicCards):
  - Sedge Sliver: `All Sliver creatures have "This creature gets +1/+1 as long as you control a Swamp."` This is the only printed population-conditioned granted inner, and it changes P/T only.
  - Sanctum of Tranquil Light: `{5}{W}: Tap target creature. This ability costs {1} less to activate for each Shrine you control.` It is a legendary enchantment, Shrine; the test file already carries the verbatim text.
- The host text is synthetic and labelled so; no printed card reaches this defect today.

## Step 1: skills
- `/add-static-ability`: this is the layer/continuous-effect pass. Checklist items that apply: condition evaluation, layer-6 grant, tests. No parser, frontend or AI change.
- `/card-test`: the integration rows use the GameScenario + `GameRunner::cast(..).resolve()` + `CastOutcome` recipe.
- `/add-engine-variant`: **not triggered.** No new enum variant on any engine type. The one new type is a module-private `enum PinnedHolder` inside layers.rs. It is a local helper, not engine surface, and is never serialized.

## Step 2: analogous trace
Traced the existing CR 614.12 pin and the CopyValues-payload carrier end to end:
- `filter::BattlefieldEntrantProjection::new` (filter.rs): `state.clone()` → `zones::move_to_zone_with_entry_flags` → optional transform / `apply_battlefield_entry_controller_override` → `layers::pin_entry_population_conditions(&mut scratch, state, Some(origin))` → `mark_layers_full` + `flush_layers(&mut scratch)`.
- `pin_entry_population_conditions` (layers.rs) → `pinned_definition_conditions` → `pin_population_leaves` / `pin_population_children` → `pinned_leaf_truth` → `evaluate_condition_with_context(pre_entry, leaf, controller, source, ConditionContext::ability_holder(controller))`.
- Then `write_pinned_conditions` with `Arc::make_mut`.
- Scratch flush: `flush_layers` → `evaluate_layers_with_retirement_owner` → `derive_layer_characteristics`.
  - Step 1 resets every battlefield object to base. `reset_recipient_to_base` sets `obj.controller = obj.base_controller.unwrap_or(obj.owner)`.
  - After layer 1, `gather_active_continuous_effects` → `collect_layer_pass_active_continuous_effects` → `collect_derivation_continuous_effects` (static + transient gathers, each calling `expand_granted_static_effects` over `state.battlefield`) → `append_late_granted_static_effects` (also over `state.battlefield`).
  - Layer loop → `apply_continuous_effect_filtered` → `active_effect_condition_controller` → `condition_controller` (CR 109.5: the source's current controller for a static).
  - Layer 6 `GrantStaticAbility` apply also installs a clone of the parent's `definition` onto the recipient's `static_definitions`.
- Consumer: `change_zone` entry-gate projection (change_zone.rs ~1674-1691) → `BattlefieldEntrantProjection::matches`.

## Step 3: files read
- crates/engine/src/game/layers.rs:
  - `ConditionContext` doc (~1377-1404);
  - `condition_controller`;
  - the pin block (~1598-1829);
  - `condition_uses_recipient_context`, `static_condition_uses_object_population`;
  - `GatherPass`, `collect_continuous_effects`, `active_continuous_effects_from_static_definitions`, `expand_granted_static_effects`, `append_late_granted_static_effects`, the transient gather;
  - the layer-6 `GrantStaticAbility` apply;
  - `evaluate_layers` and the Step 1 reset;
  - the `pin_entry_population_conditions_cases` unit test.
- crates/engine/src/game/filter.rs: `BattlefieldEntrantProjection` and its doc.
- crates/engine/tests/integration/conditional_entry_gate_projection.rs: helpers and every test.
- crates/engine/src/types/ability.rs: the `StaticCondition::SourceMatchesFilter` / `SourceControllerEquals` docs, `ContinuousModification::GrantStaticAbility { definition: Box<StaticDefinition> }`, `walk_self_and_granted`.

## Step 3.5: probes (all ran to completion in a throwaway worktree, since removed)
1. **Parser.** `Each enchantment has "…"` and `All enchantments have "…"` both parse to an unconditional host whose affected set is all enchantments (`controller: None`), with `GrantStaticAbility{ inner: AddType Creature, SetPower/SetToughness 1, condition IsPresent{Shrine, controller You} }`. `Enchantments you control have "…"` also produces a grant.
2. **Red at base.** Rows A–E above; the post-entry live reach held in every row.
3. **Prototype of the design below (green).** Rows: A=(0, true, 1/1), B=(2, true, 3/3), C=(0, true, 1/1), D=(2, true, 3/3), E=(0, true, 1/1).
   - Every existing `conditional_entry_gate_projection` test passed (17/17 including the probes).
   - `cargo test -p phase-engine --lib -- pin_entry_population entrant_projection projection` passed (61 tests, including `pin_entry_population_conditions_cases` after the refactor).
   - Positive marker: C and D each have recipients with differing truth (C: host and P1 Shrine true, Sanctum false; D: host false, P0 Shrine and Sanctum true). So the per-recipient (non-constant) branch was reached, not only the constant branch.
- **Not probed:** clippy/fmt on the prototype; a stolen-recipient (layer-2 control) row.

## Design decision: seam choice
The triage suggested threading pre-entry state into grant expansion. This plan instead pins per recipient inside `pin_entry_population_conditions`, before the scratch flush. It reaches the same per-recipient semantics, for two reasons that come from the code:

1. **Expansion-time pinning would bind "you" wrongly.** Expansion runs after the Step 1 reset (`obj.controller = base_controller.unwrap_or(owner)`) and before layer 2. So `recipient_controller` at expansion is the base controller. That is wrong for a recipient under a layer-2 control effect. The expansion's own comment says the condition is kept "until application, after preceding layers have established that controller". The pin runs before the flush on a scratch cloned from a settled board, so `obj.controller` is the last settled layer-2 controller, or the entry controller for the entrant (CR 110.2a). That is exactly the convention the existing object carriers already use.
2. **Expansion has no way to receive pre-entry state.** Expansion is reached only through `&GameState` gathers called from `flush_layers`. Every gather entry would need a new parameter, or the scratch `GameState` would need a projection-only field, or a thread-local ambient would be needed. Each option leaks projection state into the live layer machinery. Pinning before the flush keeps the invariant the pin already documents: "Only `scratch` is written". The live pass is unchanged by construction.

The scratch therefore encodes per-recipient pinned truth in the inner condition itself, using existing types:
- Inner children are synthesized with `source_id = recipient`, and every grant expansion candidate is a battlefield object (`expand_granted_static_effects` callers and `append_late_granted_static_effects` all draw from `state.battlefield`); a layer-6 install onto an off-battlefield object only contributes on a later layer pass, which a CR 614.12 projection's single flush never runs.
- So for each pinning leaf of a granted inner, compute `truth(r) = evaluate_condition_with_context(pre_entry, leaf, ctrl_scratch(r), r, ability_holder(ctrl_scratch(r)))` for every scratch battlefield object r.
- Rewrite the leaf to:
  - `pinned_static_condition(false)` if no r holds;
  - `pinned_static_condition(true)` if every r holds;
  - otherwise `StaticCondition::SourceMatchesFilter { filter: TargetFilter::Or { filters: [SpecificObject{r} for holding r] } }`.
- `SourceMatchesFilter` evaluates `matches_target_filter(state, source_id, …)` against the child's source, which is the recipient. `condition_uses_recipient_context` is false for `SpecificObject`, so `apply_continuous_effect_filtered` takes the non-recipient path.
- The same encoding is correct for the layer-6 installed copy and for nested grants, because "you" in any granted static is the controller of the object it is on (CR 109.5).

## Pattern Coverage
- **Class:** every `GrantStaticAbility` (printed static, transient, or inside a CopyValues payload static, at any nesting depth) whose inner condition has a population- or origin-zone-reading leaf. The projection must read those leaves at their CR 614.12 pre-entry truth, with each recipient's "you".
- **Printed cards today:** Sedge Sliver (P/T only, so no entry gate observes it). Any future "have '… as long as you control …'" grant joins the class automatically.
- This is a contract fix for the class, not for one card. It covers the same entry-gate consumers as the rest of the pin: Recommission, What Must Be Done, and every `conditional_enter_with_counters` / `enters_modified_if` gate.

## Sizing
- **Units:** 1.
  - U1 = "the CR 614.12 entry pin covers granted-static inner conditions per recipient". One `/add-static-ability` pass.
  - Registration surfaces: `layers::pin_entry_population_conditions` and its private helpers only.
  - Discriminating tests: `release_juno_granted_shrine_condition_counts_pre_entry_shrines` (row A) and `release_juno_granted_condition_binds_each_recipients_controller` (row C).
- **Dependency edges:** none.
- **Expected scope-path count (phase-fit rule): 2** (layers.rs, filter.rs). filter.rs is a comment-only edit. The integration test file is a test fixture and is excluded from the count.
- **SCOPE_PATHS:**
  - crates/engine/src/game/layers.rs
  - crates/engine/src/game/filter.rs
  - crates/engine/tests/integration/conditional_entry_gate_projection.rs

## Building Blocks
Reused, not duplicated:
- `pin_population_leaves` / `pin_population_children` (leaf type generalized, see Step 5);
- `pinned_static_condition`;
- `pinned_leaf_truth`, with its predicate factored into `leaf_pins`;
- `evaluate_condition_with_context` + `ConditionContext::ability_holder`;
- `condition_uses_recipient_context`, `static_condition_uses_object_population`, `static_condition_reads_zone_membership`;
- existing `StaticCondition::SourceMatchesFilter` and `TargetFilter::{Or, SpecificObject}`;
- `Arc::make_mut` copy-on-write;
- test helpers `board`, `add_what_must_be_done`, `add_graveyard_sanctum`, `set_core_types`, `cast_release_juno`, `is_creature`, `put_counter_resolved`; scenario builder `add_enchantment_from_oracle(..).with_subtypes(..)`.

New private helpers, all justified as the one generalization of the existing definition pin:
- `enum PinnedHolder`;
- `struct EntryPopulationPin`;
- `EntryPopulationPin::{condition, definition, grants, definitions}`;
- `write_pinned_definitions`, which replaces `pinned_definition_conditions` / `write_pinned_conditions`.

No parallel pin.

## Logic Placement
- Engine `game/layers.rs` only: the pin is layer-system scratch preparation.
- filter.rs gets a doc sentence only.
- No parser, frontend or AI change.

## Rust Idioms
- `PinnedHolder { Carrier { controller, source_id }, EachRecipient }` instead of a bool, which keeps the two "you" bindings explicit.
- `match` on `ContinuousModification` uses an explicit `_ => None`. This is a COW rewrite selector over a large enum where only two variants carry statics; it mirrors the existing `let … else continue`.
- Copy-on-write `Option` returns. Whole-definition replacement keeps untouched `Arc`s shared.
- No new engine enum variants.

## Nom Compliance
N/A. No parser file changes.

## Extension vs Creation
This extends the existing pin. The definition-level rewrite changes from "condition only" to "condition plus grant bodies, recursively", and the transient loop gains a `GrantStaticAbility` arm beside the existing `CopyValues` arm. No new pattern.

## Analogous Trace
Traced the CopyValues-payload carrier: `pin_entry_population_conditions` → `pinned_definition_conditions` → `pinned_leaf_truth` → `evaluate_condition_with_context`. Path: filter.rs `BattlefieldEntrantProjection::new` → layers.rs pin → `flush_layers` → `derive_layer_characteristics` → `collect_derivation_continuous_effects` → `expand_granted_static_effects` → `apply_continuous_effect_filtered` → `active_effect_condition_controller`.

## Variant Discoverability
No engine variant is added. `/add-engine-variant` is not triggered.

## Identity / Provenance Contract
- **"You" in a granted static** (CR 109.5: "For a static ability, this is the current controller of the object it's on"; CR 113.1a: abilities granted to objects).
  - Authority: each recipient's `PlayerId`, read as `scratch.objects[r].controller` at pin time. That is the last settled layer-2 controller, or the entry controller for the entrant (CR 110.2a).
  - Binding time: when the projection is built, before the scratch flush.
  - Semantics: snapshotted onto the scratch only.
  - Storage: the rewritten inner `StaticDefinition.condition` inside the scratch's `GrantStaticAbility`.
  - Consumer: `apply_continuous_effect_filtered` via the child's `source_id = r`.
  - Invalidation: the scratch is dropped with the projection.
  - Hostile multi-authority fixture: rows C and D (host controller ≠ recipient controller).
- **The pinned truth's population** (CR 614.12 + CR 700.5 + the Heliod ruling 2020-01-24) is `pre_entry`, the board before the entry event. Same as the existing pin.
- **The carrier "you" for non-grant conditions** is unchanged: an object's scratch controller; for a transient, `tce.controller`; for a CopyValues payload, the copy recipient's scratch controller.

## Reference Readings
- Row B and row D (expected 2 counters) are derived, not parity values. Derivation for B:
  - Each enchantment has the granted static.
  - Sanctum would be an enchantment on the battlefield, so it would have it.
  - "You" is P0, and P0 controls "P0 Shrine" before entry.
  - So Sanctum is a 1/1 creature as it would enter, the "if it's a creature" gate holds, and it gets two counters: 3/3.
- Row D is the same, with the P1 host irrelevant to "you".
- Rows A, C and E are derived as 0 (P0 controls no Shrine before entry; the entrant's own Shrine subtype isn't counted, CR 614.12 / Heliod ruling). Each has the post-entry live reach: Sanctum is a 1/1 creature after entry, because P0 now controls a Shrine.
- No row copies another route's reading.
- Existing rows that assert preservation (all prior conditional_entry_gate_projection tests and `pin_entry_population_conditions_cases`) were measured green on the prototype. Their expected values were derived in earlier rounds and are unchanged.

## Verification Matrix
All new tests are red at a0e023a78 unless marked green-at-base.

V1. Row A, gate false, pre-entry truth.
- Seam: `pin_entry_population_conditions` grant arm.
- Entry: `BattlefieldEntrantProjection::new` via Release Juno.
- Test: new `release_juno_granted_shrine_condition_counts_pre_entry_shrines`, case `(other_shrine=false)`.
- Assertions:
  - Sanctum on the battlefield with 0 +1/+1 counters;
  - no `PutCounter` resolved;
  - reach-guard: `is_creature` is true and P/T is 1/1 after entry. This is the live-pass guard: the live pass still evaluates the inner condition live.
- Red at base: 2 counters, 3/3.

V2. Row B, positive control, same test, case `(other_shrine=true)`.
- Assertions: 2 counters, 3/3, creature. Green at base; guards against an over-pin.

V3. Rows C and D, per-recipient controller.
- Test: new `release_juno_granted_condition_binds_each_recipients_controller`.
- Assertions:
  - (host P1, P1 Shrine only) → 0 counters, creature 1/1 after entry. Red at base: 2.
  - (host P1, P0 Shrine only) → 2 counters, 3/3. Green at base.
  - (host P1, no Shrines) → 0 counters, creature 1/1 after entry. Red at base: 2.
- Hostile: a pin keyed to the host's controller fails both the first and second rows. A single per-definition constant pin cannot satisfy both the first row and the P1-host recipients. Both rows reach the non-constant per-recipient branch (measured).

V4. Live pass unchanged.
- Reach-guards in V1 and V3 (creature 1/1 after entry). Add one explicit live assertion in V1: after resolution, the host and Sanctum are both creatures, i.e. the live layer pass applies the granted condition with the live population.
- `recommission_heliod_four_devotion_still_entered_as_a_creature` remains a live-leak guard.

V5. Building-block unit test.
- Extend `pin_entry_population_conditions_cases` in the layers.rs tests with two carriers:
  - (i) a P1-controlled battlefield object whose `base_static_definitions` / `static_definitions` hold a `GrantStaticAbility` whose inner condition is `you_control_a_creature()`;
  - (j) a transient `GrantStaticAbility` (controller P1) with the same inner.
- Recipients: P0's anchor (P0 had no creature pre-entry, false) and P1 objects (true).
- Assert behaviorally, independent of the encoding: for each scratch battlefield object r, `evaluate_condition(&scratch, &pinned_inner, scratch.objects[&r].controller, r)` equals that object's controller's pre-entry truth.
- Assert the host's own `def.condition` (None) stays None.
- Add a grant with a non-population inner (`SourceIsTapped`) and assert untouched plus `Arc::ptr_eq` on that carrier.
- Assert that the existing assertions (a)–(h) still hold unchanged. Their definitions now come back as whole-definition rewrites; the assertions compare conditions only.
- Red at base: the inner condition stays `IsPresent` and evaluates on the scratch, where P0 has "Scratch Bear", so the P0 recipient reads true.

Hostile, unreachable or empty:
- No recipients (empty battlefield) → no children are synthesized, so the encoding is irrelevant.
- A recipient-context inner leaf (`RecipientMatchesFilter`) is excluded by `leaf_pins` (covered by the unit test's existing (c)-style untouched assertion; add a granted variant to (V5)).
- A CopyValues payload with no single bound recipient stays wholly unpinned, as the existing documented rule says. No card reaches it.

Coverage status impact: none (no parser change).

**REVERT notes** (append to the file header's REVERT DISCRIMINATORS):
"Drop the pin's `GrantStaticAbility` arm (`EntryPopulationPin::grants`) and `release_juno_granted_shrine_condition_counts_pre_entry_shrines` gets 2 counters with Sanctum as the only Shrine (expected 0), and `release_juno_granted_condition_binds_each_recipients_controller` gets 2 in its P1-Shrine-only and no-Shrine rows (expected 0). Pin the granted inner with the host's controller instead of each recipient's and the P1-Shrine-only row gets 2 (expected 0) and the P0-Shrine-only row gets 0 (expected 2)."

Add the matching REVERT line in the V5 unit-test doc.

## Step 5: implementation steps

1. **crates/engine/src/game/layers.rs, pin block (from `pin_population_leaves` through the end of `pin_entry_population_conditions`).**
   - Generalize `pin_population_leaves` / `pin_population_children` to take `pin_leaf: &impl Fn(&StaticCondition) -> Option<StaticCondition>`. The leaf arm returns `pin_leaf(leaf)`. Carrier callers wrap with `.map(pinned_static_condition)`.
   - Factor `leaf_pins(leaf, origin) -> bool` out of `pinned_leaf_truth`: not recipient-context, and (object population, or origin-zone membership). `pinned_leaf_truth` becomes `leaf_pins(..).then(|| evaluate_condition_with_context(..))`.
   - Add the private `enum PinnedHolder { Carrier { controller: PlayerId, source_id: ObjectId }, EachRecipient }`. Doc: CR 109.5. The carrier's "you" is fixed; a granted static's "you" is the controller of each object it is on.
   - Add the private `struct EntryPopulationPin<'a> { pre_entry: &'a GameState, origin: Option<Zone>, grant_recipients: Vec<(ObjectId, PlayerId)> }`. `grant_recipients` is `scratch.battlefield` with scratch controllers. Doc: every grant expansion candidate is a battlefield object (`expand_granted_static_effects` callers and `append_late_granted_static_effects` all draw from `state.battlefield`); a layer-6 install onto an off-battlefield object only contributes on a later layer pass, which a CR 614.12 projection's single flush never runs.
   - Methods on `EntryPopulationPin`:
     - `condition(&self, &StaticCondition, PinnedHolder) -> Option<StaticCondition>`. `Carrier` uses `pinned_leaf_truth` → `pinned_static_condition`. `EachRecipient` uses `leaf_pins` then the per-recipient truth table → `pinned_static_condition(false)` when none hold, `pinned_static_condition(true)` when all hold, else `SourceMatchesFilter { filter: TargetFilter::Or { filters: SpecificObject… } }`. Put this in a small `fn pinned_recipient_condition(holding: Vec<ObjectId>, recipient_count: usize) -> StaticCondition` beside `pinned_static_condition`, with a doc noting it is scratch-local and that the child's source is its recipient.
     - `definition(&self, &StaticDefinition, PinnedHolder) -> Option<StaticDefinition>`: own condition under `holder`, plus `grants(&def.modifications)`; copy-on-write.
     - `grants(&self, &[ContinuousModification]) -> Option<Vec<ContinuousModification>>`: each `GrantStaticAbility { definition }` → `definition(definition, EachRecipient)`, recursing through nested grants; copy-on-write.
     - `definitions(&self, &[StaticDefinition], PinnedHolder) -> Vec<(usize, StaticDefinition)>`.
   - Replace `pinned_definition_conditions` / `write_pinned_conditions` with `write_pinned_definitions(defs: &mut [StaticDefinition], Vec<(usize, StaticDefinition)>)`.
   - In `pin_entry_population_conditions`:
     - Build `EntryPopulationPin` first.
     - Object carriers use `pin.definitions(.., Carrier { obj.controller, id })` for base (via `Arc::make_mut`) and live (`static_definitions.get_mut(index)` whole-def replacement).
     - The transient loop keeps the condition pin with `Carrier { tce.controller, tce.source_id }`. Per modification:
       - `GrantStaticAbility` → `pin.definition(definition, EachRecipient)`;
       - `CopyValues` → only with a bound recipient (unchanged rule), `pin.definitions(&values.static_definitions, Carrier { recipient_controller, recipient })`, written into a cloned modification via `Arc::make_mut`;
       - collected as `(tce_index, modification_index, ContinuousModification)` and written after the loop.
     - The prototype of exactly this shape compiled and passed (Step 3.5 probe 3).
   - Update the `pin_entry_population_conditions` doc "Carriers" list. Add: "every `GrantStaticAbility` body any of these carry, at any depth, with each recipient's scratch controller as "you" (CR 109.5: the controller of the object the granted static is on), encoded per recipient because one grant fans out over recipients with different controllers. Pinned before the flush because the gather runs after the CR 613.1 Step 1 controller reset and before layer 2 (CR 613.1b), so an expansion-time pin would read the base controller."
   - Annotate the new per-recipient arm with `// CR 614.12 + CR 109.5: …`.

2. **crates/engine/src/game/layers.rs, `ConditionContext::ability_holder` doc (~1395-1400).** Extend the sentence: "…for a printed or copied static, each recipient's scratch controller for a granted static's condition, and the effect's controller for a transient (CR 109.5)."

3. **crates/engine/src/game/layers.rs, `expand_granted_static_effects` comment at `retained_inner_condition`.** One sentence: in a CR 614.12 projection scratch the condition arrives already pinned per recipient by `pin_entry_population_conditions`.

4. **crates/engine/src/game/filter.rs, `BattlefieldEntrantProjection` doc (comment only).** Change "(printed on any object, copied by a layer-1 copy effect)" to "(printed on any object, copied by a layer-1 copy effect, or granted by a `GrantStaticAbility`, per recipient)".

5. **crates/engine/src/game/layers.rs tests.** Extend `pin_entry_population_conditions_cases` per V5, including its REVERT note.

6. **crates/engine/tests/integration/conditional_entry_gate_projection.rs.**
   - Add the const `SHRINE_GRANT_HOST` (synthetic, labelled, no printed card): `Each enchantment has "As long as you control a Shrine, this enchantment is a 1/1 creature in addition to its other types."`.
   - Add the helper `release_juno_under_shrine_grant(host_controller, p0_shrine, p1_shrine) -> (CastOutcome, ObjectId)`. Use `board()`, `add_what_must_be_done`, `add_graveyard_sanctum`, `add_enchantment_from_oracle(host_controller, "Shrine Grant Host", SHRINE_GRANT_HOST)`, Shrine anchors via `add_enchantment_from_oracle(P, "<P> Shrine", "").with_subtypes(vec!["Shrine"])`, `set_core_types(.., &[CoreType::Enchantment])`, `cast_release_juno`.
   - Premise assertion: the host's parsed static is a `GrantStaticAbility` whose inner condition is `Some(_)`.
   - Add the tests `release_juno_granted_shrine_condition_counts_pre_entry_shrines` (V1/V2/V4) and `release_juno_granted_condition_binds_each_recipients_controller` (V3).
   - Extend the header's REVERT DISCRIMINATORS with the text above.

7. Run `cargo fmt --all`. Verify through Tilt `test-engine` + `clippy` when Tilt is up (`./scripts/tilt-wait.sh clippy test-engine`). Otherwise use an isolated target dir with `cargo test -p phase-engine --test integration conditional_entry_gate_projection` and `cargo test -p phase-engine --lib pin_entry_population`, plus clippy on phase-engine.

**CRs verified by grep** (/mnt/data/dev/phase-personal/docs/MagicCompRules.txt): 614.12, 109.5, 113.1a, 113.3d, 113.7, 604.1, 611.2c, 613.1b, 613.1f, 613.8a, 700.5, 603.10, 611.2e.

**C1 (CR 611.2e onset deferral) is not foreclosed.** The change is confined to the projection scratch pin and touches no onset or timestamp path.