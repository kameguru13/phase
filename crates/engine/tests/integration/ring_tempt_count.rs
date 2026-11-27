use engine::game::combat::AttackTarget;
use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::types::actions::GameAction;
use engine::types::game_state::WaitingFor;
use engine::types::mana::{parse_mtgjson_mana_cost, ManaColor, ManaType, ManaUnit};
use engine::types::phase::Phase;
use engine::types::Zone;
use engine::types::{ObjectId, PlayerId};

const SAGA_NAME: &str = "One Ring to Rule Them All";
const SAGA_ORACLE: &str = "(As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)\nI — The Ring tempts you, then each player mills cards equal to your Ring-bearer's power.\nII — Destroy all nonlegendary creatures.\nIII — Each opponent loses 1 life for each creature card in that player's graveyard.";
const FRODO_NAME: &str = "Frodo, Adventurous Hobbit";
const FRODO_ORACLE: &str = "Partner with Sam, Loyal Attendant\nVigilance\nWhenever Frodo attacks, if you gained 3 or more life this turn, the Ring tempts you. Then if Frodo is your Ring-bearer and the Ring has tempted you two or more times this game, draw a card.";

fn mana(types: &[ManaType]) -> Vec<ManaUnit> {
    types
        .iter()
        .map(|&kind| ManaUnit::new(kind, ObjectId(0), false, vec![]))
        .collect()
}

fn add_saga(scenario: &mut GameScenario) -> ObjectId {
    scenario
        .add_spell_to_hand(P0, SAGA_NAME, false)
        .as_enchantment()
        .with_subtypes(vec!["Saga"])
        .with_mana_cost(parse_mtgjson_mana_cost("{2}{B}{B}"))
        .from_oracle_text(SAGA_ORACLE)
        .id()
}

fn add_frodo(scenario: &mut GameScenario) -> ObjectId {
    scenario
        .add_creature(P0, FRODO_NAME, 1, 3)
        .as_legendary()
        .with_subtypes(vec!["Halfling", "Scout"])
        .from_oracle_text_with_keywords(&["Partner", "Partner with", "Vigilance"], FRODO_ORACLE)
        .id()
}

fn add_revitalize(scenario: &mut GameScenario) -> ObjectId {
    scenario
        .add_spell_to_hand_from_oracle(P0, "Revitalize", true, "You gain 3 life.\nDraw a card.")
        .with_mana_cost(parse_mtgjson_mana_cost("{1}{W}"))
        .id()
}

fn add_birthday_escape(scenario: &mut GameScenario) -> ObjectId {
    scenario
        .add_spell_to_hand_from_oracle(
            P0,
            "Birthday Escape",
            false,
            "Draw a card. The Ring tempts you.",
        )
        .with_mana_cost(parse_mtgjson_mana_cost("{U}"))
        .id()
}

fn choose_bearer(runner: &mut GameRunner, player: PlayerId, bearer: ObjectId) {
    assert!(matches!(
        &runner.state().waiting_for,
        WaitingFor::ChooseRingBearer { player: chooser, candidates }
            if *chooser == player && candidates.contains(&bearer)
    ));
    runner
        .act(GameAction::ChooseRingBearer { target: bearer })
        .expect("choose the offered Ring-bearer");
    runner.advance_until_stack_empty();
    assert!(runner.state().stack.is_empty());
    assert!(matches!(
        runner.state().waiting_for,
        WaitingFor::Priority { .. }
    ));
}

#[test]
fn saga_chapter_one_uses_the_bearer_just_chosen_for_every_player() {
    let mut scenario = GameScenario::new_n_player(3, 42);
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_mana_pool(
        P0,
        mana(&[
            ManaType::Colorless,
            ManaType::Colorless,
            ManaType::Black,
            ManaType::Black,
        ]),
    );
    scenario.add_creature(P0, "Small candidate", 1, 1);
    let bearer = scenario
        .add_creature_from_oracle(
            P0,
            "Boneyard Wurm",
            0,
            0,
            "Boneyard Wurm's power and toughness are each equal to the number of creature cards in your graveyard.",
        )
        .id();
    for _ in 0..3 {
        scenario.add_creature_to_graveyard(P0, "Grizzly Bears", 2, 2);
    }
    for _ in 0..5 {
        scenario
            .add_spell_to_library_top(P0, "Grizzly Bears", false)
            .as_creature();
    }
    scenario.add_creature(P1, "Opponent's creature", 5, 5);
    for player in [P1, PlayerId(2)] {
        for _ in 0..5 {
            scenario.add_card_to_library_top(player, "Swamp");
        }
    }
    let saga = add_saga(&mut scenario);
    let mut runner = scenario.build();
    runner.cast(saga).commit();
    runner.advance_until_stack_empty();
    assert!(runner.state().ring_tempt_count.is_empty());
    assert!(runner
        .state()
        .players
        .iter()
        .all(|player| player.library.len() == 5));
    assert_eq!(runner.state().objects[&bearer].power, Some(3));

    choose_bearer(&mut runner, P0, bearer);

    assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&1));
    // The controller's own mill increased the bearer's power. Every seat
    // still mills the original three cards from this one instruction.
    assert_eq!(runner.state().objects[&bearer].power, Some(6));
    for player in &runner.state().players {
        assert_eq!(player.library.len(), 2, "{:?}", player.id);
        assert_eq!(player.graveyard.len(), if player.id == P0 { 6 } else { 3 });
    }
}

