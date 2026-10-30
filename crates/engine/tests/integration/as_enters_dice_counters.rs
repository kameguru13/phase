//! CR 614.1c + CR 603.6d + CR 706.4: "As this creature enters, roll <dice>. It
//! enters with a number of +1/+1 counters on it equal to the result / the total
//! of those results." (Neverwinter Hydra, Hydradoodle, Hammer Jammer, Jumbo Imp,
//! Temp of the Damned), and the entry-timing rule the class needs: an as-enters
//! continuation is part of its entrant's entry, so the entry is observed and
//! collected for triggers once, after the continuation completes, by whichever
//! collector owns it (CR 603.2 + CR 603.2c + CR 603.6a + CR 614.12a).
//!
//! Oracle text is verbatim (Scryfall / card-data). Barbarian Class and First Day
//! of Class carry only the line under test (the Class level-1 line; First Day of
//! Class's first sentence, without its Learn rider). Beck carries Beck's half.
//! Dice-dependent rows choose their RNG seed by search and assert against the
//! emitted `DieRolled` results, never a hard-coded total.

use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::types::ability::Effect;
use engine::types::actions::GameAction;
use engine::types::counter::CounterType;
use engine::types::events::GameEvent;
use engine::types::game_state::{GameState, StackEntryKind, WaitingFor};
use engine::types::identifiers::ObjectId;
use engine::types::keywords::Keyword;
use engine::types::mana::{ManaCost, ManaCostShard, ManaType, ManaUnit};
use engine::types::phase::Phase;
use engine::types::player::PlayerId;
use engine::types::zones::Zone;

const NEVERWINTER_HYDRA: &str = "As this creature enters, roll X d6. It enters with a number of +1/+1 counters on it equal to the total of those results.\nTrample\nWard {4} (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays {4}.)";
const HYDRADOODLE: &str = "As this creature enters, roll X six-sided dice. This creature enters with a number of +1/+1 counters on it equal to the total of those results.\nReach, trample";
const HAMMER_JAMMER: &str = "As this creature enters, roll a six-sided die. This creature enters with a number of +1/+1 counters on it equal to the result.\nWhenever you roll a die, remove all +1/+1 counters from this creature, then put a number of +1/+1 counters on it equal to the result.";
const JUMBO_IMP: &str = "Flying\nAs this creature enters, roll a six-sided die. This creature enters with a number of +1/+1 counters on it equal to the result.\nAt the beginning of your upkeep, roll a six-sided die and put a number of +1/+1 counters on this creature equal to the result.\nAt the beginning of your end step, roll a six-sided die and remove a number of +1/+1 counters from this creature equal to the result.";

const BARBARIAN_CLASS_L1: &str = "If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.";
const KIORA: &str = "Whenever a creature you control with power 4 or greater enters, draw a card.\n[−1]: Untap target permanent.";
const ELEMENTAL_BOND: &str =
    "Whenever a creature you control with power 3 or greater enters, draw a card.";
const SOUL_WARDEN: &str = "Whenever another creature enters, you gain 1 life.";
const BLOOD_ARTIST: &str = "Whenever this creature or another creature dies, target player loses 1 life and you gain 1 life.";
const THUNDER_THRASH_ELDER: &str = "Devour 3 (As this creature enters, you may sacrifice any number of creatures. It enters with three times that many +1/+1 counters on it.)";
const HARDENED_SCALES: &str = "If one or more +1/+1 counters would be put on a creature you control, that many plus one +1/+1 counters are put on it instead.";
const BRANCHING_EVOLUTION: &str = "If one or more +1/+1 counters would be put on a creature you control, twice that many +1/+1 counters are put on that creature instead.";
const ZOMBIFY: &str = "Return target creature card from your graveyard to the battlefield.";
const RISE_OF_THE_DARK_REALMS: &str =
    "Put all creature cards from all graveyards onto the battlefield under your control.";
const WELCOMING_VAMPIRE: &str = "Flying\nWhenever one or more other creatures you control with power 2 or less enter, draw a card. This ability triggers only once each turn.";
const THROUGH_THE_BREACH: &str = "You may put a creature card from your hand onto the battlefield. That creature gains haste. Sacrifice that creature at the beginning of the next end step.";
const BRAZEN_DWARF: &str =
    "Whenever you roll one or more dice, this creature deals 1 damage to each opponent.";
const FIRST_DAY_OF_CLASS: &str = "Whenever a creature you control enters this turn, put a +1/+1 counter on it and it gains haste until end of turn.";
const BECK: &str = "Whenever a creature enters this turn, you may draw a card.";
const ELESH_NORN: &str = "Vigilance\nOther creatures you control get +2/+2.\nCreatures your opponents control get -2/-2.";
const AJANIS_WELCOME: &str = "Whenever a creature you control enters, you gain 1 life.";
const GLUTTONOUS_SLIME: &str = "Flash\nDevour 1 (As this creature enters, you may sacrifice any number of creatures. It enters with that many +1/+1 counters on it.)";
const NIGHT_OF_SOULS_BETRAYAL: &str = "All creatures get -1/-1.";
const OPT: &str = "Scry 1. (Look at the top card of your library. You may put that card on the bottom.)\nDraw a card.";
const ANGELIC_SHIELD: &str = "Creatures you control get +0/+1.\nSacrifice this enchantment: Return target creature to its owner's hand.";
const BOX_OF_FREE_RANGE_GOBLINS: &str =
    "Roll a six-sided die. Create a number of 1/1 red Goblin creature tokens equal to the result.";

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn board(seed: u64) -> GameScenario {
    let mut scenario = GameScenario::new_n_player(2, seed);
    scenario.at_phase(Phase::PreCombatMain);
    let library: Vec<String> = (0..12).map(|i| format!("Library Card {i}")).collect();
    let names: Vec<&str> = library.iter().map(String::as_str).collect();
    scenario.with_library_top(P0, &names);
    scenario.with_library_top(P1, &names);
    scenario
}

fn green(count: usize) -> Vec<ManaUnit> {
    (0..count)
        .map(|_| ManaUnit::new(ManaType::Green, ObjectId(0), false, vec![]))
        .collect()
}

/// A `{X}{X}{G}{G}` creature in P0's hand, with mana for `x`.
fn x_creature(scenario: &mut GameScenario, name: &str, oracle: &str, x: usize) -> ObjectId {
    let id = scenario
        .add_creature_to_hand_from_oracle(P0, name, 0, 0, oracle)
        .with_mana_cost(ManaCost::Cost {
            generic: 0,
            shards: vec![
                ManaCostShard::X,
                ManaCostShard::X,
                ManaCostShard::Green,
                ManaCostShard::Green,
            ],
        })
        .id();
    scenario.with_mana_pool(P0, green(2 * x + 2));
    id
}

fn free_creature(
    scenario: &mut GameScenario,
    name: &str,
    power: i32,
    toughness: i32,
    oracle: &str,
) -> ObjectId {
    scenario
        .add_creature_to_hand_from_oracle(P0, name, power, toughness, oracle)
        .with_mana_cost(ManaCost::generic(0))
        .id()
}

/// A free creature whose keyword line needs MTGJSON's keyword hint (the colon
/// form `devour:N`, which `Keyword`'s `FromStr` splits on).
fn free_keyword_creature(
    scenario: &mut GameScenario,
    name: &str,
    (power, toughness): (i32, i32),
    keywords: &[&str],
    oracle: &str,
) -> ObjectId {
    let mut builder = scenario.add_creature_to_hand(P0, name, power, toughness);
    builder.from_oracle_text_with_keywords(keywords, oracle);
    builder.with_mana_cost(ManaCost::generic(0));
    builder.id()
}

