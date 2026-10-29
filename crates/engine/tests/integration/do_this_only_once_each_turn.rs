//! CR 603.2h: "Do this only once each turn" limits the *action*, not the trigger.
//!
//! "A triggered ability may have an instruction followed by 'Do this only once
//! each turn.' This ability triggers only if its source's controller has not yet
//! taken the indicated action that turn." The controller takes the indicated
//! action by accepting the ability's "you may" instruction; declining takes no
//! action, so the ability keeps triggering. Once the action is taken, instances
//! already on the stack resolve with no effect (Calix / Riveteers Ascendancy
//! rulings), and a payment the controller cannot make is never offered
//! (CR 118.3 + CR 608.2d). "This ability triggers only once each turn" is a
//! trigger limit (CR 603.2) and stays spent by triggering.
//!
//! Every fixture drives the real pipeline (cast → trigger → `apply`) and uses the
//! printed Oracle text (verified against MTGJSON).

use std::collections::HashSet;

use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::types::ability::TriggerDefinitionRef;
use engine::types::actions::{CastChoice, GameAction};
use engine::types::counter::CounterType;
use engine::types::game_state::{
    AutoMayChoice, CastOfferKind, CastPaymentMode, MayTriggerAutoChoiceScope, StackEntryKind,
    WaitingFor,
};
use engine::types::identifiers::ObjectId;
use engine::types::keywords::Keyword;
use engine::types::mana::{ManaColor, ManaCost, ManaType, ManaUnit};
use engine::types::phase::Phase;
use engine::types::player::PlayerId;
use engine::types::zones::Zone;

const PANTLAZA: &str = "Whenever Pantlaza or another Dinosaur you control enters, you may discover X, where X is that creature's toughness. Do this only once each turn. (Exile cards from the top of your library until you exile a nonland card with that mana value or less. Cast it without paying its mana cost or put it into your hand. Put the rest on the bottom in a random order.)";

const ACADEMY_WALL: &str = "Defender\nWhenever you cast an instant or sorcery spell, you may draw a card. If you do, discard a card. This ability triggers only once each turn.";

const ANCIENT_CORNUCOPIA: &str = "Whenever you cast a spell that's one or more colors, you may gain 1 life for each of that spell's colors. Do this only once each turn.";

const GRAHA: &str = "Lifelink\nThrow Wide the Gates \u{2014} Whenever you cast a noncreature spell, you may pay X life, where X is that spell's mana value. If you do, create a 1/1 colorless Hero creature token and put X +1/+1 counters on it. Do this only once each turn.";

const ROAMING_THRONE: &str = "Ward {2}\nAs this creature enters, choose a creature type.\nThis creature is the chosen type in addition to its other types.\nIf a triggered ability of another creature you control of the chosen type triggers, it triggers an additional time.";

/// Turn Against (an instant), cast by P1 during P0's turn. Donate's targeted
/// `GiveControl` recipient has no player slot today (see
/// `gain_control_multi_target_6205.rs`), so the control change uses the working
/// `GainControl` path instead.
const TURN_AGAINST: &str = "Devoid (This card has no color.)\nGain control of target creature until end of turn. Untap that creature. It gains haste until end of turn.";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn free() -> ManaCost {
    ManaCost::generic(0)
}

fn add_pantlaza(scenario: &mut GameScenario) -> ObjectId {
    scenario
        .add_creature_from_oracle(P0, "Pantlaza, Sun-Favored", 4, 4, PANTLAZA)
        .with_subtypes(vec!["Dinosaur"])
        .id()
}

fn add_dinosaur_to_hand(scenario: &mut GameScenario, player: PlayerId, name: &str) -> ObjectId {
    scenario
        .add_creature_to_hand(player, name, 2, 2)
        .with_subtypes(vec!["Dinosaur"])
        .with_mana_cost(free())
        .id()
}

/// A mana value 0 nonland card on top of `player`'s library: a discover hit for
/// any X (decouples the tests from the X computation).
fn add_hit(scenario: &mut GameScenario, player: PlayerId, name: &str) -> ObjectId {
    scenario
        .add_spell_to_library_top(player, name, false)
        .with_mana_cost(free())
        .id()
}

