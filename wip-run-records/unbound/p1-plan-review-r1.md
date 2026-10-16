## Phase 1 plan review, round 1 (`p1-plan-r1.md`)

**Mode:** phase-plan mode, with the phase-fit context declared. Charter phase index 1. The deferral allowlist is the phase-1 deferral list: stifled-ability copy → phase 2; line-1 doubling and Unbound supported → phase 3; ordering → phase 4. Check 12 (Sizing) is blocking in this mode.

**Constraints followed:** read-only. I ran no cargo builds or tests and no probes. Evidence comes from grep, sed and jq at HEAD b4685f2, which is a clean tree.

**Verdict: not clean.** There are 3 blocking `behavior` findings and 3 `text` findings. Sizing is consistent.

---

### Blocking findings

**B1 `behavior`: the activation-cost {X} fact is looked up by source, not by the activation (check 10). A mana ability can then falsely trigger Unbound, and the charter's "mana abilities don't trigger" rule holds only by accident and is never tested.**

The plan answers "that ability's activation cost contains {X}" with `casting_costs::pending_activation_cost_has_x(state, activated_id)`. The identity row (plan:322) names its authority as "the most recent `(source, ability_index)` in `pending_activations`", and U3 (§5.4, plan:520-526) extends that same lookup to the departed-source (LKI) branch.

- The lookup (`casting_costs.rs:17127-17143`) takes the *latest* `pending_activations` entry for that **source id**.
- Only stack activations push an entry (`casting.rs:27952`, inside `record_activated_ability_placed`). The list is cleared only on PassPriority (`engine.rs:9055`).
- Mana abilities also emit `AbilityActivated`, with `stack_entry: None`:
  - `mana_abilities.rs:3379`
  - `mana_sources.rs:1102`
  - `casting_costs.rs:14296`
  - The matcher doc says so: `trigger_matchers.rs:5123` ("stack-using abilities and mana abilities alike").
- Unbound's new activation half carries no `ActivatedAbilityIsNonMana` condition.

The hostile case: a permanent has a non-mana {X} ability and a non-X mana ability. Activate the {X} ability, keep priority, then activate the mana ability. The lookup finds the earlier {X} entry for that source and returns true, so Unbound triggers on a non-X mana ability. That is wrong under any reading of the card. The copy step then finds no stack entry.

The charter's run-level deferral 1 ("run keeps today's behaviour, no trigger for mana abilities") is only kept because mana abilities usually leave no pending entry. The plan has no row for it: plan:391 and §5.6 record it as a summary note only. The charter's identity rule ("identity is the stack-entry id, never the source object", phase 2 decision 3) points to the right key.

**Fix:**
- Thread the event's `stack_entry` from `match_ability_activated` (it is already in the destructured event at `trigger_matchers.rs:5139`, behind the `..`) into `activated_source_matches` and `bind_activation_cost_facts`.
- Compute the fact from that entry:
  - Look up `state.stack` for the entry with that id and match `StackEntryKind::ActivatedAbility { source_id, ability }`.
  - Take `ability.ability_index`, read that ability definition's cost on the source, and call `ability_cost_has_x`.
  - `stack_entry: None` (a mana ability, CR 605.3b) returns false. Its doc line names charter run-level deferral 1.
- This also covers the live branch. The filter is bound before either branch evaluates, so `filter.rs:8788` is never reached for this trigger.
- Update the identity row (plan:322) and the §5.4 doc to match.
- Add two rows to `unbound_flourishing.rs`:
  - (a) `unbound_ignores_mana_ability_of_source_with_pending_x_activation`. Use a verbatim-text fixture with "{X}: <non-mana effect>" plus "{T}: Add {G}." Activate the X ability, then the mana ability without passing. Exactly one Unbound trigger is placed. The paired positive is the X-ability trigger itself.
  - (b) `unbound_does_not_trigger_on_x_mana_ability`, using Wizard's Rockets verbatim ("{X}, {T}, Sacrifice this artifact: Add X mana in any combination of colors."). No Unbound trigger is placed. I9 is the positive guard. This replaces the out-of-run note at plan:391 with a test of the behaviour that is kept.