fn thunder_thrash_elder(scenario: &mut GameScenario) -> ObjectId {
    free_keyword_creature(
        scenario,
        "Thunder-Thrash Elder",
        (1, 1),
        &["devour:3"],
        THUNDER_THRASH_ELDER,
    )
}

fn gluttonous_slime(scenario: &mut GameScenario) -> ObjectId {
    free_keyword_creature(
        scenario,
        "Gluttonous Slime",
        (2, 2),
        &["flash", "devour:1"],
        GLUTTONOUS_SLIME,
    )
}

fn free_spell(scenario: &mut GameScenario, name: &str, instant: bool, oracle: &str) -> ObjectId {
    scenario
        .add_spell_to_hand_from_oracle(P0, name, instant, oracle)
        .with_mana_cost(ManaCost::generic(0))
        .id()
}

/// A printed 0/0 creature card in P0's graveyard.
fn graveyard_creature(scenario: &mut GameScenario, name: &str, oracle: &str) -> ObjectId {
    scenario
        .add_creature_to_graveyard(P0, name, 0, 0)
        .from_oracle_text(oracle)
        .id()
}

fn act(runner: &mut GameRunner, action: GameAction, events: &mut Vec<GameEvent>) {
    let result = runner
        .act(action.clone())
        .unwrap_or_else(|err| panic!("{action:?} rejected: {err:?}"));
    events.extend(result.events);
    record_stack(runner.state());
}

/// Pass priority while the stack is non-empty and no player is being asked
/// anything else.
fn pass_until_prompt(runner: &mut GameRunner, events: &mut Vec<GameEvent>) {
    for _ in 0..60 {
        record_stack(runner.state());
        let state = runner.state();
        if matches!(state.waiting_for, WaitingFor::Priority { .. }) && !state.stack.is_empty() {
            act(runner, GameAction::PassPriority, events);
        } else {
            return;
        }
    }
    panic!("the stack never paused or emptied");
}

/// Cast `spell` (committing it to the stack) and pass priority until its
/// resolution pauses or the stack is empty.
fn cast_and_pass(
    runner: &mut GameRunner,
    spell: ObjectId,
    x: Option<u32>,
    target: Option<ObjectId>,
    events: &mut Vec<GameEvent>,
) {
    let mut cast = runner.cast(spell);
    if let Some(x) = x {
        cast = cast.x(x);
    }
    if let Some(target) = target {
        cast = cast.target_object(target);
    }
    let commit = cast.commit();
    events.extend(commit.events().iter().cloned());
    drop(commit);
    pass_until_prompt(runner, events);
}

/// The default answers this file uses once its own decisions are made.
#[derive(Clone, Copy)]
struct Answers {
    /// A triggered ability's player target.
    player_target: PlayerId,
    /// Accept "you may" offers.
    accept_optional: bool,
}

const DEFAULT_ANSWERS: Answers = Answers {
    player_target: P1,
    accept_optional: true,
};

/// What a run to an empty stack saw.
#[derive(Default)]
struct Settled {
    optional_offers: Vec<ObjectId>,
}

