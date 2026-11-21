# Phase 3 plan: one substitution predicate, and the fence decision (outcome A)

Phase-plan mode. Run `cz86-rework-9670`; charter `phase-charter` (frozen), P3 entry only. Binding
inputs: `addenda/phase-3` (outcome A; claim narrowed to protection; no uniform one-choice claim
under CR 700.1; the fence reads the runtime predicate), `maintainer-answer-2026-11-17T1351.md`
item 1, `phase-1-accepted-summary.md`, `phase-2-accepted-summary.md`.

- `PHASE_BASE_SHA` = `6f40e4dc6a` (branch `engine/unpreventable-damage-riders`). Every "today"
  claim below was read at this commit, by function name.
- **What P2 did not do** (it was trimmed by the user). Combat rider pauses are still settled inside
  the combat batch under `damage_replacement_choice_unasked_in_combat`. The in-leg earlier-rider
  drain and the Phase D aggregate riders are still auto-settled there. Guards are still read once
  per pass entry, not carried across a park. P3 does not touch any of this. That fence and its
  machinery stay as P1 left them.
- Attempt history: pre-charter plan r1 had 2 behavior findings and 1 text finding. Charter r1 had
  2 corrections. P1 was accepted (2 plan rounds, 0 impl-review findings). P2 ran a long loop, was
  trimmed to U3 (Channel Harm's "may"), and was accepted. Budget: standard. No expansion case is
  open.
- **Probe status.** The planning worktree is read-only and nothing was built or run. No runtime
  claim here was probed by running code. Each claim is read from source and named by function, or
  is listed in §15 as a claim the executor settles before the edit that depends on it. One
  parse-output fact was **measured from data**: the card-data export at commit `51adf5568`
  (`client/public/card-data.json`, generated 2026-10-22, before both #9670 and this branch).

---

## 0. Premise (Step 0)

Oracle text below is read from MTGJSON `data/mtgjson/AtomicCards.json`, version `5.3.0+20261021`
(the charter's version), not from the brief.

**The class (charter class A).**
- **Soul-Scar Mage**: "Prowess / If a source you control would deal noncombat damage to a creature
  an opponent controls, put that many -1/-1 counters on that creature instead."
- **Szadek, Lord of Secrets**: "Flying / If Szadek would deal combat damage to a player, instead
  put that many +1/+1 counters on Szadek and that player mills that many cards."
- **Undead Alchemist**: "If a Zombie you control would deal combat damage to a player, instead
  that player mills that many cards. / Whenever a creature card is put into an opponent's
  graveyard from their library, exile that card and create a 2/2 black Zombie creature token."
- **Phytohydra**: "If damage would be dealt to this creature, put that many +1/+1 counters on it
  instead."
- **Lichenthrope**: "If damage would be dealt to this creature, put that many -1/-1 counters on it
  instead. / At the beginning of your upkeep, remove a -1/-1 counter from this creature."
- **Dralnu, Lich Lord**: "If damage would be dealt to Dralnu, sacrifice that many permanents
  instead. / {T}: Target instant or sorcery card in your graveyard gains flashback until end of
  turn. …"
- **Force Bubble**: "If damage would be dealt to you, put that many depletion counters on this
  enchantment instead. / When there are four or more depletion counters on this enchantment,
  sacrifice it. / At the beginning of each end step, remove all depletion counters from this
  enchantment."
- **Delaying Shield**: "If damage would be dealt to you, put that many delay counters on this
  enchantment instead. / At the beginning of your upkeep, remove all delay counters from this
  enchantment. For each delay counter removed this way, you lose 1 life unless you pay {1}{W}."
- **Nefarious Lich**: "If damage would be dealt to you, exile that many cards from your graveyard
  instead. If you can't, you lose the game. / If you would gain life, draw that many cards
  instead. / When this enchantment leaves the battlefield, you lose the game."
- **Crumbling Sanctuary**: "If damage would be dealt to a player, that player exiles that many
  cards from the top of their library instead."
- **Oozeavite**: "Oozeavite enters with each of a vigilance, reach, trample, and hexproof counter
  on it. These are also +1/+1 counters. / If Oozeavite would be dealt damage, instead create a 0/0
  green Ooze creature token and move a counter from Oozeavite onto it. Then if Oozeavite has no
  counters on it, sacrifice it. / {3}{G}, {T}: Move any number of counters from other creatures you
  control onto Oozeavite."
- **Underdark Beholder**: "Underdark Beholder enters the battlefield with ten eyestalk counters on
  it. / If Underdark Beholder would be dealt damage, remove that many eyestalk counters from it
  instead. If you can't, sacrifice it. / Whenever Underdark Beholder attacks, reveal cards from the
  top of your library until … You may cast it without paying its mana cost. Shuffle your library."

**Witness and fixture cards used by the new rows.**
- **Pyroclasm** {1}{R}: "Pyroclasm deals 2 damage to each creature."
- **Silver Knight** {W}{W} 2/2: "First strike, protection from red"
- **Teferi's Protection** {2}{W}: "Until your next turn, your life total can't change and you gain
  protection from everything. All permanents you control phase out. (While they're phased out,
  they're treated as though they don't exist. They phase in before you untap during your untap
  step.) / Exile Teferi's Protection."
- **Pit Fight**: "Target creature you control fights another target creature. (Each deals damage
  equal to its power to the other.)"
- **Test of Faith**: "Prevent the next 3 damage that would be dealt to target creature this turn.
  For each 1 damage prevented this way, put a +1/+1 counter on that creature."
- **Flame Slash** {R}: "Flame Slash deals 4 damage to target creature."
- **Daunting Defender** 3/3 Human Cleric: "If a source would deal damage to a Cleric creature you
  control, prevent 1 of that damage."
- **Prodigal Sorcerer**: "{T}: This creature deals 1 damage to any target."
- **Vizier of Remedies**: "If one or more -1/-1 counters would be put on a creature you control,
  that many -1/-1 counters minus one are put on it instead."
- **Doubling Season**: "If an effect would create one or more tokens under your control, it creates
  twice that many of those tokens instead. / If an effect would put one or more counters on a
  permanent you control, it puts twice that many of those counters on that permanent instead."
- **Hardened Scales**: "If one or more +1/+1 counters would be put on a creature you control, that
  many plus one +1/+1 counters are put on it instead."
- **Pariah**: "Enchant creature / All damage that would be dealt to you is dealt to enchanted
  creature instead."
- **Lightning Bolt**: "Lightning Bolt deals 3 damage to any target."
- **Furnace of Rath**: "If a source would deal damage to a permanent or player, it deals double
  that damage to that permanent or player instead."

**CR numbers**, each grep-verified in `docs/MagicCompRules.txt`: 101.4, 104.4a, 120.8, 202.2,
510.2, 608.2c, 614.1a, 614.5, 614.6, 615.1a, 615.5, 615.7, 615.12, 615.13, 616.1, 616.1e, 616.1f,
616.1g, 700.1, 701.14a, 702.16b, 702.16e, 704.5q. The ones that carry the design:
- CR 614.6: "If an event is replaced, it never happens. A modified event occurs instead…"
- CR 615.5: "…The prevention takes place at the time the original event would have happened; the
  rest of the effect takes place immediately afterward."
- CR 616.1e: "Any of the applicable replacement and/or prevention effects may be chosen."
- CR 702.16e: "Any damage that would be dealt by sources that have the stated quality to a
  permanent or player with protection is prevented."
- CR 700.1: "…The text of triggered abilities and replacement effects defines the event they're
  looking for…"
- CR 120.8: "If a source would deal 0 damage, it does not deal damage at all…"

The premise of the charter's P3 entry holds against the cards. Main's three cards and the branch's
seven cards are one rules class (CR 614.1a "instead", not CR 615.1a "prevent"). Main's predicate
and the branch's tag select disjoint parts of that class today. §2 shows this from the code.

---

## 1. Pattern Coverage

This is phase-plan mode, so the charter's Step 0.5 record is cited here and not re-run.
- `jq-1.8.1`, MTGJSON `5.3.0+20261021`, query `class/query.sh`.
- Class A, damage substitutions: `class/substitution_class.json`, **count 12**, 38 distinct
  rulings, hash `3f5d7d097714407289320c69a66af51663beebf175f6eeb177f48921597b256b`. The 12 are
  Crumbling Sanctuary, Delaying Shield, Dralnu, Lich Lord, Force Bubble, Lichenthrope, Nefarious
  Lich, Oozeavite, Phytohydra, Soul-Scar Mage, Szadek, Lord of Secrets, Undead Alchemist and
  Underdark Beholder.
- Bearing rulings for P3 (charter attribution: S1–S5, S6, S10, S12), quoted verbatim from
  `class/substitution_class.json`:
  - S1 [Soul-Scar Mage] "Soul-Scar Mage's effect isn't a prevention effect. Damage that can't be
    prevented will be replaced with -1/-1 counters." → V3.
  - S2 [Phytohydra] "Phytohydra's ability doesn't prevent damage. Damage that can't be prevented
    will still be replaced by +1/+1 counters being put on Phytohydra." → V3.
  - S3 [Delaying Shield] "Note that the damage is replaced and is not "prevented". This means it can
    replace damage that "can't be prevented"." → V3.
  - S4 [Underdark Beholder] "Underdark Beholder's second ability isn't a damage prevention effect.
    It can replace damage that can't be prevented." → class membership; V11.
  - S5 [Force Bubble] "The effect does not prevent damage, it replaces it so the damage is neither
    dealt or prevented." → V9 (CR 615.13).
  - S6 [Force Bubble] "If you control more than one, each time you take damage you can decide which
    one replaces the damage, and that one replaces all of the damage. You can't split the damage
    between them. You decide each time you take damage. Note that damage from multiple sources that
    happens at one time (such as combat damage) gives one choice, not one per source." → V13. Per
    the addendum this stays a Force Bubble row. It is not a class claim.
  - S7 [Soul-Scar Mage] "If multiple prevention and/or replacement effects are trying to apply to
    the same damage, the controller of the creature that would be dealt damage chooses the order in
    which to apply them. As examples, an opponent could choose to apply the effect of Djeru's
    Resolve and prevent the damage rather than put -1/-1 counters on it; …" → V7, V12 (the
    protection gap is exactly this choice going unasked).
  - S8 [Szadek] "If another effect would prevent Szadek's combat damage from being dealt to the
    defending player or replace it with something else, that player chooses which effect applies
    first. …" → V7, V12.
  - S9 [Phytohydra] "If another effect would prevent damage from being dealt to Phytohydra or
    replace it with something else, Phytohydra's controller chooses which effect to apply first."
    → V7, V12.
  - S10 [Soul-Scar Mage] "If an effect causes a creature you control to fight a creature an
    opponent controls, the damage is dealt simultaneously. For instance, if your 2/2 creature fights
    an opponent's 4/4 creature, your creature will be dealt 4 damage and your opponent's creature
    will have two -1/-1 counters put on it." → V6a, V6b.
  - S12 [Undead Alchemist] "If you control multiple Undead Alchemists, the multiple replacement
    abilities will have no added effect. Combat damage dealt to a player by a Zombie you control
    will be replaced only once with cards being put into that player's graveyard." → V9b.

**What P3 covers.** Every printed line that lowers to a `DamageDone` replacement with no shield,
no amount modification and a non-damage, implemented action. That is the ten class members that
parse today:
- the branch's seven: Phytohydra, Lichenthrope, Dralnu, Force Bubble, Delaying Shield, Nefarious
  Lich, Crumbling Sanctuary;
- main's three: Soul-Scar Mage, Szadek, Undead Alchemist.

They get one runtime authority and one coverage fence. The two other class members (Oozeavite and
Underdark Beholder) do not lower to a replacement at all (§2.5), so they stay unsupported. The
class is 12 printed cards, so the pattern-coverage stop (count 1) does not apply.

---

## 2. What the code does at `6f40e4dc6a` (by function)

### 2.1 Two representations, two disjoint selections

- **The tag.** `ShieldKind::Substitution` is declared in `crates/phase-ast/src/types/ability.rs`.
  Its builder is `ReplacementDefinition::substitution_shield`. The parser sets it in two places:
  - `parse_damage_to_self_instead_followup_with_provenance` ("if damage would be dealt to ~/you,
    … instead"; Phytohydra, Lichenthrope, Dralnu, Force Bubble, Delaying Shield, Nefarious Lich);
  - `parse_damage_to_player_instead_followup` ("if damage would be dealt to a player, …";
    Crumbling Sanctuary).
- **Main's predicate.** `replacement::is_damage_substitution` (def-level, private) and
  `damage_execute_substitutes_event_type` (rid-level). The predicate requires:
  - `event == DamageDone`;
  - `shield_kind` is `ShieldKind::None`;
  - `damage_modification` is `None`;
  - an `execute` whose effect is neither `Effect::Unimplemented` nor `Effect::DealDamage`.

  Main's `parse_damage_to_counters_substitution` produces exactly that shape (Soul-Scar Mage,
  Szadek, Undead Alchemist).
- The tag's seven fail main's predicate (`ShieldKind::None` is required). Main's three never carry
  the tag. So each runtime consumer sees only its own half.

### 2.2 Runtime consumers in `crates/engine/src/game/replacement.rs`

| # | Function | Reads | Effect today |
|---|---|---|---|
| R1 | `hold_damage_rider` | `shield_kind_for_rid == Some(Substitution)` | `RiderTiming::WithEvent` for the tag. Main's three are held `AfterEvent`, like a prevention rider. |
| R2 | `damage_done_applier`, the tag arm (before Branch 2) | `shield_kind_for_rid == Some(Substitution)` | Drains an earlier resident rider (unless `damage_batch_holds_riders`), stamps `last_effect_count`, returns `Prevented` / `PreventedBehindRider`. |
| R3 | `damage_done_applier` Branch 3 | `damage_execute_substitutes_event_type` | Main's three: returns `Prevented`. No earlier-rider drain, no stamp. |
| R4 | `damage_replacement_substitutes` (P1's two-representation helper; only caller `replaced_event_amount`) | both | Already unified, by design of P1 decision 3. |
| R5 | `damage_shape_prevents` (exhaustive over `ShieldKind`) | `ShieldKind::Substitution => Some(false)` | Main's three fall to `None`, so `is_damage_prevention_replacement` uses its legacy description scan. That scan answers `false` for their texts, which have no "prevent", so the outcome agrees today. |
| R6 | `replacement_spends_shared_capacity` (exhaustive) | `ShieldKind::None \| … \| ShieldKind::Substitution => false` | Both halves already return `false`. |
| R7 | candidate filter in `find_applicable_replacements` (the CR 120.8 block above the CR 615.12 gate) | `matches!(shield_kind, Substitution) && amount == 0` | Only the tag is gated. Main's three remain candidates for a 0-damage event. |
| R8 | `candidate_materiality` | `Prevention \| PreventionOneShot \| Substitution` → `Writes{Damage, NonCommuting}`; then `is_damage_substitution` → `Unconditional` | The tag is `Writes`, main's three are `Unconditional`. |

`crates/engine/src/game/coverage.rs` has its own copy, `fn is_damage_substitution`. It matches
`DamageDone | DealtDamage` and the tag, and is called from
`extract_damage_replacement_choice_feature`, which emits
`StructuralFeature::DamageSubstitutionChoiceIncomplete`. So main's three are **not** fenced today.
This is the copy the maintainer named.

### 2.3 Other `ShieldKind::Substitution` / builder sites (compiler-forced on deletion)

- `crates/engine/src/game/replacement.rs`, test
  `replaced_event_amount_latches_each_owning_kind_and_zero_prevented`, definition 3:
  `ReplacementDefinition::new(DamageDone).substitution_shield()` **with no `execute`**. Under the
  structural predicate a definition with no action is not a substitution. Index 3 therefore needs
  Phytohydra's real action, or its assertions turn false (§16, step 3).
- `crates/engine/src/game/effects/deal_damage.rs`, test
  `damage_to_player_prevention_exiles_from_that_players_library`: `.substitution_shield()` on a
  Crumbling Sanctuary definition that has an `ExileTop` action.
- Parser tests: `oracle_replacement.rs` (the Nefarious Lich test, `damage_to_self_puts_…`,
  `damage_to_you_puts_…`, `damage_to_player_exiles_…`,
  `damage_instead_lines_are_substitutions_scoped_to_their_recipient`) and `oracle_tests.rs`
  (`crumbling_sanctuary_parses_as_replacement_without_swallowed_clause`).
- No TypeScript, snapshot (`*.snap`), JSON fixture or AI/server reference names the variant
  (`git grep` over non-`.rs` files, excluding `protocol/changes`).

### 2.4 Where the drain goes (one stash path)

Both `ApplyResult` arms of the replacement application (`Modified` and
`Prevented | PreventedBehindRider`) build the drain with `mandatory_post_drain` and
`replaced_event_amount`. When the event is damage, they pass it through `hold_damage_rider`. So
re-keying R1 and R4 covers every stash.

### 2.5 Oozeavite and Underdark Beholder

- **Measured (card data at `51adf5568`, before both #9670 and this branch).**
  - Oozeavite's damage line exports as
    `Unimplemented { name: "static_structure", "Static pattern matched but line failed static
    parser: If ~ would be dealt damage, instead create …" }`, with no replacement.
  - Underdark Beholder's exports as
    `Unimplemented { name: "replacement_structure", "Replacement pattern matched but line failed
    replacement parser: If ~ would be dealt damage, remove that many eyestalk counters fro…" }`.
    Its only replacement is the `Moved` enters-with-counters one.
- **Traced (no parse run).** No parser constructor at this head builds a `DamageDone` definition
  with an `execute` and no shield for "<subject> would be dealt damage, … instead" without
  "prevent". These are every `DamageDone` constructor in `crates/phase-parser/src/parser/`, and
  what each requires:
  - `parse_replacement_line`'s "would deal" stub needs "would deal";
  - `parse_damage_modification_*` sets `damage_modification`;
  - `parse_damage_to_counters_substitution` needs " would deal ";
  - the two followups need "if damage would be dealt to …";
  - the prevention and redirection builders set a shield;
  - `oracle_effect/mod.rs` sets `damage_modification` or a prevention shield.

  `git diff 51adf5568 HEAD -- crates/phase-parser/src/parser/` adds "would be dealt damage"
  grammar only in "prevent" forms. P3 changes no parser dispatch (§7).
- **Conclusion.** Neither face carries a definition the predicate can select, before or after P3.
  Each stays unsupported through its unimplemented damage line. **No support change.** The
  structural predicate never selects them silently: an unparsed line is
  `Effect::unimplemented`, and the predicate excludes `Effect::Unimplemented`. V11 measures this
  through the coverage authority.

---

## 3. Sizing

| Unit | Behavior | Registration surfaces | Discriminating tests |
|---|---|---|---|
| **U1 One damage-substitution predicate, with the uniform fence** | The tag and builder are deleted. Every consumer (R1–R8 and coverage) reads `is_damage_substitution` / `damage_execute_substitutes_event_type`. The branch's runtime semantics (R1, R2, R5, R7) reach main's three. Main's `Unconditional` materiality (R8) reaches the branch's seven. The fence emits for every face the predicate selects, and its stated reason narrows to protection. | `phase-ast` `ShieldKind` + builder; the two parser followups; `replacement.rs` R1–R8; `coverage.rs` fence + import; protocol record | V4, V5, V6b (runtime reach onto main's three), V10 (fence re-key, which also pre-measures the coverage flips), V12 (protection witnesses) |

This is one unit under the planner's rule: one coherent mechanic, one skill-checklist pass
(`/add-replacement-effect` with the parser and pipeline-coverage checklists folded in), one
authority. The rows in §11 are discriminating tests of that authority's consumers, not
independent behaviors. Each one fails only if a consumer does not read the predicate. The fence is
the predicate's coverage consumer, and charter outcome A says "Under A, the phase is one unit".

Dependency edges: none inside the phase. P3 depends on P1's seam-1 amount authority
(`replaced_event_amount`, `ReplacedEventContext`) and on P1's single stash path, both landed.

**Scope paths** (phase-fit counting rule; directories expanded; fixtures, regenerated data and
protocol-version surfaces excluded; test files counted):
1. `crates/phase-ast/src/types/ability.rs`
2. `crates/phase-parser/src/parser/oracle_replacement.rs`
3. `crates/phase-parser/src/parser/oracle_tests.rs`
4. `crates/engine/src/game/replacement.rs`
5. `crates/engine/src/game/coverage.rs`
6. `crates/engine/src/game/effects/deal_damage.rs` (test module only; compiler-forced)
7. `crates/engine/tests/integration/damage_substitution_replacements.rs`

Not counted: `protocol/changes/<new uuid>.json`, which only moves the `game` version (registry
step).

**Count: 7 counted paths. T1: 1 unit. T2: 7 < 13. The T1∧T2 conjunction does not fire.** It still
does not fire if the protocol record is counted (8), or if the fence is counted as a second unit
(2 units over 7 paths).

---

## 4. Building Blocks

- **The predicate.** `replacement::is_damage_substitution(&ReplacementDefinition)` is the
  definition-level authority. `replacement::damage_execute_substitutes_event_type(state, rid)` is
  its rid wrapper. Its lookup is the same objects-or-`pending_damage_replacements` sentinel lookup
  that `shield_kind_for_rid` uses (`rid.source == ObjectId(0)`), so swapping R1/R2/R4 onto it
  keeps which definition is read.
- **Coverage.** Coverage already reads runtime authorities directly:
  `crate::game::replacement::as_enters_choice_type` in coverage.rs, and
  `crate::game::resolution_prompt::{effect_offers_choice, quantity_offers_up_to_choice}` in
  `damage_rider_effect_asks_a_choice`. The fence follows that pattern, with
  `crate::game::replacement::is_damage_substitution`. No coverage-side copy remains.
- **Runtime.** The branch's arm body is reused unchanged: `drain_damage_rider`, `DamageRiderDrain`,
  `damage_batch_holds_riders`, `u32_to_i32_saturating`, `ApplyResult::PreventedBehindRider`.
  `RiderTiming` is reused. `damage_shape_prevents` stays the single shape authority behind the
  CR 615.12 gate and `replaced_event_amount`.
- **Tests.** The coverage test harness `counter_rider_face`, `extract_card_features`,
  `card_face_gaps` and `card_face_coverage`. The integration harness in
  `damage_substitution_replacements.rs`: `scenario`, `instant`/`sorcery`, `cast`,
  `answer_batch_prompts`, `choose`/`choose_source`, `combat_answering`, `counters`, `damage`,
  `prevented`, `assert_at_rest`. Szadek's attack driver follows `szadek_combat_damage_to_counters`
  (`advance_to_combat`, `declare_attackers`, `combat_damage`). Protection fixtures follow the
  existing `protection_prevents_ahead_of_a_substitutions_choice`.
- **Protocol.** `node scripts/protocol-versions.mjs bump game --reason …`.

No new helper is proposed.

---

## 5. Logic Placement

- **Owner.** `is_damage_substitution` in `crates/engine/src/game/replacement.rs` (main's location
  and name). It becomes `pub(crate)` so `coverage.rs` can call it. `damage_execute_substitutes_event_type`
  stays private.
- **Delegating callers.** Each reads the predicate, and none inspects `shield_kind` to recognize
  the class:
  - `hold_damage_rider` (timing);
  - `damage_done_applier` (one arm);
  - `replaced_event_amount` (directly; the helper is deleted);
  - `damage_shape_prevents` (the `ShieldKind::None` arm);
  - the CR 120.8 candidate block;
  - `candidate_materiality` (`Unconditional`, which already reads it);
  - `coverage::extract_damage_replacement_choice_feature`.
- **Changed scope for an unchanged signature.** No signature changes. The set each consumer
  selects grows: R1, R2, R5, R7 add main's three, and R8 adds the branch's seven. Consumers whose
  output moves:
  - **(a)** two-phase batch Phase D ordering (`run_damage_batch_phases`: `held.sort_by_key((timing,
    rank))`, `settle_event_losses` before the first `AfterEvent`). Main's three now sort with the
    substitution actions;
  - **(b)** the per-recipient path's resident-drain order when a prevention rider applied first on
    the same event;
  - **(c)** `last_effect_count` after a main-card substitution (the per-instruction
    `last_effect_amount` channel reads `DamageDealt` events only (`amount_channel`), so a spell's
    "that much" is unaffected);
  - **(d)** candidate sets at 0 damage;
  - **(e)** the CR 616.1 prompt set for the branch's seven against a `Disjoint` co-candidate (§8);
  - **(f)** the coverage verdict for main's three.

---

## 6. Rust Idioms

- A deletion: `ShieldKind` returns to main's six variants and main's doc text. No new type, field
  or variant.
- Exhaustive matches stay exhaustive with no wildcard. `damage_shape_prevents` keeps its full
  `ShieldKind` match. The `ShieldKind::None` arm returns `is_damage_substitution(repl).then_some(false)`,
  and every other arm is unchanged. `replacement_spends_shared_capacity` drops the deleted arm.
- `hold_damage_rider`'s timing becomes
  `if damage_execute_substitutes_event_type(state, rid) { RiderTiming::WithEvent } else { RiderTiming::AfterEvent }`.
  The verdict is a typed enum, and the predicate is a classification query, not a stored flag.
- The unused `ShieldKind` import in `coverage.rs` is removed (clippy `-D warnings`).

---

## 7. Nom Compliance

`crates/phase-parser/src/parser/oracle_replacement.rs` and `oracle_tests.rs` change. **No
detection, dispatch or classification step changes.** The two followups keep their existing
combinators (`tag("if damage would be dealt to ")`, `alt((value(Some(SelfRef), tag("~")),
value(None, tag("you"))))`, `take_until(" instead")`, and the `alt((tag("if you can't,"),
tag("if you cannot,")))` rider). They also keep `parse_effect_chain_with_context_and_provenance`
and `parse_effect_chain_with_provenance`. The only parser edit is removing the builder call
`.substitution_shield()` from the two `ReplacementDefinition::new(ReplacementEvent::DamageDone)`
chains, so `shield_kind` stays its default `ShieldKind::None`.

No Oracle text is newly accepted, and no accepted text changes its semantic coverage.
`./scripts/check-parser-combinators.sh` is the gate.

---

## 8. Extension vs Creation

This change extends one authority and deletes a second one. It creates no new pattern.

- The runtime semantics are main's predicate carrying the branch's arm body. Charter decision 2
  lists those semantics:
  - drain an earlier rider first, unless a batch holds riders;
  - not a prevention effect (CR 615.12, CR 615.1a);
  - the CR 120.8 zero-damage gate;
  - main's `Unconditional` materiality (CR 616.1);
  - no shared prevention capacity (CR 615.7);
  - held `WithEvent` (CR 614.6).
- **Materiality.** Charter decision 2 adopts main's `Unconditional`. For the branch's seven this
  replaces `Writes{Damage, NonCommuting}`. Against every printed damage co-candidate the two give
  the same verdict:
  - a prevention shield and `PreventionOneShot` are `Writes{Damage, NonCommuting}`;
  - an amount modification is `Writes{Damage, <class>}`, and `NonCommuting` never commutes;
  - a redirect is `Unconditional`;
  - another substitution is `Unconditional`.

  They differ only against a `Disjoint` damage co-candidate, and no class-A pairing in the census
  meets one. So V7 is a parity row, labelled as such in §12.

---

## 9. Analogous Trace (Step 2)

- **Traced: main's #9670 damage substitution, end to end.**
  1. `crates/phase-parser/src/parser/oracle_replacement.rs` `parse_damage_to_counters_substitution`
     (shape: `DamageDone`, `ShieldKind::None`, action);
  2. `crates/engine/src/game/replacement.rs`: `find_applicable_replacements` (candidate filter),
     then `candidate_materiality` (`is_damage_substitution` → `Unconditional`), then
     `damage_done_applier` Branch 3, then the `Prevented` arm (`mandatory_post_drain` +
     `replaced_event_amount` + `hold_damage_rider` + `stash_post_replacement_continuation`);
  3. `crates/engine/src/game/effects/deal_damage.rs` `run_damage_batch_phases` Phase D (held
     riders);
  4. tests: `crates/engine/tests/integration/soul_scar_mage_damage_to_counters.rs`,
     `szadek_combat_damage_to_counters.rs`, `undead_alchemist_combat_damage_to_mill.rs`.
- **Traced: the branch's tag, end to end.**
  1. `crates/phase-ast/src/types/ability.rs` `ShieldKind::Substitution` /
     `ReplacementDefinition::substitution_shield`;
  2. `oracle_replacement.rs` `parse_damage_to_self_instead_followup_with_provenance`,
     `parse_damage_to_player_instead_followup`;
  3. `replacement.rs` R1, R2, R4–R8;
  4. `coverage.rs` `is_damage_substitution` → `extract_damage_replacement_choice_feature`;
  5. tests: `damage_substitution_replacements.rs`.
- **Traced: a coverage fence keyed on a runtime authority.** `coverage.rs`
  `damage_rider_effect_asks_a_choice` → `resolution_prompt::effect_offers_choice` (exhaustive
  runtime classifier read by coverage); and `coverage.rs` → `replacement::as_enters_choice_type`.

---

## 10. Variant Discoverability (`/add-engine-variant`, deletion)

This removes a variant and adds none. The gate's sibling check only:
- After the deletion `ShieldKind` is `None`, `Regeneration`, `Prevention { amount }`,
  `DamageReplacementOneShot`, `Redirection { recipient, amount, lifetime }` and
  `PreventionOneShot`. No sibling cluster is introduced, and the deletion removes the one variant
  that duplicated a shape main already classifies structurally.
- `data/engine-inventory.json` (present in the worktree, generated 2026-11-03) is not
  load-bearing for a deletion. The compiler enumerates every remaining site.

---

## 11. Verification Matrix (with Identity / Provenance)

Production entries follow `/card-test`: `cast` / `answer_batch_prompts` / `combat_answering` /
the Szadek attack driver. Each completion is `assert_at_rest` or its instruction's
`EffectResolved` / combat end. A row marked NEW is added in
`crates/engine/tests/integration/damage_substitution_replacements.rs`, unless it names another
file.

| Row | Claim (CR, ruling) | Changed seam | Test (entry → completion) | Revert-failing assertion | Positive reach-guard | Siblings / hostile fixtures | Identity / provenance |
|---|---|---|---|---|---|---|---|
| **V1** parse shape | The branch's seven lower to main's structural shape (CR 614.1a, CR 615.1a). | the two parser followups | UPDATED `oracle_replacement.rs`: the Nefarious Lich test, `damage_to_self_puts_that_many_counters_instead`, `damage_to_you_puts_that_many_counters_on_source_instead`, `damage_to_player_exiles_…`, `damage_instead_lines_are_substitutions_scoped_to_their_recipient`; `oracle_tests.rs` `crumbling_sanctuary_parses_…` | `shield_kind == ShieldKind::None` and `damage_modification.is_none()` for each of the seven (a reverted tag fails `None`) | each test still asserts `event == DamageDone` and its exact `execute` | Phantom Centaur stays `Prevention { All }` (existing); main's Soul-Scar/Szadek parser tests unchanged | the shape is the predicate's whole input; see V2 |
| **V2** predicate | One predicate classifies the class by shape. | `is_damage_substitution` (now `pub(crate)`) | NEW unit in `replacement.rs`: `is_damage_substitution_reads_the_definitions_shape` | true: Phytohydra's (`PutCounter` P1P1 `EventContextAmount` SelfRef), Szadek's (`PutCounter` + `Mill` sub), Nefarious Lich's (`ChangeZone` + `LoseTheGame` sub); false for each one-field mutation: `shield_kind = Prevention{All}` (Vigor's rider shape), `damage_modification = Some(Double)`, `execute = None`, action `Effect::unimplemented(..)`, action `DealDamage`, `event = DealtDamage`, `event = GainLife` | each false case is a one-field mutation of a true case | — | — |
| **V2b** amount latch | The latched amount reads the predicate (CR 614.6). | `replaced_event_amount` (helper deleted) | UPDATED `replaced_event_amount_latches_each_owning_kind_and_zero_prevented`: definition 3 becomes Phytohydra's real shape (no builder call) | the definition-3 rows keep `Some(4)` (Replaced, AcceptedBeforeApplying) and `None` (Modified) | existing reach-guard loop over all 10 definitions | definition 9 (unimplemented action) stays `None` | — |
| **V3** CR 615.12 | The class is not a prevention effect: damage that can't be prevented is still replaced (S1–S4). | `damage_shape_prevents` `None` arm | existing `soul_scar_mage_damage_to_counters::damage_cant_be_prevented_does_not_stop_the_substitution`, `phytohydra_replaces_damage_preventable_or_not`, `lichenthrope_…`, `player_substitutions_replace_damage_preventable_or_not` (unchanged) | **parity** (§12): with neither arm, the legacy description scan also answers `false` for these texts | each row's "preventable" half applies the substitution | Stomp's "Damage can't be prevented this turn" | — |
| **V4** CR 120.8 for main's cards | NEW `a_soul_scar_substitution_has_no_zero_damage_event_to_replace`: P0's Soul-Scar Mage; P1's Daunting Defender. P0's Prodigal Sorcerer taps to deal 1 damage to the Defender. P1 orders (CR 616.1). | CR 120.8 candidate block | activate → `assert_at_rest` | **Defender first:** no `GameEvent::ReplacementApplied { source_id: <Soul-Scar Mage> }`, 0 -1/-1 counters, 0 damage. At base, Soul-Scar applies to the 0 event (§15 U2). | **Soul-Scar first:** one -1/-1 counter on the Defender, 0 damage, the `ReplacementApplied` for Soul-Scar present, and a `ReplacementChoice` for P1 before either | existing `a_substitution_has_no_zero_damage_event_to_replace` (Nefarious Lich, Urza's Armor) stays | the gate reads the definition of the candidate's rid at scan time (live) |
| **V5** earlier rider for main's cards | NEW `an_earlier_rider_is_not_lost_to_a_soul_scar_substitution`: Flame Slash (P0) at P1's 5/5 under P1's Test of Faith, with P0's Soul-Scar Mage. P1 orders (CR 616.1). | `damage_done_applier` single arm | cast → `assert_at_rest`; drives both orders | **ToF first:** 3 prevented, then the last 1 replaced. Three +1/+1 and one -1/-1 annihilate (CR 704.5q), leaving **2 +1/+1, 0 -1/-1**, 0 damage. **Soul-Scar first:** **4 -1/-1**, 0 +1/+1, 0 damage (CR 616.1f: nothing left for ToF). Base outcome unprobed (§15 U2). | `ReplacementChoice` for P1 offering both sources | branch analog `an_earlier_rider_is_not_lost_to_a_substitution_and_reads_its_own_amount` (Phytohydra) stays | each drain latches its own `event_amount` (3 / 1): two authorities on one event |
| **V6a** S10 ruling | "your creature will be dealt 4 damage and your opponent's creature will have two -1/-1 counters". NEW `soul_scar_mage_fight_replaces_one_direction`: P0's Soul-Scar Mage; P0's Grizzly Bears 2/2 fights P1's 4/4 (Pit Fight). | fight batch (`run_object_damage_batch`) | cast → `EffectKind::Fight` resolved once, `assert_at_rest` | **parity**: the Bears are dealt 4 (in the graveyard after SBA); the 4/4 has two -1/-1 and 0 damage | the same board without Soul-Scar Mage: the 4/4 is dealt 2 | — | substitution held in the batch, authority P1 |
| **V6b** WithEvent timing for main's cards | CR 614.6 vs CR 615.5 + CR 101.4. NEW `a_soul_scar_fight_substitution_comes_with_the_event`: P0 active; P0 controls Soul-Scar Mage, Hardened Scales, Doubling Season; P0's Bears 2/2 under P0's Test of Faith; P1's 4/4 with P1's Doubling Season and Vizier of Remedies. Pit Fight: Bears vs 4/4. | `hold_damage_rider` timing | cast → answer → `assert_at_rest` | the **first** counter-order `ReplacementChoice` is **P1's** (the -1/-1 counters are the modified event), then P0's (Test of Faith's counters follow, CR 615.5). With `AfterEvent` (base timing) the APNAP sort asks P0 first. Answered P1: Doubling Season first, P0: Hardened Scales first → **4/4: 2·2−1 = 3 -1/-1, 0 damage; Bears: (3+1)·2 = 8 +1/+1, 1 damage**. Second drive, P1 Vizier first and P0 Doubling Season first → **(2−1)·2 = 2 -1/-1; 3·2+1 = 7 +1/+1**. | two counter-order prompts, one per player; Pit Fight completes once (`assert_completes_once_after_its_damage`) | existing `a_fight_holds_an_answered_shields_rider_for_apnap_order` (AfterEvent riders keep APNAP) | `HeldBatchRider.authority` = the recipient's controller (P1 for the 4/4) |
| **V7** materiality | The class is order-material against every damage replacement (S7–S9, CR 616.1). | `candidate_materiality` | existing `a_substitution_is_ordered_against_a_doubler`, `a_substitution_is_ordered_against_a_prevention_shield` (unchanged); NEW `a_substitution_and_a_redirect_ask_their_order`: P1 controls Force Bubble and a creature enchanted by P1's Pariah; P0's Lightning Bolt at P1 | **parity** (§8): `ReplacementChoice` for P1. **Bubble first:** 3 depletion counters, life 20, the creature 0 damage. **Pariah first:** the creature is dealt 3, 0 counters (CR 616.1f) | prompt candidates include both sources | — | — |
| **V8** CR 615.7 | No shared prevention capacity is spent. | `replacement_spends_shared_capacity` | existing `a_batch_keeps_a_shared_shield_for_the_players_choice` (unchanged) | parity: `ShieldKind::None` already returns `false` | — | — | — |
| **V9** CR 615.13 | Nothing counts as prevented (S5). | single arm (no `DamagePrevented`) | existing `szadek…::unblocked_combat_damage_becomes_counters_and_mill`, `soul_scar…::burn_at_opponents_creature_becomes_minus_counters`, `phytohydra_…` (unchanged) | parity: no `DamagePrevented` | counters placed | — | — |
| **V9b** CR 614.5, S12 | NEW `two_undead_alchemists_replace_a_zombies_damage_once`: P0 controls two Undead Alchemists; one attacks P1 unblocked; P1's library is noncreature cards. | applied-set re-check (CR 616.1f) | attack → answer every P1 prompt with its first candidate → combat end | **P1 mills exactly 4**, life 20, no Zombie token created (no creature card milled) | P1's graveyard count 4 | the second Alchemist's definition is the same predicate | two authorities: the event's `applied` set excludes the applied rid; the other no longer matches the mill |
| **V10** fence re-key | The fence reads the runtime predicate and covers the class uniformly (addendum; maintainer item 1). | `coverage::extract_damage_replacement_choice_feature` | NEW `coverage.rs` unit `a_damage_substitution_is_a_coverage_gap` (harness `counter_rider_face` + `extract_card_features`) | tag `structural:damage_substitution_choice_incomplete` = `Unhandled` for Soul-Scar Mage (kw Prowess), Szadek, Lord of Secrets (kw Flying), Undead Alchemist. **For these three `card_face_gaps(&face) == ["ResolverFeature:structural:damage_substitution_choice_incomplete"]`**: the fence is the drop's sole cause. The old tag-keyed copy leaves all three unfenced. | Phytohydra, Force Bubble, Nefarious Lich fenced | not fenced: Test of Faith, Furnace of Rath, Phantom Centaur | the fence reads the parsed definition (`face.replacements`), the same function the runtime reads |
| **V11** honest gaps | Oozeavite and Underdark Beholder stay unsupported, and the predicate never selects them (charter P3 decision 4). | — (measurement) | NEW `coverage.rs` unit `an_unlowered_damage_substitution_stays_unsupported` | for each: no `face.replacements` entry satisfies `replacement::is_damage_substitution`; the fence tag absent; `!card_face_coverage(&face).is_supported()` | V10's Phytohydra row, same harness | — | — |
| **V12** protection witnesses (strict-failure) | CR 702.16e prevents at the pre-replacement gate, before the CR 616.1 choice (CR 616.1e). | none (documents the fenced gap) | existing `protection_prevents_ahead_of_a_substitutions_choice` (Phytohydra); NEW `protection_prevents_ahead_of_soul_scar_mages_choice`: P0's Soul-Scar Mage; P0's Pyroclasm staged with its printed {1}{R} cost (CR 202.2 makes it red); P1's Silver Knight (protection from red) and P1's 3/3 Hill Giant. NEW `protection_prevents_ahead_of_szadeks_choice`: P1 casts Teferi's Protection, then Szadek attacks P1 unblocked. | **by the card** P1 would choose (Soul-Scar first → two -1/-1 on the Knight; protection first → none). **The engine:** no `ReplacementChoice`, the Knight has 0 counters and a `DamagePrevented` for it. Szadek: no prompt, 0 counters on Szadek, P1's library unchanged, `DamagePrevented` to P1. Each doc says so and names the fence. | the Hill Giant gets two -1/-1 and survives as a 1/1 (Soul-Scar applied in that cast); Szadek: `DamagePrevented { target: Player(P1) }` (the damage reached the gate) | — | — |
| **V13** S6 | Force Bubble's one choice across sources is Force Bubble's own ruling, not a class claim (CR 700.1). | none | existing `two_force_bubbles_take_all_of_a_combat_batch_on_one`, `a_repeated_answer_puts_a_noncombat_batch_on_one_force_bubble`: assertions unchanged, docs reworded (§16, step 7) | unchanged | unchanged | — | — |
| **V14** protocol | The deleted serialized variant gets a registry record. | `protocol/changes/` | `node scripts/protocol-versions.mjs check`; `node scripts/check-protocol-version.mjs --base <fetched main>` | both pass | — | — | — |
| **V15** suites | Main's #9670 suites and the branch's suites pass unchanged in intent (task acceptance). | all | `soul_scar_mage_damage_to_counters`, `szadek_combat_damage_to_counters`, `undead_alchemist_combat_damage_to_mill`, `adamantoise_vigor_combat_order`, `combat_hit_bookkeeping_follows_delivery`, `awe_strike_prevention`, `ultimate_magic_meteor_per_opponent_destroy`, `swans_prevention_followup`, `damage_substitution_replacements`, `unpreventable_damage_prevention_effects`, `ironscale_hydra_combat_prevention` | green | — | — | — |

**Hostile coverage per seam:**
- **The empty / no-candidate path.** V4's Defender-first leg, where the substitution has no event,
  and V11.
- **Multi-authority cases.** V5 (two replacements on one event, each latching its own amount), V6b
  (two players' held work in one batch), V9b (two copies of one replacement) and V7 (a
  substitution against a redirect).
- **Controller vs recipient.** V6b's authority is the recipient's controller.
- **The first production branch each fixture reaches:**
  - V4: the CR 120.8 candidate block (amount 0 after Defender);
  - V5: the arm's `!damage_batch_holds_riders(state) && state.has_post_replacement_drain()`;
  - V6b: `hold_damage_rider`'s timing `if`;
  - V10: `extract_damage_replacement_choice_feature`'s `if`.

**Parser semantics deferred?** No. No Oracle text is accepted with semantics deferred. Coverage
grows **more** conservative (three faces fenced).

---

## 12. Reference Readings

Rows whose expected value comes from another reading. Each reading is derived from the card text
and the CR, clause by clause:

- **V3 (parity).** S1/S2/S3 and CR 615.12 ("…any applicable prevention effects are still
  applied…"; a replacement isn't one, CR 615.1a). Damage that can't be prevented is replaced: the
  counters or the exile are put, and nothing is dealt. Measured reference: the existing rows assert
  exactly this today. They agree.
- **V6a (parity).** The S10 ruling states the reading verbatim: 2/2 dealt 4, the 4/4 gets two
  -1/-1. CR 701.14a: each deals damage equal to its power, and Soul-Scar replaces only the 2/2's
  damage to the opponent's creature (noncombat, from a source P0 controls). The measured reference
  is the same fight without Soul-Scar Mage, in which the 4/4 is dealt 2. Derived, not copied.
- **V7 (parity).** Force Bubble + Pariah: both apply to damage dealt to P1, and P1 chooses
  (CR 616.1, CR 616.1e).
  - Bubble first: the event is replaced by depletion counters (3), nothing is dealt, and Pariah has
    no event left.
  - Pariah first: the event becomes damage to the enchanted creature (3), and "dealt to you" no
    longer applies (CR 616.1f).

  The reference is the order prompt main's `Unconditional` gives. It is derived, not copied.
- **V8, V9 (parity).** CR 615.7 governs a limited prevention amount, and a substitution spends
  none. CR 615.13 triggers on prevention, and S5 says the damage "is neither dealt or prevented".
  The existing rows assert those readings.
- **V9b.** S12 and CR 614.5 say the event is replaced once. After one Alchemist applies, the event
  is a mill (CR 614.6), which the other's "would deal combat damage" no longer matches
  (CR 616.1f). So P1 mills 4, once.
- **V15.** Main's and the branch's suites stand on their own derivations (P1 plan §11–§12,
  unchanged). If one goes red, the executor classifies it under §15 U1 before any edit. A red row
  whose reading the card derives differently is a defective reference, and it is fixed in-phase
  (defective-reference route). It is never re-pinned to the new output.

Every non-parity expected value (V4, V5, V6b) is derived in its row from the cards and the CR, and
none is copied from a sibling route.

---

## 13. Skills and checklist steps (Step 1)

- **`/add-replacement-effect`.**
  - Phase 1 (types): delete the variant and the builder (step 1).
  - Phase 2 (registry): the `DamageDone` handler (`damage_done_applier`) is unchanged in
    registration. Its arms merge (step 3).
  - Phase 3 (parser): two builder calls (step 2). No routing change.
  - Phase 4: N/A.
  - Phase 5 (tests):
    - parser test (V1);
    - pipeline test with a revert-failing assertion (V4, V5, V6b);
    - engine flow (V6a, V9b, V12);
    - provenance (V5, V6b);
    - candidate count, 1 and 2+ (V4, V5, V7, V9b);
    - multi-replacement (V9b);
    - optional accept/decline: existing `soul_scar…::optional_substitution_accept_puts_counters_and_decline_deals_damage`
      and `szadek…::optional_substitution_accept_performs_counters_and_mill_decline_deals_damage`,
      unchanged, which now run the branch's arm;
    - serialized state (V14).
- **`/oracle-parser`.** No dispatch change. The combinator gate runs (§7).
- **`/add-engine-variant`.** The deletion's sibling check (§10).
- **`/add-card-data-pipeline`.** Parser output shape change: `shield_kind` for seven faces goes
  from `Substitution` to the default.
  - Audit serialized consumers: none outside Rust (§2.3).
  - The coverage honesty gate: three faces are newly fenced, and none is newly supported.
  - Snapshots: none name the variant.
  - Regenerate card data and run the coverage diff at run-level verification (orchestrator).
- **`/card-test`.** Every runtime row names its production entry and its completion boundary
  (§11).

DEFERRED: none (outcome A; the charter's P3 deferral list is empty under A).

---

## 14. Serialized surfaces and protocol

- `ShieldKind` is serialized in `ReplacementDefinition` (GameState and card data). Deleting
  `Substitution` means a state or card data written with that tag no longer decodes. Card data is
  regenerated. Saved states from the unreleased branch are the only writers of the tag.
- **Registry step**, run by the executor from the repository root:
  ```
  node scripts/protocol-versions.mjs bump game --reason "ShieldKind::Substitution is removed: '…instead' damage substitutions are recognized by their shape (replacement::is_damage_substitution) and serialize with ShieldKind::None, so a GameState or card data written with the Substitution tag no longer decodes."
  ```
  Then `node scripts/protocol-versions.mjs check` and
  `node scripts/check-protocol-version.mjs --base <fetched main>`.
- The branch's record `c85ad0f6-12d8-45cc-ae35-edf384ac7f40.json`, which added the variant, is
  kept. This follows CLAUDE.md and project-reference ("Do not … alter/delete existing records";
  "Reverting an incompatible change requires a new record") and addendum P2 Q-2's precedent. Its
  reason text becomes historical. §16 has the open question.

---

## 15. Unprobed assertions (each settled at implementation by the named check)

- **U1. Main's suites stay green** under the semantics that newly reach main's three: the
  earlier-rider drain, `WithEvent` timing, the CR 120.8 gate and the `last_effect_count` stamp.
  - Read from source: main's rows have no prevention rider resident when a substitution applies;
    their multi-source rows hold only substitution actions, so timing alone does not reorder them;
    and the spell's "that much" reads `last_effect_amount` through `amount_channel`, not
    `last_effect_count`.
  - Settled by running V15.
  - A red row is classified before any edit:
    - **(a)** the row encodes a behavior the card and CR derive differently: defective reference,
      fixed in-phase with the derived value;
    - **(b)** the fold broke a derived reading: fix the consumer;
    - **(c)** the red row is outside P3's seams: stop and return.
- **U2. V4, V5 and V6b are red at `PHASE_BASE_SHA`.** The executor runs each new row against
  `6f40e4dc6a` first. They need no API the phase deletes, so they compile at base. Run them in a
  scratch worktree, and prune it afterwards.
  - V4 and V6b: expected red. Base keeps Soul-Scar as a 0-damage candidate, and its timing is
    `AfterEvent`. If either is green at base, it is not discriminating: report it, do not ship it as
    revert-failing.
  - V5: base outcome unknown. If green, it ships as a parity row, labelled so. If red with a
    different value, its derived value stands and the fold is the fix.
- **U3. Main's three are supported at base, with no other gap.** Settled by V10's only-gap
  assertion, run at base without the fence edit (expected: gaps empty). If any of them has another
  gap at base, it is not a coverage-honesty drop of P3's, and the flip list (§16) drops it.
- **U4. Teferi's Protection's player protection is checked in the combat path at the
  pre-replacement gate.** This is the maintainer's reading: the player gate in
  `deal_damage::pre_replacement_damage_gate` and "the same gate in `combat_damage.rs`".
  `combat_damage.rs` documents that gated assignments go through `pre_replacement_damage_gate`.
  Settled by V12's Szadek row. If the combat path does not emit `DamagePrevented` there, stop and
  return. The witness would then be wrong, not the fence.
- **U5. Oozeavite and Underdark Beholder parse unchanged.** Traced and data-measured (§2.5).
  Settled by V11.

---

## 16. Step-by-step plan (one commit on `6f40e4dc6a`, then the protocol record)

Run `cargo fmt --all` directly. Check Tilt first (`tilt get uiresource clippy`), and use
`./scripts/tilt-wait.sh` / `tilt logs` when it is up. Run `./scripts/check-parser-combinators.sh`.

1. **`crates/phase-ast/src/types/ability.rs`**
   - Delete the `ShieldKind::Substitution` variant and its doc.
   - Restore the enum doc's first line to main's text: "regenerate, prevent, modify a damage
     amount, or redirect."
   - Delete `ReplacementDefinition::substitution_shield` and its doc.
   - Add a doc on `ShieldKind::None` (comment only): `/// No shield. Continuous damage
     modifications (Furnace of Rath) and CR 614.1a damage substitutions (Soul-Scar Mage,
     Phytohydra) keep this kind; the engine recognizes a substitution by its shape.`
2. **`crates/phase-parser/src/parser/oracle_replacement.rs`**
   - Remove `.substitution_shield()` from both chains
     (`parse_damage_to_self_instead_followup_with_provenance`,
     `parse_damage_to_player_instead_followup`).
   - Update the tests listed in V1: `ShieldKind::Substitution` → `ShieldKind::None`. Add
     `assert!(def.damage_modification.is_none())` where absent. In the Nefarious Lich test keep the
     message "CR 614.1a: the damage is replaced, not prevented".
   - **`crates/phase-parser/src/parser/oracle_tests.rs`**:
     `crumbling_sanctuary_parses_as_replacement_without_swallowed_clause` → `ShieldKind::None`.
3. **`crates/engine/src/game/replacement.rs`**
   - **`is_damage_substitution`.** Make it `pub(crate)`. Its doc lists the consumers: the applier,
     the stash's amount latch, the batch hold's timing, the CR 120.8 candidate block, the CR 615.12
     shape, the CR 616.1 materiality and coverage's `damage_substitution_choice_incomplete`.
     "…so they can never disagree."
   - **`hold_damage_rider`.** Timing = `if damage_execute_substitutes_event_type(state, rid) {
     RiderTiming::WithEvent } else { RiderTiming::AfterEvent }`. Keep its CR 614.6 / CR 615.5
     comment.
   - **`damage_done_applier`.**
     - Delete the tag arm, the `if matches!(shield_kind_for_rid(..), Some(ShieldKind::Substitution))`
       block before Branch 2.
     - Replace Branch 3's body (`if damage_execute_substitutes_event_type(state, rid) { return
       ApplyResult::Prevented; }`) with the deleted arm's body. That body keeps: the
       `ProposedEvent::Damage { amount, .. }` read; the earlier drain gated on
       `!damage_batch_holds_riders(state) && state.has_post_replacement_drain()`; the
       `state.last_effect_count = Some(u32_to_i32_saturating(amount))` stamp; the
       `DamageRiderDrain::Completed → Prevented` / `Paused { owner } → PreventedBehindRider`
       mapping; and the `ApplyResult::Modified(event)` fallback.
     - Merge the two comments into one Branch 3 comment citing CR 614.1a, CR 614.6, CR 615.5 and
       CR 615.13 (Soul-Scar Mage; Force Bubble's "neither dealt or prevented"; Test of Faith, then
       Phytohydra).
   - **`damage_replacement_substitutes`.** Delete it. `replaced_event_amount` calls
     `damage_execute_substitutes_event_type(state, rid)` in its place.
   - **`damage_shape_prevents`.**
     - Remove the `ShieldKind::Substitution => Some(false)` arm.
     - Split `ShieldKind::None` from the `None`-returning arm:
       `ShieldKind::None => is_damage_substitution(repl).then_some(false)`, commented "CR 614.1a +
       CR 615.1a: "instead" replaces; Phytohydra's ruling: "doesn't prevent damage"."
     - `Regeneration`, `DamageReplacementOneShot` and `Redirection { .. }` keep returning `None`.
   - **`replacement_spends_shared_capacity`.** Remove `| ShieldKind::Substitution`.
   - **The CR 120.8 candidate block.** `matches!(repl_def.shield_kind, ShieldKind::Substitution)` →
     `is_damage_substitution(repl_def)`. Keep its comment, and add Soul-Scar Mage beside Nefarious
     Lich.
   - **`candidate_materiality`.**
     - Remove `ShieldKind::Substitution` from the `Writes{Damage, NonCommuting}` arm, along with
       its two-line Phytohydra comment.
     - In the `is_damage_substitution` → `Unconditional` arm's comment, name the class (Soul-Scar
       Mage, Szadek, Phytohydra, Force Bubble) and rulings S7–S9.
   - **Test.** `replaced_event_amount_latches_each_owning_kind_and_zero_prevented` definition 3 →
     `ReplacementDefinition::new(ReplacementEvent::DamageDone).execute(AbilityDefinition::new(
     AbilityKind::Spell, Effect::PutCounter { counter_type: CounterType::Plus1Plus1, count:
     QuantityExpr::Ref { qty: QuantityRef::EventContextAmount }, target: TargetFilter::SelfRef }))`.
     Its comment stays "Phytohydra, …".
   - **New unit `is_damage_substitution_reads_the_definitions_shape`** (V2).
4. **`crates/engine/src/game/effects/deal_damage.rs`** (test module): remove `.substitution_shield()`
   from `damage_to_player_prevention_exiles_from_that_players_library`.
5. **`crates/engine/src/game/coverage.rs`**
   - Delete `fn is_damage_substitution`.
   - In `extract_damage_replacement_choice_feature`, call
     `crate::game::replacement::is_damage_substitution(repl)`.
   - Remove `ShieldKind` from the `use` list if nothing else uses it.
   - Replace the `DamageSubstitutionChoiceIncomplete` variant doc with:
     ```
     /// CR 614.1a + CR 616.1 + CR 702.16e: a damage substitution (every replacement
     /// `replacement::is_damage_substitution` selects: Phytohydra, Force Bubble, Nefarious Lich,
     /// Soul-Scar Mage, Szadek, Lord of Secrets, Undead Alchemist) whose CR 616.1 choice is not
     /// offered against protection. Protection prevents the damage at the pre-replacement gate,
     /// before the substitution is offered ("Any damage that would be dealt by sources that have the
     /// stated quality to a permanent or player with protection is prevented."), so the affected
     /// player never chooses between them (CR 616.1e): Pyroclasm at a Silver Knight under an
     /// opposing Soul-Scar Mage, or Szadek's combat damage to a player under Teferi's Protection.
     /// The card parses, and with no protection it works, so nothing else in coverage can see it.
     ```
   - Replace the `MEASURED UNHANDLED` comment above `DamageSubstitutionChoiceIncomplete =>
     FeatureSupport::Unhandled` with one naming the three protection witnesses:
     `damage_substitution_replacements` (`protection_prevents_ahead_of_a_substitutions_choice`,
     `protection_prevents_ahead_of_soul_scar_mages_choice`,
     `protection_prevents_ahead_of_szadeks_choice`: the protected recipient's player is never
     asked, and no counters are put).
   - Add unit tests `a_damage_substitution_is_a_coverage_gap` (V10) and
     `an_unlowered_damage_substitution_stays_unsupported` (V11), with verbatim Oracle text.
6. **`crates/engine/tests/integration/damage_substitution_replacements.rs`**
   - Add V4, V5, V6a, V6b, V7, V9b and the two V12 witnesses, with verbatim Oracle constants for
     Pyroclasm (already present), Silver Knight, Teferi's Protection, Flame Slash, Daunting
     Defender, Prodigal Sorcerer, Vizier of Remedies, Soul-Scar Mage, Szadek, Lord of Secrets and
     Undead Alchemist.
   - Fixture notes:
     - the Szadek witness: P1 casts Teferi's Protection with priority before combat damage;
     - the Silver Knight witness: Pyroclasm carries its printed {1}{R} cost;
     - V9b: P1's library holds noncreature cards, so Undead Alchemist's trigger stays idle.
   - Module doc, comment only: "The seven lines are …" → "The class is every `DamageDone`
     replacement `replacement::is_damage_substitution` selects: these seven, and Soul-Scar Mage,
     Szadek, Lord of Secrets and Undead Alchemist."
7. **The Force Bubble rows (V13), comment only.** In `two_force_bubbles_take_all_of_a_combat_batch_on_one`
   and `a_repeated_answer_puts_a_noncombat_batch_on_one_force_bubble`, replace "Force Bubble reports
   unsupported for it (`ResolverFeature:structural:damage_substitution_choice_incomplete`)." with:
   "This gap is Force Bubble's own ruling. CR 700.1 ("The text of triggered abilities and
   replacement effects defines the event they're looking for") keeps it from carrying over to a
   source-qualified substitution, so `damage_substitution_choice_incomplete` does not claim it for
   the class. Force Bubble reports unsupported through that fence's protection gap."
   The existing Phytohydra protection row's doc keeps its fence citation.
8. **Protocol record** (§14). Run `bump game`, `check` and `check-protocol-version --base`.
9. **Checks.**
   - Direct: `cargo fmt --all` and the parser combinator gate.
   - Tilt or fallback: `clippy`, `test-engine` (V15 suites and the new rows), `phase-parser` tests.
   - Not per-phase: card-data regeneration and the run-base-to-final parser-output comparison are
     run-level (SKILL Step 4).

---

## 17. Expected coverage flips (measured at run-level verification)

- **Drops, all coverage-honesty, accepted under outcome A:**
  - **Soul-Scar Mage**
  - **Szadek, Lord of Secrets**
  - **Undead Alchemist**

  Each newly reports `ResolverFeature:structural:damage_substitution_choice_incomplete` as its only
  gap (V10, pre-measured at unit level; §15 U3).
- **No change:**
  - Phytohydra, Lichenthrope, Dralnu, Lich Lord, Force Bubble, Delaying Shield, Nefarious Lich and
    Crumbling Sanctuary are already fenced by the tag at base, and stay fenced by the predicate.
  - Oozeavite and Underdark Beholder are unsupported through their unimplemented damage line, and
    stay so (§2.5, V11).
- **No gains.** No card becomes supported.
- **Parse-signature changes:** seven faces' `shield_kind` (`Substitution` → default). This is the
  reverse of the branch's own "Substitution (7)" parse-diff entry. No `event`, `execute` or filter
  changes.

---

## 18. For the orchestrator / human

- **PR text (run-level, suggested wording).**
  - pr-body §1, "…They are now `DamageDone` replacements with a new `ShieldKind::Substitution`." →
    "…They are now `DamageDone` substitutions, recognized by main's structural predicate
    (`replacement::is_damage_substitution`), which also reads Soul-Scar Mage, Szadek and Undead
    Alchemist."
  - The fence bullets (pr-body lines 68 and 228) → "A damage substitution reports
    `damage_substitution_choice_incomplete`: protection prevents ahead of its CR 616.1 choice
    (CR 702.16e). Force Bubble, Phytohydra, Lichenthrope, Crumbling Sanctuary, Delaying Shield,
    Soul-Scar Mage, Szadek, Lord of Secrets and Undead Alchemist become unsupported. Force Bubble's
    one choice across simultaneous sources is its own ruling's gap, not claimed for the class
    (CR 700.1)." Nefarious Lich and Dralnu already report other gaps.
  - The parse-diff "Substitution (7)" entry now reads `shield_kind` Prevention{All} → default
    (None).
  - The protocol list gains the new record.
- **Human decision (minor).** The brief calls for a new protocol record. That leaves the branch's
  unreleased `c85ad0f6` (adds the variant) beside a record that removes it: two `game` bumps for a
  variant main never had. `check-protocol-version --base` validates only base records, so deleting
  `c85ad0f6` would also pass. But it contradicts the repository's "do not delete existing records"
  rule, so this plan keeps it, per addendum P2 Q-2's precedent. Flag only if the maintainer prefers
  the other way.
- **Riskiest parts.**
  1. Moving the branch's arm body (earlier-rider drain, `last_effect_count` stamp) and `WithEvent`
     timing onto main's three, against main's suites (U1).
  2. The Teferi's Protection combat witness's entry and its gate assumption (U4).
  3. Whether V4/V6b discriminate at base (U2).
