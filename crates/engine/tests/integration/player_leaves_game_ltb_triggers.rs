//! CR 800.4a + CR 603.6c: when a player leaves the game, every object they own
//! leaves the game. A phased-in permanent leaving this way triggers
//! leaves-the-battlefield abilities, but it is not exiled, so an ability that
//! names where an object went never sees it. An object a leaving player
//! controls but doesn't own is exiled, and both kinds of observer see that.

use engine::game::game_object::{PhaseOutCause, PhaseStatus};
use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::game::triggers::drain_order_triggers_with_identity;
use engine::types::ability::{
    DelayedTriggerCondition, Effect, QuantityExpr, ResolvedAbility, TargetFilter,
};
use engine::types::actions::GameAction;
use engine::types::game_state::{DelayedTrigger, WaitingFor};
use engine::types::identifiers::ObjectId;
use engine::types::phase::Phase;
use engine::types::player::PlayerId;
use engine::types::zones::Zone;

const P2: PlayerId = PlayerId(2);

const EXTRACTOR_DEMON_ORACLE: &str = "Flying\n\
     Whenever another creature leaves the battlefield, you may have target player mill two \
     cards.";

const SOULHERDER_ORACLE: &str =
    "Whenever a creature is exiled from the battlefield, put a +1/+1 counter on this creature.";

const EXILE_WATCHER_ORACLE: &str =
    "Whenever a creature is put into exile from the battlefield, you gain 1 life.";

/// The board every test shares: P0 has Extractor Demon, Soulherder and an
/// exile watcher (a zone-change trigger naming exile as the destination); P2
/// owns a bear, and controls a bear P1 owns. Two P0 enchantments are the sources
/// of delayed abilities a test may install on the owned bear.
struct Board {
    runner: GameRunner,
    demon: ObjectId,
    soulherder: ObjectId,
    exile_watcher: ObjectId,
    owned_bear: ObjectId,
    borrowed_bear: ObjectId,
    leaves_marker: ObjectId,
    dies_or_exiled_marker: ObjectId,
}

fn board(configure: impl FnOnce(&mut GameScenario)) -> Board {
    let mut scenario = GameScenario::new_n_player(3, 42);
    scenario.at_phase(Phase::PreCombatMain);
    let demon = scenario
        .add_creature_from_oracle(P0, "Extractor Demon", 5, 5, EXTRACTOR_DEMON_ORACLE)
        .id();
    let soulherder = scenario
        .add_creature_from_oracle(P0, "Soulherder", 1, 1, SOULHERDER_ORACLE)
        .id();
    let exile_watcher = scenario
        .add_creature_from_oracle(P0, "Exile Watcher", 1, 1, EXILE_WATCHER_ORACLE)
        .id();
    let owned_bear = scenario.add_creature(P2, "P2 Bear", 2, 2).id();
    let borrowed_bear = scenario
        .add_creature(P1, "P1 Bear", 2, 2)
        .controlled_by(P2)
        .id();
    let leaves_marker = scenario
        .add_creature(P0, "Leaves Marker", 0, 0)
        .as_enchantment()
        .id();
    let dies_or_exiled_marker = scenario
        .add_creature(P0, "Dies Or Exiled Marker", 0, 0)
        .as_enchantment()
        .id();
    configure(&mut scenario);
    Board {
        runner: scenario.build(),
        demon,
        soulherder,
        exile_watcher,
        owned_bear,
        borrowed_bear,
        leaves_marker,
        dies_or_exiled_marker,
    }
}

/// CR 603.7c: install a one-shot delayed ability P0 controls, sourced from
/// `marker`, that gains P0 one life when `condition` is met.
fn install_delayed_gain_life(
    runner: &mut GameRunner,
    marker: ObjectId,
    condition: DelayedTriggerCondition,
) {
    runner
        .state_mut()
        .delayed_triggers
        .push(DelayedTrigger::new(
            condition,
            Box::new(ResolvedAbility::new(
                Effect::GainLife {
                    amount: QuantityExpr::Fixed { value: 1 },
                    player: TargetFilter::Controller,
                },
                vec![],
                marker,
                P0,
            )),
            P0,
            marker,
            true,
        ));
}