thread_local! {
    /// Every triggered-ability stack entry this test thread's drivers saw, as
    /// `(entry id, source id)`, so a trigger put on the stack and resolved
    /// inside any driver is still counted once.
    static TRIGGERS_SEEN: std::cell::RefCell<Vec<(ObjectId, ObjectId)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

fn record_stack(state: &GameState) {
    TRIGGERS_SEEN.with(|seen| {
        let mut seen = seen.borrow_mut();
        for entry in &state.stack {
            if matches!(entry.kind, StackEntryKind::TriggeredAbility { .. })
                && !seen.iter().any(|(id, _)| *id == entry.id)
            {
                seen.push((entry.id, entry.source_id));
            }
        }
    });
}

fn reset_triggers_seen() {
    TRIGGERS_SEEN.with(|seen| seen.borrow_mut().clear());
}

/// Triggered abilities from `source` the drivers have seen on the stack since
/// the last call, then forget them.
fn take_triggers_seen_from(source: ObjectId) -> usize {
    TRIGGERS_SEEN.with(|seen| {
        let seen = std::mem::take(&mut *seen.borrow_mut());
        seen.iter().filter(|(_, from)| *from == source).count()
    })
}

/// Drive to Priority on an empty stack, answering every prompt with the
/// default policy, and assert the settled postcondition.
fn settle(runner: &mut GameRunner, events: &mut Vec<GameEvent>, answers: Answers) -> Settled {
    let mut settled = Settled::default();
    for _ in 0..120 {
        record_stack(runner.state());
        let action = match &runner.state().waiting_for {
            WaitingFor::Priority { .. } if resolution_is_settled(runner.state()) => break,
            WaitingFor::Priority { .. } => GameAction::PassPriority,
            WaitingFor::OrderTriggers { .. } => {
                engine::game::triggers::drain_order_triggers_with_identity(runner.state_mut());
                continue;
            }
            WaitingFor::DieKeepChoice {
                ignorable_indices,
                ignore_count,
                ..
            } => GameAction::SelectDieRolls {
                ignore_indices: ignorable_indices
                    .iter()
                    .take(*ignore_count)
                    .copied()
                    .collect(),
            },
            WaitingFor::ScryChoice { cards, .. } => GameAction::SelectCards {
                cards: cards.clone(),
            },
            WaitingFor::OptionalEffectChoice { source_id, .. } => {
                settled.optional_offers.push(*source_id);
                GameAction::DecideOptionalEffect {
                    accept: answers.accept_optional,
                }
            }
            WaitingFor::TriggerTargetSelection { .. } => GameAction::ChooseTarget {
                target: Some(engine::types::ability::TargetRef::Player(
                    answers.player_target,
                )),
            },
            other => panic!("unhandled prompt while settling: {other:?}"),
        };
        act(runner, action, events);
    }
    let state = runner.state();
    assert!(
        matches!(state.waiting_for, WaitingFor::Priority { .. }) && resolution_is_settled(state),
        "settled: Priority on an empty stack, got {:?} with {} stack entries",
        state.waiting_for,
        state.stack.len()
    );
    settled
}

/// Nothing is on the stack and no resolution is still under way. A logical
/// zone-change owner whose paused member was answered on a counter-ordering
/// choice resumes at the next priority pass, with its spell still resolving.
fn resolution_is_settled(state: &GameState) -> bool {
    state.stack.is_empty()
        && state.resolution_stack.is_empty()
        && state.resolving_stack_entry.is_none()
}

fn die_results(events: &[GameEvent]) -> Vec<u32> {
    events
        .iter()
        .filter_map(|event| match event {
            GameEvent::DieRolled { result, .. } => result.map(u32::from),
            _ => None,
        })
        .collect()
}

fn ignored_rolls(events: &[GameEvent]) -> usize {
    events
        .iter()
        .filter(|event| matches!(event, GameEvent::DieRollIgnored { .. }))
        .count()
}

fn roll_resolved_for(events: &[GameEvent], source: ObjectId) -> usize {
    events
        .iter()
        .filter(|event| {
            matches!(
                event,
                GameEvent::EffectResolved {
                    kind: engine::types::ability::EffectKind::RollDie,
                    source_id,
                    ..
                } if *source_id == source
            )
        })
        .count()
}

fn plus_counters(state: &GameState, id: ObjectId) -> u32 {
    state.objects[&id]
        .counters
        .get(&CounterType::Plus1Plus1)
        .copied()
        .unwrap_or(0)
}

fn zone(state: &GameState, id: ObjectId) -> Zone {
    state.objects[&id].zone
}

fn hand(state: &GameState, player: PlayerId) -> usize {
    state.players[player.0 as usize].hand.len()
}

fn life(state: &GameState, player: PlayerId) -> i32 {
    state.players[player.0 as usize].life
}

fn has_unimplemented(state: &GameState, id: ObjectId) -> bool {
    state.objects[&id]
        .abilities
        .iter()
        .any(|a| matches!(*a.effect, Effect::Unimplemented { .. }))
}

fn parked_entry_of(state: &GameState, id: ObjectId) -> bool {
    state.deferred_entry_events.iter().any(|event| {
        matches!(
            event,
            GameEvent::ZoneChanged { object_id, to: Zone::Battlefield, .. } if *object_id == id
        )
    })
}

fn queued_or_stacked_from(state: &GameState, source: ObjectId) -> usize {
    let stacked = state
        .stack
        .iter()
        .filter(|entry| {
            matches!(entry.kind, StackEntryKind::TriggeredAbility { .. })
                && entry.source_id == source
        })
        .count();
    let queued = state
        .deferred_triggers
        .iter()
        .filter(|context| context.pending.source_id == source)
        .count();
    stacked + queued
}

fn tie_prompt_results(state: &GameState) -> Option<Vec<u8>> {
    match &state.waiting_for {
        WaitingFor::DieKeepChoice { results, .. } => Some(results.clone()),
        _ => None,
    }
}

/// The first seed in `0..` for which `accept` holds.
fn find_seed(what: &str, accept: impl Fn(u64) -> bool) -> u64 {
    (0..600)
        .find(|seed| accept(*seed))
        .unwrap_or_else(|| panic!("no seed found for {what}"))
}

/// The survivors' total of a Barbarian Class tie (one ignore of the lowest).
fn survivors_total(naturals: &[u8]) -> u32 {
    let sum: u32 = naturals.iter().map(|&n| u32::from(n)).sum();
    sum - u32::from(*naturals.iter().min().expect("dice"))
}

// ---------------------------------------------------------------------------
// U3: the roll's total is the counter count; X is the paid X; X = 0 rolls none
// ---------------------------------------------------------------------------

/// R4. CR 706.4 + CR 107.3m: the counters equal the sum of the emitted results,
/// one die per paid X.
#[test]
fn counters_equal_the_total_of_the_rolled_dice() {
    for (name, oracle) in [
        ("Neverwinter Hydra", NEVERWINTER_HYDRA),
        ("Hydradoodle", HYDRADOODLE),
    ] {
        let mut scenario = board(7);
        let hydra = x_creature(&mut scenario, name, oracle, 3);
        let mut runner = scenario.build();
        let mut events = Vec::new();
        cast_and_pass(&mut runner, hydra, Some(3), None, &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let rolls = die_results(&events);
        assert_eq!(rolls.len(), 3, "{name}: {rolls:?}");
        assert_eq!(zone(runner.state(), hydra), Zone::Battlefield, "{name}");
        assert_eq!(roll_resolved_for(&events, hydra), 1, "{name}");
        assert_eq!(
            plus_counters(runner.state(), hydra),
            rolls.iter().sum::<u32>(),
            "{name}"
        );
    }
}

/// R5. CR 706.1 + CR 706.4 + CR 704.5f: X = 0 rolls no dice and gives no
/// counters, so the 0/0 dies; an anthem keeps it alive with none. A non-cast
/// entry (X = 0 by CR 107.3m) whose producer stamps an effect count still
/// gives none, and a roll-count replacement does not roll a zero-die roll.
#[test]
fn x_zero_rolls_no_dice_and_gives_no_counters() {
    // Cast with X = 0.
    let mut scenario = board(7);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 0);
    let mut runner = scenario.build();
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(0), None, &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert!(!has_unimplemented(runner.state(), hydra));
    assert_eq!(
        roll_resolved_for(&events, hydra),
        1,
        "reach guard: the roll ran"
    );
    assert!(die_results(&events).is_empty());
    assert_eq!(zone(runner.state(), hydra), Zone::Graveyard);

    // The anthem variant: alive with no counters.
    let mut scenario = board(7);
    scenario.add_enchantment_from_oracle(P0, "Angelic Shield", ANGELIC_SHIELD);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 0);
    let mut runner = scenario.build();
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(0), None, &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert_eq!(roll_resolved_for(&events, hydra), 1);
    assert_eq!(zone(runner.state(), hydra), Zone::Battlefield);
    assert_eq!(plus_counters(runner.state(), hydra), 0);

    // Multi-authority: reanimated (no X paid) by a ChangeZone producer.
    let mut scenario = board(7);
    scenario.add_enchantment_from_oracle(P0, "Angelic Shield", ANGELIC_SHIELD);
    let hydra = graveyard_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA);
    let zombify = free_spell(&mut scenario, "Zombify", false, ZOMBIFY);
    let mut runner = scenario.build();
    let mut events = Vec::new();
    cast_and_pass(&mut runner, zombify, None, Some(hydra), &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert_eq!(
        roll_resolved_for(&events, hydra),
        1,
        "reach guard: the roll ran"
    );
    assert_eq!(zone(runner.state(), hydra), Zone::Battlefield);
    assert_eq!(plus_counters(runner.state(), hydra), 0);

    // Barbarian Class does not turn a zero-die roll into a one-die roll.
    let mut scenario = board(7);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 0);
    let mut runner = scenario.build();
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(0), None, &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert_eq!(roll_resolved_for(&events, hydra), 1);
    assert!(die_results(&events).is_empty());
    assert_eq!(ignored_rolls(&events), 0);
}

fn barbarian_hydra(seed: u64, x: usize) -> (GameRunner, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, x);
    (scenario.build(), hydra)
}

fn pauses_on_tie(runner: &mut GameRunner, hydra: ObjectId, x: u32) -> Option<Vec<u8>> {
    let mut events = Vec::new();
    cast_and_pass(runner, hydra, Some(x), None, &mut events);
    tie_prompt_results(runner.state())
}

/// R6. CR 706.6: a roll modifier sees the as-enters roll; the counters are the
/// survivors' total.
#[test]
fn roll_modifiers_apply_to_the_as_enters_roll() {
    let seed = find_seed("no tie", |seed| {
        let (mut runner, hydra) = barbarian_hydra(seed, 2);
        pauses_on_tie(&mut runner, hydra, 2).is_none()
    });
    let (mut runner, hydra) = barbarian_hydra(seed, 2);
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(2), None, &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    let rolls = die_results(&events);
    assert_eq!(rolls.len(), 2, "{rolls:?}");
    assert_eq!(ignored_rolls(&events), 1);
    assert_eq!(roll_resolved_for(&events, hydra), 1);
    assert_eq!(
        plus_counters(runner.state(), hydra),
        rolls.iter().sum::<u32>()
    );

    // The tie path: the roller's keep choice is reached and answered.
    let seed = find_seed("a tie", |seed| {
        let (mut runner, hydra) = barbarian_hydra(seed, 2);
        pauses_on_tie(&mut runner, hydra, 2).is_some()
    });
    let (mut runner, hydra) = barbarian_hydra(seed, 2);
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(2), None, &mut events);
    let naturals = tie_prompt_results(runner.state()).expect("DieKeepChoice reached");
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    let rolls = die_results(&events);
    assert_eq!(rolls.iter().sum::<u32>(), survivors_total(&naturals));
    assert_eq!(
        plus_counters(runner.state(), hydra),
        survivors_total(&naturals)
    );
}

