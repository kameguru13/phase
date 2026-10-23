//! CR 611.2e: when a later instruction of the same resolution changes a permanent that an
//! earlier instruction put onto the battlefield, the instruction's verb form decides whether
//! enter observers see the change. An "is/has" statement applies simultaneously with the entry,
//! so observers see it; a "becomes"/"gains" (and any other non-"is/has" wording, such as
//! "gets", CR 608.2c) effect applies after the permanent is on the battlefield, so observers
//! read the entrant before it. For a token, the creating instruction's "It has …" defines the
//! token (CR 111.3). Continuous effects from static abilities apply as the permanent enters
//! (CR 611.3c).
//!
//! The parser stamps the onset on the rider's `AbilityDefinition::entry_onset`; the
//! resolution's entry-observation boundary defers only for a remaining `WithEntry` instruction.
//!
//! Built via the `/card-test` recipe: `GameScenario` + `GameRunner::cast(..).resolve()` /
//! `activate(..).resolve()`, on verbatim Oracle text (synthetic hosts are labelled). Every
//! negative assertion is paired with a positive reach guard in the same test.
//!
//! REVERT DISCRIMINATORS:
//! - Restoring the kind classifier in `effects::resolved_chain_may_apply_with_entry`
//!   (`GenericEffect | Animate | ReturnAsAura | ApplyPerpetual` defers whatever its verb) makes
//!   the "gains/gets/becomes" rows fail: Macabre Mockery and Kardur draw 1, Hunting Wilds draws
//!   2, Rampaging Growth gains 1 life, Heroic Reinforcements draws 2.
//! - Dropping the `WithEntry` stamp (`subject::predicate_onset` answering `WhenFollowed`) makes
//!   the "is/has" rows fail: the Admiral Brass rider host draws 0, the copula conjuncts drain no
//!   life, the bare comma conjunct draws 0, and the Reliquary Dragon gets no haste (the real
//!   Admiral Brass trigger and Dragonbroods' Relic rows also fail their parse reach guards). These rows pass on a
//!   whole-change revert by construction: the blanket deferral already deferred every rider.

use engine::game::scenario::{CastOutcome, GameRunner, GameScenario, P0, P1};
use engine::types::ability::{
    AbilityDefinition, ContinuousEffectOnset, Effect, SubAbilityLink, TargetRef,
};
use engine::types::actions::GameAction;
use engine::types::card_type::{CoreType, Supertype};
use engine::types::counter::CounterType;
use engine::types::game_state::{GameState, WaitingFor};
use engine::types::identifiers::ObjectId;
use engine::types::keywords::Keyword;
use engine::types::mana::{ManaType, ManaUnit};
use engine::types::phase::Phase;
use engine::types::zones::Zone;

/// Elemental Bond {2}{G} Enchantment — verbatim Oracle text.
const ELEMENTAL_BOND: &str =
    "Whenever a creature you control with power 3 or greater enters, draw a card.";
/// Soul Warden {W} Creature — Human Cleric 1/1 — verbatim Oracle text.
const SOUL_WARDEN: &str = "Whenever another creature enters, you gain 1 life.";
/// Glorious Anthem {1}{W}{W} Enchantment — verbatim Oracle text.
const GLORIOUS_ANTHEM: &str = "Creatures you control get +1/+1.";
/// Wayward Servant {W}{B} Creature — Zombie 2/2 — verbatim Oracle text.
const WAYWARD_SERVANT: &str =
    "Whenever another Zombie you control enters, each opponent loses 1 life and you gain 1 life.";
/// Dragon Tempest {1}{R} Enchantment — verbatim Oracle text.
const DRAGON_TEMPEST: &str = "Whenever a creature you control with flying enters, it gains haste until end of turn.
Whenever a Dragon you control enters, it deals X damage to any target, where X is the number of Dragons you control.";

