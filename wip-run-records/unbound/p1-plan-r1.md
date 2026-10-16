# Phase 1 plan r1: X-cost qualifier grammar and the cast-or-activate split (run `unbound`)

**Mode.** engine-planner, phase-plan mode. Phase 1 of the frozen charter `.git/engine-implementer-runs/unbound/phase-charter` (r1 with corrections C1–C8).

**Base.** `PHASE_BASE_SHA = BASE_SHA = b4685f2e9c7cfae0eeb00a8c39fd1c59791a4de3` (branch `card/unbound-flourishing`). The base includes the Zimone work (protocol 144 / wire 126). This phase touches no version surface.

**How this plan was made.**
- I read the code at the base and measured with `jq` over `client/public/card-data.json`, `client/public/coverage-data.json` and `data/mtgjson/AtomicCards.json`, all generated at b4685f2.
- The spawn constraint ruled out cargo, so **I ran no probes**. Each runtime claim below is tagged:
  - **[read]**: read from source.
  - **[measured]**: from jq over the generated data.
  - **[UNPROBED]**: neither. The executor must turn each one into a measurement (see §Probes owed).
- Charter decisions are applied as written. I re-litigate none of them.

---

## Step 0: Premise verification (hard gate)

**Unbound Flourishing.** Local MTGJSON (`jq '.data["Unbound Flourishing"][0]'`) matches `/mnt/project-files/unbound-zimone/oracle-verified.md` word for word:

> {2}{G} Enchantment
> Whenever you cast a permanent spell with a mana cost that contains {X}, double the value of X.
> Whenever you cast an instant or sorcery spell or activate an ability, if that spell's mana cost or that ability's activation cost contains {X}, copy that spell or ability. You may choose new targets for the copy.

All 14 rulings were read. This phase relies on these:
- **1:** the copy is made even if the original was countered, and the copy resolves first.
- **2:** triggered abilities that pay {X} are not copied.
- **3:** an X that is not the {X} mana symbol is not copied.
- **5:** a modal copy keeps its modes.
- **6:** X = 0 is still copied.
- **7:** two Unbounds copy twice.
- **8:** an {X} only in an additional cost does not count.
- **9:** the copy has the same X.
- **10:** the copy is not cast or activated.
- **11:** new targets are optional.
- **13:** any {X} instant, sorcery or activated ability is copied, not only targeted ones.

**Class cards.** Their verbatim lines were re-read from MTGJSON with a corpus jq over trigger lines matching `^(whenever|when) …cast… or activate…`. These are every printed trigger line of that shape:

| Card | Line |
|---|---|
| Bill Potts | "Whenever you cast an instant or sorcery spell that targets only Bill Potts or activate an ability that targets only Bill Potts, copy that spell or ability. You may choose new targets for the copy. This ability triggers only once each turn." |
| Glava, Five-Advents Mage | "Whenever you cast a permanent spell with {X} in its mana cost or activate an ability with {X} in its activation cost that isn't a mana ability, you may have the value of X become 5. Do this only once each turn." |
| Neerdiv, Devious Diver | "Whenever you cast a spell from your graveyard or activate an ability of a card in your graveyard, draw a card and put a +1/+1 counter on Neerdiv." |
| Vazi, Keen Negotiator | "Whenever an opponent casts a spell or activates an ability, if mana from a Treasure was spent to cast it or activate it, put a +1/+1 counter on target creature, then draw a card." |
| Unbound Flourishing | line 2, quoted above |

Non-candidates that share words but not the shape (checked so the detector does not claim them):
- Magus Lucea Kane: "When you **next** cast …", inside an effect.
- Sunken Palace: "When you **spend** this mana to cast …", inside an effect.
- Laboratory Drudge: "At the beginning of …".
- Repeated Reverberation: "When you next cast …", inside an effect.

**Test fixtures**, verbatim from MTGJSON:

| Card | Cost and type | Oracle text |
|---|---|---|
| Prosperity | {X}{U} Sorcery | "Each player draws X cards." |
| Stroke of Genius | {X}{2}{U} Instant | "Target player draws X cards." |
| Divination | {2}{U} Sorcery | "Draw two cards." |
| Counterspell | {U}{U} | "Counter target spell." |
| Walking Ballista | {X}{X} 0/0 | "This creature enters with X +1/+1 counters on it. / {4}: Put a +1/+1 counter on this creature. / Remove a +1/+1 counter from this creature: It deals 1 damage to any target." |
| Oracle of Nectars | {2}{G/W} 2/2 | "{X}, {T}: You gain X life." |
| Fountain of Youth | {0} Artifact | "{2}, {T}: You gain 1 life." |
| Snake Basket | {4} Artifact | "{X}, Sacrifice this artifact: Create X 1/1 green Snake creature tokens. Activate only as a sorcery." |
| Crashing Wave | {U}{U} Sorcery | "As an additional cost to cast this spell, waterbend {X}. (…) / Tap up to X target creatures, then distribute three stun counters among any number of tapped creatures your opponents control. (…)" |

Magus Lucea Kane's text is the `MAGUS_ORACLE` constant in `issue_1010_magus_lucea_kane.rs`, which matches MTGJSON.

**Current parse [measured], `jq '."<name>".triggers'`.**
- **Unbound line 1:** `SpellCast`, valid_card `Typed[Permanent]` with **no** `HasXInManaCost`, effect `Unimplemented("unparsed_quantity", "double the value of X")`.
  - Root cause [read]: `parse_spell_qualifier_payload` splits off "with a mana cost that contains {x}". `parse_post_spell_modifier` does not know that string, so the function returns `None`. The `parse_type_phrase_folding` fallback then discards `_rest` (`oracle_trigger.rs`, the "you cast a/an" arm of the SpellCast condition parser).
- **Unbound line 2:** one `SpellCast`, valid_card `Or[Instant, Sorcery]`. The activation leg is silently dropped. The effect is `Unimplemented("unparsed_condition", "if that spell's mana cost or …")`.
- **Bill Potts:** one `SpellCast`, `Or[Instant+TargetsOnly(SelfRef), Sorcery+TargetsOnly(SelfRef)]`, constraint `OncePerTurn`. The activation leg is dropped. Coverage: **supported**, a silent drop.
- **Glava:** one `SpellCast Typed[Permanent]`, `OncePerTurn`, effect Unimplemented. Coverage: unsupported.
- **Neerdiv:** trigger[1] is `SpellCast`, `spell_cast_origin Equals(Graveyard)`, Draw + PutCounter. The activation leg is dropped. Coverage: **supported**, a silent drop.
- **Vazi:** `SpellCast` with an Opponent valid_target and `condition: null`. The Treasure condition is dropped, which is pre-existing; C8 routes it to triage. The effect has a `put_counter_tail` gap. Coverage: unsupported.

## Step 1: Applicable skills (checklist skeleton)

| Skill | Use in phase 1 |
|---|---|
| `/oracle-parser` | Checklist 9a, Rule Zero (nom first), and the self-review checklist for U1 and U2. |
| `/add-trigger` | Phase 5 (parser) and Phase 8 (tests) for U2's split halves. Phase 3 (matcher) for U3's `activated_source_matches` fix. |
| `/card-test` | Every runtime row: GameScenario + GameRunner, verbatim Oracle text, `CastOutcome` deltas, paired reach guards. |

**`/oracle-parser` §9a, step by step:**

| Step | Plan section |
|---|---|
| Phase 1, where it belongs | `oracle_trigger.rs`, the trigger owner. |
| Phase 2, test first and nom first | Step 5 orders the tests first. §Nom Compliance gives the combinators. |
| Phase 3, the subject | The actor phrase is carried into both halves; agreement is handled by the verb. |
| Phase 4, chain composition | n/a. The effect body is reused verbatim per half. |
| Phase 5, routing | n/a. Both lines already route to the trigger parser. |
| Phase 6, tests and verification | §Verification Matrix and §Step 5.7. |
| Snapshot tests | No `oracle_ir/snapshots` file contains any class card (grep over `crates/engine/src/parser` for the five card names found none). If a snapshot moves, regenerate it and list it in the commit. |
| `cargo coverage` | §Coverage impact. |

**`/add-trigger` checklist, step by step:**