/// R7. CR 706.1: one instruction is one roll event for "whenever you roll one
/// or more dice"; a zero-die instruction rolls nothing.
#[test]
fn a_battlefield_observer_sees_the_as_enters_roll_once() {
    for (x, expected_loss) in [(2, 1), (0, 0)] {
        let mut scenario = board(7);
        scenario.add_creature_from_oracle(P0, "Brazen Dwarf", 1, 3, BRAZEN_DWARF);
        let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, x);
        let mut runner = scenario.build();
        let before = life(runner.state(), P1);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, hydra, Some(x as u32), None, &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(roll_resolved_for(&events, hydra), 1, "x = {x}");
        assert!(!has_unimplemented(runner.state(), hydra));
        assert_eq!(before - life(runner.state(), P1), expected_loss, "x = {x}");
    }
}

/// R10. CR 107.3m: the paid X binds to the entrant, not to another X permanent.
#[test]
fn the_paid_x_binds_to_the_entrant() {
    let mut scenario = board(7);
    let doodle = x_creature(&mut scenario, "Hydradoodle", HYDRADOODLE, 5);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 2);
    let mut runner = scenario.build();
    let mut events = Vec::new();
    cast_and_pass(&mut runner, doodle, Some(5), None, &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert_eq!(die_results(&events).len(), 5);
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(2), None, &mut events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    let rolls = die_results(&events);
    assert_eq!(rolls.len(), 2, "{rolls:?}");
    assert_eq!(
        plus_counters(runner.state(), hydra),
        rolls.iter().sum::<u32>()
    );
}

// ---------------------------------------------------------------------------
// U4a: an inline as-enters continuation is observed with its counters
// ---------------------------------------------------------------------------

fn kiora(scenario: &mut GameScenario) -> ObjectId {
    scenario
        .add_planeswalker_from_oracle(P0, "Kiora, Behemoth Beckoner", "Kiora", 7, KIORA)
        .id()
}

fn kiora_hydra(seed: u64) -> (GameRunner, ObjectId) {
    let mut scenario = board(seed);
    kiora(&mut scenario);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 1);
    (scenario.build(), hydra)
}

fn first_roll(seed: u64, build: impl Fn(u64) -> (GameRunner, ObjectId), x: Option<u32>) -> u32 {
    let (mut runner, id) = build(seed);
    let mut events = Vec::new();
    cast_and_pass(&mut runner, id, x, None, &mut events);
    die_results(&events).first().copied().unwrap_or(0)
}

/// R8a. CR 603.6d + CR 614.1c + CR 603.6a: the counters the creature enters
/// with are part of the entry, so Kiora sees a 4+ power entrant (its ruling:
/// "…as well as any +1/+1 counters the creature enters with").
#[test]
fn an_inline_as_enters_roll_is_observed_with_its_counters() {
    for (wants_draw, what) in [(true, "result >= 4"), (false, "result <= 3")] {
        let seed = find_seed(what, |seed| {
            let r = first_roll(seed, kiora_hydra, Some(1));
            (r >= 4) == wants_draw
        });
        let (mut runner, hydra) = kiora_hydra(seed);
        let hand_before = hand(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, hydra, Some(1), None, &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let rolls = die_results(&events);
        assert_eq!(
            plus_counters(runner.state(), hydra),
            rolls.iter().sum::<u32>()
        );
        let draws = hand(runner.state(), P0) + 1 - hand_before;
        assert_eq!(draws, usize::from(wants_draw), "{what}: {rolls:?}");
    }
}

fn bond_imp(seed: u64) -> (GameRunner, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND);
    let imp = free_creature(&mut scenario, "Jumbo Imp", 0, 0, JUMBO_IMP);
    (scenario.build(), imp)
}

/// R8b. The sibling class card and observer.
#[test]
fn jumbo_imp_is_observed_with_its_counters() {
    for (wants_draw, what) in [(true, "result >= 3"), (false, "result <= 2")] {
        let seed = find_seed(what, |seed| {
            (first_roll(seed, bond_imp, None) >= 3) == wants_draw
        });
        let (mut runner, imp) = bond_imp(seed);
        let hand_before = hand(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, imp, None, None, &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let rolls = die_results(&events);
        assert_eq!(rolls.len(), 1);
        assert_eq!(plus_counters(runner.state(), imp), rolls[0]);
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(wants_draw)
        );
    }
}

// ---------------------------------------------------------------------------
// U4b: a paused continuation's entry is held, then released once
// ---------------------------------------------------------------------------

fn kiora_barbarian_hydra(seed: u64) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    let kiora = kiora(&mut scenario);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 2);
    (scenario.build(), hydra, kiora)
}

/// R8c. CR 614.12a + CR 603.6d: a paused roll defers the entry until it
/// completes; it is collected in the answering action, against the realized
/// entrant.
#[test]
fn a_paused_roll_defers_the_entry_until_it_completes() {
    let seed = find_seed("a tie whose survivors total >= 4", |seed| {
        let (mut runner, hydra, _) = kiora_barbarian_hydra(seed);
        pauses_on_tie(&mut runner, hydra, 2).is_some_and(|n| survivors_total(&n) >= 4)
    });
    let (mut runner, hydra, kiora) = kiora_barbarian_hydra(seed);
    let hand_before = hand(runner.state(), P0);
    let mut events = Vec::new();
    cast_and_pass(&mut runner, hydra, Some(2), None, &mut events);
    let state = runner.state();
    let naturals = tie_prompt_results(state).expect("DieKeepChoice reached");
    assert!(
        parked_entry_of(state, hydra),
        "the entry is parked at the pause"
    );
    assert_eq!(queued_or_stacked_from(state, kiora), 0, "the hold");
    let WaitingFor::DieKeepChoice {
        ignorable_indices,
        ignore_count,
        ..
    } = state.waiting_for.clone()
    else {
        unreachable!()
    };
    let mut answer_events = Vec::new();
    act(
        &mut runner,
        GameAction::SelectDieRolls {
            ignore_indices: ignorable_indices.into_iter().take(ignore_count).collect(),
        },
        &mut answer_events,
    );
    assert_eq!(
        queued_or_stacked_from(runner.state(), kiora),
        1,
        "collected in the SelectDieRolls action"
    );
    assert!(!parked_entry_of(runner.state(), hydra));
    events.extend(answer_events);
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert_eq!(
        plus_counters(runner.state(), hydra),
        survivors_total(&naturals)
    );
    assert_eq!(hand(runner.state(), P0) + 1 - hand_before, 1);

    // Non-entry hostile: a plain die prompt parks nothing.
    let seed = find_seed("a Box of Free-Range Goblins tie", |seed| {
        let mut scenario = board(seed);
        scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
        let spell = free_spell(
            &mut scenario,
            "Box of Free-Range Goblins",
            false,
            BOX_OF_FREE_RANGE_GOBLINS,
        );
        let mut runner = scenario.build();
        let mut events = Vec::new();
        cast_and_pass(&mut runner, spell, None, None, &mut events);
        tie_prompt_results(runner.state()).is_some()
    });
    let mut scenario = board(seed);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let spell = free_spell(
        &mut scenario,
        "Box of Free-Range Goblins",
        false,
        BOX_OF_FREE_RANGE_GOBLINS,
    );
    let mut runner = scenario.build();
    let mut events = Vec::new();
    cast_and_pass(&mut runner, spell, None, None, &mut events);
    assert!(tie_prompt_results(runner.state()).is_some());
    assert!(runner.state().deferred_entry_events.is_empty());
}

fn scales_hydra(seed: u64) -> (GameRunner, ObjectId) {
    let mut scenario = board(seed);
    kiora(&mut scenario);
    scenario.add_enchantment_from_oracle(P0, "Hardened Scales", HARDENED_SCALES);
    scenario.add_enchantment_from_oracle(P0, "Branching Evolution", BRANCHING_EVOLUTION);
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 1);
    (scenario.build(), hydra)
}