- Units and paths stay the same (`trigger_matchers.rs` only).

**B2 `behavior`: the scope matrix leaves out the activation zone where the ability is announced off the battlefield (check 11; ruling 13: "any … activated ability whose … activation cost contains {X}").**

`activated_source_matches` (`trigger_matchers.rs:5183-5213`) has three cases:
- a live source on the battlefield (I8);
- a battlefield source that a cost moved away, answered from LKI (I9, I10);
- an ability announced from hand or graveyard. Here `emit_ability_activated` (`casting_targets.rs:698-705`) sets no LKI, so the `None` (live) branch evaluates the *moved* object. A discard cost has already sent it to the graveyard.

The third case is reachable through the same placement path. `record_activated_ability_placed` runs for every activation, and cycling is tagged after it at `casting.rs:27953-27962`. Printed {X} cards in this case include:
- Shark Typhoon, Rampaging War Mammoth, Valor's Flagship, Webstrike Elite (Cycling {X}…);
- Arashi, Jiwari, Ghost-Lit Drifter, Roaring Earth, Shigeki (Channel {X}…);
- Swell of Courage, Wren's Run Hydra (Reinforce X).

Command: `jq -r '.data|to_entries[]|.value[0] as $c|($c.text//"")|split("\n")[]|select(test("^(Cycling|Channel|Reinforce)[^(]*\\{X\\}"))|$c.name' data/mtgjson/AtomicCards.json`

PR5 (plan:447) asks only whether the stripped `Typed{}` matches in the battlefield live and LKI evaluators. It does not ask about a graveyard object reached through the live branch. Unbound goes coverage-supported in phase 3, so a miss here would be a supported card behaving wrongly.

**Fix:**
- Add a scope row `unbound_copies_x_cycling_ability_from_hand`. Shark Typhoon verbatim, cycled at X=1 with Unbound out. P0 draws 2: the cycle and the copy (CR 707.10). Exactly one Shark token, because the "When you cycle" trigger is not copied. Paired control without Unbound: draws 1.
- Extend PR5 to the non-battlefield-announced live case.
- If the row fails, the executor stops and returns.

**B3 `behavior`: P1-T8 accepts a Neerdiv outcome that keeps the card supported with no runtime test (check 9).**

The row (plan:343) lets Neerdiv's trigger [1] be "either `AbilityActivated` with a graveyard-scoped `valid_card`, or `TriggerMode::Unknown(_)`". The first outcome keeps Neerdiv coverage-supported on a new runtime reading: "activate an ability of a card in your graveyard". That reading has no runtime row, and it runs through the B2 live-branch regime on objects whose cost may exile them (embalm-style costs). The plan's own coverage line (plan:397) expects Unknown. By reading, `try_parse_ability_activation_trigger`'s `parse_single_source_type` (`oracle_trigger.rs:9739-9750`) has no `card` arm, which agrees with that expectation.

**Fix:**
- P1-T8 asserts `[1]` is `TriggerMode::Unknown(_)`.
- Any typed `AbilityActivated` outcome is a stop-and-return to plan, which must then add a runtime row with CR 113.7a and zone semantics.
- Change plan:445 (PR3) to match.

**T1 `text`: the CR 603.2h citation, plan:489.**
- Old: `/// CR 603.2h: a "do this only once each turn" / "triggers only once each turn" rider bounds ONE printed ability.`
- New: `/// CR 603.2h: "Do this only once each turn" (and the printed "This ability triggers only once each turn" rider) limits the one printed ability.`
- Reason: CR 603.2h covers only "Do this only once each turn" ("triggers only if its source's controller has not yet taken the indicated action that turn"). The other rider is printed text, not that rule.