/// Macabre Mockery {2}{B}{R} Instant — verbatim Oracle text.
const MACABRE_MOCKERY: &str = "Put target creature card from an opponent's graveyard onto the \
     battlefield under your control. It gets +2/+0 and gains haste until end of turn. Sacrifice \
     it at the beginning of the next end step.";
/// Hunting Wilds {3}{G} Sorcery — verbatim Oracle text.
const HUNTING_WILDS: &str = "Kicker {3}{G} (You may pay an additional {3}{G} as you cast this \
     spell.)\nSearch your library for up to two Forest cards, put them onto the battlefield \
     tapped, then shuffle.\nIf this spell was kicked, untap all Forests put onto the battlefield \
     this way. They become 3/3 green creatures with haste that are still lands.";
/// Rampaging Growth {3}{G} Instant — verbatim Oracle text.
const RAMPAGING_GROWTH: &str = "Search your library for a basic land card, put it onto the \
     battlefield, then shuffle. Until end of turn, that land becomes a 4/3 Insect creature with \
     reach and haste. It's still a land.";
/// Synthetic host: chapter III of Kardur's Vicious Return (verbatim chapter text) as a sorcery.
const KARDUR_CHAPTER_III: &str = "Return target creature card from your graveyard to the \
     battlefield. Put a +1/+1 counter on it. It gains haste until your next turn.";
/// Heroic Reinforcements {2}{R}{W} Sorcery — verbatim Oracle text.
const HEROIC_REINFORCEMENTS: &str = "Create two 1/1 white Soldier creature tokens. Until end of \
     turn, creatures you control get +1/+1 and gain haste. (They can attack and {T} this turn.)";
/// Admiral Brass, Unsinkable {2}{U}{B}{R} Legendary Creature — Human Pirate 3/3 — verbatim
/// Oracle text.
const ADMIRAL_BRASS: &str = "When Admiral Brass enters, mill four cards.\nAt the beginning of \
     combat on your turn, you may return target Pirate creature card from your graveyard to the \
     battlefield with a finality counter on it. It has base power and toughness 4/4. It gains \
     haste until end of turn. (If a creature with a finality counter on it would die, exile it \
     instead.)";
/// Synthetic host: Admiral Brass's return sentence and both verbatim riders, as a sorcery.
const ADMIRAL_BRASS_RIDERS: &str = "Return target creature card from your graveyard to the \
     battlefield. It has base power and toughness 4/4. It gains haste until end of turn.";

/// Dragonbroods' Relic {1}{G} Artifact — verbatim Oracle text.
const DRAGONBROODS_RELIC: &str = "{T}, Tap an untapped creature you control: Add one mana of any \
     color.\n{3}{W}{U}{B}{R}{G}, Sacrifice this artifact: Create a 4/4 Dragon creature token \
     named Reliquary Dragon that's all colors. It has flying, lifelink, and \"When this token \
     enters, it deals 3 damage to any target.\" Activate only as a sorcery.";
/// Synthetic pair of Dragonbroods' Relic: the "It has …" sentence replaced by a "gains" rider.
const DRAGONBROODS_RELIC_GAINS: &str = "{T}, Tap an untapped creature you control: Add one mana \
     of any color.\n{3}{W}{U}{B}{R}{G}, Sacrifice this artifact: Create a 4/4 Dragon creature \
     token named Reliquary Dragon. It gains flying until end of turn. Activate only as a sorcery.";
/// Synthetic: an "is" conjunct after a "gets" conjunct.
const GETS_AND_IS_A_ZOMBIE: &str = "Return target creature card from your graveyard to the \
     battlefield. It gets +1/+0 and is a Zombie in addition to its other types.";
/// Synthetic: a contracted "is" conjunct before a "gains" conjunct.
const IS_A_ZOMBIE_AND_GAINS: &str = "Return target creature card from your graveyard to the \
     battlefield. It's a Zombie in addition to its other types and gains flying.";