fn replacement_index(state: &GameState, source_name: &str) -> usize {
    match &state.waiting_for {
        WaitingFor::ReplacementChoice { candidates, .. } => candidates
            .iter()
            .position(|c| c.source_name == source_name)
            .unwrap_or_else(|| panic!("{source_name} among {candidates:?}")),
        other => panic!("expected ReplacementChoice, got {other:?}"),
    }
}

/// R8d. A pause on the counters (not the roll) also defers the entry.
#[test]
fn a_pause_on_the_counters_also_defers_the_entry() {
    for (first, wants_seed, expected) in [
        ("Hardened Scales", None, None),
        ("Branching Evolution", Some(1u32), Some(3u32)),
    ] {
        let seed = match wants_seed {
            None => 7,
            Some(r) => find_seed("r = 1", |seed| first_roll(seed, scales_hydra, Some(1)) == r),
        };
        let (mut runner, hydra) = scales_hydra(seed);
        let hand_before = hand(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, hydra, Some(1), None, &mut events);
        assert!(
            matches!(
                runner.state().waiting_for,
                WaitingFor::ReplacementChoice { .. }
            ),
            "{:?}",
            runner.state().waiting_for
        );
        assert!(parked_entry_of(runner.state(), hydra));
        let r = die_results(&events)[0];
        let index = replacement_index(runner.state(), first);
        act(
            &mut runner,
            GameAction::ChooseReplacement { index },
            &mut events,
        );
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let counters = plus_counters(runner.state(), hydra);
        let expected = expected.unwrap_or((r + 1) * 2);
        assert_eq!(counters, expected, "{first} first, r = {r}");
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(counters >= 4),
            "{first} first"
        );
    }
}

/// R9. CR 603.10 + CR 603.6a + CR 608.2k: Hammer Jammer exists immediately
/// after the entry that contains its roll, so its "whenever you roll a die"
/// triggers once; its bare "it" is Hammer Jammer, so the counters end equal to
/// the as-enters result.
#[test]
fn hammer_jammer_sees_its_own_as_enters_roll() {
    let mut scenario = board(7);
    let jammer = free_creature(&mut scenario, "Hammer Jammer", 0, 0, HAMMER_JAMMER);
    let mut runner = scenario.build();
    let mut events = Vec::new();
    {
        let commit = runner.cast(jammer).commit();
        events.extend(commit.events().iter().cloned());
    }
    // Pass until the creature spell itself has resolved.
    let mut guard = 0;
    while zone(runner.state(), jammer) == Zone::Stack {
        act(&mut runner, GameAction::PassPriority, &mut events);
        guard += 1;
        assert!(guard < 10);
    }
    let rolls = die_results(&events);
    assert_eq!(rolls.len(), 1, "{rolls:?}");
    let from_jammer = runner
        .state()
        .stack
        .iter()
        .filter(|entry| entry.source_id == jammer)
        .count();
    assert_eq!(from_jammer, 1, "its die-roll trigger, once");
    settle(&mut runner, &mut events, DEFAULT_ANSWERS);
    assert_eq!(zone(runner.state(), jammer), Zone::Battlefield);
    assert_eq!(plus_counters(runner.state(), jammer), rolls[0]);
}

// ---------------------------------------------------------------------------
// Devour (an existing as-enters continuation): its counters are part of the entry
// ---------------------------------------------------------------------------

fn effect_zone_choice(state: &GameState) -> bool {
    matches!(state.waiting_for, WaitingFor::EffectZoneChoice { .. })
}

/// R12. CR 702.82a + CR 614.1c + CR 603.6d: Kiora sees the devoured counters.
#[test]
fn devour_counters_are_part_of_the_entry() {
    for devour in [true, false] {
        let mut scenario = board(7);
        let kiora = kiora(&mut scenario);
        let victim = scenario.add_vanilla(P0, 2, 2);
        let elder = thunder_thrash_elder(&mut scenario);
        let mut runner = scenario.build();
        let hand_before = hand(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, elder, None, None, &mut events);
        assert!(
            effect_zone_choice(runner.state()),
            "{:?}",
            runner.state().waiting_for
        );
        let cards = if devour { vec![victim] } else { vec![] };
        act(&mut runner, GameAction::SelectCards { cards }, &mut events);
        assert_eq!(
            plus_counters(runner.state(), elder),
            if devour { 3 } else { 0 }
        );
        assert_eq!(
            queued_or_stacked_from(runner.state(), kiora)
                + usize::from(matches!(
                    runner.state().waiting_for,
                    WaitingFor::OrderTriggers { .. }
                )),
            usize::from(devour),
            "Kiora's trigger is placed by the answering action"
        );
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(devour)
        );
    }
}

/// R14. CR 603.3b: the devoured creature's dies trigger and the Devourer's
/// ETB observer share the sacrifice handler's drain — one ordering window.
#[test]
fn devour_dies_and_enters_triggers_share_one_drain() {
    for devour in [true, false] {
        let mut scenario = board(7);
        let kiora = kiora(&mut scenario);
        let artist = scenario
            .add_creature_from_oracle(P0, "Blood Artist", 0, 1, BLOOD_ARTIST)
            .id();
        let victim = scenario.add_vanilla(P0, 2, 2);
        let elder = thunder_thrash_elder(&mut scenario);
        let mut runner = scenario.build();
        let hand_before = hand(runner.state(), P0);
        let (p0_before, p1_before) = (life(runner.state(), P0), life(runner.state(), P1));
        let mut events = Vec::new();
        cast_and_pass(&mut runner, elder, None, None, &mut events);
        assert!(effect_zone_choice(runner.state()));
        let cards = if devour { vec![victim] } else { vec![] };
        act(&mut runner, GameAction::SelectCards { cards }, &mut events);
        if devour {
            assert_eq!(plus_counters(runner.state(), elder), 3);
            match &runner.state().waiting_for {
                WaitingFor::OrderTriggers { triggers, .. } => {
                    let sources: Vec<ObjectId> = triggers.iter().map(|t| t.source_id).collect();
                    assert!(sources.contains(&artist), "{sources:?}");
                    assert!(sources.contains(&kiora), "{sources:?}");
                }
                other => panic!("one ordering window for both triggers, got {other:?}"),
            }
        } else {
            assert!(!matches!(
                runner.state().waiting_for,
                WaitingFor::OrderTriggers { .. }
            ));
        }
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(devour)
        );
        assert_eq!(life(runner.state(), P1), p1_before - i32::from(devour));
        assert_eq!(life(runner.state(), P0), p0_before + i32::from(devour));
    }
}

// ---------------------------------------------------------------------------
// U4c: under a logical zone-change owner, the paused member is collected once
// ---------------------------------------------------------------------------

fn warden_bond_barbarian_imp(seed: u64) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_creature_from_oracle(P0, "Soul Warden", 1, 1, SOUL_WARDEN);
    scenario.add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let imp = graveyard_creature(&mut scenario, "Jumbo Imp", JUMBO_IMP);
    let zombify = free_spell(&mut scenario, "Zombify", false, ZOMBIFY);
    (scenario.build(), imp, zombify)
}

fn zombify_tie(seed: u64) -> Option<Vec<u8>> {
    let (mut runner, imp, zombify) = warden_bond_barbarian_imp(seed);
    let mut events = Vec::new();
    cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
    tie_prompt_results(runner.state())
}