| Phase | Status |
|---|---|
| 1, types | No new `TriggerMode`, `TriggerCondition` or `TriggerConstraint`. |
| 2, events | Existing `SpellCast` and `AbilityActivated`. |
| 3, matcher | U3 binds the activation-cost fact in `activated_source_matches`; registry unchanged. |
| 4, targets | n/a. `CopySpell{TriggeringSource}` takes no stack-time target. |
| 5, parser | U1 and U2. |
| 6, conditions | n/a. There is no new condition. The fold produces no `TriggerCondition`, so there is no resolution recheck (decision 3). LKI: U3 is exactly this item, for a cost-departed source. |
| 7, resolution | n/a. |
| 8, tests | Parser rows, integration rows, and the constraint row (P1-T6: a decline instead of a once-per-turn split). APNAP: n/a, since all triggers belong to one controller. |

**Later-phase checklist items:**
- `/casting-stack-conditions` X-channel work: **DEFERRED(phase 3)**.
- `/add-engine-effect` for "double the value of X": **DEFERRED(phase 3)**.
- `/add-engine-variant` field audit (departed map): **DEFERRED(phase 2)**.
- The trigger-ordering profile: **DEFERRED(phase 4)**.

## Step 2: Analogous trace (hard gate)

**T-A: Magus Lucea Kane, the delayed "next {X} spell or ability" copy.** Same two filters, same copy effect. Trace:
1. `parser/oracle_effect/mod.rs::try_parse_when_next_spell_or_activate_disjunction`, using `take_until(" or activate an ability ")`.
2. `extract_when_next_spell_filter`, which calls `oracle_trigger::parse_post_spell_modifier`, and `oracle_trigger::parse_post_activation_modifier`.
3. `build_when_next_delayed_trigger(SpellCast, …, Some((AbilityActivated, …)))`.
4. `types/ability.rs DelayedTriggerCondition::WhenNextEvent{trigger, or_trigger}`.
5. `game/triggers.rs` WhenNextEvent arm, then `trigger_matchers::match_spell_cast` / `match_ability_activated`, then `activated_source_matches`.
6. `game/filter.rs` `FilterProp::HasXInActivationCost`, then `casting_costs::pending_activation_cost_has_x`. That function reads `state.pending_activations`, which is pushed in `casting.rs::record_activated_ability_placed`.
7. `game/effects/copy_spell.rs::copy_source_entry`. The `TriggeringSource` arm goes to `targeting::triggering_spell` (`OnStack` / `Departed` / `Gone`), else to `triggering_activation`.
8. The copy is pushed and the retarget walk opens.

Test: `crates/engine/tests/integration/issue_1010_magus_lucea_kane.rs`.

**T-B: printed compound-trigger splitting.** Trace:
1. `oracle_trigger.rs::parse_trigger_lines_at_index_ir` calls `split_trigger` (condition, effect).
2. Pattern 1, `split_and_when_compound`.
3. The zone-change gate, `parse_disjunctive_zone_change_condition`.
4. Pattern 2, `split_shared_subject_event_list` (`split_serial_event_compound`, `split_cross_subject_event_compound`, `split_or_event_compound`).
5. Each half becomes `format!("{cond}, {effect}")`, then goes to `parse_trigger_line_with_index_ir(.., base_trigger_index.map(|b| b.offset(i)), ctx)`, then `lower_trigger_ir`, whose `let mut def = ir.partial_def.clone()` copies `valid_card` from the IR [read].

The single-event halves land in:
- the SpellCast "you cast a/an" arm (`parse_spell_qualifier_payload`);
- `try_parse_ability_activation_trigger::parse_line`, with `you activate ` giving `valid_target Controller` and an optional "of <source type>" filter.

`split_serial_event_compound` sets the case-safe rule this plan follows: split the original and lowercase texts in lockstep on the same ASCII delimiters.

**T-C, the LKI branch of the activation matcher (U3).** Trace:
1. `casting_targets::emit_ability_activated` sets `departed_source_lki` when the source was announced from the battlefield and has since left it.
2. `trigger_matchers::match_ability_activated` calls `activated_source_matches`.
3. In its `Some(lki)` arm, `filter::matches_target_filter_on_departed_battlefield_lki` calls `matches_target_filter_on_lki_snapshot_with_incarnation`, which synthesizes a `ZoneChangeRecord`.
4. That reaches `zone_change_record_matches_property`, whose `FilterProp::HasXInActivationCost` arm fails closed ("X-in-cost is a spell-cast-time predicate … Fail closed") [read].

## Step 3: Files read (at the base)

- `parser/oracle_trigger.rs`:
  - `parse_trigger_lines_at_index_ir`, `parse_trigger_line_with_index_ir`, `lower_trigger_ir`.
  - `split_and_when_compound`, `split_shared_subject_event_list`, `split_cross_subject_event_compound`, `split_serial_event_compound`, `split_or_event_compound`.
  - `parse_trigger_keyword_prefix`, `parse_trigger_actor`, `try_parse_keyword_activation_trigger`, `try_parse_ability_activation_trigger`.
  - The SpellCast "you cast" arm, `parse_spell_qualifier_payload`, `parse_post_spell_modifier`, `parse_post_activation_modifier`.
  - `add_spell_prop`, `add_another_prop`, `parse_trigger_constraint`, `strip_first_time_each_turn_qualifier`, `make_base`, `unknown_trigger_definition`, `try_parse_nth_spell_trigger` (its `Rejected → TriggerMode::Unknown` precedent).
- `parser/oracle_trigger_tests.rs`: conventions and existing `parse_post_spell_modifier_*` tests.
- `parser/oracle_ir/trigger.rs`: `TriggerIr`.
- `parser/oracle_effect/mod.rs`: the Magus disjunction helper.
- `parser/oracle_effect/imperative.rs`: the "copy that spell or ability" branch, which always yields `TargetFilter::TriggeringSource`.
- `parser/oracle_static/grammar.rs`: the third caller of `parse_post_spell_modifier`.
- `parser/oracle_util.rs`: `TextPair`.
- `parser/oracle_nom/bridge.rs`: `nom_on_lower`.
- `game/trigger_matchers.rs`: `match_ability_activated`, `activated_source_matches`, `bind_triggering_player_controller`.
- `game/filter.rs`: the `HasXInActivationCost` arms (live object, spell record, zone-change record), `matches_target_filter_on_departed_battlefield_lki`, `zone_change_filter_inner`.
- `game/casting_costs.rs`: `ability_cost_has_x`, `pending_activation_cost_has_x`.
- `game/casting.rs`: `record_activated_ability_placed`.
- `game/casting_targets.rs`: `emit_ability_activated`.
- `game/effects/copy_spell.rs`: `copy_source_entry`, `triggering_activation`, `copy_source_from_triggering_spell`.
- `game/targeting.rs`: `triggering_spell`.
- `game/triggers.rs`: the `emit_ability_activated` call near the BecomeMonarch fixture, which is test code.
- `types/events.rs`: `GameEvent::AbilityActivated`.
- `types/game_state.rs`: `AbilityActivationRecord`, `LKISnapshot`, `pending_activations`, `WaitingFor::CopyRetarget`.
- `types/ability.rs`: `AbilityCost`, `TriggerConstraint` variants.
- `game/scenario.rs`: the cast and activate builders, `commit`, `x`, `target_object`, `add_*_from_oracle`.
- Integration tests: `issue_1010_magus_lucea_kane.rs`, `issue_6877_departed_triggering_spell.rs`, `copy_that_ability_referent.rs`, `attached_to_declared_target.rs` (driving `CopyRetarget`), `main.rs` (mod ordering).

## Step 3.5: Probes

None were run (spawn constraint). Each design-relevant uncertainty is listed in §Probes owed, with the decision it could change. The executor runs them first, with an isolated `CARGO_TARGET_DIR` or via Tilt logs.

---

## Pattern Coverage

Assessed against the charter's class attribution (phase-plan mode).