#[test]
fn frodo_draws_only_with_a_second_completed_temptation_and_his_own_designation() {
    for (prior_temptation, choose_frodo, expect_draw) in [
        (false, true, false),
        (true, false, false),
        (true, true, true),
    ] {
        let mut scenario = GameScenario::new();
        scenario.at_phase(Phase::PreCombatMain);
        scenario.with_mana_pool(
            P0,
            mana(&[ManaType::Colorless, ManaType::White, ManaType::Blue]),
        );
        let frodo = add_frodo(&mut scenario);
        let other = scenario.add_creature(P0, "Other candidate", 2, 2).id();
        for _ in 0..5 {
            scenario.add_card_to_library_top(P0, "Plains");
        }
        let revitalize = add_revitalize(&mut scenario);
        let birthday = add_birthday_escape(&mut scenario);
        let mut runner = scenario.build();
        runner.cast(revitalize).resolve();
        assert_eq!(runner.state().players[0].life_gained_this_turn, 3);
        if prior_temptation {
            runner.cast(birthday).commit();
            runner.advance_until_stack_empty();
            choose_bearer(&mut runner, P0, other);
        }
        let count = u32::from(prior_temptation);
        let hand = runner.state().players[0].hand.len();
        runner.advance_to_combat();
        runner
            .declare_attackers(&[(frodo, AttackTarget::Player(P1))])
            .unwrap();
        assert!(
            !runner.state().stack.is_empty(),
            "Frodo's attack must trigger"
        );
        runner.advance_until_stack_empty();
        assert_eq!(
            runner
                .state()
                .ring_tempt_count
                .get(&P0)
                .copied()
                .unwrap_or(0),
            count
        );
        let bearer = if choose_frodo { frodo } else { other };
        choose_bearer(&mut runner, P0, bearer);
        assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&(count + 1)));
        assert_eq!(
            runner.state().players[0].hand.len(),
            hand + usize::from(expect_draw)
        );
    }
}

#[test]
fn invalid_ring_bearer_choice_leaves_the_count_unchanged() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_mana_pool(P0, mana(&[ManaType::Blue]));
    scenario.add_card_to_library_top(P0, "Island");
    let first = scenario.add_creature(P0, "Grizzly Bears", 2, 2).id();
    let second = scenario.add_creature(P0, "Runeclaw Bear", 2, 2).id();
    let opponent = scenario.add_creature(P1, "Silvercoat Lion", 2, 2).id();
    let birthday = add_birthday_escape(&mut scenario);
    let mut runner = scenario.build();
    runner.cast(birthday).commit();
    runner.advance_until_stack_empty();

    let offers_both_candidates = |waiting: &WaitingFor| {
        matches!(
            waiting,
            WaitingFor::ChooseRingBearer { player: P0, candidates }
                if candidates.len() == 2 && candidates.contains(&first) && candidates.contains(&second)
        )
    };
    assert!(offers_both_candidates(&runner.state().waiting_for));
    assert!(runner
        .act(GameAction::ChooseRingBearer { target: opponent })
        .is_err());

    // CR 701.54a + CR 701.54d: Only a completed choice counts when candidates exist.
    assert!(!runner.state().ring_tempt_count.contains_key(&P0));
    assert!(offers_both_candidates(&runner.state().waiting_for));
    choose_bearer(&mut runner, P0, first);
    assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&1));
}

