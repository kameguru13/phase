# Phase 2 plan, revision 1 — Every combat pause asked (cz86-rework-9670)

Mode: `engine-planner` **phase-plan mode** for charter phase **P2**. Inputs read: `task.md`,
`phase-charter` (frozen; P2 entry, deferral list, seam notes), `addenda/phase-2` (empty),
`phase-inputs/phase-2`, `phase-inputs/run-level`, `phase-1-accepted-summary.md`,
`maintainer-answer-2026-11-17T1351.md`, `pr-body.md`, `class/`, and plan-r1 §2 seam 3 / §13 (the
rows the charter points at). Attempt history: pre-charter plan r1 = 2 behavior + 1 text; charter r1
= 2 corrections (frozen); P1 plan r1 = 2 behavior, r2 = 1 text, then clean; P1 implementation
review r1 = 0 findings, P1 accepted. This is P2 plan r1. Budget: standard.

- `PHASE_BASE_SHA` = P1's accepted candidate **`00396db22d`** (HEAD of
  `engine/unpreventable-damage-riders`). Every "today" claim below was read at that commit.
- **Probe status.** The planning worktree is read-only and a full test run holds the target dir, so
  **no claim here was probed by running code.** Each runtime claim is read from source and named by
  function, or listed in §18 (Unprobed) with the test that settles it. §3 lists the four claims
  the executor establishes first, before any edit depends on them.
- Q1 is outcome A and touches only P3. Q2/Q4 were accepted in P1 and P2 changes neither.

---

## 0. Premise (Step 0)

Oracle text read from MTGJSON `AtomicCards.json` `5.3.0+20261021`, verbatim:

- **Channel Harm**: "Prevent all damage that would be dealt to you and permanents you control this
  turn by sources you don't control. If damage is prevented this way, you may have Channel Harm deal
  that much damage to target creature." Rulings include: "The new damage is not combat damage, even
  if the prevented damage was." and the two-Channel-Harm loop "either player can break by choosing
  to not have Channel Harm try to deal damage" (class notes P3, P4).
- **Awe Strike**: "The next time target creature would deal damage this turn, prevent that damage.
  You gain life equal to the damage prevented this way."
- **Gatta and Luzzu**: "Flash / When Gatta and Luzzu enters, choose target creature you control. If
  damage would be dealt to that creature this turn, prevent that damage and put that many +1/+1
  counters on it."
- **Temper**: "Prevent the next X damage that would be dealt to target creature this turn. For each
  1 damage prevented this way, put a +1/+1 counter on that creature."
- **Phytohydra**: "If damage would be dealt to this creature, put that many +1/+1 counters on it
  instead."
- **Undergrowth Champion**: "If damage would be dealt to this creature while it has a +1/+1 counter
  on it, prevent that damage and remove a +1/+1 counter from this creature. / Landfall — …" Ruling:
  "If Undergrowth Champion is dealt damage from multiple sources at the same time (for example, if
  it's blocked by multiple creatures), its prevention effect will apply once to all that damage.
  You'll remove only one +1/+1 counter."
- **New Way Forward**: "The next time a source of your choice would deal damage to you this turn,
  prevent that damage. When damage is prevented this way, New Way Forward deals that much damage to
  that source's controller and you draw that many cards."
- **Swans of Bryn Argoll**: "Flying / If a source would deal damage to this creature, prevent that
  damage. The source's controller draws cards equal to the damage prevented this way."
- **Abundance**: "If you would draw a card, you may instead choose land or nonland and reveal cards
  from the top of your library until you reveal a card of the chosen kind. Put that card into your
  hand and put all other cards revealed this way on the bottom of your library in any order."
- **Hardened Scales**: "If one or more +1/+1 counters would be put on a creature you control, that
  many plus one +1/+1 counters are put on it instead." Ruling: "If two or more effects attempt to
  modify how many counters would be put on a creature you control, you choose the order to apply
  those effects, no matter who controls the sources of those effects." Also: "Each additional
  Hardened Scales you control will increase the number of +1/+1 counters placed on a creature you
  control by one."
- **Doubling Season**: "If an effect would create one or more tokens under your control, it creates
  twice that many of those tokens instead. / If an effect would put one or more counters on a
  permanent you control, it puts twice that many of those counters on that permanent instead."
- **Rhox Faithmender**: "Lifelink … / If you would gain life, you gain twice that much life
  instead." **Leyline of Hope**: "… If you would gain life, you gain that much life plus 1 instead.
  …"
- **Dralnu, Lich Lord**: "If damage would be dealt to Dralnu, sacrifice that many permanents
  instead. / {T}: …"; **Nefarious Lich**: "If damage would be dealt to you, exile that many cards
  from your graveyard instead. If you can't, you lose the game. / …"
- Vanilla bodies used by the new rows (all real, no rules text): **Grizzly Bears** 2/2, **Hill
  Giant** 3/3, **Centaur Courser** 3/3, **Rumbling Baloth** 4/4, **Eager Cadet** 1/1.
  **Child of Night**: "Lifelink". **Lightning Bolt**: "Lightning Bolt deals 3 damage to any
  target."

CR numbers used in this plan, each grep-verified in `docs/MagicCompRules.txt`: 101.4, 104.1,
117.2e, 117.4, 120.8, 121.2, 510.1c, 510.2, 510.3, 510.3a, 510.4, 603.2, 603.3, 603.3b, 608.2c,
608.2d, 608.2f, 609.7b, 614.5, 614.6, 615.1a, 615.5, 615.7, 615.13, 616.1, 616.1e, 616.1f,
616.1g, 700.1, 701.21a, 702.4b, 702.15b, 704.3, 800.4a, 800.4e.

## 1. Pattern Coverage (Step 0.5: the charter's attribution, cited, not re-run)

Charter record: `jq-1.8.1`, MTGJSON `5.3.0+20261021`, `class/query.sh`.
- **Class B** (prevention with additional effects): `class/prevention_rider_class.json`, 39 cards,
  110 distinct rulings, hash `a49c1dc6…e5de6b5`. P2 owns its **aggregate (Phase D) riders** and
  **every rider prompt inside a combat batch**: Channel Harm, Awe Strike, Temper, Test of Faith,
  Brace for Impact, Inkshield, New Way Forward, Deflecting Palm, Acolyte's Reward, Swans of Bryn
  Argoll, Outfitted Jouster and the rest of the 39 whose rider can pause.
- **Class C** (CR-defined combat replacement/prevention choices, CR 510.2 + CR 616.1): A ∪ B minus
  the three noncombat-only = 48. P2 owns the aggregate and in-leg rider asks of this class.
- **Guards** (charter decision 4; CR 608.2f + CR 700.1): every printed self-scoped "while" damage
  shield `read_damage_guards` reads (Undergrowth Champion, Polukranos, Oathsworn Knight, Ugin's
  Conjurant, Jared Carthalion, Outfitted Jouster; pr-body §2). The Undergrowth Champion ruling above
  is the bearing ruling.
- Bearing rulings attributed to P2 by the charter (class/notes.txt): **P1, P2** (riders
  "immediately afterward … before that spell or ability resumes"), **P3** (Channel Harm's damage is
  not combat damage), **P4** (Channel Harm's "may" is a real choice), **P7** (Test of Faith /
  Temper counters at the same time as the prevention), **P8** (state-based actions after the
  batch), **S11** (Dralnu's sacrifice is a choice). Each drives a matrix row in §12 (named there).

The pattern stop (Step 4) is assessed against the charter's class (39 / 48 cards), not P2's diff.
No row is built for one card: every behavior below is the combat batch's rider drain, park or
resume, which every class member with a pausing rider reaches.

## 2. What the P1 base does today (read at `00396db22d`, named by function)

- `replacement::drain_damage_rider` routes combat (`state.combat_prevention_tally.is_some()`) to
  `drain_combat_damage_rider`, which calls `settle_combat_rider_pauses`: an order park takes
  candidate 0, a "may" is declined, a rider's own prompt is abandoned (`CombatRiderPause`,
  `answer_combat_rider_park`, `abandon_combat_rider_pause`). It is reached from the substitution
  arm of the damage applier (the in-leg earlier rider) and from
  `combat_damage::fire_combat_prevention_riders` (Phase D).
