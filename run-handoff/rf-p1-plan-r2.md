# Phase 1 plan: entry projection population, object-scoped keyword kinds, incarnation fallback, any-of entry gate, settled harness (etb-review-fixes, phase index 1), revision r2

- **Mode:** `engine-planner` phase-plan mode. Charter: `.git/engine-implementer-runs/etb-review-fixes/phase-charter`, phase 1. Deferral allowlist: all C1 content is `DEFERRED(phase 2)`.
- **PHASE_BASE_SHA:** `addbc884ee251794a64f2e1da0487cd329f8ce90`. The main checkout is at this commit and was not edited (`git status --short` empty before and after). r2 probes ran in a detached scratch worktree (Step 3.5).
- **Loop position:** this revision is plan-loop design round 3 of 5. It revises `rf-p1-plan-r1.md` under `rf-p1-plan-review-r1.md` (behavior F1–F4, text F5–F6, one non-blocking note applied). The earlier constraints from `rf-plan-review-r1.md` (F3, F4, F5, F9, F7/F8) still bind and are kept. Budget: standard.
- **Addenda file:** `.git/engine-implementer-runs/etb-review-fixes/addenda/phase-1`.
- **Evidence labels.** **[probed]** = measured by a command named in Step 3.5. **[read]** = established by reading code at PHASE_BASE_SHA. **[unprobed]** = a claim the executor confirms before relying on it.

---

## Changes from r1