| Unit | Class | Cards |
|---|---|---|
| P1-U1: shared cost-X qualifier combinator | "with [a(n)] {X} in its mana/activation cost" ∪ "with a(n) mana/activation cost that contains {X}" | Unbound line 1 changes (the over-fire is fixed). Every existing user of the two post-modifiers is re-pointed with no output change: Brass Infiniscope class, Magus, the `oracle_static/grammar.rs` cost-reduction users, and the nth-spell "first spell with {X}" users. The "cost that contains {X}" grammar has exactly one printed user (corpus jq). The building block is the slot-parameterized qualifier, not the card. |
| P1-U2: cast-or-activate compound splitter with the cost-X intervening-if fold and two honest declines | every printed "<actor> cast(s) <spell> or activate(s) an ability<tail>" trigger line | 5 cards (the Step 0 table). Unbound gains a real activation leg. Neerdiv's leg becomes visible. Bill Potts, Glava and Vazi go honestly red (decisions 4 and 5). |
| P1-U3: activation-cost fact bound from the activation, not the source's LKI (draft residual 3) | every `AbilityActivated` trigger whose `valid_card` carries `HasXInActivationCost` (Unbound's leg, Magus's `or_trigger`), times every {X} ability whose cost moves its source | 17 corpus lines match `^{X}…Sacrifice…:` (jq count). Examples: Pernicious Deed, Snake Basket, Hearth Kami, Treasure Vault, Cinder Elemental, Orcish Settlers. |

## Sizing (phase-plan mode)

**Units.**

| # | Unit | Registration surfaces | Discriminating test |
|---|---|---|---|
| P1-U1 | `CostSlot` plus `parse_x_cost_slot_qualifier`, delegated to by `parse_post_spell_modifier` and `parse_post_activation_modifier` | `parser/oracle_trigger.rs` | P1-T3: Unbound line 1's `valid_card` carries `HasXInManaCost`. Reverting U1 drops it. I1 is the runtime row. |
| P1-U2 | `split_cast_or_activate_compound` (NotCandidate / Split / Declined), the fold, and the hook before Pattern 2 | `parser/oracle_trigger.rs` | P1-T4: line 2 yields exactly 2 definitions. I2 and I8: an {X} sorcery and an {X} activated ability are each copied. |
| P1-U3 | `bind_activation_cost_facts` in `activated_source_matches` | `game/trigger_matchers.rs` | I9 and I10: Snake Basket X=2 with Unbound, or with Magus, makes 4 Snakes. On the unfixed tree it makes 2. |

**Dependency edges.**
- U1 → U2: the fold shares U1's `parse_cost_slot`.
- U2 → U3 for the Unbound row I9.
- U3's other row, I10 (Magus), does not depend on U2, so U3 is discriminating-tested on its own.

**Expected scope paths** (phase-fit counting rule). Excluded: regenerated pipeline data (`client/public/card-data.json`, `data/card-data.json`, coverage data, none of them committed) and test fixtures (none touched).

*Authored product:*
1. `crates/engine/src/parser/oracle_trigger.rs`
2. `crates/engine/src/game/trigger_matchers.rs`. This is the charter's conditional anchor for draft residual 3. §Constraints shows the measurement-by-read that requires it, so the orchestrator writes the `addenda/phase-1` line at scope-freeze. `filter.rs` is **not** needed.

*Authored tests:*

3. `crates/engine/src/parser/oracle_trigger_tests.rs`
4. `crates/engine/tests/integration/unbound_flourishing.rs` (new)

*Standing registration class:*

5. `crates/engine/tests/integration/main.rs` (`mod unbound_flourishing;`, between `ultimate_spider_man_mass_counter_double` and `unholy_citadel_legendary_color_banding_grant`)

*Compiler-forced paths:* none expected. No public type or signature changes, and all new enums are file-private.

**Total: 5 paths (4 without the registration file), 3 units.** T1 is met and T2 (13 or more) is not, so the conjunction does not fire.

## Building Blocks (composed, by name)

**Parser:**
- `parse_trigger_keyword_prefix`: keyword capture, "Whenever " / "When ".
- `parse_trigger_actor`: you / an opponent / a player / another player.
- `oracle_nom::primitives::split_once_on`: lockstep ASCII splits, as in `split_serial_event_compound`.
- `nom::bytes::complete::take_until` / `tag`, `nom::combinator::{eof, peek, opt, value}`, `nom::multi::separated_list1`, `nom::sequence::{preceded, terminated, delimited}`.
- `oracle_nom::bridge::nom_on_lower`: maps the fold's lowercase remainder back to the original-case effect text.
- `parse_trigger_constraint`, reused to detect frequency riders.
- `strip_first_time_each_turn_qualifier`, reused to detect the "for the first time each turn" rider.
- `parse_trigger_line_with_index_ir`, `PrintedTriggerIndex::offset`.
- `add_spell_prop`, reused to attach the folded prop. It distributes into `Or`/`And` and wraps others; only its doc changes to say it also serves activation-source filters.

**Runtime:**
- `casting_costs::pending_activation_cost_has_x`, the existing single authority for the activation-cost {X} fact.
- `trigger_matchers::bind_triggering_player_controller`, the precedent for binding an event fact into the filter before evaluation (same function, same Or/And/Not distribution).
- `TargetFilter::None`.

**New helpers, each justified:**
- (a) `CostSlot`, `parse_cost_slot`, `parse_x_cost_slot_qualifier`: one combinator replaces two hand-duplicated `alt`s and adds the "cost that contains {X}" form for both slots (decision 1).
- (b) `split_cast_or_activate_compound` with its typed outcome. No existing splitter covers a cast/activate disjunction, and the global event-verb lexicon stays unwidened (decision 2).
- (c) `parse_cost_x_intervening_if` and `parse_event_cost_subject`: the fold grammar (decision 3).
- (d) `constraint_bounds_per_definition`: an exhaustive classifier for decision 4.
- (e) `bind_activation_cost_facts`: U3.

## Logic Placement

- **The qualifier, the splitter, the fold and the declines** go in `parser/oracle_trigger.rs`, which owns printed-trigger compound splitting (T-B). The fold is event-referential ("that spell's", "that ability's"). That is the CLAUDE.md-sanctioned exception to `parse_inner_condition` single authority, since the predicate needs the trigger's events as context.
- **The activation-cost binding** goes in `game/trigger_matchers.rs::activated_source_matches`. That function is the one place where an activation event's facts meet the `valid_card` filter. The fact belongs to the activation (CR 602.2b, CR 113.7a), not to the source's characteristics, so it must not live in `filter.rs`'s zone-change-record evaluator. That evaluator correctly knows nothing about activations.
- Nothing goes in the transport or frontend layers.

## Rust Idioms

- **`CostSlot { Mana, Activation }`.** File-private, `Clone, Copy, PartialEq, Eq, Debug`. It has `fn x_filter_prop(self) -> FilterProp` with an exhaustive match (`Mana → HasXInManaCost`, `Activation → HasXInActivationCost`).
- **Slot check at each caller.** Each post-modifier caller matches the slot exhaustively: `CostSlot::Mana => return Some(..)`, `CostSlot::Activation => {}`. A wrong-slot phrase falls through to the remaining arms.
- **`VerbAgreement { Base, ThirdPerson }`.** Has `fn cast_delimiter(self) -> &'static str` (`" cast "` / `" casts "`), `fn activate_conjunction(self)` (`" or activate an ability"` / `" or activates an ability"`) and `fn activate_verb(self)` (`"activate"` / `"activates"`), all exhaustive.
- **`CastOrActivateHalf { Spell, Ability }`.** Has `fn cost_slot(self) -> CostSlot` (`Spell → Mana`, `Ability → Activation`).
- **`CastOrActivateDecline { PerDefinitionFrequencyRider, UnfoldableInterveningIf }`.** Has `fn strict_failure_description(self) -> &'static str`.
- **`CastOrActivateSplit { NotCandidate, Split(CastOrActivateHalves), Declined(CastOrActivateDecline) }`.** This mirrors `OtherThanFirstSpellParse::{Accepted, Rejected, NotCandidate}`.
- **`CastOrActivateHalves`.** `spell: SplitHalf`, `ability: SplitHalf`, `effect: String`, where `SplitHalf { condition: String, cost_x: Option<CostSlot> }`. No `bool` anywhere.
- **`constraint_bounds_per_definition(&TriggerConstraint) -> bool`.** An exhaustive match over all eleven variants with no `_` arm:
  - `true`: `OncePerTurn`, `OncePerGame`, `MaxTimesPerTurn{..}`, `OncePerOpponentPerTurn`.
  - `false`: `OnlyDuringYourTurn`, `OnlyDuringOpponentsTurn`, `OnlyDuringYourMainPhase`, `AtClassLevel{..}`, `EventSourceControlledBy{..}`, `NthSpellThisTurn{..}`, `NthDrawThisTurn{..}`.
  - The executor confirms in `triggers.rs::check_trigger_constraint_with_ref` that each `true` variant's counter is keyed per definition, and each `false` one is a gate or a per-player count. Any mismatch is fixed in the classifier, not argued.
