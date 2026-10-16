# Plan r1 — Unbound Flourishing (class: X-cost spell/ability amplifiers)

Planner run: engine-planner, ordinary mode, round 1. Base: committed HEAD `6295aa7` (read with `git show HEAD:<path>` for the six files another executor is editing right now; that executor's uncommitted diff is in `oracle_nom/condition.rs` and `types/ability.rs`, adding `NumberQuality` and the prime/even/odd control-count condition, which this plan does not touch). **No probes were run.** The caller ruled out cargo for this round, so every runtime claim below is either read from source (and labelled **[read]**) or explicitly **[UNPROBED]**. The executor's first job is to turn the UNPROBED rows into measured ones (see §Probes owed).

---

## Step 0 — Premise verification (hard gate)

Source: local MTGJSON `data/mtgjson/AtomicCards.json` (`.data["Unbound Flourishing"][0].text`). It matches the task brief verbatim:

> Unbound Flourishing {2}{G} — Enchantment
> Whenever you cast a permanent spell with a mana cost that contains {X}, double the value of X.
> Whenever you cast an instant or sorcery spell or activate an ability, if that spell's mana cost or that ability's activation cost contains {X}, copy that spell or ability. You may choose new targets for the copy.

All 14 rulings are in `/mnt/project-files/unbound-zimone/oracle-verified.md`, and the load-bearing ones were re-read. Fixture cards are also verified from the same file: Walking Ballista `{X}{X}` "This creature enters with X +1/+1 counters on it. …"; Hydroid Krasis `{X}{G}{U}` "When you cast this spell, you gain half X life and draw half X cards. Round down each time. / Flying, trample / This creature enters with X +1/+1 counters on it."; Stifle `{U}` "Counter target activated or triggered ability. (Mana abilities can't be targeted.)"; Counterspell `{U}{U}` "Counter target spell."; Rings of Brighthearth (its ruling: "The triggered ability of Rings of Brighthearth and the copy it creates both resolve … even if that ability is countered").

**Current parse.** Measured from `data/card-data.json` (`jq '."unbound flourishing".triggers'`):
- Line 1 gives `SpellCast` with `valid_card = Typed[Permanent]` and **no `HasXInManaCost`**, so "with a mana cost that contains {X}" is silently dropped and the trigger over-fires on every permanent spell. The effect is `Unimplemented("unparsed_quantity", "double the value of X")`.
- Line 2 gives one `SpellCast` with `valid_card = Or[Instant, Sorcery]`. The **"or activate an ability" leg is silently dropped.** The effect is `Unimplemented("unparsed_condition", "if that spell's mana cost or …")`.

Root cause of the line-1 drop **[read]**: in `oracle_trigger.rs` (HEAD ~19491–19520), `parse_spell_qualifier_payload` returns `None` because `parse_post_spell_modifier` does not know "with a mana cost that contains {x}". The fallback `parse_type_phrase_folding(payload)` then discards its `_rest`.

---

## Step 1 — Applicable skills (checklist skeleton)

| Skill | Why | Used as |
|---|---|---|
| `/add-engine-variant` | new `Effect` variant, new parser-local enums, new `GameState` field | gate (§Variant Discoverability) |
| `/add-engine-effect` | new `Effect::ModifyAnnouncedX` resolver | Phases 1–8 checklist (§Step 5, U3) |
| `/add-trigger` | compound cast-or-activate trigger, intervening-if, `SpellCast` / `AbilityActivated` matchers | U2 |
| `/oracle-parser` | nom combinators in `oracle_trigger.rs` and `oracle_effect/counter.rs` | U1, U2, U3-parser |
| `/casting-stack-conditions` | "X Channels" contract: this effect *rewrites* X on the stack, per CR 107.3c | U3 authority |
| `/card-test` | every runtime row uses `GameScenario` + `GameRunner::cast(..).resolve()` with verbatim Oracle | §Verification Matrix |

`/add-interactive-effect` does not apply: no new `WaitingFor`. The copy retarget prompt is the existing `copy_choice::open_copy_retarget_walk`. `/add-frontend-component` does not apply: no new UI.

---

## Step 2 — Analogous traces (hard gate)

**T-A: Magus Lucea Kane delayed "next {X} spell or ability" copy.** This is the closest feature: the same two filters and the same copy effect.
`parser/oracle_effect/mod.rs::try_parse_when_next_event` → `try_parse_when_next_spell_or_activate_disjunction` (`take_until(" or activate an ability ")`) → `extract_when_next_spell_filter` + `oracle_trigger::parse_post_activation_modifier` → `build_when_next_delayed_trigger(SpellCast, spell_filter, inner, Some((AbilityActivated, ability_filter)))` → `types/ability.rs DelayedTriggerCondition::WhenNextEvent { trigger, or_trigger }` → runtime `game/triggers.rs` (WhenNextEvent arm, chains `or_trigger` through `trigger_matchers::trigger_matcher`) → `trigger_matchers::match_spell_cast` / `match_ability_activated` → `activated_source_matches` → `filter.rs` `FilterProp::HasXInActivationCost` → `casting_costs::pending_activation_cost_has_x` (reads `state.pending_activations`, pushed in `casting.rs` at activation placement, cleared on PassPriority in `engine.rs`) → `game/effects/copy_spell.rs::resolve` → `copy_source_entry` → `targeting::triggering_spell` (spell; `OnStack` / `Departed` / `Gone`) or `triggering_activation` (ability; `GameEvent::AbilityActivated.stack_entry`; **returns `Gone` when the entry left the stack**) → `stack::push_copy_to_stack` → `copy_choice::open_copy_retarget_walk`. Test: `crates/engine/tests/integration/issue_1010_magus_lucea_kane.rs` (spell X=2 and activated X=2, each drawing 4 in total).

**T-B: printed compound-trigger splitting.** `oracle_trigger.rs::parse_trigger_lines_at_index_ir` → `split_trigger` (condition / effect at the first effect comma) → Pattern 1 `split_and_when_compound`, the zone-change gate, Pattern 2 `split_shared_subject_event_list` (`split_serial_event_compound` / `split_cross_subject_event_compound` / `split_or_event_compound`) → each half becomes `format!("{cond}, {effect}")` → `parse_trigger_line_with_index_ir(.., base_trigger_index.map(|b| b.offset(i)), ..)` → `lower_trigger_ir`. Building blocks: `parse_trigger_keyword_prefix`, `extract_keyword_and_subject`. The active-voice "whenever you activate an ability" half is parsed by `try_parse_keyword_activation_trigger::parse_line` into `AbilityActivated` with `valid_target = Controller`. This is the same output as Rings of Brighthearth in card-data.

**T-C: `Effect::Double`**, the "double …" effect family. `types/ability.rs Effect::Double { target_kind: DoubleTarget, target }` → parser `oracle_effect/counter.rs::try_parse_double_effect` (nom dispatch on "double …") → `game/effects/double.rs::resolve` → `effects/mod.rs` dispatch → registrations in `ability_rw.rs` (`obj(StateKind::ObjectCounters, target)`), `ability_scan.rs`, `coverage.rs`, `ability_visit.rs`, `oracle_ir/{ast,doc}.rs`, `resolution_prompt.rs`, `analysis/ability_graph.rs`, `phase-ai/policies/{effect_classify,redundancy_avoidance}.rs`. These sites were found by grepping `Effect::DoublePTAll` / `Effect::Double`; they are the compiler-forced set a new variant will hit.

**T-D: the X channels of a spell on the stack.**
- `casting_costs.rs` finalize (HEAD ~12130–12200) stamps `obj.cost_x_paid = ability.chosen_x`, then at ~12536 inserts `state.stack_paid_facts[object_id] = StackPaidSnapshot { x_value: cost_x_paid, .. }`, then pushes `GameEvent::SpellCast`.
- **Ordering resolved [read]:** both stamps precede the `SpellCast` event, so a cast trigger always sees the cast X.
- Readers:
  - `stack.rs` ~2553: `resolving_spell_x = paid_snapshot.x_value.or(obj.cost_x_paid)`. The **paid snapshot wins** at resolution. It feeds intrinsic ETB counters and the "enters with X counters" path.
  - `quantity.rs` `QuantityRef::CostXPaid`: trigger_source / object `cost_x_paid`, falling back to `chosen_x`.
  - `quantity.rs` `QuantityRef::Variable{"X"}`: `chosen_x`, then the `current_trigger_event` source object's `cost_x_paid` (the CR 107.3e fallback that Hydroid Krasis uses), then trigger_source.
  - `types/mana.rs::mana_value_with_x(zone, cost_x_paid)` gives mana value on the stack (CR 202.3e).
  - `derived_views.rs::stack_paid_facts` gives the UI X display.

  Only `set_chosen_x_recursive` writes `chosen_x` after announcement, so **no effect today rewrites a stack spell's X**.

---

## Step 3 — Files read (HEAD unless noted)

`types/ability.rs` (TriggerDefinition, TriggerCondition, FilterProp HasX*, Effect::Double/CopySpell, DoubleTarget, ResolvedAbility, `set_chosen_x_recursive`), `types/triggers.rs` (TriggerMode compound siblings, AbilityActivated doc), `types/events.rs` (`GameEvent::AbilityActivated` fields incl. `stack_entry`, `kind`), `types/game_state.rs` (`cost_x_paid`, `StackPaidSnapshot`, `stack_paid_facts`, `departed_stack_spells`, `retain_trigger_referenced_departed_spells`, `pending_activations`, `AbilityActivationRecord`), `parser/oracle_trigger.rs` (parse_trigger_lines_at_index_ir, split_* family, split_trigger, try_parse_keyword_activation_trigger, SpellCast payload path, parse_spell_qualifier_payload, parse_post_spell_modifier, parse_post_activation_modifier), `parser/oracle_effect/mod.rs` (try_parse_when_next_event, disjunction helper), `parser/oracle_effect/counter.rs` (try_parse_double_effect), `parser/oracle_ir/context.rs` (ParseContext.object_pronoun_ref), `parser/oracle_ir/trigger.rs` (TriggerIr.partial_def), `game/trigger_matchers.rs` (registry, match_ability_activated, activated_source_matches), `game/trigger_index.rs` (keys_from_trigger_def), `game/triggers.rs` (printed match loop, OncePerTurn keyed by TriggerDefinitionRef, delayed WhenNextEvent arm, TriggeringSpellMatchesFilter / ActivatedAbilityIsNonMana evaluators), `game/effects/copy_spell.rs` (resolve, copy_source_entry, triggering_activation, ActivatedAbility copy arm), `game/effects/double.rs`, `game/targeting.rs::triggering_spell`, `game/stack.rs` (record_departed_stack_spell, remove_nonresolving_stack_entry_at, resolving_spell_x), `game/casting_costs.rs` (finalize stamps, cost_has_x, ability_cost_has_x, pending_activation_cost_has_x), `game/casting.rs` (pending_activations push, emit_ability_activated), `game/casting_targets.rs::emit_ability_activated`, `game/effects/counter.rs` (uses remove_nonresolving_stack_entry_at), `game/quantity.rs` (Variable X / CostXPaid), `game/ability_rw.rs` (StateKind, rw_effect `obj`, Double/ChangeTargets/CopySpell arms, `QuantityRef::Variable => RwProfile::empty()`), `game/filter.rs` (HasX* evaluation), `game/turns.rs:1303` (departed clear), `game/visibility.rs` (departed projection), `tests/integration/issue_1010_magus_lucea_kane.rs`, `.claude/skills/{engine-planner,add-engine-variant,add-engine-effect,casting-stack-conditions(X Channels),card-test}/SKILL.md`, the engine-implementer phase-fit section.

---

## Pattern Coverage

| Building block | Cards it changes | Direction |
|---|---|---|
| U1 cost-X qualifier combinator ("with {X} in its mana/activation cost" ∪ "with a(n) mana/activation cost that contains {X}") | Unbound line 1 (fixes the over-fire). Re-points Brass Infiniscope, Anina, Elementalist's Palette, Geometer's Arthropod, Lattice Library, Matterbending Mage, Nev, Owlin Spiralmancer, Zaxara, Zimone Infinite Analyst and Magus onto the shared combinator; output must not change | 1 fixed, ~11 unchanged |
| U2 cast-or-activate compound split ("<actor> cast(s) <spell> or activate(s) an ability[<qualifier>]") plus the cost-X intervening-if fold | Unbound (gains the activation leg). Bill Potts, Glava, Neerdiv and Vazi also have their silently dropped activation leg surfaced: each becomes a real `AbilityActivated` trigger or an honest `Unknown`/`Unimplemented` | 1 fixed; 4 changed toward honest coverage (measure) |
| U3 `Effect::ModifyAnnouncedX` + `set_stack_object_announced_x` authority | Unbound line 1. Glava's "have the value of X become 5" is the second member of the class; it gets a `Become` modification later, deferred | 1 (+1 queued) |
| U4 same-event ordering: X-on-event-object writes conflict with event-live X reads | Unbound against every same-event cast trigger whose body reads the spell's X via `Variable("X")` (Hydroid Krasis and the `Variable X` cast-trigger class) | ordering prompts (rules-required by ruling 4) |
| U5 departed activated-ability record so `CopySpell(TriggeringSource/context)` copies a countered ability | Unbound, Rings of Brighthearth (ruling: copy resolves "even if that ability is countered"), Magus Lucea Kane, Illusionist's/Battlemage's Bracers, every "copy that ability" trigger | ~6+ |

Card count for the whole task is about 20 cards touched, with Unbound as the anchor. It is not a one-card build.

---

## Sizing (mandatory)

**Units** (one coherent behavior per single skill-checklist pass):

| # | Unit | Registration surfaces | Discriminating test |
|---|---|---|---|
| U1 | Shared cost-X qualifier combinator, wired into `parse_post_spell_modifier` and `parse_post_activation_modifier` | `parser/oracle_trigger.rs` | Parser: "Whenever you cast a permanent spell with a mana cost that contains {X}, …" → `valid_card` carries `HasXInManaCost` (reverting it drops the prop). Runtime: a non-X permanent spell does not fire line 1 (paired with an X spell that does) |
| U2 | Cast-or-activate compound split plus the cost-X intervening-if fold into per-half `valid_card` | `parser/oracle_trigger.rs` | Parser: line 2 → exactly two triggers (`SpellCast` Or[Instant,Sorcery] ∧ HasXInManaCost; `AbilityActivated` HasXInActivationCost), both `CopySpell{MayChooseNewTargets}`. Runtime: an X instant is copied and a `{X}` activated ability is copied; non-X siblings are not |
| U3 | `Effect::ModifyAnnouncedX { target, modification: XModification::Double }` with resolver, single X-rewrite authority, and parser arm "double the value of x" | `types/ability.rs`, `game/effects/modify_announced_x.rs` (new), `game/effects/mod.rs`, `game/stack.rs`, `parser/oracle_effect/counter.rs`, plus compiler-forced `ability_scan.rs`, `ability_rw.rs`, `ability_visit.rs`, `coverage.rs`, `resolution_prompt.rs`, `oracle_ir/ast.rs`, `oracle_ir/doc.rs`, `analysis/ability_graph.rs`, `phase-ai/.../effect_classify.rs`, `phase-ai/.../redundancy_avoidance.rs` | Walking Ballista X=2 + Unbound enters with 4 counters and has mana value 8 on the stack while 4 mana is spent; two Unbounds give 8 counters |
| U4 | Ordering-profile honesty: `ModifyAnnouncedX` writes the event object, and `QuantityRef::Variable{"X"}` is an event-live read | `game/ability_rw.rs` | Hydroid Krasis X=4 + Unbound: the engine presents `OrderTriggers` (not auto-ordered). Doubling first gives +4 life and 4 cards drawn; Krasis first gives +2 and 2 |
| U5 | Departed activated-ability record, consumed by `copy_spell::triggering_activation` | `types/game_state.rs`, `game/stack.rs`, `game/turns.rs`, `game/effects/copy_spell.rs`, `game/visibility.rs` (compiler-forced destructure, if any) | Activate `{X}` Draw-X with Unbound, Stifle the ability in response, then Unbound's copy still resolves and draws X. On HEAD it draws 0 (`CopySourceLookup::Gone`) |

**Dependency edges:**
- U1 → U2: U2's fold reuses U1's `CostSlot` combinator.
- U3 → U4: U4 classifies U3's writer. Its read-side change is independent but has no discriminating test without U3.
- U5 is infrastructure. Its consumer is U2's copy test, and it is testable on its own with Rings of Brighthearth / Magus if phased first.
- The card's coverage only goes green after U1 + U2 + U3.

**Expected scope-path count** (phase-fit counting rule). Test fixtures and the regenerated `card-data.json` / `coverage-data.json` are excluded. There are no protocol-version surfaces: `Effect` gains an additive serde-tagged variant and `GameState` an additive `#[serde(default, skip_serializing_if)]` field, with no wire-version bump.

Authored product files (one group each):
1. `crates/engine/src/types/ability.rs`
2. `crates/engine/src/game/effects/modify_announced_x.rs` (new)
3. `crates/engine/src/game/effects/mod.rs`
4. `crates/engine/src/game/stack.rs`
5. `crates/engine/src/game/ability_rw.rs`
6. `crates/engine/src/game/ability_scan.rs`
7. `crates/engine/src/types/ability_visit.rs`
8. `crates/engine/src/game/coverage.rs`
9. `crates/engine/src/game/resolution_prompt.rs`
10. `crates/engine/src/parser/oracle_ir/ast.rs`
11. `crates/engine/src/parser/oracle_ir/doc.rs`
12. `crates/engine/src/analysis/ability_graph.rs`
13. `crates/phase-ai/src/policies/effect_classify.rs`
14. `crates/phase-ai/src/policies/redundancy_avoidance.rs`
15. `crates/engine/src/parser/oracle_effect/counter.rs`
16. `crates/engine/src/parser/oracle_trigger.rs`
17. `crates/engine/src/types/game_state.rs`
18. `crates/engine/src/game/turns.rs`
19. `crates/engine/src/game/effects/copy_spell.rs`
20. `crates/engine/src/game/visibility.rs` (only if a GameState destructure forces it)

Test sources (counted, since they are authored tests and not fixtures):
21. `crates/engine/src/parser/oracle_trigger_tests.rs`
22. `crates/engine/src/parser/oracle_effect/tests.rs`
23. `crates/engine/tests/integration/unbound_flourishing.rs` (new)
24. `crates/engine/tests/integration/main.rs`

**About 22–24 paths, 5 units.** T1 (≥2 units) and T2 (≥13 paths) both fire, so the phase-fit gate is expected to decompose this. Candidate green-tree seams in dependency order:
- **Phase A: U1 + U2** (parser only, about 3 product + 2 test paths). Line 1's effect stays `Unimplemented`, so the card stays honestly unsupported. The copy line is fully runtime-tested.
- **Phase B: U5** (about 5 paths). It is independently testable with Rings of Brighthearth or Magus.
- **Phase C: U3** (about 15 paths, mostly compiler-forced one-line arms). This is the one unit whose forced-site fan-out alone exceeds 13; it is a single unit, so T1 does not fire for it alone.
- **Phase D: U4** (1–2 paths). Its test needs U3.

Each phase is a single unit or two parser units sharing one file; per-phase counts are under 13 except C, which is single-unit.

---

## Building Blocks (composed, by name)

- **Parser:**
  - `nom` `alt`/`tag`/`value`/`opt`/`preceded`/`terminated`/`separated_list1`/`take_until`/`eof`/`all_consuming`.
  - `oracle_trigger::{parse_trigger_keyword_prefix, extract_keyword_and_subject, split_trigger, parse_trigger_line_with_index_ir, parse_spell_qualifier_payload, parse_post_spell_modifier, parse_post_activation_modifier, try_parse_keyword_activation_trigger}`.
  - `oracle_effect::counter::try_parse_double_effect` (host dispatch).
  - `ParseContext::object_pronoun_ref` (spell-cast trigger body sets `TriggeringSource`).
  - `TextPair` for case-preserving half reconstruction.
  - `TargetFilter::And` composition exactly as `parse_spell_qualifier_payload` does it.
- **Runtime:**
  - `trigger_matchers::{match_spell_cast, match_ability_activated}` (unchanged).
  - `filter.rs` `HasXInManaCost` / `HasXInActivationCost` (unchanged).
  - `targeting::triggering_spell` (incarnation-pinned `OnStack`/`Departed`/`Gone` authority, CR 400.7).
  - `ResolvedAbility::set_chosen_x_recursive`.
  - `state.stack_paid_facts`.
  - `stack::record_departed_stack_spell` and `remove_nonresolving_stack_entry_at` (extended).
  - `copy_spell::{copy_source_entry, triggering_activation, CopySource}`.
  - `GameState::retain_trigger_referenced_departed_spells` (sibling added).
  - `ability_rw::rw_effect` `obj(..)` helper and `place_object_write`.
- **New helpers, each justified:**
  - (a) `parse_x_cost_slot_qualifier`: one combinator replaces two hand-duplicated `alt`s and adds the "cost that contains {X}" grammar for both slots.
  - (b) `split_cast_or_activate_compound`: no existing splitter covers a cast/activate disjunction. The global event-head lexicon is not widened (see Extension vs Creation).
  - (c) `game/stack.rs::set_stack_object_announced_x`: the **single authority** for rewriting X on a stack object, because three channels must move in lockstep.
  - (d) `stack::record_departed_stack_ability`: the activation sibling of `record_departed_stack_spell`.

## Logic Placement

- **Grammar:** the cost-X qualifier, the compound split and the intervening-if fold live in `parser/oracle_trigger.rs`, the trigger-condition owner. The fold is source/event-referential ("that spell's", "that ability's"), the CLAUDE.md-sanctioned exception to `parse_inner_condition` single authority. The "double the value of X" verb lives in `oracle_effect/counter.rs::try_parse_double_effect`, next to the other "double …" dispatch.
- **Effect semantics:** in `game/effects/modify_announced_x.rs`.
- **X rewrite:** in `game/stack.rs`, which owns stack entries and `stack_paid_facts` removal/insert, so the writer lives with the other stack-object mutators.
- **Departed-ability lifecycle:** in `stack.rs` (record), `turns.rs` (clear), `game_state.rs` (field and loop-normalization prune), and `copy_spell.rs` (consume).
- **Ordering profile:** in `ability_rw.rs`.
- Nothing in the transport or frontend layers.

## Rust Idioms

- `XModification` is an enum, not a `bool`, and is extensible to Glava's `Become { value: QuantityExpr }`. It has an exhaustive `apply(self, current: u32) -> u32` (`Double => current.saturating_mul(2)`).
- `CostSlot { Mana, Activation }` is a parser-local enum with `fn x_filter_prop(self) -> FilterProp`, matched exhaustively.
- The `set_stack_object_announced_x` match over `StackEntryKind` is exhaustive with no `_` arm: `Spell` and `ActivatedAbility` write; `TriggeredAbility`, `KeywordAction` and `CombatDamage` are no-ops, documented with CR 107.3a ("spell or activated ability"). The new variant is destructured without `..` in `ability_rw.rs` and `ability_scan.rs`.
- The new `GameState` field is `im::HashMap<ObjectId, StackEntry>` with `#[serde(default, skip_serializing_if = "im::HashMap::is_empty")]` and the deterministic serializer that `departed_stack_spells` uses.

## Nom Compliance (parser files change)

| Step | Combinator |
|---|---|
| Cost-X qualifier | `alt((preceded((tag("with "), opt(tag("an ")), tag("{x} in its ")), terminated(cost_slot, tag(" cost"))), preceded(alt((tag("with a "), tag("with an "))), terminated(cost_slot, tag(" cost that contains {x}")))))`, where `cost_slot = alt((value(CostSlot::Mana, tag("mana")), value(CostSlot::Activation, tag("activation"))))`. Callers require `eof` (or `rest.trim().is_empty()`, matching the existing arms) and the right slot |
| Compound split detection | `(parse_trigger_keyword_prefix, actor_agreement, take_until(or_activate_tag), or_activate_tag, rest)`. `actor_agreement = alt((value(Agreement::Base, tag("you ")), value(Agreement::ThirdPerson, alt((tag("an opponent "), tag("a player "), tag("another player "))))))`, and the verbs are chosen by agreement: `tag("cast ")` / `tag(" or activate an ability")` vs `tag("casts ")` / `tag(" or activates an ability")`. The ability tail is `alt((eof, preceded(space1, rest)))` so "an abilityx" cannot match |
| Intervening-if fold | `terminated(preceded(tag("if "), separated_list1(tag(" or "), event_cost_subject)), (tag(" contains {x}"), tag(", ")))` with `event_cost_subject = alt((value(EventCostSubject::Spell, tag("that spell's mana cost")), value(EventCostSubject::Ability, tag("that ability's activation cost"))))`. **Lowercase** effect text is run through `nom_on_lower` so the original-case remainder is preserved |
| "double the value of x" | `nom_on_lower(lower, lower, \|i\| value((), (tag("double the value of x"), eof)).parse(i))` inside `try_parse_double_effect`, placed **before** the generic `tag("double ")` arm |

No `contains`/`find`/`starts_with`/`split_once` is used for dispatch. `TextPair::strip_prefix`/slicing is used only for structural original-case reconstruction of the half texts.

## Extension vs Creation

- **U1 extends** `parse_post_spell_modifier` / `parse_post_activation_modifier`. It factors their duplicated X arms into one combinator.
- **U2 extends** the established printed-trigger compound-split pattern (T-B): one trigger per event, sharing the effect, with offset base indexes. It is a dedicated splitter rather than adding "activate"/"activates" to `parse_event_verb_start`, because that lexicon has 11 census-pinned consultation sites (see `split_or_event_compound`'s doc). Widening it would move unrelated cards. The dedicated splitter touches only lines with the `cast … or activate an ability` shape.
  - *Rejected alternative:* a `TriggerDefinition.or_trigger` field (mirroring `WhenNextEvent`). It would be rules-better for once-per-turn ("This ability triggers only once each turn" is one ability, CR 603.2h). But it fans out to every `mode` consumer (trigger index keys, printed/latched match loops, mana-activation boundary collection, coverage, analysis, AI). It is not needed for Unbound, which has no per-ability counter. See Known residuals.
  - *Rejected alternative:* a sibling `TriggerMode::SpellCastOrAbilityActivated`. That is the sibling-cluster smell (EntersOrAttacks/AttacksOrBlocks/SaddlesOrCrews/CycledOrDiscarded…), and it would conflate two `valid_card` subject kinds (the spell vs the ability source) in one filter.
- **U3 creates** a new effect. Justification is in §Variant Discoverability.
- **U4 extends** the existing profiler arms.
- **U5 extends** the existing departed-spell record pattern to activated-ability entries.

## Analogous Trace

Covered in Step 2 (T-A Magus, T-B compound split, T-C Effect::Double, T-D X channels), with the full file paths listed there.

## Variant Discoverability (`/add-engine-variant`)

`cargo engine-inventory` was **not run** (no-cargo constraint), and `data/engine-inventory.json` is absent. The five-grep protocol was run by hand over `types/`, `parser/` and `game/`.

**`Effect::ModifyAnnouncedX { target: TargetFilter, modification: XModification }`**
- **Stage 1:** DOES_NOT_EXIST. Greps for `chosen_x`/`cost_x_paid` writers find only finalize and `publish_announced_x`/`set_chosen_x_recursive`. No `Effect` variant writes X, and `DoubleTarget` has `Counters`, `LifeTotal` and `ManaPool` only.
- **Stage 2:** EXTEND_OK. There is no X-effect sibling cluster. Glava's "become 5" is the anticipated second value of `XModification`, so the parameter exists from day one instead of a later `SetX` sibling.
- **Stage 3:** **refuse `DoubleTarget::AnnouncedX`**. `DoubleTarget`'s axis is CR 701.10d/e/f, and the value of X is CR 107.3 (CR 107.3c: "the value of X may change while that spell or ability is on the stack"), so that would CROSS_SECTIONS. The dedicated effect's axis (`XModification`) lies within CR 107.3, so it is WITHIN_SECTION.
- **Effect registrations:** `effect_variant_name`, `EffectKind::ModifyAnnouncedX` with `From<&Effect>`, and the compiler-forced sites listed in Sizing.
- **ability_scan:** `Axes::CONSERVATIVE`; resolution choice defaults to `MayPrompt`.
- **ability_rw:** see U4.

**`XModification { Double }`** is a new enum (`#[serde(tag = "type")]`). Its CR annotation is "CR 107.3c + CR 701.10: the printed 'double the value of X'". CR 701.10 is cited only as the keyword the text uses; 701.10 has no X sub-rule, and the comment must say so.

**`GameState::departed_stack_abilities`** is a new field, not a variant. The serialized-surface audit:
- It is additive, with serde default and skip-if-empty.
- Add it to the manual `PartialEq` chain at the `departed_stack_spells == other.departed_stack_spells` site (HEAD `types/game_state.rs` ~32861). **That site is not compiler-forced.**
- Add it to every exhaustive `GameState` destructure (~32223/~32517, compiler-forced) and to the `Default`/`new` initializer (~29500).
- Repo-owned serialized states keep loading via serde default. Evidence: the existing `departed_stack_spells_round_trip…` test pattern, plus a sibling round-trip test for the new field.

Parser-local enums (`CostSlot`, `Agreement`, `EventCostSubject`) are not engine-surface variants.

---

## Identity / Provenance Contract

| Phrase | Authority type / id | Binding time | Live vs latched | Storage | Consumer | Invalidation | Multi-authority hostile fixture |
|---|---|---|---|---|---|---|---|
| "copy **that spell**" | `GameEvent::SpellCast.object_id` + incarnation pin (`ability.context.triggering_spell`) | trigger placement (CR 603.3) | live on-stack entry, else the `departed_stack_spells` record (CR 608.2h LKI) | existing | `copy_spell::copy_source_entry` → `targeting::triggering_spell` | step transition clear (`turns.rs:1303`), loop prune | X instant countered (Counterspell) before Unbound's trigger resolves → copy still created (ruling 1) |
| "copy **that ability**" | `GameEvent::AbilityActivated.stack_entry: Option<ObjectId>` (the stack object the activation created, CR 602.2a) | event emission | live entry, else **new** `departed_stack_abilities[entry_id]` (CR 113.7a + 608.2h) | new map | `copy_spell::triggering_activation` | cleared with `departed_stack_spells` (turns.rs), pruned in the loop-normalizer sibling of `retain_trigger_referenced_departed_spells` (keep entries a stacked/resolving `TriggeredAbility { trigger_event: Some(AbilityActivated { stack_entry: Some(id), .. }) }` names) | Same source has two activations (an X one and a non-X one) on the stack. Stifle the **X** one; the copy must be of the X ability (entry id), never the sibling sharing `source_id` |
| "**that spell's mana cost** … contains {X}" | spell object `mana_cost` (`cost_has_x`) | event match (`match_spell_cast` valid_card) | immutable after cast (CR 601.2f, 107.3a); see fold rationale | `valid_card` HasXInManaCost | `filter.rs` HasXInManaCost | n/a | X only in an **additional** cost → no trigger (ruling 8). X=0 → still triggers (ruling 6) |
| "**that ability's activation cost** contains {X}" | `(source_id, ability_index)` most recent in `pending_activations` → `ability_cost_has_x` (mana legs only) | event match (immediately after placement) | immutable after activation (CR 602.2b) | `valid_card` HasXInActivationCost | `casting_costs::pending_activation_cost_has_x` | `pending_activations` cleared on pass, which is why the predicate is **not** rechecked at resolution | Same source has `{X}: …` and `{1}: …`. Activating `{1}` → no copy; activating `{X}` → copy. "Pay X life" cost → no copy (ruling 3) |
| "double **the value of X**" | the triggering spell's stack entry, via `targeting::triggering_spell` = `OnStack(entry)` only (incarnation-pinned, CR 400.7) | resolution | live write | `obj.cost_x_paid`, `stack_paid_facts[id].x_value`, `Spell{ability: Some(a)}` → `a.set_chosen_x_recursive` | ETB counters (`stack.rs` resolving_spell_x), CostXPaid, Variable X, `mana_value_with_x` | `Departed`/`Gone` → no-op (EffectResolved only) | (i) Spell countered before the doubling resolves → graveyard card's `cost_x_paid` unchanged. (ii) Two Unbounds → ×4 (ruling 7) |

**Fold rationale (CR 603.4).** The intervening-if is lowered into each half's `valid_card` (event-match time), not a rechecked `TriggerCondition`, for these reasons:
1. Its predicate reads the announced mana cost or activation cost, which is fixed once the spell is cast (CR 601.2f) or the ability activated (CR 602.2b). CR 603.4's resolution recheck therefore has the same answer, and the "copy even if countered" ruling confirms the recheck passes via last-known information.
2. The ability leg's only existing evaluator (`pending_activation_cost_has_x`) reads `pending_activations`, which is cleared on PassPriority. A rechecked leaf would **always** fail at resolution and silently never copy an ability. That is the decisive hostile case.
3. It matches how the engine already models the equivalent "with {X} in its mana cost" grammar.

The disjunct is distributed by anaphor: "that ability's …" has no referent on a cast event, and "that spell's …" none on an activation event. The splitter **declines** (no split, text stays unparsed, coverage stays red) unless every listed subject maps to exactly one half and each half receives exactly one subject.

---

## Verification Matrix

Runtime rows follow `/card-test`: verbatim Oracle via `add_*_from_oracle`, `runner.cast(..).x(n).resolve()` / `runner.activate(src, i).x(n).resolve()`, and `CastOutcome` deltas. Every negative names its paired positive reach guard.

| # | Claim | Changed seam | Entry point | Test (file :: name) | Revert-failing assertion | Paired positive / sibling / hostile | First production branch |
|---|---|---|---|---|---|---|---|
| V1 | Line 1 filter carries HasXInManaCost | `parse_post_spell_modifier` via `parse_x_cost_slot_qualifier` | `parse_trigger_lines` | oracle_trigger_tests :: `cast_permanent_spell_with_mana_cost_that_contains_x_filters_x` | `valid_card` == And[Typed Permanent, Typed{HasXInManaCost}] | Siblings "with {X} in its mana cost" and "with an {X} in its mana cost" unchanged (Brass Infiniscope-class strings). Hostile "with a mana cost that contains {R}" → `None` (no prop) | `parse_spell_qualifier_payload` post arm |
| V2 | Activation slot grammar | `parse_post_activation_modifier` | direct | oracle_trigger_tests :: `activation_cost_that_contains_x_qualifier` | "with an activation cost that contains {x}" → HasXInActivationCost | Mana-slot text fed to the activation parser → `None` (slot mismatch) | combinator slot check |
| V3 | Line 2 → two triggers with folded filters | `split_cast_or_activate_compound` | `parse_trigger_lines` | oracle_trigger_tests :: `unbound_flourishing_copy_line_splits_cast_and_activate` | len == 2; [0] SpellCast valid_card And[Or[Instant,Sorcery], HasXInManaCost]; [1] AbilityActivated valid_card HasXInActivationCost, valid_target Controller; both execute CopySpell{retarget: MayChooseNewTargets}; **no** `TriggerCondition`; no Unimplemented anywhere | Hostile: subject mismatch ("if that spell's mana cost contains {X}" alone on a cast-or-activate line) → no split, honest Unimplemented. "an abilityx" non-match. Magus delayed text unchanged (its own test) | the new Pattern gate before Pattern 2 |
| V4 | Other "cast … or activate" cards change honestly | same | coverage | oracle_trigger_tests :: `cast_or_activate_split_surfaces_activation_leg` (Bill Potts, Glava, Neerdiv, Vazi verbatim) | each yields 2 triggers. The activation half is `AbilityActivated` **or** contains Unknown/Unimplemented (never silently absent) | `cargo coverage` delta reported for exactly these 4 plus Unbound; any card flipping supported → unsupported is listed as an honest correction | — |
| V5 | Line 1 effect parses | `try_parse_double_effect` arm | `parse_trigger_lines` | oracle_effect tests :: `double_the_value_of_x_in_spell_cast_trigger` | execute == ModifyAnnouncedX{target: TriggeringSource, modification: Double} | Outside a spell-cast trigger body (no `object_pronoun_ref`) → `Effect::unimplemented`. "double the value of y" → not this arm | `object_pronoun_ref` gate |
| V6 | Doubling feeds ETB counters, MV, no extra mana | `set_stack_object_announced_x` | `runner.cast(ballista).x(2)` | integration :: `unbound_doubles_x_for_walking_ballista` | Ballista enters with **4** +1/+1 counters. While on the stack after Unbound's trigger resolves, `spell_mana_value() == 8` and `stack_paid_facts.x_value == Some(4)`. Mana spent == 4 | **Reach guard:** the same scenario without Unbound gives 2 counters. Hostile: Ballista X=0 → 0 counters and the trigger still fires/resolves (ruling 6). Non-X permanent (Grizzly-bear-style vanilla) → no trigger on the stack (paired with the X case firing) | resolver `OnStack` arm |
| V7 | Two Flourishings → ×4 | same | same | integration :: `two_unbound_flourishings_quadruple_x` | 8 counters at X=2 | Single-Unbound row V6 as control | — |
| V8 | Doubling a countered spell is a no-op | resolver `Departed/Gone` arm | cast X permanent, Counterspell it above Unbound's trigger | integration :: `unbound_double_after_counter_is_noop` | graveyard card `cost_x_paid` stays the cast X; no panic; EffectResolved emitted | Positive: V6 | `triggering_spell` returns Departed |
| V9 | X instant copied with same X; copy resolves first | U2 + existing CopySpell | `runner.cast(stroke).x(2).target_player(P0)` (Stroke of Genius `{X}{2}{U}` "Target player draws X cards." verbatim) | integration :: `unbound_copies_x_instant` | P0 draws 4 in total (2 + 2). Copy on top of the original immediately after Unbound's trigger resolves | **Negative:** Opt-style non-X instant → no copy (draw only its own 1). Reach guard: V9's positive in the same file. X=0 Stroke → copy still created (two zero-draw resolutions; assert the copy entry existed) | `match_spell_cast` + HasXInManaCost |
| V10 | `{X}` activated ability copied with same X | U2 half B | `runner.activate(src, 0).x(2)` with ability `{X}: Draw X` built as in issue_1010 | integration :: `unbound_copies_x_activated_ability` | draws 4 | **Negative:** same source's `{1}: Draw a card` (index 1) → 1 card only. **Hostile:** `{X}` paid as "Pay X life" (`AbilityCost::PayLife` with X), if the scenario builder supports it; otherwise a `ability_cost_has_x` unit test → no copy (ruling 3) | `pending_activation_cost_has_x` |
| V11 | Copy even if original countered (spell) | existing Departed path | V9 + Counterspell targeting Stroke above Unbound's trigger | integration :: `unbound_copies_countered_x_instant` | copy still created and resolves: P0 draws 2 | Reach: V9 | `TriggeringSpell::Departed` |
| V12 | Copy even if original countered (ability) | **U5** | V10 + Stifle the activated entry above Unbound's trigger | integration :: `unbound_copies_stifled_x_ability` | P0 draws 2. On HEAD: 0 (Gone) | Hostile: second activation of the **same source** left on the stack; the copy must be of the stifled entry's ability (its X), not the sibling (use X=3 vs X=1 to discriminate) | `triggering_activation` departed fallback |
| V13 | Ordering is the player's choice (ruling 4) | **U4** `ability_rw` | Hydroid Krasis X=4 + Unbound; drive `WaitingFor::OrderTriggers` manually (the harness auto-answers it, so drive these steps by hand) | integration :: `unbound_and_krasis_cast_trigger_order_is_chosen` | `OrderTriggers` is presented. Order "double first" → +4 life, 4 cards; "Krasis first" → +2 life, 2 cards; Krasis enters with 8 counters in both | Control: Krasis alone → no order prompt (single trigger) | `profiles_conflict` same-event gate |
| V14 | Departed-ability record lifecycle | U5 | unit (game_state.rs) | `departed_stack_abilities_round_trip_and_prune` | serde round-trip; prune keeps the entry referenced by a stacked AbilityActivated trigger and drops unreferenced; turns-transition clear | — | `retain_…` sibling |
| V15 | Authority writes all three channels; abilities too | `set_stack_object_announced_x` | unit (stack.rs) | `announced_x_rewrite_moves_every_channel` | Spell entry: cost_x_paid, paid facts and ability chain all == new. ActivatedAbility entry: chain chosen_x == new. TriggeredAbility: unchanged | — | exhaustive `StackEntryKind` match |
| V16 | Triggered abilities paying {X} are not copied (ruling 2) | structural | — | proof from code: `AbilityActivated` is emitted only by `casting_targets::emit_ability_activated`; callers are activation paths (`casting.rs`, `casting_costs.rs`, `mana_abilities.rs`, `mana_sources.rs`, `triggers.rs:25148` — **the executor must read that last caller and confirm it is not a triggered-ability path**; CR 603.2a) | — | — | — |

**Coverage status impact:** Unbound goes unsupported → **supported** only when U1+U2+U3 have all landed. Bill Potts, Glava, Neerdiv and Vazi may flip supported → unsupported (honest; activation leg was silently dropped); this must be reported, not hidden. **Is any text accepted while its semantics are deferred?** Only one case: the mana-ability gap below, which is runtime and not parser. No Oracle clause is accepted with deferred semantics: every clause lowers to its full typed form or stays `Unimplemented`.

## Reference Readings

| Row | Reference it copies | Derived value (Oracle + CR) | Measured? |
|---|---|---|---|
| V9/V10 "copy has the same X" | Magus issue_1010 behavior (copy of Draw X=2 draws 2) | Ruling "The copy has the same value of X" + CR 707.10 (copies "the value of X") → copy draws X | Measured by the existing passing test `issue_1010_magus_lucea_kane.rs` (both legs); not re-run this round (no cargo). Agrees with derivation |
| V11 "copy of a countered spell" | existing `TriggeringSpell::Departed` behavior | Ruling 1 + CR 608.2h → copy created from last-known spell | **Not measured** (no cargo). Command: `cargo test -p phase-engine --test integration unbound_copies_countered_x_instant`. If it fails on the existing path, that is a pre-existing defect and is fixed in this plan's U5 phase (defective-reference route) |
| V12 "copy of a countered ability" | **none**: HEAD returns `Gone` (read from `copy_spell.rs::triggering_activation`) | Ruling 1; Rings of Brighthearth ruling ("resolve even if that ability is countered") + CR 113.7a + 608.2h → copy created | The HEAD reading **disagrees** with the derivation: a **pre-existing defect**, fixed by U5, with rows asserting the derived value |
| V6 "Ballista without Unbound gives 2 counters" | existing ETB X path | CR 107.3m → X counters | Not measured this round; it is the reach guard |

## Known residuals (labelled, not hidden)

1. **Mana abilities with `{X}`** (Wizard's Rockets' "{X}, Sacrifice …: Add X mana", the corpus's only one per `jq`).
   - `pending_activations` is not pushed for mana abilities, so `HasXInActivationCost` is false and the activation half never triggers. By the rules it would trigger, and its copy would then do nothing: a mana ability never uses the stack (CR 605.3b), and `copy_source_entry` already returns `Absent` for that case.
   - The observable difference is a no-op trigger that does not appear. **Deferred** with discriminating test `unbound_triggers_on_x_mana_ability_and_copies_nothing` (`DEFERRED(follow-up)`), unless review asks for it in scope.
2. **Once-per-turn across split halves.** This is pre-existing in the split convention: OncePerTurn is keyed per `TriggerDefinitionRef` (`triggers.rs` ~14381).
   - Bill Potts and Glava would get per-half counters once their halves parse; after U2 they parse to Unknown, so they ship nothing wrong. Unbound has no constraint.
   - Follow-up: a single-definition disjunctive trigger (`or_trigger`-style field), which is also the parameterization target for the `EntersOrAttacks`-family siblings.
3. **Departed-LKI source** (an `{X}, Sacrifice ~: …` ability). `activated_source_matches` takes the `matches_target_filter_on_departed_battlefield_lki` path, and **whether** `HasXInActivationCost` answers there is **[UNPROBED]**.
   - The executor probes it: an `{X}, Sacrifice this artifact: … X …` fixture.
   - If false, fix it in U2's phase by evaluating `HasXInActivationCost` from `pending_activations` by object id in the LKI arm. This is a compiler-visible, local change.

## Probes owed (executor, first, with isolated `CARGO_TARGET_DIR`)

| Probe | Assumption it checks | Design decision it can change |
|---|---|---|
| P1 | `parse_trigger_lines` on the reconstructed half "Whenever you activate an ability, copy that spell or ability. You may choose new targets for the copy." | That it yields `AbilityActivated` + `CopySpell{MayChooseNewTargets}` with a context-ref target accepted by `copy_source_entry` (TriggeringSource or ParentTarget). If not, U2 adds the anaphor lift (mirroring `lift_parent_target_to_triggering_source`) |
| P2 | Departed-LKI `HasXInActivationCost` | residual 3 |
| P3 | The Hydroid Krasis + Unbound pair | That it is auto-ordered on HEAD+U3 (expected, since `Variable` reads `RwProfile::empty()`), which confirms U4 is required. Also whether the group gate unions profiles (`reads_and_writes_event_object` over the group) or needs a pairwise feed. That sets the exact profiler edit |
| P4 | Stifle can target an activated-ability stack entry via `runner.cast(stifle).target_object(entry_id)` | if not, the V12 driver must issue `GameAction::CastSpell`/`SelectTargets` manually |
| P5 | Whether `triggers.rs:25148`'s `emit_ability_activated` caller is a triggered-ability path | V16 |

---

## Step 5 — Step-by-step implementation

CR numbers below were all verified by `grep -n "^<rule>" docs/MagicCompRules.txt` this round: 107.3a, 107.3c, 107.3e, 107.3m, 202.3e, 400.7, 601.2f, 601.2i, 602.2a, 602.2b, 603.2a, 603.3b, 603.4, 605.3b, 608.2h, 701.6a, 701.10, 707.10, 707.10c, 113.7a.

### Phase A — U1 + U2 (parser): `crates/engine/src/parser/oracle_trigger.rs`, `oracle_trigger_tests.rs`, integration test

1. **U1 combinator.** Add `enum CostSlot { Mana, Activation }` with `fn x_filter_prop(self) -> FilterProp` (Mana → HasXInManaCost, Activation → HasXInActivationCost; `// CR 107.3a + CR 601.2f / CR 602.2b`). Add `fn parse_x_cost_slot_qualifier(input) -> OracleResult<'_, CostSlot>` with the grammar in §Nom Compliance.
   - Replace the X arm of `parse_post_spell_modifier` with: `if let Ok((rest, CostSlot::Mana)) = parse_x_cost_slot_qualifier(modifier) { if rest.trim().is_empty() { return Some(Typed(default().properties(vec![HasXInManaCost]))) } }`.
   - Do the same in `parse_post_activation_modifier` with `CostSlot::Activation`.
   - Update both doc comments.
2. **U2 splitter.** Add `fn split_cast_or_activate_compound(condition: TextPair, effect: TextPair) -> Option<CastOrActivateSplit>`. `CastOrActivateSplit { spell_condition: String, ability_condition: String, effect: String, spell_cost: Option<CostSlot>, ability_cost: Option<CostSlot> }`, with `spell_cost`/`ability_cost` set only by the fold.
   - Grammar: §Nom Compliance.
   - Half texts:
     - `spell_condition = "{keyword}{actor}{cast_verb}{spell_payload}"`, using original-case slices of `condition`.
     - `ability_condition = "{keyword}{actor}{activate_verb} an ability{ability_tail}"`.
   - **Fold:** run the intervening-if combinator on the effect's lowercase text.
     - On success, map each `EventCostSubject` to its half (Spell → `spell_cost = Some(CostSlot::Mana)`, Ability → `ability_cost = Some(CostSlot::Activation)`). If any half receives zero or more than one subject, or a subject repeats, return `None` (decline: honest red).
     - On success, `effect` is the remainder after ", " (original case via `nom_on_lower`). With no intervening-if, `effect` is unchanged and both costs are `None`.
   - Doc: `CR 603.2` (one event per arm) + `CR 603.4` (fold rationale paragraph from §Identity, condensed) + `CR 602.2b`.
3. **Hook.** In `parse_trigger_lines_at_index_ir`, after Pattern 1 and the zone-change gate and **before** Pattern 2:
   ```
   if let Some(split) = split_cast_or_activate_compound(TextPair(condition), TextPair(effect)) {
       for (i, (cond, cost)) in [(split.spell_condition, split.spell_cost), (split.ability_condition, split.ability_cost)].into_iter().enumerate() {
           let text = if split.effect.is_empty() { cond } else { format!("{cond}, {}", split.effect) };
           let mut ir = parse_trigger_line_with_index_ir(&text, card_name, base_trigger_index.map(|b| b.offset(i)), ctx);
           if let Some(slot) = cost { and_valid_card_prop(&mut ir.partial_def, slot.x_filter_prop()); }
           results.push(ir);
       }
       return results;
   }
   ```
   `and_valid_card_prop` is a small private helper: `None` → `Some(Typed(default().properties([prop])))`, `Some(f)` → `Some(And{[f, Typed(..[prop])]})`. **Confirm `lower_trigger_ir` copies `partial_def.valid_card` verbatim** (read `lower_trigger_ir` at HEAD ~2052 before relying on it). If lowering rebuilds `valid_card`, apply the fold to the lowered `TriggerDefinition` instead, by returning lowered definitions from this arm the way `parse_trigger_lines_at_index` does.
4. **Tests.**
   - Parser tests V1–V4 in `oracle_trigger_tests.rs`.
   - Integration `crates/engine/tests/integration/unbound_flourishing.rs` with rows V9, V10, V11 (V11 lands here if the existing Departed path passes; otherwise it moves to Phase B with a fix), plus V16's proof as a comment test where feasible.
   - Add `mod unbound_flourishing;` to `tests/integration/main.rs`.
   - Use the verbatim `UNBOUND_ORACLE` constant (both lines).

### Phase B — U5 (departed activated ability)

5. `types/game_state.rs`:
   - Add `pub departed_stack_abilities: im::HashMap<ObjectId, StackEntry>` with doc `CR 113.7a + CR 608.2h + CR 707.10: an activated ability's stack entry as it last existed on the stack, keyed by its stack-entry id; consulted by copy_spell::triggering_activation once a "copy that ability" trigger's ability has left the stack`.
   - Use the serde attrs/serializer used by `departed_stack_spells`.
   - Add it to init, the manual `PartialEq` chain, and every exhaustive destructure.
   - Add `retain_trigger_referenced_departed_abilities` and call it wherever `retain_trigger_referenced_departed_spells` is called. It keeps keys named by a stacked/resolving `TriggeredAbility{ trigger_event: Some(GameEvent::AbilityActivated{ stack_entry: Some(id), .. }) }`.
6. `game/stack.rs`: add `pub(crate) fn record_departed_stack_ability(state, entry)`. It records only `StackEntryKind::ActivatedAbility` (exhaustive match; others are no-ops) and is called from `remove_nonresolving_stack_entry_at` beside `record_departed_stack_spell` (CR 701.6a: countered = removed without resolving). Doc cites CR 113.7a.
7. `game/turns.rs`: clear the new map beside `departed_stack_spells.clear()` (~1303).
8. `game/effects/copy_spell.rs::triggering_activation`: when the entry is not on the stack, look it up in `state.departed_stack_abilities` → `CopySourceLookup::Source(Box::new(CopySource { entry, departed_object: None }))`, else `Gone`. Update the doc (CR 113.7a + 608.2h).
9. `game/visibility.rs`: only if a `GameState` destructure or projection is exhaustive. Activated-ability entries are public, so the projection copies the map unchanged.
10. Tests: V12 (integration) and V14 (unit). Add a Rings of Brighthearth regression (pay {2}, Stifle the ability, copy still resolves) to show class benefit. Its ruling is verified above.

### Phase C — U3 (doubling effect)

11. `types/ability.rs`:
    - `Effect::ModifyAnnouncedX { #[serde(default = "default_target_filter_any")] target: TargetFilter, modification: XModification }`, with doc `CR 107.3c: an effect may change the value of X while the spell or ability is on the stack ("double the value of X" — Unbound Flourishing). Writes every X channel through stack::set_stack_object_announced_x.`
    - `pub enum XModification { Double }` with `pub fn apply(self, x: u32) -> u32 { match self { Self::Double => x.saturating_mul(2) } }`. Annotate `// CR 107.3c + CR 701.10: the printed verb "double"; CR 701.10 has no X sub-rule.`
    - Add `effect_variant_name`, `EffectKind::ModifyAnnouncedX`, and the `From<&Effect>` arm.
12. `game/stack.rs`: add `pub(crate) fn set_stack_object_announced_x(state: &mut GameState, entry_id: ObjectId, x: u32)` with doc `CR 107.3a + CR 107.3c + CR 107.3m + CR 202.3e: the single authority for rewriting the announced X of a spell or activated ability on the stack`.
    - Find the stack entry by id.
    - Exhaustive match on `kind`:
      - `Spell { ability, .. }`: if `ability` is `Some(a)`, call `a.set_chosen_x_recursive(x)`. Then set `state.objects[entry_id].cost_x_paid = Some(x)`, and if `state.stack_paid_facts[entry_id]` exists, set its `x_value = Some(x)`.
      - `ActivatedAbility { ability, .. }`: `ability.set_chosen_x_recursive(x)` (CR 107.3k: the ability's own X; never `cost_x_paid`).
      - `TriggeredAbility | KeywordAction | CombatDamage`: no-op (CR 107.3a covers spells and activated abilities).
    - Check `cr733_resolved_entry_retags.rs` and the stack-entry mutation census for whether an in-place `&mut entry.kind` edit must be registered. The `ChangeTargets` effect is precedent for an unjournaled in-place stack edit. If a census pins write sites, register this one with its CR.
13. `game/effects/modify_announced_x.rs` (new): `pub fn resolve(state, ability, events)`.
    - Destructure `Effect::ModifyAnnouncedX { target, modification }`.
    - Resolve the stack object: `TargetFilter::TriggeringSource` → `targeting::triggering_spell(state)`. `OnStack(entry)` → `entry.id`; `Departed`/`Gone`/`None` → no object.
    - Any other target uses `super::resolved_effect_object_ids(state, ability, target)` filtered to ids whose object is in `Zone::Stack` with a `StackEntryKind::Spell` entry.
    - For each object, read the current X (`obj.cost_x_paid`, falling back to the entry ability's `chosen_x`, falling back to 0), compute `modification.apply(current)`, and call `set_stack_object_announced_x`.
    - Always push `GameEvent::EffectResolved { kind: EffectKind::ModifyAnnouncedX, source_id, subject: None }`.
    - Add `mod modify_announced_x;` and the dispatch arm in `game/effects/mod.rs`.
14. **Compiler-forced arms.** Mirror the `Effect::DoublePTAll`/`Double` arm in each file listed in Sizing:
    - `ability_scan.rs`: CONSERVATIVE; choice freedom `MayPrompt`.
    - `ability_visit.rs`: visit `target`.
    - `coverage.rs`: supported.
    - `resolution_prompt.rs`: non-prompting group.
    - `oracle_ir/ast.rs` and `doc.rs`: doc text "double the value of X".
    - `analysis/ability_graph.rs`.
    - `phase-ai effect_classify.rs`: `EffectPolarity::Beneficial`; target arm like `DoublePTAll`.
    - `redundancy_avoidance.rs`.
    - `ability_rw.rs`: `Effect::ModifyAnnouncedX { target, modification: _ } => obj(StateKind::StackShape, target)`, so `TriggeringSource` scopes to the event object. Executor: confirm `scope_of(TriggeringSource, ..) == WriteScope::EventObject`; if not, place the write explicitly with `place_object_write(.., WriteScope::EventObject)`.
15. `parser/oracle_effect/counter.rs::try_parse_double_effect`: insert before the generic `tag("double ")` arm:
    `if nom_on_lower(lower, lower, |i| value((), (tag("double the value of x"), eof)).parse(i)).is_some() { return Some(match ctx.object_pronoun_ref { Some(TargetFilter::TriggeringSource) => Effect::ModifyAnnouncedX { target: TargetFilter::TriggeringSource, modification: XModification::Double }, _ => Effect::unimplemented("double_value_of_x_without_event_spell", lower) }); }`
    Annotate `// CR 107.3e + CR 107.3c: in a spell-cast trigger body "the value of X" names the triggering spell's X`. **Executor:** confirm `ctx` at this dispatch carries the trigger body's `object_pronoun_ref` (the `body_context` seed). If it does not, thread it via the existing seed rather than inventing a field.
16. Tests: V5 (parser), V6, V7, V8 (integration), V15 (unit in stack.rs).

### Phase D — U4 (ordering honesty)

17. `game/ability_rw.rs`: split `QuantityRef::Variable { name: _ } | QuantityRef::SelfManaValue => RwProfile::empty()` into:
    - `QuantityRef::Variable { name } if name == "X"` → a profile with `reads_event_live = true`. Doc: `CR 107.3e + CR 603.3b: a bare X with no own chosen_x reads the triggering object's X at resolution (quantity.rs Variable-X fallback), so a same-event writer of that X must order-conflict`.
    - The remaining `Variable`/`SelfManaValue` arms stay empty.
    - Confirm that U3's arm sets `writes_event_object` (P3 decides whether the group gate needs anything beyond this).
18. Run the full-DB parity sweep, `FORGE_TEST_FULL_DB=1 cargo test -p phase-engine ordering_parity_sweep`, and read the delta. If the completeness-asserted `SAME_EVENT_EVENT_OBJECT_GENUINE` fails for Unbound-class pairs, add them with CR 603.3b evidence (ruling 4). Never bump a floor.
19. Test: V13.

### Every phase

- Run `cargo fmt --all`.
- Use the Tilt resources `clippy`, `test-engine` and `card-data` (if Tilt is down, use direct cargo with an isolated target dir).
- Run `cargo coverage` and report the Unbound / Bill Potts / Glava / Neerdiv / Vazi rows.
- Run `cargo semantic-audit` on those cards.
- Grep-verify every CR annotation.
- Serialize behind the concurrent executor editing `oracle_trigger.rs` / `condition.rs`, and rebase onto its commit first. Its hunks (count-quality condition) are in different regions than §Step 5.1–3.
