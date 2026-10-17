# Phase 1 plan: entry projection population, object-scoped keyword kinds, incarnation fallback, any-of entry gate, settled harness (etb-review-fixes, phase index 1)

- **Mode:** `engine-planner` phase-plan mode. Charter: `rf-charter-frozen.md`, phase 1. Deferral allowlist: all C1 content is `DEFERRED(phase 2)`.
- **PHASE_BASE_SHA:** `addbc884ee251794a64f2e1da0487cd329f8ce90`. Verified with `git rev-parse HEAD`; the tree was clean before and after probing (`git status --short` empty).
- **Loop position:** this plan resumes the plan loop at design round 2 of 5. It carries rf-plan-review-r1 F3, F4, F5, F9 (behavior) and the phase-1 parts of F7/F8 (text) as constraints. Budget: standard.
- **Addenda file:** `.git/engine-implementer-runs/etb-review-fixes/addenda/phase-1`.
- **Evidence labels.** **[probed]** = measured by a command named in Step 3.5. **[read]** = established by reading code at PHASE_BASE_SHA. **[unprobed]** = a claim the executor confirms before relying on it.

---

## Step 0: premise (MTGJSON `/home/user/wt/data-root/mtgjson/AtomicCards.json`, re-read for this plan) [probed]

| Card | Verbatim Oracle text (relevant clause) | Used by |
|---|---|---|
| Heliod, Sun-Crowned {2}{W} | "As long as your devotion to white is less than five, Heliod isn't a creature." and "Whenever you gain life, put a +1/+1 counter on target creature or enchantment you control." Rulings 2020-01-24: (a) "As a God enters the battlefield, your devotion to its color will determine whether any replacement effects that affect creatures entering the battlefield apply to that God. Because replacement effects are considered before the God is on the battlefield, the mana symbols in its mana cost won't be counted when determining this." (b) "When a God enters the battlefield, your devotion to its color (including the mana symbols in the mana cost of the God itself) will determine if a creature entered the battlefield or not for abilities that trigger whenever a creature enters the battlefield." | B1 (V1–V3, V6a) |
| Recommission {1}{W} | "Return target artifact or creature card with mana value 3 or less from your graveyard to the battlefield. If a creature enters this way, it enters with an additional +1/+1 counter on it." | B1 |
| What Must Be Done {3}{W}{W}, mode **Release Juno** | "Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature." (F7 phase-1 correction: the mode is "Release Juno", not "Let the Past Return".) | B1 (V4, V5), C4 template |
| Rusted Relic {4} | "Metalcraft — This artifact is a 5/5 Golem artifact creature as long as you control three or more artifacts." (F7 verbatim) | B1 (V4, V6b) |
| Starfield of Nyx {4}{W} | "As long as you control five or more enchantments, each other non-Aura enchantment you control is a creature in addition to its other types and has base power and base toughness each equal to its mana value." | B1 group static (V5) |
| Sanctum of Tranquil Light {W} | Legendary Enchantment — Shrine: "{5}{W}: Tap target creature. This ability costs {1} less to activate for each Shrine you control." | V5 entrant, V19 case 3 |
| Elemental Bond {2}{G} | "Whenever a creature you control with power 3 or greater enters, draw a card." | V3 (F4), V19 |
| Soul Warden {W} | "Whenever another creature enters, you gain 1 life." | Not used as a fixture (F4 confound, measured below) |
| Memnite {0} | Artifact Creature — Construct 1/1, no rules text | V19 case 1 |
| Heroic Return; Winter Soldier, Reborn Avenger; Summoner's Grimoire; Oviya, Automech Artisan | gate consumers, each moving one object ("target creature card", "target creature card …", "a creature card", "a creature or Vehicle card") | multi-entrant claim (P2) |

The PR review's quotes and both Heliod rulings match MTGJSON. The premise holds.

**CR numbers, each grep-verified in `docs/MagicCompRules.txt`** [probed]: 614.12 (l.3104), 614.12a (3109), 614.1c (3066), 700.5 (3242), 109.5 (610), 110.2a (618), 613.1 (2964), 603.10 (2642), 603.10a (2644), 603.6a (2605), 608.2h (2812), 608.2i (2814), 608.2c (2799), 611.3c (2933), 400.7 (1955), 702.26b (4185), 708.2a (5726), 707.2 (5633), 111.3 (649), 508.1a (2269), 122.6 (1210).

## Step 1: skills (checklist skeleton)

| Skill | Phase-1 content | Later-phase content |
|---|---|---|
| `/add-static-ability` | U1: condition evaluation for the scratch layer pass (`layers.rs`) | — |
| `/add-replacement-effect` | U1: the CR 614.12 entrant projection; U5: the any-of entry gate read by `entry_gate_matches` | — |
| `/add-trigger` | U2: keyword-kind reads on the entry-observation matcher and record matchers; U3: the no-observation fallback | `DEFERRED(phase 2)`: the D1 deferral predicate and its walkers |
| `/oracle-parser` | U5: slice-pattern extension of `card_type_condition_as_entry_this_way` (no new text matching) | `DEFERRED(phase 2)`: the onset produced by predicate dispatch |
| `/add-engine-variant` | Not triggered: no variant is added (U1 uses existing `StaticCondition::None`; U5 uses existing `TypeFilter::AnyOf`). The inventory was consulted anyway (P6). | `DEFERRED(phase 2)`: the onset enum and field gate |
| `/card-test` | every cast-pipeline row (GameScenario + `GameRunner::cast(..).resolve()`, verbatim Oracle text, `CastOutcome` deltas) | `DEFERRED(phase 2)`: C1 rows |

Checklist steps that do not apply in phase 1: types/serde (no type change), frontend, AI, multiplayer filter, `WaitingFor`/`GameAction`.

## Step 2: analogous traces