/// Synthetic: a bare comma conjunct — "gains …, has …".
const GAINS_COMMA_HAS: &str = "Return target creature card from your graveyard to the \
     battlefield. It gains haste, has base power and toughness 4/4.";

/// A main-phase board with stocked libraries (draws are counted).
fn board() -> GameScenario {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_library_top(P0, &["Card A", "Card B", "Card C", "Card D"]);
    scenario.with_library_top(P1, &["Card E", "Card F", "Card G", "Card H"]);
    scenario
}

fn add_bond(scenario: &mut GameScenario) {
    scenario.add_enchantment_from_oracle(P0, "Elemental Bond", ELEMENTAL_BOND);
}

fn mana(kind: ManaType, n: usize) -> Vec<ManaUnit> {
    vec![ManaUnit::new(kind, ObjectId(0), false, vec![]); n]
}

fn power(state: &GameState, id: ObjectId) -> Option<i32> {
    state.objects.get(&id).and_then(|o| o.power)
}

/// The entry observations recorded for `id` (the power each one read).
fn observed_powers(state: &GameState, id: ObjectId) -> Vec<Option<i32>> {
    state
        .battlefield_entry_observations
        .values()
        .filter(|row| row.identity.reference.object_id == id)
        .map(|row| row.lki.power)
        .collect()
}

/// The onsets of the `GenericEffect` rider nodes of `def`'s chain, in `sub_ability` order.
fn rider_onsets(def: &AbilityDefinition) -> Vec<ContinuousEffectOnset> {
    let mut out = Vec::new();
    let mut node = Some(def);
    while let Some(current) = node {
        if matches!(*current.effect, Effect::GenericEffect { .. }) {
            out.push(current.entry_onset);
        }
        node = current.sub_ability.as_deref();
    }
    out
}

fn reanimate(
    observer: impl FnOnce(&mut GameScenario),
    text: &str,
    owner: engine::types::player::PlayerId,
    power: i32,
    toughness: i32,
) -> (CastOutcome, ObjectId) {
    let mut scenario = board();
    observer(&mut scenario);
    let returned = scenario
        .add_creature_to_graveyard(owner, "Returned Creature", power, toughness)
        .id();
    let spell = scenario
        .add_spell_to_hand_from_oracle(P0, "Rider Spell", true, text)
        .id();
    let mut runner = scenario.build();
    let outcome = runner.cast(spell).target_object(returned).resolve();
    (outcome, returned)
}

/// V1 — CR 611.2e + CR 608.2c: Macabre Mockery's "gets +2/+0 and gains haste" applies after the
/// opponent's 1/2 is on the battlefield, so Elemental Bond reads a 1-power entrant and does not
/// trigger, although the creature is 3/2 when the spell finishes.
#[test]
fn v1_macabre_mockery_gets_gains_rider_is_not_observed() {
    let (outcome, returned) = reanimate(add_bond, MACABRE_MOCKERY, P1, 1, 2);
    assert_eq!(outcome.zone_of(returned), Zone::Battlefield);
    assert_eq!(outcome.controller(returned), P0);
    assert_eq!(
        power(outcome.state(), returned),
        Some(3),
        "reach: the rider applied"
    );
    assert_eq!(
        observed_powers(outcome.state(), returned),
        vec![Some(1)],
        "the boundary before the rider observed the entrant as it entered"
    );
    outcome.assert_hand_drawn(P0, 0);
}

/// V1 pair — CR 603.10: a 3/3 enters with power 3, so Bond draws.
#[test]
fn v1_pair_three_power_entrant_satisfies_bond() {
    let (outcome, returned) = reanimate(add_bond, MACABRE_MOCKERY, P1, 3, 3);
    assert_eq!(power(outcome.state(), returned), Some(5));
    outcome.assert_hand_drawn(P0, 1);
}