#[test]
fn blinked_frodo_does_not_draw() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_mana_pool(
        P0,
        mana(&[ManaType::Colorless, ManaType::White, ManaType::Blue]),
    );
    // CR 106.4 + CR 500.5: The main-phase pool empties before combat, so Cloudshift is
    // paid from a land during the declare-attackers step.
    scenario.add_basic_land(P0, ManaColor::White);
    scenario.with_library_top(P0, &["Plains", "Plains", "Plains"]);
    let frodo = add_frodo(&mut scenario);
    let other = scenario.add_creature(P0, "Grizzly Bears", 2, 2).id();
    let revitalize = add_revitalize(&mut scenario);
    let birthday = add_birthday_escape(&mut scenario);
    let cloudshift = scenario
        .add_spell_to_hand_from_oracle(
            P0,
            "Cloudshift",
            true,
            "Exile target creature you control, then return that card to the battlefield under your control.",
        )
        .with_mana_cost(parse_mtgjson_mana_cost("{W}"))
        .id();
    let mut runner = scenario.build();
    runner.cast(revitalize).resolve();
    assert_eq!(runner.state().players[0].life_gained_this_turn, 3);
    runner.cast(birthday).commit();
    runner.advance_until_stack_empty();
    choose_bearer(&mut runner, P0, other);
    assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&1));

    runner.advance_to_combat();
    runner
        .declare_attackers(&[(frodo, AttackTarget::Player(P1))])
        .unwrap();
    assert_eq!(runner.state().stack.len(), 1);
    let attacking_incarnation = runner.state().objects[&frodo].incarnation;
    runner.cast(cloudshift).target_object(frodo).commit();
    assert_eq!(runner.state().stack.len(), 2);
    runner.resolve_top();
    assert_eq!(runner.state().stack.len(), 1);

    // CR 400.7: Frodo returns as a new object. The engine keeps the id and
    // marks the new object with a new incarnation.
    let returned_frodo = &runner.state().objects[&frodo];
    assert_eq!(returned_frodo.zone, Zone::Battlefield);
    assert_ne!(returned_frodo.incarnation, attacking_incarnation);
    let hand = runner.state().players[0].hand.len();
    runner.advance_until_stack_empty();
    choose_bearer(&mut runner, P0, frodo);
    assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&2));
    assert_eq!(runner.state().ring_bearer.get(&P0), Some(&Some(frodo)));
    assert_eq!(runner.state().players[0].hand.len(), hand);
}

#[test]
fn opponent_count_does_not_satisfy_frodo() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_mana_pool(P0, mana(&[ManaType::Colorless, ManaType::White]));
    scenario.with_library_top(P0, &["Plains", "Plains"]);
    let frodo = add_frodo(&mut scenario);
    scenario.add_creature(P0, "Grizzly Bears", 2, 2);
    let revitalize = add_revitalize(&mut scenario);
    let mut runner = scenario.build();
    runner.state_mut().ring_level.insert(P1, 2);
    runner.state_mut().ring_tempt_count.insert(P1, 2);
    assert!(!runner.state().ring_tempt_count.contains_key(&P0));
    runner.cast(revitalize).resolve();
    assert_eq!(runner.state().players[0].life_gained_this_turn, 3);

    let hand = runner.state().players[0].hand.len();
    runner.advance_to_combat();
    runner
        .declare_attackers(&[(frodo, AttackTarget::Player(P1))])
        .unwrap();
    assert_eq!(runner.state().stack.len(), 1);
    runner.advance_until_stack_empty();
    choose_bearer(&mut runner, P0, frodo);

    // CR 109.5: "You" refers to the controller of Frodo's trigger.
    assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&1));
    assert_eq!(runner.state().ring_tempt_count.get(&P1), Some(&2));
    assert_eq!(runner.state().ring_bearer.get(&P0), Some(&Some(frodo)));
    assert_eq!(runner.state().players[0].hand.len(), hand);
}

#[test]
fn one_ring_with_no_creatures_mills_zero() {
    let mut scenario = GameScenario::new_n_player(3, 42);
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_mana_pool(
        P0,
        mana(&[
            ManaType::Colorless,
            ManaType::Colorless,
            ManaType::Black,
            ManaType::Black,
        ]),
    );
    scenario.with_library_top(P0, &["Swamp", "Swamp"]);
    scenario.with_library_top(P1, &["Forest", "Forest"]);
    scenario.with_library_top(PlayerId(2), &["Island", "Island"]);
    scenario.add_creature(P1, "Grizzly Bears", 2, 2);
    let saga = add_saga(&mut scenario);
    let mut runner = scenario.build();
    let libraries: Vec<_> = runner
        .state()
        .players
        .iter()
        .map(|player| player.library.clone())
        .collect();

    runner.cast(saga).commit();
    runner.advance_until_stack_empty();

    // CR 701.54d: The temptation completes even without a creature to choose.
    assert!(runner.state().stack.is_empty());
    assert!(matches!(
        runner.state().waiting_for,
        WaitingFor::Priority { .. }
    ));
    assert_eq!(runner.state().ring_tempt_count.get(&P0), Some(&1));
    for (player, library) in runner.state().players.iter().zip(libraries) {
        assert_eq!(player.library, library, "{:?}", player.id);
    }
}