fn cast(runner: &mut GameRunner, object_id: ObjectId) {
    let card_id = runner.state().objects[&object_id].card_id;
    runner
        .act(GameAction::CastSpell {
            object_id,
            card_id,
            targets: vec![],
            payment_mode: CastPaymentMode::Auto,
        })
        .expect("cast from hand");
}

/// Pass priority until the cast `spell` has resolved, answering an as-enters
/// creature-type choice with `chosen`. Stops with any triggers it caused on the
/// stack (pattern: `giada_metallic_mimic.rs::cast_and_choose`).
fn resolve_spell(runner: &mut GameRunner, spell: ObjectId, chosen: &str) {
    for _ in 0..12 {
        let on_stack = runner
            .state()
            .stack
            .iter()
            .any(|entry| entry.id == spell || entry.source_id == spell);
        match runner.state().waiting_for.clone() {
            WaitingFor::NamedChoice { .. } => {
                runner
                    .act(GameAction::ChooseOption {
                        choice: chosen.to_string(),
                    })
                    .expect("answer the creature-type choice");
            }
            WaitingFor::Priority { .. } if on_stack => {
                runner.act(GameAction::PassPriority).expect("pass priority");
            }
            _ => return,
        }
    }
    panic!("the spell never resolved");
}

fn cast_and_resolve(runner: &mut GameRunner, spell: ObjectId) {
    cast(runner, spell);
    resolve_spell(runner, spell, "Dinosaur");
}

/// The definition refs of every stacked triggered ability from `source`.
fn stacked_trigger_refs(
    runner: &GameRunner,
    source: ObjectId,
) -> Vec<Option<TriggerDefinitionRef>> {
    runner
        .state()
        .stack
        .iter()
        .filter_map(|entry| match &entry.kind {
            StackEntryKind::TriggeredAbility {
                source_id, ability, ..
            } if *source_id == source => Some(ability.trigger_definition_ref.clone()),
            _ => None,
        })
        .collect()
}

fn assert_optional_prompt(runner: &GameRunner, player: PlayerId, why: &str) {
    assert!(
        matches!(
            runner.state().waiting_for,
            WaitingFor::OptionalEffectChoice { player: p, .. } if p == player
        ),
        "{why}: expected an OptionalEffectChoice for {player:?}, got {:?}",
        runner.state().waiting_for
    );
}

fn assert_settled_priority(runner: &GameRunner, why: &str) {
    assert!(
        matches!(runner.state().waiting_for, WaitingFor::Priority { .. }),
        "{why}: expected Priority, got {:?}",
        runner.state().waiting_for
    );
    assert!(
        runner.state().stack.is_empty(),
        "{why}: the stack must be empty"
    );
}

fn decide(runner: &mut GameRunner, accept: bool) {
    runner
        .act(GameAction::DecideOptionalEffect { accept })
        .expect("answer the optional instruction");
}

/// Answer a discover `CastOffer` for `hit` by declining the cast, which puts the
/// hit into its owner's hand (pattern:
/// `cast_during_resolution_pipeline.rs::discover_decline_sends_hit_to_hand`).
fn decline_discover_cast(runner: &mut GameRunner, hit: ObjectId) {
    assert!(
        matches!(
            runner.state().waiting_for,
            WaitingFor::CastOffer {
                kind: CastOfferKind::Discover { hit_card, .. },
                ..
            } if hit_card == hit
        ),
        "expected a discover CastOffer for {hit:?}, got {:?}",
        runner.state().waiting_for
    );
    runner
        .act(GameAction::DiscoverChoice {
            choice: CastChoice::Decline,
        })
        .expect("decline the discover cast");
    assert_eq!(runner.state().objects[&hit].zone, Zone::Hand);
}

fn indicated_actions(runner: &GameRunner) -> HashSet<(TriggerDefinitionRef, PlayerId)> {
    (*runner.state().trigger_indicated_actions_this_turn).clone()
}