- **`bind_activation_cost_facts`.** Takes `(&TargetFilter, &GameState, ObjectId) -> TargetFilter`. It computes the fact inside, so there is no `bool` parameter.

## Nom Compliance (parser files change)

| Step | Combinator |
|---|---|
| Cost slot | `alt((value(CostSlot::Mana, tag("mana")), value(CostSlot::Activation, tag("activation"))))` |
| Cost-X qualifier | `preceded(tag("with "), alt((preceded((opt(tag("an ")), tag("{x} in its ")), terminated(parse_cost_slot, tag(" cost"))), preceded(alt((tag("an "), tag("a "))), terminated(parse_cost_slot, tag(" cost that contains {x}"))))))`. The callers keep the existing `rest.trim().is_empty()` exactness check. |
| Split detection (lowercase condition) | `(parse_trigger_keyword_prefix, parse_trigger_actor, alt((value(VerbAgreement::Base, tag("cast ")), value(VerbAgreement::ThirdPerson, tag("casts ")))))`, then on the remainder `terminated(take_until(agreement.activate_conjunction()), tag(agreement.activate_conjunction()))`, then the ability tail must satisfy `alt((eof, peek(tag(" "))))`. That makes "an abilityx" or "an ability's" a non-candidate. |
| Per-definition rider | `parse_trigger_constraint(full_lower)` (existing) and `strip_first_time_each_turn_qualifier(condition).1` (existing). No new string matching. |
| Fold head | `tag("if ")` on the lowercase effect, a presence check only. |
| Fold | `parse_event_cost_subject = (preceded(tag("that "), alt((value(CastOrActivateHalf::Spell, tag("spell's ")), value(CastOrActivateHalf::Ability, tag("ability's "))))), terminated(parse_cost_slot, tag(" cost")))`. `parse_cost_x_intervening_if = delimited(tag("if "), separated_list1(tag(" or "), parse_event_cost_subject), (tag(" contains {x}"), tag(", ")))`. It runs through `nom_on_lower(effect, effect_lower, parse_cost_x_intervening_if)` so the effect remainder keeps its original case. |
| Half reconstruction (original case) | `split_once_on(condition, agreement.activate_conjunction())` gives (spell_condition, ability_tail). `split_once_on(spell_condition, agreement.cast_delimiter())` gives the head (`before`). The ability half is `format!("{before} {} an ability{ability_tail}", agreement.activate_verb())`. These are lockstep ASCII splits, as in `split_serial_event_compound`, with no cross-case byte arithmetic. |

No `contains`, `find`, `starts_with` or `split_once` is used for dispatch. The new code adds no `allow-noncombinator` annotation.

## Extension vs Creation

- **U1 extends** both post-modifiers by factoring their duplicated X arms into one slot-parameterized combinator (decision 1).
- **U2 extends** the T-B compound-split pattern: one trigger per event, sharing the effect, with offset base indexes.
  - It is a dedicated splitter that does not widen `parse_event_verb_start` / `parse_event_head_start`, which are census-pinned consultation sites.
  - Rejected, and staying rejected per decision 2: a `TriggerMode::SpellCastOrAbilityActivated` sibling, and a `TriggerDefinition.or_trigger` field (run-level deferral 2).
  - The declines reuse the `Rejected → TriggerMode::Unknown(<reason>)` strict-failure precedent from `try_parse_nth_spell_trigger`.
- **U3 extends** `activated_source_matches` with a second event-fact binder beside `bind_triggering_player_controller`.

## Analogous Trace

See Step 2:
- T-A, Magus: `oracle_effect/mod.rs` → `oracle_trigger.rs` post-modifiers → `trigger_matchers.rs` → `filter.rs` → `casting_costs.rs` → `copy_spell.rs`.
- T-B, compound split: `oracle_trigger.rs` `parse_trigger_lines_at_index_ir` → split family → `parse_trigger_line_with_index_ir` → `lower_trigger_ir`.
- T-C, the LKI activation matcher: `casting_targets.rs` → `trigger_matchers.rs` → `filter.rs`.

## Variant Discoverability

- **No engine enum variant is added.** `TriggerMode`, `TriggerConstraint`, `FilterProp`, `Effect` and `TargetFilter` are untouched. `TriggerMode::Unknown(String)` is an existing variant.
- The new enums (`CostSlot`, `VerbAgreement`, `CastOrActivateHalf`, `CastOrActivateDecline`, `CastOrActivateSplit`) are file-private, unserialized parser types, so they are not engine surface.
- A grep of `crates/engine/src` for existing cost-slot enums found only `TurnUpCostSource`, `PayCostKind`, `SpellCostSource` and `ExileCostSourceZone`. None of them covers the mana-vs-activation cost axis.
- `cargo engine-inventory` was not run (no-cargo constraint), and `data/engine-inventory.json` is absent. The `/add-engine-variant` gate does not apply to this phase. Its phase 2 and phase 3 runs are charter-scheduled.

## Identity / Provenance Contract

| Phrase | Authority | Binding time | Live or latched | Storage | Consumer | Invalidation | Multi-authority hostile fixture |
|---|---|---|---|---|---|---|---|
| "you cast" / "you activate" (actor) | `valid_target = TargetFilter::Controller` on each half, from the existing single-event parsers [read] | event match | live | `TriggerDefinition.valid_target` | `valid_player_matches` | n/a | SHAPE row P1-T4 asserts `Controller` on both halves. P1-T8 asserts the third-person agreement case gives an Opponent-typed `valid_target` on both halves. |
| "copy **that spell**" | `GameEvent::SpellCast.object_id` + the incarnation pin, through `targeting::triggering_spell` | trigger placement (CR 603.3) | on-stack entry, else the departed record (CR 608.2h) | existing | `copy_source_entry`, `TriggeringSource` arm | step-transition clear (existing) | I6: Counterspell counters the X sorcery above Unbound's trigger. The copy is still made from the departed record (ruling 1). |
| "copy **that ability**" | `GameEvent::AbilityActivated.stack_entry` (CR 602.2a) | event emission | live stack entry only. Departure gives `Gone`, by design until phase 2. | existing | `copy_spell::triggering_activation` | the entry leaving the stack | The stifled case is **DEFERRED(phase 2)**. Phase 1 asserts only the live case (I8). |
| "**that spell's mana cost** … contains {X}" | the spell object's printed `mana_cost`, via `cost_has_x` | event match (`match_spell_cast`, `valid_card`) | latched by CR 601.2f (locked in); not rechecked (decision 3) | `valid_card` `HasXInManaCost` on the spell half only | `filter.rs` live-object arm | n/a | I7: X only in the additional cost (Crashing Wave) gives no trigger (ruling 8). I3: X = 0 still triggers (ruling 6). |
| "**that ability's activation cost** contains {X}" | the most recent `(source, ability_index)` in `pending_activations`, through `ability_cost_has_x` (mana legs only) | event match, right after placement | latched by CR 602.2b; never rechecked, since `pending_activations` is cleared on pass | `valid_card` `HasXInActivationCost` on the ability half only, then bound to the activation fact by U3 | `activated_source_matches` → `bind_activation_cost_facts` | n/a | I8 vs its sibling: Oracle of Nectars `{X},{T}` is copied, Fountain of Youth `{2},{T}` is not. I9: a source sacrificed by its own cost is still answered from the activation, not from LKI. |
| "You may choose new targets for the copy" | `CopyRetargetPermission::MayChooseNewTargets`, then `WaitingFor::CopyRetarget` | copy creation (CR 707.10c) | n/a | effect field | `copy_choice` walk | n/a | I5: the copy is retargeted to P0 (P0 +2, P1 +2) vs kept (P1 +4). |

## Verification Matrix

The runtime rows follow `/card-test`:
- Fixtures are built from verbatim Oracle text (`add_enchantment_from_oracle`, `add_spell_to_hand_from_oracle`, `add_creature_from_oracle`, `add_artifact_from_oracle`), with mana costs set via `with_mana_cost` where the builder needs them.
- Players act through `runner.cast(..).x(n)…resolve()/commit()` and `runner.activate(src, i).x(n).resolve()`.
- Assertions use `CastOutcome` deltas or state reads.
- The constant is `UNBOUND_ORACLE`, both lines verbatim, joined with `\n`.
- Every negative row names its paired positive.

