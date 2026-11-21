# Plan r1 — resolution-created restrictions survive a later "lose all abilities" effect

BASE_SHA 7e2a538799e672af23caca367ba938bc24194d66. Worktree X:/Users/jakob/Documents/dev/phase-wt/resolved-restrictions.
Budget: standard. Mode: ordinary. Planner work was read-only and ran no cargo builds; every probe is in §3.5 for the executor.

## 0. Premise check (Step 0) — verbatim Oracle text (MTGJSON AtomicCards 5.3.0+20261016)

- **Dress Down** ({1}{U} Enchantment): "Flash / When this enchantment enters, draw a card. / Creatures lose all abilities. / At the beginning of the end step, sacrifice this enchantment." Ruling 2021-06-18: "If an effect grants a creature an ability after Dress Down has entered the battlefield, it won't lose that ability. … It will, however, lose any abilities it already had."
- **Turn to Frog** ({1}{U} Instant): "Until end of turn, target creature loses all abilities and becomes a blue Frog with base power and toughness 1/1." Ruling: "If the affected creature gains an ability after Turn to Frog resolves, it will keep that ability."
- **Humility** ({2}{W}{W} Enchantment): "All creatures lose all abilities and have base power and toughness 1/1."
- **Panic** ({R} Instant): "Cast this spell only during combat before blockers are declared. / Target creature can't block this turn. / Draw a card at the beginning of the next turn's upkeep."
- **Distortion Strike** ({U} Sorcery): "Target creature gets +1/+0 until end of turn and can't be blocked this turn. / Rebound (…)". Ruling: "Distortion Strike doesn't grant an ability to the targeted creature. Rather, it affects the game rules and states something that's now true about that creature. The creature can't be blocked even if it loses all abilities."
- **Goblin Tunneler** ({1}{R} 1/1): "{T}: Target creature with power 2 or less can't be blocked this turn." Ruling (same wording): "…the creature can't be blocked that turn even if it loses all abilities."
- **Promise of Loyalty** ({4}{W} Sorcery): "Each player puts a vow counter on a creature they control and sacrifices the rest. Each of those creatures can't attack you or planeswalkers you control for as long as it has a vow counter on it."
- **Craterous Stomp** ({1}{R} Kindred Instant — Giant): "Craterous Stomp deals 3 damage to target creature an opponent controls. Each other creature that player controls becomes a Coward in addition to its other types and gains "This creature can't block Giants or Warriors.""
- **Incite Hysteria** ({2}{R} Sorcery): "Radiance — Until end of turn, target creature and each other creature that shares a color with it gain "This creature can't block."" Ruling 2005-11-01: "…gain an ability that says they can't block this turn."
- **Jump** ({U} Instant): "Target creature gains flying until end of turn."
- **Hulking Goblin** ({1}{R} 2/2): "This creature can't block."  **Hill Giant** ({3}{R} 3/3 Giant) and **Grizzly Bears** ({1}{G} 2/2 Bear): vanilla.
- **Muraganda Petroglyphs** ({3}{G} Enchantment): "Creatures with no abilities get +2/+2."
- **Immortal Obligation** (stacked PR, not parsed here): text as in task.md, verified.
- **Psychic Paper**: "Equipped creature has ward {1}, it can't be blocked, and its name and creature type are the last chosen name and creature type."