/// Advance real turns until a later turn's precombat main, with P0 holding
/// priority (copied from `avalanche_of_sector_7_activation_trigger.rs`).
fn next_main_with_p0_priority(runner: &mut GameRunner) {
    let turn = runner.state().turn_number;
    for _ in 0..80 {
        if runner.state().turn_number != turn && runner.state().phase == Phase::PreCombatMain {
            while !matches!(runner.state().waiting_for, WaitingFor::Priority { player } if player == P0)
            {
                runner.act(GameAction::PassPriority).expect("pass to P0");
            }
            return;
        }
        let action = match runner.state().waiting_for {
            WaitingFor::Priority { .. } => GameAction::PassPriority,
            WaitingFor::DeclareAttackers { .. } => GameAction::DeclareAttackers {
                attacks: vec![],
                bands: vec![],
            },
            ref other => panic!("unexpected prompt advancing turns: {other:?}"),
        };
        runner.act(action).expect("advance the turn");
    }
    panic!("the next main phase was never reached");
}

fn pool(kinds: &[ManaType]) -> Vec<ManaUnit> {
    kinds
        .iter()
        .map(|&kind| ManaUnit::new(kind, ObjectId(0), false, vec![]))
        .collect()
}

/// Pantlaza on the battlefield, Roaming Throne in hand naming Dinosaur, two
/// discover hits (top first), and Dino A in hand. Throne's own entry is a
/// Dinosaur entry (CR 603.10) doubled by Throne (CR 603.2d); both Pantlaza
/// instances are declined, so the limit is still untaken when this returns with
/// Dino A's two Pantlaza instances on the stack.
struct ThroneBoard {
    runner: GameRunner,
    pantlaza: ObjectId,
    hit_1: ObjectId,
    hit_2: ObjectId,
}

fn throne_board_with_dino_a_doubled() -> ThroneBoard {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    let pantlaza = add_pantlaza(&mut scenario);
    let throne = scenario
        .add_creature_to_hand_from_oracle(P0, "Roaming Throne", 4, 4, ROAMING_THRONE)
        .as_artifact_creature()
        .with_subtypes(vec!["Golem"])
        .with_mana_cost(free())
        .id();
    let dino_a = add_dinosaur_to_hand(&mut scenario, P0, "Dino A");
    let hit_2 = add_hit(&mut scenario, P0, "Hit Two");
    let hit_1 = add_hit(&mut scenario, P0, "Hit One");
    let mut runner = scenario.build();
    runner.state_mut().all_creature_types = vec!["Dinosaur".into(), "Golem".into(), "Human".into()];

    cast_and_resolve(&mut runner, throne);
    assert!(
        runner.state().objects[&throne]
            .card_types
            .subtypes
            .iter()
            .any(|subtype| subtype == "Dinosaur"),
        "reach: Roaming Throne's chosen type is Dinosaur"
    );
    assert_eq!(
        stacked_trigger_refs(&runner, pantlaza).len(),
        2,
        "reach: Throne's own Dinosaur entry triggers Pantlaza twice (CR 603.2d + CR 603.10)"
    );
    for _ in 0..2 {
        runner.advance_until_stack_empty();
        assert_optional_prompt(&runner, P0, "each Throne-entry instance offers its may");
        decide(&mut runner, false);
    }
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "both Throne-entry instances declined");
    assert!(
        indicated_actions(&runner).is_empty(),
        "declining both instances takes no indicated action"
    );

    cast_and_resolve(&mut runner, dino_a);
    assert_eq!(
        stacked_trigger_refs(&runner, pantlaza).len(),
        2,
        "reach: Dino A's entry triggers Pantlaza twice under Roaming Throne"
    );
    ThroneBoard {
        runner,
        pantlaza,
        hit_1,
        hit_2,
    }
}

// ---------------------------------------------------------------------------
// T1 — declining does not spend the limit
// ---------------------------------------------------------------------------

