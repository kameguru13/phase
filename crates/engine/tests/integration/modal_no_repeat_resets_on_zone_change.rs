//! CR 400.7 + CR 700.2: "choose one that hasn't been chosen" history belongs to
//! one object. A permanent that leaves the battlefield and returns is a new
//! object with no memory of its previous existence, so every mode is available
//! again. A control change is not a zone change and keeps the history.
//!
//! Oracle (Scryfall, Gandalf the Grey):
//!   Whenever you cast an instant or sorcery spell, choose one that hasn't been
//!   chosen —
//!   • You may tap or untap target permanent.
//!   • Gandalf the Grey deals 3 damage to each opponent.
//!   • Copy target instant or sorcery spell you control. You may choose new
//!     targets for the copy.
//!   • Put Gandalf the Grey on top of its owner's library.
//!
//! The engine reuses `ObjectId` as storage identity across zone moves, so the
//! per-object modal history must be cleared at the zone-exit boundary
//! (`zones::apply_zone_exit_cleanup`).

use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::game::zones::move_to_zone;
use engine::types::actions::GameAction;
use engine::types::game_state::{CastPaymentMode, WaitingFor};
use engine::types::identifiers::ObjectId;
use engine::types::mana::ManaCost;
use engine::types::phase::Phase;
use engine::types::zones::Zone;

const GANDALF_ORACLE: &str = "Whenever you cast an instant or sorcery spell, choose one that hasn't been chosen \u{2014}\n\
\u{2022} You may tap or untap target permanent.\n\
\u{2022} Gandalf the Grey deals 3 damage to each opponent.\n\
\u{2022} Copy target instant or sorcery spell you control. You may choose new targets for the copy.\n\
\u{2022} Put Gandalf the Grey on top of its owner's library.";

const INSTANT_ORACLE: &str = "You gain 1 life.";

/// The "deals 3 damage to each opponent" mode.
const DAMAGE_MODE: usize = 1;

struct Board {
    runner: GameRunner,
    gandalf: ObjectId,
    instants: Vec<ObjectId>,
}

fn board() -> Board {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    let gandalf = scenario
        .add_creature_from_oracle(P0, "Gandalf the Grey", 4, 5, GANDALF_ORACLE)
        .id();
    let instants = (0..2)
        .map(|i| {
            scenario
                .add_spell_to_hand_from_oracle(
                    P0,
                    &format!("Test Instant {i}"),
                    true,
                    INSTANT_ORACLE,
                )
                .with_mana_cost(ManaCost::zero())
                .id()
        })
        .collect();
    Board {
        runner: scenario.build(),
        gandalf,
        instants,
    }
}

/// Cast the given instant, answer Gandalf's mode prompt with the damage mode,
/// and resolve the stack. Returns the `unavailable_modes` the prompt offered.
fn cast_and_choose_damage(b: &mut Board, instant: ObjectId) -> Vec<usize> {
    let card_id = b.runner.state().objects[&instant].card_id;
    b.runner
        .act(GameAction::CastSpell {
            object_id: instant,
            card_id,
            targets: vec![],
            payment_mode: CastPaymentMode::Auto,
        })
        .expect("casting the instant must be accepted");

    let mut unavailable: Option<Vec<usize>> = None;
    for _ in 0..40 {
        match b.runner.state().waiting_for.clone() {
            WaitingFor::OrderTriggers { triggers, .. } => {
                b.runner
                    .act(GameAction::OrderTriggers {
                        order: (0..triggers.len()).collect(),
                    })
                    .expect("ordering triggers must be accepted");
            }
            WaitingFor::AbilityModeChoice {
                source_id,
                unavailable_modes,
                ..
            } => {
                assert_eq!(source_id, b.gandalf, "reach: the prompt is Gandalf's");
                unavailable = Some(unavailable_modes);
                b.runner
                    .act(GameAction::SelectModes {
                        indices: vec![DAMAGE_MODE],
                    })
                    .expect("choosing the damage mode must be accepted");
            }
            WaitingFor::Priority { .. } if b.runner.state().stack.is_empty() => {
                return unavailable.expect("reach: Gandalf's mode prompt must surface");
            }
            WaitingFor::Priority { .. } => {
                b.runner
                    .act(GameAction::PassPriority)
                    .expect("passing priority must be accepted");
            }
            other => panic!("unexpected wait: {}", other.variant_name()),
        }
    }
    panic!("the stack did not empty");
}

fn life(b: &Board, player: engine::types::player::PlayerId) -> i32 {
    b.runner.state().players[player.0 as usize].life
}