**Premise correction (load-bearing).** The task names Craterous Stomp as an *affected* card. Its text grants a **quoted ability** ("gains "This creature can't block Giants or Warriors.""), which per CR 113.10 is an ability of the creature, so a later Dress Down **correctly removes it** — the same reading the Incite Hysteria ruling states for its identical shape. Craterous Stomp is therefore this plan's **hostile** card (must stay removed), not a positive. Promise of Loyalty ("can't attack you…", no "gains") is a positive.

## 1. The design question, settled with evidence

### 1.1 Where the bit is lost (measured from card-data.json, a recent export; executor re-measures on BASE, §3.5 P1)

A restriction from a resolving effect and a quoted ability grant reach the layer system with **byte-identical** transient-effect payloads:

| Card | rules meaning | `GenericEffect` → TCE `modifications` |
|---|---|---|
| Panic "Target creature can't block this turn." | effect (CR 611.2) | `[AddStaticMode{CantBlock}]` |
| Incite Hysteria "…gain "This creature can't block."" | ability (CR 113.10) | `[AddStaticMode{CantBlock}]` |
| Promise of Loyalty "…can't attack you or planeswalkers you control…" | effect | `[GrantStaticAbility{CantAttack, SelfRef, attack_defended}]` |
| Whipgrass Entangler "…gains "This creature can't attack or block unless…"" | ability | `[GrantStaticAbility{CantAttackOrBlock, SelfRef, UnlessPay}]` |
| Craterous Stomp (quoted) | ability | `[AddSubtype{Coward}, AddStaticMode{BlockRestriction}]` |

Causes: (a) `parse_quoted_rule_static_modifications` (crates/phase-parser/src/parser/oracle_static/grammar.rs) collapses a quoted trivial static (`affected == SelfRef`, no condition, no modifications) into `AddStaticMode`, the same variant ~766 bare-restriction producers emit; (b) non-quoted rule effects that need a full definition (Promise class, combat-tax "this turn" effects, temporary cost modifications) ride `GrantStaticAbility` (subject.rs comment: "the only shape with a per-recipient slot for CR 109.5's 'you'"), the variant quoted non-trivial grants use. The TCE keeps no wrapper data, and the GenericEffect wrapper `mode` is not a discriminator either (40 wrappers are `Continuous` with a **bare** `AddStaticMode`, e.g. Break Through the Line, Glaring Spotlight, Vines of Vastwood).

**Conclusion: no engine-only rule (layer arm, consumer, TCE read) can keep both acceptance-criterion halves.** Keying on the variant fails one side in each variant. An engine-only "every TCE graft survives" regresses the quoted class (Incite Hysteria, Craterous Stomp, Full Steam Ahead, Ogre Marauder, Wilson, Stilt-Man, …, ~18 TCE cards). Two small AST changes are **strictly required**. The expected parse diff is therefore **not none** (§7).

### 1.2 Existing provenance reused

- **Replacements**: `ReplacementDefinition.origin: ReplacementOrigin { Characteristic (default), Resolution }`, stamped by `GameObject::install_resolution_replacement`. `RemoveAllAbilities` and `set_land_subtype_replacing` keep `is_resolution_installed()` defs (CR 613.1f + CR 611.2c), and the copy snapshot and text substitution skip them. This plan **generalizes that enum** (rename `ReplacementOrigin` → `DefinitionOrigin`, same variants, wire-identical) and gives `StaticDefinition` the same serde-defaulted `origin` field, predicate and retain. One provenance concept serves both definition kinds. No parallel enum.
- **Transient provenance**: `ActiveContinuousEffect.transient_id.is_some()` is the existing marker that a layer effect came from a resolution-created TCE (CR 611.2). `unstarted_effect_generator_is_suppressed` already treats transient-derived effects as not suppressible by ability removal.
- **TCE-read precedents** (goad `goading_players_for_creature_gated`, CantBeTargeted/CantBeCountered `transient_cant_be_targeted_prohibits`, CantUntap/CantPhaseIn raw TCE scans, `transient_relative_blocker_filter`). These are kept unchanged. They are **not** extended: a TCE-read for every restriction mode would touch every consumer (CantBlock, CantAttack, BlockRestriction, CantBeBlocked*, MustAttack/MustBlock, CantTap, …), which is not one authority, and it would still need the quoted bit.

### 1.3 Chosen design

1. **CR rule implemented.** A continuous effect generated by the resolution of a spell or ability that does not modify characteristics "modifies the rules of the game" (CR 611.2c) and is applied after all layers (CR 613.11). It is not an ability of the object, so a layer-6 ability-removing effect (CR 613.1f) never removes it, whatever the timestamps (CR 613.7 orders only effects inside layer 6). An effect that says the object "gains"/"has" an ability adds an ability (CR 113.10) and is removed in timestamp order.
2. **Single authority: the layer-6 graft arms + the `RemoveAllAbilities` arm in `crates/engine/src/game/layers.rs`.** Every restriction consumer reads grafted `static_definitions` (combat `collect_blocker_restriction_statics` etc. via `functioning_abilities`), so fixing what survives in layer 6 fixes every consumer at once.
   - `AddStaticMode` arm: the def it manufactures gets `origin = DefinitionOrigin::Resolution` iff `effect.transient_id.is_some()`.
   - `GrantStaticAbility` arm: unchanged. It pushes the inner definition verbatim, so the inner def's declared `origin` rides through.
   - `RemoveAllAbilities` arm: `obj.static_definitions.retain(StaticDefinition::is_resolution_installed)` replaces `.clear()`, mirroring the replacement retain beside it.
3. **Parser declares the two cases the engine cannot know.**
   - U3 — quoted static grants are always `GrantStaticAbility` (the quoted authority stops collapsing to `AddStaticMode`), with `origin` left `Characteristic`: a granted ability is a characteristic of its recipient (CR 109.3, CR 113.10). After U3, a TCE `AddStaticMode` is always a rule effect.
   - U2 — every non-quoted rule-effect vehicle that rides `GrantStaticAbility` declares `origin = Resolution` on its inner def through one builder, `StaticDefinition::resolution_installed()`.
4. **Sibling consumer of the same fact:** `filter::object_has_no_abilities` ignores `is_resolution_installed()` statics. A Panicked vanilla creature still has no abilities (Muraganda Petroglyphs).

## 2. Pattern Coverage (Step 0.5 record)

- MTGJSON version `5.3.0+20261016`; `jq-1.8.2`.
- Class: resolving spells/abilities whose effect puts an attack/block/evasion restriction on permanents with a stated window. Regex `(can't|cannot|doesn't|don't) (attack|block|be blocked)[^."]*(this turn|until end of turn|until your next turn|this combat|for as long as)`, keyword empty, `add: []`. `drop` (18 printed static abilities that the duration words also match, not resolution effects — verified line by line): Bloodcrazed Goblin, Bloodthirster, Bontu the Glorified, Fugitive Droid, Furtive Courier, Goblin Cohort, Haunting Figment, Hollow Warrior, Illvoi Infiltrator, June, Bounty Hunter, Mogg Conscripts, Nimble Brigand, Nimble Trapfinder, Port Razer, Relic Runner, Silburlind Snapper, Sword Coast Serpent // Capsizing Wave, Tome Anima. `masked_only: []`, `rejected_add: []`, `rejected_drop: []`.
- Final command: the engine-planner Step 0.5 jq template with the regex above, `--argjson add '[]' --argjson drop "$(cat class-drop.json)"`, over `data/mtgjson/AtomicCards.json`. Output: `X:/Users/jakob/Documents/dev/phase/.git/engine-implementer-runs/removal-restrictions/class.json` (drop list in `class-drop.json` beside it). `jq -cS . class.json | sha256sum` = `bc328027296c71f850492d5ae6c59e5db8493bedb4772ee90cb0d39a2dc3f208`.
- **Count: 427 cards; 907 distinct rulings, 1045 ruling instances, all read.** The regex misses window-less resolution effects ("for as long as" is in it; Craterous Stomp's quoted grant has no window). The parse census below covers the parser side.
- Parse-side census (card-data.json): 814 cards carry a resolving-effect static graft. 766 wrappers carry bare `AddStaticMode`, 61 carry `GrantStaticAbility` and 40 carry `Continuous` + `AddStaticMode`. Of the GrantStaticAbility cards, 37 are non-quoted vehicles (list in §7). About 18 are quoted `AddStaticMode` collapses in resolving effects, and about 10 more are in printed static lines (§7).
- Bearing rulings (> 30 distinct, so only bearing ones are quoted):
  - "Distortion Strike doesn't grant an ability to the targeted creature. Rather, it affects the game rules and states something that's now true about that creature. The creature can't be blocked even if it loses all abilities." (Distortion Strike) → row V1.
  - "The ability doesn't grant an ability to the targeted creature. … After the ability resolves, the creature can't be blocked that turn even if it loses all abilities." (Goblin Tunneler) → V1 (same rule, activated-ability producer; optional V1b).
  - "Venser's second ability doesn't lock in what it applies to … all creatures can't be blocked that turn, including … creatures that have lost all abilities." (Venser, the Sojourner) → V1/V2.
  - "Monstrous isn't an ability that a creature has … If the creature … loses its abilities, it will continue to be monstrous." (Stoneshock Giant): the designation analog, consistent with goad (no row).
  - Dynamic-population rulings (CR 611.2c second sentence), e.g. "Because Magmatic Chasm's effect doesn't change the characteristics of any permanents, the set of creatures affected … is constantly updated. Creatures without flying that enter the battlefield later in the turn won't be able to block." (Magmatic Chasm; same for Cosmotronic Wave, Veiling Oddity, Pyrophobia, Tectonic Rift, Destructive Tampering, Tezzeret's Gatebreaker, Seismic Stomp/Elemental, Gruul Charm, Awe for the Guilds, Sundering Eruption, The Motherlode, Keeper of Keys, Concussive Bolt, Orzhov Advokist, Ember Gale, Chaos, Hazardous Blast, Rumbling Ruin, Temur Charm, …). This is an **adjacent, pre-existing** sibling (a broadcast rule effect frozen to a resolution-time `SpecificObject` set). It does not bind this design. Out of scope, follow-up (§9 Q6).
  - Promise of Loyalty: "If the vow counter is removed from the creature, it can attack as normal." The duration stays governed by the TCE (unchanged) → V7 keeps the existing vow-removal rows green.
  - Incite Hysteria ruling (quoted = ability) → hostile rows H2/H3.
  - Dress Down / Turn to Frog rulings (a grant **after** the removal keeps the ability) → timestamp guard H4.

## 3. Applicable skills, analogous trace, files read

Skills: `add-static-ability` (layer-6 grafting, static-mode consumers), `card-test` (GameScenario/GameRunner casts, CastOutcome, declare_attackers/blockers boundaries), `oracle-parser` (U2/U3 touch parser output, no new recognizers), `add-engine-variant`: **not triggered**. No enum variant is added: the enum is renamed with the same variants, a struct field is added, and the perpetual `TryFrom` gains an arm (§5).

**Analogous trace (hard gate):**
1. Resolution replacement provenance: `crates/phase-ast/src/types/ability.rs` `ReplacementOrigin` / `ReplacementDefinition::origin` / `is_resolution_installed` → `crates/engine/src/game/game_object.rs` `install_resolution_replacement` (stamp + base invariant debug_assert) and `reseed_replacements_carrying_resolution_effects` → `crates/engine/src/game/layers.rs` `settle_resolution_replacements_to_tail`, the `RemoveAllAbilities` arm retain, the `set_land_subtype_replacing` retain → `crates/engine/src/game/printed_cards.rs` (snapshot filter) and `text_substitution.rs` (skip) → test `layers.rs::humility_does_not_end_a_resolution_created_shield`.
2. Resolution restriction graft: `crates/phase-parser/src/parser/oracle_effect/subject.rs` (bare `AddStaticMode` producers; the Promise-class `GrantStaticAbility` at the `can't attack` + `parse_cant_attack_defended_scope_nom` branch; `try_parse_combat_tax_effect_clause`) → `crates/engine/src/game/effects/effect.rs` `register_transient_effect` / `bind_transient_effect` / `install_transient` → `crates/engine/src/types/game_state.rs` `add_transient_continuous_effect_inner` → `layers.rs` TCE gather (skip list for Goaded/CantBeTargeted/…; `ActiveContinuousEffect{transient_id: Some(..), mode: Continuous}`) → `apply_continuous_effect_filtered` `AddStaticMode` / `GrantStaticAbility` arms → `crates/engine/src/game/combat.rs` `collect_blocker_restriction_statics` / `collect_blocker_allowed_statics` / cant-be-blocked collectors via `functioning_abilities::{game,battlefield}_functioning_statics`.
3. Quoted grant: `oracle_effect/imperative.rs::try_parse_gain_quoted_ability` and the subject path → `oracle_static/keyword_grant.rs::{parse_quoted_ability_modifications, classify_quoted_inner}` → `oracle_static/grammar.rs::parse_quoted_rule_static_modifications` (the collapse).
4. Goad TCE-read: layers skip list `StaticMode::Goaded` → `combat.rs::goading_players_for_creature_gated` (unchanged).

Files read for this plan: the sections named above in layers.rs (RemoveAllAbilities, AddStaticMode, GrantStaticAbility arms, TCE gather, suppression predicate, suspected derivation, set_land_subtype_replacing), effect.rs (bind/install), game_state.rs (TCE struct), phase-ast ability.rs (StaticDefinition, ReplacementOrigin, ContinuousModification docs, Perpetual TryFrom), phase-ast layers.rs (ActiveContinuousEffect), grammar.rs (collapse), keyword_grant.rs (classify ladder), subject.rs (Promise and combat-tax producers), mod.rs (temp cost-mod producer), static_abilities.rs (TCE readers), turns.rs (untap scans), filter.rs (`object_has_no_abilities`), combat.rs (collectors), burden_of_proof.rs and promise_of_loyalty.rs (harness).

### 3.5 Probes (planner could not build). Executor runs these first, then records results in the phase-fit record

Each probe names the decision it can change.

- **P1 (decides whether U3 is needed).** On BASE, run Panic and Incite Hysteria through `GameRunner::cast(..).resolve()` and print `state.transient_continuous_effects[i].modifications`. Predicted: both `[AddStaticMode{mode: CantBlock}]`. Positive marker: the TCE count rises by 1 on each cast. If they differ (Incite Hysteria already carries a distinct shape), U3 shrinks to the remaining collapsed producers.
- **P2 (defect on BASE).** Distortion Strike on P0's Bears, then Dress Down, then P0 attacks: `declare_blockers([(p1_bears, p0_bears)])` returns Ok on BASE, and the attacker's `static_definitions` holds no `CantBeBlocked`. Reach guard: Dress Down removed a printed ability (a P0 creature with a printed keyword lost it).
- **P3.** Muraganda + Panic on a vanilla P1 Bears at BASE: the Bears' P/T reads 2/2 after Panic resolves (4/4 before). If it reads 4/4, the layer gather evaluates `HasNoAbilities` before layer-6 grafts, and V6 is dropped with that measurement as its reason.
- **P4 (U3 blast radius).** After U3, regenerate card-data and diff it against BASE. Every changed card is a quoted static grant whose `AddStaticMode{mode}` became `GrantStaticAbility{definition: {mode, affected: SelfRef, …}}`. Compare against the §7 prediction. Run `cargo coverage` before and after: no card changes supported status.
- **P5 (U3 runtime parity).** For each P4 card that resolves to an object graft, the recipient's grafted def after U3 must equal BASE's with `description` and `origin` ignored. The anchors that only the AddStaticMode arm stamps (`source_controller` for `static_mode_needs_source_controller_anchor`, chosen color, `source_object` for `MustAttackDefender`, `bound_actor`) must not apply to any P4 card. If one does, return it (§9 Q7).

## 4. Sizing

| Unit | Mechanic | Registration surfaces | Discriminating test |
|---|---|---|---|
| U3 | Quoted static grants are `GrantStaticAbility` (no AddStaticMode collapse) | `oracle_static/grammar.rs::parse_quoted_rule_static_modifications`; `phase-ast` `TryFrom<ContinuousModification> for PerpetualGrantModification` (accept a trivial `GrantStaticAbility` as `AddStaticMode`); callers found by the P4/P5 census | parser unit `quoted_trivial_static_is_a_granted_static_ability` (fails on BASE: AddStaticMode); integration H2 (Craterous Stomp), which fails on **U1-without-U3** |
| U1 | Resolution provenance on grafted statics; ability removal keeps it; no-abilities predicate ignores it | `phase-ast` ability.rs (`DefinitionOrigin` rename, `StaticDefinition.origin`, `is_resolution_installed`, `resolution_installed()` builder); layers.rs (AddStaticMode arm stamp, RemoveAllAbilities retain); filter.rs; game_object.rs + essence_burn.rs (rename); compiler-forced `StaticDefinition {…}` literal sites | V1 Distortion Strike, V2 Panic, V3 Turn to Frog, V4 Hulking Goblin, V6 Muraganda, layers unit U1-a/b (all fail on BASE) |
| U2 | Non-quoted rule-effect vehicles declare `origin: Resolution` | subject.rs (Promise class; combat tax); oracle_effect/mod.rs (temporary cost modification ×3 sites, graveyard-permission and flash grants); any other site the §7 vehicle census finds | V7 Promise of Loyalty + Humility (fails on BASE) |

Dependency edges: **U3 → U1** (U1 alone regresses the quoted class, and H2/H3 are U1's regression guard). **U1 → U2** (the field and the retain).

Expected scope paths (counting rule applied; test fixtures and regenerated card-data excluded):
1 crates/phase-ast/src/types/ability.rs · 2 crates/engine/src/game/layers.rs · 3 crates/engine/src/game/filter.rs · 4 crates/engine/src/game/game_object.rs · 5 crates/phase-parser/src/parser/oracle_static/grammar.rs · 6 crates/phase-parser/src/parser/oracle_effect/subject.rs · 7 crates/phase-parser/src/parser/oracle_effect/mod.rs · 8 crates/engine/tests/integration/resolved_restriction_ability_removal.rs (new) · 9 crates/engine/tests/integration/shard_5.rs · 10 crates/engine/tests/integration/essence_burn.rs.
Plus compiler-forced `StaticDefinition` literal sites (each literal lists every field). `grep -rn "bound_actor: None"` hits 57 lines in 21 files, but some are `TransientContinuousEffect` literals; the compiler's error list is authoritative. Likely files: engine game/coverage.rs, layers.rs, triggers.rs, casting_tests.rs, visibility.rs, stickers.rs, quantity.rs, meld_tests.rs, effects/create_emblem.rs, dungeon.rs, derived_views.rs, combat_damage.rs, ai_support/mod.rs; phase-ai policies/reactive_self_protection.rs, features/aggro_pressure.rs, policies/planeswalker_loyalty.rs; phase-parser oracle_modal.rs; tests najeela_extra_combat_grant_2898.rs, kaito_integration.rs. Plus U3 caller sites found by P4/P5. **Expected count ≈ 25–30 (≥13).** 3 units → T1 and T2 both hold; the phase-fit gate is expected to fire.
Charter-friendly seam: phase 1 = U3 (parser-only representation, runtime parity, tree green); phase 2 = U1; phase 3 = U2. U3+U1 together is also a green seam.

## 5. Architecture sections

**Building Blocks.** `ReplacementOrigin` (generalized), `ReplacementDefinition::is_resolution_installed` (pattern for `StaticDefinition::is_resolution_installed`), `Definitions::retain`, `ActiveContinuousEffect::transient_id`, `parse_static_line_multi` (unchanged inner-quote authority), `StaticDefinition::new/affected/...` builders, the `GameScenario`/`GameRunner` harness and the burden_of_proof.rs `to_declare_blockers_after_flash`/`try_block` pattern (copied locally; no cross-module `crate::` use, so shard_5). No new helper except the `resolution_installed()` builder. Its closest authority is `ReplacementDefinition`'s origin stamping. That is engine-side at install, and a parser-authored vehicle def has no engine install seam that knows it is a vehicle.

**Logic Placement.**
- `apply_continuous_effect_filtered` `ContinuousModification::AddStaticMode` arm (layers.rs): owns stamping, because it manufactures the def. It reads `effect.transient_id`.
- `ContinuousModification::RemoveAllAbilities` arm: owns removal. It calls `StaticDefinition::is_resolution_installed`.
- `GrantStaticAbility` arm: unchanged (verbatim push carries the declared origin).
- `filter::object_has_no_abilities`: owns the HasNoAbilities reading.
- Parser `parse_quoted_rule_static_modifications`: owns quoted lowering.
- U2 producers in subject.rs and oracle_effect/mod.rs call `.resolution_installed()`.
- Consumers whose behavior changes with unchanged signatures: all combat restriction collectors (they now see retained grafts), `static_mode_presence` (a retained graft keeps its kind present), AddStaticMode dedup (`sd == &def` now includes `origin`, so a printed or static-ability graft and an identical TCE graft no longer collapse; precedent: the replacement-side hostile fixture `carried_resolution_def_does_not_suppress_an_identical_derived_grant`). Executor checks accumulating-mode consumers (`ExtraBlockers`, `CrewContribution`) by running `test-engine` and reports any count change.

**Rust Idioms.** Typed enum provenance (no bool). `#[serde(default, skip_serializing_if = "DefinitionOrigin::is_characteristic")]` keeps every existing serialized static byte-identical. The perpetual `TryFrom` arm is an exhaustive pattern: `GrantStaticAbility { definition }` whose definition is trivial maps to `Self::AddStaticMode { mode }`; any other `GrantStaticAbility` stays rejected (fail-closed preserved). Renaming an enum without changing variants is wire-compatible.

**Nom Compliance.** Parser files change (grammar.rs, subject.rs, mod.rs), but **no detection, dispatch or classification step is added or altered**. U3 changes only the output constructor after `parse_static_line_multi` has already classified the text; U2 appends a builder call to already-built defs. No `contains`/`starts_with` is introduced. No Oracle text becomes newly accepted, so coverage stays honest (P4 checks it).

**Extension vs Creation.** Extends the existing replacement provenance to statics and the existing transient marker. Creates nothing parallel.

**Variant Discoverability.** No new variant. `cargo engine-inventory`: not required. The executor greps `data/engine-inventory.json` (after regeneration) for `ReplacementOrigin` references to rename.

**Identity / Provenance Contract.**

| Concept | Authority / value | Bound when | Live vs latched | Stored | Consumer | Invalidation | Hostile fixture |
|---|---|---|---|---|---|---|---|
| "effect of a resolved spell, not an ability" (CR 611.2c) | `DefinitionOrigin::Resolution` on the grafted `StaticDefinition` | each layer pass, at the layer-6 graft (AddStaticMode: from `transient_id.is_some()`); parse time for vehicles | re-derived every pass from the live TCE; the TCE's duration/condition gates it (unchanged) | `obj.static_definitions` (never `base_*`; mirror the replacement base invariant with a `debug_assert` in `reset_recipient_to_base`'s graft path if a natural seat exists) | `RemoveAllAbilities` retain; `object_has_no_abilities` | TCE expiry/prune removes the graft next pass; zone change resets the object | V4: printed `Characteristic` CantBlock and a TCE `Resolution` CantBlock on two creatures under one Dress Down |
| "granted ability" (CR 113.10) | `GrantStaticAbility` from the quoted authority, `origin: Characteristic` | parse | removed in timestamp order | same | `RemoveAllAbilities` (removed) | — | H2/H3 quoted removed; H4 later grant kept |

## 6. Verification Matrix

Harness: `GameScenario::new_n_player(2, 42)` at `PreCombatMain`, stocked libraries, `add_*_from_oracle` with the verbatim text above, `runner.cast(id)…resolve()`. Combat and blocker checks go through a clone (burden_of_proof pattern): `declare_attackers` → `pass_both_players` → `WaitingFor::DeclareBlockers` → `try_block` on a clone. Each refusal asserts the restriction-specific error string (read the real message from combat.rs). Each row has a reach guard showing that Dress Down / Turn to Frog / Humility is in force, e.g. a printed keyword or trigger of another creature is gone and P/T is 1/1 for Humility/Turn to Frog.

| Row | Claim / seam | Production entry | Assertion (fails on BASE?) | Paired positive / hostile |
|---|---|---|---|---|
| V1 | Distortion Strike's resolved CantBeBlocked survives Dress Down (AddStaticMode arm stamp + retain) | cast Distortion Strike on P0 Bears (main 1), cast Dress Down, attack | P1 `try_block(p1_bears, p0_bears)` is Err with the can't-be-blocked error, and the attacker still carries a `CantBeBlocked` static with `origin == Resolution`. **Fails on BASE** (P2) | P1 blocks a second, unaffected P0 attacker Ok; reach guard: Dress Down removed a printed keyword |
| V1b (optional) | activated-ability producer | Goblin Tunneler `{T}` on a power-1 attacker, then Dress Down | as V1 | same |
| V2 | Panic (instant, in combat) survives a later flash Dress Down | P0 attacks; in the declare-attackers window cast Panic on P1 Bears A, then Dress Down | `try_block(A, attacker)` Err; A absent from `valid_blocker_ids`. **Fails on BASE** | P1 Bears B blocks Ok |
| V3 | second removal witness, targeted and until end of turn | Panic on A, then Turn to Frog on A | A is a 1/1 Frog with no abilities, and its block is still Err. **Fails on BASE** | B blocks Ok |
| V4 | multi-authority: printed vs resolution provenance under one removal | P1 controls Hulking Goblin G1 and G2; Panic on G1; Dress Down | G2 (printed only) **may** block (Ok: its printed ability is removed); G1 Err (Panic's effect survives). G1 fails on BASE | G2 Ok is the paired positive and also shows the printed static is still removed |
| V5 | building block (layers.rs unit, `install_until_end_of_turn` + Humility-style fixture) | `add_transient_continuous_effect` + `evaluate_layers` | U1-a: a TCE `AddStaticMode{CantBlock}` with an earlier timestamp than a `RemoveAllAbilities` static leaves exactly one CantBlock def, `origin Resolution`. U1-b: a **static-ability**-sourced `AddStaticMode` graft (Psychic Paper class) is still removed (#282 unchanged). U1-c: TCE `GrantStaticAbility` with inner `origin Characteristic` is removed; with `Resolution` it is kept. U1-a and U1-c(kept) fail on BASE | U1-b and U1-c(removed) are hostile siblings |
| V6 | `object_has_no_abilities` ignores Resolution statics | Muraganda on the battlefield; Panic on vanilla P1 Bears | Bears stays 4/4. **Fails on BASE** (P3) | a printed-keyword creature gets no +2/+2 (filter live) |
| V7 | U2 vehicle survives (Promise of Loyalty) | resolve Promise (promise_of_loyalty.rs flow, copied), cast Humility in main 1, advance to P1's declare attackers | P1 `declare_attackers([(keeper, Player(P0))])` Err. **Fails on BASE** | an unmarked bystander attacks P0 Ok; reach guard: keeper base P/T 1/1 (Humility in force); existing promise_of_loyalty.rs rows stay green |
| H1 | a granted **keyword** is still removed | Jump on P0 Bears, then Dress Down, then attack | P1 ground Bears blocks Ok (flying gone, CR 702.9b) | control without Dress Down: the block is Err (proves Jump applied) |
| H2 | a quoted static grant is still removed (U3 guard) | P0 attacks with Hill Giant; cast Craterous Stomp at P1 creature X (dies); P1 creature Y gains the quoted ability; cast Dress Down | Y **may** block the Giant (Ok); Y is still a Coward (layer 4 untouched). Passes on BASE; **fails on U1-without-U3** | control without Dress Down: Y blocking the Giant is Err |
| H3 | quoted CantBlock shape removed | Incite Hysteria (main 1) on P1 Bears, then Dress Down | Bears may block (Ok). Same discrimination as H2 | control without Dress Down: Err |
| H4 | timestamp: a grant after the removal is kept (Dress Down ruling) | Dress Down first, then Jump | P1 ground Bears cannot block (Err). Non-discriminating guard | — |
| U3-p | parser unit | `parse_quoted_rule_static_modifications("This creature can't block.")` | `[GrantStaticAbility{mode CantBlock, affected SelfRef}]`. Fails on BASE | Dancer's Chakrams-shape non-trivial stays GrantStaticAbility (unchanged) |
| U3-q | perpetual parity | the existing perpetual quoted-static tests (effects/perpetual.rs) | still green; the TryFrom maps the trivial grant to `AddStaticMode` | a `GrantStaticAbility` with modifications is still rejected |
| U2-p | parser unit | Promise and Summon: Yojimbo lowerings | inner def `origin == Resolution` | an Incite Hysteria lowering has no `origin` key |

Coverage impact: none (P4). Strict-failure markers: none needed (nothing newly accepted).

**Reference Readings.**
- H2/H3 assert the BASE value (quoted ability removed). It is derived independently from CR 113.10 and the Incite Hysteria ruling; it is not parity.
- H4 matches BASE; derived from the Dress Down ruling.
- P5's "runtime parity" for the U3 cards is a preservation claim. It is derived per card: the quoted text grants a SelfRef static with exactly that mode and no controller-relative scope. The executor confirms no anchor-bearing mode is in the set, and any card that needs an anchor goes to §9 Q7, not into parity.
- Existing Promise rows (vow removal, control change) are preserved by the unchanged TCE duration; derived from the Promise rulings.

## 7. Step-by-step implementation

**U3 (first).**
1. `crates/phase-parser/src/parser/oracle_static/grammar.rs::parse_quoted_rule_static_modifications`: replace the `affected == SelfRef && condition none && modifications empty → AddStaticMode` branch with an unconditional `GrantStaticAbility { definition: Box::new(definition) }`. Rewrite the comment: CR 113.3d + CR 113.10 + CR 604.1, a quoted static is an ability the recipient gains; it is never a rule-effect `AddStaticMode` (CR 611.2c), and layer-6 ability removal depends on that distinction.
2. `crates/phase-ast/src/types/ability.rs` `TryFrom<ContinuousModification> for PerpetualGrantModification`: add an arm for `GrantStaticAbility { definition }` when the definition is trivial (`affected == Some(SelfRef)`, `condition.is_none()`, `modifications.is_empty()`, and every other field default). It maps to `Self::AddStaticMode { mode: definition.mode }`; otherwise `Err`. Annotate it as the perpetual-baseline install of a quoted static ability.
3. Census (P4/P5): grep engine, phase-ai and phase-parser code that pattern-matches `AddStaticMode { mode: StaticMode::<X> }` for each mode in the P4 diff list. Each match becomes the same reading for the trivial `GrantStaticAbility` or gets a one-line reason. Expected modes: MustAttack, GraveyardCastPermission, CrewContribution, BlockRestriction, CantAttack, CantUntap, ReduceAbilityCost, CantBlock, Other, CastWithAlternativeCost, CantBeBlocked, ModifyCost, CanAttackWithDefender, UntapsDuringEachOtherPlayersUntapStep, DamageNotRemovedDuringCleanup, CantBeBlockedByMoreThan. Also check the token-builder (`oracle_effect/token.rs`, `engine game/effects/token.rs:6133`), become_copy_except, return_as_aura and oracle_replacement callers of `classify_quoted_inner`. Insta snapshots that change are fixtures; accept them after review.
Predicted U3 diff (resolving effects): Carnage, Crimson Chaos; Case of the Uneaten Feast; Craterous Stomp; Elrond of the White Council; Full Steam Ahead; Game Knights Live; Incite Hysteria; Jon Irenicus, Shattered One; Kethis, the Hidden Hand; Ogre Marauder; Richlau, Headmaster; Scalar Scholar; Stilt-Man, Towering Terror; Surge Engine; Teferi's Contingency; Tetravus; Vronos, Masked Inquisitor; Wilson, Subtle Bear. Printed static lines: Karlach ×5, Ethereal Grasp, Cloudspire Coordinator, Firion, Incessant Provocation, Mine Security, Sanguine Soothsayer, Switchgrass Grazer, Teyo, Aegis Adept, Urban Burgeoning (approximate; P4 is authoritative).

**U1.**
4. `phase-ast` ability.rs: rename `ReplacementOrigin` → `DefinitionOrigin` (13 refs; `rg -n ReplacementOrigin crates`) and generalize its doc. Characteristic = an ability or characteristic of the object (CR 109.3, CR 613.1). Resolution = a continuous effect generated by resolution (CR 611.2a), not a characteristic (CR 611.2c), not removed by ability-removing effects (CR 613.1f); replacements are carried across the reset by `reseed_…`, statics are re-grafted each pass. Add `pub origin: DefinitionOrigin` to `StaticDefinition` with the serde attributes of `ReplacementDefinition::origin`, plus `pub fn is_resolution_installed(&self) -> bool` and `pub fn resolution_installed(mut self) -> Self`. Initialize the field in `StaticDefinition::new`/`continuous` and fix every compiler-forced literal with `origin: DefinitionOrigin::Characteristic`.
5. layers.rs `AddStaticMode` arm: after the existing stamps and before the dedup, `if effect.transient_id.is_some() { def.origin = DefinitionOrigin::Resolution; }` with `// CR 611.2c + CR 613.11: a rule effect generated by a resolving spell or ability is stored on its recipient, not one of the recipient's abilities.`
6. layers.rs `RemoveAllAbilities` arm: `obj.static_definitions.retain(StaticDefinition::is_resolution_installed);` and extend the existing CR 613.1f + CR 611.2c comment to cover statics. Leave `set_land_subtype_replacing` as it is: it runs in layer 4, before this pass's layer-6 grafts, so it never holds one (one-line comment).
7. filter.rs `object_has_no_abilities`: `obj.static_definitions.iter_all().all(StaticDefinition::is_resolution_installed)` in place of `is_empty()`. Comment: CR 611.2c, a resolved rule effect is not an ability. (The analogous replacement conjunct is a pre-existing gap, §9 Q4.)
8. Tests: `crates/engine/tests/integration/resolved_restriction_ability_removal.rs` (V1–V4, V6, H1–H4), registered by `mod resolved_restriction_ability_removal;` in shard_5.rs (byte order ≥ `reflexive_body_token_referent`). Add the V5 unit tests in layers.rs's test module beside `humility_does_not_end_a_resolution_created_shield`. Do the essence_burn.rs rename.

**U2.**
9. Census: every non-test `ContinuousModification::GrantStaticAbility {` constructed inside an `Effect::GenericEffect` in crates/phase-parser/src/parser/ outside grammar.rs, e.g. subject.rs (Promise class, combat tax) and oracle_effect/mod.rs (temporary spell cost modification, three sites; `graveyard_permission_grant`; flash grant). Each one wraps its inner def with `.resolution_installed()`, commented `// CR 611.2c: a resolution rule effect hosted on the recipient, not a granted ability.` Engine-side GenericEffect builders (`database/synthesis.rs`, `game/deck_loading.rs`, `casting.rs:13403`) are checked the same way.
Predicted U2 diff (non-quoted vehicles): Alchemist's Refuge, Armor Wars, Ballad of the Black Flag, Borne Upon a Wind, Chainer, Nightmare Adept, Chomping Kavu, Complete the Circuit, Emergence Zone, Forbidding Spirit, Forgotten Cellar, Gaea's Will, Goblin Maskmaker, Izzet Steam Maze, Liliana, Untouched by Death, Magus of the Will, Ood Sphere, Promise of Loyalty, Rowan, Scion of War, Spellbinding Soprano, Streetwise Negotiator, Summon: Yojimbo, Teferi, Time Raveler, Teyo, Geometric Tactician, The Great Work, The Legend of Arena, The Pieces Are Coming Together, The Tomb of Aclazotz, Urza, Planeswalker (×3 printings), War Cadence, War Tax, Whispersteel Dagger, Will, Scion of Peace, Winding Canyons, Yawgmoth's Will (+ Immortal Obligation's CantAttack in the stacked PR, which must use the same builder). Executor regenerates the list: `jq` over card-data for GenericEffect `GrantStaticAbility` on cards whose Oracle text has no quoted grant. A vehicle whose inner def carries `modifications` (e.g. Streetwise Negotiator) is flagged, and its synthesized-effect behavior under removal is measured.
10. Test V7 + U2-p.

**CR citations (all grep-verified in docs/MagicCompRules.txt):** 109.3 (l.592), 109.5 (l.610), 113.3d (l.767), 113.10 (l.813), 508.1c (l.2273), 509.1b (l.2352), 604.1 (l.2669), 611.2 (l.2908), 611.2a (l.2910), 611.2c (l.2915), 613.1c (l.2970), 613.1d (l.2972), 613.1f (l.2976), 613.4c (l.2996), 613.7 (l.3013), 613.11 (l.3056), 701.15b (l.3402), 701.60c (l.3817), 702.9b (l.3978).

## 8. Sibling census (consumers of restriction statics / ability-removal)

- Combat attack/block legality (CantAttack, CantBlock, CantAttackOrBlock, BlockRestriction, CantBeBlocked*, MustAttack/MustBlock*): **fixed by U1** (they read grafted statics).
- Goad: TCE-read, already survives; unchanged (V-rows keep it untouched; the stacked Immortal Obligation PR tests it).
- CantBeTargeted / CantBeCountered / opponent-relative CantBeBlockedBy: TCE-read, already survive; unchanged.
- CantUntap / CantPhaseIn: TCE raw scans (turns.rs `cant_untap_ids`, `transient_grants_static_mode_to_object`), already survive; unchanged.
- Suspected (CR 701.60c): derived abilities at reset, removed by later ability removal, which is correct (they are abilities); unchanged.
- `set_land_subtype_replacing` (layer 4): never holds a layer-6 graft; comment only.
- Text substitution (layer 3, CR 613.1c): runs before layer-6 grafts, so a graft is never rewritten; n/a.
- `object_has_no_abilities`: **fixed by U1** (statics); replacement half is a follow-up (Q4).
- Static-line restriction grafts from printed statics (Psychic Paper "it can't be blocked", #282): **out of scope**. They are CR 611.3 effects of a static ability, not resolution effects, and need a third provenance value plus the printed quoted class that U3 un-collapses. U1-b pins the unchanged behavior.
- Frozen population of broadcast rule effects (Falter/Magmatic Chasm rulings, CR 611.2c second sentence): pre-existing, adjacent, out of scope (Q6).

## 9. Open questions for the orchestrator / Jakob

1. **Parse diff is not none.** Two parser-output changes are strictly required (§1.1 evidence): U3 (~30 cards change `AddStaticMode` → `GrantStaticAbility`, no support change) and U2 (~37 cards gain `origin: "Resolution"` on vehicle inner defs). Does Jakob accept this in place of the task's "none expected"?
2. **Phasing.** T1 (3 units) and T2 (≈25–30 paths) both hold, so the phase-fit gate should fire. Suggested charter: P1 = U3, P2 = U1, P3 = U2 (or U3+U1 together). Should Promise / Immortal Obligation's attack clause (U2) ride this PR or a follow-up stacked under #9608?
3. Rename `ReplacementOrigin` → `DefinitionOrigin` (preferred, single concept), or keep the replacement name and reuse it on statics?
4. `object_has_no_abilities` also counts resolution-installed **replacements** as abilities (a pre-existing gap). Include its one-conjunct fix here (needs a regenerate-shield row) or defer?
5. Craterous Stomp correction (§0): confirm it is a hostile witness, not an affected card.
6. Follow-ups to file: CR 611.2c dynamic population for broadcast resolution rule effects; Psychic Paper #282; quoted grants' CR 109.5 "you" (after U3 no installer anchor applies to quoted statics).
7. If P5 finds a U3 card whose mode needs an AddStaticMode-only anchor, should the GrantStaticAbility arm reproduce it for trivial quoted defs (parity), or drop it as CR 109.5-correct for an ability? Planner default: drop, with a derived row.