/// CR 603.2h: declining the discover takes no action, so the next Dinosaur's
/// entry triggers Pantlaza again and can discover (acceptance criterion 1).
#[test]
fn pantlaza_declined_discover_leaves_the_next_dinosaur_free_to_discover() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    let pantlaza = add_pantlaza(&mut scenario);
    let dino_a = add_dinosaur_to_hand(&mut scenario, P0, "Dino A");
    let dino_b = add_dinosaur_to_hand(&mut scenario, P0, "Dino B");
    let hit = add_hit(&mut scenario, P0, "Discovered Hit");
    let mut runner = scenario.build();

    cast_and_resolve(&mut runner, dino_a);
    assert_eq!(stacked_trigger_refs(&runner, pantlaza).len(), 1);
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "reach: Dino A triggers Pantlaza");
    decide(&mut runner, false);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after declining Dino A's discover");
    assert!(indicated_actions(&runner).is_empty());
    assert_eq!(runner.state().objects[&hit].zone, Zone::Library);

    cast_and_resolve(&mut runner, dino_b);
    assert_eq!(
        stacked_trigger_refs(&runner, pantlaza).len(),
        1,
        "the declined discover did not spend the limit, so Dino B triggers Pantlaza"
    );
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "Dino B's Pantlaza trigger offers discover");
    decide(&mut runner, true);
    decline_discover_cast(&mut runner, hit);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after Dino B's discover");
}

// ---------------------------------------------------------------------------
// T2 — taking the action spends the limit for the rest of the turn
// ---------------------------------------------------------------------------

/// CR 603.2h: once P0 discovers, Pantlaza stops triggering for the rest of the
/// turn (acceptance criterion 2), and "that turn" resets at the next turn.
#[test]
fn pantlaza_discover_stops_the_trigger_until_the_next_turn() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    let pantlaza = add_pantlaza(&mut scenario);
    let dino_a = add_dinosaur_to_hand(&mut scenario, P0, "Dino A");
    let dino_b = add_dinosaur_to_hand(&mut scenario, P0, "Dino B");
    let dino_c = scenario
        .add_creature_to_hand(P0, "Dino C", 2, 2)
        .with_subtypes(vec!["Dinosaur"])
        .with_keyword(Keyword::Flash)
        .with_mana_cost(free())
        .id();
    add_hit(&mut scenario, P0, "Hit C");
    let hit_b = add_hit(&mut scenario, P0, "Hit B");
    let hit_a = add_hit(&mut scenario, P0, "Hit A");
    scenario.with_library_top(P1, &["P1 Card A", "P1 Card B", "P1 Card C"]);
    let mut runner = scenario.build();

    cast_and_resolve(&mut runner, dino_a);
    let refs = stacked_trigger_refs(&runner, pantlaza);
    let [Some(pantlaza_ref)] = refs.as_slice() else {
        panic!("one Pantlaza trigger with a definition ref, got {refs:?}");
    };
    let pantlaza_ref = pantlaza_ref.clone();
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "reach: Dino A triggers Pantlaza");
    decide(&mut runner, true);
    decline_discover_cast(&mut runner, hit_a);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after Dino A's discover");
    assert_eq!(
        indicated_actions(&runner),
        HashSet::from([(pantlaza_ref.clone(), P0)]),
        "accepting the discover takes the indicated action"
    );

    cast_and_resolve(&mut runner, dino_b);
    assert!(
        stacked_trigger_refs(&runner, pantlaza).is_empty(),
        "Pantlaza no longer triggers this turn"
    );
    assert_settled_priority(&runner, "Dino B's entry triggers nothing");
    assert_eq!(runner.state().objects[&hit_b].zone, Zone::Library);

    // CR 603.2h "that turn": the next turn (P1's) resets the limit; Dino C has flash.
    next_main_with_p0_priority(&mut runner);
    assert!(
        indicated_actions(&runner).is_empty(),
        "the indicated action resets at the next turn"
    );
    cast_and_resolve(&mut runner, dino_c);
    assert_eq!(stacked_trigger_refs(&runner, pantlaza).len(), 1);
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "Pantlaza triggers again on a new turn");
}

// ---------------------------------------------------------------------------
// T3 — "This ability triggers only once each turn" is unchanged
// ---------------------------------------------------------------------------