/// R15. CR 603.2c + CR 603.6d: a reanimated paused entry is collected once,
/// at the owner's resume, with its counters.
#[test]
fn a_reanimated_paused_roll_is_collected_once_with_its_counters() {
    for wants_draw in [true, false] {
        let seed = find_seed("an Imp tie", |seed| {
            zombify_tie(seed).is_some_and(|n| (survivors_total(&n) >= 3) == wants_draw)
        });
        let (mut runner, imp, zombify) = warden_bond_barbarian_imp(seed);
        let hand_before = hand(runner.state(), P0);
        let life_before = life(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
        let naturals = tie_prompt_results(runner.state()).expect("DieKeepChoice reached");
        let WaitingFor::DieKeepChoice {
            ignorable_indices,
            ignore_count,
            ..
        } = runner.state().waiting_for.clone()
        else {
            unreachable!()
        };
        act(
            &mut runner,
            GameAction::SelectDieRolls {
                ignore_indices: ignorable_indices.into_iter().take(ignore_count).collect(),
            },
            &mut events,
        );
        assert!(
            runner.state().active_change_zone_frame().is_none(),
            "the owner completed in the answering action"
        );
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(
            plus_counters(runner.state(), imp),
            survivors_total(&naturals)
        );
        assert_eq!(
            life(runner.state(), P0),
            life_before + 1,
            "Soul Warden once"
        );
        // Zombify left the hand.
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(wants_draw),
            "Elemental Bond against the realized Imp"
        );
    }
}

fn warden_kiora_scales_imp(seed: u64) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_creature_from_oracle(P0, "Soul Warden", 1, 1, SOUL_WARDEN);
    kiora(&mut scenario);
    scenario.add_enchantment_from_oracle(P0, "Hardened Scales", HARDENED_SCALES);
    scenario.add_enchantment_from_oracle(P0, "Branching Evolution", BRANCHING_EVOLUTION);
    let imp = graveyard_creature(&mut scenario, "Jumbo Imp", JUMBO_IMP);
    let zombify = free_spell(&mut scenario, "Zombify", false, ZOMBIFY);
    (scenario.build(), imp, zombify)
}

/// R16. The same under a counter-ordering pause.
#[test]
fn a_reanimated_counter_order_pause_is_collected_once() {
    for (first, wants_r1) in [("Hardened Scales", false), ("Branching Evolution", true)] {
        let seed = if wants_r1 {
            find_seed("r = 1", |seed| {
                let (mut runner, imp, zombify) = warden_kiora_scales_imp(seed);
                let mut events = Vec::new();
                cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
                die_results(&events).first() == Some(&1)
            })
        } else {
            7
        };
        let (mut runner, imp, zombify) = warden_kiora_scales_imp(seed);
        let hand_before = hand(runner.state(), P0);
        let life_before = life(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
        assert!(matches!(
            runner.state().waiting_for,
            WaitingFor::ReplacementChoice { .. }
        ));
        assert!(parked_entry_of(runner.state(), imp));
        let r = die_results(&events)[0];
        let index = replacement_index(runner.state(), first);
        act(
            &mut runner,
            GameAction::ChooseReplacement { index },
            &mut events,
        );
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let counters = plus_counters(runner.state(), imp);
        let expected = if wants_r1 { 3 } else { (r + 1) * 2 };
        assert_eq!(counters, expected, "{first} first");
        assert_eq!(
            life(runner.state(), P0),
            life_before + 1,
            "Soul Warden once"
        );
        // Zombify left the hand.
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(counters >= 4),
            "{first} first"
        );
    }
}

/// R17. Devour under a targeted owner.
#[test]
fn a_reanimated_devour_is_collected_once_with_its_counters() {
    for devour in [true, false] {
        let mut scenario = board(7);
        scenario.add_creature_from_oracle(P0, "Soul Warden", 1, 1, SOUL_WARDEN);
        kiora(&mut scenario);
        let victim = scenario.add_vanilla(P0, 2, 2);
        let elder = {
            let mut builder = scenario.add_creature_to_graveyard(P0, "Thunder-Thrash Elder", 1, 1);
            builder.from_oracle_text_with_keywords(&["devour:3"], THUNDER_THRASH_ELDER);
            builder.id()
        };
        let zombify = free_spell(&mut scenario, "Zombify", false, ZOMBIFY);
        let mut runner = scenario.build();
        let hand_before = hand(runner.state(), P0);
        let life_before = life(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, zombify, None, Some(elder), &mut events);
        assert!(
            effect_zone_choice(runner.state()),
            "{:?}",
            runner.state().waiting_for
        );
        let cards = if devour { vec![victim] } else { vec![] };
        act(&mut runner, GameAction::SelectCards { cards }, &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(
            plus_counters(runner.state(), elder),
            if devour { 3 } else { 0 }
        );
        assert_eq!(
            life(runner.state(), P0),
            life_before + 1,
            "Soul Warden once"
        );
        // Zombify left the hand.
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(devour)
        );
    }
}

fn mass_board(seed: u64) -> (GameRunner, ObjectId, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_creature_from_oracle(P0, "Soul Warden", 1, 1, SOUL_WARDEN);
    scenario.add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND);
    scenario.add_creature_from_oracle(P0, "Welcoming Vampire", 2, 3, WELCOMING_VAMPIRE);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let imp = graveyard_creature(&mut scenario, "Jumbo Imp", JUMBO_IMP);
    let bear = scenario
        .add_creature_to_graveyard(P0, "Grizzly Bears", 2, 2)
        .id();
    let rise = free_spell(
        &mut scenario,
        "Rise of the Dark Realms",
        false,
        RISE_OF_THE_DARK_REALMS,
    );
    (scenario.build(), imp, bear, rise)
}

/// R18. CR 603.2c + CR 603.6a: a paused member of a mass entry is collected
/// with its co-entrant once; a batched observer fires once.
#[test]
fn a_paused_member_of_a_mass_entry_is_collected_with_its_co_entrant() {
    for imp_qualifies_for_bond in [true, false] {
        let seed = find_seed("a mass Imp tie", |seed| {
            let (mut runner, _, _, rise) = mass_board(seed);
            let mut events = Vec::new();
            cast_and_pass(&mut runner, rise, None, None, &mut events);
            tie_prompt_results(runner.state())
                .is_some_and(|n| (survivors_total(&n) >= 3) == imp_qualifies_for_bond)
        });
        let (mut runner, imp, bear, rise) = mass_board(seed);
        let hand_before = hand(runner.state(), P0);
        let life_before = life(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, rise, None, None, &mut events);
        let naturals = tie_prompt_results(runner.state()).expect("DieKeepChoice reached");
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(zone(runner.state(), imp), Zone::Battlefield);
        assert_eq!(zone(runner.state(), bear), Zone::Battlefield);
        assert_eq!(
            plus_counters(runner.state(), imp),
            survivors_total(&naturals)
        );
        assert_eq!(life(runner.state(), P0), life_before + 2, "two entrants");
        // Rise of the Dark Realms left the hand.
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            1 + usize::from(imp_qualifies_for_bond),
            "one Welcoming Vampire draw, plus Elemental Bond for a 3+ power Imp"
        );
    }
}

fn breach_board(seed: u64) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_creature_from_oracle(P0, "Soul Warden", 1, 1, SOUL_WARDEN);
    scenario.add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let imp = scenario
        .add_creature_to_hand_from_oracle(P0, "Jumbo Imp", 0, 0, JUMBO_IMP)
        .id();
    let breach = free_spell(
        &mut scenario,
        "Through the Breach",
        true,
        THROUGH_THE_BREACH,
    );
    (scenario.build(), imp, breach)
}

/// Cast Through the Breach and accept the put, stopping at the next prompt.
fn breach_until_roll(runner: &mut GameRunner, breach: ObjectId, imp: ObjectId) -> Vec<GameEvent> {
    let mut events = Vec::new();
    cast_and_pass(runner, breach, None, None, &mut events);
    for _ in 0..8 {
        let action = match &runner.state().waiting_for {
            WaitingFor::OptionalEffectChoice { .. } => {
                GameAction::DecideOptionalEffect { accept: true }
            }
            WaitingFor::EffectZoneChoice { .. } | WaitingFor::ChooseFromZoneChoice { .. } => {
                GameAction::SelectCards { cards: vec![imp] }
            }
            _ => break,
        };
        act(runner, action, &mut events);
    }
    events
}