- **U1 (condition evaluated against a population other than the scratch's).** `filter.rs::BattlefieldEntrantProjection::new` → `zones::move_to_zone_with_entry_flags` → `zones::apply_battlefield_entry_controller_override` → `layers::mark_layers_full` / `layers::flush_layers` → seed (`obj.static_definitions = Arc::clone(&obj.base_static_definitions)`) → `static_source_effects_in_pass` (reads `source.static_definitions`) / `gather_transient_continuous_effects_with` → condition evaluated with `active_effect_condition_controller` → `condition_controller` → `evaluate_condition_with_context` → `evaluate_condition_inner` (`DevotionGE` → `devotion::count_devotion(state, controller, ..)`; `QuantityComparison` → `quantity::resolve_quantity_with_ctx(state, .., controller, ..)` → `resolve_ref` → `FilterContext::from_source_with_controller(source_id, controller)`; `IsPresent` → `ability_holder` or `FilterContext::from_source(state, source_id)`). Classifiers reused: `layers::static_condition_uses_object_population`, `layers::static_condition_reads_zone_membership`, `layers::condition_uses_recipient_context`.
- **U2 (keyword reads on a held object).** `filter.rs::matches_filter_prop`: `FilterProp::WithKeyword` → `obj.has_keyword` → `keywords::has_keyword(obj)` (object-scoped) against `FilterProp::HasKeywordKind` / `WithoutKeywordKind` → `keywords::object_has_effective_keyword_kind(state, object_id, ..)` (id-keyed: live battlefield keywords, else `off_zone_characteristics::off_zone_has_keyword_kind`). Callers enumerated in U2's disposition table.
- **U3 (incarnation convention).** `types/game_state.rs::GameState::battlefield_entry_observation` (`record.entered_incarnation.is_none_or(|incarnation| row.identity.reference.incarnation == incarnation)`), `zones::settle_battlefield_entry_observations` (same convention), consumed at `trigger_matchers::zone_change_clause_matches` → `filter::matches_target_filter_on_entry_observation`.
- **U5 (anaphoric entry-gate fold).** `conditions.rs::anaphoric_entry_counter_gate` → `card_type_condition_as_entry_this_way` (also consumed by `sequence.rs::is_anaphoric_entry_counters_clause`) → `lower.rs::fold_enters_this_way_counter_rider` (accepts any `ZoneChangedThisWay` filter into `conditional_enter_with_counters`) → `change_zone.rs::enter_with_counters_for_object` → `entry_gate_matches` → `BattlefieldEntrantProjection::matches`.
- **U6.** `etb_trigger_entry_time_characteristics.rs::drive_to_empty` and its five callers (`glider_with_tempest` → `r5b_glider_flying_counter_gives_haste`; `search_then_counters` → `r5d_search_bears`, `r5d_search_giant_sibling`; `r11_clear_the_land`; `r12_path_of_the_schemer`).

## Step 3: files read (all at PHASE_BASE_SHA)

`game/filter.rs` (the projection, `matches_target_filter_on_entry_observation`, every `filter_inner_for_object` caller, `matches_filter_prop`, `mod entry_observation_tests`, `battlefield_entrant_projection_cases`), `game/layers.rs` (`ConditionContext`, `evaluate_condition*`, `condition_controller`, `active_effect_condition_controller`, the three classifiers, gather path), `game/keywords.rs`, `game/off_zone_characteristics.rs` (read-only), `game/devotion.rs` (read-only), `game/quantity.rs` (`resolve_quantity_with_ctx`, `resolve_ref` filter context; read-only), `game/zones.rs` (`settle_battlefield_entry_observations`, incarnation writes, controller reset on exit; read-only), `game/effects/change_zone.rs` (`EntryGateContext`, `entry_gate_projection`, `enter_with_counters_for_object`, `effective_enter_mods`, the multi-object loop), `game/engine_replacement.rs` (`replay_deferred_entry_events`, `capture_deferred_entry_events_if_mid_entry_choice`; read-only), `game/trigger_matchers.rs` (read-only), `types/game_state.rs` (read-only), `parser/oracle_effect/conditions.rs`, `parser/oracle_effect/lower.rs` (read-only), `parser/oracle_effect/tests.rs` (`multi_type_entry_rider_is_not_folded`), both integration test files.

## Step 3.5: probes

Probe harness: a temporary integration module `zz_p1_probe.rs` (kept at `scratchpad/p1r/zz_p1_probe.rs`) plus a temporary `eprintln!` of a settle flag inside `drive_to_empty`, built and run with `CARGO_TARGET_DIR=/home/user/phase-personal/target CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test -p phase-engine --features test-support --test integration -- zz_probe r5b_ r5d_ r11_ r12_ --nocapture --test-threads=1`. Every probe ran to `test result: ok` and printed a positive marker (the entrant on the battlefield, a non-empty parse slot, nonzero counters somewhere in the same test). The temporary files were then removed and the two edited files restored byte-for-byte from copies; `git status --short` is empty.

| # | Probe | Result at base | Decision it informs |
|---|---|---|---|
| P1 | Heliod via Recommission, P0 anchors {W}{W}×2 (pre-entry devotion 4) [probed] | 1 counter, creature, 6/6 | V1 is red at base (derived 0) |
| P1b | same, {W}{W}×2 + {W}×1 (pre-entry devotion 5) [probed] | 1 counter | V2 positive control equals base |
| P1c | devotion 4 + Elemental Bond [probed] | 1 counter, draws 1 | V3 observer fires on Heliod (5/5 or 6/6 ≥ 3) |
| P1d | devotion 4 + Soul Warden [probed] | **2 counters**, life +1, a `PutCounter` resolved | F4 confound confirmed: Heliod's own life-gain trigger targeted Heliod. Soul Warden is rejected as V3's observer; Elemental Bond (no life gain) is used |
| P1e | Heliod via Recommission, no anchors [probed] | 0 counters, not a creature | existing `what_must_be_done_reads_gods_own_static` / first half of the existing Recommission row are unchanged |
| P2 | Rusted Relic via Release Juno, 2 / 3 other artifacts [probed] | 2 counters (7/7 creature) / 2 counters | V4: red at 2 others (derived 0), control at 3 |
| P3 | Starfield of Nyx + Sanctum via Release Juno, Starfield + 3 / Starfield + 4 other enchantments, anchors with mana value 1 [probed] | 2 counters (3/3 creature) / 2 counters (3/3) | V5: red at 4 pre-entry (derived 0), control at 5. Fixture note: zero-mana-value anchors die as 0/0 creatures under Starfield once five enchantments exist, so anchors need mana value ≥ 1 (a first run with MV-0 anchors measured that confound) |
| P4 | Synthetic "Put target creature/artifact card from an opponent's graveyard onto the battlefield under your control. If a creature enters this way, it enters with an additional +1/+1 counter on it." [probed] | Parses to `ChangeZone{enters_under: Some(You), conditional_enter_with_counters: [(Creature, P1P1, 1)]}`, no sub. Heliod owned by P1 (P0 pre-entry devotion 4, P1 6): 1 counter. Rusted Relic owned by P1 (P0 controls 2 artifacts, P1 3): 1 counter | F3 runtime rows V6c/V6d: red at base (derived 0) and an owner-semantics pin would also give 1 |
| P5 | Release Juno with "… if it's an artifact or creature." [probed] | Gate `[]`; sub `PutCounter{P1P1, 2, ParentTarget}` with `RevealedHasCardType{[Artifact, Creature]}`. Memnite + Bond: 2 counters, 3/3, **draws 0**, a `PutCounter` resolved. Sol Ring: 2 counters via a resolved `PutCounter`. Sanctum: 0 counters | V19 (F9): red at base on draws and on the `PutCounter` event |
| P6 | `cargo engine-inventory` → `data/engine-inventory.json` (gitignored), `StaticCondition` variants matching none/always/never/true/false/const/unrecog [probed] | `None` (unit; evaluated `true` by `evaluate_condition_inner`) and `Unrecognized` (parse fallback). No constant-false variant | U1 representation: `StaticCondition::None` for true, `StaticCondition::Not { condition: Box::new(StaticCondition::None) }` for false |
| P7 | `scratchpad/p1r/gates.py` over `/home/user/wt/data-out/p2/card-data.json`: every node carrying a non-empty `conditional_enter_with_counters` or an `enters_modified_if` [probed] | 6 cards, each a `ChangeZone` of one object: Heroic Return, Oviya, Recommission, Summoner's Grimoire (`enters_modified_if` only), What Must Be Done, Winter Soldier, Reborn Avenger. None carries both gates; none is a `ChangeZoneAll` | Charter decision 1: no gate consumer is multi-entrant (established, not re-defined). Charter decision 8 / C6: no printed entrant carries both gates |
| P8 | `scratchpad/p1r/r5scan.py`: Continuous statics that write a core type or subtype under a population-reading condition, classed own / group / granted / effect-carried, plus recipient-context and source-state operands in those leaves [probed] | own 28, group 1 (Starfield of Nyx), granted 0, effect-carried 2 (Arvinox, Ultron); population leaves with recipient context: 0; `QuantityComparison` mixing a population count with source state: 0 | R5 reach (see Residuals) |
| P9 | `scratchpad/p1r/c4scan.py` plus a positive-control census [probed] | 0 battlefield `ChangeZone` subs of shape `RevealedHasCardType → PutCounter`; the census reaches 3 `TargetMatchesFilter → PutCounter` subs (Defy Death) and 12 multi-type `RevealedHasCardType` conditions elsewhere | U5 changes no printed card's parse; card-data regeneration is expected to be byte-identical |
| P10 | Kind-level filter census over card data (`HasKeywordKind` / `WithoutKeywordKind` with the path to each) [probed] | 17 shapes over ~45 cards (Suspend, Flashback, Augment, Foretell, Cycling, Mutate, Disturb, Madness, Disguise, Awaken, FirstStrike, Eternalize, Embalm, Decayed). Paths: live targets/searches/`ObjectCount`/static `affected`, `TargetMatchesFilter`, the zone-change record path (Wilhelt), and `EventObjectMatchesFilter` (live then LKI record; Anje, Pollywog Symbiote). None sits on an enters-observer subject, a battlefield-entry replacement filter, an attack-declaration / counter-added / damage-record matcher, or the connive event snapshot | U2 has no printed reach on the paths it changes; it is building-block correctness (the reviewer's own framing) |
| P11 | Attack-declaration record: Bird with Flying snapshotted, then Flying removed from the live object; `matches_target_filter_on_attack_declaration_record` [probed] | `HasKeywordKind(Flying)` = false, `WithoutKeywordKind(Flying)` = true, `WithKeyword(Flying)` = true | V10 (F5 sibling) is red at base and shows the base disagreement between `WithKeyword` and its kind sibling on the same snapshot |
| P12 | `drive_to_empty` settle flag over its five callers [probed] | settled = true for r5b (prompts ChooseOneOfBranch, Priority×5), r5d_search_bears, r5d_search_giant_sibling, r11, r12 | V20: the new assertion holds for every existing caller |

Unprobed (executor confirms): U-a: the lib unit rows V7–V9, V11 and V6a/V6b give the base values stated (derived by reading; a lib-test build was not run, since it previously took ~18 min and was once OOM-killed on this box). U-b: no existing `filter.rs` / `keywords.rs` / `layers.rs` unit test changes verdict under U1/U2 except the ones this plan edits (the executor runs the three modules' tests).

---

## Units

### U6: L1/C5 — `drive_to_empty` proves it settled (lands first)

File: `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs`.

- Add `let mut settled = false;` before the loop; the `WaitingFor::Priority { .. } if runner.state().stack.is_empty()` arm becomes `{ settled = true; break }`.
- After the loop: `assert!(settled, "drive_to_empty did not reach Priority on an empty stack within 40 steps; prompts seen: {prompts:?}");`.
- Return type and callers unchanged. P12 shows all five callers settle at base.

### U2: B2/C2 — one object-scoped effective keyword-kind authority

**`crates/engine/src/game/keywords.rs`.** Add:

```rust
/// CR 603.10 + CR 608.2i + CR 400.7: whether the object a caller holds has a
/// keyword of `kind`. A battlefield-zoned object answers from its own
/// (layer-applied or snapshotted) keywords. An off-battlefield object that IS
/// the live object of its id (`std::ptr::eq`) answers through the off-zone
/// effective path, which adds off-zone grants. Any other off-battlefield object
/// is a snapshot or a projection, and answers from its own keywords — the same
/// object `FilterProp::WithKeyword` reads.
pub fn effective_keyword_kind_of(state: &GameState, obj: &GameObject, kind: KeywordKind) -> bool
```

Body: `obj.zone == Zone::Battlefield` → `has_keyword_kind(obj, kind)`; else if `state.objects.get(&obj.id).is_some_and(|live| std::ptr::eq(live, obj))` → `off_zone_characteristics::off_zone_has_keyword_kind(state, obj.id, kind)`; else `has_keyword_kind(obj, kind)`. Pointer identity is the existing idiom for "this is the very object" (`off_zone_characteristics.rs` matches its scope frame by `std::ptr::eq(frame.state, state)`).

`object_has_effective_keyword_kind(state, object_id, kind)` becomes `state.objects.get(&object_id).is_some_and(|obj| effective_keyword_kind_of(state, obj, kind))` — lookup plus delegation, one authority. Its answer for every id-keyed caller is unchanged (the looked-up object is the live object by construction).

**`crates/engine/src/game/filter.rs` `matches_filter_prop`.** `HasKeywordKind { value }` → `crate::game::keywords::effective_keyword_kind_of(state, obj, *value)`; `WithoutKeywordKind { value }` → its negation. Both polarities use the object held. Add a CR 603.10 + CR 608.2i comment naming the invariant.

**Invariant (charter decision 4).** For every caller, the kind-level answer comes from the same object as that caller's `WithKeyword` answer (`obj.has_keyword`). The only branch where the two read differently is the live off-battlefield object, where the kind answer adds off-zone grants to the same object; the charter keeps that path.

**Per-caller disposition (F5 replacement for the draft's "Live-path semantics are unchanged" bullet).** "Today" is what the id-keyed lookup answers at base [read], except where marked [probed].

| `filter_inner_for_object` caller | Object held | Class | Reading after U2 | CR | Today |
|---|---|---|---|---|---|
| `filter_inner_with_lookup` (all `matches_target_filter*` doors) | `&state.objects[id]` | live | battlefield: own keywords; off-battlefield: off-zone effective | — | same (unchanged) |
| `matches_target_filter_including_phased_out` | `&state.objects[id]` | live | same as above | 702.26b | same |
| `matches_target_filter_in_owner_zone`, fast path (`controller == owner`) | `&state.objects[id]` | live | same as above | 109.5 | same |
| `matches_target_filter_in_owner_zone`, clone path (`controller != owner`) | controller-rescoped clone | projection | own keywords (no off-zone grants) | 109.5 | off-zone effective by id. Unreachable in production [read]: `zones.rs` resets `controller = base_controller.unwrap_or(owner)` on every move to a non-battlefield, non-stack zone, and `base_controller = Some(..)` is written outside tests only by `engine_debug.rs` |
| `matches_target_filter_on_entry_observation`, observation present, exact incarnation live | clone of the live object overlaid with the observation (`zone` = Battlefield) | entry observation | observed keywords | 603.10 + 603.6a | live keywords (post-entry changes leak) |
| same, entrant gone or another incarnation | `GameObject::new` skeleton overlaid with the observation; `overlay_source_projection` stamps `zone = identity.expected_zone`, which `settle_battlefield_entry_observations` only records for a battlefield object, i.e. Battlefield [read] | entry observation | observed keywords ("a departed entrant's skeleton answers as the battlefield reading it records") | 603.10 + 400.7 | removed: false; moved: the new object's off-zone answer |
| same, no observation | live arm / record arm (U3) | live / record | live: unchanged; record arm does not reach this prop (it uses `zone_change_record_matches_property`, which already reads `record.keywords`) | 603.10 | same |
| `matches_target_filter_on_object` (`EntryProjection::delivered` / `proposed`, from `zone_pipeline.rs`) | projected clone, `zone` = Battlefield | projection | own keywords | 614.12 | delivered: same (the entrant is live on the battlefield and the allowlisted statics write only subtype/colour); proposed: the off-battlefield card's off-zone effective keywords |
| `entry_filter_matches_staged` | projected clone (Battlefield), or the unprojected staged object: the live object (live), a liminal projection (`replacement.rs` stamps `zone = Battlefield`), or an enter-as-copy clone | live / projection | live: unchanged; projections: own keywords | 614.12 + 707.2 | copy clone: the pre-copy card's off-zone keywords |
| `matches_target_filter_on_battlefield_entry`, face-down arm | clone with face-down characteristics, origin zone | projection | own keywords (none) | 708.2a + 614.12 | the face-up card's off-zone keywords |
| same, `TokenEntry` arm | liminal token projection (`LiminalEntrant::projected`) | projection | own keywords | 111.3 + 614.12 | false (no object at the id) or the id's object [read] |
| same, `CreateToken` arm | `build_battlefield_entry_token_object` (`ObjectId(u64::MAX)`, Battlefield) | projection | the token spec's keywords | 111.3 + 614.12 | false (id absent) |
| `matches_target_filter_on_counter_added_record` | skeleton (Battlefield) | record | snapshot keywords | 603.10 + 122.6 | live by id |
| `matches_target_filter_on_attack_declaration_record` | skeleton (Battlefield) | record | snapshot keywords | 508.1a + 608.2i | live by id ([probed] P11: false while `WithKeyword` is true) |
| `matches_target_filter_on_damage_record_source` | skeleton stamped `record.source_zone` | record | snapshot keywords, whether Battlefield or not (a fresh skeleton is never the live object) | 608.2i + 608.2h | live by id |
| `matches_target_filter_on_event_snapshot` (connive, `replacement.rs` subject) | skeleton stamped `snapshot.zone` | record | snapshot keywords | 603.10a + 400.7 (the function's own contract: "never by re-reading the live object") | live by id |

Recursive `filter_inner_for_object` calls (`Not`, `Or`, `And`, tracked-set) pass the same object through, so they inherit the caller's class.

Printed reach of the changed rows: none (P10). F5's "measured statement" alternative is therefore also satisfied; the plan adds the sibling row V10 regardless.

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

**Claim the charter asks to establish: production reach of the mismatch [read].** The only production caller is `trigger_matchers::zone_change_clause_matches`. A record reaches the no-observation branch only when its entry was never settled: no instruction boundary followed it, settlement was deferred (D1, a same-instruction co-entry), or a mid-entry choice parked it (`capture_deferred_entry_events_if_mid_entry_choice` → `replay_deferred_entry_events`, which runs `process_triggers` before any game action). Incarnation changes only on a zone move (`zones.rs`: `bump_incarnation` on non-battlefield moves, `reset_for_battlefield_entry` on entry). No production route was found that moves the entrant between an unsettled entry and the trigger check: a later instruction that moves it ("sacrifice it") is itself an instruction boundary that settles first, and replay runs before any action. So U3 is defensive hardening at a crate-public authority, as the charter frames it. The discriminating row is the lib unit test V11.

### U1: B1 — population-reading condition leaves are pinned to their pre-entry truth in the scratch

**`crates/engine/src/game/layers.rs`.** Add, next to `active_effect_condition_controller`:

1. `fn pinned_static_condition(truth: bool) -> StaticCondition` — the single documented constructor: `StaticCondition::None` when true, `StaticCondition::Not { condition: Box::new(StaticCondition::None) }` when false (P6). Doc: scratch-local constant truth; never serialized or observed outside the projection; no new variant (charter decision 3).
2. `fn pin_population_leaves(condition: &StaticCondition, pin_leaf: &impl Fn(&StaticCondition) -> Option<bool>) -> Option<StaticCondition>` — recursive over `And { conditions }` / `Or { conditions }` / `Not { condition }`; any other node is a leaf. Returns `Some(rewritten)` only if at least one leaf pinned (copy-on-write), `None` otherwise.
3. `pub(crate) fn pin_entry_population_conditions(scratch: &mut GameState, pre_entry: &GameState, origin: Option<Zone>)`, doc-commented `CR 614.12 + CR 700.5 + CR 109.5` with Heliod ruling (a):
   - **Which leaves pin:** `!condition_uses_recipient_context(leaf) && (static_condition_uses_object_population(leaf) || origin.is_some_and(|zone| static_condition_reads_zone_membership(leaf, zone)))`.
   - **Pinned truth:** `evaluate_condition_with_context(pre_entry, leaf, controller, source_id, ConditionContext::ability_holder(controller))`.
   - **Controller (charter decision 2, F3):** the player the scratch layer pass would use for that carrier — `condition_controller` semantics read on the scratch:
     - printed statics: carriers are the `base_static_definitions` of every object in `scratch.battlefield` and `scratch.command_zone` (exactly the population `for_each_static_effect_source` draws from; the scratch's `static_source_index` is stale until its own layer pass, so the ids are read from the zone lists). `controller = scratch_obj.controller` (for the entrant: its post-override controller, CR 110.2a), `source_id = scratch_obj.id`;
     - transients: every `scratch.transient_continuous_effects[i].condition`; `controller = tce.controller`, `source_id = tce.source_id`.
   - `ability_holder` carries the same player only so `IsPresent` names it instead of reading the pre-entry source's controller through `FilterContext::from_source(pre_entry, source_id)`. Add one sentence to `ConditionContext::ability_holder`'s field doc naming this use (charter decision 2).
   - **Rewrite:** a carrier is written only when `pin_population_leaves` returns `Some`: `Arc::make_mut(&mut obj.base_static_definitions)[i].condition = Some(rewritten)`; transients via `im::Vector` index set. The full pass's seed (`obj.static_definitions = Arc::clone(&obj.base_static_definitions)`) then carries the pinned definitions.

**`crates/engine/src/game/filter.rs` `BattlefieldEntrantProjection::new`.**
- Replace the first line with `let origin = state.entering_or_live_object(entrant)?.zone;`.
- After the controller override and before `mark_layers_full`: `crate::game::layers::pin_entry_population_conditions(&mut scratch, state, Some(origin));` with comment `// CR 614.12 + CR 700.5: conditions count the pre-entry population (Heliod ruling 2020-01-24); the entrant's own statics still apply.`
- Doc comment on the struct: add "Population- and origin-zone-reading condition leaves are evaluated against the pre-entry state (`layers::pin_entry_population_conditions`); the live game, triggers and entry observations keep the realized battlefield."
- `battlefield_entrant_projection_cases` case (b) comment: "Devotion is 1 (its own {W})" → "Pre-entry devotion is 0: its own {W} is not counted (CR 614.12 + CR 700.5, Heliod ruling 2020-01-24)". The outcome is unchanged (not a creature).

**Pre-entry means before the event (charter decision 1).** Established, not re-defined: every gate consumer moves one object (P7), and both projection builders (`enter_with_counters_for_object`, `effective_enter_mods`) run before that object's move [read], so `state` at the call is the pre-event state. The multi-object loop (`process_one_zone_move_with_terminal`) would hand a later member a state containing earlier members; no printed gate consumer reaches it (residual R5-g).

**Live state is never touched.** The pin writes only `scratch`. Ruling (b) is guarded by V3 and by the post-entry creature assertions in V1/V4/V5.

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
- **U1:** every population- or origin-zone-reading `StaticCondition` leaf, under any static or transient, inside the projection. Printed carriers: 28 own statics (24 Theros Gods, Rusted Relic, Phoenix Fleet Airship, Grond, and the "enchanted permanent" Auras) and 1 group static (Starfield of Nyx) (P8), crossed with the 6 gate consumers (P7).
- **U2:** every kind-level keyword filter evaluated on a held object that is not the live object (13 caller classes above); zero printed reach today (P10).
- **U3:** every enters observer whose record has no observation.
- **U5:** the any-of anaphoric entry gate building block (0 printed, P9).
- **U6:** harness hardening for every `drive_to_empty` caller, including phase 2's.

## Sizing

| Unit | Behavior | Registration surfaces | Discriminating test |
|---|---|---|---|
| U6 | L1/C5 settled assertion | `tests/integration/etb_trigger_entry_time_characteristics.rs` | V20 |
| U2 | B2/C2 object-scoped keyword kinds | `game/keywords.rs`, `game/filter.rs` (`matches_filter_prop`) | V7, V8, V10 |
| U3 | C3 incarnation-checked fallback | `game/filter.rs` | V11 |
| U1 | B1 pre-entry population pin | `game/layers.rs`, `game/filter.rs` (projection) | V1, V4, V5, V6c, V6d (integration); V6a/V6b (unit) |
| U5 | C4 any-of entry gate | `parser/oracle_effect/conditions.rs` | V18, V19 |

C6 is comment-only, not a unit. **5 units → T1 fires (≥ 2).**

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

**8 counted if test files count, 5 if they are classed as fixtures. T2 (≥ 13) does not fire either way, so the T1 ∧ T2 conjunction does not fire.** No compiler-forced site: no signature, type or variant changes (`object_has_effective_keyword_kind` keeps its signature). No new integration `mod` line (all rows go in existing files, so `tests/integration/main.rs` is untouched). Card data: regenerated by the pipeline, excluded; expected identical (P9; U1–U3 are runtime-only).

## Building Blocks

- U1: `layers::evaluate_condition_with_context`, `ConditionContext::ability_holder`, `static_condition_uses_object_population`, `static_condition_reads_zone_membership`, `condition_uses_recipient_context` (all existing, private to `layers.rs`; the new function sits beside them, so no visibility widening beyond the new `pub(crate)` entry), `condition_controller` semantics, `StaticCondition::None`, `Arc::make_mut`. New helpers justified: no existing primitive evaluates a condition against one state and installs the result in another.
- U2: `keywords::has_keyword_kind`, `off_zone_characteristics::off_zone_has_keyword_kind`. The new function is the object-scoped form of the existing effective lookup, which now delegates to it.
- U3: the `entered_incarnation.is_none_or(..)` convention; `matches_target_filter_on_zone_change_record_licensed`.
- U5: `TypeFilter::AnyOf`, `core_type_to_type_filter`.

## Logic Placement

- U1: condition semantics belong to `layers.rs` (condition-evaluation authority); `filter.rs`'s projection only calls it. Engine-only.
- U2: keyword authority is `keywords.rs`; the filter only forwards the object it holds.
- U3: the entry-observation matcher (`filter.rs`) is the authority.
- U5: the parser's single condition converter (`conditions.rs`).
- U6: test harness.

## Rust Idioms

- No bools in data: the pin's truth is mapped through one constructor onto an existing enum shape.
- Slice patterns `[]` / `[core]` / `types` in U5; exhaustive in practice (three arms cover every slice).
- `is_none_or` / `is_some_and` / `Option::filter` in U3; copy-on-write via `Option<StaticCondition>` return + `Arc::make_mut`.
- `std::ptr::eq` for object identity (existing idiom in `off_zone_characteristics.rs`).

## Nom Compliance

`parser/oracle_effect/conditions.rs` changes a slice match only. No detection, dispatch or classification on text is added; no `contains`/`starts_with`/`find`. No Oracle text becomes newly accepted.

## Extension vs Creation

Extensions: U2 (object-scoped form of the existing authority), U3 (existing convention), U5 (existing converter, existing variant), U6. Creation: the U1 pin function (justified above). No new enum, variant or field.

## Variant Discoverability

`cargo engine-inventory` was run (P6). No variant is added in phase 1, so the `/add-engine-variant` gate is not triggered; it is `DEFERRED(phase 2)` for the onset enum.

## Identity / Provenance Contract

- **"If a creature enters this way" / "it's a creature" / "if it's an artifact or creature"** (Recommission, Release Juno, synthetic): the object is the ChangeZone's moved entrant, read as CR 614.12's would-be permanent: own statics and existing effects apply; their population/origin-zone leaves are bound to the pre-entry state at projection time (snapshotted, scratch-local, discarded with the projection); "you" = the scratch carrier's controller (post-override for the entrant, CR 110.2a; the effect controller for a transient). Hostile fixture: V6c/V6d (owner ≠ controller).
- **Entry observation:** identity `record.object_id` + `record.entered_incarnation` (CR 400.7). With an observation, kind-level reads use the observed object (U2). Without one, the live object is used only while it is the entered incarnation (U3). Hostile fixtures: V8 (departed), V11 (re-incarnated).
- **Record skeletons:** event-time snapshots; kind-level reads use the snapshot (U2). Hostile fixture: V10.

## Verification Matrix

Integration rows go in existing files under `crates/engine/tests/integration/`; their `mod` lines already exist. Lib unit rows sit beside their `pub(crate)` seams (filter.rs `entry_observation_tests` / projection tests, layers.rs `mod tests`) because those seams have no public door. "Base" values marked [probed] come from Step 3.5; [read] values are derived from code and labelled.

| # | Claim | Seam / production entry | Test (file, name) | Revert-failing assertion (base → fixed) | Reach guard / hostile siblings |
|---|---|---|---|---|---|
| V1 | B1: pre-entry devotion 4 → Heliod isn't a creature as it would enter → 0 counters (ruling a) | `pin_entry_population_conditions` ← `BattlefieldEntrantProjection::new` ← `enter_with_counters_for_object` ← Recommission cast | `conditional_entry_gate_projection.rs`, rewrite `recommission_reads_entrants_own_isnt_a_creature_static`: helper `recommission_on_heliod(ww_anchors, w_anchors)`; case (2, 0) | counters **1 → 0** [probed P1] | reach: Heliod on the battlefield and **a creature after entry** (live devotion 5 counts its own {W}, ruling b); no `PutCounter` event |
| V2 | B1 positive control: pre-entry devotion 5 → 1 counter | same | same test, case (2, 1) (two {W}{W} + one {W}) | 1 → 1 [probed P1b] (control) | proves the gate still applies when the condition already holds; case (0, 0) stays 0 / not a creature [probed P1e] |
| V3 | B1 ruling (b): a creature-enters observer sees Heliod as a creature; the pin does not leak into live state (F4: Elemental Bond, which gains no life) | live `trigger_matchers` enters observer | same file, new `recommission_heliod_four_devotion_still_entered_as_a_creature`: case (2, 0) + Elemental Bond | draws 1 → 1 [probed P1c] **and** counters 1 → 0 | a pin leaking into live state would make Heliod a noncreature and draw 0. Soul Warden is not used: at base it yields 2 counters via Heliod's own life-gain trigger [probed P1d] |
| V4 | B1 class, own static with `ObjectCount` (Rusted Relic) | same projection, `QuantityComparison` leaf | same file, new `release_juno_rusted_relic_counts_pre_entry_artifacts`: 2 / 3 other artifacts | 2 others: counters **2 → 0** [probed P2]; 3 others: 2 → 2 | reach: Relic on the battlefield in both; at 2 others it is a 5/5 artifact creature after entry (live count 3, ruling-b analog); at 3 others 7/7 |
| V5 | B1 class, another object's group static (Starfield of Nyx) | same projection, other-object static carrier | same file, new `release_juno_sanctum_counts_pre_entry_enchantments`: Starfield + 3 / Starfield + 4 others, anchors with mana value 1 | 4 pre-entry: counters **2 → 0** [probed P3]; 5: 2 → 2 | reach: Sanctum on the battlefield and a 1/1-based creature after entry in both (live count 5); anchors carry mana value 1 so they survive as creatures (P3 note) |
| V6a | B1 pin unit: population leaf pinned; recursion; non-pinned leaves | `layers::pin_entry_population_conditions` | `layers.rs` `mod tests`, new `pin_entry_population_conditions_cases` | (a) `Not(DevotionGE)` on a scratch static becomes `Not(pinned(pre-entry truth))`; (b) `And[DuringYourTurn, IsPresent]` pins only the `IsPresent` child; (c) a recipient-context leaf (`RecipientMatchesFilter`) and a non-population leaf (`SourceIsTapped`) are untouched and the definitions `Arc` is not cloned (`Arc::ptr_eq` before/after); (d) a graveyard-count `QuantityComparison` leaf pins with `origin = Some(Graveyard)` and is untouched with `origin = Some(Hand)`; (e) a transient's population condition pins with `tce.controller` | (c) are the hostile negatives; base: fn absent (unit, [read]) |
| V6b | F3 unit case: entrant owned by P1 entering under P0 | `BattlefieldEntrantProjection::new(&state, id, Some(P0), false)` | `filter.rs` projection tests, new `battlefield_entrant_projection_reads_new_controllers_pre_entry_population`: Heliod-shaped static (P0 devotion 4, P1 6) and Rusted Relic-shaped static (P0 2 artifacts, P1 3) | both not a creature (base: creature, [read]; owner semantics would also give creature) | sibling: P0 devotion 5 / P0 3 artifacts → creature |
| V6c | F3 runtime, `DevotionGE` | Recommission-voiced synthetic "Put target creature card from an opponent's graveyard onto the battlefield under your control. If a creature enters this way, it enters with an additional +1/+1 counter on it." | `conditional_entry_gate_projection.rs`, new `stolen_heliod_counts_new_controllers_pre_entry_devotion` | counters **1 → 0** [probed P4]; an owner-keyed pin would give 1 | premise: slot `[(Creature, P1P1, 1)]`, `enters_under: Some(You)` [probed]; reach: Heliod on the battlefield under P0 |
| V6d | F3 runtime, `ObjectCount` `QuantityComparison` | same synthetic with "target artifact card" | same file, new `stolen_rusted_relic_counts_new_controllers_pre_entry_artifacts` | counters **1 → 0** [probed P4]; owner-keyed would give 1 | reach: Relic on the battlefield under P0, a creature after entry (P0 now controls 3 artifacts) |
| V7 | B2: kind-level answers read the observation, both polarities | `matches_filter_prop` kind arms ← `matches_target_filter_on_entry_observation` | `filter.rs` `entry_observation_tests::entry_observation_shapes_read_the_projection`, beside its `WithKeyword` shape: `HasKeywordKind(Flying)` and `WithoutKeywordKind(Flying)` with the live entrant gaining Flying after entry | `HasKeywordKind`: (evaluator, live) = (true, true) → **(false, true)**; `WithoutKeywordKind`: (false, false) → **(true, false)** [read] | the existing `WithKeyword` assertion `(false, true)` is the paired positive |
| V8 | B2: a departed entrant keeps its kind-level answer | skeleton branch | `filter.rs` `entry_observation_answers_for_departed_entrant`: fixture entrant has Flying at entry; after removal of the object, `HasKeywordKind(Flying)` | **false → true** [read]; plus the moved case (incarnation + 1, graveyard, keywords and base keywords cleared): false → true [read] | negative sibling: `HasKeywordKind(FirstStrike)` (never had) stays false |
| V9 | B2: live paths unchanged | `object_has_effective_keyword_kind` → `effective_keyword_kind_of` | `keywords.rs` `mod tests`, new `effective_keyword_kind_of_live_and_snapshot`: a graveyard card granted a kind by an off-zone grant answers true through both entries; a detached clone of it answers from its own keywords (false) | preservation (live); clone false [read] | reference reading: see Reference Readings |
| V10 | B2 (F5 sibling): attack-declaration record answers the declaration-time kind | `matches_target_filter_on_attack_declaration_record` (pub) | **`etb_trigger_entry_time_characteristics.rs`** (see V10 placement note) | `HasKeywordKind(Flying)` **false → true**, `WithoutKeywordKind(Flying)` **true → false** [probed P11] | positive: `WithKeyword(Flying)` = true on the same record [probed]; negative: `HasKeywordKind(FirstStrike)` false |
| V11 | C3: the no-observation fallback rejects a later incarnation | `matches_target_filter_on_entry_observation`, no-observation branch | `filter.rs` `entry_observation_tests`, new `entry_observation_absent_rejects_later_incarnation`: `fixture(false)`, two counters added, then `incarnation += 1` on the live object; filter `power_at_least(3)` | **true → false** [read] | positive sibling: same incarnation → live arm true (existing `entry_observation_absent_matches_todays_arms` stays green); record arm answer equals `matches_target_filter_on_zone_change_record_licensed` |
| V18 | C4 parse: multi-type rider folds into an any-of gate | `card_type_condition_as_entry_this_way` | `parser/oracle_effect/tests.rs`: rename `multi_type_entry_rider_is_not_folded` → `multi_type_entry_rider_folds_into_any_of_gate`; assert the gate is `[(Typed(AnyOf[Artifact, Creature]), P1P1, 2)]` and no surviving sub. `conditions.rs` `card_type_condition_as_entry_this_way_cases`: the `[Artifact, Creature]` row now expects `Some(ZoneChangedThisWay{AnyOf[..]})`; add `[]` → `None` | not folded → folded [read; P5 measured the unfolded base shape] | hostile: additional/subtype filter → `None`, `Not` → `None`, single type → plain filter (existing rows) |
| V19 | C4 runtime (F9, required): counters are part of the entry (CR 614.1c) | `entry_gate_matches` with `AnyOf` ← Release Juno | `conditional_entry_gate_projection.rs`, new `release_juno_any_of_gate_is_an_entry_replacement` with `MULTI_TYPE_RELEASE_JUNO` (synthetic, labelled; What Must Be Done's mode with "if it's an artifact or creature") + Elemental Bond | case 1, Memnite (1/1 artifact creature): draws **0 → 1**, `PutCounter` event **present → absent** [probed P5]; counters 2 → 2, 3/3 | case 2, Sol Ring (noncreature artifact): counters 2 → 2, `PutCounter` event present → absent; case 3, Sanctum (legendary noncreature non-artifact enchantment): 0 → 0, still on the battlefield. F9 named "a 2/2 artifact creature"; Memnite (1/1 → 3/3, power ≥ 3) is the measured equivalent |
| V20 | L1/C5: the harness proves it settled | `drive_to_empty` | the assertion itself | settled = true for r5b, r5d×2, r11, r12 [probed P12] | structural (no non-settling caller exists to drive red); the prompts list is in the panic message |
| — | C1 rows (Kardur's III, Hunting Wilds, Rampaging Growth, is-form regression and classifier rows) | — | — | `DEFERRED(phase 2)` | — |

**V10 placement:** the row goes in `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs` (not `conditional_entry_gate_projection.rs`, which covers entry gates) beside the observation rows, named `attack_declaration_record_reads_declaration_time_keyword_kind`, built with `GameScenario` (a creature given Flying, `AttackDeclarationRecord { object_id, incarnation: Some(..), lki: obj.snapshot_public_characteristics(), is_token: false, is_commander: false }`, then Flying removed from the live object), exactly as P11 measured.

Coverage-status impact: none. No Oracle text changes its supported/unsupported status (P9; U1–U3 are runtime-only). The executor confirms by regenerating card data and diffing.

Integration-file header updates: `conditional_entry_gate_projection.rs`'s module doc and REVERT DISCRIMINATORS lines that state the old Heliod reading ("With two {W}{W} permanents, devotion is 1 + 4 = 5 …", "`recommission_reads_entrants_own_isnt_a_creature_static` gets 1 counter (expected 0)") are rewritten to the pre-entry reading and the new rows; the Release Juno constant keeps its verbatim text.

## Reference Readings

- **V2, V4 (3 others), V5 (5 enchantments) positive controls.** Derived from CR 614.12 and Heliod ruling (a), not from parity. With the condition already met pre-entry, the entrant is a creature as it would exist on the battlefield, so the gate applies: 1 counter for V2 and 2 counters for V4/V5. Base gives the same values, by its own route.
- **V3 draw (1).** Derived from ruling (b) and Elemental Bond's text: after entry, live devotion is 5 including Heliod's {W}, so Heliod is a 5/5 creature entering (power ≥ 3) → draw 1. Base gives 1 [probed]. The fixed counter count (0) is V1's derivation.
- **V9 live path.** Derived: a live off-battlefield object's kind answer is its effective off-zone keywords (CR 613.1 continuous effects that apply to it where it is). Measured reference: the existing off-zone keyword tests in `keywords.rs` / `off_zone_characteristics.rs`, which the executor runs; reference equals derivation [read].
- **V11 positive sibling / `entry_observation_absent_matches_todays_arms`.** Derived: the same incarnation is the same object (CR 400.7), so the live answer is the entrant's. Base equals derivation [read].
- **V19 cases 2 and 3.** Derived from the synthetic text: Sol Ring is an artifact → gate true → 2 counters as the entry; Sanctum is neither → 0. Base gives the same counts [probed] by a later `PutCounter` route; the rows assert the derived counts plus the absent event.
- **Defective references fixed in this phase:** Heliod at pre-entry devotion 4 (base 1, derived 0), Rusted Relic at 2 others (base 2, derived 0), Starfield at 4 (base 2, derived 0), the stolen-entrant rows (base 1, derived 0), the kind reads on observations and records, and the multi-type rider (base: later `PutCounter`, derived: entry). Every row asserts the derived value.

## Step-by-step plan

1. **U6.** Edit `drive_to_empty` as above. Run the five callers.
2. **U2.** Add `keywords::effective_keyword_kind_of`; make `object_has_effective_keyword_kind` delegate; switch both `matches_filter_prop` kind arms. Add V7, V8 (filter.rs), V9 (keywords.rs), V10 (integration).
3. **U3.** Incarnation filter on the no-observation arm and its doc sentence; add V11.
4. **U1.** In `layers.rs`: `pinned_static_condition`, `pin_population_leaves`, `pin_entry_population_conditions`, the `ability_holder` doc sentence. In `filter.rs`: origin capture, the call before `mark_layers_full`, struct doc, case (b) comment. Rewrite the Recommission/Heliod row (V1/V2), add V3, V4, V5, V6c, V6d (integration), V6a (layers.rs), V6b (filter.rs).
5. **U5.** Slice match in `card_type_condition_as_entry_this_way` + doc; update the conditions unit case; rename/rewrite the parser test (V18); add V19.
6. **C6.** Doc comment on `entry_gate_projection`.
7. `cargo fmt --all`; `./scripts/check-parser-combinators.sh`; Tilt `clippy` / `test-engine` / `card-data` when Tilt is up, otherwise the focused filters `cargo test -p phase-engine --features test-support --test integration -- conditional_entry_gate_projection etb_trigger_entry_time_characteristics` and `cargo test -p phase-engine --lib -- filter:: layers:: keywords:: oracle_effect::` under the caller's build constraints. Regenerate card data and confirm no diff.

CR annotations to add (all verified above): CR 614.12 + CR 700.5 + CR 109.5 + CR 110.2a (U1), CR 603.10 + CR 608.2i + CR 400.7 (U2), CR 400.7 (U3), CR 614.1c + CR 614.12 (U5).

## Residuals (run-level; reach established here)

- **R5 (B1 limits), reach per P7/P8:**
  - (a) population leaves that also use recipient context are not pinned: 0 among type-writing statics [probed P8];
  - (b) dynamic P/T quantities in modifications are not conditions: no gate consumer reads P/T — every gate filter is a core type or subtype [probed P7];
  - (c) a layer-1 copy effect in the scratch copies a source's pinned definitions: not measured; no gate consumer's entrant is projected as a copy by `BattlefieldEntrantProjection` [unprobed];
  - (d) `ForAsLongAs` durations of transients are pruned, not pinned: not measured [unprobed];
  - (e) granted statics carrying a population-gated type write: 0 [probed P8];
  - (f) a population leaf also reading the source's own state is pinned wholesale: 0 among type-writing statics [probed P8];
  - (g) multi-object `ChangeZone` iteration would count earlier co-entrants: no gate consumer is multi-entrant [probed P7].
- **R6:** C6's two-projection bound; no printed entrant carries both gates [probed P7].
- **U2 residual:** the owner-zone clone path drops off-zone grants for a kind read; production-unreachable [read].
- R1–R4, R7, R8 and all C1 content: phase 2 / run-level, untouched here.