/// CR 603.2: "This ability triggers only once each turn" is spent by triggering,
/// so declining the first draw still leaves the ability spent (acceptance
/// criterion 3).
#[test]
fn academy_wall_triggers_only_once_each_turn_even_when_declined() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.add_creature_from_oracle(P0, "Academy Wall", 0, 5, ACADEMY_WALL);
    let first = scenario
        .add_spell_to_hand_from_oracle(P0, "Test Instant One", true, "You gain 1 life.")
        .with_mana_cost(free())
        .id();
    let second = scenario
        .add_spell_to_hand_from_oracle(P0, "Test Instant Two", true, "You gain 1 life.")
        .with_mana_cost(free())
        .id();
    scenario.with_library_top(P0, &["Card A", "Card B"]);
    let mut runner = scenario.build();

    cast(&mut runner, first);
    runner.advance_until_stack_empty();
    assert_optional_prompt(
        &runner,
        P0,
        "reach: the first instant triggers Academy Wall",
    );
    decide(&mut runner, false);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after declining the first draw");
    assert_eq!(runner.state().triggers_fired_this_turn.len(), 1);

    cast(&mut runner, second);
    runner.advance_until_stack_empty();
    assert_settled_priority(
        &runner,
        "the declined trigger still spent \"triggers only once each turn\"",
    );
    assert_eq!(runner.state().triggers_fired_this_turn.len(), 1);
    assert!(indicated_actions(&runner).is_empty());
}

// ---------------------------------------------------------------------------
// T4 / T4a / T4b — instances already on the stack (Roaming Throne)
// ---------------------------------------------------------------------------

/// CR 603.2h + CR 603.2d: Roaming Throne doubles Pantlaza's trigger; both
/// instances are on the stack before either resolves. Once the first discovers,
/// the second "resolves but has no effect": no prompt and no second discover.
#[test]
fn roaming_throne_second_pantlaza_instance_does_nothing_after_the_first_discovers() {
    let ThroneBoard {
        mut runner,
        pantlaza,
        hit_1,
        hit_2,
    } = throne_board_with_dino_a_doubled();
    let hand_before = runner.state().players[P0.0 as usize].hand.len();

    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "reach: the first instance offers discover");
    decide(&mut runner, true);
    decline_discover_cast(&mut runner, hit_1);
    runner.advance_until_stack_empty();

    assert_settled_priority(&runner, "the second instance resolves with no prompt");
    assert!(stacked_trigger_refs(&runner, pantlaza).is_empty());
    assert_eq!(
        runner.state().objects[&hit_2].zone,
        Zone::Library,
        "the second instance must not discover again"
    );
    assert_eq!(
        runner.state().players[P0.0 as usize].hand.len(),
        hand_before + 1,
        "only the first discover's hit reaches the hand"
    );
}

/// CR 603.2h: a declined instance takes no action, so the next instance on the
/// stack still offers its "you may" and can discover.
#[test]
fn roaming_throne_declining_the_first_pantlaza_instance_leaves_the_second_free() {
    let ThroneBoard {
        mut runner,
        hit_1,
        hit_2,
        ..
    } = throne_board_with_dino_a_doubled();

    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "reach: the first instance offers discover");
    decide(&mut runner, false);
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "the second instance still offers discover");
    decide(&mut runner, true);
    decline_discover_cast(&mut runner, hit_1);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after the second instance discovers");
    assert_eq!(runner.state().objects[&hit_2].zone, Zone::Library);
}

/// CR 603.2h + CR 603.5: a remembered "always accept" answer for the first
/// instance cannot take the action again for the second — the spent limit is
/// checked before any stored answer is read.
#[test]
fn roaming_throne_remembered_accept_does_not_discover_twice() {
    let ThroneBoard {
        mut runner,
        hit_1,
        hit_2,
        ..
    } = throne_board_with_dino_a_doubled();

    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "reach: the first instance offers discover");
    runner
        .act(GameAction::DecideOptionalEffectAndRemember {
            choice: AutoMayChoice::Accept,
            scope: MayTriggerAutoChoiceScope::ExactInstance,
        })
        .expect("accept and remember");
    decline_discover_cast(&mut runner, hit_1);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "the second instance neither prompts nor discovers");
    assert_eq!(
        runner.state().objects[&hit_2].zone,
        Zone::Library,
        "the remembered accept must not discover a second time"
    );
}

// ---------------------------------------------------------------------------
// T6 — the limit is per ability (two sources)
// ---------------------------------------------------------------------------