/// Control: with no zone change, the second trigger cannot repeat the damage
/// mode.
#[test]
fn gandalf_mode_stays_unavailable_without_zone_change() {
    let mut b = board();
    let first = b.instants[0];
    let second = b.instants[1];
    let life_start = life(&b, P1);

    let unavailable = cast_and_choose_damage(&mut b, first);
    assert!(unavailable.is_empty(), "first trigger: every mode is open");
    assert_eq!(
        life(&b, P1),
        life_start - 3,
        "reach: the damage mode resolved"
    );

    let card_id = b.runner.state().objects[&second].card_id;
    b.runner
        .act(GameAction::CastSpell {
            object_id: second,
            card_id,
            targets: vec![],
            payment_mode: CastPaymentMode::Auto,
        })
        .expect("casting the second instant must be accepted");
    for _ in 0..8 {
        match b.runner.state().waiting_for.clone() {
            WaitingFor::OrderTriggers { triggers, .. } => {
                b.runner
                    .act(GameAction::OrderTriggers {
                        order: (0..triggers.len()).collect(),
                    })
                    .expect("ordering triggers must be accepted");
            }
            WaitingFor::AbilityModeChoice {
                unavailable_modes, ..
            } => {
                assert_eq!(
                    unavailable_modes,
                    vec![DAMAGE_MODE],
                    "CR 700.2: the same object cannot choose the damage mode twice"
                );
                assert!(
                    b.runner
                        .act(GameAction::SelectModes {
                            indices: vec![DAMAGE_MODE],
                        })
                        .is_err(),
                    "CR 700.2: re-choosing a used mode is rejected"
                );
                return;
            }
            other => panic!("unexpected wait: {}", other.variant_name()),
        }
    }
    panic!("reach: Gandalf's second mode prompt must surface");
}

/// CR 400.7: bouncing Gandalf and returning it makes a new object, so the
/// damage mode is available again and resolves a second time.
#[test]
fn gandalf_mode_history_resets_after_leaving_and_returning() {
    let mut b = board();
    let first = b.instants[0];
    let second = b.instants[1];
    let life_start = life(&b, P1);

    assert!(cast_and_choose_damage(&mut b, first).is_empty());
    assert_eq!(
        life(&b, P1),
        life_start - 3,
        "reach: the damage mode resolved"
    );

    let gandalf = b.gandalf;
    let mut events = Vec::new();
    move_to_zone(b.runner.state_mut(), gandalf, Zone::Hand, &mut events);
    move_to_zone(
        b.runner.state_mut(),
        gandalf,
        Zone::Battlefield,
        &mut events,
    );
    assert_eq!(
        b.runner.state().objects[&gandalf].zone,
        Zone::Battlefield,
        "reach: Gandalf is back on the battlefield"
    );

    let unavailable = cast_and_choose_damage(&mut b, second);
    assert!(
        unavailable.is_empty(),
        "CR 400.7: the returned Gandalf has no memory of earlier choices, got {unavailable:?}"
    );
    assert_eq!(
        life(&b, P1),
        life_start - 6,
        "the returned Gandalf deals 3 damage again"
    );
}

/// CR 400.7: zone exit clears the turn-scoped no-repeat set for the departing
/// object only; an unrelated object's entry survives.
#[test]
fn zone_exit_clears_turn_scoped_modal_history_for_departing_object_only() {
    let mut b = board();
    let gandalf = b.gandalf;
    let unrelated = ObjectId(9_999);
    {
        let state = b.runner.state_mut();
        state.modal_modes_chosen_this_turn.insert((gandalf, 0));
        state.modal_modes_chosen_this_turn.insert((unrelated, 0));
    }
    assert!(
        b.runner
            .state()
            .modal_modes_chosen_this_turn
            .contains(&(gandalf, 0)),
        "reach: the seeded entry is present before the move"
    );

    let mut events = Vec::new();
    move_to_zone(b.runner.state_mut(), gandalf, Zone::Graveyard, &mut events);

    let chosen = &b.runner.state().modal_modes_chosen_this_turn;
    assert!(
        !chosen.contains(&(gandalf, 0)),
        "CR 400.7: the departed object's turn-scoped history is cleared"
    );
    assert!(
        chosen.contains(&(unrelated, 0)),
        "an unrelated object's history is untouched"
    );
}

/// A control change is not a zone change: the permanent remains the same
/// object and keeps its no-repeat history. The control change is applied by
/// writing `controller` directly (P0 -> P1 -> P0); no zone-exit path runs.
#[test]
fn gandalf_mode_history_survives_control_change() {
    let mut b = board();
    let first = b.instants[0];
    assert!(cast_and_choose_damage(&mut b, first).is_empty());

    let gandalf = b.gandalf;
    for controller in [P1, P0] {
        b.runner
            .state_mut()
            .objects
            .get_mut(&gandalf)
            .expect("Gandalf exists")
            .controller = controller;
    }
    assert!(
        b.runner
            .state()
            .modal_modes_chosen_this_game
            .contains(&(gandalf, DAMAGE_MODE)),
        "the same object keeps its game-scoped no-repeat history"
    );
}