**T2 `text`: the CR citation for "becomes activated", plan:502.**
- Old: `an ability becoming activated, CR 602.2a`
- New: `an ability becoming activated, CR 602.2b + CR 601.2i`
- Reason: CR 602.2a is the announcement and stack-creation step. CR 602.2b imports 601.2b–i, and 601.2i is the step where the ability "becomes activated".

**T3 `text`: a visibility setting that would fail clippy, plan:460 and plan:471.**
- At plan:471, old: `` `pub(crate) fn parse_x_cost_slot_qualifier(input: &str) -> OracleResult<'_, CostSlot>` ``. New: `` `fn parse_x_cost_slot_qualifier(input: &str) -> OracleResult<'_, CostSlot>` (private; the child test module reaches it via `use super::*`) ``.
- At plan:460, old: `Make the new combinator `pub(super)` or `pub(crate)` only as far as the test module needs (match the visibility of the existing `parse_post_*` functions).` New: `Keep the new combinators private; `oracle_trigger_tests.rs` is a child module (`#[path]` at oracle_trigger.rs:23139) and sees them via `use super::*`.`
- Reason: a `pub(crate)` function returning the file-private `CostSlot` trips the `private_interfaces` lint, which fails under `-D warnings`.

---

### Sizing (check 12, blocking in this mode): consistent

- **Units:** 3 (U1, U2, U3). They match the body's three discriminating test sets. The two declines sit inside U2's single `/oracle-parser` pass.
- **Paths:** 5, matching the §5 file list. With 13 or more needed for T2, T2 is not met and the gate does not fire.
- **Effect of the fixes:** B1–B3 add only rows to the two test files already counted and a signature change inside `trigger_matchers.rs`, so the counts don't change.

### Checked and found sound

- **Card text:** the class table is complete; a corpus jq over AtomicCards for `^(whenever|when)…casts?…or activates? an` returns exactly the 5 cards. Every fixture's Oracle text matches MTGJSON. The only printed users of "cost that contains {X}" are Unbound and The First Tyrannic War (not a trigger qualifier). The "17 corpus lines" count reproduces.
- **Base readings:** the parse and coverage readings at base reproduce. Bill Potts and Neerdiv are supported; Glava, Vazi and Unbound are not. All five lower to one SpellCast each (Neerdiv's second).
- **Variant count:** `TriggerConstraint` has exactly 11 variants (`ability.rs:31245+`).
- **Code reads:**
  - `parse_trigger_constraint` (`oracle_trigger.rs:3946`) catches both riders from the full lowercase line.
  - `nom_on_lower`, `split_once_on` and `PrintedTriggerIndex::offset` have the signatures the plan assumes.
  - `TargetFilter::None` matches nothing.
  - Object ids persist across zone moves (incarnation model), so the graveyard lookup in U3 resolves.
- **CR numbers:** every one the plan cites exists in `docs/MagicCompRules.txt`.
- **Derived values:** I2, I4, I5, I8 and I9 match my own derivation from the Oracle text and CR 707.10. The I6 reference route matches the charter.

### Non-blocking notes

- **Ruling 3 (an X that is not the {X} mana symbol):** the plan relies on it (plan:28) but has no row. A cheap negative would be a planeswalker −X ability or a "Remove X counters" cost with Unbound, expecting no copy (`ability_cost_has_x` `_ => false`, `casting_costs.rs:17121`).
- **Rulings 5 and 14 (modal copies, additional-cost effects):** relied on through the existing copy authority, with no rows.
- **P1-T6 and P1-T7:** they assert `Unknown(_)` with a wildcard. Today's fallbacks are SpellCast, so the rows still fail on revert. Pinning `Unknown(<decline>.strict_failure_description())` would also prove the decline route was the one taken.

### Pre-existing

- **Magus Lucea Kane:** its delayed `or_trigger` (`AbilityActivated` + `HasXInActivationCost`) uses the same source-keyed `pending_activation_cost_has_x` lookup. At base it fires falsely on a mana ability whose source has a pending {X} activation. B1's fix corrects it as a side effect.

