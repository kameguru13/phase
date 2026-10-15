//! Zimone, All-Questioning — end to end through the real turn pipeline.
//!
//! Oracle: "At the beginning of your end step, if a land entered the battlefield
//! under your control this turn and you control a prime number of lands, create
//! Primo, the Indivisible, a legendary 0/0 green and blue Fractal creature token,
//! then put that many +1/+1 counters on it."
//!
//! - CR 513.1: the trigger fires as the end step begins.
//! - CR 603.4: the intervening-if is checked when the trigger event occurs and
//!   again on resolution; if false then, the ability does nothing.
//! - CR 608.2i: "a land entered … this turn" looks back; the land need not still
//!   be on the battlefield.
//! - CR 608.2h: "that many" is the land count as the effect is applied.

use engine::game::coverage::card_face_gaps;
use engine::game::game_object::GameObject;
use engine::game::restrictions::record_battlefield_entry;
use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::game::zones::{create_object, move_to_zone};
use engine::parser::parse_oracle_text;
use engine::types::actions::GameAction;
use engine::types::card::CardFace;
use engine::types::card_type::{CardType, CoreType, Supertype};
use engine::types::counter::CounterType;
use engine::types::game_state::WaitingFor;
use engine::types::identifiers::{CardId, ObjectId};
use engine::types::mana::ManaColor;
use engine::types::phase::Phase;
use engine::types::player::PlayerId;
use engine::types::zones::Zone;

const ZIMONE: &str = "At the beginning of your end step, if a land entered the battlefield under your control this turn and you control a prime number of lands, create Primo, the Indivisible, a legendary 0/0 green and blue Fractal creature token, then put that many +1/+1 counters on it. (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, and 31 are prime numbers.)";

const PRIMO: &str = "Primo, the Indivisible";

/// A board for P0: Zimone, `forests` Forests already in play, and (when
/// `land_in_hand`) one land in hand. Returns the runner and the hand land.
fn board(forests: usize, land_in_hand: bool) -> (GameRunner, Option<ObjectId>) {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.add_creature_from_oracle(P0, "Zimone, All-Questioning", 1, 1, ZIMONE);
    for _ in 0..forests {
        scenario.add_basic_land(P0, ManaColor::Green);
    }
    let hand_land = land_in_hand.then(|| scenario.add_land_to_hand(P0, "Wastes").id());
    (scenario.build(), hand_land)
}

/// Play `land` from hand through the real `PlayLand` action (which records the
/// battlefield entry).
fn play_land(runner: &mut GameRunner, land: ObjectId) {
    let card_id = runner.state().objects[&land].card_id;
    runner
        .act(GameAction::PlayLand {
            object_id: land,
            card_id,
        })
        .expect("the land is playable");
}

/// Put a land onto `player`'s battlefield with no entry-ledger stamp.
fn put_land(runner: &mut GameRunner, player: PlayerId) -> ObjectId {
    let state = runner.state_mut();
    let card_id = CardId(state.next_object_id);
    let id = create_object(
        state,
        card_id,
        player,
        "Extra Land".to_string(),
        Zone::Battlefield,
    );
    let obj = state.objects.get_mut(&id).unwrap();
    obj.card_types.core_types.push(CoreType::Land);
    obj.base_card_types = obj.card_types.clone();
    id
}

/// Every battlefield token named Primo, the Indivisible.
fn primos(runner: &GameRunner) -> Vec<&GameObject> {
    let state = runner.state();
    state
        .battlefield
        .iter()
        .filter_map(|id| state.objects.get(id))
        .filter(|o| o.is_token && o.name == PRIMO)
        .collect()
}

fn plus_counters(obj: &GameObject) -> u32 {
    obj.counters
        .get(&CounterType::Plus1Plus1)
        .copied()
        .unwrap_or(0)
}

/// Advance to the end step. Zimone can attack, so the turn stops at the
/// declare-attackers prompt (CR 508.1); declare no attackers and continue.
fn advance_to_end_step(runner: &mut GameRunner) {
    runner.advance_to_end_step();
    if matches!(
        runner.state().waiting_for,
        WaitingFor::DeclareAttackers { .. }
    ) {
        runner
            .declare_attackers(&[])
            .expect("declaring no attackers is legal");
        runner.advance_to_end_step();
    }
    assert_eq!(runner.state().phase, Phase::End);
}

/// Play the hand land, run the end step, drain the stack.
fn play_and_finish_turn(runner: &mut GameRunner, land: Option<ObjectId>) {
    if let Some(land) = land {
        play_land(runner, land);
    }
    advance_to_end_step(runner);
    runner.advance_until_stack_empty();
}

/// The one Primo on the battlefield and its +1/+1 counters.
fn the_primo_counters(runner: &GameRunner) -> u32 {
    let found = primos(runner);
    assert_eq!(found.len(), 1, "exactly one Primo");
    plus_counters(found[0])
}

#[test]
fn zimone_face_is_coverage_supported() {
    let parsed = parse_oracle_text(
        ZIMONE,
        "Zimone, All-Questioning",
        &[],
        &["Creature".to_string()],
        &["Human".to_string(), "Wizard".to_string()],
    );
    // Reach-guard: the face carries the one end-step trigger.
    assert_eq!(parsed.triggers.len(), 1);
    let face = CardFace {
        name: "Zimone, All-Questioning".to_string(),
        card_type: CardType {
            supertypes: vec![Supertype::Legendary],
            core_types: vec![CoreType::Creature],
            subtypes: vec!["Human".to_string(), "Wizard".to_string()],
        },
        oracle_text: Some(ZIMONE.to_string()),
        abilities: parsed.abilities,
        triggers: parsed.triggers,
        static_abilities: parsed.statics,
        replacements: parsed.replacements,
        ..Default::default()
    };
    assert_eq!(card_face_gaps(&face), Vec::<String>::new());
}