| Review finding | Resolution in r2 |
|---|---|
| F1 (MED) copied statics escape the pin | U1 carrier set gains the static definitions latched in every scratch transient's `ContinuousModification::CopyValues { values }` payload (copy-on-write, same leaf rule, same constructor), with the copy recipient's scratch controller as "you" (the copier is the copied static's source, CR 109.5 + CR 707.2 + CR 613.1a). New discriminating row V5b (Copy Enchantment copying Starfield of Nyx, Release Juno on Sanctum of Tranquil Light): base 2 counters, derived 0 [probed P13]; positive control 2 → 2. V6a case (h). R5(c) rewritten with the true mechanism. |
| F2 (LOW) command-zone pin is a no-op; graveyard statics missed | U1 takes printed carriers from **every object in `scratch.objects`** and writes the pinned tree to **both** `static_definitions` (what `static_source_effects_in_pass` reads, never reseeded off the battlefield) and `base_static_definitions` (what the battlefield reseed copies). The "exactly the population" sentence is removed. New V6a cases (f) command-zone emblem and (g) graveyard carrier; new runtime row V5c (an emblem carrying Starfield's static): base 2, derived 0 [probed P14]. |
| F3 (LOW) `ptr::eq` drops off-zone grants on the owner-zone clone | `std::ptr::eq` is gone. Every `filter_inner_for_object` door now declares what it holds with a typed `keywords::HeldObject { Live, Detached }`; the owner-zone clone is `Live` (a view of the live object rescoped only in controller). The disposition row is corrected, the U2 residual deleted, and V9b added: Divination in the graveyard, given flashback by Snapcaster Mage's resolved trigger, with `controller != owner`, answers `HasKeywordKind(Flashback)` true through `matches_target_filter_in_owner_zone` [probed P15: base true; r1's address test false, because the grant reaches the card only through the off-zone path]. (Measured nuance: the review's "the layer pass never materializes graveyard keywords" does not hold for an `InZone Graveyard` grant such as Lier's, which is materialized and so cannot discriminate; an identity-targeted grant such as Snapcaster's is not.) The doc names the rescoped-view rule (review non-blocking note). |
| F4 (LOW) L1/C5 has no red-at-base test | `drive_to_empty` delegates to `drive_to_empty_within(runner, pick, branch, max_steps)` and asserts the **postcondition** (Priority on an empty stack) after the loop. New `#[should_panic(expected = "did not reach Priority on an empty stack")]` row V20 on a committed, unresolved spell with `max_steps = 0`, plus the settled-board sibling V20b [probed P16: unasserted harness returns silently, asserted panics, settled board does not panic]. |
| F5 (text) | Applied verbatim to the Reference Readings V11 bullet. |
| F6 (text) | Applied verbatim to Residuals R5(d). |
| Non-blocking: `ability_holder` controller for transients | The `ConditionContext::ability_holder` doc sentence now states that the pin binds a transient's "you" to the effect's controller (CR 109.5), and that this deliberately differs from the live pass's `FilterContext::from_source` reading when the source changed controllers. |

Everything the review did not fault is kept from r1 (U6 → U2 → U3 → U1 → U5 → C6 order, V1–V4, V5, V6b–V6d, V7, V8, V10, V11, V18, V19, the C6 comment).

---

## Step 0: premise (MTGJSON `/home/user/wt/data-root/mtgjson/AtomicCards.json`, re-read for this revision) [probed]

| Card | Verbatim Oracle text (relevant clause) | Used by |
|---|---|---|
| Heliod, Sun-Crowned {2}{W} | "As long as your devotion to white is less than five, Heliod isn't a creature." and "Whenever you gain life, put a +1/+1 counter on target creature or enchantment you control." Rulings 2020-01-24: (a) "As a God enters the battlefield, your devotion to its color will determine whether any replacement effects that affect creatures entering the battlefield apply to that God. Because replacement effects are considered before the God is on the battlefield, the mana symbols in its mana cost won't be counted when determining this." (b) "When a God enters the battlefield, your devotion to its color (including the mana symbols in the mana cost of the God itself) will determine if a creature entered the battlefield or not for abilities that trigger whenever a creature enters the battlefield." | B1 (V1–V3, V6a) |
| Recommission {1}{W} | "Return target artifact or creature card with mana value 3 or less from your graveyard to the battlefield. If a creature enters this way, it enters with an additional +1/+1 counter on it." | B1 |
| What Must Be Done {3}{W}{W}, mode **Release Juno** | "Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature." (F7 phase-1 correction: the mode is "Release Juno", not "Let the Past Return".) | B1 (V4, V5, V5b, V5c), C4 template |
| Rusted Relic {4} | "Metalcraft — This artifact is a 5/5 Golem artifact creature as long as you control three or more artifacts." (F7 verbatim) | B1 (V4, V6b) |
| Starfield of Nyx {4}{W} | "As long as you control five or more enchantments, each other non-Aura enchantment you control is a creature in addition to its other types and has base power and base toughness each equal to its mana value." | B1 group static (V5, V5b, V5c) |
| Copy Enchantment {2}{U} | "You may have this enchantment enter as a copy of any enchantment on the battlefield." | F1 (V5b) |
| Sanctum of Tranquil Light {W} | Legendary Enchantment — Shrine: "{5}{W}: Tap target creature. This ability costs {1} less to activate for each Shrine you control." | V5, V5b, V5c entrant; V19 case 3 |
| Snapcaster Mage {1}{U} | Creature — Human Wizard 2/1: "Flash / When this creature enters, target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost. (You may cast that card from your graveyard for its flashback cost. Then exile it.)" | F3 (V9b) |
| Lier, Disciple of the Drowned {3}{U}{U} | "… Each instant and sorcery card in your graveyard has flashback. The flashback cost is equal to that card's mana cost." | P15 control fixture only (its grant is materialized, so it cannot discriminate) |
| Divination {2}{U} | Sorcery: "Draw two cards." | F3 (V9b), V20 fixture |
| Elemental Bond {2}{G} | "Whenever a creature you control with power 3 or greater enters, draw a card." | V3 (F4 of rf-plan-review-r1), V19 |
| Soul Warden {W} | "Whenever another creature enters, you gain 1 life." | Not used as a fixture (confound, measured P1d) |
| Memnite {0} | Artifact Creature — Construct 1/1, no rules text | V19 case 1 |
| Sol Ring {1} | "{T}: Add {C}{C}." | V19 case 2 |
| Heroic Return; Winter Soldier, Reborn Avenger; Summoner's Grimoire; Oviya, Automech Artisan | gate consumers, each moving one object | multi-entrant claim (P7) |

The PR review's quotes and both Heliod rulings match MTGJSON. The premise holds.

**CR numbers, each grep-verified in `docs/MagicCompRules.txt`** [probed]: 614.12, 614.12a, 614.1c, 700.5, 109.5, 110.2a, 613.1, 613.1a, 603.10, 603.10a, 603.6a, 608.2h, 608.2i, 608.2c, 611.2c, 611.3c, 400.3, 400.7, 702.26b, 702.34a, 707.2, 708.2a, 111.3, 113.6, 114.4, 508.1a, 122.6. (Command: `grep -n "^<rule>\.\? " docs/MagicCompRules.txt` per rule.)

## Step 1: skills (checklist skeleton)

| Skill | Phase-1 content | Later-phase content |
|---|---|---|
| `/add-static-ability` | U1: condition evaluation for the scratch layer pass (`layers.rs`), including the layer-1 copy payload and off-battlefield carriers | — |
| `/add-replacement-effect` | U1: the CR 614.12 entrant projection; U5: the any-of entry gate read by `entry_gate_matches` | — |
| `/add-trigger` | U2: keyword-kind reads on the entry-observation matcher and record matchers; U3: the no-observation fallback | `DEFERRED(phase 2)`: the D1 deferral predicate and its walkers |
| `/oracle-parser` | U5: slice-pattern extension of `card_type_condition_as_entry_this_way` (no new text matching) | `DEFERRED(phase 2)`: the onset produced by predicate dispatch |
| `/add-engine-variant` | Not triggered: no variant is added to an existing enum (U1 uses existing `StaticCondition::None`; U5 uses existing `TypeFilter::AnyOf`). U2 creates one crate-private, non-serialized two-value enum `keywords::HeldObject`; Stage 1 existence check run anyway (see Variant Discoverability). | `DEFERRED(phase 2)`: the onset enum and field gate |
| `/card-test` | every cast-pipeline row (GameScenario + `GameRunner::cast(..).resolve()`, verbatim Oracle text, `CastOutcome` deltas) | `DEFERRED(phase 2)`: C1 rows |

Checklist steps that do not apply in phase 1: types/serde (no serialized type change), frontend, AI, multiplayer filter, `WaitingFor`/`GameAction`.

## Step 2: analogous traces

- **U1 (condition evaluated against a population other than the scratch's).** `filter.rs::BattlefieldEntrantProjection::new` → `zones::move_to_zone_with_entry_flags` → `zones::apply_battlefield_entry_controller_override` → `layers::mark_layers_full` / `layers::flush_layers` → `derive_layer_characteristics` → `reset_recipient_to_base` (battlefield phased-in ids only) → `seed_live_characteristics_from_base` (`obj.static_definitions = Arc::clone(&obj.base_static_definitions).into()`) → layer 1: `ContinuousModification::CopyValues` → `printed_cards::apply_copiable_values` (`obj.static_definitions = Arc::clone(&values.static_definitions).into()`, then the generator index is refreshed so copied statics generate effects in the same pass) → `for_each_static_effect_source` (battlefield sources, command-zone sources via `object_sources_static_from_command_zone`, then every other object carrying an `active_zones` or CDA static, e.g. the Incarnations) → `static_source_effects_in_pass` (reads `source.static_definitions`) / `gather_transient_continuous_effects_with` → condition evaluated with `active_effect_condition_controller` → `condition_controller` → `evaluate_condition_with_context` → `evaluate_condition_inner` (`DevotionGE` → `devotion::count_devotion(state, controller, ..)`; `QuantityComparison` → `quantity::resolve_quantity_with_ctx(state, .., controller, ..)`; `IsPresent` → `ability_holder` or `FilterContext::from_source(state, source_id)`). Classifiers reused: `layers::static_condition_uses_object_population`, `layers::static_condition_reads_zone_membership`, `layers::condition_uses_recipient_context`. Copy installers read: `effects/become_copy.rs` (binds `affected = SpecificObject { id: recipient }` and `affected_recipient`), `merge.rs::install_merge_layer_effect` (`SpecificObject { id: target }`).
- **U2 (keyword reads on a held object).** `filter.rs::matches_filter_prop`: `FilterProp::WithKeyword` → `obj.has_keyword` (object-scoped) against `FilterProp::HasKeywordKind` / `WithoutKeywordKind` → `keywords::object_has_effective_keyword_kind(state, object_id, ..)` (id-keyed: live battlefield keywords, else `off_zone_characteristics::off_zone_has_keyword_kind`). Door-declared typed modes traced as precedent: `layers::StaticZoneAdmission { LiveSource, PreFilteredBaseStatic }` (a caller says what screening it already applied) and `triggers::TriggerSource { Live, Context }`. Callers enumerated in U2's disposition table.
- **U3 (incarnation convention).** `types/game_state.rs::GameState::battlefield_entry_observation` (`record.entered_incarnation.is_none_or(|incarnation| row.identity.reference.incarnation == incarnation)`), `zones::settle_battlefield_entry_observations` (same convention), consumed at `trigger_matchers::zone_change_clause_matches` → `filter::matches_target_filter_on_entry_observation`.
- **U5 (anaphoric entry-gate fold).** `conditions.rs::anaphoric_entry_counter_gate` → `card_type_condition_as_entry_this_way` (also consumed by `sequence.rs::is_anaphoric_entry_counters_clause`) → `lower.rs::fold_enters_this_way_counter_rider` → `change_zone.rs::enter_with_counters_for_object` → `entry_gate_matches` → `BattlefieldEntrantProjection::matches`.
- **U6.** `etb_trigger_entry_time_characteristics.rs::drive_to_empty` and its callers (`glider_with_tempest` → `r5b_glider_flying_counter_gives_haste`; `search_then_counters` → `r5d_search_bears`, `r5d_search_giant_sibling`; `r11_clear_the_land`; `r12_path_of_the_schemer`). The `CastCommit` door (`SpellCast::commit`, "Drive the cast pipeline until the spell is committed to the stack and a priority window opens") is the fixture source for an unsettled runner.

## Step 3: files read (all at PHASE_BASE_SHA)

`game/filter.rs` (the projection, `matches_target_filter_on_entry_observation`, every `filter_inner_for_object` caller, `matches_target_filter_in_owner_zone`, `matches_target_filter_for_zone`, `ControllerLookup`, `effective_controller`, `matches_filter_prop`, `SourceContext`, `entry_filter_matches_staged`, `matches_target_filter_on_battlefield_entry`, `mod entry_observation_tests`, `battlefield_entrant_projection_cases`), `game/layers.rs` (`ConditionContext`, `evaluate_condition*`, `condition_controller`, `active_effect_condition_controller`, the three classifiers, `derive_layer_characteristics`, `reset_recipient_to_base`, `seed_live_characteristics_from_base`, `for_each_static_effect_source`, `static_source_effects_in_pass`, `gather_transient_continuous_effects_with`, `transient_effect_passes_other_gates`, `state_duration_subject`, the layer-1 `CopyValues` arm), `types/definitions.rs` (`Definitions::get_mut`, `iter_all`, `as_slice`; read-only), `types/game_state.rs` (`TransientContinuousEffect`; read-only), `types/ability.rs` (`ContinuousModification::CopyValues`, `CopiableValues::static_definitions`; read-only), `game/effects/become_copy.rs` and `game/merge.rs` (CopyValues installers; read-only), `game/functioning_abilities.rs` (`object_sources_static_from_command_zone`; read-only), `game/keywords.rs`, `game/off_zone_characteristics.rs` (read-only), `game/devotion.rs` (read-only), `game/quantity.rs` (read-only), `game/zones.rs` (`apply_battlefield_entry_controller_override`, `create_object`; read-only), `game/zone_pipeline.rs` (`enters_with_additional_counters_for_entry`, the only production caller of `matches_target_filter_on_object`; read-only), `game/effects/change_zone.rs`, `game/engine_replacement.rs` (read-only), `game/trigger_matchers.rs` (read-only), `game/scenario.rs` (`SpellCast::commit`, `accept_optional`, `replacement_choice`, `copy_target`; read-only), `parser/oracle_effect/conditions.rs`, `parser/oracle_effect/lower.rs` (read-only), `parser/oracle_effect/tests.rs`, both integration test files.

## Step 3.5: probes

**r1 probes (kept).** Harness: a temporary integration module `zz_p1_probe.rs` (kept at `scratchpad/p1r/zz_p1_probe.rs`) plus a temporary `eprintln!` in `drive_to_empty`. Every probe ran to `test result: ok` with a positive marker.

**r2 probes (new).** Harness: `scratchpad/p1r2/zz_p1r2_probe.rs`, installed as `mod zz_p1r2_probe;` in a detached worktree `git worktree add --detach scratchpad/p1r2-wt addbc884…`, built and run with `CARGO_TARGET_DIR=scratchpad/p1r2-target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test -p phase-engine --features test-support --test integration -- zz_probe_r2 --nocapture --test-threads=1` (crates.io/npmjs stripped from `no_proxy`). Both runs (`zz_probe_r2` and the follow-up `zz_probe_r2_f3b`) ended `test result: ok` with exit 0, and every probe printed its positive marker (listed per row). The worktree and its target dir were removed afterwards (`git worktree remove --force`); the main checkout was never touched.

| # | Probe | Result at base | Decision it informs |
|---|---|---|---|
| P1 | Heliod via Recommission, P0 anchors {W}{W}×2 (pre-entry devotion 4) [probed] | 1 counter, creature, 6/6 | V1 is red at base (derived 0) |
| P1b | same, {W}{W}×2 + {W}×1 (pre-entry devotion 5) [probed] | 1 counter | V2 positive control equals base |
| P1c | devotion 4 + Elemental Bond [probed] | 1 counter, draws 1 | V3 observer fires on Heliod |
| P1d | devotion 4 + Soul Warden [probed] | **2 counters**, life +1, a `PutCounter` resolved | confound confirmed: Soul Warden rejected as V3's observer; Elemental Bond (no life gain) is used |
| P1e | Heliod via Recommission, no anchors [probed] | 0 counters, not a creature | existing rows unchanged |
| P2 | Rusted Relic via Release Juno, 2 / 3 other artifacts [probed] | 2 counters (7/7 creature) / 2 counters | V4: red at 2 others (derived 0), control at 3 |
| P3 | Starfield of Nyx + Sanctum via Release Juno, Starfield + 3 / + 4 other enchantments, anchors with mana value 1 [probed] | 2 counters (3/3) / 2 counters (3/3) | V5: red at 4 pre-entry (derived 0), control at 5. Anchors need mana value ≥ 1 (MV-0 anchors die as 0/0 creatures) |
| P4 | Synthetic "Put target creature/artifact card from an opponent's graveyard onto the battlefield under your control. If a creature enters this way, it enters with an additional +1/+1 counter on it." [probed] | Parses to `ChangeZone{enters_under: Some(You), conditional_enter_with_counters: [(Creature, P1P1, 1)]}`. Heliod owned by P1 (P0 devotion 4, P1 6): 1 counter. Rusted Relic owned by P1 (P0 2 artifacts, P1 3): 1 counter | V6c/V6d red at base (derived 0) |
| P5 | Release Juno with "… if it's an artifact or creature." [probed] | Gate `[]`; sub `PutCounter{P1P1, 2, ParentTarget}` with `RevealedHasCardType{[Artifact, Creature]}`. Memnite + Bond: 2 counters, 3/3, **draws 0**, a `PutCounter` resolved. Sol Ring: 2 via a resolved `PutCounter`. Sanctum: 0 | V19 red at base on draws and on the `PutCounter` event |
| P6 | `cargo engine-inventory` → `data/engine-inventory.json`, `StaticCondition` constant-truth variants [probed] | `None` (evaluated `true`) and `Unrecognized` (parse fallback). No constant-false variant | U1 representation: `StaticCondition::None` / `Not { None }` |
| P7 | `scratchpad/p1r/gates.py` over `/home/user/wt/data-out/p2/card-data.json` [probed] | 6 gate consumers, each a `ChangeZone` of one object; none carries both gates | charter decision 1 (no multi-entrant gate consumer); C6 |
| P8 | `scratchpad/p1r/r5scan.py` [probed] | type-writing population-gated statics: own 28, group 1 (Starfield of Nyx), granted 0, effect-carried 2; recipient-context population leaves 0; population + source-state `QuantityComparison` 0 | R5 reach |
| P9 | `scratchpad/p1r/c4scan.py` [probed] | 0 printed battlefield `ChangeZone` subs of shape `RevealedHasCardType → PutCounter` | U5 changes no printed parse |
| P10 | kind-level filter census [probed] | ~45 cards, none on the record/observation/projection paths U2 changes | U2 is building-block correctness |
| P11 | attack-declaration record, Flying snapshotted then removed live [probed] | `HasKeywordKind(Flying)` false, `WithoutKeywordKind(Flying)` true, `WithKeyword(Flying)` true | V10 red at base |
| P12 | `drive_to_empty` over its five callers [probed] | every caller ends at Priority on an empty stack | V20's postcondition holds for every existing caller |
| P13 | **F1:** Copy Enchantment (P0, cast, optional accepted, replacement 0, copy target P1's Starfield of Nyx) + 3 / + 4 other P0 enchantments (MV 1), then Release Juno on Sanctum [probed] | copy + 3 (4 pre-entry): Sanctum gets **2** counters, a 3/3 creature, no `PutCounter` event; copy + 4: 2 counters. Reach: the copier is on the battlefield named Starfield of Nyx under P0 with one `CopyValues` transient (its `base_static_definitions` is empty, its live `static_definitions` holds the copied static); the anchors are not creatures before the cast at copy + 3 (0 of 3) and are at copy + 4 (4 of 4) | V5b |
| P14 | **F2:** a P0 command-zone object (`is_emblem = true`) carrying Starfield of Nyx's parsed static definitions (both fields cloned from a P1 Starfield donor, layers flushed), P0 controls 4 / 5 enchantments (MV 1), Release Juno on Sanctum [probed] | 4 pre-entry: Sanctum gets **2** counters, a 3/3 creature; 5: 2 counters. Reach: the emblem is in P0's command zone; the anchors are creatures before the cast at 5 (5 of 5) and not at 4 (0 of 4) | V5c |
| P15 | **F3:** Divination and a vanilla Bear in P0's graveyard, granted Flashback by (i) Lier on the battlefield and (ii) Snapcaster Mage's resolved enters trigger; `HasKeywordKind(Flashback)` through `matches_target_filter_in_owner_zone`, with `controller == owner` (fast path) and then `controller = P1` (clone path), plus the card's own `keywords` [probed] | Snapcaster Mage (P0) resolved targeting Divination in P0's graveyard (one transient installed): Divination answers **true** through `matches_target_filter_in_owner_zone` on the fast path and, after `controller = P1`, on the clone path; `object_has_effective_keyword_kind` true; its own `keywords` lack Flashback (`has_keyword_kind` false), so r1's address test (own keywords of the clone) answers **false**. A vanilla Bear answers false everywhere. Control fixture: Lier's grant ("Each instant and sorcery card in your graveyard has flashback", an `InZone Graveyard` filter) **is** materialized into the graveyard card's `keywords` by the layer pass, so with Lier every design answers true and the row would not discriminate; Snapcaster's `SpecificObject` grant is not materialized (`layers::collect_scan_zones` scans an identity filter only in `layer_pass_materializes_keywords` zones) and reaches the card only through the off-zone path | V9b; the owner-zone row of the disposition table |
| P16 | **F4:** Divination committed with `runner.cast(spell).commit()`; a local copy of the harness loop with `max_steps` 0 / 40, with and without the postcondition assertion, under `catch_unwind` [probed] | after `commit()` the runner is at `Priority` with 1 stack object; with `max_steps = 0` the unasserted loop returns (no panic) and the asserted one panics; with `max_steps = 40` the asserted loop settles (no panic); a settled board (no cast) with `max_steps = 0` does not panic | V20, V20b |
| P17 | Copier census: replacement definitions carrying `BecomeCopy` of an enchantment (card-data) [probed] | 4 cards: Copy Enchantment, Mirrormade, Copy Artifact, Copy Land (the last two name "enchantment" in their exception clause, not their copy target) | F1 printed reach: Copy Enchantment and Mirrormade copying Starfield of Nyx |
| P18 | Off-battlefield population statics: statics with non-empty `active_zones`, and `CreateEmblem` statics, with a population leaf (`scratchpad/p1r2/offzone_scan.py`) [probed] | `active_zones` all-zones: 56 (cost modifiers and kin); `[Graveyard]`: 10 (the Incarnations, Gravecrawler, …); emblems: 2 (Ersta, Gideon of the Trials). **None writes a type or subtype** | F2 is building-block correctness for the gate (no printed gate reach); its runtime row V5c is a labelled synthetic carrier |

Unprobed (executor confirms): U-a: the lib unit rows V6a, V6b, V7–V9, V11 give the base values stated (derived by reading; a lib-test build was not run, since it previously took ~18 min and was once OOM-killed on this box). U-b: no existing `filter.rs` / `keywords.rs` / `layers.rs` unit test changes verdict except the ones this plan edits (the executor runs the three modules' tests).

---

## Units

### U6: L1/C5 — `drive_to_empty` proves it settled (lands first)

File: `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs`.

- Rename the body to `fn drive_to_empty_within(runner: &mut GameRunner, pick: &[ObjectId], branch: Option<&str>, max_steps: usize) -> (usize, Vec<String>)`; the loop becomes `for _ in 0..max_steps`. `drive_to_empty(runner, pick, branch)` becomes a one-line delegation with `max_steps = 40`, so every existing caller is unchanged.
- After the loop, assert the postcondition rather than a loop flag:

  ```rust
  let state = runner.state();
  assert!(
      matches!(state.waiting_for, WaitingFor::Priority { .. }) && state.stack.is_empty(),
      "drive_to_empty did not reach Priority on an empty stack within {max_steps} steps; prompts seen: {prompts:?}"
  );
  ```

  The postcondition is checked on the state itself, so a run whose last permitted step settles the game passes (a break flag would not see it), and a run that exhausted its steps unsettled fails.
- Add V20 and V20b (Verification Matrix). P12: every existing caller settles at base.

### U2: B2/C2 — one object-scoped effective keyword-kind authority, with door-declared liveness

**`crates/engine/src/game/keywords.rs`.** Add:

```rust
/// CR 603.10 + CR 608.2i + CR 400.7: what a filter door holds when it asks
/// about an object's keywords. The door declares it; the authority never
/// infers liveness from an address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HeldObject {
    /// The live object of its id, or a view of it that the door rescoped only
    /// in who controls it (`matches_target_filter_in_owner_zone`'s CR 109.5
    /// owner substitution). Off the battlefield it answers through the live
    /// object's off-zone effective keywords, which include off-zone grants.
    Live,
    /// A snapshot of an earlier moment (attack, counter, damage and event
    /// records; entry observations) or a projection of a hypothetical one
    /// (CR 614.12 entry projections, face-down and token entry objects). It
    /// answers from its own keywords: the object `FilterProp::WithKeyword` reads.
    Detached,
}

/// CR 603.10 + CR 608.2i + CR 400.7: whether the object a caller holds has a
/// keyword of `kind`. A caller holding a rescoped view of the live object
/// (same object, one attribute substituted) passes `HeldObject::Live`.
pub(crate) fn effective_keyword_kind_of(
    state: &GameState,
    obj: &GameObject,
    held: HeldObject,
    kind: KeywordKind,
) -> bool {
    match held {
        HeldObject::Detached => has_keyword_kind(obj, kind),
        HeldObject::Live if obj.zone == Zone::Battlefield => has_keyword_kind(obj, kind),
        HeldObject::Live => {
            crate::game::off_zone_characteristics::off_zone_has_keyword_kind(state, obj.id, kind)
        }
    }
}
```

`object_has_effective_keyword_kind(state, object_id, kind)` becomes `state.objects.get(&object_id).is_some_and(|obj| effective_keyword_kind_of(state, obj, HeldObject::Live, kind))` — lookup plus delegation, one authority, answer unchanged for every id-keyed caller.

**`crates/engine/src/game/filter.rs`.**
- `filter_inner_for_object` gains a final parameter `held: crate::game::keywords::HeldObject`; the recursive `Not` / `Or` / `And` / tracked-set calls pass it through unchanged.
- `matches_filter_prop` gains `held: HeldObject` after `object_id`; its call in `filter_inner_for_object` and its own recursive calls (`AnyOf`-style `.any`, `Not { prop }`, the `Historic` negation) pass it through.
- `entry_filter_matches_staged` gains `staged_held: HeldObject` and evaluates `projected.as_ref().map_or((staged, staged_held), |projected| (projected, HeldObject::Detached))`.
- `HasKeywordKind { value }` → `crate::game::keywords::effective_keyword_kind_of(state, obj, held, *value)`; `WithoutKeywordKind { value }` → its negation. Both polarities use the object held. Comment: `// CR 603.10 + CR 608.2i: the kind answer comes from the object this door holds, the same object WithKeyword reads; a Live door adds the live object's off-zone grants.`
- Every door passes the class its row below states. No `std::ptr::eq`, no address test.

**Invariant (charter decision 4).** For every caller, the kind-level answer comes from the same object as that caller's `WithKeyword` answer (`obj.has_keyword`). The only place the two read differently is a `Live` door holding an off-battlefield object, where the kind answer adds that same object's off-zone grants; the charter keeps that path, and the owner-zone clone is such a door.

**Per-caller disposition (constraint rf-plan-review-r1 F5).** "Today" is what the id-keyed lookup answers at base [read], except where marked [probed].

| `filter_inner_for_object` caller | Object held | `HeldObject` | Reading after U2 | CR | Today |
|---|---|---|---|---|---|
| `filter_inner_with_lookup` (all `matches_target_filter*` doors) | `&state.objects[id]` | `Live` | battlefield: own keywords; off-battlefield: off-zone effective | — | same (unchanged) |
| `matches_target_filter_including_phased_out` | `&state.objects[id]` | `Live` | same as above | 702.26b | same |
| `matches_target_filter_in_owner_zone`, fast path (`controller == owner`) | `&state.objects[id]` | `Live` | same as above | 109.5 + 400.3 | same |
| `matches_target_filter_in_owner_zone`, clone path (`controller != owner`) | the live object cloned only to set `controller = owner` | `Live` (a rescoped view of the live object) | off-zone effective, including off-zone grants | 109.5 + 400.3 (ownership stands in for control in an owner-scoped zone; nothing about the object's keywords changes) + 702.34a for the V9b grant | same (unchanged) [probed P15] |
| `matches_target_filter_on_entry_observation`, observation present, exact incarnation live | clone of the live object overlaid with the observation (`zone` = Battlefield) | `Detached` | observed keywords | 603.10 + 603.6a | live keywords (post-entry changes leak) |
| same, entrant gone or another incarnation | `GameObject::new` skeleton overlaid with the observation; `overlay_source_projection` stamps `zone = identity.expected_zone`, which `settle_battlefield_entry_observations` only records for a battlefield object, i.e. Battlefield [read] | `Detached` | observed keywords ("a departed entrant's skeleton answers as the battlefield reading it records") | 603.10 + 400.7 | removed: false; moved: the new object's off-zone answer |
| same, no observation | live arm / record arm (U3) | live arm goes through `matches_target_filter` (`Live`) | live: unchanged; record arm does not reach this prop (it uses `zone_change_record_matches_property`, which already reads `record.keywords`) | 603.10 | same |
| `matches_target_filter_on_object` (`EntryProjection::delivered` / `proposed`; sole production caller `zone_pipeline::enters_with_additional_counters_for_entry`) | projected clone, `zone` = Battlefield | `Detached` | own keywords | 614.12 | delivered: same; proposed: the off-battlefield card's off-zone effective keywords |
| `entry_filter_matches_staged` | projected clone (`Detached`), or the unprojected staged object: the live object (`Live`), a liminal projection (`Detached`; `replacement.rs` stamps `zone = Battlefield`), or an enter-as-copy clone (`Detached`) | per object | live: unchanged; projections: own keywords | 614.12 + 707.2 | copy clone: the pre-copy card's off-zone keywords |
| `matches_target_filter_on_battlefield_entry`, face-down arm | clone with face-down characteristics, origin zone | `Detached` | own keywords (none) | 708.2a + 614.12 | the face-up card's off-zone keywords |
| same, `TokenEntry` arm | liminal token projection (`LiminalEntrant::projected`) | `Detached` | own keywords | 111.3 + 614.12 | false (no object at the id) or the id's object [read] |
| same, `CreateToken` arm | `build_battlefield_entry_token_object` (`ObjectId(u64::MAX)`, Battlefield) | `Detached` | the token spec's keywords | 111.3 + 614.12 | false (id absent) |
| `matches_target_filter_on_counter_added_record` | skeleton (Battlefield) | `Detached` | snapshot keywords | 603.10 + 122.6 | live by id |
| `matches_target_filter_on_attack_declaration_record` | skeleton (Battlefield) | `Detached` | snapshot keywords | 508.1a + 608.2i | live by id ([probed] P11: false while `WithKeyword` is true) |
| `matches_target_filter_on_damage_record_source` | skeleton stamped `record.source_zone` | `Detached` | snapshot keywords, whether Battlefield or not | 608.2i + 608.2h | live by id |
| `matches_target_filter_on_event_snapshot` (connive, `replacement.rs` subject) | skeleton stamped `snapshot.zone` | `Detached` | snapshot keywords | 603.10a + 400.7 (the function's own contract: "never by re-reading the live object") | live by id |

Recursive `filter_inner_for_object` calls (`Not`, `Or`, `And`, tracked-set) pass the same object and the same `HeldObject` through, so they inherit the caller's class.

Printed reach of the changed rows: none (P10). F5's "measured statement" alternative is therefore also satisfied; the plan adds the sibling rows V10 and V9b regardless.

### U3: C3 — the no-observation fallback honours the entered incarnation

**`crates/engine/src/game/filter.rs` `matches_target_filter_on_entry_observation`.** The `let Some(observation) = … else` arm becomes:

```rust
// CR 400.7: the live arm lends only the entered incarnation; a later object
// at the same storage id is a new object, so the record answers instead.
return match state
    .objects
    .get(&record.object_id)
    .filter(|live| record.entered_incarnation.is_none_or(|incarnation| live.incarnation == incarnation))
{
    Some(_) => matches_target_filter(state, record.object_id, filter, ctx),
    None => matches_target_filter_on_zone_change_record_licensed(state, record, licensed, filter, ctx),
};
```

Update the function's doc sentence "the live entrant, or a gone entrant's record" to "the live entrant while it is the entered incarnation, otherwise its record (CR 400.7)".

**Claim the charter asks to establish: production reach of the mismatch [read].** The only production caller is `trigger_matchers::zone_change_clause_matches`. A record reaches the no-observation branch only when its entry was never settled: no instruction boundary followed it, settlement was deferred (D1, a same-instruction co-entry), or a mid-entry choice parked it (`capture_deferred_entry_events_if_mid_entry_choice` → `replay_deferred_entry_events`, which runs `process_triggers` before any game action). Incarnation changes only on a zone move (`zones.rs`: `bump_incarnation` on non-battlefield moves, `reset_for_battlefield_entry` on entry). No production route was found that moves the entrant between an unsettled entry and the trigger check: a later instruction that moves it is itself an instruction boundary that settles first, and replay runs before any action. So U3 is defensive hardening at a crate-public authority, as the charter frames it. The discriminating row is the lib unit test V11.

### U1: B1 — population-reading condition leaves are pinned to their pre-entry truth in the scratch

**`crates/engine/src/game/layers.rs`.** Add, next to `active_effect_condition_controller`:

1. `fn pinned_static_condition(truth: bool) -> StaticCondition` — the single documented constructor: `StaticCondition::None` when true, `StaticCondition::Not { condition: Box::new(StaticCondition::None) }` when false (P6). Doc: scratch-local constant truth; never serialized or observed outside the projection; no new variant (charter decision 3).
2. `fn pin_population_leaves(condition: &StaticCondition, pin_leaf: &impl Fn(&StaticCondition) -> Option<bool>) -> Option<StaticCondition>` — recursive over `And { conditions }` / `Or { conditions }` / `Not { condition }`; any other node is a leaf. Returns `Some(rewritten)` only if at least one leaf pinned (copy-on-write), `None` otherwise.
3. `fn pinned_definition_conditions(defs: &[StaticDefinition], pin_leaf: &impl Fn(&StaticCondition) -> Option<bool>) -> Vec<(usize, StaticCondition)>` — the rewrites for one definition list (index, rewritten condition), empty when nothing pins. Every carrier below computes its rewrites through this one function and writes them only when the list is non-empty, so an untouched list keeps its shared `Arc`.
4. `pub(crate) fn pin_entry_population_conditions(scratch: &mut GameState, pre_entry: &GameState, origin: Option<Zone>)`, doc-commented `CR 614.12 + CR 700.5 + CR 109.5 + CR 613.1a + CR 707.2` with Heliod ruling (a):
   - **Which leaves pin:** `!condition_uses_recipient_context(leaf) && (static_condition_uses_object_population(leaf) || origin.is_some_and(|zone| static_condition_reads_zone_membership(leaf, zone)))`.
   - **Pinned truth:** `evaluate_condition_with_context(pre_entry, leaf, controller, source_id, ConditionContext::ability_holder(controller))`.
   - **Carriers and their "you" (charter decision 2, constraint rf-plan-review-r1 F3).** The player is the one the scratch layer pass would use for that carrier, read on the scratch:
     - **Printed statics (F2):** every object in `scratch.objects`, whatever its zone. A static that does not function where its object is never generates an effect, so pinning it is inert; every static that does function (battlefield sources, command-zone emblems and other `object_sources_static_from_command_zone` sources, graveyard `active_zones` statics such as the Incarnations, CDAs) is covered without restating `for_each_static_effect_source`'s admission rules. `controller = scratch_obj.controller` (for the entrant: its post-override controller, CR 110.2a), `source_id = scratch_obj.id`. The rewrites are computed from `base_static_definitions` and from `static_definitions` separately and written to **both** fields: `Arc::make_mut(&mut obj.base_static_definitions)[i].condition = Some(..)` and `obj.static_definitions.get_mut(i)` (the existing copy-on-write `Definitions::get_mut`). The full pass reseeds `static_definitions` from `base_static_definitions` only for battlefield objects (`reset_recipient_to_base`), while `static_source_effects_in_pass` reads `static_definitions` for every source, so both fields must carry the pin.
     - **Copied statics (F1):** every scratch transient whose modifications include `ContinuousModification::CopyValues { values, .. }`: the rewrites of `values.static_definitions` are written through `Arc::make_mut(&mut values.static_definitions)`. Layer 1 of the scratch pass installs that payload as the recipient's `static_definitions` (`printed_cards::apply_copiable_values`), and the copied statics then generate effects with the recipient as their source. So `source_id` and `controller` are the copy recipient and its scratch controller (CR 707.2 + CR 613.1a + CR 109.5: the copier is the copied static's source and its controller is "you"). The recipient is the transient's bound `affected_recipient`, else its `TargetFilter::SpecificObject { id }`; both copy installers bind one (`become_copy`: `SpecificObject` + `affected_recipient`; `merge::install_merge_layer_effect`: `SpecificObject`) [read]. A `CopyValues` payload with no single bound recipient is left unpinned; no producer emits one [read].
     - **Transient conditions:** every `scratch.transient_continuous_effects[i].condition`; `controller = tce.controller`, `source_id = tce.source_id`.
   - `ability_holder` carries the same player only so `IsPresent` names it instead of reading the pre-entry source's controller through `FilterContext::from_source(pre_entry, source_id)`. Add to `ConditionContext::ability_holder`'s field doc: "The CR 614.12 entry pin (`pin_entry_population_conditions`) binds it to the carrier's "you": the source's scratch controller for a printed or copied static, and the effect's controller for a transient (CR 109.5). For a transient whose source has since changed controllers this deliberately differs from the live pass, whose `IsPresent` reads the source's current controller through `FilterContext::from_source`." (review non-blocking note; charter decision 2).
   - **Transients mutation:** compute every transient's rewrites with immutable iteration first, then write through `scratch.transient_continuous_effects.get_mut(i)`, so an `im::Vector` with nothing to pin is never touched.

**`crates/engine/src/game/filter.rs` `BattlefieldEntrantProjection::new`.**
- Replace the first line with `let origin = state.entering_or_live_object(entrant)?.zone;`.
- After the controller override and before `mark_layers_full`: `crate::game::layers::pin_entry_population_conditions(&mut scratch, state, Some(origin));` with comment `// CR 614.12 + CR 700.5: conditions count the pre-entry population (Heliod ruling 2020-01-24); the entrant's own statics still apply.`
- Doc comment on the struct: add "Population- and origin-zone-reading condition leaves of every static the scratch can apply (printed on any object, copied by a layer-1 copy effect) and of every transient are evaluated against the pre-entry state (`layers::pin_entry_population_conditions`); the live game, triggers and entry observations keep the realized battlefield."
- `battlefield_entrant_projection_cases` case (b) comment: "Devotion is 1 (its own {W})" → "Pre-entry devotion is 0: its own {W} is not counted (CR 614.12 + CR 700.5, Heliod ruling 2020-01-24)". The outcome is unchanged (not a creature).

**Pre-entry means before the event (charter decision 1).** Established, not re-defined: every gate consumer moves one object (P7), and both projection builders (`enter_with_counters_for_object`, `effective_enter_mods`) run before that object's move [read], so `state` at the call is the pre-event state. The multi-object loop (`process_one_zone_move_with_terminal`) would hand a later member a state containing earlier members; no printed gate consumer reaches it (residual R5(g)).

**Live state is never touched.** The pin writes only `scratch`. Ruling (b) is guarded by V3 and by the post-entry creature assertions in V1/V4/V5/V5b/V5c.

### U5: C4 — multi-type anaphoric gate folds as `TypeFilter::AnyOf`

**`crates/engine/src/parser/oracle_effect/conditions.rs` `card_type_condition_as_entry_this_way`.** Replace `let [core] = card_types.as_slice() else { return None };` and the `Some(..)` with a `match card_types.as_slice()`:
- `[] => return None`;
- `[core] => core_type_to_type_filter(*core)`;
- `types => TypeFilter::AnyOf(types.iter().copied().map(core_type_to_type_filter).collect())`;
then `Some(AbilityCondition::ZoneChangedThisWay { filter: TargetFilter::Typed(TypedFilter::new(type_filter)), destination: Some(Zone::Battlefield) })`. `additional_filter` / `subtype_filter` `Some` stay `None` (strict path). Doc comment: drop "several types" from the `None` list; state "several types → any-of (CR 614.1c + CR 614.12)". `anaphoric_entry_counter_gate` and `sequence.rs::is_anaphoric_entry_counters_clause` consume the same function unchanged, so the swallow recognizer follows (the executor runs the existing sequence tests).

Coverage honesty: U5 changes the lowering of already-accepted text; no text becomes newly accepted, and P9 shows no printed card changes.

### C6: comment only

**`crates/engine/src/game/effects/change_zone.rs`**, doc comment on `entry_gate_projection`: "Each gate consumer (`enter_with_counters_for_object`, `effective_enter_mods`) builds its own projection, and only when its gate is non-empty, so an entrant costs at most two projections. No printed card carries both gates on one entrant (card-data census, phase-1 plan P7), so one shared projection is not threaded through the two call paths." No non-comment line changes.

---

## Pattern Coverage

Assessed against the charter's class attribution (phase-plan mode):
- **U1:** every population- or origin-zone-reading `StaticCondition` leaf, under any static the scratch can apply (printed on an object in any zone, or latched in a layer-1 copy payload) or transient, inside the projection. Printed carriers: 28 own statics (24 Theros Gods, Rusted Relic, Phoenix Fleet Airship, Grond, the "enchanted permanent" Auras) and 1 group static (Starfield of Nyx) (P8), plus every enter-as-copy permanent that copies that group static (Copy Enchantment, Mirrormade; P17), crossed with the 6 gate consumers (P7). Off-battlefield carriers with population leaves exist (P18: 56 all-zone, 10 graveyard, 2 emblems) but none writes a type a gate reads; they are covered as building-block correctness.
- **U2:** every kind-level keyword filter, at every door (14 doors, each declaring `Live` or `Detached`); zero printed reach on the changed doors today (P10).
- **U3:** every enters observer whose record has no observation.
- **U5:** the any-of anaphoric entry gate building block (0 printed, P9).
- **U6:** harness hardening for every `drive_to_empty` caller, including phase 2's.

## Sizing

| Unit | Behavior | Registration surfaces | Discriminating test |
|---|---|---|---|
| U6 | L1/C5 settled postcondition | `tests/integration/etb_trigger_entry_time_characteristics.rs` | V20 (red when the assertion is reverted), V20b |
| U2 | B2/C2 object-scoped keyword kinds, door-declared liveness | `game/keywords.rs` (`HeldObject`, `effective_keyword_kind_of`), `game/filter.rs` (`filter_inner_for_object` / `matches_filter_prop` / `entry_filter_matches_staged` parameter, every door) | V7, V8, V10 (red at base); V9, V9b (preservation guards) |
| U3 | C3 incarnation-checked fallback | `game/filter.rs` | V11 |
| U1 | B1 pre-entry population pin (printed carriers in every zone, copy payloads, transients) | `game/layers.rs`, `game/filter.rs` (projection) | V1, V4, V5, V5b, V5c, V6c, V6d (integration); V6a, V6b (unit) |
| U5 | C4 any-of entry gate | `parser/oracle_effect/conditions.rs` | V18, V19 |

C6 is comment-only, not a unit. The F1/F2 carriers and the F3 door declaration extend U1 and U2; they are not new behaviors, so the unit count is unchanged. **T1 = 5 units → fires (≥ 2).**

Dependency edges: none in logic (no unit's discriminating test needs another unit's code). Order edge (verification only): U6 first, so every later row in its file runs under the hardened harness. Execution order: U6, U2, U3, U1, U5, C6.

Expected counted scope paths (T2 rule: test fixtures and regenerated pipeline data excluded outright; standing-class paths count, since T2 is re-counted over the materialized list):
1. `crates/engine/src/game/layers.rs`
2. `crates/engine/src/game/filter.rs`
3. `crates/engine/src/game/keywords.rs`
4. `crates/engine/src/parser/oracle_effect/conditions.rs`
5. `crates/engine/src/game/effects/change_zone.rs` (comment-only standing class)
6. `crates/engine/tests/integration/conditional_entry_gate_projection.rs`
7. `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs`
8. `crates/engine/src/parser/oracle_effect/tests.rs`

**T2 = 8 counted if test files count, 5 if they are classed as fixtures. T2 (≥ 13) does not fire either way, so the T1 ∧ T2 conjunction does not fire.** No compiler-forced site outside these files: the new `HeldObject` parameter is on three private `filter.rs` functions whose every caller is in `filter.rs`; `object_has_effective_keyword_kind` keeps its signature; `HeldObject` is not serialized. No new integration `mod` line (all rows go in existing files, so `tests/integration/main.rs` is untouched). Card data: regenerated by the pipeline, excluded; expected identical (P9; U1–U3 are runtime-only).

## Building Blocks

- U1: `layers::evaluate_condition_with_context`, `ConditionContext::ability_holder`, `static_condition_uses_object_population`, `static_condition_reads_zone_membership`, `condition_uses_recipient_context` (all existing, private to `layers.rs`; the new functions sit beside them, so no visibility widening beyond the new `pub(crate)` entry), `condition_controller` semantics, `StaticCondition::None`, `Definitions::get_mut` (existing copy-on-write positional access), `Definitions::iter_all` / `as_slice`, `Arc::make_mut`, `im::Vector::get_mut`. New helpers justified: no existing primitive evaluates a condition against one state and installs the result in another, and `pinned_definition_conditions` exists so the three carrier kinds share one rewrite computation.
- U2: `keywords::has_keyword_kind`, `off_zone_characteristics::off_zone_has_keyword_kind`. The new function is the object-scoped form of the existing effective lookup, which now delegates to it. `HeldObject` follows the door-declared-mode precedent of `layers::StaticZoneAdmission`.
- U3: the `entered_incarnation.is_none_or(..)` convention; `matches_target_filter_on_zone_change_record_licensed`.
- U5: `TypeFilter::AnyOf`, `core_type_to_type_filter`.
- U6: `SpellCast::commit` for the unsettled fixture.

## Logic Placement

- U1: condition semantics belong to `layers.rs` (condition-evaluation authority, and the owner of the layer-1 copy application whose payload it pins); `filter.rs`'s projection only calls it. Engine-only.
- U2: keyword authority and the `HeldObject` vocabulary are in `keywords.rs`; the filter doors only declare what they hold and forward it.
- U3: the entry-observation matcher (`filter.rs`) is the authority.
- U5: the parser's single condition converter (`conditions.rs`).
- U6: test harness.

## Rust Idioms

- No bools in data: the pin's truth is mapped through one constructor onto an existing enum shape; liveness is a two-value enum declared by the caller, matched exhaustively (`Detached` / `Live` with a battlefield guard / `Live`), not an address comparison or a flag.
- Slice patterns `[]` / `[core]` / `types` in U5.
- `is_none_or` / `is_some_and` / `Option::filter` in U3; copy-on-write via computed rewrite lists + `Arc::make_mut` / `Definitions::get_mut` / `im::Vector::get_mut`, written only when non-empty.
- U6 asserts a state postcondition rather than tracking a loop flag.

## Nom Compliance

`parser/oracle_effect/conditions.rs` changes a slice match only. No detection, dispatch or classification on text is added; no `contains`/`starts_with`/`find`. No Oracle text becomes newly accepted.

## Extension vs Creation

Extensions: U2 (object-scoped form of the existing authority; door-declared mode on the existing door functions), U3 (existing convention), U5 (existing converter, existing variant), U6. Creation: the U1 pin functions (justified above) and the `HeldObject` enum (justified under Variant Discoverability). No new variant on an existing enum, no new field, no serialized change.

## Variant Discoverability

`cargo engine-inventory` was run (P6). No variant is added to an existing enum in phase 1. `HeldObject` is a new crate-private, non-serialized enum, so the `/add-engine-variant` gate does not apply; its Stage 1 existence check was run anyway: `ControllerLookup` (filter.rs) is the controller-reading axis of the same doors and is not one-to-one with liveness (its `LiveOrLki` serves both live doors and projections), so folding liveness into it would conflate two axes; `TriggerSource` (triggers.rs, private) and `StaticZoneAdmission` (layers.rs, private) are the precedent shape, scoped to their own modules; no keyword-scoped live/snapshot type exists. The gate is `DEFERRED(phase 2)` for the onset enum.

## Identity / Provenance Contract

- **"If a creature enters this way" / "it's a creature" / "if it's an artifact or creature"** (Recommission, Release Juno, synthetic): the object is the ChangeZone's moved entrant, read as CR 614.12's would-be permanent: own statics and existing effects apply; their population/origin-zone leaves are bound to the pre-entry state at projection time (snapshotted, scratch-local, discarded with the projection). "You" per carrier: a printed static's source's scratch controller (post-override for the entrant, CR 110.2a); a copied static's copy recipient's scratch controller (CR 707.2 + CR 109.5); a transient's effect controller (CR 109.5). Hostile fixtures: V6b/V6c/V6d (owner ≠ controller), V5b (copier ≠ copied object's controller: P1's Starfield copied by P0), V6a (h) (transient controller ≠ copy recipient's controller).
- **Entry observation:** identity `record.object_id` + `record.entered_incarnation` (CR 400.7). With an observation, kind-level reads use the observed object (`Detached`). Without one, the live object is used only while it is the entered incarnation (U3). Hostile fixtures: V8 (departed), V11 (re-incarnated).
- **Record skeletons:** event-time snapshots; kind-level reads use the snapshot (`Detached`). Hostile fixture: V10.
- **Owner-zone view:** the live object with ownership substituted for control (CR 109.5 + CR 400.3), `Live`; kind reads keep the live object's off-zone grants. Hostile fixture: V9b (controller ≠ owner, an off-zone grant bound to the card by a resolved trigger).

## Verification Matrix

Integration rows go in existing files under `crates/engine/tests/integration/`; their `mod` lines already exist. Lib unit rows sit beside their `pub(crate)` seams (filter.rs `entry_observation_tests` / projection tests, layers.rs `mod tests`, keywords.rs `mod tests`) because those seams have no public door. "Base" values marked [probed] come from Step 3.5; [read] values are derived from code and labelled.

| # | Claim | Seam / production entry | Test (file, name) | Revert-failing assertion (base → fixed) | Reach guard / hostile siblings |
|---|---|---|---|---|---|
| V1 | B1: pre-entry devotion 4 → Heliod isn't a creature as it would enter → 0 counters (ruling a) | `pin_entry_population_conditions` ← `BattlefieldEntrantProjection::new` ← `enter_with_counters_for_object` ← Recommission cast | `conditional_entry_gate_projection.rs`, rewrite `recommission_reads_entrants_own_isnt_a_creature_static`: helper `recommission_on_heliod(ww_anchors, w_anchors)`; case (2, 0) | counters **1 → 0** [probed P1] | reach: Heliod on the battlefield and **a creature after entry** (live devotion 5 counts its own {W}, ruling b); no `PutCounter` event |
| V2 | B1 positive control: pre-entry devotion 5 → 1 counter | same | same test, case (2, 1) | 1 → 1 [probed P1b] (control) | case (0, 0) stays 0 / not a creature [probed P1e] |
| V3 | B1 ruling (b): a creature-enters observer sees Heliod as a creature; the pin does not leak into live state | live `trigger_matchers` enters observer | same file, new `recommission_heliod_four_devotion_still_entered_as_a_creature`: case (2, 0) + Elemental Bond | draws 1 → 1 [probed P1c] **and** counters 1 → 0 | a pin leaking into live state would make Heliod a noncreature and draw 0. Soul Warden is not used [probed P1d] |
| V4 | B1 class, own static with `ObjectCount` (Rusted Relic) | same projection, `QuantityComparison` leaf | same file, new `release_juno_rusted_relic_counts_pre_entry_artifacts`: 2 / 3 other artifacts | 2 others: counters **2 → 0** [probed P2]; 3 others: 2 → 2 | reach: Relic on the battlefield in both; a 5/5 artifact creature after entry at 2 others (live count 3); 7/7 at 3 |
| V5 | B1 class, another object's group static (Starfield of Nyx) | same projection, other-object static carrier | same file, new `release_juno_sanctum_counts_pre_entry_enchantments`: Starfield + 3 / + 4 others, anchors MV 1 | 4 pre-entry: counters **2 → 0** [probed P3]; 5: 2 → 2 | reach: Sanctum on the battlefield and a creature after entry in both (live count 5) |
| V5b | B1 class, **copied** group static (F1): Copy Enchantment (P0) entered as a copy of P1's Starfield of Nyx | `pin_entry_population_conditions` copy-payload carrier ← projection | same file, new `release_juno_sanctum_counts_copied_starfields_pre_entry_enchantments`: cast Copy Enchantment (`accept_optional`, `replacement_choice(0)`, `copy_target(starfield)`), then copy + 3 / copy + 4 other P0 enchantments (MV 1), Release Juno on Sanctum | copy + 3 (4 pre-entry): counters **2 → 0** [probed P13]; copy + 4: 2 → 2 | reach: the copier is on the battlefield named Starfield of Nyx under P0 with a `CopyValues` transient [probed P13]; Sanctum on the battlefield and a creature after entry (live count 5). Hostile: the original Starfield is P1's, so only the copy's "you" (P0) can make Sanctum a creature |
| V5c | B1 class, **command-zone** carrier (F2) | `pin_entry_population_conditions` printed carrier written to `static_definitions` ← projection | same file, new `release_juno_sanctum_counts_command_zone_static_pre_entry` (labelled synthetic: a P0 command-zone object with `is_emblem = true` carrying Starfield of Nyx's parsed static definitions, cloned from a P1 Starfield donor; layers flushed): 4 / 5 P0 enchantments (MV 1) | 4 pre-entry: counters **2 → 0** [probed P14]; 5: 2 → 2 | reach: the emblem is in P0's command zone and the anchors are creatures before the cast at 5 (the emblem's static functions) [probed P14]; Sanctum a creature after entry at 4 (live count 5). Pins the base-only write: with the pin written to `base_static_definitions` alone the command-zone static is unpinned and the row stays red |
| V6a | B1 pin unit: population leaf pinned; recursion; non-pinned leaves; every carrier kind | `layers::pin_entry_population_conditions` | `layers.rs` `mod tests`, new `pin_entry_population_conditions_cases` | (a) `Not(DevotionGE)` on a scratch battlefield static becomes `Not(pinned(pre-entry truth))` in both definition fields; (b) `And[DuringYourTurn, IsPresent]` pins only the `IsPresent` child; (c) a recipient-context leaf (`RecipientMatchesFilter`) and a non-population leaf (`SourceIsTapped`) are untouched and neither definitions `Arc` is cloned (`Arc::ptr_eq` before/after); (d) a graveyard-count `QuantityComparison` leaf pins with `origin = Some(Graveyard)` and is untouched with `origin = Some(Hand)`; (e) a transient's population condition pins with `tce.controller`; **(f) F2:** a command-zone emblem carrier's population condition is pinned in `static_definitions` (the field `static_source_effects_in_pass` reads) and in `base_static_definitions`; **(g) F2:** a graveyard carrier (`active_zones: [Graveyard]`, `IsPresent` leaf) is pinned in `static_definitions`; **(h) F1:** a `CopyValues` transient (controller P1) whose recipient is controlled by P0 has its payload static's population leaf pinned to P0's pre-entry truth (P0 and P1 differ in the fixture), and the transient's own `condition` is untouched when it has none | (c) are the hostile negatives; base: fn absent (unit, [read]) |
| V6b | F3 (rf-plan-review-r1) unit case: entrant owned by P1 entering under P0 | `BattlefieldEntrantProjection::new(&state, id, Some(P0), false)` | `filter.rs` projection tests, new `battlefield_entrant_projection_reads_new_controllers_pre_entry_population` | both not a creature (base: creature, [read]; owner semantics would also give creature) | sibling: P0 devotion 5 / P0 3 artifacts → creature |
| V6c | F3 runtime, `DevotionGE` | synthetic "Put target creature card from an opponent's graveyard onto the battlefield under your control. If a creature enters this way, it enters with an additional +1/+1 counter on it." | `conditional_entry_gate_projection.rs`, new `stolen_heliod_counts_new_controllers_pre_entry_devotion` | counters **1 → 0** [probed P4]; an owner-keyed pin would give 1 | premise: slot `[(Creature, P1P1, 1)]`, `enters_under: Some(You)` [probed]; reach: Heliod on the battlefield under P0 |
| V6d | F3 runtime, `ObjectCount` `QuantityComparison` | same synthetic with "target artifact card" | same file, new `stolen_rusted_relic_counts_new_controllers_pre_entry_artifacts` | counters **1 → 0** [probed P4] | reach: Relic on the battlefield under P0, a creature after entry |
| V7 | B2: kind-level answers read the observation, both polarities | `matches_filter_prop` kind arms ← `matches_target_filter_on_entry_observation` (`Detached`) | `filter.rs` `entry_observation_tests::entry_observation_shapes_read_the_projection`, beside its `WithKeyword` shape: `HasKeywordKind(Flying)` and `WithoutKeywordKind(Flying)` with the live entrant gaining Flying after entry | `HasKeywordKind`: (evaluator, live) = (true, true) → **(false, true)**; `WithoutKeywordKind`: (false, false) → **(true, false)** [read] | the existing `WithKeyword` assertion `(false, true)` is the paired positive |
| V8 | B2: a departed entrant keeps its kind-level answer | skeleton branch (`Detached`) | `filter.rs` `entry_observation_answers_for_departed_entrant`: fixture entrant has Flying at entry; after removal, `HasKeywordKind(Flying)` | **false → true** [read]; plus the moved case (incarnation + 1, graveyard, both `keywords` and `base_keywords` cleared; the base off-zone path, `off_zone_characteristics::effective_off_zone_keyword_contributions`, reads `base_keywords` plus off-zone effects, and the fixture's flush back-fills `base_keywords` from `keywords` through `sync_missing_base_characteristics`): false → true [read] | negative sibling: `HasKeywordKind(FirstStrike)` stays false |
| V9 | B2: live paths unchanged; the declared mode decides | `object_has_effective_keyword_kind` → `effective_keyword_kind_of` | `keywords.rs` `mod tests`, new `effective_keyword_kind_of_live_and_detached`: a graveyard card granted a kind by an identity-targeted (`SpecificObject`) off-zone grant, which the layer pass does not materialize into its `keywords` (as in `off_zone_characteristics::command_zone_emblem_grants_keyword_to_non_battlefield_card`), answers true through `object_has_effective_keyword_kind` and through `effective_keyword_kind_of(.., HeldObject::Live, ..)` on the live object **and on a clone of it**; the same clone passed as `HeldObject::Detached` answers from its own keywords (false) | preservation (live); detached false [read] | the clone-as-`Live` case is the F3 guard at unit level; see Reference Readings |
| V9b | B2 (F3): the owner-zone clone path keeps the off-zone granted kind | `matches_target_filter_in_owner_zone` clone path (`controller != owner`, `Live`) | `etb_trigger_entry_time_characteristics.rs`, new `owner_zone_view_keeps_off_zone_granted_keyword_kind`: Snapcaster Mage cast from P0's hand, its enters trigger targeting Divination in P0's graveyard and resolved; a vanilla Bear also in P0's graveyard; then `controller = P1` on both cards; filter `Typed(card + HasKeywordKind(Flashback))`, `FilterContext::from_source(state, snapcaster)` | Divination, clone path: **true → true** [probed P15]. Preservation at base, and discriminating against the address test: Divination's own `keywords` lack Flashback, so any design that answers the clone from its own keywords (r1's `ptr::eq`) gives false [probed P15] | positive reach: one transient installed; the fast path (`controller == owner`) answers true for Divination [probed P15]; negative sibling: the Bear answers false on both paths [probed P15]; guard that the row exercises the off-zone path: assert `!has_keyword_kind(&state.objects[&divination], KeywordKind::Flashback)` (a grant materialized into `keywords`, as Lier's is, would make the row non-discriminating) |
| V10 | B2 (F5 sibling): attack-declaration record answers the declaration-time kind | `matches_target_filter_on_attack_declaration_record` (pub, `Detached`) | `etb_trigger_entry_time_characteristics.rs`, new `attack_declaration_record_reads_declaration_time_keyword_kind` (see placement note) | `HasKeywordKind(Flying)` **false → true**, `WithoutKeywordKind(Flying)` **true → false** [probed P11] | positive: `WithKeyword(Flying)` = true on the same record [probed]; negative: `HasKeywordKind(FirstStrike)` false |
| V11 | C3: the no-observation fallback rejects a later incarnation | `matches_target_filter_on_entry_observation`, no-observation branch | `filter.rs` `entry_observation_tests`, new `entry_observation_absent_rejects_later_incarnation`: `fixture(false)`, two counters added, then `incarnation += 1` on the live object; filter `power_at_least(3)` | **true → false** [read] | positive sibling: same incarnation → live arm true (existing `entry_observation_absent_matches_todays_arms` stays green); record arm answer equals `matches_target_filter_on_zone_change_record_licensed` |
| V18 | C4 parse: multi-type rider folds into an any-of gate | `card_type_condition_as_entry_this_way` | `parser/oracle_effect/tests.rs`: rename `multi_type_entry_rider_is_not_folded` → `multi_type_entry_rider_folds_into_any_of_gate`; assert the gate is `[(Typed(AnyOf[Artifact, Creature]), P1P1, 2)]` and no surviving sub. `conditions.rs` `card_type_condition_as_entry_this_way_cases`: the `[Artifact, Creature]` row now expects `Some(ZoneChangedThisWay{AnyOf[..]})`; add `[]` → `None` | not folded → folded [read; P5 measured the unfolded base shape] | hostile: additional/subtype filter → `None`, `Not` → `None`, single type → plain filter (existing rows) |
| V19 | C4 runtime (required): counters are part of the entry (CR 614.1c) | `entry_gate_matches` with `AnyOf` ← Release Juno | `conditional_entry_gate_projection.rs`, new `release_juno_any_of_gate_is_an_entry_replacement` with `MULTI_TYPE_RELEASE_JUNO` (synthetic, labelled) + Elemental Bond | case 1, Memnite: draws **0 → 1**, `PutCounter` event **present → absent** [probed P5]; counters 2 → 2, 3/3 | case 2, Sol Ring: counters 2 → 2, `PutCounter` event present → absent; case 3, Sanctum: 0 → 0, still on the battlefield |
| V20 | L1/C5 (F4): the harness refuses an unsettled runner | `drive_to_empty_within` postcondition | `etb_trigger_entry_time_characteristics.rs`, new `#[should_panic(expected = "did not reach Priority on an empty stack")] fn drive_to_empty_refuses_an_unsettled_runner`: `scenario.add_spell_to_hand_from_oracle(P0, "Divination", false, DIVINATION)`, `drop(runner.cast(spell).commit())`, then `drive_to_empty_within(&mut runner, &[], None, 0)` | with the assertion reverted the call returns silently and the test fails (no panic → panic) [probed P16: after `commit()` the runner is at Priority with one stack object; the unasserted loop returns; the asserted one panics] | V20b is the paired positive; every existing caller is the reach guard (P12) |
| V20b | L1/C5: the postcondition is not vacuous | same | same file, new `drive_to_empty_accepts_a_settled_runner`: a scenario with no cast, `drive_to_empty_within(&mut runner, &[], None, 0)` returns `(0, vec![])` | no panic [probed P16] | rules out an always-panicking assertion |
| — | C1 rows (Kardur's III, Hunting Wilds, Rampaging Growth, is-form regression and classifier rows) | — | — | `DEFERRED(phase 2)` | — |

**V10 / V9b placement:** both rows go in `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs` (not `conditional_entry_gate_projection.rs`, which covers entry gates), beside the observation rows. V10 is built with `GameScenario` (a creature given Flying, `AttackDeclarationRecord { object_id, incarnation: Some(..), lki: obj.snapshot_public_characteristics(), is_token: false, is_commander: false }`, then Flying removed from the live object), exactly as P11 measured. V9b is built exactly as P15 measured (verbatim Snapcaster Mage and Divination text constants).

Coverage-status impact: none. No Oracle text changes its supported/unsupported status (P9; U1–U3 are runtime-only). The executor confirms by regenerating card data and diffing.

Integration-file header updates: `conditional_entry_gate_projection.rs`'s module doc and REVERT DISCRIMINATORS lines that state the old Heliod reading ("With two {W}{W} permanents, devotion is 1 + 4 = 5 …", "`recommission_reads_entrants_own_isnt_a_creature_static` gets 1 counter (expected 0)") are rewritten to the pre-entry reading and the new rows (including V5b/V5c: "drop the copy-payload carrier and `release_juno_sanctum_counts_copied_starfields_pre_entry_enchantments` gets 2 counters; write the pin to `base_static_definitions` only and `release_juno_sanctum_counts_command_zone_static_pre_entry` gets 2"); the Release Juno constant keeps its verbatim text. `etb_trigger_entry_time_characteristics.rs`'s REVERT DISCRIMINATORS gains: "drop `drive_to_empty_within`'s postcondition and `drive_to_empty_refuses_an_unsettled_runner` fails; read liveness by address in the kind authority and `owner_zone_view_keeps_off_zone_granted_keyword_kind` fails."

## Reference Readings

- **V2, V4 (3 others), V5 (5 enchantments) positive controls.** Derived from CR 614.12 and Heliod ruling (a), not from parity. With the condition already met pre-entry, the entrant is a creature as it would exist on the battlefield, so the gate applies: 1 counter for V2 and 2 counters for V4/V5. Base gives the same values, by its own route.
- **V5b (copy + 4) and V5c (5 anchors) positive controls.** Derived from Starfield of Nyx's text, CR 614.12, CR 707.2 and CR 109.5: the copy (V5b) or the emblem (V5c) is P0's, so "you" is P0; with five P0 enchantments before Sanctum enters, the condition holds, Sanctum is a creature as it would exist on the battlefield, and Release Juno's gate applies: 2 counters. Base: 2 and 2 [probed P13/P14]; reference equals derivation.
- **V3 draw (1).** Derived from ruling (b) and Elemental Bond's text: after entry, live devotion is 5 including Heliod's {W}, so Heliod is a 5/5 creature entering (power ≥ 3) → draw 1. Base gives 1 [probed]. The fixed counter count (0) is V1's derivation.
- **V9 live path.** Derived: a live off-battlefield object's kind answer is its effective off-zone keywords (CR 613.1 continuous effects that apply to it where it is). Measured reference: the existing off-zone keyword tests in `keywords.rs` / `off_zone_characteristics.rs`, which the executor runs; reference equals derivation [read].
- **V9b.** Derived from Snapcaster Mage's text ("target instant or sorcery card in your graveyard gains flashback until end of turn") and CR 702.34a: Divination is the targeted sorcery card in its owner P0's graveyard, so it has flashback until end of turn. Its `controller` field naming P1 changes nothing: a card in a graveyard is neither a permanent nor a spell, so "your" names its owner (CR 109.5 + CR 400.3); the effect's affected object was fixed when it began (CR 611.2c) and no rule removes a granted ability because of that field. So `HasKeywordKind(Flashback)` is true on both paths; the Bear (never targeted) is false. Reference: base true / false [probed P15]; reference equals derivation.
- **V11 positive sibling / `entry_observation_absent_matches_todays_arms`.** Derived: the same incarnation is the same object (CR 400.7), so the live answer is the entrant's. Base equals derivation [read]. **V11 record-arm parity.** Derived: once the live object is a later incarnation, the entered object no longer exists (CR 400.7), so the entry record answers: the 1/1 Bear as it entered, before the two counters. `power_at_least(3)` is false. That is the value `matches_target_filter_on_zone_change_record_licensed` returns for the record (reference: the record-arm assertions of `entry_observation_absent_matches_todays_arms` on the same fixture), so reference equals derivation [read].
- **V6b sibling; post-entry creature reach guards (V1, V4, V5, V5b, V5c, V6d); V10 positive guard; V8/V10 first-strike negatives.** V6b sibling: derived from Heliod's and Rusted Relic's text, CR 614.12, CR 700.5, CR 109.5 and CR 110.2a. The entrant enters under P0, so "you" is P0. With P0's pre-entry devotion to white at 5 (Heliod), or three P0 artifacts before entry (Rusted Relic), the condition already holds before entry, so the entrant is a creature as it would exist on the battlefield. Base also gives creature, because its scratch counts the entrant [read]; reference equals derivation. Post-entry reach guards: derived from Heliod ruling (b) and each card's text. After entry the live population counts the entrant: Heliod's {W} makes P0's devotion 5, Rusted Relic makes the third artifact, and Sanctum makes the fifth enchantment. So the condition holds and the entrant is a creature. Base gives the same [probed P1–P4, P13, P14]. V10 positive: the attack-declaration snapshot recorded Flying (CR 508.1a + CR 608.2i), so `WithKeyword(Flying)` on the record is true at base and after [probed P11]. First-strike negatives: no fixture object ever has first strike, so `HasKeywordKind(FirstStrike)` is false at base and after [read].
- **V19 cases 2 and 3.** Derived from the synthetic text: Sol Ring is an artifact → gate true → 2 counters as the entry; Sanctum is neither → 0. Base gives the same counts [probed] by a later `PutCounter` route; the rows assert the derived counts plus the absent event.
- **V20b.** Derived from the harness contract ("Answer every remaining prompt the rows below need"): a runner already at Priority on an empty stack has nothing left to answer, so the call returns with no draws and no prompts. Not a game-rules reading.
- **Defective references fixed in this phase:** Heliod at pre-entry devotion 4 (base 1, derived 0), Rusted Relic at 2 others (base 2, derived 0), Starfield at 4 (base 2, derived 0), the copied Starfield at 4 (base 2, derived 0), the command-zone Starfield static at 4 (base 2, derived 0), the stolen-entrant rows (base 1, derived 0), the kind reads on observations and records, and the multi-type rider (base: later `PutCounter`, derived: entry). Every row asserts the derived value.

## Step-by-step plan

1. **U6.** Split `drive_to_empty` into the delegating wrapper and `drive_to_empty_within` with the postcondition assertion. Add V20, V20b (and a `DIVINATION` verbatim constant). Run the five existing callers.
2. **U2.** In `keywords.rs`: `HeldObject`, `effective_keyword_kind_of`, delegation in `object_has_effective_keyword_kind`. In `filter.rs`: the `held` parameter on `filter_inner_for_object`, `matches_filter_prop` and (`staged_held`) `entry_filter_matches_staged`; every door's declaration per the disposition table; both kind arms. Add V7, V8 (filter.rs), V9 (keywords.rs), V9b and V10 (integration).
3. **U3.** Incarnation filter on the no-observation arm and its doc sentence; add V11.
4. **U1.** In `layers.rs`: `pinned_static_condition`, `pin_population_leaves`, `pinned_definition_conditions`, `pin_entry_population_conditions` (printed carriers over `scratch.objects` into both fields, copy payloads with the recipient's controller, transient conditions), the `ability_holder` doc sentence. In `filter.rs`: origin capture, the call before `mark_layers_full`, struct doc, case (b) comment. Rewrite the Recommission/Heliod row (V1/V2), add V3, V4, V5, V5b, V5c, V6c, V6d (integration), V6a (layers.rs), V6b (filter.rs). Update `conditional_entry_gate_projection.rs`'s header.
5. **U5.** Slice match in `card_type_condition_as_entry_this_way` + doc; update the conditions unit case; rename/rewrite the parser test (V18); add V19.
6. **C6.** Doc comment on `entry_gate_projection`.
7. `cargo fmt --all`; `./scripts/check-parser-combinators.sh`; Tilt `clippy` / `test-engine` / `card-data` when Tilt is up, otherwise the focused filters `cargo test -p phase-engine --features test-support --test integration -- conditional_entry_gate_projection etb_trigger_entry_time_characteristics` and `cargo test -p phase-engine --lib -- filter:: layers:: keywords:: oracle_effect::` under the caller's build constraints. Regenerate card data and confirm no diff.

CR annotations to add (all verified above): CR 614.12 + CR 700.5 + CR 109.5 + CR 110.2a + CR 613.1a + CR 707.2 (U1), CR 603.10 + CR 608.2i + CR 400.7 + CR 109.5 (U2, including the `HeldObject` doc), CR 400.7 (U3), CR 614.1c + CR 614.12 (U5).

## Residuals (run-level; reach established here)

- **R5 (B1 limits), reach per P7/P8/P17/P18:**
  - (a) population leaves that also use recipient context are not pinned: 0 among type-writing statics [probed P8];
  - (b) dynamic P/T quantities in modifications are not conditions: no gate consumer reads P/T — every gate filter is a core type or subtype [probed P7];
  - (c) a layer-1 copy effect's latched static definitions (`ContinuousModification::CopyValues { values }`, which layer 1 of the scratch pass installs as the copier's `static_definitions`) are pinned like printed statics, with the copy recipient as source and its scratch controller as "you" (CR 707.2 + CR 613.1a + CR 109.5); reach: Copy Enchantment and Mirrormade copying Starfield of Nyx [probed P17], row V5b. Left unpinned: a `CopyValues` payload with no single bound recipient (no producer emits one [read]);
  - (d) `ForAsLongAs` durations of transients are pruned, not pinned. No rules-correct gate reach: a resolution-created effect that modifies characteristics fixes its affected set when it begins (CR 611.2c, verified in `docs/MagicCompRules.txt`), so it never applies to an entrant that was not on the battlefield then, and an effect that only modifies the rules writes no type or subtype a gate reads. An engine transient that applied to the entrant anyway would be a CR 611.2c defect present at base, outside this phase [derived];
  - (e) granted statics (`GrantStaticAbility { definition }`) carrying a population-gated type write are not pinned: 0 printed [probed P8];
  - (f) a population leaf also reading the source's own state is pinned wholesale: 0 among type-writing statics [probed P8];
  - (g) multi-object `ChangeZone` iteration would count earlier co-entrants: no gate consumer is multi-entrant [probed P7].
- **R6:** C6's two-projection bound; no printed entrant carries both gates [probed P7].
- R1–R4, R7, R8 and all C1 content: phase 2 / run-level, untouched here.
