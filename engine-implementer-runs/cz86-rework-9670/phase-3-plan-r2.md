# Phase 3 plan r2: one substitution predicate, and the fence decision (outcome A)

Phase-plan mode. Run `cz86-rework-9670`; charter `phase-charter` (frozen), P3 entry only. Binding
inputs: `addenda/phase-3` (outcome A; claim narrowed to protection; no uniform one-choice claim
under CR 700.1; the fence reads the runtime predicate), `maintainer-answer-2026-11-17T1351.md`
item 1, `phase-1-accepted-summary.md`, `phase-2-accepted-summary.md`.

This is r2 of the P3 plan. It keeps r1 except where it closes the four findings of
`phase-3-plan-review-r1.md`:

| Finding | Where it closes |
|---|---|
| 1 (behavior, high): substitute before an earlier rider (CR 614.6 + CR 615.5) | §2.6 (source evidence), §5, §8, §16 step 3, §11 V5–V5e, §12, §18 **charter conflict C-1** |
| 2 (behavior, med): V9b both Alchemists asked, printed subtypes | §11 V9b, V4; §16 step 6 |
| 3 (behavior, med): V8 real substitution witness; V11 per-clause diagnostic | §11 V8, V11; §12; §15 U6, U7 |
| 4 (text, low): sizing rationale | §3, applied verbatim |

- `PHASE_BASE_SHA` = `6f40e4dc6a` (branch `engine/unpreventable-damage-riders`). Every "today"
  claim below was read at this commit, by function name.
- **What P2 did not do** (it was trimmed by the user). Combat rider pauses are still settled inside
  the combat batch under `damage_replacement_choice_unasked_in_combat`. The in-leg earlier-rider
  drain and the Phase D aggregate riders are still auto-settled there. Guards are still read once
  per pass entry, not carried across a park. P3 does not change any of that behavior. The in-leg
  earlier-rider drain moves from the applier arm into the stash helper (§16 step 3) with its
  combat behavior unchanged, and the fence and its machinery stay as P1 left them.
