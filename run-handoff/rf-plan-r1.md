# Plan r1: etb-review-fixes (PR #10036 review findings B1, B2, L1, C1–C6)

BASE_SHA = e56851e69a75e2cae192189f36e80ec33dbc3e21 (HEAD, verified with `git rev-parse HEAD`).
Mode: ordinary. Budget: standard. Out of scope: the Zimone "create a token, then put N counters on it" parser path (untouched), and the docstring-coverage pre-merge note.

Evidence labels: **[probed]** means a measurement was run and is quoted. **[read]** means it was established by reading code at BASE_SHA. **[unprobed]** means it is a claim the executor must confirm. The box was disk-saturated (97%, about 1.4G free) and another cargo integration job held the target lock, so this round compiled no new engine probe tests. The runtime rows below are designed and labelled, not pre-run.

---

## Step 0: Premise verification (Oracle text, MTGJSON `/home/user/wt/data-root/mtgjson/AtomicCards.json`)

Each card's text was checked verbatim against AtomicCards.json **[probed]**.

| Card | Oracle text (relevant clause) | Used for |
|---|---|---|
| Heliod, Sun-Crowned {2}{W} | "As long as your devotion to white is less than five, Heliod isn't a creature." Rulings 2020-01-24: (a) "As a God enters the battlefield, your devotion to its color will determine whether any replacement effects that affect creatures entering the battlefield apply to that God. Because replacement effects are considered before the God is on the battlefield, the mana symbols in its mana cost won't be counted when determining this." (b) "When a God enters the battlefield, your devotion to its color (including the mana symbols in the mana cost of the God itself) will determine if a creature entered the battlefield or not for abilities that trigger whenever a creature enters the battlefield." | B1 |
| Recommission | "Return target artifact or creature card with mana value 3 or less from your graveyard to the battlefield. If a creature enters this way, it enters with an additional +1/+1 counter on it." | B1 |
| What Must Be Done (Let the Past Return mode) | "Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature." | B1, C4 |
| Rusted Relic {4} | "Metalcraft — Rusted Relic is a 5/5 Golem artifact creature as long as you control three or more artifacts." | B1 class row |
| Starfield of Nyx | "...As long as you control five or more enchantments, each other non-Aura enchantment you control is a creature in addition to its other types and has base power and base toughness each equal to its mana value." | B1 group row |
| Sanctum of Tranquil Light | Legendary Enchantment — Shrine (historic, non-Aura) | B1 group row (entrant) |
| Soul Warden | "Whenever another creature enters, you gain 1 life." | B1 post-entry observer |
| Elemental Bond | "Whenever a creature you control with power 3 or greater enters, draw a card." | C1 observer |
| Kardur's Vicious Return, chapter III | "Return target creature card from your graveyard to the battlefield. Put a +1/+1 counter on it. It gains haste until your next turn." | C1 |
| Hunting Wilds | "Kicker {3}{G} / Search your library for up to two Forest cards and put them onto the battlefield tapped. Then shuffle. / If this spell was kicked, untap all Forests put onto the battlefield this way. They become 3/3 green creatures with haste that are still lands." | C1 |
| Call a Surprise Witness | "Return target creature card with mana value 3 or less from your graveyard to the battlefield. Put a flying counter on it. It's a Spirit in addition to its other types." | C1 residual |
| Rise from the Grave, Path of the Schemer | is-form ("is a black Zombie in addition…", "It's a Phyrexian in addition…") | C1 regression rows r7/r12 |
| Oviya Pashiri; Heroic Return; Winter Soldier; Summoner's Grimoire; Semester's End; Spark Double | gate consumers (type and subtype gates) | B1 consumer measurement, C4 note |

Premise corrections to the review text: none. The review's Heliod and Recommission quotes and the rulings match MTGJSON. The C4 orchestrator note ("no printed card uses a multi-type 'enters with … if it's an X or Y' rider") is consistent with the C4 corpus grep **[probed]**.

CR numbers, each grep-verified in `docs/MagicCompRules.txt` **[probed]**:

| Rule | Line in MagicCompRules.txt |
|---|---|
| 614.12 | 3104 |
| 614.12a | 3109 |
| 614.1c | 3066 |
| 700.5 | 3242 |
| 611.2e | 2921 |
| 611.3c | 2933 |
| 613.1 | 2964 |
| 603.10 | 2642 |
| 603.6a | 2605 |
| 608.2c | 2799 |
| 400.7 | 1955 |
| 702.26b | 4185 |

## Step 1: Applicable skills (checklist skeletons)

- `/add-static-ability`: the condition evaluation used by the B1 projection-scope pinning.
- `/add-replacement-effect`: the CR 614.12 entrant projection, and the C4 entry-gate fold.
- `/add-trigger`: entry-observation matcher arms (B2, C3) and the deferral predicate (C1).
- `/oracle-parser`: the C1 onset classifier and the C4 condition conversion. Nom mandate.
- `/add-engine-variant`: the new `ContinuousEffectOnset` enum and the new field (C1). Mandatory gate.
- `/card-test`: every integration row (GameScenario plus `GameRunner::cast(..).resolve()` with verbatim Oracle text).

Checklist steps that apply: types, then parser, then resolver/predicate, then serde mirrors, then card-data regen, then tests. No frontend, AI or multiplayer-filter steps apply: no new effect, no WaitingFor, no new player choice.

## Step 2: Analogous traces

- **C1 field (per-node parse-stamped metadata)**: traced `SubAbilityLink` through these sites.
  - `types/ability.rs`: the `SubAbilityLink` enum; the `AbilityDefinition.sub_link` struct field; the hand-written serde Repr/Serialize/Deserialize mirrors; `AbilityDefinition::new()`; `ResolvedAbility.sub_link` with its PartialEq destructures and `ResolvedAbility::new()`.
  - Parser stamping: `parser/oracle_effect/assembly.rs` and `sequence.rs`, at the `def.sub_link = …` sites.
  - Conversion: `game/ability_utils.rs` (`resolved.sub_link = def.sub_link;`) and `game/effects/separate_piles.rs` (same).
  - Consumers: `game/effects/mod.rs`.
  - The registry list in `parser/audit_projection.rs` (`"sub_link"` key).
  - Compiler-forced destructures: `game/quantity.rs`, `game/effects/mod.rs`, `game/stack.rs`, `game/ability_rw.rs`, `game/text_substitution.rs`, `game/ability_scan.rs`, plus the struct literals in the `game/effects/{skip_next_turn,player_counter,additional_phase,reverse_turn_order,extra_turn,double,vote,skip_next_step,grant_extra_loyalty_activations}.rs` files. The executor regenerates this list with `grep -rn "sub_link:" crates/engine/src`.
