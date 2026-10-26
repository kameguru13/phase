//! CR 800.4a + CR 603.6c: when a player leaves the game, every object they own
//! leaves the game. A phased-in permanent leaving this way triggers
//! leaves-the-battlefield abilities, but it is not exiled, so an ability that
//! names where an object went never sees it. An object a leaving player
//! controls but doesn't own is exiled, and both kinds of observer see that.

use engine::game::game_object::{PhaseOutCause, PhaseStatus};
use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::game::triggers::drain_order_triggers_with_identity;
use engine::types::actions::GameAction;
use engine::types::game_state::WaitingFor;
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

/// The board every test shares: P0 has Extractor Demon and Soulherder; P2 owns
/// a bear, and controls a bear P1 owns.
struct Board {
    runner: GameRunner,
    demon: ObjectId,
    soulherder: ObjectId,
    owned_bear: ObjectId,
    borrowed_bear: ObjectId,
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
    let owned_bear = scenario.add_creature(P2, "P2 Bear", 2, 2).id();
    let borrowed_bear = scenario
        .add_creature(P1, "P1 Bear", 2, 2)
        .controlled_by(P2)
        .id();
    configure(&mut scenario);
    Board {
        runner: scenario.build(),
        demon,
        soulherder,
        owned_bear,
        borrowed_bear,
    }
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

/// CR 104.3a + CR 800.4a + CR 603.6c: P2 concedes. Their bear leaves the game
/// and the bear they controlled is exiled (it is still controlled by P2 once the
/// owned objects are gone). Extractor Demon sees both leave the battlefield;
/// Soulherder sees only the exiled one.
///
/// REVERT-FAILING: without the concession collecting its triggers, the Demon
/// triggers zero times; without the leave-the-game departure, Soulherder
/// triggers for the bear that left the game too.
#[test]
fn concession_triggers_leaves_the_battlefield_but_not_exile_for_owned_permanents() {
    let Board {
        mut runner,
        demon,
        soulherder,
        owned_bear,
        borrowed_bear,
    } = board(|_| {});

    runner
        .act(GameAction::Concede { player_id: P2 })
        .expect("P2 concedes");
    assert_eq!(zone(&runner, owned_bear), Zone::Exile);
    assert_eq!(zone(&runner, borrowed_bear), Zone::Exile);

    drive_trigger_prompts(&mut runner);
    assert_eq!(
        triggers_from(&runner, demon),
        2,
        "CR 603.6c: Extractor Demon sees the conceding player's bear leave the game \
         and the bear they controlled get exiled"
    );
    assert_eq!(
        triggers_from(&runner, soulherder),
        1,
        "CR 800.4a: only the bear P2 controlled but didn't own was exiled"
    );
}

/// CR 704.5a + CR 800.4a + CR 603.6c: the same departure by losing the game to a
/// state-based action. The post-action pipeline already scans these events, so
/// this row pins the departure reading: the owned bear left the game and was not
/// exiled.
///
/// REVERT-FAILING: without the leave-the-game departure, Soulherder triggers
/// twice.
#[test]
fn losing_to_zero_life_triggers_leaves_the_battlefield_but_not_exile_for_owned_permanents() {
    let Board {
        mut runner,
        demon,
        soulherder,
        ..
    } = board(|scenario| {
        scenario.with_life(P2, 0);
    });

    runner
        .act(GameAction::PassPriority)
        .expect("state-based actions run");
    assert!(runner.state().players[2].is_eliminated);

    drive_trigger_prompts(&mut runner);
    assert_eq!(triggers_from(&runner, demon), 2);
    assert_eq!(
        triggers_from(&runner, soulherder),
        1,
        "CR 800.4a: only the bear P2 controlled but didn't own was exiled"
    );
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
        ..
    } = board(|_| {});
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
}

/// CR 800.4a + CR 603.10a: everything P2 owns leaves the game at once. An
/// Extractor Demon P2 owns but P0 controls leaves with P2's bear and, looking
/// back, sees the bear leave in the same event; P0 controls that trigger. A
/// trigger P2 would control isn't put on the stack (CR 800.4d).
///
/// REVERT-FAILING: without marking the departure simultaneous, the departing
/// Demon doesn't see the bear and triggers zero times.
#[test]
fn concession_departing_observer_sees_the_other_permanents_that_leave_with_it() {
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
    let mut runner = scenario.build();

    runner
        .act(GameAction::Concede { player_id: P2 })
        .expect("P2 concedes");
    assert_eq!(zone(&runner, borrowed_demon), Zone::Exile);

    drive_trigger_prompts(&mut runner);
    assert_eq!(
        triggers_from(&runner, borrowed_demon),
        2,
        "CR 603.10a: the departing Demon P0 controlled sees the other two creatures \
         leave with it"
    );
    assert_eq!(
        triggers_from(&runner, p2_demon),
        0,
        "CR 800.4d: a trigger the departed player would control isn't put on the stack"
    );
    assert!(runner
        .state()
        .stack
        .iter()
        .all(|entry| entry.controller == P0));
}