/// R19. An un-owned non-cast entry (single-pick put) is held and released once.
#[test]
fn an_unowned_put_onto_the_battlefield_is_collected_once() {
    for wants_draw in [true, false] {
        let seed = find_seed("a Breach Imp tie", |seed| {
            let (mut runner, imp, breach) = breach_board(seed);
            breach_until_roll(&mut runner, breach, imp);
            tie_prompt_results(runner.state())
                .is_some_and(|n| (survivors_total(&n) >= 3) == wants_draw)
        });
        let (mut runner, imp, breach) = breach_board(seed);
        let hand_before = hand(runner.state(), P0);
        let life_before = life(runner.state(), P0);
        let mut events = breach_until_roll(&mut runner, breach, imp);
        let naturals = tie_prompt_results(runner.state()).expect("DieKeepChoice reached");
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(zone(runner.state(), imp), Zone::Battlefield);
        assert!(runner.state().objects[&imp].has_keyword(&Keyword::Haste));
        assert_eq!(
            plus_counters(runner.state(), imp),
            survivors_total(&naturals)
        );
        assert_eq!(
            life(runner.state(), P0),
            life_before + 1,
            "Soul Warden once"
        );
        // Jumbo Imp and Through the Breach left the hand.
        assert_eq!(
            hand(runner.state(), P0) + 2 - hand_before,
            usize::from(wants_draw)
        );
    }
}

fn bond_zombify_imp(seed: u64) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND);
    let imp = graveyard_creature(&mut scenario, "Jumbo Imp", JUMBO_IMP);
    let zombify = free_spell(&mut scenario, "Zombify", false, ZOMBIFY);
    (scenario.build(), imp, zombify)
}

/// R20. An inline entry under an owner (no pause).
#[test]
fn an_inline_entry_under_an_owner_is_observed_with_its_counters() {
    for wants_draw in [true, false] {
        let seed = find_seed("an owned Imp roll", |seed| {
            let (mut runner, imp, zombify) = bond_zombify_imp(seed);
            let mut events = Vec::new();
            cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
            die_results(&events)
                .first()
                .is_some_and(|&r| (r >= 3) == wants_draw)
        });
        let (mut runner, imp, zombify) = bond_zombify_imp(seed);
        let hand_before = hand(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let rolls = die_results(&events);
        assert_eq!(plus_counters(runner.state(), imp), rolls[0]);
        // Zombify left the hand.
        assert_eq!(
            hand(runner.state(), P0) + 1 - hand_before,
            usize::from(wants_draw)
        );
    }
}

// ---------------------------------------------------------------------------
// Delayed "enters this turn" triggers see a paused entry once
// ---------------------------------------------------------------------------

/// Cast First Day of Class's first sentence and resolve it; returns its id.
fn resolve_first_day_of_class(runner: &mut GameRunner, fdoc: ObjectId) {
    let mut events = Vec::new();
    cast_and_pass(runner, fdoc, None, None, &mut events);
    settle(runner, &mut events, DEFAULT_ANSWERS);
    assert!(
        !runner.state().delayed_triggers.is_empty(),
        "reach guard: First Day of Class's delayed trigger is installed"
    );
}

fn fdoc_hydra(seed: u64) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
    let fdoc = free_spell(
        &mut scenario,
        "First Day of Class",
        false,
        FIRST_DAY_OF_CLASS,
    );
    let hydra = x_creature(&mut scenario, "Neverwinter Hydra", NEVERWINTER_HYDRA, 2);
    let mut runner = scenario.build();
    resolve_first_day_of_class(&mut runner, fdoc);
    (runner, hydra, fdoc)
}

/// R21. CR 603.2 + CR 603.7: a delayed "whenever a creature you control enters
/// this turn" sees a paused cast entry once, after its continuation completes.
#[test]
fn a_delayed_enters_trigger_sees_a_paused_cast_entry_once() {
    for tie in [true, false] {
        let seed = find_seed("an FDoC Hydra roll", |seed| {
            let (mut runner, hydra, _) = fdoc_hydra(seed);
            pauses_on_tie(&mut runner, hydra, 2).is_some() == tie
        });
        let (mut runner, hydra, fdoc) = fdoc_hydra(seed);
        reset_triggers_seen();
        let mut events = Vec::new();
        cast_and_pass(&mut runner, hydra, Some(2), None, &mut events);
        let survivors = if tie {
            let naturals = tie_prompt_results(runner.state()).expect("DieKeepChoice reached");
            assert_eq!(queued_or_stacked_from(runner.state(), fdoc), 0, "the hold");
            Some(survivors_total(&naturals))
        } else {
            None
        };
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let total = survivors.unwrap_or_else(|| die_results(&events).iter().sum());
        assert_eq!(
            plus_counters(runner.state(), hydra),
            total + 1,
            "tie = {tie}"
        );
        assert!(runner.state().objects[&hydra].has_keyword(&Keyword::Haste));
        assert_eq!(take_triggers_seen_from(fdoc), 1, "tie = {tie}");
    }
}

/// R24. CR 603.2 + CR 603.7: custody runs the paused member's delayed triggers
/// once, on the owner route.
#[test]
fn custody_runs_a_paused_members_delayed_triggers_once() {
    fn fdoc_zombify(seed: u64) -> (GameRunner, ObjectId, ObjectId, ObjectId) {
        let mut scenario = board(seed);
        scenario.add_enchantment_from_oracle(P0, "Barbarian Class", BARBARIAN_CLASS_L1);
        let fdoc = free_spell(
            &mut scenario,
            "First Day of Class",
            false,
            FIRST_DAY_OF_CLASS,
        );
        let imp = graveyard_creature(&mut scenario, "Jumbo Imp", JUMBO_IMP);
        let zombify = free_spell(&mut scenario, "Zombify", false, ZOMBIFY);
        let mut runner = scenario.build();
        resolve_first_day_of_class(&mut runner, fdoc);
        (runner, imp, zombify, fdoc)
    }
    for tie in [true, false] {
        let seed = find_seed("an FDoC Imp roll", |seed| {
            let (mut runner, imp, zombify, _) = fdoc_zombify(seed);
            let mut events = Vec::new();
            cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
            tie_prompt_results(runner.state()).is_some() == tie
        });
        let (mut runner, imp, zombify, fdoc) = fdoc_zombify(seed);
        reset_triggers_seen();
        let mut events = Vec::new();
        cast_and_pass(&mut runner, zombify, None, Some(imp), &mut events);
        let survivors = if tie {
            let naturals = tie_prompt_results(runner.state()).expect("DieKeepChoice reached");
            let WaitingFor::DieKeepChoice {
                ignorable_indices,
                ignore_count,
                ..
            } = runner.state().waiting_for.clone()
            else {
                unreachable!()
            };
            act(
                &mut runner,
                GameAction::SelectDieRolls {
                    ignore_indices: ignorable_indices.into_iter().take(ignore_count).collect(),
                },
                &mut events,
            );
            assert!(runner.state().active_change_zone_frame().is_none());
            Some(survivors_total(&naturals))
        } else {
            None
        };
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let total = survivors.unwrap_or_else(|| die_results(&events).iter().sum());
        assert_eq!(plus_counters(runner.state(), imp), total + 1, "tie = {tie}");
        assert!(runner.state().objects[&imp].has_keyword(&Keyword::Haste));
        assert_eq!(take_triggers_seen_from(fdoc), 1, "tie = {tie}");
    }
}