fn hunting_wilds(kicked: bool) -> (CastOutcome, Vec<ObjectId>) {
    let mut scenario = board();
    add_bond(&mut scenario);
    let forests: Vec<ObjectId> = (0..2)
        .map(|_| {
            scenario
                .add_land_to_library_top(P0, "Forest")
                .with_subtypes(vec!["Forest"])
                .id()
        })
        .collect();
    let spell = scenario
        .add_spell_to_hand_from_oracle(P0, "Hunting Wilds", false, HUNTING_WILDS)
        .id();
    scenario.with_mana_pool(P0, mana(ManaType::Green, 8));
    let mut runner = scenario.build();
    let cast = runner.cast(spell);
    let cast = if kicked {
        cast.accept_optional()
    } else {
        cast.decline_optional()
    };
    let outcome = cast.search_first_legal().resolve();
    (outcome, forests)
}

/// V2 — CR 611.2e: kicked Hunting Wilds' Forests "become 3/3 green creatures" after they are on
/// the battlefield, so Bond sees two lands and draws nothing.
#[test]
fn v2_hunting_wilds_kicked_become_rider_is_not_observed() {
    let (outcome, forests) = hunting_wilds(true);
    for forest in &forests {
        assert_eq!(outcome.zone_of(*forest), Zone::Battlefield);
        let object = &outcome.state().objects[forest];
        assert!(
            object.card_types.core_types.contains(&CoreType::Creature),
            "reach: the Forest became a creature"
        );
        assert_eq!(object.power, Some(3));
    }
    assert_eq!(
        forests
            .iter()
            .map(|forest| observed_powers(outcome.state(), *forest).len())
            .sum::<usize>(),
        2,
        "both entries were observed before the rider"
    );
    outcome.assert_hand_drawn(P0, 0);
}

/// V2 sibling — unkicked: the entrants are lands at entry, so Bond draws nothing. (The
/// "become" rider's kicker gate is a separate defect; only the observation is asserted.)
#[test]
fn v2_hunting_wilds_unkicked_draws_nothing() {
    let (outcome, forests) = hunting_wilds(false);
    for forest in &forests {
        assert_eq!(outcome.zone_of(*forest), Zone::Battlefield, "reach");
    }
    outcome.assert_hand_drawn(P0, 0);
}

/// V3 — CR 611.2e + CR 205.1b: Rampaging Growth's land "becomes a 4/3 Insect creature" after it
/// enters ("It's still a land" qualifies that same effect), so Soul Warden sees a land enter and
/// gains no life.
#[test]
fn v3_rampaging_growth_becomes_rider_is_not_observed() {
    let mut scenario = board();
    scenario.add_creature_from_oracle(P0, "Soul Warden", 1, 1, SOUL_WARDEN);
    let forest = scenario
        .add_land_to_library_top(P0, "Forest")
        .with_subtypes(vec!["Forest"])
        .id();
    let spell = scenario
        .add_spell_to_hand_from_oracle(P0, "Rampaging Growth", true, RAMPAGING_GROWTH)
        .id();
    let mut runner = scenario.build();
    {
        let object = runner.state_mut().objects.get_mut(&forest).unwrap();
        object.card_types.supertypes.push(Supertype::Basic);
        object.base_card_types.supertypes.push(Supertype::Basic);
    }
    let outcome = runner.cast(spell).search_first_legal().resolve();
    let object = &outcome.state().objects[&forest];
    assert_eq!(object.zone, Zone::Battlefield);
    assert!(object.card_types.core_types.contains(&CoreType::Land));
    assert!(
        object.card_types.core_types.contains(&CoreType::Creature),
        "reach: the land became a creature"
    );
    assert_eq!((object.power, object.toughness), (Some(4), Some(3)));
    assert_eq!(
        observed_powers(outcome.state(), forest),
        vec![None],
        "observed as the land it entered as"
    );
    outcome.assert_life_delta(P0, 0);
}