- **B1 (condition evaluation against a population other than the scratch's)**: traced the phase-1 `BattlefieldEntrantProjection::new` (`game/filter.rs`).
  - `move_to_zone_with_entry_flags`, then `apply_battlefield_entry_controller_override`, then `mark_layers_full` / `flush_layers` (`game/layers.rs`), then `evaluate_condition_inner` → `DevotionGE` → `devotion::count_devotion` (iterates `state.battlefield`).
  - Also `entered_object_perturbs_static_condition` / `static_condition_uses_object_population` / `static_condition_reads_zone_membership` (`game/layers.rs`). These already classify which `StaticCondition` leaves read battlefield population or zone membership; they are the classifiers this plan reuses.
- **B2/C2**: traced `FilterProp::WithKeyword` (view-scoped `obj.has_keyword`) against `HasKeywordKind` (`keywords::object_has_effective_keyword_kind(state, object_id, …)`, which reads the live object) inside `filter.rs::matches_filter_prop`, reached from `matches_target_filter_on_entry_observation` → `filter_inner_for_object`.
- **C3**: traced `engine_replacement.rs::replay_deferred_entry_events` → `process_triggers` → `trigger_matchers.rs::zone_change_clause_matches` (the `enters_observer` arm) → `filter.rs::matches_target_filter_on_entry_observation`, both the no-observation branch and the observation branch. The incarnation convention is `types/game_state.rs::battlefield_entry_observation` (`record.entered_incarnation.is_none_or(|inc| row…incarnation == inc)`).
- **C4**: traced `conditions.rs::anaphoric_entry_counter_gate` → `card_type_condition_as_entry_this_way` → (callers `oracle_effect/mod.rs` and `sequence.rs::is_anaphoric_entry_counters_clause`) → `lower.rs::fold_enters_this_way_counter_rider` → `conditional_enter_with_counters` → `change_zone.rs::enter_with_counters_for_object` → `entry_gate_matches` → `BattlefieldEntrantProjection::matches`.

## Step 3: Files read (all at BASE_SHA)

- `game/filter.rs`: projection, entry-observation matcher, `matches_filter_prop`, and the unit tests `battlefield_entrant_projection_cases` and `entry_observation_*`.
- `game/keywords.rs`
- `game/layers.rs`: `evaluate_condition*`, `ConditionContext`, the population, zone and recipient classifiers, `seed_live_characteristics_from_base`.
- `game/devotion.rs`
- `game/effects/change_zone.rs`: `EntryGateContext`, `entry_gate_*`, `enter_with_counters_for_object`, `effective_enter_mods`.
- `game/effects/mod.rs`: `settle_entries_before_instruction`, the D1 walkers, `entry_observation_deferred`.
- `game/engine_replacement.rs`: capture and replay.
- `game/trigger_matchers.rs`
- `types/game_state.rs`
- `types/ability.rs`: `AbilityDefinition`, `ResolvedAbility`, `SubAbilityLink`, `Effect::GenericEffect`, `TypeFilter::AnyOf`.
- `game/ability_utils.rs` and `game/effects/separate_piles.rs`: the conversion sites.
- `parser/oracle_effect/{assembly,subject,conditions,lower,tests}.rs`
- The integration test files `conditional_entry_gate_projection.rs` and `etb_trigger_entry_time_characteristics.rs`
- `game/scenario.rs`: the `SpellCast` API, including `.accept_optional()` for kicker.

## Step 3.5: Probes run

| # | Probe | Result | Decision it informs |
|---|---|---|---|
| P1 | `rf/c1scan.py` over `/home/user/wt/data-out/p2/card-data.json`: chains with a battlefield-entering ChangeZone followed by a D1-kind node (GenericEffect / Animate / ReturnAsAura / ApplyPerpetual), and the subset with an intervening characteristic modification (PutCounter etc.) before the D1 node **[probed]** | 334 cards have enter → D1. 6 have enter → modification → D1: Call a Surprise Witness, Grishnákh, Hunting Wilds, Kardur's Vicious Return, Takklemaggot, The Bus Runner. A regex verb classification of the 334: about 165 after-entry (gains/becomes), about 68 with-entry (is/has), 9 perpetual, 3 mixed, 89 unclassified. Regenerate with `python3 -I rf/c1scan.py`. | C1 is a real, printed-card defect, and a typed onset is worth it. |
| P2 | Per-card reading of the 6 **[probed / derived]** | Wrong readings on printed cards at base: **Hunting Wilds kicked + Elemental Bond** draws 2 (correct is 0: the Forests enter as non-creature lands and "become" applies after); **Kardur's III + Bond on a 2/2** draws 1 (correct is 0: it enters 2/2, then gets a counter, then gains haste). No wrong reading for Grishnákh, The Bus Runner or Takklemaggot (their riders modify other objects or add non-P/T counters). Call a Surprise Witness is is-form: a residual, out of the onset's reach (see Residuals). | C1 rows |
| P3 | Synthetic oracle-gen probe (`rf/synth/`, `target/tool/oracle-gen synth`) of Hunting Wilds- and Kardur-shaped chains **[probed]** | Every rider is a separate `SubAbilityLink::SequentialSibling` clause (Hunting Wilds' SetTapState carries the AdditionalCostPaid condition). So a per-node onset stamp reaches the D1 node. | C1 field placement (node level, not effect level) |
| P4 | Synthetic modal-bullet probe of "Return … It enters with two additional +1/+1 counters on it if it's an artifact or creature." in both voices **[probed]** | Parses to `RevealedHasCardType{card_types:[Artifact, Creature]}` and stays a later PutCounter sub. The single-type version folds into `conditional_enter_with_counters`. The same text as a **non-modal** line parses to `Unimplemented "enters_with_gate"` at the line level, in both the single-type and multi-type versions. | C4 fix point is `card_type_condition_as_entry_this_way`. The non-modal routing gap is pre-existing (listed below). |
| P5 | `rf/b1scan.py` / `gates.py` / `grant.py` over card-data **[probed]** | Gate consumers (conditional entry counters and `enters_modified_if`) are type or subtype filters only: Creature (Recommission, What Must Be Done), Artifact (Oviya), Subtype Hero (Heroic Return, Winter Soldier), Enchantment via enters_modified_if (Summoner's Grimoire). Population-gated type-writing statics: **28 own-static cards** (24 Theros Gods, Rusted Relic, Phoenix Fleet Airship, Grond, plus 3 Aura "enchanted"-cases), **1 group static** (Starfield of Nyx), **0 granted** (GrantStaticAbility carrying a population-gated type write). | B1 pins conditions on every scratch object's statics (own and group) plus TCEs; the granted residual is empty. |
| P6 | C6 corpus check **[probed]** | No printed ChangeZone carries both a non-empty `conditional_enter_with_counters` and an `enters_modified_if` gate on the same entrant. | C6: no code change. |
| P7 | C3 production reach **[read]** | `capture_deferred_entry_events_if_mid_entry_choice` captures only while a mid-entry choice is pending, and replay runs `process_triggers` before `drain_pending_continuation`. No game action can move the entrant between capture and replay. | The C3 test sits at the seam (a lib unit test), and the fix is defensive hardening. |

Unprobed (the executor confirms each before relying on it):
- U-a: `clause_ir.source.fragment()` includes the clause subject (for example "It gains haste until your next turn") for the D1 kinds.
- U-b: `subject::find_predicate_start` returns the offset of "gains" / "become" / "is" for the five fragments in the C1 rows.
- U-c: The B1 integration rows give red at base with the values stated (the base values are derived from the code read: the scratch includes the entrant's own devotion and its own artifact or enchantment).
- U-d: The count of entering-chain D1 nodes left `None` after regeneration (measured by the executor).

---

## Findings: verdict and fix

### B1: holds (pre-entry population)
**Evidence.**
- `BattlefieldEntrantProjection::new` moves the entrant onto the scratch battlefield and then runs a full layer pass. `DevotionGE` → `count_devotion` iterates `scratch.battlefield`, which includes the entrant **[read]**.
- Heliod's static parses as `Not(DevotionGE{White,5})` gating `RemoveType Creature` **[probed]**.
- The current row `recommission_reads_entrants_own_isnt_a_creature_static` (2 WW anchors, so 4 devotion) asserts 1 counter. Ruling (a) says that is wrong: replacement effects are considered before the God is on the battlefield, so 4 devotion means Heliod isn't a creature and gets 0 counters.
- The scratch is right to carry the entrant's **own** statics (CR 614.12: "continuous effects from the permanent's own static abilities that would apply to it once it's on the battlefield"). What it gets wrong is the **population** those statics' conditions count.

**Class.**
- Any population- or zone-membership-reading condition of any static evaluated inside the projection: the entrant's own, a group static (Starfield of Nyx), or a TCE condition.
- Printed reach: 28 own-static cards plus Starfield of Nyx, crossed with the gate consumers in P5.

**Fix: pin population-reading condition leaves to their pre-entry truth inside the scratch.**

1. Add `pub(crate) fn pin_entry_population_conditions(scratch: &mut GameState, pre_entry: &GameState, entrant: ObjectId, origin: Option<Zone>)` to `game/layers.rs`.
   - It visits every scratch object's `base_static_definitions` (`Arc::make_mut`, only when a leaf actually pins, to avoid needless clones) and every `scratch.transient_continuous_effects[i].condition`.
   - It rewrites each `StaticCondition` tree recursively through `And` / `Or` / `Not`.
   - A **leaf** is pinned iff `!condition_uses_recipient_context(leaf) && (static_condition_uses_object_population(leaf) || origin.is_some_and(|z| static_condition_reads_zone_membership(leaf, z)))`.
     - The zone clause covers conditions such as "as long as N cards in your graveyard", which the entrant's departure from its origin zone would otherwise perturb.
     - These are the existing private classifiers, kept in the same module. No visibility widening beyond `pub(crate)` on the new function.
   - The pinned truth is `evaluate_condition_with_context(pre_entry, leaf, holder_controller, source_id, ConditionContext { ability_holder: (source_id == entrant).then_some(entrant_controller_in_scratch), ..ConditionContext::NONE })`.
     - Using the scratch controller keeps the controller override (CR 614.12, "taking into account replacement effects that have already modified how it enters") while the **population** stays pre-entry.
     - The executor confirms that `ConditionContext.ability_holder` is the field `IsPresent` reads for "you control"-relative counts, and passes the same value the live layer pass would.
   - The pinned value is represented through one documented helper, `fn pinned_condition(truth: bool) -> StaticCondition`: `StaticCondition::None` when true (it evaluates to true, layers.rs `StaticCondition::None` arm) and `Not(Box::new(StaticCondition::None))` when false.
     - Why no new variant: the pinned value is a constant truth, scratch-local, and never serialized or observed outside the projection.
     - **Variant-discoverability check**: grep the inventory for an existing constant-truth `StaticCondition`. If one exists (for example `Always` / `Never`), use it instead.
   - Doc comment: CR 614.12 + CR 700.5, citing ruling (a).
2. Call it in `BattlefieldEntrantProjection::new` after placement, transform and controller override, and **before** `mark_layers_full` / `flush_layers`.
   - It needs the pre-entry `&GameState` (the `state` argument, which is still the live pre-entry state) and the entrant's origin zone, which is read from `state.objects[&entrant].zone` before the scratch move.
3. Live layers, triggers and the entry observation are untouched. Ruling (b) (the post-entry creature observer) keeps reading the live, post-entry battlefield.
4. Fix the comment in the `battlefield_entrant_projection_cases` case (b): "Devotion is 1 (its own {W})" becomes "pre-entry devotion is 0: its own {W} is not counted (CR 614.12 + CR 700.5, Heliod ruling)". Re-check that case's expectation: with zero anchors it was already "not a creature", so the outcome is unchanged.

**Residuals (documented, not consumer reads at base).**
- Population leaves that **also** use recipient context are not pinned. Measured consumers: none.
- Dynamic P/T quantities inside `ContinuousModification` values (for example `*/*` CDAs counting creatures) are not conditions, so they are not pinned. No gate consumer reads P/T (all of P5 are type and subtype filters).

### B2 / C2: holds (keyword-kind reads the live object)
**Evidence.** `matches_filter_prop`'s `HasKeywordKind` / `WithoutKeywordKind` arms call `object_has_effective_keyword_kind(state, object_id, …)`. That reads `state.objects[object_id]`, the live object, not the observation view passed into `filter_inner_for_object` **[read]**. For a departed entrant (skeleton view), the live object is gone or off-battlefield, so the kind-level answer is lost. `WithKeyword` reads `obj.has_keyword` (the view) **[read]**. Cited rule: CR 603.10.

**Fix.**
- In `game/keywords.rs`, add `pub fn effective_keyword_kind_of(state: &GameState, obj: &GameObject, kind: KeywordKind) -> bool`:
  - when `obj.zone == Zone::Battlefield`, it returns `has_keyword_kind(obj, kind)`;
  - otherwise it returns `off_zone_characteristics::off_zone_has_keyword_kind(state, obj.id, kind)`, keeping the off-battlefield effective path.
- `object_has_effective_keyword_kind` becomes a lookup plus delegation, so there is a single authority.
- `matches_filter_prop` calls `effective_keyword_kind_of(state, obj, *value)` for both polarities, where `obj` is the object it already holds (the view on the observation path, and the live object elsewhere). Live-path semantics are unchanged.
- The executor checks the view's `zone`:
  - The observation view of a battlefield entrant has `zone == Battlefield`, because it is cloned from the scratch or observation.
  - The departed skeleton built by `GameObject::new` must also be stamped `Battlefield`. If it isn't, it must be so it answers from its observed keywords. Confirm this in `matches_target_filter_on_entry_observation`'s skeleton construction **[unprobed]**.
- Exhaustive scan: the other `object_id` reads in `matches_filter_prop` are combat and history identity reads (blocking, attacked-this-turn, etc.), not characteristics. They stay **[read]**.

### C3: holds at the seam; production route unreachable
**Evidence.** The no-observation branch of `matches_target_filter_on_entry_observation` takes the live arm whenever `state.objects.contains_key(&record.object_id)`, with no incarnation check **[read]**. It is unreachable through replay today (P7), but the predicate is a public-in-crate authority and must match the `battlefield_entry_observation` convention. Cited rule: CR 400.7.

**Fix.** The live arm requires `state.objects.get(&record.object_id).is_some_and(|live| record.entered_incarnation.is_none_or(|inc| live.incarnation == inc))`. Otherwise it falls to `matches_target_filter_on_zone_change_record_licensed` (the record snapshot). Doc comment: CR 400.7 (a new object after the zone change).

### C1: holds (blanket D1)
**Evidence.**
- `effect_may_apply_with_entry` is a membership test on the effect kind, and `resolved_chain_may_apply_with_entry` walks the whole remaining chain **[read]**.
- So Kardur's III (ChangeZone → PutCounter → GenericEffect "gains haste") defers settlement past the PutCounter. The entry observation is then taken on a 3/3, and Elemental Bond draws.
- CR 611.2e: an "is/has/doesn't have" effect from a resolving spell that puts the object onto the battlefield applies with the entry; "becomes/gains" does not. Corpus: P1 and P2.

**Fix: typed onset, stamped at parse time and consumed by D1.**
1. In `types/ability.rs`, add `pub enum ContinuousEffectOnset { SimultaneousWithEntry, AfterEntry }` (serde, Copy, Eq, doc comment citing CR 611.2e).
   - Add the field `pub continuous_effect_onset: Option<ContinuousEffectOnset>` on `AbilityDefinition`, in every hand-written serde mirror (struct, Repr, Serialize destructure, Deserialize, `new()` with `None`, `skip_serializing_if = "Option::is_none"`, `default`).
   - Add the same field on `ResolvedAbility`: the PartialEq destructures and `new()`.
   - Add the key to `parser/audit_projection.rs`'s key list, next to `"sub_link"`.
   - Fill the compiler-forced destructures and struct literals listed in the Step 2 trace.
   - `None` means "not a D1-kind node, or onset unknown", so a wrong default is impossible.
2. Copy the field at `game/ability_utils.rs` (next to `resolved.sub_link = def.sub_link;`) and `game/effects/separate_piles.rs` (same).
3. Add the parser classifier `pub(super) fn classify_continuous_effect_onset(fragment: &str) -> Option<ContinuousEffectOnset>` in `parser/oracle_effect/subject.rs`, next to `find_predicate_start`. It is nom only, operating on the lowercased fragment:
   - `alt((tag("it's "), tag("they're ")))` maps to `SimultaneousWithEntry`.
   - Otherwise, `find_predicate_start(text)` gives the predicate offset (a structural word-boundary offset, the existing building block). On the remainder:
     - `opt(tag("then "))`, then `opt(tag("perpetually "))`;
     - then `alt((value(Simultaneous, alt((tag("is "), tag("are "), tag("isn't "), tag("aren't "), tag("has "), tag("have "), tag("doesn't have "), tag("don't have ")))), value(AfterEntry, alt((tag("becomes "), tag("become "), tag("gains "), tag("gain "))))))`;
     - anything else gives `None`.
   - Verb axes are composed as single `alt()`s, with no permutation enumeration.
   - Doc comment: CR 611.2e. Tests are classifier unit tests over each verb axis, plus the relative-clause case that `find_predicate_start` already guards ("each creature that is … becomes" gives AfterEntry).
4. Stamp it in `parser/oracle_effect/assembly.rs` at the `ClauseIr` → `AbilityDefinition` construction (`let mut def = AbilityDefinition::new(kind, clause_effect);`). Precedent: the CR 702.26a SetTapState description read of `clause_ir.source.fragment()` in the same function.
   - When `def.effect` is a D1 kind, set `def.continuous_effect_onset = classify_continuous_effect_onset(clause_ir.source.fragment())`.
   - The D1-kind test is the **same predicate** D1 uses: expose `effect_may_apply_with_entry` as `pub(crate) fn effect_kind_may_apply_with_entry(&Effect)` from `game/effects/mod.rs`, or move it onto `Effect` as an inherent method in `types/ability.rs`. The executor picks the inherent method if it is cleaner, since it puts the kind list in one authority.
   - Also stamp at the other def-construction sites in `assembly.rs` that build D1 kinds from a clause, if any. The executor greps `AbilityDefinition::new(` in `assembly.rs` and handles each D1-capable site.
5. In `game/effects/mod.rs`, change D1 to `fn node_may_apply_with_entry(effect: &Effect, onset: Option<ContinuousEffectOnset>) -> bool { effect_kind_may_apply_with_entry(effect) && onset != Some(ContinuousEffectOnset::AfterEntry) }`.
   - Both walkers, the frame walker and the library-search resume walker pass the node's field.
   - `None` stays conservative (it defers, as today). Exhaustive `match` on the onset rather than `!=` if clippy and readability prefer it.
6. Regenerate card data and measure U-d: the count of entering-chain D1 nodes with `None` onset. Report it. It does not gate the change, since `None` keeps base behavior.

### C4: holds (multi-type rider not folded)
**Evidence.** P4. `card_type_condition_as_entry_this_way` requires `let [core] = card_types.as_slice()` **[read]**. At runtime, `RevealedHasCardType` with several types means any-of **[read]**.

**Fix.**
- In `conditions.rs::card_type_condition_as_entry_this_way`, map `[core]` to the single type filter (unchanged).
- Map `[first, rest @ ..]` with `rest` non-empty to `TypeFilter::AnyOf(card_types.iter().map(|t| core_type_to_type_filter(*t)).collect())`.
- Map `[]` to `None`.
- `additional_filter` / `subtype_filter` `Some` stays `None` (the strict path, unchanged).
- Update the doc comment ("several types" is no longer in the `None` list) and the unit case `card_type_condition_as_entry_this_way_cases` (the `[Artifact, Creature]` input now gives `Some(ZoneChangedThisWay{AnyOf[Artifact, Creature]})`).
- Rewrite `multi_type_entry_rider_is_not_folded` (rename it to `multi_type_entry_rider_folds_into_any_of_gate`) so it asserts that the fold happened: the gate is non-empty with an `AnyOf` filter, and no surviving PutCounter sub.
- `is_anaphoric_entry_counters_clause` (`sequence.rs`) consumes the same function, so the swallow check follows automatically. The executor confirms this with the existing sequence tests.
- Printed impact is zero. This is building-block correctness, and the strict-failure alternative is not needed because AnyOf is a clean, typed representation.

### L1 / C5: holds
**Evidence.** `drive_to_empty` loops `for _ in 0..40` and breaks on Priority with an empty stack, but never asserts that it broke **[read]**.

**Fix.** Track `let mut settled = false;`, set it at the break, and after the loop run `assert!(settled, "drive_to_empty did not settle in 40 steps; prompts seen: {prompts:?}")`. Callers r5b, r5d, r11 and r12 are unchanged.

### C6: observation holds; zero printed impact, no code change
**Evidence.**
- Both `enter_with_counters_for_object` and `effective_enter_mods` build a projection only when their gate is non-empty or `Some` **[read]**. Neither builds one when its gate is empty.
- No printed card has both gates on one entrant (P6).
- Sharing would mean threading `Option<&BattlefieldEntrantProjection>` through `process_one_zone_move_with_terminal` and two pending-delivery paths (`mod.rs` and `engine_resolution_choices.rs`), where the two calls sit in different functions.

**Decision.** Not a clean refactor for zero printed cost. Add a single doc comment on `entry_gate_projection` naming the 2N bound and its printed-card reach of zero, so the decision is visible. Revisit it if a card with both gates is printed.

---

## Pattern Coverage

- **B1**: 28 own-static cards plus 1 group (Starfield of Nyx) with population-gated type writes, crossed with the type and subtype gate consumers (Recommission, What Must Be Done, Oviya Pashiri, Heroic Return, Winter Soldier, Summoner's Grimoire). It covers every population or zone leaf of `StaticCondition` under any static inside the projection, not just devotion.
- **B2**: every `HasKeywordKind` / `WithoutKeywordKind` filter on an entry-observation trigger (the class of "whenever a creature with <kind> enters").
- **C3**: every enters-observer with a no-observation record.
- **C1**: 334 enter → D1 chains. About 165 gain the precise AfterEntry reading. The printed defect reaches Hunting Wilds and Kardur's Vicious Return, and the class is any "ChangeZone → modification → becomes/gains" chain.
- **C4**: the building block for any "if it's an X or Y" anaphoric entry gate (0 printed today).
- **L1/C5**: test-harness hardening.

## Sizing

Units (one skill-checklist pass each):

| Unit | Behavior | Registration surfaces | Discriminating test |
|---|---|---|---|
| U1 | B1 pre-entry population pinning in the projection | `game/layers.rs` (pin fn and helper), `game/filter.rs` (call site, and the case (b) comment) | Heliod 4-devotion gives 0 counters (red at base); Rusted Relic and Starfield rows |
| U2 | B2 keyword-kind reads the view | `game/keywords.rs`, `game/filter.rs` | kind-level assertions in `entry_observation_shapes_read_the_projection`, plus the departed-skeleton kind case |
| U3 | C3 incarnation-checked fallback | `game/filter.rs` | lib unit test, with an incarnation bump on a no-observation record |
| U4 | C1 typed onset | `types/ability.rs` (enum, field, serde mirrors), `game/ability_utils.rs`, `game/effects/separate_piles.rs`, `parser/oracle_effect/subject.rs` (classifier), `parser/oracle_effect/assembly.rs` (stamp), `game/effects/mod.rs` (D1), `parser/audit_projection.rs` (key list), plus compiler-forced literals and destructures (standing class), plus regenerated card data | Kardur's III + Elemental Bond gives 0 draws (red at base: 1); Hunting Wilds kicked gives 0 draws (red at base: 2) |
| U5 | C4 AnyOf gate | `parser/oracle_effect/conditions.rs` | the rewritten `multi_type_entry_rider_folds_into_any_of_gate`, plus the conditions unit case |
| U6 | L1/C5 settle assertion | test fixture only | the assertion itself |

Dependency edges: none between units. U1–U6 are independent, and all touch disjoint logic. U1, U2 and U3 share `filter.rs` but edit disjoint functions. Suggested order: U6 (harness first, so later rows are trustworthy), then U2, U3, U1, U5, U4 (the largest, last).

Expected counted scope paths, excluding test files, test fixtures, regenerated card data and standing compiler-forced sites. There are **11**:
1. `crates/engine/src/game/layers.rs`
2. `crates/engine/src/game/filter.rs`
3. `crates/engine/src/game/keywords.rs`
4. `crates/engine/src/types/ability.rs`
5. `crates/engine/src/game/ability_utils.rs`
6. `crates/engine/src/game/effects/separate_piles.rs`
7. `crates/engine/src/parser/oracle_effect/subject.rs`
8. `crates/engine/src/parser/oracle_effect/assembly.rs`
9. `crates/engine/src/game/effects/mod.rs`
10. `crates/engine/src/parser/oracle_effect/conditions.rs`
11. `crates/engine/src/parser/audit_projection.rs` (shared registration file; it may be classed as standing)

`change_zone.rs` adds one doc comment for C6 (comment-only, standing). Test files: `tests/integration/conditional_entry_gate_projection.rs`, `tests/integration/etb_trigger_entry_time_characteristics.rs` and `parser/oracle_effect/tests.rs`, plus the inline unit modules.

Six units and about 10–11 counted paths is within T1 (multi-unit) and near T2 (paths). The orchestrator adjudicates phase-fit. A natural seam, if needed: phase A is U1, U2, U3, U5, U6 (6 paths); phase B is U4 (7 paths). No DEFERRED rows are needed either way, because the units are independent.

## Building Blocks

- **B1**: `layers::evaluate_condition_with_context`, `ConditionContext::NONE`, `static_condition_uses_object_population`, `static_condition_reads_zone_membership`, `condition_uses_recipient_context`, `devotion::count_devotion` (reached through evaluation), and `Arc::make_mut` on `base_static_definitions`. The one new helper is the pin function, justified because no existing helper evaluates a condition against one state and installs the result in another.
- **B2**: `keywords::has_keyword_kind` and `off_zone_characteristics::off_zone_has_keyword_kind`. The new `effective_keyword_kind_of` is the object-scoped form of the existing effective lookup (the existing fn delegates to it).
- **C3**: the `entered_incarnation.is_none_or(...)` convention from `GameState::battlefield_entry_observation`, and `matches_target_filter_on_zone_change_record_licensed`.
- **C1**: `subject::find_predicate_start`; nom `tag`, `alt`, `value` and `opt`; the `SubAbilityLink` field pattern; `ClauseIr.source.fragment()`.
- **C4**: `TypeFilter::AnyOf` and `core_type_to_type_filter`.

## Logic Placement

- **B1**: the population semantics of conditions belong to `layers.rs`, the authority for condition evaluation. The projection (`filter.rs`) only invokes it. Live state is never touched.
- **B2**: keyword authority is `keywords.rs`.
- **C3**: the matcher is the authority for entry observation (`filter.rs`).
- **C1**: onset is a **parse-time** fact about Oracle grammar ("becomes" against "is"), so the parser classifies it and the types carry it. The runtime (`effects/mod.rs`) only reads the typed field and does no string inspection.
- **C4**: the parser condition conversion (`conditions.rs`).

## Rust Idioms

- A typed `ContinuousEffectOnset` enum inside an `Option`, not a bool (two semantic values plus unknown).
- Exhaustive `match` in the D1 predicate.
- Slice patterns `[core]` / `[_, _, ..]` in C4.
- `is_none_or` / `is_some_and`.
- `Arc::make_mut` only after the pin is known to change something.
- `pinned_condition(bool)` is a constructor fn, not a bool field.

## Nom Compliance (parser files: subject.rs, assembly.rs, conditions.rs)

- The classifier is exactly as specified in C1 step 3. It uses `tag`, `alt` and `value` over the verb axis, plus `opt` for the "then "/"perpetually " prefixes, and `find_predicate_start` (existing) for the structural offset.
- It uses no `contains`, `starts_with` or `find` for dispatch.
- Lowercase input comes from `fragment().to_lowercase()` with `tag().parse()` per the CLAUDE.md lowercase guidance, or `nom_parse_lower` if the fragment is mixed-case.
- `assembly.rs` only calls the classifier.
- `conditions.rs` uses a slice-pattern change only, with no new text matching.
- No Oracle text is newly accepted with semantics deferred. C4 changes the lowering of already-accepted text, and C1 adds metadata to already-parsed clauses.

## Extension vs Creation

- **Extensions**: B2, C3, C4 and the D1 predicate.
- **New items**: the B1 pin function and the C1 enum and field.
  - The pin function is justified because no existing primitive evaluates against a foreign population.
  - The enum and field are justified because the Stage-1 greps of `/add-engine-variant` (the inventory and `grep -rn "Onset\|SimultaneousWithEntry\|AfterEntry" crates/engine/src`) found no onset or timing concept on ability nodes, and adding a field to `Effect::GenericEffect` would touch about 155 literal constructions across 4 effect kinds, against a per-node field that mirrors `sub_link`.

## Variant Discoverability

- The executor runs `cargo engine-inventory` and greps `data/engine-inventory.json` for `Onset`, `Simultaneous` and an existing constant-truth `StaticCondition` before adding anything.
- The executor runs the `/add-engine-variant` checklist for `ContinuousEffectOnset`.
- No sibling-cluster smell: the enum has two values on one axis, inside one CR section (611.2e).

## Verification Matrix

All integration rows go in existing files under `crates/engine/tests/integration/`. Main.rs mod lines already exist, so no new top-level binary is created. Cast rows follow `/card-test`: GameScenario, `GameRunner::cast(..).resolve()`, verbatim Oracle text, and `CastOutcome` deltas.

| # | Claim | Seam / production entry | Test | Revert-failing assertion | Paired positive / hostile |
|---|---|---|---|---|---|
| V1 | B1: 4 pre-entry devotion means Heliod is not a creature at entry, so it gets 0 counters | `pin_entry_population_conditions` via `BattlefieldEntrantProjection::new` ← `enter_with_counters_for_object`, Recommission cast | correct `recommission_reads_entrants_own_isnt_a_creature_static`: 2 WW anchors give **0** counters, and Heliod is on the battlefield **and is a creature after entry** (5 devotion live) | 0 counters (base gives 1) | reach guard: Heliod is on the battlefield, which is the ChangeZone reach, and is a creature post-entry, which shows the live layers still count its own {W} (ruling b) |
| V2 | B1 positive control: 5 pre-entry devotion gives 1 counter | same | new row: 2 WW + 1 W anchors | 1 counter | positive control for V1, which shows the projection still applies the gate when the condition is met pre-entry |
| V3 | B1 post-entry creature observer (ruling b) | `trigger_matchers` enters-observer, live | new row: Soul Warden on the battlefield, Heliod returned by Recommission at 4 pre-entry devotion | 0 counters **and** life +1 | the observer firing is the positive guard. Hostile: the pin must not leak into live state (life +1 discriminates a leak) |
| V4 | B1 class (own static, artifact count): Rusted Relic via What Must Be Done | same projection, `IsPresent` / QuantityCompare with ObjectCount | new row: 2 other artifacts give 0 counters (base gives 2); 3 other artifacts give 2 | 0 at 2 others | 3-others row is the positive control; Relic is on the battlefield in both |
| V5 | B1 class (group static): Starfield of Nyx + What Must Be Done on Sanctum of Tranquil Light | TCE or other-object static condition pinning | new row: 4 enchantments pre-entry (Starfield plus 3) give 0 counters (base gives 2); 5 give 2 | 0 at 4 | 5 is the positive control |
| V6 | B1 pin function unit | `layers.rs` unit tests | (a) a population leaf is pinned to its pre-entry truth; (b) a recipient-context leaf is left unchanged; (c) a non-population leaf (for example `SourceTapped`-style) is left unchanged; (d) And/Not recursion pins only the population child | as stated | (b) and (c) are the hostile negatives; (a) is positive |
| V7 | B1 zone-membership leaf: a graveyard-count condition is evaluated pre-departure | `static_condition_reads_zone_membership` arm | unit-level in V6 (e), on a scratch built by `BattlefieldEntrantProjection::new` from the graveyard, if a condition fixture exists; otherwise a unit test on the classifier path only, labelled | pinned truth equals the pre-entry evaluation | needs no printed consumer; this is a unit-only row |
| V8 | B2: kind-level answers read the view | `matches_filter_prop` `HasKeywordKind` / `WithoutKeywordKind` on the observation path | extend `entry_observation_shapes_read_the_projection`: next to its `WithKeyword(Flying)` assertion, assert `HasKeywordKind(Flying)` is true and `WithoutKeywordKind(Flying)` is false on the observation, while the live object has lost Flying | the kind assertions (at base they read live, giving false and true) | the `WithKeyword` assertion already present is the paired positive |
| V9 | B2: departed entrant keeps its kind-level answer | skeleton branch | extend `entry_observation_answers_for_departed_entrant` with `HasKeywordKind` | true (at base, false because the live object is absent or off-battlefield) | negative sibling: a kind it never had gives false |
| V10 | B2: off-battlefield live path unchanged | `object_has_effective_keyword_kind` → off-zone | existing off-zone keyword tests stay green; add one unit test of `effective_keyword_kind_of` on a graveyard object, if none exists | n/a (preservation, derived: CR 702 off-zone characteristics) | — |
| V11 | C3: the no-observation fallback rejects a newer incarnation | `matches_target_filter_on_entry_observation`, no-observation branch | new lib unit test in `filter.rs` (`fixture(false)`): a record with `entered_incarnation = Some(n)`, a live object bumped to incarnation n+1 that gained counters, and a filter that the record snapshot fails while the live object passes | false (at base the live arm gives true) | positive sibling: a matching incarnation gives the live answer (true), and the existing `entry_observation_absent_matches_todays_arms` stays green |
| V12 | C1: Kardur's III with the haste rider gets entry settlement before the counter | D1 `node_may_apply_with_entry` ← `settle_entries_before_instruction`; synthetic sorcery with Kardur's chapter III text verbatim | new row in `etb_trigger_entry_time_characteristics.rs`: Elemental Bond; return a 2/2 | **0 draws** (base gives 1) | reach guards: the 2/2 ends 3/3 with a +1/+1 counter **and** haste (the rider resolved). Positive sibling: returning a 3/3 base creature gives 1 draw, showing the Bond observer fires on the same route |
| V13 | C1: Hunting Wilds kicked: Forests are not creatures at entry | same predicate; Hunting Wilds cast with `.accept_optional()` | new row: 2 Forests in library, Elemental Bond | **0 draws** (base gives 2) | reach guards: both Forests are on the battlefield, untapped, 3/3, green, with haste and still Lands. Negative sibling: unkicked gives 0 draws and the Forests stay tapped |
| V14 | C1: is-form riders still defer (onset Simultaneous) | same | existing r7 (Rise from the Grave) and r12 (Path of the Schemer) stay green, plus parse assertions that their D1 nodes carry `Some(SimultaneousWithEntry)` | the onset value | regression |
| V15 | C1 classifier unit | `classify_continuous_effect_onset` | each verb axis (is/are/isn't/aren't/has/have/doesn't have/don't have gives Simultaneous; becomes/become/gains/gain gives AfterEntry; "it's"/"they're" give Simultaneous; "then " and "perpetually " prefixes are handled); relative-clause hostile ("each creature that is tapped becomes …" gives AfterEntry); unknown verb gives None | as stated | the None row is the conservative negative |
| V16 | C1 stamp reaches card data | assembly | parse tests: the Kardur's III and Hunting Wilds chains carry AfterEntry on their D1 node, and the Call a Surprise Witness rider carries Simultaneous | onset values | — |
| V17 | C1 serde round-trip | `AbilityDefinition` hand-written serde | round-trip unit test with `Some(AfterEntry)`, plus absence (skip) when None | equality | — |
| V18 | C4: multi-type rider folds into an AnyOf gate | `card_type_condition_as_entry_this_way` | `multi_type_entry_rider_folds_into_any_of_gate` (renamed and rewritten) plus the conditions unit case | the gate is `AnyOf[Artifact, Creature]` and there is no PutCounter sub | hostile: an additional or subtype filter still gives None (existing unit case); the single-type row still folds to a plain type (existing) |
| V19 | C4 runtime (optional, if a modal-bullet synthetic is cheap) | `entry_gate_matches` with AnyOf | the What Must Be Done-shaped synthetic modal returning an artifact non-creature gives 2 counters; a non-artifact non-creature permanent gives 0 | counters | the executor may make this a parse-only row if no historic non-artifact non-creature fixture is cheap. Labelled optional |
| V20 | L1/C5 | `drive_to_empty` | the assertion itself; r5b, r5d, r11, r12 stay green | `settled` | — |

Coverage-status impact: none expected (no Oracle text changes its supported or unsupported status). The executor confirms this with `cargo coverage` before and after if the card-data regeneration shows any diff in `Unimplemented` counts.

## Reference Readings

- **V10** (preservation of the off-zone keyword path). Derived from the expectation that an off-battlefield object's effective keywords come from its printed and granted characteristics (unchanged code path). The reference equals the derivation, measured by the existing off-zone keyword tests.
- **V14** (r7/r12 preservation). Derived: Rise from the Grave ("is a black Zombie") and Path of the Schemer ("It's a Phyrexian") are CR 611.2e is-form, so they apply with the entry. The current rows assert the entry-time reading, and the reference agrees with the derivation **[read, the rows' assertions]**.
- **V12/V13 positive siblings** (3/3 return, unkicked). Derived from the Oracle text directly, not from parity.
- No row asserts parity with a base reading that was not derived. The two base readings that were found to disagree with their derivation (Heliod 4-devotion: base gives 1, derived gives 0; Kardur's and Hunting Wilds draws) are the defects this plan fixes. Their rows assert the derived values.

## Identity / Provenance Contract

- **"If a creature enters this way" / "it's a creature"** (Recommission, What Must Be Done): the object identity is the ChangeZone's moved entrant. Its characteristics come from the CR 614.12 projection ("as it would exist on the battlefield"), with own statics applied and **population evaluated pre-entry** (the new contract). The controller is the post-override controller.
- **Entry observation** (triggers): the identity is `record.object_id` plus `record.entered_incarnation` (CR 400.7). The no-observation fallback now honors the incarnation too (C3).
- **"They" / "Forests put onto the battlefield this way"** (Hunting Wilds): the existing tracked set, unchanged. Only the observation timing changes.
- **Onset**: a provenance fact of the clause text, stamped once at parse and carried unchanged through `ResolvedAbility` conversion.

## Step-by-step plan

1. **U6**: harden `drive_to_empty` (the settled flag plus the assertion). Run r5b, r5d, r11 and r12.
2. **U2**: add `keywords::effective_keyword_kind_of`, delegate `object_has_effective_keyword_kind` to it, and switch the two `matches_filter_prop` arms. Check the zone stamp of the departed skeleton. Add V8, V9 and V10.
3. **U3**: add the incarnation check in the no-observation live arm, and V11.
4. **U1**: in `layers.rs`, add `pinned_condition` and `pin_entry_population_conditions` (with the inventory check for a constant-truth variant), and call it from `BattlefieldEntrantProjection::new` before `mark_layers_full`. Fix the case (b) comment. Correct the V1 row, add V2–V5, and add the V6/V7 unit tests.
5. **U5**: AnyOf in `card_type_condition_as_entry_this_way`; update the doc and the unit case; rewrite the C4 parser test; optionally add V19.
6. **U4**:
   - Run `cargo engine-inventory` and the `/add-engine-variant` checklist.
   - Add the enum and field with all serde mirrors, `new()` and PartialEq.
   - Update the audit_projection key list and fill the compiler-forced sites.
   - Add the copies in `ability_utils.rs` and `separate_piles.rs`.
   - Add the classifier in `subject.rs` plus V15.
   - Add the stamp in `assembly.rs` (and any other D1-capable def site), and change the D1 predicate in `effects/mod.rs`.
   - Regenerate card data; add V12–V14, V16 and V17; measure U-d.
7. Add the C6 doc comment on `entry_gate_projection`.
8. Run `cargo fmt --all`, the parser combinator gate, Tilt `clippy`, `test-engine` and `card-data`, or the focused integration filters if Tilt is down, under the caller's build constraints.

## Residuals (documented, not fixed here)

- **R1 (C1)**: Call a Surprise Witness, an is-form rider after a counter: "Put a flying counter on it. It's a Spirit…". Deferral (Simultaneous) keeps the observation after the counter. The fully correct reading would apply the Spirit type with the entry but observe before the flying counter, which needs the rider applied inside the entry projection. That is a separate mechanism. A Bond-style observer does not care about either change, so there is no wrong printed reading for P/T-gated observers. Flag it for a type-gated observer.
- **R2 (C1)**: D1 nodes whose onset classifies `None` keep the conservative deferral (count U-d reported by the executor).
- **R3 (B1)**: population leaves with recipient context, and dynamic P/T quantities, are not pinned. There are no gate consumers today.
- **R4 (C6)**: 2N projections per N doubly-gated entrants, with zero printed cards.

## Pre-existing (separate list, not in this scope)

- **PE1**: A non-modal anaphoric "It enters with … counters on it if it's a <type>." line, after a Return in the same Oracle line, parses to `Unimplemented "enters_with_gate"` at the line-routing level, in both the single-type and multi-type versions (P4). Printed What Must Be Done is modal, so it is unaffected.
- **PE2**: Base's raw graveyard-type check for the Heliod gate had the same wrong result before the PR (noted by the reviewer). It is subsumed by U1.