| # | Claim | Changed seam | Entry point | Test (file :: name) | Revert-failing assertion | Paired positive / sibling / hostile | First production branch reached |
|---|---|---|---|---|---|---|---|
| P1-T1 | Cost-X qualifier: both slots, both grammars | `parse_x_cost_slot_qualifier` | direct | oracle_trigger_tests :: `x_cost_slot_qualifier_accepts_both_slots_and_grammars` | Each of `with {x} in its mana cost`, `with an {x} in its mana cost`, `with a mana cost that contains {x}`, and the 3 activation forms (`… an activation cost that contains {x}`) returns the right slot with an empty rest. | Hostile: `with a mana cost that contains {r}` is Err. `with an {x} in its loyalty cost` is Err. `with {x} in its mana costs` leaves rest `"s"` (callers reject it). | `parse_cost_slot` alt |
| P1-T2 | Post-modifiers route by slot; existing strings unchanged | `parse_post_spell_modifier`, `parse_post_activation_modifier` | direct | oracle_trigger_tests :: `post_modifiers_route_cost_x_qualifier_by_slot` | Spell modifier: `with a mana cost that contains {x}` gives `Typed{[HasXInManaCost]}`. Activation modifier: `with an activation cost that contains {x}` gives `Typed{[HasXInActivationCost]}`. | Wrong slot: spell modifier with the activation phrase is `None`, and vice versa. Preservation (see §Reference Readings): `with {x} in its mana cost` and `with an {x} in its mana cost` still give HasXInManaCost; `with {x} in its activation cost` still gives HasXInActivationCost. | slot match arm |
| P1-T3 | Unbound line 1 carries the X filter | U1 through the SpellCast "you cast" arm | `parse_trigger_lines` (verbatim line 1) | oracle_trigger_tests :: `unbound_line_one_filters_permanent_spells_by_x_in_mana_cost` | `len == 1`, `mode == SpellCast`, `valid_card == And{[Typed{[Permanent]}, Typed{props:[HasXInManaCost]}]}` (the shape `parse_spell_qualifier_payload` composes) | Reach and honest-red guard: the execute effect is still `Effect::Unimplemented` ("double the value of X", **DEFERRED(phase 3)**), so the card stays red. | `parse_spell_qualifier_payload` post arm |
| P1-T4 | Line 2 gives two definitions with folded filters and the shared copy | U2 split + fold + hook | `parse_trigger_lines` (verbatim line 2) and `parse_oracle_text` (full card) | oracle_trigger_tests :: `unbound_copy_line_splits_cast_and_activate_with_folded_cost_x` | See the P1-T4 assertion list below the table. | Sibling: the Rings of Brighthearth verbatim line still gives exactly 1 `AbilityActivated` with `ActivatedAbilityIsNonMana`, since it has no cast leg (non-candidate). | `split_cast_or_activate_compound` Split arm |
| P1-T5 | Agreement and non-candidate boundaries | split detection | `parse_trigger_lines` | oracle_trigger_tests :: `cast_or_activate_split_agreement_and_boundaries` | See the P1-T5 assertion list below the table. | The Magus `MAGUS_ORACLE` delayed trigger is unchanged; its own `issue_1010` tests still pass. | detection combinator |
| P1-T6 | F3 / decision 4: per-definition frequency riders decline to strict failure | Declined(PerDefinitionFrequencyRider) | `parse_trigger_lines` | oracle_trigger_tests :: `cast_or_activate_split_declines_per_definition_frequency_riders` | Bill Potts verbatim gives **exactly 1** definition with `TriggerMode::Unknown(_)` (today: a supported `SpellCast` with the leg dropped). Glava verbatim: the same. A grammar fixture (synthetic: no printed card has it) "Whenever you cast a spell or activate an ability for the first time each turn, draw a card." also gives 1 Unknown. | Paired positive in the same test: Unbound line 2 gives 2 non-Unknown definitions. Gate negative: a synthetic "Whenever you cast a spell or activate an ability, draw a card. This ability triggers only during your turn." still **splits** (2 definitions, each with `OnlyDuringYourTurn`), proving the classifier declines only on counted riders. | `constraint_bounds_per_definition` |
| P1-T7 | F4 / decision 5: an unfoldable or partial intervening-if declines | Declined(UnfoldableInterveningIf) | `parse_trigger_lines` | oracle_trigger_tests :: `cast_or_activate_split_declines_unfoldable_intervening_if` | Vazi verbatim gives exactly 1 definition, `TriggerMode::Unknown(_)`. These synthetic fold hostiles each give 1 Unknown: a half receives zero subjects ("…, if that spell's mana cost contains {X}, copy that spell or ability."); a repeated subject ("if that spell's mana cost or that spell's mana cost contains {X}"); a slot mismatch ("if that spell's activation cost or that ability's mana cost contains {X}"). | Paired positive: Unbound line 2 splits. | fold validation |
| P1-T8 | Decision 8: Neerdiv's activation leg surfaces honestly | Split, no fold | `parse_trigger_lines` (Neerdiv verbatim line 2) | oracle_trigger_tests :: `neerdiv_cast_or_activate_split_surfaces_activation_leg` | `len == 2`. [0] is `SpellCast` with `spell_cast_origin == Equals(Graveyard)` and the Draw + PutCounter body, preserving today's reading (§Reference Readings). [1] is either `AbilityActivated` with a graveyard-scoped `valid_card`, or `TriggerMode::Unknown(_)`. It is **never** `AbilityActivated` with `valid_card == None`, which would be a silent drop the split introduced. Expected [read, UNPROBED, probe PR3]: `Unknown`. | Reach: [0] is non-Unknown with no Unimplemented in its execute. | half re-parse |
| I1 | Line 1 no longer over-fires (runtime) | U1 | `runner.cast(..).commit()` | integration :: `unbound_line_one_triggers_only_for_x_permanent_spells` | See the I1 assertion list below the table. | Positive reach guard: the Ballista commit in the same test. [read: Unimplemented-effect triggers are still placed, `triggers.rs` "Trigger with no execute -- use Unimplemented as no-op marker"; UNPROBED, PR1.] | `match_spell_cast` valid_card |
| I2 | An {X} sorcery is copied with the same X, and the copy is not cast | U2 spell half + `CopySpell` | `runner.cast(prosperity).x(2).resolve()` | integration :: `unbound_copies_x_sorcery_with_same_x` | `assert_hand_drawn(P0, 4)` and `assert_hand_drawn(P1, 4)`. Exactly 4 shows that one copy has X = 2 (CR 707.10, ruling 9) and that the copy did not re-trigger Unbound (CR 707.10: a copy isn't cast; ruling 10). | Negative: Divination (no X) with Unbound gives `assert_hand_drawn(P0, 2)` (no copy). Its reach guard is I2's positive. | spell half `HasXInManaCost` |
| I3 | X = 0 is still copied | same | Prosperity `.x(0)` | integration :: `unbound_copies_x_zero_spell` | At least one `GameEvent::SpellCopied` is among the run's events (ruling 6). Both players draw 0. | Positive guard: the `SpellCopied` event itself. Contrast with I2's Divination negative (no event). | same |
| I4 | Two Unbounds copy twice (F6, ruling 7) | same | Prosperity `.x(1)` with two Unbounds | integration :: `two_unbounds_copy_an_x_spell_twice` | `assert_hand_drawn(P0, 3)` and `(P1, 3)`. | Control: I2 (one Unbound). | same |
| I5 | The copy may take new targets; the copy resolves first (CR 707.10c, ruling 1, ruling 11) | existing retarget walk, reached via U2 | Stroke of Genius `.x(2).target_player(P1).commit()`, then driven by hand: pass priority until `WaitingFor::CopyRetarget` (the `attached_to_declared_target::drive_to_retarget_prompt` pattern), then `GameAction::ChooseTarget{target: Some(TargetRef::Player(P0))}`, then pass to an empty stack | integration :: `unbound_copy_may_choose_new_targets` | While the retarget prompt is open, the top stack entry is the copy (above the original Stroke). Final: P0 drew 2, P1 drew 2. | Sibling, same file, `…_keeping_targets`: `ChooseTarget{target: None}` gives P1 drew 4, P0 drew 0. | `CopyRetarget` arm |
| I6 | Countered spell: the copy is still made (ruling 1). Measured here per the charter. | existing `TriggeringSpell::Departed` path, reached via U2 | Prosperity `.x(2).commit()`. With Unbound's trigger on the stack, P0 casts a zero-cost "Counter target spell." instant (verbatim text; cost elided, as in `copy_that_ability_referent::add_zero_cost_instant`) targeting Prosperity with `.commit()`. Then pass to an empty stack. | integration :: `unbound_copies_countered_x_spell` | `GameEvent::SpellCountered` and `GameEvent::SpellCopied` both appear. Prosperity is in the graveyard. Each player drew 2. | Reach: I2. **If this row fails on the phase-1 tree, the executor stops and returns.** The orchestrator writes `addenda/phase-1`, the row becomes `DEFERRED(phase 2)`, and the fix takes the defective-reference route in phase 2 (the charter's constraint). | `triggering_spell` Departed arm |
| I7 | An {X} only in an additional cost does not trigger (F6, ruling 8) | U2 spell half (`HasXInManaCost` reads `mana_cost` only) | Crashing Wave `.x(0).commit()` | integration :: `unbound_ignores_x_only_in_an_additional_cost` | After commit, no `StackEntryKind::TriggeredAbility` with `source_id == unbound` is on the stack. | Paired positive, same test: a fresh scenario where Prosperity `.x(0).commit()` does put Unbound's trigger on the stack. [UNPROBED, PR4: whether the driver announces X for a waterbend additional cost. If `.x(0)` is unused, drop `.x`.] | `match_spell_cast` valid_card |
| I8 | An {X} activated ability is copied with the same X; a non-X ability is not | U2 ability half + U3 binding on the live branch | `runner.activate(nectars, 0).x(3).resolve()` | integration :: `unbound_copies_x_activated_ability` | P0's life delta is +6 (3 + 3). | Negative sibling, same file: `runner.activate(fountain, 0).resolve()` gives +1 (no copy). Reach guard: the Nectars positive. [UNPROBED, PR7: `{T}` with no summoning sickness for a scenario battlefield creature.] | `bind_activation_cost_facts` → `pending_activation_cost_has_x` |
| I9 | A cost that sacrifices its own source is still copied (draft residual 3) | **U3** LKI branch | Snake Basket `.x(2).resolve()` with Unbound | integration :: `unbound_copies_x_ability_whose_cost_sacrificed_its_source` | P0 controls **4** Snake tokens. On the tree with U1 + U2 but without U3 it controls 2 (the measurement, PR2). | Control: the same scenario without Unbound gives 2 Snakes. | `activated_source_matches` `Some(lki)` arm |
| I10 | The U3 class benefit through Magus, independent of U2 | U3 | `runner.activate(magus, 0).resolve()`, then Snake Basket `.x(2).resolve()` | integration :: `magus_copies_x_ability_whose_cost_sacrificed_its_source` | 4 Snakes. | Control: Magus's existing live-source X ability row (`issue_1010`) still draws 4. | same |
| S1 | Triggered abilities that pay {X} are not copied (ruling 2; C6) | structural | none | none: a proof from code, recorded in the commit message and as a doc line on the splitter. | Every production caller of `casting_targets::emit_ability_activated` is an activation path: `casting.rs::record_activated_ability_placed`, `casting_costs.rs` (mana-ability auto-tap), `mana_abilities.rs`, `mana_sources.rs`. The `triggers.rs` call is inside a `#[cfg(test)]` fixture [read]. So no triggered ability emits `AbilityActivated`. CR 113.3b / CR 602.1 vs CR 113.3c / CR 603.1. | n/a | n/a |