/// V4 — CR 608.2c + CR 611.2e (synthetic host: Kardur's Vicious Return chapter III as a
/// sorcery): the +1/+1 counter and the "gains haste" rider are both later instructions, so Bond
/// reads the 2/2 entrant and draws nothing.
#[test]
fn v4_kardur_counter_then_gains_rider_is_not_observed() {
    let (outcome, returned) = reanimate(add_bond, KARDUR_CHAPTER_III, P0, 2, 2);
    assert_eq!(outcome.zone_of(returned), Zone::Battlefield);
    assert_eq!(outcome.counters(returned, CounterType::Plus1Plus1), 1);
    assert_eq!(power(outcome.state(), returned), Some(3), "reach");
    assert!(outcome.state().objects[&returned].has_keyword(&Keyword::Haste));
    assert_eq!(observed_powers(outcome.state(), returned), vec![Some(2)]);
    outcome.assert_hand_drawn(P0, 0);
}

fn heroic_reinforcements(anthems: usize) -> CastOutcome {
    let mut scenario = board();
    add_bond(&mut scenario);
    for _ in 0..anthems {
        scenario.add_enchantment_from_oracle(P0, "Glorious Anthem", GLORIOUS_ANTHEM);
    }
    let spell = scenario
        .add_spell_to_hand_from_oracle(P0, "Heroic Reinforcements", false, HEROIC_REINFORCEMENTS)
        .id();
    let mut runner = scenario.build();
    runner.cast(spell).resolve()
}

fn soldier_powers(outcome: &CastOutcome) -> Vec<Option<i32>> {
    outcome
        .state()
        .battlefield
        .iter()
        .filter(|id| outcome.state().objects[id].is_token)
        .map(|id| outcome.state().objects[id].power)
        .collect()
}

/// V5 — CR 611.2e + CR 611.3c: the Soldier tokens enter as 2/2 under Glorious Anthem; the
/// spell's "creatures you control get +1/+1" applies after they are on the battlefield, so Bond
/// draws nothing although they are 3/3 when the spell finishes.
#[test]
fn v5_heroic_reinforcements_token_entrants_global_get_is_not_observed() {
    let outcome = heroic_reinforcements(1);
    assert_eq!(soldier_powers(&outcome), vec![Some(3), Some(3)], "reach");
    outcome.assert_hand_drawn(P0, 0);
}

/// V5 pair — CR 611.3c: under two Anthems the tokens are 3/3 as they enter, so Bond draws twice.
#[test]
fn v5_pair_two_anthems_satisfy_bond_at_entry() {
    let outcome = heroic_reinforcements(2);
    assert_eq!(soldier_powers(&outcome), vec![Some(4), Some(4)]);
    outcome.assert_hand_drawn(P0, 2);
}

/// V6 — CR 611.2e: Admiral Brass's "It has base power and toughness 4/4" applies as the Pirate
/// enters, so Bond sees a 4/4 and draws; the trailing "It gains haste" is a later rider. Driven
/// through the real beginning-of-combat trigger. This path records no entry observation (the
/// observer reads the live entrant after the resolution), so it pins the parse and the outcome;
/// the runtime deferral is discriminated by the sorcery-host row below.
#[test]
fn v6_admiral_brass_has_rider_is_observed() {
    let mut scenario = board();
    add_bond(&mut scenario);
    let admiral = scenario
        .add_creature_from_oracle(P0, "Admiral Brass, Unsinkable", 3, 3, ADMIRAL_BRASS)
        .with_subtypes(vec!["Human", "Pirate"])
        .id();
    let pirate = scenario
        .add_creature_to_graveyard(P0, "Pirate Deckhand", 2, 2)
        .with_subtypes(vec!["Human", "Pirate"])
        .id();
    let mut runner = scenario.build();

    // Parse reach guard: the trigger's chain is return → "has" (WithEntry) → "gains"
    // (WhenFollowed).
    let execute = runner.state().objects[&admiral]
        .trigger_definitions
        .as_slice()
        .iter()
        .find_map(|trigger| {
            let execute = trigger.definition.execute.as_deref()?;
            matches!(*execute.effect, Effect::ChangeZone { .. }).then_some(execute)
        })
        .expect("the beginning-of-combat return trigger");
    assert_eq!(
        rider_onsets(execute),
        vec![
            ContinuousEffectOnset::WithEntry,
            ContinuousEffectOnset::WhenFollowed
        ],
        "{execute:#?}"
    );

    let hand_before = runner.state().players[0].hand.len();
    runner.advance_to_phase(Phase::BeginCombat);
    drive_trigger(&mut runner, pirate);
    let state = runner.state();
    let object = &state.objects[&pirate];
    assert_eq!(
        object.zone,
        Zone::Battlefield,
        "reach: the trigger resolved"
    );
    assert_eq!((object.power, object.toughness), (Some(4), Some(4)));
    assert_eq!(
        object.counters.get(&CounterType::Finality).copied(),
        Some(1)
    );
    assert!(object.has_keyword(&Keyword::Haste));
    assert_eq!(
        state.players[0].hand.len() - hand_before,
        1,
        "Bond saw the 4/4 entrant"
    );
}