/// CR 603.2h: the limit belongs to each ability — with two Ancient Cornucopias
/// each gains life once (Nykthos / Riveteers "each copy" rulings).
#[test]
fn two_ancient_cornucopias_each_take_their_own_action_once() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.add_artifact_from_oracle(P0, "Ancient Cornucopia", ANCIENT_CORNUCOPIA);
    scenario.add_artifact_from_oracle(P0, "Ancient Cornucopia", ANCIENT_CORNUCOPIA);
    let first = scenario
        .add_spell_to_hand_from_oracle(P0, "Red Instant One", true, "Draw a card.")
        .with_mana_cost(free())
        .with_color(vec![ManaColor::Red])
        .id();
    let second = scenario
        .add_spell_to_hand_from_oracle(P0, "Red Instant Two", true, "Draw a card.")
        .with_mana_cost(free())
        .with_color(vec![ManaColor::Red])
        .id();
    scenario.with_library_top(P0, &["Card A", "Card B", "Card C"]);
    let mut runner = scenario.build();
    let life_before = runner.life(P0);

    cast(&mut runner, first);
    for prompt in ["first Cornucopia", "second Cornucopia"] {
        runner.advance_until_stack_empty();
        assert_optional_prompt(&runner, P0, prompt);
        decide(&mut runner, true);
    }
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after both Cornucopias resolve");
    assert_eq!(
        runner.life(P0),
        life_before + 2,
        "each Cornucopia gained 1 life"
    );
    assert_eq!(indicated_actions(&runner).len(), 2);

    cast(&mut runner, second);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "both Cornucopias already took their action");
    assert_eq!(runner.life(P0), life_before + 2);
}

// ---------------------------------------------------------------------------
// T7 — the limit is keyed by the source's controller
// ---------------------------------------------------------------------------

/// CR 603.2h "its source's controller has not yet taken the indicated action":
/// after P0 discovers and P1 then gains control of Pantlaza, P1 has taken no
/// action, so Pantlaza triggers for P1.
#[test]
fn pantlaza_under_a_new_controller_triggers_for_that_controller() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    let pantlaza = add_pantlaza(&mut scenario);
    let dino_a = add_dinosaur_to_hand(&mut scenario, P0, "Dino A");
    let turn_against = scenario
        .add_spell_to_hand_from_oracle(P1, "Turn Against", true, TURN_AGAINST)
        .with_mana_cost(free())
        .from_oracle_text_with_keywords(&["Devoid"], TURN_AGAINST)
        .id();
    let p1_dino = scenario
        .add_creature_to_hand(P1, "P1 Dino", 2, 2)
        .with_subtypes(vec!["Dinosaur"])
        .with_keyword(Keyword::Flash)
        .with_mana_cost(free())
        .id();
    let hit = add_hit(&mut scenario, P0, "Hit");
    add_hit(&mut scenario, P1, "P1 Hit");
    let mut runner = scenario.build();

    cast_and_resolve(&mut runner, dino_a);
    let refs = stacked_trigger_refs(&runner, pantlaza);
    let [Some(pantlaza_ref)] = refs.as_slice() else {
        panic!("one Pantlaza trigger with a definition ref, got {refs:?}");
    };
    let pantlaza_ref = pantlaza_ref.clone();
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "reach: P0's Pantlaza offers discover");
    decide(&mut runner, true);
    decline_discover_cast(&mut runner, hit);
    runner.advance_until_stack_empty();
    assert_eq!(
        indicated_actions(&runner),
        HashSet::from([(pantlaza_ref.clone(), P0)])
    );

    // P1 takes Pantlaza with an instant during P0's turn ("that turn" is unchanged).
    runner.act(GameAction::PassPriority).expect("P0 passes");
    assert!(
        matches!(runner.state().waiting_for, WaitingFor::Priority { player } if player == P1),
        "P1 holds priority: {:?}",
        runner.state().waiting_for
    );
    runner.cast(turn_against).target_object(pantlaza).resolve();
    assert_eq!(
        runner.state().objects[&pantlaza].controller,
        P1,
        "reach: Turn Against gives Pantlaza to P1"
    );
    assert_settled_priority(&runner, "after Turn Against resolves");

    runner.act(GameAction::PassPriority).expect("P0 passes");
    assert!(
        matches!(runner.state().waiting_for, WaitingFor::Priority { player } if player == P1),
        "P1 holds priority: {:?}",
        runner.state().waiting_for
    );
    cast(&mut runner, p1_dino);
    resolve_spell(&mut runner, p1_dino, "Dinosaur");
    assert_eq!(
        stacked_trigger_refs(&runner, pantlaza),
        vec![Some(pantlaza_ref.clone())],
        "P1's Pantlaza trigger is the same ability (no zone change, CR 400.7)"
    );
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P1, "P1 has not taken the action this turn");
    decide(&mut runner, false);
    runner.advance_until_stack_empty();
    assert_eq!(
        indicated_actions(&runner),
        HashSet::from([(pantlaza_ref, P0)]),
        "P1's decline takes no action"
    );
}