/// The two delayed abilities on the owned bear: "when it leaves the
/// battlefield" and "when it dies or is exiled".
fn install_delayed_watchers_on_owned_bear(board: &mut Board) {
    let bear = board.owned_bear;
    install_delayed_gain_life(
        &mut board.runner,
        board.leaves_marker,
        DelayedTriggerCondition::WhenLeavesPlay { object_id: bear },
    );
    install_delayed_gain_life(
        &mut board.runner,
        board.dies_or_exiled_marker,
        DelayedTriggerCondition::WhenDiesOrExiled {
            filter: TargetFilter::SpecificObject { id: bear },
        },
    );
}

/// Put pending triggers on the stack: order them as listed and give each the
/// first legal target, until a player has priority.
fn drive_trigger_prompts(runner: &mut GameRunner) {
    for _ in 0..64 {
        match &runner.state().waiting_for {
            WaitingFor::Priority { .. } => return,
            WaitingFor::OrderTriggers { .. } => {
                drain_order_triggers_with_identity(runner.state_mut());
            }
            WaitingFor::TriggerTargetSelection { .. } => {
                runner
                    .choose_first_legal_target()
                    .expect("a trigger takes its first legal target");
            }
            other => panic!("unexpected prompt while putting triggers on the stack: {other:?}"),
        }
    }
    panic!("triggers were not put on the stack within the prompt budget");
}

fn triggers_from(runner: &GameRunner, source: ObjectId) -> usize {
    runner
        .state()
        .stack
        .iter()
        .filter(|entry| entry.source_id == source)
        .count()
}

fn zone(runner: &GameRunner, id: ObjectId) -> Zone {
    runner.state().objects[&id].zone
}

/// Assert what P0's observers put on the stack for P2's departure: the bear P2
/// owned left the game, and the bear P2 controlled but didn't own was exiled.
fn assert_departure_triggers(board: &Board) {
    let runner = &board.runner;
    assert_eq!(
        triggers_from(runner, board.demon),
        2,
        "CR 603.6c: Extractor Demon sees the bear P2 owned leave the game and the bear \
         P2 controlled get exiled"
    );
    assert_eq!(
        triggers_from(runner, board.soulherder),
        1,
        "CR 800.4a: only the bear P2 controlled but didn't own was exiled"
    );
    assert_eq!(
        triggers_from(runner, board.exile_watcher),
        1,
        "CR 800.4a: a trigger naming exile as the destination sees only the exiled bear"
    );
}

/// Assert what the two delayed abilities on the owned bear put on the stack.
fn assert_delayed_departure_triggers(
    runner: &GameRunner,
    leaves: ObjectId,
    dies_or_exiled: ObjectId,
) {
    assert_eq!(
        triggers_from(runner, leaves),
        1,
        "CR 603.6c: the delayed leaves-the-battlefield ability sees the bear leave the game"
    );
    assert_eq!(
        triggers_from(runner, dies_or_exiled),
        0,
        "CR 800.4a: the bear that left the game neither died nor was exiled"
    );
}

/// CR 104.3a + CR 800.4a + CR 603.6c: P2 concedes. Their bear leaves the game
/// and the bear they controlled is exiled (it is still controlled by P2 once the
/// owned objects are gone). Leaves-the-battlefield observers, ordinary and
/// delayed, see both leave; exile observers see only the exiled one.
///
/// REVERT-FAILING: without the concession collecting its triggers (ordinary and
/// delayed), the Demon and the delayed leaves ability trigger zero times; without
/// the leave-the-game departure, Soulherder and the exile watcher trigger for the
/// bear that left the game too.
#[test]
fn concession_triggers_leaves_the_battlefield_but_not_exile_for_owned_permanents() {
    let mut board = board(|_| {});
    install_delayed_watchers_on_owned_bear(&mut board);

    board
        .runner
        .act(GameAction::Concede { player_id: P2 })
        .expect("P2 concedes");
    assert_eq!(zone(&board.runner, board.owned_bear), Zone::Exile);
    assert_eq!(zone(&board.runner, board.borrowed_bear), Zone::Exile);

    drive_trigger_prompts(&mut board.runner);
    assert_departure_triggers(&board);
    assert_delayed_departure_triggers(
        &board.runner,
        board.leaves_marker,
        board.dies_or_exiled_marker,
    );
}