/// V6 (synthetic host: Admiral Brass's return text as a sorcery, without the finality
/// counter) — CR 611.2e: the "has" rider defers the observation, so Bond reads the 4/4 and
/// draws, while the trailing "gains haste" rider does not hold it back on its own.
#[test]
fn v6_has_rider_defers_the_observation() {
    let (outcome, returned) = reanimate(add_bond, ADMIRAL_BRASS_RIDERS, P0, 2, 2);
    let object = &outcome.state().objects[&returned];
    assert_eq!(object.zone, Zone::Battlefield);
    assert_eq!(
        (object.power, object.toughness),
        (Some(4), Some(4)),
        "reach"
    );
    assert!(object.has_keyword(&Keyword::Haste));
    outcome.assert_hand_drawn(P0, 1);
}

/// Answer the beginning-of-combat trigger's prompts until it has resolved.
fn drive_trigger(runner: &mut GameRunner, target: ObjectId) {
    for _ in 0..20 {
        let action = match &runner.state().waiting_for {
            WaitingFor::Priority { .. } if runner.state().stack.is_empty() => return,
            WaitingFor::Priority { .. } => GameAction::PassPriority,
            WaitingFor::TriggerTargetSelection { .. } => GameAction::SelectTargets {
                targets: vec![TargetRef::Object(target)],
            },
            WaitingFor::OptionalEffectChoice { .. } => {
                GameAction::DecideOptionalEffect { accept: true }
            }
            WaitingFor::OrderTriggers { triggers, .. } => GameAction::OrderTriggers {
                order: (0..triggers.len()).collect(),
            },
            other => panic!("unexpected prompt {other:?}"),
        };
        runner.act(action).expect("legal answer");
    }
    panic!(
        "the trigger did not resolve: {:?}",
        runner.state().waiting_for
    );
}

fn reliquary_dragon(text: &str) -> (GameState, ObjectId, ObjectId) {
    let mut scenario = board();
    scenario.add_enchantment_from_oracle(P0, "Dragon Tempest", DRAGON_TEMPEST);
    let relic = scenario
        .add_artifact_from_oracle(P0, "Dragonbroods' Relic", text)
        .id();
    let mut pool = Vec::new();
    for kind in [
        ManaType::White,
        ManaType::Blue,
        ManaType::Black,
        ManaType::Red,
        ManaType::Green,
    ] {
        pool.extend(mana(kind, 1));
    }
    pool.extend(mana(ManaType::Colorless, 3));
    scenario.with_mana_pool(P0, pool);
    let mut runner = scenario.build();
    let outcome = runner.activate(relic, 1).target_player(P1).resolve();
    let state = outcome.state().clone();
    let token = state
        .battlefield
        .iter()
        .copied()
        .find(|id| state.objects[id].is_token)
        .expect("the Reliquary Dragon token");
    (state, relic, token)
}

