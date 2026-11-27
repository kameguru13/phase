use engine::game::combat::AttackTarget;
use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::types::actions::GameAction;
use engine::types::game_state::WaitingFor;
use engine::types::mana::{parse_mtgjson_mana_cost, ManaType, ManaUnit};
use engine::types::phase::Phase;
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
    let saga = scenario
        .add_spell_to_hand(P0, SAGA_NAME, false)
        .as_enchantment()
        .with_subtypes(vec!["Saga"])
        .with_mana_cost(parse_mtgjson_mana_cost("{2}{B}{B}"))
        .from_oracle_text(SAGA_ORACLE)
        .id();
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
        let frodo = scenario
            .add_creature(P0, FRODO_NAME, 1, 3)
            .as_legendary()
            .with_subtypes(vec!["Halfling", "Scout"])
            .from_oracle_text_with_keywords(&["Partner", "Partner with", "Vigilance"], FRODO_ORACLE)
            .id();
        let other = scenario.add_creature(P0, "Other candidate", 2, 2).id();
        for _ in 0..5 {
            scenario.add_card_to_library_top(P0, "Plains");
        }
        let revitalize = scenario
            .add_spell_to_hand_from_oracle(P0, "Revitalize", true, "You gain 3 life.\nDraw a card.")
            .with_mana_cost(parse_mtgjson_mana_cost("{1}{W}"))
            .id();
        let birthday = scenario
            .add_spell_to_hand_from_oracle(
                P0,
                "Birthday Escape",
                false,
                "Draw a card. The Ring tempts you.",
            )
            .with_mana_cost(parse_mtgjson_mana_cost("{U}"))
            .id();
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