**P1-T4 assertions:**
- `len == 2`.
- **[0]:**
  - `mode == SpellCast`, `valid_target == Some(Controller)`.
  - `valid_card == Or{[Typed{[Instant], props:[HasXInManaCost]}, Typed{[Sorcery], props:[HasXInManaCost]}]}` (`add_spell_prop` distributes into the `Or`).
  - `condition == None`, `constraint == None`.
  - The execute effect is `CopySpell{target: TriggeringSource, retarget: MayChooseNewTargets, ..}`.
- **[1]:**
  - `mode == AbilityActivated`, `valid_target == Some(Controller)`.
  - `valid_card == Typed{props:[HasXInActivationCost]}`.
  - `condition == None`, and the same execute.
- No `Effect::Unimplemented` anywhere in either definition.
- With `parse_oracle_text`, Unbound yields 3 triggers whose printed indices are distinct.

**P1-T5 assertions:**
- Third person: the synthetic grammar fixture "Whenever an opponent casts a spell or activates an ability, draw a card." gives [`SpellCast`, `AbilityActivated`], each with an Opponent-typed `valid_target`.
- Non-candidates keep today's single output:
  - "Whenever an opponent casts a spell, …" (no conjunction);
  - "Whenever you cast a spell or activate an abilityx, …" (synthetic boundary case);
  - Rings verbatim.

**I1 assertions:**
- With Unbound on the battlefield, cast Walking Ballista (verbatim, `{X}{X}`) `.x(1).commit()`: exactly one `StackEntryKind::TriggeredAbility` with `source_id == unbound` is on the stack.
- In a fresh scenario, cast the vanilla creature `add_creature_to_hand(P0, "Grizzly Bears", 2, 2)` with `{1}{G}` and `.commit()`: no Unbound trigger is on the stack.

**Out-of-phase and out-of-run rows (deferral vocabulary).**