/// V8 — CR 111.3 + CR 611.2e: Dragonbroods' Relic's "It has flying, lifelink, and …" defines
/// the token, so it has flying as it enters and Dragon Tempest gives it haste.
#[test]
fn v8_token_defining_has_rider_is_observed() {
    let mut scenario = board();
    let relic = scenario
        .add_artifact_from_oracle(P0, "Dragonbroods' Relic", DRAGONBROODS_RELIC)
        .id();
    let runner = scenario.build();
    let ability = &runner.state().objects[&relic].abilities[1];
    assert!(
        matches!(*ability.effect, Effect::Token { .. }),
        "{ability:#?}"
    );
    let sibling = ability
        .sub_ability
        .as_deref()
        .expect("the It has sibling survives as a chain node");
    assert!(matches!(*sibling.effect, Effect::GenericEffect { .. }));
    assert_eq!(sibling.sub_link, SubAbilityLink::SequentialSibling);
    assert_eq!(sibling.entry_onset, ContinuousEffectOnset::WithEntry);

    let (state, relic, token) = reliquary_dragon(DRAGONBROODS_RELIC);
    assert_eq!(
        state.objects[&relic].zone,
        Zone::Graveyard,
        "reach: activated"
    );
    let dragon = &state.objects[&token];
    assert!(dragon.has_keyword(&Keyword::Flying));
    assert!(dragon.has_keyword(&Keyword::Lifelink));
    assert!(
        dragon.has_keyword(&Keyword::Haste),
        "Dragon Tempest saw a flying entrant"
    );
}

/// V8 pair (synthetic) — CR 611.2e: "It gains flying until end of turn" applies after the token
/// is on the battlefield, so Dragon Tempest does not see a flying entrant.
#[test]
fn v8_pair_gains_flying_rider_is_not_observed() {
    let (state, relic, token) = reliquary_dragon(DRAGONBROODS_RELIC_GAINS);
    assert_eq!(state.objects[&relic].zone, Zone::Graveyard);
    let dragon = &state.objects[&token];
    assert!(
        dragon.has_keyword(&Keyword::Flying),
        "reach: the rider applied"
    );
    assert!(!dragon.has_keyword(&Keyword::Haste));
}

fn servant_return(text: &str) -> (CastOutcome, ObjectId) {
    reanimate(
        |scenario| {
            scenario
                .add_creature_from_oracle(P0, "Wayward Servant", 2, 2, WAYWARD_SERVANT)
                .with_subtypes(vec!["Zombie"]);
        },
        text,
        P0,
        2,
        2,
    )
}

/// V9 — CR 611.2e: a clause with an "is" conjunct applies with the entry whichever conjunct
/// comes first, so Wayward Servant sees a Zombie enter.
#[test]
fn v9_copula_conjunct_is_observed() {
    for text in [GETS_AND_IS_A_ZOMBIE, IS_A_ZOMBIE_AND_GAINS] {
        let (outcome, returned) = servant_return(text);
        let object = &outcome.state().objects[&returned];
        assert_eq!(object.zone, Zone::Battlefield, "{text:?}");
        assert!(
            object.card_types.subtypes.iter().any(|s| s == "Zombie"),
            "reach: {text:?}"
        );
        outcome.assert_life_delta(P1, -1);
        outcome.assert_life_delta(P0, 1);
    }
}

/// V9 sibling — CR 611.2e: the bare comma conjunct "gains haste, has base power and toughness
/// 4/4" carries a "has" statement, so Bond sees a 4/4 enter.
#[test]
fn v9_bare_comma_has_conjunct_is_observed() {
    let (outcome, returned) = reanimate(add_bond, GAINS_COMMA_HAS, P0, 2, 2);
    let object = &outcome.state().objects[&returned];
    assert_eq!(object.zone, Zone::Battlefield);
    assert_eq!(
        (object.power, object.toughness),
        (Some(4), Some(4)),
        "reach"
    );
    outcome.assert_hand_drawn(P0, 1);
}