/// 1 Forest + 1 played = 2 lands (prime): Primo enters with two counters.
#[test]
fn two_lands_create_primo_with_two_counters() {
    let (mut runner, land) = board(1, true);
    play_and_finish_turn(&mut runner, land);
    let found = primos(&runner);
    assert_eq!(found.len(), 1, "exactly one Primo");
    let primo = found[0];
    assert!(primo.card_types.supertypes.contains(&Supertype::Legendary));
    assert!(primo.card_types.core_types.contains(&CoreType::Creature));
    assert!(primo.card_types.subtypes.iter().any(|s| s == "Fractal"));
    assert!(primo.color.contains(&ManaColor::Green));
    assert!(primo.color.contains(&ManaColor::Blue));
    // CR 704.5f: with its counters the 0/0 survives state-based actions.
    assert_eq!(plus_counters(primo), 2);
}

/// CR 603.4 (ruling 1): no land entered this turn — no trigger, even with a
/// prime land count. Paired with the same board plus a played land.
#[test]
fn no_land_entered_means_no_primo() {
    let (mut runner, _) = board(3, false);
    play_and_finish_turn(&mut runner, None);
    assert!(primos(&runner).is_empty());

    // Reach-guard: 2 Forests + 1 played = 3 lands (prime).
    let (mut runner, land) = board(2, true);
    play_and_finish_turn(&mut runner, land);
    assert_eq!(the_primo_counters(&runner), 3);
}

/// True primality, not parity or "more than one": 4, 1 and 9 lands make nothing.
#[test]
fn non_prime_land_counts_make_no_primo() {
    for forests in [3, 0, 8] {
        let (mut runner, land) = board(forests, true);
        play_and_finish_turn(&mut runner, land);
        assert!(primos(&runner).is_empty(), "{} lands", forests + 1);
    }
}

/// CR 603.4 + CR 608.2h: the count is read as the effect is applied, and a
/// count that stops being prime while on the stack removes the ability.
#[test]
fn resolution_recheck_and_live_count() {
    // (a) 2 lands at fire time, 3 at resolution → three counters.
    let (mut runner, land) = board(1, true);
    play_land(&mut runner, land.unwrap());
    advance_to_end_step(&mut runner);
    assert_eq!(runner.state().stack.len(), 1, "the trigger is on the stack");
    put_land(&mut runner, P0);
    runner.advance_until_stack_empty();
    assert_eq!(the_primo_counters(&runner), 3);

    // (b) 3 lands at fire time, 4 at resolution → the ability does nothing.
    let (mut runner, land) = board(2, true);
    play_land(&mut runner, land.unwrap());
    advance_to_end_step(&mut runner);
    assert_eq!(runner.state().stack.len(), 1, "the trigger is on the stack");
    put_land(&mut runner, P0);
    runner.advance_until_stack_empty();
    assert!(runner.state().stack.is_empty());
    assert!(primos(&runner).is_empty());
}

/// Ruling 3: primes continue past the reminder list's 31.
#[test]
fn thirty_seven_lands_is_prime() {
    let (mut runner, land) = board(36, true);
    play_and_finish_turn(&mut runner, land);
    assert_eq!(the_primo_counters(&runner), 37);
}

/// CR 608.2i: the entry still counts after the land has left the battlefield.
#[test]
fn entered_land_need_not_remain() {
    // (a) 2 Forests + 1 played, then the played land returns to hand: the entry
    // counts and 2 lands remain (prime).
    let (mut runner, land) = board(2, true);
    let land = land.unwrap();
    play_land(&mut runner, land);
    move_to_zone(runner.state_mut(), land, Zone::Hand, &mut Vec::new());
    play_and_finish_turn(&mut runner, None);
    assert_eq!(the_primo_counters(&runner), 2);

    // (b) 0 Forests + 1 played, then the land goes to the graveyard: the entry
    // counts but 0 lands remain (not prime).
    let (mut runner, land) = board(0, true);
    let land = land.unwrap();
    play_land(&mut runner, land);
    move_to_zone(runner.state_mut(), land, Zone::Graveyard, &mut Vec::new());
    play_and_finish_turn(&mut runner, None);
    assert!(primos(&runner).is_empty());
}

/// "you"/"your": only P0's lands count, and only an entry under P0's control.
#[test]
fn only_your_lands_and_your_entries_count() {
    // (a) P0: 2 Forests + 1 played = 3; P1's two Forests are not counted.
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.add_creature_from_oracle(P0, "Zimone, All-Questioning", 1, 1, ZIMONE);
    for _ in 0..2 {
        scenario.add_basic_land(P0, ManaColor::Green);
        scenario.add_basic_land(P1, ManaColor::Green);
    }
    let land = scenario.add_land_to_hand(P0, "Wastes").id();
    let mut runner = scenario.build();
    play_and_finish_turn(&mut runner, Some(land));
    assert_eq!(the_primo_counters(&runner), 3);

    // (b) P0: 3 Forests, no land played; a land entered under P1's control.
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.add_creature_from_oracle(P0, "Zimone, All-Questioning", 1, 1, ZIMONE);
    for _ in 0..3 {
        scenario.add_basic_land(P0, ManaColor::Green);
    }
    let mut runner = scenario.build();
    let opponent_land = put_land(&mut runner, P1);
    // Stamp the production entry ledger, as `record_zone_change` does.
    record_battlefield_entry(runner.state_mut(), opponent_land);
    play_and_finish_turn(&mut runner, None);
    assert!(primos(&runner).is_empty());
}