- `replacement::drain_combat_damage_continuation` (the leg's own continuation) suspends the ledger
  and calls `apply_pending_post_replacement_effect` directly; a pause becomes
  `CombatDamageReplacementStep::ContinuationPaused` and parks the pass (asked since P1).
- `replace_combat_damage_batch` reads `read_damage_guards(state)` on **every entry**.
- `combat_damage::drain_combat_lifelink` parks only on a lifelink gain's
  `ReplacementDeferred::ReplacementChoice`; Phase D (`fire_combat_prevention_riders`) runs every
  shield in one call and returns nothing.
- The resume authority is `combat_damage::resume_pending_combat_lifelink`. It is reached from
  `engine_replacement::handle_replacement_choice` (a `ChooseReplacement` answer, in the answering
  action), from `combat_damage::resolve_combat_damage`'s re-entry branch (only when the turn
  machine re-enters the step after **all players pass priority**, priority.rs
  `combat_damage_incomplete`), and from `turns.rs`'s departed-active-player path.
- `coverage::StructuralFeature::DamageReplacementChoiceUnaskedInCombat` fences the cards whose rider
  asks a choice in combat.

## 3. Claims the executor establishes first (before the edit that depends on each)

- **E1 — an answer that is not a `ChooseReplacement` does not resume a parked combat batch in its
  own action.** Read: `apply_non_priority_pass_action`'s post-match code and
  `engine::resume_pending_continuation_if_priority` never reach `resume_pending_combat_lifelink`;
  only `handle_replacement_choice` and the priority-pass re-entry do. So after P1's
  `dralnus_sacrifice_choice_is_asked_inside_the_batch` answers the `EffectZoneChoice`, the action
  returns `WaitingFor::Priority` with `state.pending_combat_lifelink` still `Some`, and the batch
  completes only after both players pass (a priority window between combat damage being assigned
  and dealt, against CR 510.2). **Measure it** at `PHASE_BASE_SHA` by adding the helper invariant of
  Step B1 (§16) to `fight_driving` first and running that one test: it must fail with
  "a parked combat batch outlived its answer". If it passes, E1 is false: drop Step B6 (the
  action-boundary call) and report it; nothing else changes. This decides D5 only.
- **E2 — a post-replacement dispatch that pauses leaves its prompt in `state.waiting_for`.** The
  noncombat batch already relies on this (`run_damage_batch`'s `damage_batch_has_paused_rider` arm
  parks and returns with the rider's prompt standing), and D2 reads `state.waiting_for` for the
  in-leg park. Settled by §16 row N-2's first counter prompt (its `ReplacementChoice` must offer
  exactly Hardened Scales and Doubling Season). Guarded in code by a `debug_assert!`.
- **E3 — two Awe Strikes cast in order fire their Phase D riders in cast order.** Both are
  `pending_damage_replacements` entries (`AppliedReplacementKey::Floating { index }`), and
  `apply_combat_damage_survivors` sorts the tally by `(1, 0, index, 0)` for floating keys. Settled by
  row N-1's prompt-2 life snapshot. The order between two shields is an engine convention (§13).
- **E4 — Channel Harm's "may" surfaces as `WaitingFor::OptionalEffectChoice { player: P0, .. }`
  answered by `GameAction::DecideOptionalEffect { accept }`.** Settled by row C-D3; if the engine
  raises another wait for that "may", the executor answers that wait in the test closure, and the
  derived values do not change.

## 4. The P2 decisions, as carried out

### D1 — One combat rider drain (charter decision 1)