/// R22. CR 603.2 + CR 603.7 + CR 603.3b: Beck's delayed trigger sees a Devour
/// entry once, in the sacrifice handler's drain beside the dies trigger.
#[test]
fn a_delayed_enters_trigger_sees_a_devour_entry_once() {
    for devour in [true, false] {
        let mut scenario = board(7);
        let beck = free_spell(&mut scenario, "Beck", false, BECK);
        let artist = scenario
            .add_creature_from_oracle(P0, "Blood Artist", 0, 1, BLOOD_ARTIST)
            .id();
        let victim = scenario.add_vanilla(P0, 2, 2);
        let elder = thunder_thrash_elder(&mut scenario);
        let mut runner = scenario.build();
        let mut events = Vec::new();
        cast_and_pass(&mut runner, beck, None, None, &mut events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert!(!runner.state().delayed_triggers.is_empty());
        let hand_before = hand(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, elder, None, None, &mut events);
        assert!(effect_zone_choice(runner.state()));
        let cards = if devour { vec![victim] } else { vec![] };
        act(&mut runner, GameAction::SelectCards { cards }, &mut events);
        assert_eq!(
            plus_counters(runner.state(), elder),
            if devour { 3 } else { 0 }
        );
        if devour {
            match &runner.state().waiting_for {
                WaitingFor::OrderTriggers { triggers, .. } => {
                    let sources: Vec<ObjectId> = triggers.iter().map(|t| t.source_id).collect();
                    assert!(sources.contains(&artist), "{sources:?}");
                    assert!(sources.contains(&beck), "{sources:?}");
                }
                other => panic!("one drain for both triggers, got {other:?}"),
            }
        }
        let settled = settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        let beck_offers = settled
            .optional_offers
            .iter()
            .filter(|source| **source == beck)
            .count();
        assert_eq!(beck_offers, 1, "devour = {devour}");
        assert_eq!(hand(runner.state(), P0) + 1 - hand_before, 1);
    }
}

// ---------------------------------------------------------------------------
// The release runs before the settling action's SBAs and its drain gate
// ---------------------------------------------------------------------------

fn slime_board(seed: u64, with_norn: bool) -> (GameRunner, ObjectId, ObjectId) {
    let mut scenario = board(seed);
    if with_norn {
        scenario.add_creature_from_oracle(P1, "Elesh Norn, Grand Cenobite", 4, 7, ELESH_NORN);
    }
    scenario.add_enchantment_from_oracle(P1, "Night of Souls' Betrayal", NIGHT_OF_SOULS_BETRAYAL);
    scenario.add_enchantment_from_oracle(P0, "Ajani's Welcome", AJANIS_WELCOME);
    let big = scenario.add_vanilla(P0, 4, 4);
    let slime = gluttonous_slime(&mut scenario);
    (scenario.build(), big, slime)
}

/// R23. CR 603.2 + CR 603.10: a completed entry's enters triggers survive the
/// entrant's SBA death in the settling action.
#[test]
fn a_completed_entry_triggers_even_if_the_entrant_dies_to_sbas() {
    for (with_norn, devour) in [(true, true), (false, true), (true, false)] {
        let (mut runner, big, slime) = slime_board(7, with_norn);
        let life_before = life(runner.state(), P0);
        let mut events = Vec::new();
        cast_and_pass(&mut runner, slime, None, None, &mut events);
        assert!(effect_zone_choice(runner.state()));
        assert!(
            runner.state().deferred_triggers.is_empty(),
            "the hold: nothing is queued at the prompt"
        );
        let cards = if devour { vec![big] } else { vec![] };
        let answer = runner
            .act(GameAction::SelectCards { cards })
            .expect("devour answer");
        if devour {
            assert_eq!(zone(runner.state(), big), Zone::Graveyard);
            let placed: u32 = answer
                .events
                .iter()
                .filter_map(|event| match event {
                    GameEvent::CounterAdded {
                        object_id,
                        counter_type: CounterType::Plus1Plus1,
                        count,
                        ..
                    } if *object_id == slime => Some(*count),
                    _ => None,
                })
                .sum();
            assert_eq!(placed, 1);
        }
        let dies = with_norn;
        if dies {
            assert_eq!(zone(runner.state(), slime), Zone::Graveyard, "0/0 or -1/-1");
        }
        events.extend(answer.events);
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        assert_eq!(
            zone(runner.state(), slime),
            if dies {
                Zone::Graveyard
            } else {
                Zone::Battlefield
            }
        );
        assert_eq!(
            life(runner.state(), P0),
            life_before + 1,
            "norn = {with_norn}, devour = {devour}"
        );
    }
}

/// R25. CR 603.3b: a refused drain keeps the released entry queued with the
/// dies context; both are placed together.
#[test]
fn a_refused_drain_keeps_the_released_entry_beside_the_dies_context() {
    for with_opt in [true, false] {
        let mut scenario = board(7);
        let artist = scenario
            .add_creature_from_oracle(P0, "Blood Artist", 0, 1, BLOOD_ARTIST)
            .id();
        let bond = scenario
            .add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND)
            .id();
        let victim = scenario.add_vanilla(P0, 2, 2);
        let opt = free_spell(&mut scenario, "Opt", true, OPT);
        let slime = gluttonous_slime(&mut scenario);
        let mut runner = scenario.build();
        let hand_before = hand(runner.state(), P0);
        let (p0_before, p1_before) = (life(runner.state(), P0), life(runner.state(), P1));
        let mut events = Vec::new();
        if with_opt {
            let commit = runner.cast(opt).commit();
            events.extend(commit.events().iter().cloned());
        }
        {
            let commit = runner.cast(slime).commit();
            events.extend(commit.events().iter().cloned());
        }
        // Resolve only the Slime (the top of the stack).
        act(&mut runner, GameAction::PassPriority, &mut events);
        act(&mut runner, GameAction::PassPriority, &mut events);
        assert!(
            effect_zone_choice(runner.state()),
            "{:?}",
            runner.state().waiting_for
        );
        act(
            &mut runner,
            GameAction::SelectCards {
                cards: vec![victim],
            },
            &mut events,
        );
        assert_eq!(plus_counters(runner.state(), slime), 1);
        let state = runner.state();
        let queued = |source| {
            state
                .deferred_triggers
                .iter()
                .any(|context| context.pending.source_id == source)
        };
        let ordering = match &state.waiting_for {
            WaitingFor::OrderTriggers { triggers, .. } => {
                triggers.iter().map(|t| t.source_id).collect::<Vec<_>>()
            }
            _ => Vec::new(),
        };
        let stacked = |source| {
            state.stack.iter().any(|entry| entry.source_id == source) || ordering.contains(&source)
        };
        assert_eq!(
            queued(artist),
            queued(bond),
            "same place: {:?}",
            state.waiting_for
        );
        assert_eq!(
            stacked(artist),
            stacked(bond),
            "same place: {:?}",
            state.waiting_for
        );
        if with_opt {
            assert!(
                state.stack.iter().any(|entry| entry.id == opt),
                "Opt still on the stack"
            );
        } else {
            assert!(
                ordering.contains(&artist) && ordering.contains(&bond),
                "{ordering:?}"
            );
        }
        settle(&mut runner, &mut events, DEFAULT_ANSWERS);
        // Cards cast from hand: the Slime, and Opt when it is cast; drawn: one
        // Elemental Bond draw, and Opt's own draw.
        let opt_cards = usize::from(with_opt);
        assert_eq!(
            hand(runner.state(), P0) + 1 + opt_cards - hand_before,
            1 + opt_cards
        );
        assert_eq!(life(runner.state(), P1), p1_before - 1);
        assert_eq!(life(runner.state(), P0), p0_before + 1);
    }
}