/// CR 704.5a + CR 800.4a + CR 603.6c: the same departure by losing the game to a
/// state-based action. The post-action pipeline already scans these events, so
/// this row pins the departure reading: the owned bear left the game and was not
/// exiled.
///
/// REVERT-FAILING: without the leave-the-game departure, Soulherder and the exile
/// watcher trigger twice.
#[test]
fn losing_to_zero_life_triggers_leaves_the_battlefield_but_not_exile_for_owned_permanents() {
    let mut board = board(|scenario| {
        scenario.with_life(P2, 0);
    });

    board
        .runner
        .act(GameAction::PassPriority)
        .expect("state-based actions run");
    assert!(board.runner.state().players[2].is_eliminated);

    drive_trigger_prompts(&mut board.runner);
    assert_departure_triggers(&board);
}

/// CR 104.3a + CR 104.1: a concession that ends the game puts nothing on the
/// stack, though the conceding player's creature left the battlefield.
#[test]
fn concession_that_ends_the_game_puts_no_trigger_on_the_stack() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.add_creature_from_oracle(P0, "Extractor Demon", 5, 5, EXTRACTOR_DEMON_ORACLE);
    scenario.add_creature(P1, "P1 Bear", 2, 2);
    let mut runner = scenario.build();

    runner
        .act(GameAction::Concede { player_id: P1 })
        .expect("P1 concedes");
    assert_eq!(
        runner.state().waiting_for,
        WaitingFor::GameOver { winner: Some(P0) }
    );
    assert!(runner.state().stack.is_empty());
    assert!(runner.state().deferred_triggers.is_empty());
}

/// CR 702.26k: a phased-out permanent owned by a leaving player also leaves the
/// game, but doesn't cause zone-change abilities to trigger. The reach guard is
/// the phased-in bear P2 controlled, which the Demon still sees exiled.
///
/// REVERT-FAILING: without the phased-out departure, the Demon also triggers for
/// the phased-out bear.
#[test]
fn concession_with_a_phased_out_permanent_triggers_nothing_for_it() {
    let Board {
        mut runner,
        demon,
        soulherder,
        owned_bear,
        leaves_marker,
        dies_or_exiled_marker: filtered_leaves_marker,
        ..
    } = board(|_| {});
    install_delayed_gain_life(
        &mut runner,
        leaves_marker,
        DelayedTriggerCondition::WhenLeavesPlay {
            object_id: owned_bear,
        },
    );
    install_delayed_gain_life(
        &mut runner,
        filtered_leaves_marker,
        DelayedTriggerCondition::WhenLeavesPlayFiltered {
            filter: TargetFilter::SpecificObject { id: owned_bear },
        },
    );
    runner
        .state_mut()
        .objects
        .get_mut(&owned_bear)
        .expect("the bear is on the battlefield")
        .phase_status = PhaseStatus::PhasedOut {
        cause: PhaseOutCause::Directly,
    };

    runner
        .act(GameAction::Concede { player_id: P2 })
        .expect("P2 concedes");
    assert_eq!(zone(&runner, owned_bear), Zone::Exile);

    drive_trigger_prompts(&mut runner);
    assert_eq!(
        triggers_from(&runner, demon),
        1,
        "CR 702.26k: only the exiled bear, not the phased-out one, is seen leaving"
    );
    assert_eq!(triggers_from(&runner, soulherder), 1);
    for marker in [leaves_marker, filtered_leaves_marker] {
        assert_eq!(
            triggers_from(&runner, marker),
            0,
            "CR 702.26k: a delayed leaves-the-battlefield ability doesn't see it either"
        );
    }
}

/// How P2 leaves the game in a test.
#[derive(Clone, Copy, Debug)]
enum Departure {
    /// CR 104.3a: P2 concedes.
    Concession,
    /// CR 704.5a: P2 is at 0 life when P0 passes priority.
    ZeroLife,
}