`replacement::drain_combat_damage_rider(state, events) -> Option<WaitingFor>`: suspend the ledger,
run the resident post-replacement work, restore the ledger, return the pause. It settles nothing and
does not restore any earlier wait. It is built on one shared primitive,
`replacement::dispatch_damage_rider_outside_batch(state, events) -> Option<PostReplacementPause>`
(CR 700.1: the rider's event is independent of the batch), which `drain_damage_rider` uses too, so
the take/restore of the ledger lives in one place.

- Callers: `drain_combat_damage_continuation` (the leg's own continuation; it stops calling
  `apply_pending_post_replacement_effect` itself) and `fire_combat_prevention_riders` (Phase D).
- `drain_damage_rider` loses its combat branch and becomes one body for both: dispatch outside the
  batch; on a pause record the frame in `DamageBatchLedger::paused_rider` and return
  `DamageRiderDrain::Paused { owner }`. In combat this is reached only from the substitution arm
  (deal_damage.rs's other callers are noncombat), and it now lets the earlier rider's prompt stand.
- Deleted: `CombatRiderPause` (enum + impl), `settle_combat_rider_pauses`,
  `answer_combat_rider_park`, `abandon_combat_rider_pause`, and the old body of
  `drain_combat_damage_rider`.

### D2 — In-leg earlier rider parks the leg (charter decision 2; CR 615.5 + CR 608.2c)

When the substitution arm's earlier-rider drain pauses inside a leg, the applier already returns
`ApplyResult::PreventedBehindRider(PausedRider { owner })` and the substitution's own drain is
queued behind that frame (`stash_post_replacement_continuation` →
`install_post_replacement_drain_behind_paused`). P2 adds the leg-side half in
`replacement::settle_combat_damage_replacement`, ahead of the continuation drain:

1. `NeedsChoice` → `NeedsChoice` (unchanged).
2. CR 104.1: `state.game_end.is_some()` → clear `paused_rider`, `Settled(survivor)`.
3. CR 615.5 + CR 608.2c: `take_combat_leg_paused_rider(state)` (new private fn: `take()` the
   ledger's `paused_rider`, `is_some()`) → `ContinuationPaused { survivor, waiting_for:
   Box::new(state.waiting_for.clone()) }`, with
   `debug_assert!(!matches!(state.waiting_for, WaitingFor::Priority { .. }))` (E2).
4. Otherwise `drain_combat_damage_continuation(state, survivor, events)`.

The check must precede the drain: with a paused resident and a Ready entry queued behind it, a drain
here would run the substitution's actions ahead of the rider's unfinished work. On the round trip,
the answer finishes the rider, `effects::resume_resolution_frames` →
`dispatch_resident_post_replacement_work` runs the queued substitution actions (which may ask their
own order), and once the action settles at priority the pass resumes through main's
`AwaitingContinuation` arm (`drain_combat_damage_continuation`, which then finds nothing Ready and
settles the leg). Nothing else changes in `replace_combat_damage_batch`.

### D3 — Phase D parks the record like the lifelink-gain pause (charter decision 3)

`combat_damage::fire_combat_prevention_riders(state, prevention_tally: &mut Vec<(AppliedReplacementKey,
i32)>, events) -> Option<WaitingFor>`:

- `while !prevention_tally.is_empty()`: CR 104.1 `break` if `state.game_end.is_some()`; then
  `let (key, total_prevented) = prevention_tally.remove(0);` — the shield leaves the record
  **before** its rider runs, so a resume never fires it again. The body is unchanged through the
  `install_ready_continuation` and the `event_source` seed.
- The same `resident_mut` block that seeds `event_source` also sets
  `drain.event_amount = Some(total_prevented as u32)` (`u32::try_from`, the value is `> 0` here):
  CR 615.5 "the amount of damage that was prevented", latched on the drain so the dispatch-bound
  reader (P1's seam 1) owns it across a pause. `state.last_effect_count = Some(total_prevented)` stays.
- `if let Some(waiting_for) = replacement::drain_combat_damage_rider(state, events)` and
  `state.game_end.is_none()` → `return Some(waiting_for)`. A pause that ended the game falls through
  to the loop top's CR 104.1 break.
- Returns `None` when every shield ran.

`combat_damage::drain_combat_lifelink`: after appending `damage_to_players`, when
`state.game_end.is_none()` and `fire_combat_prevention_riders(state, &mut record.prevention_tally,
&mut record.batch_events)` returns `Some(waiting_for)`: set `state.waiting_for =
waiting_for.clone()`, park `state.pending_combat_lifelink = Some(Box::new(record))` (its
`remaining` and `damage_to_players` are already empty, `replacement` and `phase_c_remnant` are
`None`, `prevention_tally` holds the unfired shields), return `CombatDamageBatch::Paused { events:
batch_events.clone(), waiting_for: Box::new(waiting_for) }` — the shape of its lifelink arm.

Resume needs no new code: `resume_pending_combat_lifelink` → `resume_combat_damage_replacements`
(`replacement: None`) → `drain_combat_lifelink` → no gains, an empty aggregate, Phase D continues
with the remaining shields once. CR 510.2 forbids a priority window, not the rider's choice
(CR 615.5, CR 608.2d).

### D4 — Guards read once across a park (charter decision 4; CR 608.2f + CR 700.1)

- `replacement::CombatDamageReplacementPass` gains `pub(crate) settled_guards:
  Vec<SettledDamageGuard>`. `CombatDamageReplacementPass::new(state: &GameState, events)` sets it
  from `read_damage_guards(state)`: the guards are read once, when the batch's pass is created.
- `replace_combat_damage_batch` installs `settled_guards: std::mem::take(&mut pass.settled_guards)`
  in place of today's fresh `read_damage_guards(state)`, and on exit moves **both** `applications`
  and `settled_guards` back into the pass (`if let Some(ledger) = state.damage_batch_ledger.take()
  { pass.one_shot_applications = ledger.applications; pass.settled_guards = ledger.settled_guards; }`).
  Its doc's "a fresh guard reading" sentence is rewritten.
- `types::game_state::PendingCombatDamageReplacement` gains
  `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub settled_guards:
  Vec<crate::types::ability::SettledDamageGuard>` (doc: CR 608.2f + CR 700.1, the Undergrowth
  Champion ruling; absent from saves written before it existed).
- `combat_damage::run_combat_damage_replacements`' Paused arm parks `settled_guards:
  pass.settled_guards`; `resume_combat_damage_replacements` rebuilds the pass with
  `settled_guards: parked.settled_guards`; `apply_combat_damage` builds the pass with
  `CombatDamageReplacementPass::new(state, proposed_events)`.
- Compiler-forced: the two unit tests calling `CombatDamageReplacementPass::new([..])`
  (`effects/prevent_damage.rs` and the `record_proposed_events` test in `replacement.rs`) pass
  `&state` first.

A rider's own damage (Channel Harm's) still reads guards live: the drain suspends the ledger
(CR 700.1). A later batch creates a new pass and reads afresh.

### D5 — The answer resumes the batch (charter goal: "never auto-settled", CR 510.2; refines decision 3)

Contingent on E1. `combat_damage::resume_parked_combat_damage_after_answer(state, waiting_for:
&WaitingFor, action_event_start: usize, events) -> Option<WaitingFor>`: when `waiting_for` is
`Priority` and `state.pending_combat_lifelink.is_some()`, set `state.waiting_for`, call
`resume_pending_combat_lifelink(state, action_event_start, None, events)` and return its wait
(`None` otherwise). One call site, in `engine::apply_non_priority_pass_action`, after
`resume_parked_counter_cost_root_at_action_boundary` and before the post-action pipeline, with
`action_event_start = events.len()` captured just before the action `match`.

Why: once D1–D3 let a rider's own prompt stand (Channel Harm's "may", Dralnu's sacrifice, a
two-Equipment Outfitted Jouster's choice, Nefarious Lich's exile), the batch must finish in the
action that answers it. Otherwise the answer returns a priority window with damage unassigned
(Phase B, CR 510.2) or with prevention riders unrun and combat-damage triggers not yet on the stack
(Phase D; CR 615.5, CR 510.3, CR 510.3a). This is the same authority `handle_replacement_choice`
already calls for a `ChooseReplacement` answer, with the same event anchor and the same
`claim_combat_lifelink_batch_events_for_ordinary_collection`; a `ChooseReplacement` answer leaves
nothing parked at `Priority`, so the new call is a no-op after it. The priority-pass re-entry in
`resolve_combat_damage` stays for the departure paths (CR 800.4a), and its comment is narrowed to
them.

This touches `crates/engine/src/game/engine.rs`, outside P2's anchor paths. It leaves decision 3
standing (the record parks like the lifelink pause and resumes through
`resume_pending_combat_lifelink`); it adds the call site the answering action needs. **For the
orchestrator: an addendum line, not a re-charter** (§19).

### D6 — Deletions and documents (charter decision 5)

- Coverage (`game/coverage.rs`): `StructuralFeature::DamageReplacementChoiceUnaskedInCombat` (its
  doc, `tag()` arm, `support()` arm and MEASURED comment), `RiderTargets`,
  `damage_replacement_reaches_combat`, `damage_replacement_asks_a_choice_in_combat`,
  `prevention_rider_asks_a_choice_in_combat`, `damage_rider_asks_a_choice`,
  `damage_rider_effect_asks_a_choice`, the `prevention_rider_asks_a_choice_in_combat` block in
  `extract_ability_features_with_token_statics`, the combat half of
  `extract_damage_replacement_choice_feature` (it keeps the substitution half), the unit tests
  `a_damage_riders_instruction_asks_for_a_picked_player_an_up_to_count_or_a_population` and
  `a_damage_replacement_choice_combat_cant_ask_is_a_coverage_gap`, and the deleted test's doc
  paragraph that sits misplaced above `a_shared_recipient_damage_instruction_is_a_coverage_gap`
  (the "CR 608.2d + CR 510.2: a combat-reachable damage replacement … combat never meets it."
  paragraph; that test keeps its own CR 615.7 paragraph). Imports left unused (`CombatDamageScope`,
  top-level `SubAbilityLink`) are removed; check each with a grep before removing.
- `game/resolution_prompt.rs`: `quantity_offers_up_to_choice` and `effect_offers_choice` return to
  private, as on main; the branch widened them only for the deleted classifier. (Its
  `ResumeDamageBatch` arm stays.) Optional cleanup, needing an addendum (§19).
- The strict-failure module-doc paragraphs and section headers citing Command Zone #140 in the three
  test files are rewritten (§16, Step C2).
- Doc truths updated: `DamageBatchLedger::paused_rider` (now set in combat: the leg parks on it),
  `DamageRiderDrain` variants, `drain_damage_rider`, `PendingCombatLifelink` and
  `PendingCombatLifelink::prevention_tally` (a Phase D park keeps the unfired shields),
  `CombatDamageBatch::Paused` (a fourth park point: a Phase D rider's prompt),
  `resolve_combat_damage`'s re-entry comment, and comment-only lines in
  `engine_replacement::handle_replacement_choice` ("the only boundary that knows where THIS action's
  events begin") and `tests/integration/combat_lifelink_replacement_ordering.rs` (the
  `&[(AppliedReplacementKey, i32)]` parameter line).

## 5. Sizing

| Unit | Behavior | Registration surfaces | Discriminating tests |
|---|---|---|---|
| **U1 Every combat rider pause asked** | D1 one drain; D2 in-leg park; D3 Phase D park; D5 answer resumes; D6 auto-settle and fence deleted | replacement.rs, combat_damage.rs, engine.rs (one call), coverage.rs, resolution_prompt.rs, game_state.rs (docs), three test files, census baseline | N-1 (two aggregate shields), N-2 (in-leg earlier rider), N-4 (departure at a Phase D pause), C-D1…C-D5 conversions, the helper invariant over every combat row |
| **U2 Guards read once across a park** | D4 | replacement.rs, combat_damage.rs, game_state.rs (serialized field), prevent_damage.rs + replacement.rs unit tests (compiler-forced), protocol record | N-3 (Undergrowth Champion across a park, restored from serialization at the second prompt) |

Dependency edges: none. U2 and U1 share files but neither needs the other to be tested (N-3 parks
on a per-leg continuation, which P1 already asks). Order: U2 first (smaller, independent), then U1.

Scope paths under the phase-fit counting rule (directories expanded, fixtures/regenerated
data/protocol surfaces excluded):

1. `crates/engine/src/game/replacement.rs`
2. `crates/engine/src/game/combat_damage.rs`
3. `crates/engine/src/game/coverage.rs`
4. `crates/engine/src/types/game_state.rs`
5. `crates/engine/src/game/engine.rs` — outside the charter's anchors (D5, addendum)
6. `crates/engine/src/game/resolution_prompt.rs` — outside the anchors (D6 optional, addendum)
7. `crates/engine/src/game/effects/prevent_damage.rs` — compiler-forced (D4)
8. `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs`
9. `crates/engine/tests/integration/damage_substitution_replacements.rs`
10. `crates/engine/tests/integration/ironscale_hydra_combat_prevention.rs`

Comment-only (standing class): `crates/engine/src/game/engine_replacement.rs`,
`crates/engine/tests/integration/combat_lifelink_replacement_ordering.rs`. Excluded:
`scripts/post-replacement-continuation-baseline.txt` (regenerated by its census),
`protocol/changes/ccf4c183-5976-4e18-905c-68b9afee9848.json` (protocol surface).

**Count: 10 counted paths (12 with the two comment-only files). T1: 2 units. T2: 12 < 13, so the
T1∧T2 conjunction does not fire.** Dropping the optional `resolution_prompt.rs` cleanup gives 9 (11).

## 6. Building Blocks

- `replacement::{replace_combat_damage_batch, settle_combat_damage_replacement,
  drain_combat_damage_continuation, CombatDamageReplacementPass, read_damage_guards,
  settled_damage_guard_reading, damage_rider_placement, stash_post_replacement_continuation}` — the
  pass and ledger P1 kept; D2 adds one check, D4 one field.
- `engine_replacement::dispatch_pending_post_replacement_effect` / `PostReplacementPause` — the
  single dispatcher both drains call through `dispatch_damage_rider_outside_batch`.
- `combat_damage::{drain_combat_lifelink, resume_pending_combat_lifelink,
  resume_combat_damage_replacements, run_combat_damage_replacements,
  claim_combat_lifelink_batch_events_for_ordinary_collection}` — main's #9670 park and resume; D3
  copies the lifelink arm, D5 reaches the same resume.
- `effects::{resume_resolution_frames, dispatch_resident_post_replacement_work}` — the round trip
  that runs work queued behind a paused rider (D2), unchanged.
- `GameState::install_post_replacement_drain_behind_paused`, `active_post_replacement_drains_mut`,
  `PostReplacementDrainStack::resident_mut` — unchanged.
- Tests: `unpreventable::{fight, fight_answering, preferring, offered, scales_and_season,
  rider_board, assert_batch_settled, bolt_at, cast_by_p1, drive}`, `damage_substitution::
  {combat_events, combat_answering, choose_shield, choose_source, answer_counter_orders, temper,
  cast_temper_at_one, phytohydra}`, main's Szadek pattern `assert_batch_completed_by_the_answer` /
  `round_trip_at_pause` (re-expressed in the helpers, not imported).

No new helper duplicates an authority: `dispatch_damage_rider_outside_batch` is the extraction of
the ledger take/restore both drains already do; `resume_parked_combat_damage_after_answer` is a
guard around the existing resume authority.

## 7. Logic Placement

| Behavior | Owning function | Delegating callers |
|---|---|---|
| A damage rider runs outside the batch's ledger | `replacement::dispatch_damage_rider_outside_batch` | `drain_damage_rider`, `drain_combat_damage_rider` |
| A combat rider's pause is returned, never settled | `replacement::drain_combat_damage_rider` | `drain_combat_damage_continuation`, `combat_damage::fire_combat_prevention_riders` |
| An in-leg earlier rider's pause parks the leg | `replacement::settle_combat_damage_replacement` | every leg step: `replace_combat_damage_batch` (start, AwaitingChoice answer), `repropose_combat_damage_leg` |
| Phase D parks and resumes once | `combat_damage::fire_combat_prevention_riders` (pop, run, return pause) + `drain_combat_lifelink` (park) | `apply_combat_damage_survivors`, `resume_combat_damage_replacements` |
| Guards read once per batch | `CombatDamageReplacementPass::new` | `apply_combat_damage`; carried by `run_combat_damage_replacements` / `resume_combat_damage_replacements` |
| The answer completes the parked batch | `combat_damage::resume_parked_combat_damage_after_answer` → `resume_pending_combat_lifelink` | `engine::apply_non_priority_pass_action` (one call); `handle_replacement_choice` keeps its own call |

Consumers whose cardinality changes with the same signatures: `drain_damage_rider`'s combat callers
(the substitution arm) can now see `Paused`; `resolve_combat_damage`'s re-entry branch now meets a
Phase D record (`replacement: None`, empty `remaining`), which it resumes through the same path;
`elimination`'s departure handling of `pending_combat_lifelink` now meets a Phase D record too (row
N-4).

## 8. Rust Idioms

`Option<WaitingFor>` for "the pause it raised", as `drain_combat_damage_continuation` and
`apply_pending_post_replacement_effect` already return; `&mut Vec` popped from the front for the
serialized tally (a `Vec`, not a `VecDeque`, because the record's type is unchanged); no new enum;
exhaustive matches untouched. `u32::try_from(total_prevented)` for the latched amount. No `bool`
parameters.

## 9. Nom Compliance

No file under `crates/phase-parser/src/parser/` changes. No Oracle text is newly accepted.

## 10. Extension vs Creation

Extension throughout: the Phase D park is main's lifelink park applied to a second producer; the
in-leg park is main's `AwaitingContinuation` leg state; the guards ride the pass next to its
one-shot applications; the answer resume reuses `resume_pending_combat_lifelink`. The only new call
site (D5) exists because the answering action is the one boundary that holds the event anchor, as
`handle_replacement_choice`'s own doc explains.

## 11. Analogous Trace (Step 2)

Traced main's #9670 lifelink-gain park end to end, the model D3 copies (charter Step 2):
`combat_damage::apply_combat_damage` → `run_combat_damage_replacements` →
`apply_combat_damage_survivors` → `drain_combat_lifelink` (`Err(ReplacementDeferred::ReplacementChoice)`
arm: `state.pending_combat_lifelink = Some(Box::new(record))`, `CombatDamageBatch::Paused`) →
`resolve_combat_damage` (first-strike / regular arms surface `waiting_for`) → answer →
`engine_replacement::handle_replacement_choice` (generic inner handler, then the `Priority` tail
`resume_pending_combat_lifelink(.., None, ..)`) → `resume_combat_damage_replacements`
(`replacement: None` → `drain_combat_lifelink`) → `finish_combat_damage_sub_step`. Types:
`types/game_state.rs::{PendingCombatLifelink, PendingCombatDamageReplacement,
PendingCombatLegProgress}`.

Traced the leg continuation park P1 inherited from main (D2's model):
`replace_combat_damage_batch` → `settle_combat_damage_replacement` →
`drain_combat_damage_continuation` → `ContinuationPaused` → `PendingCombatLegProgress::AwaitingContinuation`
→ answer → `effects::resume_resolution_frames` → resume → `take_surfaced_leg` →
`drain_combat_damage_continuation`.

Traced the noncombat earlier-rider queue (D2's other half): `apply_damage_done_replacement`'s
substitution arm (`drain_damage_rider` → `ApplyResult::PreventedBehindRider`) →
`apply_single_replacement` (`damage_rider_placement` → `stash_post_replacement_continuation` →
`install_post_replacement_drain_behind_paused`) → `effects::resume_resolution_frames`
(`PostReplacement` arm, `queued_behind`) → `dispatch_resident_post_replacement_work`; witness
`damage_substitution_replacements::an_earlier_riders_paused_counters_finish_before_the_substitution`.

Traced guard reading: `read_damage_guards` → `DamageBatchLedger::settled_guards` →
`settled_damage_guard_reading` (in `evaluate_replacement_condition`'s path) and the noncombat
carrier `SpellContext.settled_damage_guards` / `with_damage_batch`, whose serialized pause row is
`unpreventable::the_batch_reading_survives_serialization_at_the_pause`.

## 12. Verification Matrix (with Identity / Provenance)

Production entry for every runtime row: `GameRunner` driving a real combat (declare attackers →
blockers → `AssignCombatDamage` → the batch) through `fight_driving`/`fight_answering` or
`combat_events`; completion boundary: the answering action completes the batch
(`pending_combat_lifelink.is_none()` with `combat.regular_damage_done` at that `Priority`), then
`assert_batch_settled` (no park, no frame, EndCombat priority with an empty stack), then a
post-batch Lightning Bolt for state-based actions where the row has one.

| Row | Claim (CR, ruling) | Seam | Test | Revert-failing assertion | Positive reach-guard | Hostile / sibling | Identity / provenance |
|---|---|---|---|---|---|---|---|
| V1 | Every pause a combat rider raises is answered by its player; the answer completes the batch in the same action (CR 510.2, CR 510.3, CR 615.5) | D5 + D1 | Helper invariant in `fight_driving` and `combat_answering` (Step B1), over every combat row of the three files | After an answer that leaves `Priority`, `pending_combat_lifelink.is_none()` (fails at base on `dralnus_sacrifice_choice_is_asked_inside_the_batch`, E1) | the row's own prompt was asked (each row records it) | `ChooseReplacement` answers (already resumed: no-op); answers that leave another prompt (no resume); departure (N-4, the re-entry fallback) | anchor = start of the answering action's events, a local |
| V2 | Phase D asks its riders, one shield at a time; the second shield's rider runs once, after the first one's answer (CR 615.5, CR 616.1; P1, P2) | D3 | **N-1** `two_aggregate_riders_are_asked_one_after_the_other` | 2 prompts; life at prompt 2 is 25 / 26; final life exact per answer pair (§16) | `prevented == 2`, P1 dealt no combat damage | one shield (C-D1); a shield with tally 0 (C-D3 unblocked); serialization at the Phase D park (prompt 2) | tally key `Floating { index }` popped before the rider; order = cast order (E3, engine convention) |
| V3 | An earlier rider's prompt inside a leg parks the leg; the substitution's actions wait behind it and run on the round trip (CR 615.5 + CR 608.2c, CR 616.1f) | D2 | **N-2** `an_earlier_riders_pause_inside_a_combat_leg_is_asked` | 3 prompts in order shield-order / Temper's counter / Phytohydra's counters; counters per answer pair | first prompt offers Temper and Phytohydra; at prompt 2 `pending_combat_lifelink` has `replacement.is_some()` and Phytohydra has 0 counters | Phytohydra first (no earlier rider; P1's path); no Hardened Scales (rider completes, no park) | `PausedRider { owner }` frame; the queued drain keeps its own latched amount (P1 seam 1) |
| V4 | Guards are read once, when the batch opens, and survive a park and serialization (CR 608.2f + CR 700.1; Undergrowth Champion ruling) | D4 | **N-3** `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park` | Champion `damage_marked == 3`; no prompt offers the Champion; counters 6 / 5 (fails at base: re-read at resume gives damage 1) | Temper applied (`prevented == 1`, counters > 0: the live guard did flip) | in-memory vs serialized at prompt 2; existing no-park `undergrowth_champion_double_blocked_reads_its_guard_once` (reading Holds) | `SettledDamageGuard { source: ObjectIncarnationRef, condition, reading }`; bound at `CombatDamageReplacementPass::new`; stored in the pass, the parked `PendingCombatDamageReplacement.settled_guards`, the ledger while running; consumed by `settled_damage_guard_reading`; dropped when the pass completes |
| V5 | A rider's own "may" in Phase D is its controller's (CR 608.2d; P3, P4) | D3 + D5 | **C-D3** `channel_harms_may_is_asked_after_the_batch` | accept → Giant dealt 3 by Channel Harm, `is_combat: false`, dies; decline → Giant 2 damage, alive | blocked: `prevented == 1` | unblocked: nothing prevented, no prompt, P1 at 18 | latched `event_amount = 3` on the Phase D drain |
| V6 | A rider's event "may" at Phase D (Abundance on New Way Forward's draws) is asked of the drawing player (CR 616.1, CR 121.2) | D3 | **C-D4** `new_way_forwards_draws_ask_abundance` | 2 prompts to P1; `CardDrawn` (P1) 2 decline / 0 accept | P0 life −2 exactly once | no Abundance: 2 draws, no prompt | chosen source P0's Bears (`event_source` seed) |
| V7 | Counter order of a Phase D rider asked (CR 616.1; Hardened Scales ruling) | D3 | **C-D1** `gatta_and_luzzus_counter_order_is_asked_after_the_batch`, **C-D2** `a_lifelink_answer_then_gatta_and_luzzus_rider` | exact counters per answer (§16) | `prevented == 1`; lifelink +5 (C-D2) | no Hardened Scales: 1 counter, no rider prompt | — |
| V8 | Per-leg and Phase D prompts in one batch (CR 616.1, CR 608.2d) | D1 + D3 + D5 | **C-D5** `a_may_park_and_a_riders_own_prompt_are_both_asked` | Abundance asked per draw, then Channel Harm's "may"; accept → Giant dealt 7 noncombat, dies | `prevented == 2` | no Abundance: only Channel Harm's prompt | Channel Harm's aggregate amount 3 + 4 |
| V9 | A rider prompt whose player leaves the game: the batch still finishes (CR 800.4a, CR 104.1 not reached) | D3 + fallback | **N-4** `a_phase_d_rider_prompt_whose_player_leaves_lets_the_batch_finish` | after P1 concedes at its Phase D prompt, pumping ≤ 16 actions drains the record, `regular_damage_done`, no further prompt to P1 | P1 was asked (`ReplacementChoice { player: P1 }` with `replacement.is_none()` record) | P2's life 20, P0's life 20 | — |
| V10 | CR 104.1 between Phase D shields (kept) | D3 | existing `a_prevention_rider_that_ends_the_game_ends_the_batchs_riders` (unchanged) | Awe Strike's gain never happens after Deflecting Palm's rider ends the game | game over, P0 eliminated | — | — |
| V11 | Coverage: the fence is gone and no class member is claimed supported by mistake | D6 | `cargo coverage` at run-level verification (charter decision 5) | flips listed; expected: Battletide Alchemist and Channel Harm leave `damage_replacement_choice_unasked_in_combat`; Dralnu and Nefarious Lich stay unsupported through `damage_substitution_choice_incomplete`; Outfitted Jouster through its confirmed-misparse row (pr-body line 69) | the deleted unit tests' card list | — | — |

Ruling → row: P1, P2 → V2, V3; P3, P4 → V5, V8; P7 → V3, V4; P8 → V2 (Bolt after the batch);
S11 → V1 (Dralnu row). Coverage status impact: V11. No Oracle text is newly accepted.

`DEFERRED(phase 3)`: the substitution predicate fold, `ShieldKind::Substitution` deletion and the
fence `damage_substitution_choice_incomplete`'s re-key (outcome A). No P2 row depends on them.

## 13. Reference Readings

- **Order between two Phase D shields (N-1 snapshot)** — not derivable from the cards: both riders
  are P1's, of two prevention effects on two damage events. It is the engine's tally sort
  (`apply_combat_damage_survivors`, floating index = cast order), labelled so in the test. Totals
  per row do not depend on it.
- **Lifelink before Phase D riders (C-D2 prompt order)** — the record's documented emission order
  (gains, aggregate, Phase D), an engine convention; the test keeps asserting the first prompt
  offers Rhox Faithmender, as today.
- Every other expected value is derived from the card text and the CR in §16 and is not copied from
  the engine. The base's readings these rows replace (candidate 0, a declined "may", a cleared own
  prompt) disagree with the cards; they are the defect P2 removes, not references.
- **New Way Forward (C-D4)** — a derivation disagrees with the engine in a way P2 does not fix: "When
  damage is prevented this way, …" is a triggered ability (CR 603.2, CR 615.13) that goes on the
  stack before the active player gets priority (CR 603.3, CR 510.3a), while the engine runs it as a
  rider inside the batch — the same gap P1 recorded for Outfitted Jouster. Who is asked, how many
  cards and how much damage are the card's; the timing is not. The row keeps a `strict-failure`
  line naming that gap. Fixing it needs the parser and a trigger seam outside the charter (§19 Q-3).
- **Command Zone #174** (counters "for each 1 damage" parsed one counter per event): the converted
  Gatta rows move to a 1-power blocker (Eager Cadet) so one counter is prevented and the card and
  engine agree exactly; Temper rows use X = 1. #174 stays witnessed by the existing noncombat rows
  in `damage_substitution_replacements` (`an_earlier_riders_paused_counters_finish_before_the_substitution`).

## 14. Serialized surfaces

- `PendingCombatDamageReplacement.settled_guards` (new, `serde(default, skip_serializing_if =
  "Vec::is_empty")`). An older reader drops it and re-reads the guards at its next entry (silent
  capability loss).
- `PendingCombatLifelink` can now be parked in Phase D (empty `remaining`, `replacement: None`,
  partial `prevention_tally`): no new field; an older reader resumes it through the same path.
- Registry step: the branch already owns unreleased record
  `protocol/changes/ccf4c183-5976-4e18-905c-68b9afee9848.json` (surfaces `full`, `wire`) for "the
  damage-batch guard carrier". Extend its `reason` with: "; a combat damage batch parked in its
  replacement pass carries the guard reading it opened with
  (PendingCombatDamageReplacement.settled_guards), which an older reader drops and re-reads; a
  combat batch can park in its aggregate prevention riders with the shields not yet fired in
  prevention_tally". Then `node scripts/protocol-versions.mjs generate`, `node
  scripts/protocol-versions.mjs check`, `node scripts/check-protocol-version.mjs --base 91639fdf02`.
  No new UUID: the PR is unmerged and its one record already bumps these surfaces (§19 Q-2).
- `GameState` size: unchanged (`pending_combat_lifelink` is boxed); no `game_state_size.rs` entry.

## 15. Skills and checklist steps (Step 1)

- `/add-interactive-effect`: no new `WaitingFor`/`GameAction` (existing `ReplacementChoice`,
  `OptionalEffectChoice`, `EffectZoneChoice`); Phase 2 resolver/continuation = D1–D3; Phase 3 engine
  handler = D5; Phase 4 AI legal actions: unchanged prompts, existing candidates; Phase 5 routing:
  `acting_player()` of the existing prompts; Phase 6 frontend: none (no new prompt type); Phase 7
  visibility: `pending_combat_lifelink` is already filtered (`visibility.rs`); Phase 8 tests = §16
  (production round trip, continuation, decline, resume-state via V1).
- `/add-replacement-effect`: pipeline flow and post-replacement continuation (D1, D2), no new
  `ReplacementDefinition`.
- `/card-test`: production entry and completion boundary per §12.
- `/add-card-data-pipeline`: coverage fence removal; the coverage report is measured at run-level
  verification (charter decision 5); `card-data.json` is not regenerated in P2.
- `/add-engine-variant`: one coverage variant deleted; no sibling cluster forms; no inventory gate
  for a deletion. `DEFERRED(phase 3)`: the `ShieldKind::Substitution` deletion and its gate.

## 16. Tests

### 16.1 Helper changes (Step B1, first)

- `unpreventable_damage_prevention_effects.rs`: new `pub(super) fn fight_driving(runner, attackers,
  blocks, answer: impl FnMut(&mut GameRunner) -> GameAction) -> Combat` holds today's
  `fight_answering` loop; `fight_answering` becomes `fight_driving(.., |runner|
  answer(runner.state()))`. Inside the loop, an action taken at a wait other than `Priority`,
  `OrderTriggers`, `DeclareAttackers`, `DeclareBlockers` or `AssignCombatDamage` is an answer; after
  it, when the new wait is `Priority`, assert `runner.state().pending_combat_lifelink.is_none()` with
  the message "CR 510.2 + CR 615.5: a parked combat batch outlived its answer". Also record each
  prompt's chooser: `Combat` gains `choosers: Vec<PlayerId>` pushed with `prompts`.
- `damage_substitution_replacements.rs`: `combat_answering` gets the same invariant after each
  `choose_source`; `temper` and `cast_temper_at_one` become `pub(super)` (both files are registered
  in `tests/integration/main.rs`, so the cross-module use is allowed).

### 16.2 Converted rows (Phase D; charter test obligations)

| Id | Test (old → new name) | Board | CR / ruling | Today | Derived expectation, per answer |
|---|---|---|---|---|---|
| C-D1 | `gatta_and_luzzus_counter_order_is_settled_after_the_batch` → `…_is_asked_after_the_batch` | P0's Gatta and Luzzu on P0's Grizzly Bears, blocked by P1's **Eager Cadet** (was a 2/2); P0's Hardened Scales + Doubling Season | 615.5, 616.1; Hardened Scales ruling | `[6,7,8].contains`, no prompt | 1 prevented → one counter event; P0 asked once, offered {Scales, Season}: Scales first (1+1)×2 = **4**, Season first 1×2+1 = **3**; without Scales/Season **1**, no prompt. Bears 0 damage; Cadet dies; Bolt kills the Courser |
| C-D2 | `a_lifelink_answer_then_gatta_and_luzzus_rider` | P0's Child of Night (Gatta's target) blocked by P1's **Eager Cadet**; P0's Rhox Faithmender + Leyline of Hope; ± Scales/Season | 702.15b, 616.1, 615.5 | 1 prompt, counters in a set | Prompt 1 (P0) offers Rhox: answered Rhox → life +2×2+1 = **+5**. With Scales/Season, prompt 2 (P0) the counter order: Scales **4**, Season **3** (2 prompts in all); without, **1** counter and 1 prompt. Child 0 damage; Cadet dies |
| C-D3 | `channel_harms_may_is_settled_after_the_batch` → `…_is_asked_after_the_batch` | P0's Channel Harm targeting P1's Hill Giant; P0's Grizzly Bears attacks; blocked or not | 615.5, 608.2d; P3, P4 | "may" cleared, Giant 2 | Blocked: Giant's 3 prevented, P0 asked once (E4). **Accept** → Channel Harm deals **3** to the Giant (`DamageDealt { source_id: channel_harm, amount: 3, is_combat: false }`), with the Bears' 2 → Giant in the graveyard. **Decline** → Giant `damage_marked == 2`, on the battlefield. Unblocked: nothing prevented, no prompt, P1 at **18**. Bears 0 damage throughout |
| C-D4 | `new_way_forwards_draws_are_kept_under_abundance` → `new_way_forwards_draws_ask_abundance` | P1's New Way Forward choosing P0's unblocked Grizzly Bears; P1's Abundance or not | 615.5, 616.1, 121.2 | declined unasked | P0 life **−2** once; P1 life unchanged; P1 hand **+2** either way. With Abundance: **2** prompts, chooser **P1**; decline → 2 `CardDrawn` (P1); accept + nonland → **0** `CardDrawn`. Without: 2 drawn, no prompt. `strict-failure` line: the card's "When damage is prevented this way" is a trigger (CR 603.2, CR 603.3, CR 510.3a), run here as a rider inside the batch (§13) |
| C-D5 | `a_may_park_and_a_riders_own_prompt_asks_abundance_and_settles_channel_harm` → `a_may_park_and_a_riders_own_prompt_are_both_asked` | Two P0 Grizzly Bears; P1's Hill Giant blocks the first, Swans of Bryn Argoll the second; P0's Channel Harm at the Giant; P0's Abundance or not | 616.1, 608.2d, 615.5 | Abundance asked, Channel Harm cleared | Channel Harm prevents the Giant's 3 and the Swans' 4 (**7**); the Swans prevent the Bears' 2. Rows: (Abundance, decline, accept Harm): 2 Abundance prompts then 1 Channel Harm prompt (all P0); 2 `CardDrawn`; Giant dealt **7** noncombat → graveyard. (Abundance, accept, decline Harm): 0 `CardDrawn`, hand +2; Giant `damage_marked == 2`. (no Abundance, accept Harm): 1 prompt; Giant → graveyard. Every row: `prevented == 2`; both Bears alive with 0 damage; Swans 0 damage |

### 16.3 P1 rows that gain the V1 invariant only (no expectation change)

`nefarious_lichs_exile_choice_is_asked_inside_the_batch`, `dralnus_sacrifice_choice_is_asked_inside_the_batch`,
`outfitted_jousters_sacrifice_in_combat` ((2,1) rows), `swans_of_bryn_argolls_draws_ask_abundance`,
`double_strike_asks_abundance_for_the_swans_draw_in_each_damage_step`,
`phytohydras_counter_order_is_asked_inside_the_batch`, every Ironscale row using `fight`/
`fight_answering`, and every `combat_answering` row in `damage_substitution_replacements`. Their
derived values stand (P1). Under E1 the first three are red until Step B6.

Unchanged and must stay green: `a_prevention_rider_that_ends_the_game_ends_the_batchs_riders`,
`nothing_happens_in_the_batch_after_a_lifelink_gain_ends_the_game`,
`nothing_happens_in_the_batch_after_the_game_ends`, `nefarious_lich_ends_the_game_inside_the_batch`,
`a_player_who_loses_inside_the_batch_leaves_and_the_batch_goes_on`, the guard rows, main's #9670
suites (`szadek_combat_damage_to_counters`, `adamantoise_vigor_combat_order`,
`combat_lifelink_replacement_ordering`, `awe_strike_prevention`, `swans_prevention_followup`),
`inkshield_prevented_this_way_token_rider`, `draw_from_general_post_replacement`. The ignored red
witness `outfitted_jouster_double_blocked_sacrifices_one_equipment` stays ignored.

### 16.4 New rows

- **N-1** (unpreventable file, Phase D section) `two_aggregate_riders_are_asked_one_after_the_other`.
  `rider_board()`; P0's Grizzly Bears and Hill Giant attack P1 unblocked. P1 controls Rhox
  Faithmender and Leyline of Hope. P1 casts Awe Strike at the Bears, then a second Awe Strike at the
  Giant (`cast_by_p1` + `drive`). Phase D: the Bears' shield prevented 2, the Giant's 3; each rider
  is a life gain meeting both replacements, P1 orders each (CR 616.1). For each answer pair
  (first, second) ∈ {Rhox, Leyline}²: prompt 1 at life 20; prompt 2 at life **25** (Rhox first:
  2×2+1) or **26** (Leyline first: (2+1)×2); final life: (Rhox, Rhox) **32**, (Rhox, Leyline)
  **33**, (Leyline, Rhox) **33**, (Leyline, Leyline) **34**. Exactly 2 prompts, both P1's, each
  offering {Rhox, Leyline}. At prompt 2, before answering, assert `pending_combat_lifelink` holds
  `replacement.is_none()` and `prevention_tally.len() == 1`, then round-trip the whole `GameState`
  through `serde_json` and continue on the restored state. P1 dealt no combat damage
  (`damage_dealt_to_player(.., P1)` empty); `prevented == 2`; `assert_batch_settled`. Revert probes:
  at the base there are 0 prompts; leaving the shield in the tally until after its rider gives a
  third prompt and +5 more life.
- **N-2** (damage_substitution file, beside `a_resident_rider_from_another_recipient_is_not_lost`)
  `an_earlier_riders_pause_inside_a_combat_leg_is_asked`. P1's Phytohydra blocks P0's Rumbling
  Baloth (4); P0 cast Temper with X = 1 at the Phytohydra (`cast_temper_at_one`); P1 controls
  Hardened Scales and Doubling Season. Prompt 1 (P1): {Temper, Phytohydra} order (CR 616.1; S9) —
  answer Temper. Temper prevents 1; Phytohydra replaces the other 3. Prompt 2 (P1): Temper's one
  counter, {Scales, Season}; prompt 3 (P1): Phytohydra's 3 counters, {Scales, Season} (CR 615.5:
  the prevention's additional effect immediately afterward; CR 616.1f: then the next effect applies
  to what is left). For (a, b) ∈ {Scales, Season}²: counters at prompt 3 = **4** (a = Scales,
  (1+1)×2) or **3** (a = Season, 1×2+1); final = that + **8** (b = Scales, (3+1)×2) or **7**
  (b = Season, 3×2+1): totals 12 / 11 / 11 / 10. Phytohydra damage 0; no post-replacement drain left;
  the batch completed by the last answer. Siblings: Phytohydra first → 1 prompt then one counter
  prompt of 4 (Scales (4+1)×2 = 10 / Season 4×2+1 = 9), Temper unused; Doubling Season alone →
  1 prompt, counters 1×2 + 3×2 = **8**. Revert probe: at the base, prompt 2 is settled with
  candidate 0 (2 prompts in all).
- **N-3** (unpreventable file, guard section) `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park`.
  P0's Undergrowth Champion with no counter attacks; P1's two Grizzly Bears block it; P0 cast Temper
  with X = 1 at the Champion; P0 controls two Hardened Scales and Doubling Season. The Champion
  assigns 2 to the first Bear (CR 510.1c). The batch opens with the Champion's guard false, so its
  shield applies to none of the 4 damage (ruling). The first Bear's 2: Temper prevents 1 and its
  counter meets three effects: prompt 1 (P0) offers {Scales, Scales, Season}, prompt 2 offers the two
  left (CR 616.1f). Answer policies: Scales, Scales → **6**; Scales, Season → **5**
  ((1+1)×2+1). Variant A in memory; variant B round-trips `GameState` through `serde_json` at
  prompt 2. Both: exactly 2 prompts, none offering the Champion; Champion `damage_marked == 3` (1
  after Temper + the second Bear's 2), on the battlefield; `prevented == 1`. Revert probes: at the
  base (fresh read at the resume) the second Bear's damage is prevented by the Champion: damage 1,
  counters 5 / 4; with `settled_guards` not serialized, variant B gives the same wrong result.
- **N-4** (unpreventable file, Phase D section) `a_phase_d_rider_prompt_whose_player_leaves_lets_the_batch_finish`.
  `GameScenario::new_n_player(3, 42)` at P0's precombat main, eight library cards each. P1 controls
  Rhox Faithmender and Leyline of Hope and casts Awe Strike at P0's Grizzly Bears; the Bears attack
  P1 unblocked. At P1's Phase D prompt (assert `ReplacementChoice { player: P1 }` and the record's
  `replacement.is_none()`), P1 concedes. Then act on at most 16 waits (pass priority, order triggers)
  until `pending_combat_lifelink.is_none()`. Expected: drained, with `regular_damage_done` true;
  P1 eliminated, no further prompt to P1; P0 and P2 at 20. The departed P1 receives no gain.

## 17. Step-by-step plan

Commits on top of `00396db22d`; only the candidate must be green. Run `cargo fmt --all` after each
step; Tilt-first verification as at the end.

### Commit A — U2, guards across a park

- **A1** `crates/engine/src/game/replacement.rs`: `CombatDamageReplacementPass` gains
  `settled_guards` (doc: CR 608.2f + CR 700.1, Undergrowth Champion ruling); `new(state: &GameState,
  events)`; `replace_combat_damage_batch` installs `take(&mut pass.settled_guards)` and moves
  `applications` and `settled_guards` back on exit; rewrite its doc's last paragraph ("installed on
  every entry with the pass's one-shot applications and the guard reading the pass took when it was
  created"). Fix the test at the `record_proposed_events` site to `CombatDamageReplacementPass::new(&state, [..])`
  (clone first if the borrow checker requires it).
- **A2** `crates/engine/src/types/game_state.rs`: `PendingCombatDamageReplacement.settled_guards`.
- **A3** `crates/engine/src/game/combat_damage.rs`: `apply_combat_damage` → `new(state, ..)`;
  park and restore `settled_guards` in `run_combat_damage_replacements` /
  `resume_combat_damage_replacements`.
- **A4** `crates/engine/src/game/effects/prevent_damage.rs` test: `new(&state, [proposed])`.
- **A5** N-3 (needs `temper` / `cast_temper_at_one` `pub(super)`; uses `fight_driving` from Step B1 —
  so add `fight_driving` here, without the invariant, and the invariant in B1).
- **A6** Protocol record reason (§14) + `generate` + `check`.

### Commit B — U1, every combat rider pause asked

- **B1** Helpers (§16.1): the invariant in `fight_driving` and `combat_answering`; `choosers`. Run
  `dralnus_sacrifice_choice_is_asked_inside_the_batch` alone to establish E1 (expected red).
- **B2** `replacement.rs`: `dispatch_damage_rider_outside_batch` (doc: CR 700.1); `drain_damage_rider`
  without its combat branch, on top of it (doc: CR 614.6 + CR 615.5 + CR 608.2c, now naming the
  combat leg that parks on the pause); `drain_combat_damage_rider -> Option<WaitingFor>` (doc: CR 615.5
  + CR 510.2 + CR 700.1: returns the pause, settles nothing); `drain_combat_damage_continuation`
  calls it. Delete `CombatRiderPause`, `settle_combat_rider_pauses`, `answer_combat_rider_park`,
  `abandon_combat_rider_pause`. Update `DamageRiderDrain` variant docs.
- **B3** `replacement.rs`: D2 in `settle_combat_damage_replacement` + `take_combat_leg_paused_rider`
  (doc: CR 615.5 + CR 608.2c + CR 616.1g). `types/game_state.rs`: `DamageBatchLedger::paused_rider`
  doc ("In combat the leg it paused in parks on it (`settle_combat_damage_replacement`)").
- **B4** `combat_damage.rs`: D3 (`fire_combat_prevention_riders` signature, pop-first loop, latched
  `event_amount`, return the pause; `drain_combat_lifelink` parks). Docs: `fire_combat_prevention_riders`
  (CR 615.5 + CR 615.13 + CR 608.2d + CR 104.1), `drain_combat_lifelink`, `CombatDamageBatch::Paused`
  (fourth park point). `types/game_state.rs`: `PendingCombatLifelink` and its `prevention_tally` docs.
- **B5** Tests C-D1…C-D5, N-1, N-2, N-4; rewrite the converted rows' strict-failure comments (the
  #140 sentences go; C-D4 gains the trigger-timing line).
- **B6** (if E1 held) `combat_damage.rs`: `resume_parked_combat_damage_after_answer` (doc: CR 510.2
  + CR 510.3 + CR 510.3a + CR 615.5 + CR 616.1g); `engine.rs`: capture `action_event_start`, call it
  after `resume_parked_counter_cost_root_at_action_boundary` (comment: CR 510.2 + CR 615.5);
  narrow `resolve_combat_damage`'s re-entry comment to the departure paths (CR 800.4a); comment-only
  update in `engine_replacement::handle_replacement_choice`'s doc.

### Commit C — fence and documents

- **C1** `coverage.rs`: D6 deletions; grep each removed import. `resolution_prompt.rs`: restore the
  two functions' private visibility (optional, §19).
- **C2** Test-module docs: `unpreventable_damage_prevention_effects.rs` lines 19–27 become: "A
  rider's pause inside a combat damage batch is asked of the player it belongs to, at the
  per-portion drain and at the aggregate drain after the batch's damage (CR 615.5, CR 616.1,
  CR 608.2d), and the answer completes the batch in the same action: CR 510.2 rules out a priority
  window, not the choice. The last section holds those rows; the Ironscale Hydra rows are in
  `ironscale_hydra_combat_prevention`." `damage_substitution_replacements.rs` lines 15–18 become:
  "- Combat damage asks every CR 616.1 order inside its batch, and every prompt a substitute or a
  rider raises there, of the player it belongs to (CR 510.2 rules out a priority window, not the
  choice); the answer completes the batch." The Dralnu comment at the
  `damage_replacement_choice_unasked_in_combat` mention keeps its CR 614.6 + CR 615.5 / #172 order
  note and names `structural:damage_substitution_choice_incomplete` as what keeps Dralnu unsupported.
  The two "Rider pauses inside a combat batch (CR 510.2, CR 616.1; Command Zone #140)" section
  headers become "Rider pauses inside a combat batch, asked (CR 510.2, CR 616.1)".
  `combat_lifelink_replacement_ordering.rs`: the parameter-type doc line now reads
  `&mut Vec<(AppliedReplacementKey, i32)>`, popped one shield at a time.
- **C3** `python3 scripts/post_replacement_continuation_census.py --write`, then run it without
  `--write`. Expected delta: rows `replacement.rs drain_combat_damage_continuation consumer 1`,
  `replacement.rs drain_combat_damage_rider consumer 1`, `replacement.rs drain_damage_rider consumer 1`
  go; `replacement.rs dispatch_damage_rider_outside_batch consumer 1` comes. Review each row in the
  commit message.
- **C4** `grep -rn "damage_replacement_choice_unasked_in_combat\|DamageReplacementChoiceUnaskedInCombat\|settle_combat_rider_pauses\|CombatRiderPause\|answer_combat_rider_park\|abandon_combat_rider_pause\|Command Zone #140" crates/ scripts/` returns nothing.

### Verification (Tilt first; never both Tilt and direct cargo)

`tilt get uiresource clippy` to detect Tilt. With Tilt: `./scripts/tilt-wait.sh clippy test-engine`
(exit 3 = cannot answer, not a failure). Focused first: the three P2 test files, main's #9670 suites
named in §16.3, `combat_lifelink_replacement_ordering`, `inkshield_prevented_this_way_token_rider`,
`awe_strike_prevention`, the coverage unit tests, `game_state` serde tests. Then the census check and
the protocol `check`. Deferred to run-level verification (charter): `cargo coverage` flip list (V11),
workspace clippy, all engine shards, phase-ai `--lib`, wasm32, GameState size (unchanged), the
Kathril stack test.

## 18. Unprobed assertions (each settled at implementation by the named test)

- E1 (B1 run of the Dralnu row), E2 (N-2 prompt 2's offer), E3 (N-1 prompt-2 snapshot), E4 (C-D3).
- Elimination abandons a departed player's paused Phase D rider frame and pending replacement, and
  the priority-pass re-entry then finishes the record (N-4).
- The two identical Hardened Scales plus Doubling Season ask two prompts for one counter event when
  a Scales is chosen first (N-3; Ironscale C-21 in P1 asks the same board).
- Channel Harm's aggregate amount in C-D5 is 7 (3 + 4): `combat_prevention_tally` aggregates per
  shield across the batch (`batched_combat_all_shield`).
- Temper with X = 1 parses as one prevented point and one counter event (its "for each 1 damage"
  repeat runs once).

## 19. For the orchestrator / human

- **Addendum (scope).** D5 adds one call in `crates/engine/src/game/engine.rs`, outside P2's anchor
  paths; D6's optional cleanup touches `crates/engine/src/game/resolution_prompt.rs`. Neither breaks
  a charter decision: decision 3's park and resume authority stand, and D5 is the call site its
  "asked, never auto-settled" goal needs once E1 is established. If E1 is false, D5 is dropped.
- **Q-1 (scope, D5): accept that P2 also makes P1's per-leg own-prompt rows (Nefarious Lich, Dralnu,
  a two-Equipment Outfitted Jouster) finish in the answering action.** They open a priority window
  at the base (E1). Fixing it in P2 is the defective-reference route; the alternative is a follow-up
  and a fence kept for own-prompt riders, which the charter's goal rules out.
- **Q-2 (protocol): extend the branch's own unreleased record `ccf4c183…` rather than `bump game`.**
  The PR is unmerged and that record already covers the guard carrier on the same surfaces. If the
  maintainer wants one record per change, run `node scripts/protocol-versions.mjs bump game --reason
  "<§14 text>"` instead.
- **Q-3 (follow-up, not P2): New Way Forward's "When damage is prevented this way" runs as a rider,
  not a trigger** (CR 603.2, CR 603.3, CR 510.3a), the same gap as Outfitted Jouster's (P1's
  Q-J). C-D4 keeps a strict-failure line; filing it with Jouster's follow-up is recommended.
- No other human decision. Coverage flips (V11) are measured and listed at run level.