- Attempt history: pre-charter plan r1 had 2 behavior findings and 1 text finding. Charter r1 had
  2 corrections. P1 was accepted (2 plan rounds, 0 impl-review findings). P2 ran a long loop, was
  trimmed to U3 (Channel Harm's "may"), and was accepted. P3 plan r1 had 3 behavior findings and 1
  text finding (design round 1); this is its revision. Budget: standard. No expansion case is open.
- **Probe status.** The planning worktree is read-only and nothing was built or run. No runtime
  claim here was probed by running code. Each claim is read from source and named by function, or
  is listed in §15 as a claim the executor settles before the edit that depends on it. One
  parse-output fact was **measured from data**: the card-data export at commit `51adf5568`
  (`client/public/card-data.json`, generated 2026-10-22, before both #9670 and this branch).

---

## 0. Premise (Step 0)

Oracle text below is read from MTGJSON `data/mtgjson/AtomicCards.json`, version `5.3.0+20261021`
(the charter's version), not from the brief. Each new fixture card's text and printed type line
was re-checked against `client/public/card-data.json` for r2.

**The class (charter class A).**
- **Soul-Scar Mage** (Creature — Human Wizard): "Prowess / If a source you control would deal
  noncombat damage to a creature an opponent controls, put that many -1/-1 counters on that
  creature instead."
- **Szadek, Lord of Secrets** (Legendary Creature — Vampire, 5/5): "Flying / If Szadek would deal
  combat damage to a player, instead put that many +1/+1 counters on Szadek and that player mills
  that many cards."
- **Undead Alchemist** (Creature — Zombie, 4/2): "If a Zombie you control would deal combat damage
  to a player, instead that player mills that many cards. / Whenever a creature card is put into an
  opponent's graveyard from their library, exile that card and create a 2/2 black Zombie creature
  token."
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
- **Mending Hands**: "Prevent the next 4 damage that would be dealt to any target this turn."
- **Flame Slash** {R}: "Flame Slash deals 4 damage to target creature."
- **Daunting Defender** (Creature — Human Cleric, 3/3): "If a source would deal damage to a Cleric
  creature you control, prevent 1 of that damage."
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
- **Hurloon Battle Hymn** (existing fixture): "Kicker {W} … Hurloon Battle Hymn deals 4 damage to
  target creature or planeswalker. If this spell was kicked, you gain 4 life."

**CR numbers**, each grep-verified in `docs/MagicCompRules.txt`: 101.4, 104.1, 104.4a, 120.8,
202.2, 510.2, 608.2c, 609.7b, 614.1a, 614.5, 614.6, 615.1a, 615.5, 615.7, 615.12, 615.13, 616.1,
616.1e, 616.1f, 616.1g, 700.1, 701.14a, 702.16b, 702.16e, 704.5q. The ones that carry the design:
- CR 614.6: "If an event is replaced, it never happens. A modified event occurs instead…"
- CR 615.5: "Some prevention effects also include an additional effect, which may refer to the
  amount of damage that was prevented. The prevention takes place at the time the original event
  would have happened; the rest of the effect takes place immediately afterward."
- CR 616.1e: "Any of the applicable replacement and/or prevention effects may be chosen."
- CR 616.1f: "Once the chosen effect has been applied, this process is repeated (taking into account
  only replacement or prevention effects that would now be applicable) until there are no more left
  to apply."
- CR 702.16e: "Any damage that would be dealt by sources that have the stated quality to a
  permanent or player with protection is prevented."
- CR 700.1: "…The text of triggered abilities and replacement effects defines the event they're
  looking for…"
- CR 120.8: "If a source would deal 0 damage, it does not deal damage at all…"
- CR 609.7b: "…If for any reason the shield prevents no damage or replaces no damage, the shield
  isn't used up."
- CR 104.1: "A game ends immediately when a player wins, when the game is a draw, or when the game
  is restarted."

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
    Resolve and prevent the damage rather than put -1/-1 counters on it; …" → V5, V7, V12 (the
    protection gap is exactly this choice going unasked).
  - S8 [Szadek] "If another effect would prevent Szadek's combat damage from being dealt to the
    defending player or replace it with something else, that player chooses which effect applies
    first. For example, the player can choose to first have Mending Hands prevent 4 of the damage
    and then apply Szadek's ability so that it gets only one counter." → V7, **V8**, V12.
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
  - [Undead Alchemist, second ruling] "Whenever a creature card is put into an opponent's graveyard
    from their library, the triggered ability of each Undead Alchemist you control will trigger. …"
    → V9b's library holds no creature card, so neither trigger fires.
- Bearing class-B rulings for the order of a substitute and an earlier rider (finding 1), quoted
  verbatim from `class/prevention_rider_class.json`:
  - [Acolyte's Reward] "If the amount of damage that would be dealt to the first target is in excess
    of the amount of damage that Acolyte's Reward would prevent, the source deals its excess damage
    to the first target at the same time that the rest of it is prevented. Then Acolyte's Reward
    deals its damage."
  - [Refraction Trap] "If the chosen source would deal damage, Refraction Trap prevents 3 of that
    source's damage and the source deals its excess damage (if any) at the same time. Immediately
    afterward, as part of that same prevention effect, Refraction Trap deals its damage. This
    happens before state-based actions are checked, and before the spell or ability that caused
    damage to be dealt resumes its resolution."
  - [Test of Faith] "The +1/+1 counters are put onto the creature at the same time the damage is
    prevented. If a 1/1 creature would be dealt 6 damage, 3 damage is prevented and three +1/+1
    counters are put on the creature. The creature ends up as a 4/4 creature with 3 damage marked on
    it."

  Reading: the excess, which is the event that the prevention modified, happens with the
  prevention. The rider's additional effect follows it ("Then …", "Immediately afterward"), before
  state-based actions. When a substitution replaces the excess (CR 614.6), the substitute is that
  modified event, so it comes first and the rider follows. Test of Faith's ruling places the
  counters before state-based actions (the 1/1 survives as a 4/4). It does not order them against
  the excess, so it agrees with CR 615.5.

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
| R2 | `damage_done_applier`, the tag arm (before Branch 2) | `shield_kind_for_rid == Some(Substitution)` | Drains a resident rider first (unless `damage_batch_holds_riders`), stamps `last_effect_count`, returns `Prevented` / `PreventedBehindRider`. See §2.6 for the order this produces. |
| R3 | `damage_done_applier` Branch 3 | `damage_execute_substitutes_event_type` | Main's three: returns `Prevented`. No custody of a resident rider, no stamp. See §2.6. |
| R4 | `damage_replacement_substitutes` (P1's two-representation helper; only caller `replaced_event_amount`) | both | Already unified, by design of P1 decision 3. |
| R5 | `damage_shape_prevents` (exhaustive over `ShieldKind`) | `ShieldKind::Substitution => Some(false)` | Main's three fall to `None`, so `is_damage_prevention_replacement` uses its legacy description scan. That scan answers `false` for their texts, which have no "prevent", so the outcome agrees today. |
| R6 | `replacement_spends_shared_capacity` (exhaustive) | `ShieldKind::None \| … \| ShieldKind::Substitution => false` | Both halves already return `false`. |
| R7 | candidate filter in `find_applicable_replacements` (the CR 120.8 block above the CR 615.12 gate) | `matches!(shield_kind, Substitution) && amount == 0` | Only the tag is gated. Main's three remain candidates for a 0-damage event. |
| R8 | `candidate_materiality` | `Prevention \| PreventionOneShot \| Substitution` → `Writes{Damage, NonCommuting}`; then `is_damage_substitution` → `Unconditional` | The tag is `Writes`, main's three are `Unconditional`. |
| R9 | `apply_single_replacement`'s `Prevented \| PreventedBehindRider` arm (the stash) | `hold_damage_rider`, `damage_rider_placement(state, behind)` | Builds the substitute's drain (`mandatory_post_drain`, `replaced_event_amount`) and installs it with `ResidentDrainPolicy::KeepResident`. It pushes `ReplacementApplied` after the stash. |

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
- `ApplyResult::PreventedBehindRider` and its match arms are all in `replacement.rs` (constructed
  only by R2; matched in R9 and at five `Err(ApplyResult::Modified(_) |
  ApplyResult::PreventedBehindRider(_))` sites). `PausedRider` and `damage_rider_placement` are
  also used only in `replacement.rs`.

### 2.4 Where the drain goes (one stash path)

Both `ApplyResult` arms of the replacement application (`Modified` and
`Prevented | PreventedBehindRider`) build the drain with `mandatory_post_drain` and
`replaced_event_amount`. When the event is damage, they pass it through `hold_damage_rider`. So
re-keying R1 and R4 covers every stash. The order of a substitute against a resident rider is
decided in R9 after r2 (§2.6, §16 step 3), because R9 is the one place that holds the
substitute's drain.

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
  through the coverage authority, clause by clause.

### 2.6 The order of a substitute and an earlier rider today (finding 1 evidence)

The event: Flame Slash's 4 damage to P1's creature, which is under Test of Faith, with P0's
Soul-Scar Mage on the battlefield. P1 orders Test of Faith first (CR 616.1). The pipeline:
1. Test of Faith's shield (Branch 2, `PreventionAmount::Next(3)`) prevents 3 and returns
   `Modified` with 1 damage. `apply_single_replacement`'s `Modified` arm stashes Test of Faith's
   rider with `event_amount = 3` (`replaced_event_amount`). The stash calls
   `stash_post_replacement_continuation`, which calls `install_post_replacement_drain(..,
   ResidentDrainPolicy::KeepResident)`. The rider is now the **Ready resident** of the active
   post-replacement frame.
2. The substitution applies to the remaining 1 (CR 616.1f).

What each side does at step 2:
- **The branch's arm (R2, `replacement.rs` ~3426–3452).** It runs `drain_damage_rider` on the
  resident rider **inside the applier**, before the substitute's drain exists. Then it stamps and
  returns `Prevented`. R9 then stashes the substitute, and the caller's rider drain runs it. Order
  produced: **Test of Faith's +1/+1 counters, then the substitute's counters.** This is the
  reverse of CR 615.5. The arm's own comment states why: "an earlier effect's rider takes place
  now, ahead of the stamp below, because the drain stack keeps one Ready rider and both read
  `last_effect_count`." P1 removed the second reason, because each drain now latches its own
  `event_amount` and the dispatch stamps it (`dispatch_pending_post_replacement_effect`). The
  first reason remains: the stack holds at most one Ready entry
  (`PostReplacementDrainStack::install`, KeepResident collides on `has_ready`). If the rider
  pauses (Test of Faith's counters under Hardened Scales and Doubling Season), the arm returns
  `PreventedBehindRider`, and R9 queues the substitute behind the paused rider
  (`install_post_replacement_drain_behind_paused`). The order is still rider first.
- **Main's arm (R3, Branch 3 ~3746).** It returns `Prevented` with no custody. R9 (~12731) builds
  the substitute's drain and installs it with KeepResident. Test of Faith's rider is the Ready
  resident, so `PostReplacementDrainStack::install` returns `false` and **the substitute is
  discarded**. `stash_post_replacement_continuation` ignores the return. Outcome at base for
  Soul-Scar Mage: three +1/+1 counters and **no** -1/-1 counter. The event was replaced, so the 1
  damage isn't dealt either. Main's own source at `91639fdf02` has the same Branch 3, and its
  `stash_post_replacement_continuation` also installs with KeepResident.
- **Two-phase batches (fight, multi-source)** hold both drains (`hold_damage_rider`) and run them
  in Phase D sorted by `RiderTiming`, with `WithEvent` before `AfterEvent`
  (`run_damage_batch_phases`). That order is already right for the tag, and it is right for main's
  three once R1 reads the predicate (V6b).
- **Combat legs.** The ledger never holds riders in combat (`held_riders: None` in the combat
  pass), so R2's in-leg drain runs. In combat, `drain_damage_rider` delegates to
  `drain_combat_damage_rider`, which settles every pause inside the batch
  (`settle_combat_rider_pauses`: `Order` takes candidate 0, `May` declines, and `OwnPrompt` is
  abandoned, "it is cleared and the rider ends there").

So the defect is in the per-recipient path and the answered-choice resume path, outside a
two-phase batch. There the branch's arm reverses the order and main's arm loses the substitute.

---

## 3. Sizing

| Unit | Behavior | Registration surfaces | Discriminating tests |
|---|---|---|---|
| **U1 One damage-substitution predicate, with the uniform fence** | The tag and builder are deleted. Every consumer (R1–R9 and coverage) reads `is_damage_substitution` / `damage_execute_substitutes_event_type`. The branch's runtime semantics (R1, R5, R7) reach main's three. Main's `Unconditional` materiality (R8) reaches the branch's seven. Where a substitution's action is stashed (R9), a resident earlier rider is held, the substitute runs first, and the rider follows with its own latched amount (CR 614.6 + CR 615.5; outside combat). The fence emits for every face the predicate selects, and its stated reason narrows to protection. | `phase-ast` `ShieldKind` + builder; the two parser followups; `replacement.rs` R1–R9 (+ `ApplyResult::PreventedBehindRider` deletion); `coverage.rs` fence + import; protocol record | V4, V5–V5e, V6b (runtime reach onto main's three), V10 (fence re-key, which also pre-measures the coverage flips), V12 (protection witnesses) |

Outcome A is one unit under the frozen charter. The predicate fold and uniform fence share a classification authority. The verification rows separately exercise classification, the runtime behavior selected by it, and the protection gap that justifies the fence.

The order of a substitute and an earlier rider (finding 1) is runtime behavior that classification
selects, in R9. It changes no path below. Counted as its own unit, the phase would be 2 units over
7 paths, and the T1∧T2 conjunction still would not fire.

Dependency edges: none inside the phase. P3 depends on P1's seam-1 amount authority
(`replaced_event_amount`, `ReplacedEventContext`) and on P1's single stash path, both landed.

**Scope paths** (phase-fit counting rule; directories expanded; fixtures, regenerated data and
protocol-version surfaces excluded; test files counted). Re-checked after r2's design change: the
order fix lives in `replacement.rs` and its rows in the integration file, both already listed. No
path is added.
1. `crates/phase-ast/src/types/ability.rs`
2. `crates/phase-parser/src/parser/oracle_replacement.rs`
3. `crates/phase-parser/src/parser/oracle_tests.rs`
4. `crates/engine/src/game/replacement.rs`
5. `crates/engine/src/game/coverage.rs`
6. `crates/engine/src/game/effects/deal_damage.rs` (test module only; compiler-forced)
7. `crates/engine/tests/integration/damage_substitution_replacements.rs`

Not counted: `protocol/changes/<new uuid>.json`, which only moves the `game` version (registry
step). Every path is inside the charter's P3 scope rule. Two doc comments in
`crates/engine/src/game/effects/mod.rs` (the `dispatch_resident_post_replacement_work` doc and the
`queued_behind` comment) name "Dralnu, Lich Lord's sacrifice behind Test of Faith's counters".
That example is still produced, because of the cross-recipient queue
(`work_queued_behind_a_paused_rider_runs_in_the_order_it_was_queued`, Command Zone #172), so they
stay as they are and the file is not touched.

**Count: 7 counted paths. T1: 1 unit. T2: 7 < 13. The T1∧T2 conjunction does not fire.** It still
does not fire if the protocol record is counted (8), or if the fence or the order is counted as a
second unit (2 units over 7 paths).

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
- **Runtime order (finding 1).** Only existing primitives are used, and none is added:
  - `GameState::take_ready_post_replacement_drain` (pub(crate): "take the active resident drain
    while its continuation is still ready … The caller owns the drain and re-installs it … when it
    is due");
  - `stash_post_replacement_continuation` with `behind: Option<PausedRider>`;
  - `GameState::install_post_replacement_drain_behind_paused` (FIFO beneath a paused resident,
    reached through the stash);
  - `drain_damage_rider` → `DamageRiderDrain::{Completed, Paused { owner }}`;
  - `drain_combat_damage_rider` (unchanged in-leg behavior);
  - `damage_batch_holds_riders`;
  - the existing answer-side retire path in `effects/mod.rs` (`queued_behind` →
    `dispatch_resident_post_replacement_work`), which runs work queued behind a paused dispatch in
    the answering action.

  `RiderTiming` and `ApplyResult::Prevented` are reused. `ApplyResult::PreventedBehindRider` is
  deleted, because no applier pauses inside the pipeline any more.
  `damage_shape_prevents` stays the single shape authority behind the CR 615.12 gate and
  `replaced_event_amount`.
- **Tests.** The coverage test harness: `counter_rider_face`, `extract_card_features`,
  `card_face_gaps`, `card_face_coverage` (with `CoverageGap { handler, source_text, .. }`). The
  integration harness in `damage_substitution_replacements.rs`: `scenario`, `instant`/`sorcery`,
  `cast`, `answer`, `answer_batch_prompts`, `choose`/`choose_source`/`choose_shield`,
  `answer_counter_orders`, `combat_events`, `combat_answering`, `source_set`, `counters`,
  `plus_counters`, `damage`, `prevented`, `lands` (returns the land ids), `zone_prompt`,
  `assert_at_rest`. Szadek's attack driver follows `szadek_combat_damage_to_counters`. Protection
  fixtures follow the existing `protection_prevents_ahead_of_a_substitutions_choice`. Printed
  subtypes use `CardBuilder::with_subtypes`, as `undead_alchemist_combat_damage_to_mill` does
  (`add_creature_from_oracle` adds the Creature core type only).
- **Protocol.** `node scripts/protocol-versions.mjs bump game --reason …`.

One private helper is added in `replacement.rs`: `stash_damage_substitute`, the stash for a
substitution's action (§5). It is the single place where a substitute is ordered against a
resident rider, and it composes the primitives above.

---

## 5. Logic Placement

- **Owner.** `is_damage_substitution` in `crates/engine/src/game/replacement.rs` (main's location
  and name). It becomes `pub(crate)` so `coverage.rs` can call it. `damage_execute_substitutes_event_type`
  stays private.
- **Delegating callers.** Each reads the predicate, and none inspects `shield_kind` to recognize
  the class:
  - `hold_damage_rider` (timing);
  - `damage_done_applier` (one arm: classification and the stamp only);
  - `apply_single_replacement`'s `Prevented` arm → `stash_damage_substitute` (order);
  - `replaced_event_amount` (directly; the helper is deleted);
  - `damage_shape_prevents` (the `ShieldKind::None` arm);
  - the CR 120.8 candidate block;
  - `candidate_materiality` (`Unconditional`, which already reads it);
  - `coverage::extract_damage_replacement_choice_feature`.
- **Why the order lives at the stash (R9), not in the applier.** The applier decides what happens to
  the event. It never holds the substitute's drain, because R9 builds that drain from
  `mandatory_post_effect` and `replaced_event_amount`. Ordering two drains is a stash decision. R9
  already makes the other stash decisions (`hold_damage_rider`, `damage_rider_placement`). So the
  applier arm returns `Prevented` (main's Branch 3 plus the branch's stamp), and
  `stash_damage_substitute` takes over the custody the branch's arm had.
- **`stash_damage_substitute(state, substitute, placement, events)`.** It is reached only when the
  drain was not held (`hold_damage_rider` returned it), so no two-phase batch is settling.
  - **Outside combat, with no placement behind a paused rider, and a Ready resident:**
    1. take the resident (`take_ready_post_replacement_drain`): the earlier rider, with its latched
       `event_amount`;
    2. stash the substitute and run it now (`drain_damage_rider`). This is the modified event,
       happening when the damage would have (CR 614.6);
    3. if the game ended, stop (CR 104.1);
    4. re-stash the earlier rider: as the Ready resident when the substitute completed, so the
       caller's existing rider drain runs it immediately afterward (CR 615.5); or behind the
       substitute's paused frame (`Some(PausedRider { owner })`) when the substitute left its
       prompt standing, so the answering action runs it (CR 615.5 + CR 608.2c).
  - **In combat** (`state.combat_prevention_tally.is_some()`): the branch's in-leg behavior,
    unchanged. A Ready resident is run first through `drain_combat_damage_rider` (settled in the
    batch), and then the substitute is stashed for the leg's continuation drain, which asks its
    prompts. It is marked strict-failure (§16 step 3). Running the substitute first in combat would
    put its own prompt into the in-batch settlement, which abandons an `OwnPrompt` (Dralnu's
    sacrifice: "nothing is sacrificed", `dralnus_sacrifice_choice_is_cleared_inside_the_batch`).
    Today that prompt is asked at the leg. The right combat order needs the in-leg rider park that
    P2 dropped to Command Zone #267. See §18.
  - **Otherwise** (no resident, or a placement behind another recipient's paused rider): stash as
    today. The queue order behind a paused rider of another recipient stays Command Zone #172,
    already disclosed in `damage_rider_placement`'s doc and `every_rider_queued_behind_a_paused_rider_runs`.
- **Why the post-pipeline state is safe for every caller.** After the helper, the stack holds
  exactly what a lone prevention rider leaves: one Ready resident rider, or a paused frame with
  work queued behind it. Every caller of the per-recipient path already handles both states:
  - `apply_damage_to_target`'s `Prevented` drain (`deal_damage.rs` ~656);
  - the answered-choice resume (`handle_replacement_choice_inner`'s `Prevented` arm, which
    surfaces a live prompt or drains the resident);
  - `with_damage_batch` per-recipient placement (`ledger.paused_rider`, set by `drain_damage_rider`
    on a pause).

  The inline dispatch inside the pipeline happens at the same depth at which the branch's arm
  dispatched the earlier rider today (inside `apply_single_replacement`). That path is exercised at
  base by `an_earlier_riders_paused_counters_finish_before_the_substitution`, so no new reentrancy
  surface is introduced (§15 U8).
- **Changed scope for an unchanged signature.** No public signature changes. Private:
  `damage_rider_placement` loses its `behind` parameter, which `PreventedBehindRider` used to
  feed. The set each consumer selects grows: R1, R5, R7 and R9's order add main's three, and R8
  adds the branch's seven. Consumers whose output moves:
  - **(a)** two-phase batch Phase D ordering (`run_damage_batch_phases`: `held.sort_by_key((timing,
    rank))`, `settle_event_losses` before the first `AfterEvent`). Main's three now sort with the
    substitution actions;
  - **(b)** the per-recipient path and the resume path: when a rider is resident as a substitution
    applies, the substitute now runs first and the rider follows. This applies to the branch's
    seven (a defective reference, converted in V5d/V5e) and to main's three (whose substitute was
    discarded);
  - **(c)** `last_effect_count` after a main-card substitution. It is stamped by the arm and then
    re-stamped by each drain's dispatch, so after an earlier rider it ends at the rider's amount
    rather than the substitute's. The per-instruction `last_effect_amount` channel reads
    `DamageDealt` events only (`amount_channel`), so a spell's "that much" is unaffected;
  - **(d)** candidate sets at 0 damage;
  - **(e)** the CR 616.1 prompt set for the branch's seven against a `Disjoint` co-candidate (§8);
  - **(f)** the coverage verdict for main's three;
  - **(g)** event order: R9 pushes `ReplacementApplied` before the stash, so a substitute run
    inline logs after its own `ReplacementApplied`. Stashing emits no events, so no other order
    moves.

---

## 6. Rust Idioms

- A deletion: `ShieldKind` returns to main's six variants and main's doc text. No new type, field
  or variant. `ApplyResult::PreventedBehindRider` is deleted, and its matches stay exhaustive
  (`Err(ApplyResult::Modified(_))` at the five `Err` sites).
- Exhaustive matches stay exhaustive with no wildcard. `damage_shape_prevents` keeps its full
  `ShieldKind` match. The `ShieldKind::None` arm returns `is_damage_substitution(repl).then_some(false)`,
  and every other arm is unchanged. `replacement_spends_shared_capacity` drops the deleted arm.
- `hold_damage_rider`'s timing becomes
  `if damage_execute_substitutes_event_type(state, rid) { RiderTiming::WithEvent } else { RiderTiming::AfterEvent }`.
  The verdict is a typed enum, and the predicate is a classification query, not a stored flag.
- `stash_damage_substitute` matches `DamageRiderDrain` exhaustively
  (`Completed => None`, `Paused { owner } => Some(PausedRider { owner })`). Its custody is an
  `Option<PostReplacementDrain>` from `take_ready_post_replacement_drain` (`.then(..).flatten()`
  over the guard), not a flag.
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

- The runtime semantics are main's predicate carrying the branch's behavior. Charter decision 2
  lists them. r2 differs from the charter's literal first bullet (**charter conflict C-1**, §18):
  - ~~drain an earlier rider first, unless a batch holds riders~~ → **keep custody of an earlier
    rider and run it immediately after the substitute (CR 614.6 + CR 615.5), unless a batch holds
    riders (then Phase D's `RiderTiming` order already does this); in combat the earlier-first
    in-leg drain stays, marked strict-failure (Command Zone #267)**;
  - not a prevention effect (CR 615.12, CR 615.1a);
  - the CR 120.8 zero-damage gate;
  - main's `Unconditional` materiality (CR 616.1);
  - no shared prevention capacity (CR 615.7);
  - held `WithEvent` (CR 614.6).
- **One order rule for both paths.** The two-phase batch already orders a substitute ahead of a
  rider (`RiderTiming::WithEvent` before `AfterEvent`, Phase D). The single-event path now gives
  the same order with the stack's own custody primitive. That makes one rule, "the modified event,
  then the additional effect", on both paths, instead of the branch's batch-right and
  single-event-reversed pair.
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
     `replaced_event_amount` + `hold_damage_rider` + `stash_post_replacement_continuation`, whose
     KeepResident install drops the substitute when a rider is Ready resident, §2.6);
  3. `crates/engine/src/game/effects/deal_damage.rs` `run_damage_batch_phases` Phase D (held
     riders);
  4. tests: `crates/engine/tests/integration/soul_scar_mage_damage_to_counters.rs`,
     `szadek_combat_damage_to_counters.rs`, `undead_alchemist_combat_damage_to_mill.rs`.
- **Traced: the branch's tag, end to end.**
  1. `crates/phase-ast/src/types/ability.rs` `ShieldKind::Substitution` /
     `ReplacementDefinition::substitution_shield`;
  2. `oracle_replacement.rs` `parse_damage_to_self_instead_followup_with_provenance`,
     `parse_damage_to_player_instead_followup`;
  3. `replacement.rs` R1, R2, R4–R9 (R2's earlier drain → `PreventedBehindRider` → R9's
     `damage_rider_placement(state, behind)` → `install_post_replacement_drain_behind_paused`);
  4. `coverage.rs` `is_damage_substitution` → `extract_damage_replacement_choice_feature`;
  5. tests: `damage_substitution_replacements.rs`.
- **Traced: work queued behind a paused dispatch** (the mechanism r2's paused-substitute case
  reuses in the other direction): `PostReplacementDrainStack::install_behind_paused_resident`
  (FIFO) → `effects/mod.rs` `ResolutionFrame::PostReplacement` arm (`retired_paused_dispatch`,
  `queued_behind`) → `dispatch_resident_post_replacement_work` (each Ready entry that becomes
  resident, until a pause). Test: `work_queued_behind_a_paused_rider_runs_in_the_order_it_was_queued`.
- **Traced: a coverage fence keyed on a runtime authority.** `coverage.rs`
  `damage_rider_effect_asks_a_choice` → `resolution_prompt::effect_offers_choice` (exhaustive
  runtime classifier read by coverage); and `coverage.rs` → `replacement::as_enters_choice_type`.

---

## 10. Variant Discoverability (`/add-engine-variant`, deletion)

This removes two variants and adds none. The gate's sibling check only:
- After the deletion `ShieldKind` is `None`, `Regeneration`, `Prevention { amount }`,
  `DamageReplacementOneShot`, `Redirection { recipient, amount, lifetime }` and
  `PreventionOneShot`. No sibling cluster is introduced, and the deletion removes the one variant
  that duplicated a shape main already classifies structurally.
- `ApplyResult` returns to `Modified(ProposedEvent)` and `Prevented`. That is main's shape.
- `data/engine-inventory.json` (present in the worktree, generated 2026-11-03) is not
  load-bearing for a deletion. The compiler enumerates every remaining site.

---

## 11. Verification Matrix (with Identity / Provenance)

Production entries follow `/card-test`: `cast` / `answer` / `answer_batch_prompts` /
`combat_events` / `combat_answering` / the Szadek attack driver. Each completion is
`assert_at_rest` or its instruction's `EffectResolved` / combat end. A row marked NEW is added in
`crates/engine/tests/integration/damage_substitution_replacements.rs`, unless it names another
file. Every fixture card with a printed subtype the row depends on is staged with it
(`with_subtypes`).

"#174" below is the existing strict-failure: Test of Faith's counters parse as one counter at a
time (Command Zone #174), so the engine puts N one-counter events where the card puts one event of
N. Rows keep the sibling tests' convention: each such row states the card's value and the engine's
value, and asserts the engine's under a `strict-failure:` comment. Every order assertion below
holds under both readings.

| Row | Claim (CR, ruling) | Changed seam | Test (entry → completion) | Revert-failing assertion | Positive reach-guard | Siblings / hostile fixtures | Identity / provenance |
|---|---|---|---|---|---|---|---|
| **V1** parse shape | The branch's seven lower to main's structural shape (CR 614.1a, CR 615.1a). | the two parser followups | UPDATED `oracle_replacement.rs`: the Nefarious Lich test, `damage_to_self_puts_that_many_counters_instead`, `damage_to_you_puts_that_many_counters_on_source_instead`, `damage_to_player_exiles_…`, `damage_instead_lines_are_substitutions_scoped_to_their_recipient`; `oracle_tests.rs` `crumbling_sanctuary_parses_…` | `shield_kind == ShieldKind::None` and `damage_modification.is_none()` for each of the seven (a reverted tag fails `None`) | each test still asserts `event == DamageDone` and its exact `execute` | Phantom Centaur stays `Prevention { All }` (existing); main's Soul-Scar/Szadek parser tests unchanged | the shape is the predicate's whole input; see V2 |
| **V2** predicate | One predicate classifies the class by shape. | `is_damage_substitution` (now `pub(crate)`) | NEW unit in `replacement.rs`: `is_damage_substitution_reads_the_definitions_shape` | true: Phytohydra's (`PutCounter` P1P1 `EventContextAmount` SelfRef), Szadek's (`PutCounter` + `Mill` sub), Nefarious Lich's (`ChangeZone` + `LoseTheGame` sub); false for each one-field mutation: `shield_kind = Prevention{All}` (Vigor's rider shape), `damage_modification = Some(Double)`, `execute = None`, action `Effect::unimplemented(..)`, action `DealDamage`, `event = DealtDamage`, `event = GainLife` | each false case is a one-field mutation of a true case | — | — |
| **V2b** amount latch | The latched amount reads the predicate (CR 614.6). | `replaced_event_amount` (helper deleted) | UPDATED `replaced_event_amount_latches_each_owning_kind_and_zero_prevented`: definition 3 becomes Phytohydra's real shape (no builder call) | the definition-3 rows keep `Some(4)` (Replaced, AcceptedBeforeApplying) and `None` (Modified) | existing reach-guard loop over all 10 definitions | definition 9 (unimplemented action) stays `None` | — |
| **V3** CR 615.12 | The class is not a prevention effect: damage that can't be prevented is still replaced (S1–S4). | `damage_shape_prevents` `None` arm | existing `soul_scar_mage_damage_to_counters::damage_cant_be_prevented_does_not_stop_the_substitution`, `phytohydra_replaces_damage_preventable_or_not`, `lichenthrope_…`, `player_substitutions_replace_damage_preventable_or_not` (unchanged) | **parity** (§12): with neither arm, the legacy description scan also answers `false` for these texts | each row's "preventable" half applies the substitution | Stomp's "Damage can't be prevented this turn" | — |
| **V4** CR 120.8 for main's cards | NEW `a_soul_scar_substitution_has_no_zero_damage_event_to_replace`: P0's Soul-Scar Mage; P1's Daunting Defender **staged with its printed Human Cleric subtypes** (`with_subtypes(vec!["Human", "Cleric"])`; its own "Cleric creature you control" is the recipient). P0's Prodigal Sorcerer taps to deal 1 damage to the Defender. P1 orders (CR 616.1). | CR 120.8 candidate block | activate → `assert_at_rest` | **Defender first:** no `GameEvent::ReplacementApplied { source_id: <Soul-Scar Mage> }`, 0 -1/-1 counters, 0 damage. At base, Soul-Scar applies to the 0 event (§15 U2). | P1's first `ReplacementChoice` has `source_set == {Soul-Scar Mage, Daunting Defender}` (without the Cleric subtype the Defender is no candidate and the row fails here). **Soul-Scar first:** one -1/-1 counter on the Defender, 0 damage, the `ReplacementApplied` for Soul-Scar present | existing `a_substitution_has_no_zero_damage_event_to_replace` (Nefarious Lich, Urza's Armor) stays | the gate reads the definition of the candidate's rid at scan time (live) |
| **V5** substitute, then the earlier rider (CR 614.6 + CR 615.5; Acolyte's Reward / Refraction Trap rulings) | NEW `an_earlier_rider_follows_a_soul_scar_substitution`: P0's Soul-Scar Mage; P1's vanilla 5/5; P0 casts Test of Faith at the 5/5, then Flame Slash at it. P1 orders (CR 616.1). Both orders on fresh boards. | `stash_damage_substitute` (custody branch) | cast → answer → `assert_at_rest` | **ToF first:** the 5/5's `CounterAdded` events, in order, start with `(Minus1Minus1, count 1)` (the substitute, latched 1), and every `Plus1Plus1` event comes after it, totalling 3 (Test of Faith's rider, latched 3; #174: three events of 1, card: one of 3). `prevented(&events) == 1` (the 3). Final after CR 704.5q: **2 +1/+1, 0 -1/-1**, 0 damage. **Soul-Scar first:** one `CounterAdded (Minus1Minus1, 4)`, no `Plus1Plus1` event, `prevented == 0` (CR 616.1f: nothing left for Test of Faith; CR 609.7b: not used up). Final 4 -1/-1, 0 damage. At base (traced, §2.6): ToF first gives 3 +1/+1 and **no** -1/-1 event (the substitute is discarded). Under r1's arm: the +1/+1 events precede the -1/-1. Both red. | P1's prompt `source_set` contains Soul-Scar Mage and a `SHIELD` candidate | branch analog V5d | two drains on one event, each with its own `event_amount` (1 / 3) |
| **V5b** the earlier rider pauses after the substitute | NEW `an_earlier_riders_prompt_follows_a_soul_scar_substitution`: V5's board, plus P1's Hardened Scales and Doubling Season. ToF first. Drives Scales first and Season first on fresh boards. | custody branch, `Completed` re-stash; the caller's rider drain pauses | cast → `choose_shield` → `answer_counter_orders` → `assert_at_rest` | **At P1's first counter-order prompt the 5/5 already has 2 -1/-1** (the substitute's 1, doubled by Doubling Season, with one applicable replacement and so no prompt) **and 0 +1/+1.** Then Scales first: seen `[0, 4, 8]`, 12 +1/+1 before SBA, final **10 +1/+1, 0 -1/-1**; Season first: `[0, 3, 6]`, 9, final **7 +1/+1**. strict-failure #174: the card gives one prompt and 8 / 7 (final 6 / 5). At base and under r1, the -1/-1 count at the first prompt is 0. | every counter-order prompt is P1's, with `source_set == {Hardened Scales, Doubling Season}` | — | the rider's frame is fresh after the substitute's dispatch; the prompt is the caller's drain's |
| **V5c** the substitute pauses; the earlier rider waits behind it | NEW `an_earlier_rider_waits_behind_a_soul_scar_substitutions_prompt`: V5's board, plus P1's Vizier of Remedies and Doubling Season. ToF first. Drives Vizier first and Season first. | custody branch, `Paused { owner }` → re-stash behind the paused frame; the answer-side retire path runs it | cast → `choose_shield` → answer every `ReplacementChoice` with the preferred source → `assert_at_rest` | At least one prompt is raised, and **at every prompt the 5/5 has 0 +1/+1** (Test of Faith's rider waits behind the substitute's counter event). Vizier first: (1−1) = 0 -1/-1; Season first: 1·2−1 = 1 -1/-1. Then Test of Faith's 3 +1/+1 under Doubling Season alone (no prompt): 6 (card and engine agree). Final: Vizier first **6 +1/+1, 0 -1/-1**; Season first **5 +1/+1** after CR 704.5q. At base: no prompt at all (the substitute is discarded). Under r1: the prompt shows 6 +1/+1. | the prompt is P1's with `source_set == {Vizier of Remedies, Doubling Season}` | — | the queued rider keeps its latched 3 across the prompt |
| **V5d** CONVERTED (defective reference) | `an_earlier_rider_is_not_lost_to_a_substitution_and_reads_its_own_amount`, Dralnu sub-row. Dralnu: "sacrifice that many permanents instead" is the modified event (CR 614.6), and Test of Faith's counters follow it (CR 615.5). | custody branch, `Paused` (the sacrifice prompt) | cast → `choose_shield` → SelectCards (a land from `lands(..)`) → `assert_at_rest` | shield first: at the zone prompt `(1, 5)`, **0 +1/+1** (was 3). After selecting a land: 3 +1/+1 on Dralnu, the land in the graveyard, Dralnu on the battlefield, 0 damage. The not-shield-first leg is unchanged. Phytohydra, Lichenthrope and Temper sub-rows are unchanged (final counts). | the zone prompt is raised | — | the queued rider reads 3 after the answer |
| **V5e** CONVERTED (defective reference) | `an_earlier_riders_paused_counters_finish_before_the_substitution` → renamed `an_earlier_riders_counters_follow_the_substitutions_sacrifice`. P0: Dralnu, Hardened Scales, Doubling Season, 2 lands; Test of Faith, then Flame Slash at Dralnu; shield first. | custody branch, `Paused`; the queued rider then pauses or not | cast → `choose_shield` → SelectCards → `answer_counter_orders` → `assert_at_rest` | The sacrifice prompt `(1, 5)` comes first, with 0 +1/+1 and no counter-order prompt before it (was `seen == [0, each, 2·each]` first). **(a) Sacrifice Hardened Scales**: then Doubling Season alone applies, so there is no counter-order prompt and Dralnu ends at **6 +1/+1** (card and engine agree), Scales in the graveyard (was 12/9: "sacrificing Hardened Scales cannot change a counter event that already happened" reverses). **(b) Sacrifice a land**: then the counter orders follow: Scales first `seen [0, 4, 8]` → 12, Season first `[0, 3, 6]` → 9 (strict-failure #174; card 8 / 7). Phytohydra sub-row: values unchanged (`[0, 4, 8, 12]`, 16), comment corrected (Phytohydra's 1-counter event comes first; under Scales first every 1-counter event gives 4, so V5b discriminates the order instead). Hurloon Battle Hymn sub-row: no counter-order prompt before the sacrifice prompt `(1, 7)` (counters 0, life 20); select Scales → **6 +1/+1**, life 24 (the kicked gain follows the rider: class note P1, Acolyte's Reward, "before that spell or ability resumes its resolution"). | each prompt asserted by kind | — | both pauses in one event (substitute, then rider) |
| **V6a** S10 ruling | "your creature will be dealt 4 damage and your opponent's creature will have two -1/-1 counters". NEW `soul_scar_mage_fight_replaces_one_direction`: P0's Soul-Scar Mage; P0's Grizzly Bears 2/2 fights P1's 4/4 (Pit Fight). | fight batch (`run_object_damage_batch`) | cast → `EffectKind::Fight` resolved once, `assert_at_rest` | **parity**: the Bears are dealt 4 (in the graveyard after SBA); the 4/4 has two -1/-1 and 0 damage | the same board without Soul-Scar Mage: the 4/4 is dealt 2 | — | substitution held in the batch, authority P1 |
| **V6b** WithEvent timing for main's cards | CR 614.6 vs CR 615.5 + CR 101.4. NEW `a_soul_scar_fight_substitution_comes_with_the_event`: P0 active; P0 controls Soul-Scar Mage, Hardened Scales, Doubling Season; P0's Bears 2/2 under P0's Test of Faith; P1's 4/4 with P1's Doubling Season and Vizier of Remedies. Pit Fight: Bears vs 4/4. | `hold_damage_rider` timing | cast → answer → `assert_at_rest` | the **first** counter-order `ReplacementChoice` is **P1's** (the -1/-1 counters are the modified event), then P0's (Test of Faith's counters follow, CR 615.5). With `AfterEvent` (base timing) the APNAP sort asks P0 first. Answered P1: Doubling Season first, P0: Hardened Scales first → **4/4: 2·2−1 = 3 -1/-1, 0 damage; Bears: (3+1)·2 = 8 +1/+1, 1 damage**. Second drive, P1 Vizier first and P0 Doubling Season first → **(2−1)·2 = 2 -1/-1; 3·2+1 = 7 +1/+1**. | two counter-order prompts, one per player; Pit Fight completes once (`assert_completes_once_after_its_damage`) | existing `a_fight_holds_an_answered_shields_rider_for_apnap_order` (AfterEvent riders keep APNAP) | `HeldBatchRider.authority` = the recipient's controller (P1 for the 4/4) |
| **V7** materiality | The class is order-material against every damage replacement (S7–S9, CR 616.1). | `candidate_materiality` | existing `a_substitution_is_ordered_against_a_doubler`, `a_substitution_is_ordered_against_a_prevention_shield` (unchanged); NEW `a_substitution_and_a_redirect_ask_their_order`: P1 controls Force Bubble and a creature enchanted by P1's Pariah; P0's Lightning Bolt at P1 | **parity** (§8): `ReplacementChoice` for P1. **Bubble first:** 3 depletion counters, life 20, the creature 0 damage. **Pariah first:** the creature is dealt 3, 0 counters (CR 616.1f) | prompt candidates include both sources | — | — |
| **V8** CR 615.7 with a real substitution (S8) | Replacing damage spends no shared prevention capacity: the shield is still there for the next damage. NEW `szadeks_substitution_leaves_a_shared_shield_for_the_other_attacker`: P0's Szadek, Lord of Secrets (5/5 flier) and a vanilla 4/4, both created in that order; P1 has 20 life and ≥ 6 library cards and no creatures. P0 casts Mending Hands targeting P1 in precombat main. Szadek and the 4/4 attack P1 (declared in that order), unblocked. Both orders on fresh boards. | `replacement_spends_shared_capacity` | `combat_events` → answer P1's prompt → combat end | **Szadek first:** Szadek gets 5 +1/+1, P1 mills 5, and the 4/4's 4 damage is then prevented by Mending Hands' **unspent** 4 → **P1 at 20**, one `DamagePrevented { target: Player(P1), amount: 4 }` (the 4/4's). **Mending first (S8's own example):** Mending Hands prevents 4 of Szadek's 5, Szadek replaces the last 1 → **1 +1/+1, 1 milled**; the 4/4's 4 meets no shield → **P1 at 16**. Both are **parity** (`ShieldKind::None` already returns `false`); the row exists to exercise a substitution beside the shared shield. | P1's **first** `ReplacementChoice` holds a candidate with `source_id == Szadek` and a `SHIELD` candidate (Szadek's leg met the shield with all 4 left; §15 U6) | existing `a_batch_keeps_a_shared_shield_for_the_players_choice` stays (noncombat; no substitution) | the shield's remaining amount after Szadek's leg is the one the 4/4's leg reads |
| **V9** CR 615.13 | Nothing counts as prevented (S5). | single arm (no `DamagePrevented`) | existing `szadek…::unblocked_combat_damage_becomes_counters_and_mill`, `soul_scar…::burn_at_opponents_creature_becomes_minus_counters`, `phytohydra_…` (unchanged) | parity: no `DamagePrevented` | counters placed | — | — |
| **V9b** CR 614.5, S12 | NEW `two_undead_alchemists_replace_a_zombies_damage_once`: P0 controls Undead Alchemists A and B, **each staged with its printed Zombie subtype** (`add_creature_from_oracle(P0, "Undead Alchemist", 4, 2, ..).with_subtypes(vec!["Zombie"])`); A attacks P1 unblocked, B stays home; P1's library is 7 noncreature cards. For `first` in [A, B], on a fresh board. | applied-set re-check (CR 616.1f) | `combat_answering(.., &[first])` → combat end | **exactly one** `GameEvent::ReplacementApplied` whose `source_id` is A or B, and it is `first`; **P1 mills exactly 4** (library 7 → 3, graveyard 4); life 20; no Zombie token created | `prompts.len() == 1`, `prompts[0].0 == P1`, and **`source_set(&prompts[0].1) == {A, B}`**: both Alchemists applied to the Zombie's damage, so the row cannot pass with one applicable Alchemist (without the Zombie subtype neither applies and the prompt assertion fails) | the second Alchemist's definition is the same predicate | two authorities: the event's `applied` set excludes the applied rid; the other no longer matches the mill |
| **V10** fence re-key | The fence reads the runtime predicate and covers the class uniformly (addendum; maintainer item 1). | `coverage::extract_damage_replacement_choice_feature` | NEW `coverage.rs` unit `a_damage_substitution_is_a_coverage_gap` (harness `counter_rider_face` + `extract_card_features`) | tag `structural:damage_substitution_choice_incomplete` = `Unhandled` for Soul-Scar Mage (kw Prowess), Szadek, Lord of Secrets (kw Flying), Undead Alchemist. **For these three `card_face_gaps(&face) == ["ResolverFeature:structural:damage_substitution_choice_incomplete"]`**: the fence is the drop's sole cause. The old tag-keyed copy leaves all three unfenced. | Phytohydra, Force Bubble, Nefarious Lich fenced | not fenced: Test of Faith, Furnace of Rath, Phantom Centaur | the fence reads the parsed definition (`face.replacements`), the same function the runtime reads |
| **V11** honest gaps, clause by clause | Oozeavite and Underdark Beholder stay unsupported **through their damage clause**, and the predicate never selects them (charter P3 decision 4). | — (measurement) | NEW `coverage.rs` unit `an_unlowered_damage_substitution_stays_unsupported`, faces from verbatim Oracle text | For each face, `card_face_coverage(&face).gaps` holds a gap whose `source_text` is the damage clause, and whose `handler` is that clause's unimplemented-effect label: Oozeavite's clause "would be dealt damage, instead create a 0/0 green Ooze creature token" → `Effect:static_structure`; Underdark Beholder's "would be dealt damage, remove that many eyestalk counters from it instead" → `Effect:replacement_structure` (labels measured at `51adf5568`; executor confirms them at base, §15 U5/U7). Also: no `face.replacements` entry satisfies `replacement::is_damage_substitution`, the fence tag is absent, and `!is_supported()`. | V10's Phytohydra row, same harness | the other lines' gaps (Oozeavite's counters line, Beholder's attack trigger) are not asserted, so the row pins the damage clause, not an incidental gap | — |
| **V12** protection witnesses (strict-failure) | CR 702.16e prevents at the pre-replacement gate, before the CR 616.1 choice (CR 616.1e). | none (documents the fenced gap) | existing `protection_prevents_ahead_of_a_substitutions_choice` (Phytohydra); NEW `protection_prevents_ahead_of_soul_scar_mages_choice`: P0's Soul-Scar Mage; P0's Pyroclasm staged with its printed {1}{R} cost (CR 202.2 makes it red); P1's Silver Knight (protection from red) and P1's 3/3 Hill Giant. NEW `protection_prevents_ahead_of_szadeks_choice`: P1 casts Teferi's Protection, then Szadek attacks P1 unblocked. | **by the card** P1 would choose (Soul-Scar first → two -1/-1 on the Knight; protection first → none). **The engine:** no `ReplacementChoice`, the Knight has 0 counters and a `DamagePrevented` for it. Szadek: no prompt, 0 counters on Szadek, P1's library unchanged, `DamagePrevented` to P1. Each doc says so and names the fence. | the Hill Giant gets two -1/-1 and survives as a 1/1 (Soul-Scar applied in that cast); Szadek: `DamagePrevented { target: Player(P1) }` (the damage reached the gate) | — | — |
| **V13** S6 | Force Bubble's one choice across sources is Force Bubble's own ruling, not a class claim (CR 700.1). | none | existing `two_force_bubbles_take_all_of_a_combat_batch_on_one`, `a_repeated_answer_puts_a_noncombat_batch_on_one_force_bubble`: assertions unchanged, docs reworded (§16, step 7) | unchanged | unchanged | — | — |
| **V14** protocol | The deleted serialized variant gets a registry record. | `protocol/changes/` | `node scripts/protocol-versions.mjs check`; `node scripts/check-protocol-version.mjs --base <fetched main>` | both pass | — | — | — |
| **V15** suites | Main's #9670 suites and the branch's suites pass unchanged in intent (task acceptance). | all | `soul_scar_mage_damage_to_counters`, `szadek_combat_damage_to_counters`, `undead_alchemist_combat_damage_to_mill`, `adamantoise_vigor_combat_order`, `combat_hit_bookkeeping_follows_delivery`, `awe_strike_prevention`, `ultimate_magic_meteor_per_opponent_destroy`, `swans_prevention_followup`, `damage_substitution_replacements`, `unpreventable_damage_prevention_effects`, `ironscale_hydra_combat_prevention` | green, with V5d/V5e as the named conversions | — | — | — |

**Hostile coverage per seam:**
- **The empty / no-candidate path.** V4's Defender-first leg, where the substitution has no event;
  V5's Soul-Scar-first leg, where the shield has none; and V11.
- **Multi-authority cases.** V5–V5e (two replacements on one event, each latching its own amount,
  in the CR 615.5 order, with either drain pausing, or both); V6b (two players' held work in one
  batch); V8 (a substitution beside a limited shield, across two legs); V9b (two copies of one
  replacement); and V7 (a substitution against a redirect).
- **Controller vs recipient.** V6b's authority is the recipient's controller. V5b/V5c's prompts are
  the recipient's controller's (P1), not the substitution's controller's (P0).
- **The first production branch each fixture reaches:**
  - V4: the CR 120.8 candidate block (amount 0 after Defender);
  - V5, V5b: `stash_damage_substitute`'s custody branch (`take_ready_post_replacement_drain`
    returns the rider), then `DamageRiderDrain::Completed`;
  - V5c, V5d, V5e: the same branch, then `DamageRiderDrain::Paused { owner }`;
  - V6b: `hold_damage_rider`'s timing `if`;
  - V8: the combat leg's `replacement_spends_shared_capacity` read for Szadek's rid;
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
- **V5–V5e (order).** CR 615.5: "The prevention takes place at the time the original event would
  have happened; the rest of the effect takes place immediately afterward." CR 614.6: the replaced
  event never happens, and "A modified event occurs instead". Test of Faith prevents 3. The
  original event's remainder (1) would happen now. Soul-Scar Mage / Dralnu replaces it, so the
  modified event (the -1/-1 counter, or the sacrifice) happens now. Test of Faith's "For each 1
  damage prevented this way, put a +1/+1 counter on that creature" is the rest of the effect, and
  it happens immediately afterward. The class-B rulings state the same order for an excess that is
  dealt (Acolyte's Reward: "…deals its excess damage … at the same time that the rest of it is
  prevented. Then Acolyte's Reward deals its damage."; Refraction Trap: "Immediately afterward, as
  part of that same prevention effect…"). Each effect's "that many" is its own amount: 1 for the
  substitute and 3 for the rider (CR 615.5 "the amount of damage that was prevented"; maintainer
  item 3).
  - V5b: Doubling Season, P1's, doubles the substitute's -1/-1 counter on P1's creature (one
    applicable replacement, so no choice). Test of Faith's counters then meet Hardened Scales and
    Doubling Season, and P1 orders them (CR 616.1). By the card, one event of 3: (3+1)·2 = 8 or
    3·2+1 = 7. Then CR 704.5q removes 2 of each: 6 or 5.
  - V5c: Vizier and Doubling Season both apply to the substitute's 1 -1/-1 counter, and P1 orders
    them: (1−1)·2 = 0 or 1·2−1 = 1. Then Test of Faith's 3 counters meet Doubling Season alone: 6.
    CR 704.5q leaves 6 or 5.
  - V5e (a): the sacrifice comes first, so Hardened Scales is in the graveyard when Test of Faith's
    counters are put. Only Doubling Season applies: 3·2 = 6. (b): a land goes, both apply, and the
    order gives 8 or 7.
  - V5d: the sacrifice prompt is raised before Test of Faith's counters exist (0 at the prompt), and
    they arrive after the answer (3).
  - Measured reference: the branch's own two-phase batch already orders a substitution action ahead
    of a rider (`RiderTiming::WithEvent`, Phase D). These rows derive the same order for the
    single-event path. They are derived, not copied.
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
- **V8 (parity).** S8 gives the Mending-first reading verbatim ("first have Mending Hands prevent 4
  of the damage and then apply Szadek's ability so that it gets only one counter"). CR 615.7: "Each
  1 damage that would be dealt to the "shielded" permanent or player is prevented. Preventing 1
  damage reduces the remaining shield by 1." In the Szadek-first leg nothing is prevented, so the
  shield is not reduced (CR 609.7b: "If for any reason the shield prevents no damage or replaces no
  damage, the shield isn't used up"), and its 4 meet the 4/4's 4. The Mending-first leg spends all
  4 on Szadek, and the 4/4's damage is dealt.
- **V9 (parity).** CR 615.13 triggers on prevention, and S5 says the damage "is neither dealt or
  prevented". The existing rows assert that reading.
- **V9b.** S12 and CR 614.5 say the event is replaced once. Both Alchemists apply to a Zombie's
  combat damage to a player, so P1 chooses one (CR 616.1). After it applies, the event is a mill
  (CR 614.6), which the other's "would deal combat damage" no longer matches (CR 616.1f). So P1
  mills 4, once, whichever is chosen.
- **V15.** Main's and the branch's suites stand on their own derivations (P1 plan §11–§12,
  unchanged). If one goes red, the executor classifies it under §15 U1 before any edit. A red row
  whose reading the card derives differently is a defective reference, and it is fixed in-phase
  (defective-reference route). It is never re-pinned to the new output. V5d and V5e are the two
  such rows this plan names in advance.

Every non-parity expected value (V4, V5–V5e, V6b, V9b) is derived in its row from the cards and the
CR, and none is copied from a sibling route.

---

## 13. Skills and checklist steps (Step 1)

- **`/add-replacement-effect`.**
  - Phase 1 (types): delete the variant and the builder (step 1).
  - Phase 2 (registry): the `DamageDone` handler (`damage_done_applier`) is unchanged in
    registration. Its arms merge, and the stash's substitute order moves to `stash_damage_substitute`
    (step 3).
  - Phase 3 (parser): two builder calls (step 2). No routing change.
  - Phase 4: N/A.
  - Phase 5 (tests):
    - parser test (V1);
    - pipeline test with a revert-failing assertion (V4, V5–V5e, V6b);
    - engine flow (V6a, V8, V9b, V12);
    - provenance (V5–V5c, V6b);
    - candidate count, 1 and 2+ (V4, V5, V7, V8, V9b);
    - multi-replacement (V5–V5e, V9b);
    - optional accept/decline: existing `soul_scar…::optional_substitution_accept_puts_counters_and_decline_deals_damage`
      and `szadek…::optional_substitution_accept_performs_counters_and_mill_decline_deals_damage`,
      unchanged;
    - serialized state (V14).
- **`/add-interactive-effect`.** The paused-substitute round trip (V5c–V5e) reuses the existing
  continuation: the substitute's prompt is a standing `WaitingFor`, the earlier rider waits behind
  its frame, and the answer-side retire path runs it. No new `WaitingFor`, `GameAction` or AI arm.
- **`/oracle-parser`.** No dispatch change. The combinator gate runs (§7).
- **`/add-engine-variant`.** The deletions' sibling check (§10).
- **`/add-card-data-pipeline`.** Parser output shape change: `shield_kind` for seven faces goes
  from `Substitution` to the default.
  - Audit serialized consumers: none outside Rust (§2.3).
  - The coverage honesty gate: three faces are newly fenced, and none is newly supported.
  - Snapshots: none name the variant.
  - Regenerate card data and run the coverage diff at run-level verification (orchestrator).
- **`/card-test`.** Every runtime row names its production entry and its completion boundary
  (§11).

DEFERRED: none (outcome A; the charter's P3 deferral list is empty under A). The combat order
residual is not a P3 deferral. It belongs to Command Zone #267's existing scope (§18).

---

## 14. Serialized surfaces and protocol

- `ShieldKind` is serialized in `ReplacementDefinition` (GameState and card data). Deleting
  `Substitution` means a state or card data written with that tag no longer decodes. Card data is
  regenerated. Saved states from the unreleased branch are the only writers of the tag.
  `ApplyResult` is not serialized.
- **Registry step**, run by the executor from the repository root:
  ```
  node scripts/protocol-versions.mjs bump game --reason "ShieldKind::Substitution is removed: '…instead' damage substitutions are recognized by their shape (replacement::is_damage_substitution) and serialize with ShieldKind::None, so a GameState or card data written with the Substitution tag no longer decodes."
  ```
  Then `node scripts/protocol-versions.mjs check` and
  `node scripts/check-protocol-version.mjs --base <fetched main>`.
- The branch's record `c85ad0f6-12d8-45cc-ae35-edf384ac7f40.json`, which added the variant, is
  kept. This follows CLAUDE.md and project-reference ("Do not … alter/delete existing records";
  "Reverting an incompatible change requires a new record") and addendum P2 Q-2's precedent. Its
  reason text becomes historical. §18 has the open question.

---

## 15. Unprobed assertions (each settled at implementation by the named check)

- **U1. Main's suites and the branch's suites stay green** under the semantics that newly reach
  main's three (`WithEvent` timing, the CR 120.8 gate, the `last_effect_count` stamp, the
  substitute-first order), and under the order change for the branch's seven.
  - Read from source: main's rows have no prevention rider resident when a substitution applies;
    their multi-source rows hold only substitution actions, so timing alone does not reorder them;
    and the spell's "that much" reads `last_effect_amount` through `amount_channel`, not
    `last_effect_count`. In the branch's suite, the same-event rider-plus-substitution rows outside
    combat are V5d's and V5e's tests (converted). `a_resident_rider_from_another_recipient_is_not_lost`
    (Temper + Pyroclasm) asserts final counts only, and they don't change; its combat half is
    unchanged.
  - Settled by running V15.
  - A red row is classified before any edit:
    - **(a)** the row encodes a behavior the card and CR derive differently: defective reference,
      fixed in-phase with the derived value;
    - **(b)** the fold or the order broke a derived reading: fix the consumer;
    - **(c)** the red row is outside P3's seams: stop and return.
- **U2. V4, V5, V5b, V5c, V5d, V5e and V6b are red at `PHASE_BASE_SHA`.** The executor runs each
  new or converted row against `6f40e4dc6a` first. They need no API the phase deletes, so they
  compile at base. Run them in a scratch worktree, and prune it afterwards.
  - V4 and V6b: expected red. Base keeps Soul-Scar as a 0-damage candidate, and its timing is
    `AfterEvent`.
  - V5, V5b, V5c: expected red (traced, §2.6: the substitute is discarded at base). V5d, V5e:
    expected red (base runs the rider first: 3 +1/+1 at the prompt; 12 / 9 in row (a)).
  - If any is green at base, it is not discriminating: report it, do not ship it as
    revert-failing.
- **U3. Main's three are supported at base, with no other gap.** Settled by V10's only-gap
  assertion, run at base without the fence edit (expected: gaps empty). If any of them has another
  gap at base, it is not a coverage-honesty drop of P3's, and the flip list (§17) drops it.
- **U4. Teferi's Protection's player protection is checked in the combat path at the
  pre-replacement gate.** This is the maintainer's reading: the player gate in
  `deal_damage::pre_replacement_damage_gate` and "the same gate in `combat_damage.rs`".
  `combat_damage.rs` documents that gated assignments go through `pre_replacement_damage_gate`.
  Settled by V12's Szadek row. If the combat path does not emit `DamagePrevented` there, stop and
  return. The witness would then be wrong, not the fence.
- **U5. Oozeavite and Underdark Beholder parse unchanged.** Traced and data-measured (§2.5).
  Settled by V11. The handler labels in V11 are the measured ones. P3 changes no parser dispatch,
  so the base parse is the reference: if the label at base differs from the measured one, V11
  asserts the base label. The source text must still be the damage clause.
- **U6. V8's leg order.** `replace_combat_damage_batch` starts unstarted legs in batch order when no
  prompt is outstanding, and with Szadek's prompt outstanding the 4/4's leg (whose chooser ranks
  the same) does not start ahead of it (the `start_leg` rule). Szadek is created and declared
  first. Settled by V8's reach guard: P1's first prompt must hold Szadek and the shield. If the
  batch puts the 4/4's leg first, the guard fails loudly. The executor then restages so that
  Szadek's assignment comes first, and the derived values do not change.
- **U7. V11's gaps carry `source_text`.** `CoverageGap.source_text` comes from the parse tree
  (`GapDetail`). If the damage clause's gap has no source text, V11 pins the clause through the
  parse item that carries it (the `Effect::Unimplemented` node whose description names the clause)
  and its handler instead. It never falls back to `!is_supported()` alone.
- **U8. The inline dispatch inside the pipeline.** `stash_damage_substitute` runs the substitute
  through `drain_damage_rider` from inside `apply_single_replacement`, at the depth where the
  branch's arm ran the earlier rider today. A paused substitute leaves the same state the branch's
  paused rider left (a paused frame, a `WaitingFor` surfaced by
  `handle_replacement_choice_inner`'s `Prevented` arm, `ledger.paused_rider` set). Settled by
  V5c–V5e passing and by `a_paused_rider_keeps_the_batchs_later_riders_behind_it`,
  `every_rider_queued_behind_a_paused_rider_runs` and
  `work_queued_behind_a_paused_rider_runs_in_the_order_it_was_queued` staying green.

---

## 16. Step-by-step plan (one commit on `6f40e4dc6a`, then the protocol record)

Run `cargo fmt --all` directly. Check Tilt first (`tilt get uiresource clippy`), and use
`./scripts/tilt-wait.sh` / `tilt logs` when it is up. Run `./scripts/check-parser-combinators.sh`.
Before step 3, run the U2 base check for V5–V5e (scratch worktree).

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
     the stash's amount latch and order (`stash_damage_substitute`), the batch hold's timing, the
     CR 120.8 candidate block, the CR 615.12 shape, the CR 616.1 materiality and coverage's
     `damage_substitution_choice_incomplete`. "…so they can never disagree."
   - **`hold_damage_rider`.** Timing = `if damage_execute_substitutes_event_type(state, rid) {
     RiderTiming::WithEvent } else { RiderTiming::AfterEvent }`. Keep its CR 614.6 / CR 615.5
     comment.
   - **`damage_done_applier`.**
     - Delete the tag arm, the `if matches!(shield_kind_for_rid(..), Some(ShieldKind::Substitution))`
       block before Branch 2, including its earlier-rider drain.
     - Branch 3 under `damage_execute_substitutes_event_type(state, rid)`: for
       `ProposedEvent::Damage { amount, .. }`, stamp `state.last_effect_count =
       Some(u32_to_i32_saturating(amount))` and return `ApplyResult::Prevented`. Keep the
       `ApplyResult::Modified(event)` fallback for any other event. One comment cites CR 614.1a,
       CR 614.6 and CR 615.13 (Soul-Scar Mage; Force Bubble's "neither dealt or prevented"), and
       says that the stash orders the substitute against an earlier rider
       (`stash_damage_substitute`).
   - **`ApplyResult::PreventedBehindRider`.** Delete the variant and its doc. In
     `apply_single_replacement`, the arm becomes `ApplyResult::Prevented =>` with the `behind`
     binding removed. The five `Err(ApplyResult::Modified(_) | ApplyResult::PreventedBehindRider(_))`
     patterns become `Err(ApplyResult::Modified(_))`. Update `ReplacementOutcome::Replaced`'s doc
     ("`ApplyResult::Prevented`").
   - **`PausedRider` doc.** It now names "a paused damage replacement action or rider whose frame
     later work for the same damage waits behind (CR 615.5, CR 608.2c)". It is still built from
     `DamageRiderDrain::Paused` and from `ledger.paused_rider`.
   - **`damage_rider_placement(state)`.** Drop the `behind` parameter (both callers passed `None`
     or the deleted variant's payload). Update its doc: work waits behind a rider of the same
     noncombat batch that left its prompt standing. Keep the #172 sentence, reworded: "A
     substitution queued here behind another recipient's paused rider runs after it, though by
     CR 614.6 + CR 615.5 it should come first (Command Zone #172)."
   - **`apply_single_replacement`'s `Prevented` arm (R9).**
     - Push `GameEvent::ReplacementApplied { source_id: rid.source, event_type }` before the
       `if let Some(post) = mandatory_post_effect` block, not after it. Stashing emits no events,
       so only a substitute run inline moves in the log (§5 (g)).
     - After `hold_damage_rider` hands the drain back: if `is_damage_event &&
       damage_execute_substitutes_event_type(state, rid)`, call `stash_damage_substitute(state,
       drain, placement, events)`. Otherwise call `stash_post_replacement_continuation(state, drain,
       placement)` as today.
   - **New private `stash_damage_substitute`**, beside `stash_post_replacement_continuation`:
     ```rust
     /// CR 614.6 + CR 615.5: stash a damage substitution's action. It is the modified event, so it
     /// happens when the damage would have, and a rider resident from an earlier replacement
     /// (Test of Faith: "For each 1 damage prevented this way, put a +1/+1 counter on that
     /// creature.") takes place immediately afterward, each reading the amount latched on its own
     /// drain (`replaced_event_amount`). A rider queued behind another recipient's paused rider
     /// keeps its place in that queue (Command Zone #172).
     fn stash_damage_substitute(
         state: &mut GameState,
         substitute: PostReplacementDrain,
         placement: Option<PausedRider>,
         events: &mut Vec<GameEvent>,
     ) {
         if state.combat_prevention_tally.is_some() {
             // strict-failure: CR 615.5 puts the earlier rider after the substitute, but a combat
             // leg can't park on a rider's prompt yet (Command Zone #267), and running the
             // substitute here would settle its own prompt inside the batch (Dralnu, Lich Lord's
             // sacrifice would be cleared). The rider runs first, settled in the batch, and the
             // substitute's prompts are asked at the leg.
             if state.has_post_replacement_drain() {
                 drain_combat_damage_rider(state, events);
             }
             stash_post_replacement_continuation(state, substitute, placement);
             return;
         }
         let earlier = placement
             .is_none()
             .then(|| state.take_ready_post_replacement_drain())
             .flatten();
         stash_post_replacement_continuation(state, substitute, placement);
         let Some(earlier) = earlier else { return };
         let behind = match drain_damage_rider(state, events) {
             DamageRiderDrain::Completed => None,
             DamageRiderDrain::Paused { owner } => Some(PausedRider { owner }),
         };
         // CR 104.1: a substitute that ended the game (Nefarious Lich's "If you can't, you lose
         // the game") ends it; the rider doesn't run.
         if state.game_end.is_some() {
             return;
         }
         stash_post_replacement_continuation(state, earlier, behind);
     }
     ```
     The comments in the sketch are the CR annotations to keep. Wording may be tightened, but the
     CR numbers and the strict-failure tag stay.
   - **`drain_combat_damage_rider` doc.** "the substitution arm's drain of an earlier rider inside a
     leg" → "`stash_damage_substitute`'s drain of an earlier rider inside a leg".
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
     `an_unlowered_damage_substitution_stays_unsupported` (V11, per-clause gap), with verbatim
     Oracle text.
6. **`crates/engine/tests/integration/damage_substitution_replacements.rs`**
   - Add V4, V5, V5b, V5c, V6a, V6b, V7, V8, V9b and the two V12 witnesses, with verbatim Oracle
     constants for Pyroclasm (already present), Silver Knight, Teferi's Protection, Flame Slash
     (present), Mending Hands (present), Daunting Defender, Prodigal Sorcerer (present), Vizier of
     Remedies, Soul-Scar Mage, Szadek, Lord of Secrets and Undead Alchemist.
   - Convert V5d and V5e as their rows state. Each conversion's doc names the card text, CR 614.6 +
     CR 615.5, and the corrected expectation. V5e's rename keeps its strict-failure #174 comment for
     row (b) and the Phytohydra sub-row.
   - Fixture notes:
     - printed subtypes: each Undead Alchemist `with_subtypes(vec!["Zombie"])`; Daunting Defender
       `with_subtypes(vec!["Human", "Cleric"])`;
     - V5–V5c: P1's 5/5 is a vanilla `add_creature(P1, .., 5, 5)`; Hardened Scales, Doubling Season
       and Vizier of Remedies are **P1's** (the recipient's controller);
     - V8: Szadek is created and declared before the 4/4; Mending Hands is cast by P0 at P1 in
       precombat main;
     - the Szadek protection witness: P1 casts Teferi's Protection with priority before combat
       damage;
     - the Silver Knight witness: Pyroclasm carries its printed {1}{R} cost;
     - V9b: P1's library holds noncreature cards, so Undead Alchemist's trigger stays idle.
   - Comment only: in `a_resident_rider_from_another_recipient_is_not_lost`, add a sentence to its
     combat half's doc: "In combat Test of Faith's counters are put before Phytohydra's; by
     CR 614.6 + CR 615.5 Phytohydra's come first (strict-failure, Command Zone #267)."
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
- **No gains.** No card becomes supported. The order fix changes no coverage verdict (every class-A
  face is fenced, and Test of Faith's rider is unchanged).
- **Parse-signature changes:** seven faces' `shield_kind` (`Substitution` → default). This is the
  reverse of the branch's own "Substitution (7)" parse-diff entry. No `event`, `execute` or filter
  changes.

---

## 18. For the orchestrator / human

- **Charter conflict C-1 (finding 1), for an addendum before executor dispatch.** Charter P3
  decision 2, first bullet, is frozen as: "one substitution arm in the damage applier, which drains
  an earlier rider first unless a batch holds riders". Read literally, "drains an earlier rider
  first" runs Test of Faith's additional effect before the substitute's modified event. That is
  the order CR 615.5 reverses ("the rest of the effect takes place immediately afterward";
  Acolyte's Reward: "…at the same time that the rest of it is prevented. Then Acolyte's Reward
  deals its damage."). Evidence from source is in §2.6. The branch's arm drains first because the
  stack holds one Ready entry; its comment says so. Main's arm, with no custody, loses the
  substitute entirely to the KeepResident collision. No design satisfies the bullet's literal text
  and CR 615.5 at once, so r2 does not work around it. r2 is built on this amended bullet:
  > one substitution arm in the damage applier; where the substitution's action is stashed, a rider
  > resident from an earlier replacement is kept and runs immediately after that action (CR 614.6 +
  > CR 615.5), unless a batch holds riders (Phase D's `RiderTiming` order). In combat, the in-leg
  > drain keeps the branch's earlier-first order, marked strict-failure (Command Zone #267).

  The amendment keeps the bullet's intent: the earlier rider is never lost, each reads its own
  amount, and a batch's held riders are untouched. If the orchestrator keeps the literal text,
  V5–V5e cannot ship with their derived values, and the phase must return rather than encode the
  reversed order.
- **Combat residual (inside C-1).** In a combat leg, an earlier per-event rider (Test of Faith on a
  blocking Phytohydra) still runs before the substitute, settled in the batch, as at base. P3
  cannot fix this with the existing combat pass. Moving the substitute into the in-batch drain
  would abandon its own prompt (`settle_combat_rider_pauses`' `OwnPrompt` arm; Dralnu's sacrifice),
  which the leg asks today. The fix is the in-leg rider park that P2 dropped to Command Zone #267.
  Please append "order: the substitute before the earlier rider in a combat leg (CR 614.6 +
  CR 615.5)" to #267's scope note. Every face this reaches is fenced under outcome A.
- **PR text (run-level, suggested wording).**
  - pr-body §1, "…They are now `DamageDone` replacements with a new `ShieldKind::Substitution`." →
    "…They are now `DamageDone` substitutions, recognized by main's structural predicate
    (`replacement::is_damage_substitution`), which also reads Soul-Scar Mage, Szadek and Undead
    Alchemist."
  - pr-body line 31 (M1, "later rider work waits behind a damage rider that paused (Test of Faith +
    Dralnu, Lich Lord …)") → add: "Outside combat, a substitution's action is the modified event,
    so it comes first; an earlier rider on the same damage runs immediately after it, behind the
    substitute's own prompt when it asks one (Dralnu, Lich Lord's sacrifice, then Test of Faith's
    counters; CR 614.6 + CR 615.5). In combat the rider still runs first (Command Zone #267)."
  - The fence bullets (pr-body lines 68 and 228) → "A damage substitution reports
    `damage_substitution_choice_incomplete`: protection prevents ahead of its CR 616.1 choice
    (CR 702.16e). Force Bubble, Phytohydra, Lichenthrope, Crumbling Sanctuary, Delaying Shield,
    Soul-Scar Mage, Szadek, Lord of Secrets and Undead Alchemist become unsupported. Force Bubble's
    one choice across simultaneous sources is its own ruling's gap, not claimed for the class
    (CR 700.1)." Nefarious Lich and Dralnu already report other gaps.
  - The parse-diff "Substitution (7)" entry now reads `shield_kind` Prevention{All} → default
    (None).
  - The protocol list gains the new record.
- **Sibling observed, not in scope.** Two prevention riders on one event (Test of Faith, then a
  "prevent that damage … for each 1 damage prevented this way" shield taking the rest) collide the
  same way: the second rider is dropped by KeepResident. `stash_post_replacement_continuation`'s
  doc already records same-event collisions outside the measured census. P3 does not touch
  prevention-only stashes. Filing is the orchestrator's call.
- **Human decision (minor).** The brief calls for a new protocol record. That leaves the branch's
  unreleased `c85ad0f6` (adds the variant) beside a record that removes it: two `game` bumps for a
  variant main never had. `check-protocol-version --base` validates only base records, so deleting
  `c85ad0f6` would also pass. But it contradicts the repository's "do not delete existing records"
  rule, so this plan keeps it, per addendum P2 Q-2's precedent. Flag only if the maintainer prefers
  the other way.
- **Riskiest parts.**
  1. The substitute-first order (§5, U8): an inline dispatch inside the pipeline, mirroring the
     branch's arm, plus the converted V5d/V5e rows (U1).
  2. Moving `WithEvent` timing and the stamp onto main's three, against main's suites (U1).
  3. The Teferi's Protection combat witness's entry and its gate assumption (U4), and V8's leg
     order (U6).
  4. Whether V4/V5–V5e/V6b discriminate at base (U2).