/// CR 800.4a + CR 603.10a: everything P2 owns leaves the game at once. An
/// Extractor Demon P2 owns but P0 controls leaves with P2's other creatures and,
/// looking back, sees them leave in the same event; P0 controls those triggers.
/// The Demon P2 controlled sees them too, but a trigger P2 would control isn't
/// put on the stack (CR 800.4d), and the game doesn't wait on P2.
fn assert_departing_observer_sees_the_others(departure: Departure) {
    let mut scenario = GameScenario::new_n_player(3, 42);
    scenario.at_phase(Phase::PreCombatMain);
    let borrowed_demon = scenario
        .add_creature_from_oracle(P2, "Extractor Demon", 5, 5, EXTRACTOR_DEMON_ORACLE)
        .controlled_by(P0)
        .id();
    let p2_demon = scenario
        .add_creature_from_oracle(P2, "Extractor Demon", 5, 5, EXTRACTOR_DEMON_ORACLE)
        .id();
    scenario.add_creature(P2, "P2 Bear", 2, 2);
    if matches!(departure, Departure::ZeroLife) {
        scenario.with_life(P2, 0);
    }
    let mut runner = scenario.build();

    let action = match departure {
        Departure::Concession => GameAction::Concede { player_id: P2 },
        Departure::ZeroLife => GameAction::PassPriority,
    };
    runner.act(action).expect("P2 leaves the game");
    assert!(runner.state().players[2].is_eliminated);
    assert_eq!(zone(&runner, borrowed_demon), Zone::Exile);
    assert_ne!(
        runner.state().waiting_for.acting_player(),
        Some(P2),
        "{departure:?}: CR 800.4a: the game doesn't wait on a player who left"
    );

    drive_trigger_prompts(&mut runner);
    assert_eq!(
        triggers_from(&runner, borrowed_demon),
        2,
        "{departure:?}: CR 603.10a: the departing Demon P0 controlled sees the other two creatures leave with it"
    );
    assert_eq!(
        triggers_from(&runner, p2_demon),
        0,
        "{departure:?}: CR 800.4d: a trigger the departed player would control isn't put on the stack"
    );
    assert!(runner
        .state()
        .stack
        .iter()
        .all(|entry| entry.controller == P0));
}

/// REVERT-FAILING: without marking the departure simultaneous, the departing
/// Demon doesn't see the others and triggers zero times.
#[test]
fn concession_departing_observer_sees_the_other_permanents_that_leave_with_it() {
    assert_departing_observer_sees_the_others(Departure::Concession);
}

/// REVERT-FAILING: without dropping triggers a departed player would control at
/// collection, P2's own Demon goes on the stack and the game waits on P2 to
/// choose its target.
#[test]
fn losing_to_zero_life_departing_observer_sees_the_other_permanents_that_leave_with_it() {
    assert_departing_observer_sees_the_others(Departure::ZeroLife);
}

/// CR 704.5a + CR 800.4a + CR 603.6c + CR 603.7c: delayed abilities on a
/// permanent whose owner loses to a state-based action. "When it leaves the
/// battlefield" sees it leave the game (the reach guard); "when it dies or is
/// exiled" doesn't, since it did neither. No ordinary trigger opens a prompt on
/// this board.
///
/// REVERT-FAILING: without the departure gate on the delayed matchers, the
/// "dies or is exiled" ability triggers.
#[test]
fn losing_to_zero_life_delayed_abilities_read_the_departure_as_leaving_the_game() {
    let mut scenario = GameScenario::new_n_player(3, 42);
    scenario.at_phase(Phase::PreCombatMain);
    let bear = scenario.add_creature(P2, "P2 Bear", 2, 2).id();
    let leaves = scenario
        .add_creature(P0, "Leaves Marker", 0, 0)
        .as_enchantment()
        .id();
    let dies_or_exiled = scenario
        .add_creature(P0, "Dies Or Exiled Marker", 0, 0)
        .as_enchantment()
        .id();
    scenario.with_life(P2, 0);
    let mut runner = scenario.build();
    install_delayed_gain_life(
        &mut runner,
        leaves,
        DelayedTriggerCondition::WhenLeavesPlay { object_id: bear },
    );
    install_delayed_gain_life(
        &mut runner,
        dies_or_exiled,
        DelayedTriggerCondition::WhenDiesOrExiled {
            filter: TargetFilter::SpecificObject { id: bear },
        },
    );

    runner
        .act(GameAction::PassPriority)
        .expect("state-based actions run");
    assert!(runner.state().players[2].is_eliminated);
    drive_trigger_prompts(&mut runner);
    assert_delayed_departure_triggers(&runner, leaves, dies_or_exiled);
}