| Row | Status |
|---|---|
| `unbound_copies_stifled_x_ability` (Stifle the {X} ability above Unbound's trigger; the copy still resolves, ruling 1) | **DEFERRED(phase 2)** |
| Rings of Brighthearth: copy of a stifled ability | **DEFERRED(phase 2)** |
| Same-source two-activation identity hostile (Stifle the X one; copy that entry, not its sibling) | **DEFERRED(phase 2)** |
| `unbound_doubles_x_for_walking_ballista`, two Unbounds give ×4, X = 0 doubling, doubling after a counter is a no-op | **DEFERRED(phase 3)** |
| Unbound coverage-supported | **DEFERRED(phase 3)** |
| Krasis + Unbound ordering prompt | **DEFERRED(phase 4)** |
| `unbound_triggers_on_x_mana_ability` (Wizard's Rockets) | **Out of run** (charter run-level deferral 1). Kept as today's known divergence; recorded in the phase-1 summary. |
| Bill Potts copies once per turn across both legs | **Out of run** (run-level deferral 2, `or_trigger`) |

**Coverage status impact.** Expected from the parse readings above, regenerated by the Tilt `card-data` resource and by `cargo coverage`:
- **Unbound:** stays unsupported. Line 1's effect is Unimplemented (**DEFERRED(phase 3)**). Line 2 is now fully typed.
- **Bill Potts:** supported → **unsupported**. This is honest: the silent drop becomes `Unknown` (decision 4).
- **Neerdiv:** supported → **unsupported** is expected. This is honest: the dropped leg becomes `Unknown` (decision 8, PR3).
- **Glava and Vazi:** stay unsupported. Their silently partial `SpellCast` is replaced by `Unknown` (decisions 4 and 5).
- **No other card** may change. The executor proves this with a whole-corpus trigger diff (Step 5.7) and lists every changed card in the commit message.

**Is any Oracle text accepted while its semantics stay deferred?** Only line 1's "double the value of X". It stays `Effect::Unimplemented` (unchanged), so coverage stays red. Every declined compound becomes `TriggerMode::Unknown`. No clause is dropped.

## Reference Readings

| Row | Reference | Derivation (Oracle + CR) | Measured? |
|---|---|---|---|
| P1-T2 preservation: "with {X} in its mana/activation cost" | today's post-modifier output | "with {X} in its mana cost" means the printed mana cost contains the {X} symbol (CR 107.3a, CR 202.1), so `HasXInManaCost`. "with {X} in its activation cost" means the activation cost contains {X} (CR 602.2b, the activation cost being the analog of mana cost), so `HasXInActivationCost`. Agrees. | [read] the two `alt` arms being factored. The executor's whole-corpus diff (Step 5.7) is the measurement; the command is there. |
| P1-T4 [0] base shape `Or[Instant, Sorcery]` and `valid_target Controller` | today's Unbound line-2 output | "you cast an instant or sorcery spell" (CR 601.2a; "you" is the controller) | [measured] by `jq '."unbound flourishing".triggers[1]'`. Agrees. |
| P1-T8 [0] Neerdiv spell half | today's Neerdiv trigger[1] | "cast a spell from your graveyard" gives SpellCast with cast origin Graveyard (CR 601.2a); body "draw a card and put a +1/+1 counter on Neerdiv" | [measured] `spell_cast_origin {Equals: Graveyard}`, Draw + PutCounter. Agrees. |
| I2 / I8 "the copy has the same X" | Magus `issue_1010` (the copy of Draw X=2 draws 2) | Ruling 9 + CR 707.10 ("copies … the value of X") | Measured by the existing passing tests in `issue_1010_magus_lucea_kane.rs`, green at base per the Tilt `test-engine` suite. Not re-run this round (no cargo). Agrees. |
| I6 countered-spell copy | the existing `TriggeringSpell::Departed` path | Ruling 1 + CR 608.2h + CR 707.10: the copy is made from the spell as it last existed on the stack | Reference: the existing tests `issue_6877_departed_triggering_spell::winning_trigger_copies_spell_countered_before_it_resolved` and `::optional_copy_of_countered_spell_is_created_when_accepted`, the same path with a different trigger. Not re-run (no cargo). I6 is the charter-required measurement for Unbound; command `cargo test -p phase-engine --test integration unbound_copies_countered_x_spell` (or the Tilt `test-engine` log). A failure takes the DEFERRED(phase 2) addendum route. |
| I9 control "2 Snakes without Unbound" | Snake Basket alone | "Create X 1/1 green Snake creature tokens", X = 2 | [measured] Snake Basket is coverage-supported. The runtime value is the row's own control. |

No row asserts parity with a reading that disagrees with its derivation. Residual 3 below is the one place the base reading disagrees with the card. It is a pre-existing defect fixed in this phase (defective-reference route), and its rows assert the derived value.

---

## Constraints carried (from the charter)

- **F3, decision 4.**
  - Measured fallbacks: Bill Potts is today a **supported line with a dropped leg**. Glava is unsupported.
  - The charter therefore requires the decline to route to a strict-failure marker. This plan does that **uniformly** for every per-definition rider: `TriggerMode::Unknown(<reason>)`.
  - Rows: P1-T6.
- **F4, decision 5.**
  - Vazi is today unsupported, with a `null` condition that is pre-existing and goes to triage (C8).
  - Decline is uniform: `Unknown`. Partial folds (zero subjects, repeated subjects, slot mismatch) also decline.
  - Rows: P1-T7.
- **F5.** Sizing counts authored and registration paths separately; there are no compiler-forced paths. The reviewer's whole-run figure stands; this phase contributes 5.
- **F6.** Ruling 8 is I7. Ruling 7 is I4.
- **Draft residual 3 (departed-LKI activation source), established by reading:**
  1. `emit_ability_activated` sets `departed_source_lki` when the battlefield-announced source has left.
  2. `activated_source_matches` then evaluates `valid_card` through `matches_target_filter_on_departed_battlefield_lki`, then `zone_change_record_matches_property`.
  3. That function's `HasXInActivationCost` arm returns `false` ("Fail closed — the snapshot carries no such info").
  - So Unbound (and Magus) **do not trigger** for "{X}, Sacrifice this …:" abilities today. This is a pre-existing defect class: Pernicious Deed, Snake Basket and others, with 17 corpus lines.
  - The fix (U3) requires the charter's conditional anchor `crates/engine/src/game/trigger_matchers.rs`. **The orchestrator writes the `addenda/phase-1` line admitting it at scope-freeze.**
  - The executor's I9 run on the tree before U3 is the confirming measurement (PR2). If I9 already passes without U3, the executor drops U3 and the anchor, and reports that.
- **Countered-spell copy measurement.** This is I6, with the stop-and-addendum route if it fails.

## Probes owed (executor, first)

| Probe | Assumption | Design decision it can change |
|---|---|---|
| PR1 | A trigger whose effect is `Effect::Unimplemented` is still put on the stack (the I1 positive) | If not, I1 is rewritten as an `activated`/`spell` matcher-level test in `oracle_trigger_tests.rs` over a `GameScenario` stack object (the file already imports `GameScenario`), still with its positive pair. |
| PR2 | I9 gives 2 Snakes on the U1 + U2 tree without U3 | Whether U3 and its anchor are needed at all (see Constraints). |
| PR3 | Neerdiv's reconstructed half "Whenever you activate an ability of a card in your graveyard, …" parses to `TriggerMode::Unknown` | If it parses to `AbilityActivated` with `valid_card == None` (a silent drop the split introduced), P1-T8 fails. The executor returns to plan; the remedy would be declining splits whose ability-half qualifier is not consumed, which is a decision-8 refinement. |
| PR4 | The scenario driver announces X for Crashing Wave's waterbend {X} additional cost at X = 0 | Drives the shape of I7 only. The assertion is unchanged. |
| PR5 | `bind_activation_cost_facts`'s stripped `Typed{}` (no props, no types) matches the source in both the live (`target_filter_matches_object`) and the LKI (`zone_change_filter_inner`) evaluators | If an empty `Typed` does not match on either branch, bind to `TargetFilter::Any` when the stripped leaf is empty. I8 and I9 measure this. |
| PR6 | An untargeted copy (Prosperity, Oracle of Nectars) opens no `CopyRetarget` walk, so `.resolve()` completes. Evidence: the Magus untargeted Draw-X copy already completes under `.resolve()`. | If a walk opens, drive it with `ChooseTarget{target: None}` as in I5. |
| PR7 | A creature added by `add_creature_from_oracle` can pay `{T}` immediately | If it is summoning-sick, clear sickness via `runner.state_mut()` as other tests do, or use an artifact {X},{T} source. |

---

## Step 5: Step-by-step implementation

Every CR number below was grep-verified in `docs/MagicCompRules.txt` this round: 107.3a, 113.3b, 113.3c, 113.7a, 202.1, 400.7, 601.2a, 601.2f, 602.1, 602.2a, 602.2b, 603.1, 603.2, 603.2c, 603.2h, 603.3b, 603.4, 605.1a, 605.3b, 608.2h, 701.6a, 707.10, 707.10c.

### 5.1 Tests first (`/oracle-parser` §9a)

- Write P1-T1 through P1-T8 in `crates/engine/src/parser/oracle_trigger_tests.rs`, using the existing `parse_trigger_lines`, `parse_post_spell_modifier` and `parse_post_activation_modifier` imports via `use super::*`.
- Make the new combinator `pub(super)` or `pub(crate)` only as far as the test module needs (match the visibility of the existing `parse_post_*` functions).
- Create `crates/engine/tests/integration/unbound_flourishing.rs` with rows I1–I10.
  - The `UNBOUND_ORACLE` constant holds both lines verbatim.
  - Fixture constants are verbatim per Step 0.
  - `add_colorless` / `with_library_top` helpers follow the `issue_1010` pattern.
- Add `mod unbound_flourishing;` to `crates/engine/tests/integration/main.rs`, between `ultimate_spider_man_mass_counter_double` and `unholy_citadel_legendary_color_banding_grant`.

### 5.2 U1: the cost-X qualifier (`crates/engine/src/parser/oracle_trigger.rs`)

1. Beside `parse_post_activation_modifier`, add a file-private `enum CostSlot { Mana, Activation }` with `fn x_filter_prop(self) -> FilterProp`.
   - Doc: `/// CR 107.3a + CR 202.1 + CR 602.2b: which printed cost an "{X} in its … cost" qualifier names — a spell's mana cost, or an activated ability's activation cost (CR 602.2b: the activation cost is the analog of a spell's mana cost).`
2. Add `fn parse_cost_slot(input: &str) -> OracleResult<'_, CostSlot>` and `pub(crate) fn parse_x_cost_slot_qualifier(input: &str) -> OracleResult<'_, CostSlot>` with the §Nom Compliance grammar.
   - Doc: the shared single authority for the X-cost qualifier in both slots and both phrasings ("with {X} in its mana cost", "with a mana cost that contains {X}").
3. In `parse_post_spell_modifier`, replace the `"with {x} in its mana cost"` / `"with an {x} in its mana cost"` `alt` arm with:
   - `if let Ok((rest, slot)) = parse_x_cost_slot_qualifier(modifier) { match slot { CostSlot::Mana if rest.trim().is_empty() => return Some(TargetFilter::Typed(TypedFilter::default().properties(vec![slot.x_filter_prop()]))), CostSlot::Mana | CostSlot::Activation => {} } }`
   - The executor may restructure this to satisfy clippy, keeping both arms explicit.
   - Update the doc bullet to list both phrasings.
4. In `parse_post_activation_modifier`, do the same with `CostSlot::Activation` and update its doc.

### 5.3 U2: the cast-or-activate splitter (same file, beside `split_shared_subject_event_list`)

1. Add the file-private types from §Rust Idioms.
   - `VerbAgreement`, with its three exhaustive `&'static str` methods.
   - `CastOrActivateHalf`, with `cost_slot`.
   - `SplitHalf`, `CastOrActivateHalves` and `CastOrActivateSplit`.
   - `CastOrActivateDecline`, with `strict_failure_description`:
     - `PerDefinitionFrequencyRider` → `"cast-or-activate compound with a per-ability frequency rider"`
     - `UnfoldableInterveningIf` → `"cast-or-activate compound with an intervening if that does not fold per event"`
2. Add `fn constraint_bounds_per_definition(constraint: &TriggerConstraint) -> bool` (exhaustive; see §Rust Idioms).
   - Doc: `/// CR 603.2h: a "do this only once each turn" / "triggers only once each turn" rider bounds ONE printed ability. Splitting that ability into two definitions would give each its own counter, so a counted rider makes the cast-or-activate splitter decline.`
3. Add `fn parse_event_cost_subject` and `fn parse_cost_x_intervening_if` (§Nom Compliance).
4. Add `fn split_cast_or_activate_compound(condition: &str, cond_lower: &str, effect: &str, full_lower: &str) -> CastOrActivateSplit`:
   1. Run detection on `cond_lower`. On failure return `NotCandidate`.
   2. If `parse_trigger_constraint(full_lower).as_ref().is_some_and(constraint_bounds_per_definition)` or `strip_first_time_each_turn_qualifier(condition).1.is_some()`, return `Declined(PerDefinitionFrequencyRider)`.
   3. Let `effect_lower = effect.to_lowercase()`.
      - If `tag("if ")` matches, run `nom_on_lower(effect, &effect_lower, parse_cost_x_intervening_if)`. Validate that each `(half, slot)` has `slot == half.cost_slot()` and that each half appears exactly once.
        - On success: `spell.cost_x = Some(CostSlot::Mana)`, `ability.cost_x = Some(CostSlot::Activation)`, and the effect is the original-case remainder.
        - On any failure: `Declined(UnfoldableInterveningIf)`.
      - With no leading "if ", both `cost_x` are `None` and the effect is unchanged.
   4. Reconstruct the two condition texts with the lockstep `split_once_on` recipe. If any lockstep split fails, return `NotCandidate`, because the detection and the original disagree.
   5. Return `Split`.
   - Doc:
     - `/// CR 603.1 + CR 603.2: "Whenever <actor> cast(s) <spell> or activate(s) an ability<tail>, <effect>" names two trigger events (a spell becoming cast, CR 601.2i; an ability becoming activated, CR 602.2a). Each becomes its own definition sharing the effect.`
     - `/// CR 603.4 + CR 601.2f + CR 602.2b: the "if that spell's mana cost or that ability's activation cost contains {X}" intervening if reads costs locked in at casting/activation, so its resolution recheck has the event-time answer. Each anaphor is distributed to its one event's valid_card instead of a rechecked TriggerCondition.`
     - `/// Only stack-using activations reach this leg: triggered abilities are never activated (CR 113.3b / CR 602.1 vs CR 113.3c / CR 603.1).`
5. Add `fn decline_to_strict_failure(ir: &mut TriggerIr, reason: CastOrActivateDecline)`. It sets `ir.condition` and `ir.partial_def.mode` to `TriggerMode::Unknown(reason.strict_failure_description().to_string())`.
   - Doc: the compound is left honestly unsupported instead of shipping a dropped leg or a split frequency counter.
6. **Hook.** In `parse_trigger_lines_at_index_ir`, after the `parse_disjunctive_zone_change_condition` gate and **before** Pattern 2, `match split_cast_or_activate_compound(&condition, &cond_lower, &effect, &lower)`:
   - `NotCandidate => {}`
   - `Split(halves)`: for `(i, half)` in `[halves.spell, halves.ability]`:
     - the text is `if halves.effect.is_empty() { half.condition } else { format!("{}, {}", half.condition, halves.effect) }`;
     - `let mut ir = parse_trigger_line_with_index_ir(&text, card_name, base_trigger_index.map(|b| b.offset(i)), ctx);`
     - `if let Some(slot) = half.cost_x { let prop = slot.x_filter_prop(); ir.partial_def.valid_card = Some(match ir.partial_def.valid_card.take() { Some(filter) => add_spell_prop(filter, prop), None => TargetFilter::Typed(TypedFilter::default().properties(vec![prop])) }); }`
     - push, then return the results.
   - `Declined(reason)`: `let mut ir = parse_trigger_line_with_index_ir(text, card_name, base_trigger_index, ctx); decline_to_strict_failure(&mut ir, reason); return vec![ir];`
   - Comment on the hook: dispatch-order note (before Pattern 2; Pattern 2 claims none of these lines today, [measured]: all five class lines lower to one definition at base).
7. Extend `add_spell_prop`'s doc only: it also attaches a prop to an activated-ability source filter (the folded `HasXInActivationCost`).

### 5.4 U3: activation-cost fact binding (`crates/engine/src/game/trigger_matchers.rs`)

1. Add `fn bind_activation_cost_facts(filter: &TargetFilter, state: &GameState, activated_id: ObjectId) -> TargetFilter`. It distributes through `Typed`/`Or`/`And`/`Not` exactly like `bind_triggering_player_controller`.
   - A `Typed` whose `properties` contain `FilterProp::HasXInActivationCost`: compute (once, lazily) `crate::game::casting_costs::pending_activation_cost_has_x(state, activated_id)`.
     - If true, return the `Typed` with that prop removed.
     - If false, return `TargetFilter::None`.
   - Every other leaf is cloned.
   - Doc: `/// CR 602.2b + CR 601.2f + CR 113.7a: whether "that ability's activation cost contains {X}" is a fact of the activation — its cost was locked in as it was activated, and the ability exists independently of its source — answered from the activation record (\`pending_activations\`), never from the source's characteristics. A source a cost moved off the battlefield answers its characteristics from last known information, which carries no activation (filter.rs fails that leaf closed), so the fact is bound here before either branch evaluates.`
2. In `activated_source_matches`, after `bind_triggering_player_controller`, apply `let filter = bind_activation_cost_facts(&filter, state, activated_id);`. Both the `Some(lki)` and `None` arms then evaluate the bound filter.
3. **Known limitation (doc line, fail-closed).** `pending_activation_cost_has_x` reads the ability at `ability_index` on the object after costs were paid. For a printed ability of a card or token this is the activated ability. For an ability granted only on the battlefield and lost with the move, the lookup misses and the trigger does not fire. It never fires spuriously. No class card depends on that case.

### 5.5 Comments and CR annotations

- Every new rule-implementing function carries the CR lines above.
- Do not cite CR 701.10 or anything from phases 2–4.

### 5.6 Phase-1 summary notes, for the orchestrator's acceptance record

- The mana-ability {X} divergence (charter run-level deferral 1) is kept and recorded: Wizard's Rockets does not trigger Unbound.
- Every coverage flip (Bill Potts, Neerdiv; Glava and Vazi now `Unknown`) is listed.
- Vazi's pre-existing dropped Treasure condition goes to triage (C8). With the decline it is no longer emitted at all.

### 5.7 Verification (risk-scaled, `CLAUDE.md`)

1. `cargo fmt --all`.
2. `./scripts/check-parser-combinators.sh` (the parser combinator gate).
3. If Tilt is up (`tilt get uiresource clippy >/dev/null 2>&1`): `./scripts/tilt-wait.sh --timeout 240 clippy test-engine card-data`. Otherwise use the direct fallbacks in the `project-reference` skill.
4. **Whole-corpus trigger diff, the preservation measurement.**
   - Before the edits, save `jq -c 'to_entries[] | {k: .key, t: .value.triggers}' client/public/card-data.json | sort > "$SCRATCH/triggers-base.jsonl"`.
   - After `card-data` regenerates, produce the same file again and `diff` the two.
   - The changed keys must be exactly: unbound flourishing, bill potts, glava five-advents mage, neerdiv devious diver, vazi keen negotiator.
   - Any other changed key is a U1 preservation failure, or a U2 detector over-claim, and is fixed before commit.
5. `cargo coverage` (a direct one-shot audit). Report the per-card delta for the five cards.