// ---------------------------------------------------------------------------
// T8 — an unaffordable optional life payment is not offered and spends nothing
// ---------------------------------------------------------------------------

fn graha_refs(runner: &GameRunner, graha: ObjectId) -> Vec<Option<TriggerDefinitionRef>> {
    stacked_trigger_refs(runner, graha)
}

fn heroes(runner: &GameRunner) -> Vec<ObjectId> {
    let state = runner.state();
    state
        .battlefield
        .iter()
        .copied()
        .filter(|id| state.objects[id].name == "Hero")
        .collect()
}

/// CR 118.3 + CR 119.4 + CR 608.2d: at 2 life, paying 3 life is impossible, so
/// G'raha's "you may pay X life" is not offered and takes no action; the next
/// noncreature spell (X = 1) still triggers and paying spends the limit.
#[test]
fn graha_unaffordable_life_payment_is_not_offered_and_does_not_spend_the_limit() {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    let graha = scenario
        .add_creature_from_oracle(P0, "G'raha Tia, Scion Reborn", 2, 3, GRAHA)
        .id();
    scenario.with_life(P0, 2);
    scenario.with_mana_pool(P0, pool(&[ManaType::Blue; 4]));
    let mv_three = scenario
        .add_spell_to_hand_from_oracle(P0, "Test Draw", false, "Draw a card.")
        .with_mana_cost(ManaCost::generic(3))
        .id();
    let mv_one = scenario
        .add_spell_to_hand_from_oracle(P0, "Test Draw One", false, "Draw a card.")
        .with_mana_cost(ManaCost::generic(1))
        .id();
    let mv_zero = scenario
        .add_spell_to_hand_from_oracle(P0, "Test Draw Zero", false, "Draw a card.")
        .with_mana_cost(free())
        .id();
    scenario.with_library_top(P0, &["Card A", "Card B", "Card C", "Card D"]);
    let mut runner = scenario.build();

    cast(&mut runner, mv_three);
    assert_eq!(
        graha_refs(&runner, graha).len(),
        1,
        "reach: the mana value 3 spell triggers G'raha"
    );
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "paying 3 life at 2 life is never offered");
    assert!(heroes(&runner).is_empty(), "no payment, so no Hero");
    assert_eq!(runner.life(P0), 2);
    assert!(indicated_actions(&runner).is_empty());

    cast(&mut runner, mv_one);
    let refs = graha_refs(&runner, graha);
    let [Some(graha_ref)] = refs.as_slice() else {
        panic!("one G'raha trigger with a definition ref, got {refs:?}");
    };
    let graha_ref = graha_ref.clone();
    runner.advance_until_stack_empty();
    assert_optional_prompt(&runner, P0, "paying 1 life at 2 life is offered");
    decide(&mut runner, true);
    runner.advance_until_stack_empty();
    assert_settled_priority(&runner, "after paying 1 life");
    let heroes = heroes(&runner);
    assert_eq!(heroes.len(), 1, "the paid trigger creates a Hero");
    assert_eq!(
        runner.state().objects[&heroes[0]]
            .counters
            .get(&CounterType::Plus1Plus1)
            .copied()
            .unwrap_or(0),
        1
    );
    assert_eq!(runner.life(P0), 1);
    assert_eq!(indicated_actions(&runner), HashSet::from([(graha_ref, P0)]));

    cast(&mut runner, mv_zero);
    assert!(
        graha_refs(&runner, graha).is_empty(),
        "G'raha took its action this turn, so it no longer triggers"
    );
}
