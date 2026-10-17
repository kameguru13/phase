//! CR 602.2b + CR 601.2h + CR 608.2h: an activated ability whose cost removes
//! counters reads the number of counters actually removed while paying that
//! cost ("… counters removed this way", "for each counter removed", and, for a
//! remove-all cost, "that much"). The count is bound to the one activation that
//! paid it, on both the stack route and the mana route (CR 605.3b): the
//! activation's own payment owns the count while it is in flight, so two
//! activations of one source in flight at once (CR 601.2g + CR 605.3a) each
//! read their own.
//!
//! CR 707.10: a copy of such an ability copies all decisions made for it,
//! including the value of X, and isn't activated, so the copy reads the original's
//! removed-counter count (the official copier rulings, e.g. Abstruse Archaic:
//! "If the activated ability's cost contains a choice, such as a creature to
//! sacrifice or a number of counters to remove, the copy uses that same
//! information. You can't pay the cost again, even if you want to.").
//!
//! CR 107.1: "for each N <things>" counts complete groups of N (Sage of Hours;
//! CR 500.7 extra turns). A reference that cannot express the grouping fails
//! honestly instead of reading the bare count.
//!
//! CR 107.1c: an "any number" cost lets the player choose any number, including
//! zero.

use std::collections::VecDeque;
use std::sync::Arc;

use engine::game::casting::ActivationCostGuardSite;
use engine::game::effects::attach::attach_to;
use engine::game::perf_counters;
use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::types::ability::{
    AbilityCost, AbilityDefinition, AggregateFunction, DamageChannel, Effect, QuantityExpr,
    QuantityRef, ReplacementDefinition, ReplacementMode, RoundingMode, TargetFilter, TargetRef,
};
use engine::types::actions::GameAction;
use engine::types::counter::{parse_counter_type, CounterType};
use engine::types::game_state::{
    CastPaymentMode, CounterCostChoice, GameState, ManaAbilityCostCursor, ManaChoice,
    ManaChoiceContext, PausedCounterCostRemoval, PayCostKind, PendingCast,
    PendingCostMoveCompletion, PendingCostMoveResume, ReplacementChoiceKind, StackEntryKind,
    WaitingFor,
};
use engine::types::identifiers::ObjectId;
use engine::types::mana::{ManaCost, ManaCostShard, ManaType, ManaUnit};
use engine::types::phase::Phase;
use engine::types::proposed_event::{ProposedEvent, ReplacementId};
use engine::types::replacements::ReplacementEvent;
use engine::types::zones::Zone;

const ESSENCE_BOTTLE: &str = "{3}, {T}: Put an elixir counter on this artifact.\n{T}, Remove all elixir counters from this artifact: You gain 2 life for each elixir counter removed this way.";
const MOLTEN_HYDRA: &str = "{1}{R}{R}: Put a +1/+1 counter on this creature.\n{T}, Remove all +1/+1 counters from this creature: It deals damage to any target equal to the number of +1/+1 counters removed this way.";
const RELIC_AMULET: &str = "Whenever you cast an instant, sorcery, or Wizard spell, put a charge counter on this artifact.\n{2}, {T}, Remove all charge counters from this artifact: It deals that much damage to target creature.";
const BLADEMANE_BAKU: &str = "Whenever you cast a Spirit or Arcane spell, you may put a ki counter on this creature.\n{1}, Remove X ki counters from this creature: For each counter removed, this creature gets +2/+0 until end of turn.";
const GEOMETRIC_NEXUS: &str = "Whenever a player casts an instant or sorcery spell, put a number of charge counters on this artifact equal to that spell's mana value.\n{6}, {T}, Remove all charge counters from this artifact: Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it, where X is the number of charge counters removed this way.";
const VISH_KAL: &str = "Flying, lifelink\nSacrifice a creature: Put X +1/+1 counters on Vish Kal, where X is the sacrificed creature's power.\nRemove all +1/+1 counters from Vish Kal: Target creature gets -1/-1 until end of turn for each +1/+1 counter removed this way.";
const TORTURE_CHAMBER: &str = "At the beginning of your upkeep, put a pain counter on this artifact.\nAt the beginning of your end step, this artifact deals damage to you equal to the number of pain counters on it.\n{1}, {T}, Remove all pain counters from this artifact: It deals damage to target creature equal to the number of pain counters removed this way.";
const JAR_OF_EYEBALLS: &str = "Whenever a creature you control dies, put two eyeball counters on this artifact.\n{3}, {T}, Remove all eyeball counters from this artifact: Look at the top X cards of your library, where X is the number of eyeball counters removed this way. Put one of them into your hand and the rest on the bottom of your library in any order.";
const OOZE_FLUX: &str = "{1}{G}, Remove one or more +1/+1 counters from among creatures you control: Create an X/X green Ooze creature token, where X is the number of +1/+1 counters removed this way.";
const CATTI_BRIE: &str = "Reach, first strike\nWhenever Catti-brie attacks, put a +1/+1 counter on it for each Equipment attached to it.\n{1}, Remove all +1/+1 counters from Catti-brie: It deals X damage to target attacking or blocking creature an opponent controls, where X is the number of counters removed this way.";
const SAGE_OF_HOURS: &str = "Heroic — Whenever you cast a spell that targets this creature, put a +1/+1 counter on it.\nRemove all +1/+1 counters from this creature: For each five counters removed this way, take an extra turn after this one.";
const SAPRAZZAN_COVE: &str = "This land enters tapped.\n{T}: Put a storage counter on this land.\n{T}, Remove any number of storage counters from this land: Add {U} for each storage counter removed this way.";
const BLACK_MANA_BATTERY: &str = "{2}, {T}: Put a charge counter on this artifact.\n{T}, Remove any number of charge counters from this artifact: Add {B}, then add an additional {B} for each charge counter removed this way.";
const RASPUTIN: &str = "When Rasputin enters, put a dream counter on it for each opponent you have. Each opponent creates a 1/1 red Goblin creature token.\n{T}, Remove one or more dream counters from Rasputin: Add that much {C}.\n{T}, Remove a dream counter from Rasputin: Create a 2/2 white Knight creature token with protection from red.";
const GEISTFLAME_RESERVOIR: &str = "Whenever you cast an instant or sorcery spell, put a charge counter on this artifact.\n{1}{R}, {T}, Remove any number of charge counters from this artifact: It deals that much damage to any target.\n{1}{R}, {T}: Exile the top card of your library. You may play that card this turn.";
const ANT_MAN: &str = "Whenever you draw a card, put a +1/+1 counter on The Astonishing Ant-Man.\n{2}{G}, {T}, Remove any number of +1/+1 counters from The Astonishing Ant-Man: Create that many 1/1 green Insect creature tokens.";
const RINGS_OF_BRIGHTHEARTH: &str = "Whenever you activate an ability, if it isn't a mana ability, you may pay {2}. If you do, copy that ability. You may choose new targets for the copy.";
const ILLUSIONISTS_BRACERS: &str = "Whenever an ability of equipped creature is activated, if it isn't a mana ability, copy that ability. You may choose new targets for the copy.\nEquip {3}";
const OPALESCENCE: &str = "Each other non-Aura enchantment is a creature in addition to its other types and has base power and base toughness each equal to its mana value.";
const CRYPTOLITH_RITE: &str = "Creatures you control have \"{T}: Add one mana of any color.\"";
const EXPERIMENT_KRAJ: &str = "Experiment Kraj has all activated abilities of each other creature with a +1/+1 counter on it.\n{T}: Put a +1/+1 counter on target creature.";
const MARCH_OF_THE_MACHINES: &str = "Each noncreature artifact is an artifact creature with power and toughness each equal to its mana value. (Equipment that's a creature can't equip a creature.)";
const PENTAD_PRISM: &str = "Sunburst (This artifact enters with a charge counter on it for each color of mana spent to cast it.)\nRemove a charge counter from this artifact: Add one mana of any color.";
const MAGOSI: &str = "This land enters tapped.\n{T}: Add {U}.\n{U}, {T}: Put an eon counter on this land. Skip your next turn.\n{T}, Remove an eon counter from this land and return it to its owner's hand: Take an extra turn after this one.";
const EMBER_SHOT: &str = "Ember Shot deals 3 damage to any target.\nDraw a card.";
/// C5a: the effect-path control — counters removed by an EFFECT, not a cost.
const EFFECT_PATH_CONTROL: &str = "{T}: Remove all charge counters from this artifact. You gain 1 life for each charge counter removed this way.";
/// C7d: a reader of X itself (not of the counters removed).
const X_READER: &str =
    "{1}, Remove X ki counters from this creature: This creature gets +X/+0 until end of turn.";

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn main_phase() -> GameScenario {
    let mut scenario = GameScenario::new();
    scenario.at_phase(Phase::PreCombatMain);
    scenario.with_library_top(
        P0,
        &["Forest", "Island", "Swamp", "Mountain", "Plains", "Forest"],
    );
    scenario
}

fn counter(name: &str) -> CounterType {
    parse_counter_type(name)
}

fn add_mana(scenario: &mut GameScenario, mana: &[ManaType]) {
    scenario.with_mana_pool(
        P0,
        mana.iter()
            .map(|color| ManaUnit::new(*color, ObjectId(0), false, vec![]))
            .collect(),
    );
}

fn colorless(n: usize) -> Vec<ManaType> {
    vec![ManaType::Colorless; n]
}

/// A P1 creature that survives the damage these rows deal.
fn add_victim(scenario: &mut GameScenario, name: &str, power: i32, toughness: i32) -> ObjectId {
    scenario.add_creature(P1, name, power, toughness).id()
}

fn counters_on(runner: &GameRunner, id: ObjectId, kind: &CounterType) -> u32 {
    runner.state().objects[&id]
        .counters
        .get(kind)
        .copied()
        .unwrap_or(0)
}

fn damage(runner: &GameRunner, id: ObjectId) -> u32 {
    runner.state().objects[&id].damage_marked
}

fn power(runner: &GameRunner, id: ObjectId) -> i32 {
    runner.state().objects[&id]
        .power
        .expect("a creature has power")
}

fn toughness(runner: &GameRunner, id: ObjectId) -> i32 {
    runner.state().objects[&id]
        .toughness
        .expect("a creature has toughness")
}

fn pool(runner: &GameRunner, color: ManaType) -> usize {
    runner.state().players[0].mana_pool.count_color(color)
}

/// CR 602.2b: the count lives only with the payment that removed the counters,
/// so once that activation is placed or has produced its mana no payment state
/// is left to hold it.
fn payment_ownership_is_clear(runner: &GameRunner) -> bool {
    let state = runner.state();
    state.pending_cast.is_none()
        && state.pending_cost_move_resume.is_none()
        && state.pending_replacement.is_none()
        && !matches!(state.waiting_for, WaitingFor::ChooseManaColor { .. })
}

/// The removed-counter count the paused activation's own payment holds.
fn pending_activation_count(runner: &GameRunner) -> Option<u32> {
    runner
        .state()
        .pending_cast
        .as_ref()
        .expect("an activation is paused for its mana payment")
        .ability
        .context
        .cost_removed_counter_amount
}

/// The removed-counter count bound to the top stack entry's activated ability.
fn top_stack_count(runner: &GameRunner) -> Option<u32> {
    match &runner
        .state()
        .stack
        .last()
        .expect("an ability waits on the stack")
        .kind
    {
        StackEntryKind::ActivatedAbility { ability, .. } => {
            ability.context.cost_removed_counter_amount
        }
        other => panic!("the top of the stack is an activated ability: {other:?}"),
    }
}

fn is_mana_payment(state: &GameState) -> bool {
    matches!(state.waiting_for, WaitingFor::ManaPayment { .. })
}

fn cost_removes_counters(cost: &AbilityCost) -> bool {
    match cost {
        AbilityCost::RemoveCounter { .. } => true,
        AbilityCost::Composite { costs } => costs.iter().any(cost_removes_counters),
        _ => false,
    }
}

/// Index of the `nth` activated ability of `id` whose cost removes counters.
fn counter_cost_ability(runner: &GameRunner, id: ObjectId, nth: usize) -> usize {
    runner.state().objects[&id]
        .abilities
        .iter()
        .enumerate()
        .filter(|(_, def)| def.cost.as_ref().is_some_and(cost_removes_counters))
        .map(|(index, _)| index)
        .nth(nth)
        .expect("the card has a counter-cost activated ability")
}

/// Index of the activated mana ability of `id` whose cost removes no counters.
fn mana_ability_without_counter_cost(runner: &GameRunner, id: ObjectId) -> usize {
    runner.state().objects[&id]
        .abilities
        .iter()
        .position(|def| {
            matches!(&*def.effect, Effect::Mana { .. })
                && !def.cost.as_ref().is_some_and(cost_removes_counters)
        })
        .expect("the object has a mana ability without a counter cost")
}

fn battlefield_named(runner: &GameRunner, name: &str) -> Vec<ObjectId> {
    let state = runner.state();
    state
        .battlefield
        .iter()
        .copied()
        .filter(|id| state.objects.get(id).is_some_and(|obj| obj.name == name))
        .collect()
}

/// Answers for the prompts a row may reach.
#[derive(Default)]
struct Answers {
    x: Option<u32>,
    pay_amount: Option<u32>,
    targets: VecDeque<TargetRef>,
    distribution: Option<Vec<CounterCostChoice>>,
    /// Place the trigger from this source last (on top) at an ordering prompt.
    copier_last: Option<ObjectId>,
    /// Cards a `DigChoice` offered, recorded for the Jar of Eyeballs row.
    dig_offered: Option<usize>,
    /// The colour chosen at a `ChooseManaColor` prompt.
    mana_color: Option<ManaType>,
    /// The option picked at each `ReplacementChoice` prompt, in order.
    replacement_choices: VecDeque<usize>,
    /// The permanents returned at a `PayCost { kind: ReturnToHand }` prompt.
    return_choice: Vec<ObjectId>,
}

impl Answers {
    fn x(value: u32) -> Self {
        Self {
            x: Some(value),
            ..Self::default()
        }
    }

    fn pay(amount: u32) -> Self {
        Self {
            pay_amount: Some(amount),
            ..Self::default()
        }
    }

    fn target(mut self, target: TargetRef) -> Self {
        self.targets.push_back(target);
        self
    }

    fn object(self, id: ObjectId) -> Self {
        self.target(TargetRef::Object(id))
    }
}

/// Answers every prompt from `answers` until `stop` holds.
fn drive(runner: &mut GameRunner, stop: impl Fn(&GameState) -> bool, answers: &mut Answers) {
    for _ in 0..96 {
        if stop(runner.state()) {
            return;
        }
        let action = match &runner.state().waiting_for {
            WaitingFor::Priority { .. } | WaitingFor::ManaPayment { .. } => {
                GameAction::PassPriority
            }
            WaitingFor::ChooseXValue { .. } => GameAction::ChooseX {
                value: answers.x.expect("an X answer for this row"),
            },
            WaitingFor::PayAmountChoice { .. } => GameAction::SubmitPayAmount {
                amount: answers
                    .pay_amount
                    .expect("a pay-amount answer for this row"),
            },
            WaitingFor::TargetSelection { .. } => GameAction::ChooseTarget {
                target: Some(
                    answers
                        .targets
                        .pop_front()
                        .expect("a declared target for this prompt"),
                ),
            },
            WaitingFor::OptionalEffectChoice { .. } => {
                GameAction::DecideOptionalEffect { accept: true }
            }
            WaitingFor::CopyRetarget { .. } => GameAction::KeepAllCopyTargets,
            WaitingFor::OrderTriggers { triggers, .. } => {
                let mut order: Vec<usize> = (0..triggers.len()).collect();
                order.sort_by_key(|&index| Some(triggers[index].source_id) == answers.copier_last);
                GameAction::OrderTriggers { order }
            }
            WaitingFor::DigChoice {
                cards, keep_count, ..
            } => {
                answers.dig_offered = Some(cards.len());
                GameAction::SelectCards {
                    cards: cards.iter().copied().take(*keep_count).collect(),
                }
            }
            WaitingFor::ChooseManaColor { .. } => GameAction::ChooseManaColor {
                choice: ManaChoice::SingleColor(
                    answers.mana_color.expect("a mana colour for this row"),
                ),
                count: 1,
            },
            WaitingFor::PayCost {
                kind: PayCostKind::RemoveCounter { .. },
                ..
            } => GameAction::ChooseRemoveCounterCostDistribution {
                distribution: answers
                    .distribution
                    .clone()
                    .expect("a counter distribution for this row"),
            },
            WaitingFor::PayCost {
                kind: PayCostKind::ReturnToHand,
                ..
            } => GameAction::SelectCards {
                cards: answers.return_choice.clone(),
            },
            WaitingFor::ReplacementChoice { .. } => GameAction::ChooseReplacement {
                index: answers
                    .replacement_choices
                    .pop_front()
                    .expect("a replacement choice for this row"),
            },
            other => panic!("unexpected prompt: {}", other.variant_name()),
        };
        runner
            .act(action)
            .unwrap_or_else(|err| panic!("the engine accepts the action: {err:?}"));
    }
    panic!(
        "stop condition never reached; waiting for {:?}",
        runner.state().waiting_for
    );
}

fn priority_with_stack(len: usize) -> impl Fn(&GameState) -> bool {
    move |state: &GameState| {
        matches!(state.waiting_for, WaitingFor::Priority { .. }) && state.stack.len() == len
    }
}

fn priority_with_empty_stack(state: &GameState) -> bool {
    priority_with_stack(0)(state)
}

fn priority_with_no_trigger_waiting(state: &GameState) -> bool {
    matches!(state.waiting_for, WaitingFor::Priority { .. })
        && !state
            .stack
            .iter()
            .any(|entry| matches!(entry.kind, StackEntryKind::TriggeredAbility { .. }))
}

fn activate(runner: &mut GameRunner, source_id: ObjectId, ability_index: usize) {
    runner
        .act(GameAction::ActivateAbility {
            source_id,
            ability_index,
        })
        .unwrap_or_else(|err| panic!("the ability activates: {err:?}"));
}

/// Activate the `nth` counter-cost ability of `source`, answer every prompt and
/// resolve the stack.
fn activate_and_resolve(
    runner: &mut GameRunner,
    source: ObjectId,
    nth: usize,
    mut answers: Answers,
) -> Answers {
    let index = counter_cost_ability(runner, source, nth);
    activate(runner, source, index);
    drive(runner, priority_with_empty_stack, &mut answers);
    answers
}

// ---------------------------------------------------------------------------
// C1 — stack route, `PreviousEffectAmount`
// ---------------------------------------------------------------------------

/// C1a + C4c: Essence Bottle (remove all, direct push) gains 2 life per elixir
/// counter removed; the count is bound to its own stack entry at placement.
#[test]
fn essence_bottle_gains_two_life_per_counter_removed_by_its_cost() {
    let mut scenario = main_phase();
    let bottle = scenario
        .add_artifact_from_oracle(P0, "Essence Bottle", ESSENCE_BOTTLE)
        .id();
    scenario.with_counter(bottle, counter("elixir"), 3);
    let mut runner = scenario.build();
    let life_before = runner.life(P0);

    let index = counter_cost_ability(&runner, bottle, 0);
    activate(&mut runner, bottle, index);
    let mut answers = Answers::default();
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert_eq!(
        counters_on(&runner, bottle, &counter("elixir")),
        0,
        "reach: the cost removed every elixir counter"
    );
    assert_eq!(
        top_stack_count(&runner),
        Some(3),
        "the stack entry carries its own payment's count"
    );
    assert!(
        payment_ownership_is_clear(&runner),
        "C4c: nothing else holds it"
    );

    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_eq!(runner.life(P0) - life_before, 6);
    assert!(payment_ownership_is_clear(&runner));
}

/// C1b: Molten Hydra deals damage to any target equal to the +1/+1 counters its
/// cost removed.
#[test]
fn molten_hydra_deals_damage_equal_to_counters_removed_by_its_cost() {
    let mut scenario = main_phase();
    let hydra = scenario
        .add_creature_from_oracle(P0, "Molten Hydra", 1, 1, MOLTEN_HYDRA)
        .id();
    scenario.with_counter(hydra, CounterType::Plus1Plus1, 3);
    let mut runner = scenario.build();
    let life_before = runner.life(P1);

    activate_and_resolve(
        &mut runner,
        hydra,
        0,
        Answers::default().target(TargetRef::Player(P1)),
    );

    assert_eq!(counters_on(&runner, hydra, &CounterType::Plus1Plus1), 0);
    assert_eq!(life_before - runner.life(P1), 3);
}

fn nexus_board() -> (GameRunner, ObjectId) {
    let mut scenario = main_phase();
    let nexus = scenario
        .add_artifact_from_oracle(P0, "Geometric Nexus", GEOMETRIC_NEXUS)
        .id();
    scenario.with_counter(nexus, counter("charge"), 4);
    add_mana(&mut scenario, &colorless(6));
    (scenario.build(), nexus)
}

fn assert_fractal_with_four_counters(runner: &GameRunner, nexus: ObjectId) {
    assert_eq!(counters_on(runner, nexus, &counter("charge")), 0);
    let fractals = battlefield_named(runner, "Fractal");
    assert_eq!(fractals.len(), 1, "the Fractal survives on the battlefield");
    assert_eq!(
        counters_on(runner, fractals[0], &CounterType::Plus1Plus1),
        4
    );
}

/// C1c: Geometric Nexus in the `Token.enter_with_counters` shape — the Fractal
/// enters with 4 +1/+1 counters.
#[test]
fn geometric_nexus_fractal_enters_with_the_counters_removed() {
    let (mut runner, nexus) = nexus_board();
    let index = counter_cost_ability(&runner, nexus, 0);
    let def = &runner.state().objects[&nexus].abilities[index];
    assert!(
        matches!(&*def.effect, Effect::Token { enter_with_counters, .. } if !enter_with_counters.is_empty()),
        "structural guard: the parse carries the count on the token: {:?}",
        def.effect
    );

    activate_and_resolve(&mut runner, nexus, 0, Answers::default());

    assert_fractal_with_four_counters(&runner, nexus);
}

/// C1d: Geometric Nexus in the `Token` → `PutCounter { LastCreated }` shape —
/// the seeded amount survives the head and the parent→child hand-off.
#[test]
fn geometric_nexus_fractal_gets_the_counters_removed_through_a_put_counter_sub() {
    let (mut runner, nexus) = nexus_board();
    let index = counter_cost_ability(&runner, nexus, 0);
    let mut def = runner.state().objects[&nexus].abilities[index].clone();
    let Effect::Token {
        enter_with_counters,
        ..
    } = &mut *def.effect
    else {
        panic!("the head is the token creation: {:?}", def.effect);
    };
    let (counter_type, count) = enter_with_counters
        .pop()
        .expect("the parse carries the count on the token");
    assert!(enter_with_counters.is_empty());
    assert!(def.sub_ability.is_none(), "no sub before the rewrite");
    def.sub_ability = Some(Box::new(AbilityDefinition::new(
        def.kind,
        Effect::PutCounter {
            counter_type,
            count,
            target: TargetFilter::LastCreated,
        },
    )));
    let obj = runner.state_mut().objects.get_mut(&nexus).unwrap();
    Arc::make_mut(&mut obj.abilities)[index] = def.clone();
    Arc::make_mut(&mut obj.base_abilities)[index] = def;
    let rewritten = &runner.state().objects[&nexus].abilities[index];
    assert!(
        matches!(&*rewritten.effect, Effect::Token { enter_with_counters, .. } if enter_with_counters.is_empty())
            && rewritten.sub_ability.as_deref().is_some_and(|sub| matches!(
                &*sub.effect,
                Effect::PutCounter {
                    target: TargetFilter::LastCreated,
                    ..
                }
            )),
        "structural guard: the rewrite moved the count into a PutCounter sub"
    );

    activate_and_resolve(&mut runner, nexus, 0, Answers::default());

    assert_fractal_with_four_counters(&runner, nexus);
}

/// C1e: Blademane Baku, X = 2 with 3 ki counters — +2/+0 for each of the two
/// counters removed.
#[test]
fn blademane_baku_gets_two_power_per_counter_removed() {
    let mut scenario = main_phase();
    let baku = scenario
        .add_creature_from_oracle(P0, "Blademane Baku", 1, 1, BLADEMANE_BAKU)
        .id();
    scenario.with_counter(baku, counter("ki"), 3);
    add_mana(&mut scenario, &colorless(1));
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, baku, 0, Answers::x(2));

    assert_eq!(counters_on(&runner, baku, &counter("ki")), 1);
    assert_eq!(power(&runner, baku), 5);
}

// ---------------------------------------------------------------------------
// C2 — remove-all costs (incl. "that much")
// ---------------------------------------------------------------------------

/// C2a: Relic Amulet "deals that much damage" after removing all 3 charge counters.
#[test]
fn relic_amulet_deals_that_much_damage() {
    let mut scenario = main_phase();
    let amulet = scenario
        .add_artifact_from_oracle(P0, "Relic Amulet", RELIC_AMULET)
        .id();
    scenario.with_counter(amulet, counter("charge"), 3);
    let victim = add_victim(&mut scenario, "Victim", 1, 5);
    add_mana(&mut scenario, &colorless(2));
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, amulet, 0, Answers::default().object(victim));

    assert_eq!(counters_on(&runner, amulet, &counter("charge")), 0);
    assert_eq!(damage(&runner, victim), 3);
}

/// C2b: Vish Kal removes 3 +1/+1 counters — the target gets -3/-3.
#[test]
fn vish_kal_shrinks_the_target_per_counter_removed() {
    let mut scenario = main_phase();
    let vish = scenario
        .add_creature_from_oracle(P0, "Vish Kal, Blood Arbiter", 5, 5, VISH_KAL)
        .id();
    scenario.with_counter(vish, CounterType::Plus1Plus1, 3);
    let target = add_victim(&mut scenario, "Target", 5, 5);
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, vish, 0, Answers::default().object(target));

    assert_eq!(counters_on(&runner, vish, &CounterType::Plus1Plus1), 0);
    assert_eq!((power(&runner, target), toughness(&runner, target)), (2, 2));
}

/// C2c: Torture Chamber deals damage equal to the 2 pain counters removed.
#[test]
fn torture_chamber_deals_damage_equal_to_pain_counters_removed() {
    let mut scenario = main_phase();
    let chamber = scenario
        .add_artifact_from_oracle(P0, "Torture Chamber", TORTURE_CHAMBER)
        .id();
    scenario.with_counter(chamber, counter("pain"), 2);
    let victim = add_victim(&mut scenario, "Victim", 1, 5);
    add_mana(&mut scenario, &colorless(1));
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, chamber, 0, Answers::default().object(victim));

    assert_eq!(counters_on(&runner, chamber, &counter("pain")), 0);
    assert_eq!(damage(&runner, victim), 2);
}

/// C2d: Jar of Eyeballs looks at exactly the 3 cards its cost's counters name.
#[test]
fn jar_of_eyeballs_looks_at_the_number_of_counters_removed() {
    let mut scenario = main_phase();
    let jar = scenario
        .add_artifact_from_oracle(P0, "Jar of Eyeballs", JAR_OF_EYEBALLS)
        .id();
    scenario.with_counter(jar, counter("eyeball"), 3);
    add_mana(&mut scenario, &colorless(3));
    let mut runner = scenario.build();
    let hand_before = runner.state().players[0].hand.len();

    let answers = activate_and_resolve(&mut runner, jar, 0, Answers::default());

    assert_eq!(counters_on(&runner, jar, &counter("eyeball")), 0);
    assert_eq!(
        answers.dig_offered,
        Some(3),
        "the dig offers exactly 3 cards"
    );
    assert_eq!(runner.state().players[0].hand.len(), hand_before + 1);
}

/// C2e: Ooze Flux removes 2 + 1 +1/+1 counters from among two creatures and
/// creates one 3/3 Ooze.
#[test]
fn ooze_flux_creates_an_ooze_sized_by_counters_removed_from_among() {
    let mut scenario = main_phase();
    let flux = scenario
        .add_enchantment_from_oracle(P0, "Ooze Flux", OOZE_FLUX)
        .id();
    let first = scenario.add_creature(P0, "First", 1, 1).id();
    let second = scenario.add_creature(P0, "Second", 1, 1).id();
    scenario.with_counter(first, CounterType::Plus1Plus1, 2);
    scenario.with_counter(second, CounterType::Plus1Plus1, 1);
    add_mana(&mut scenario, &[ManaType::Colorless, ManaType::Green]);
    let mut runner = scenario.build();

    let answers = Answers {
        x: Some(3),
        pay_amount: Some(3),
        distribution: Some(vec![
            CounterCostChoice {
                object_id: first,
                counter_type: CounterType::Plus1Plus1,
                count: 2,
            },
            CounterCostChoice {
                object_id: second,
                counter_type: CounterType::Plus1Plus1,
                count: 1,
            },
        ]),
        ..Answers::default()
    };
    activate_and_resolve(&mut runner, flux, 0, answers);

    assert_eq!(counters_on(&runner, first, &CounterType::Plus1Plus1), 0);
    assert_eq!(counters_on(&runner, second, &CounterType::Plus1Plus1), 0);
    let oozes = battlefield_named(&runner, "Ooze");
    assert_eq!(oozes.len(), 1, "one Ooze on the battlefield");
    assert_eq!(
        (power(&runner, oozes[0]), toughness(&runner, oozes[0])),
        (3, 3)
    );
}

/// C2f: Catti-brie deals X damage (X = counters removed) to a blocking creature
/// an opponent controls.
#[test]
fn catti_brie_deals_damage_equal_to_counters_removed() {
    let mut scenario = main_phase();
    let catti = scenario
        .add_creature_from_oracle(P0, "Catti-brie of Mithral Hall", 2, 2, CATTI_BRIE)
        .id();
    scenario.with_counter(catti, CounterType::Plus1Plus1, 3);
    let attacker = scenario.add_creature(P0, "Attacker", 1, 1).id();
    let blocker = add_victim(&mut scenario, "Blocker", 1, 5);
    let mut runner = scenario.build();

    runner.advance_to_combat();
    runner
        .declare_attackers(&[(attacker, engine::game::combat::AttackTarget::Player(P1))])
        .expect("the attack is declared");
    for _ in 0..8 {
        if !matches!(runner.state().waiting_for, WaitingFor::Priority { .. }) {
            break;
        }
        runner
            .act(GameAction::PassPriority)
            .expect("priority passes in the declare attackers step");
    }
    assert!(matches!(
        runner.state().waiting_for,
        WaitingFor::DeclareBlockers { .. }
    ));
    runner
        .declare_blockers(&[(blocker, attacker)])
        .expect("the block is declared");
    let _ = runner.state_mut().add_mana_to_pool(
        P0,
        ManaUnit::new(ManaType::Colorless, ObjectId(0), false, vec![]),
    );
    assert!(matches!(
        runner.state().waiting_for,
        WaitingFor::Priority { player } if player == P0
    ));

    activate_and_resolve(&mut runner, catti, 0, Answers::default().object(blocker));

    assert_eq!(counters_on(&runner, catti, &CounterType::Plus1Plus1), 0);
    assert_eq!(damage(&runner, blocker), 3);
}

// ---------------------------------------------------------------------------
// C3 — mana route (CR 605.3b)
// ---------------------------------------------------------------------------

/// C3a + C4c: Saprazzan Cove removes 2 of 3 storage counters and adds {U}{U}.
#[test]
fn saprazzan_cove_adds_blue_per_storage_counter_removed() {
    let mut scenario = main_phase();
    let cove = scenario
        .add_land_from_oracle(P0, "Saprazzan Cove", SAPRAZZAN_COVE)
        .id();
    scenario.with_counter(cove, counter("storage"), 3);
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, cove, 0, Answers::pay(2));

    assert_eq!(counters_on(&runner, cove, &counter("storage")), 1);
    assert_eq!(pool(&runner, ManaType::Blue), 2);
    assert!(
        payment_ownership_is_clear(&runner),
        "C4c: no payment state outlives the mana ability"
    );
}

/// C3b: Black Mana Battery removes 2 charge counters: {B}, then {B}{B} more from
/// the sub-chain.
#[test]
fn black_mana_battery_adds_an_additional_black_per_counter_removed() {
    let mut scenario = main_phase();
    let battery = scenario
        .add_artifact_from_oracle(P0, "Black Mana Battery", BLACK_MANA_BATTERY)
        .id();
    scenario.with_counter(battery, counter("charge"), 2);
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, battery, 0, Answers::pay(2));

    assert_eq!(counters_on(&runner, battery, &counter("charge")), 0);
    assert_eq!(pool(&runner, ManaType::Black), 3);
}

/// The C3c board: Ooze Flux, a 4/4 creature under Opalescence with the mana
/// ability Cryptolith Rite grants, is the only untapped mana source (the animated
/// Rite, First and Second are tapped). Its counter cost removes 3 +1/+1 counters
/// from among First (2) and Second (1).
fn ooze_flux_with_granted_mana(pool: &[ManaType]) -> (GameRunner, ObjectId, ObjectId, ObjectId) {
    let mut scenario = main_phase();
    scenario.add_enchantment_from_oracle(P0, "Opalescence", OPALESCENCE);
    let rite = scenario
        .add_enchantment_from_oracle(P0, "Cryptolith Rite", CRYPTOLITH_RITE)
        .id();
    // Ooze Flux's printed mana cost {3}{G}: Opalescence makes it a 4/4.
    let flux = scenario
        .add_enchantment_from_oracle(P0, "Ooze Flux", OOZE_FLUX)
        .with_mana_cost(ManaCost::Cost {
            shards: vec![ManaCostShard::Green],
            generic: 3,
        })
        .id();
    let first = scenario.add_creature(P0, "First", 1, 1).id();
    let second = scenario.add_creature(P0, "Second", 1, 1).id();
    scenario.with_counter(first, CounterType::Plus1Plus1, 2);
    scenario.with_counter(second, CounterType::Plus1Plus1, 1);
    add_mana(&mut scenario, pool);
    let mut runner = scenario.build();
    engine::game::layers::evaluate_layers(runner.state_mut());
    for id in [rite, first, second] {
        runner.state_mut().objects.get_mut(&id).unwrap().tapped = true;
    }
    assert!(
        runner.state().objects[&flux]
            .card_types
            .core_types
            .contains(&engine::types::card_type::CoreType::Creature),
        "reach: Opalescence makes Ooze Flux a creature"
    );
    (runner, flux, first, second)
}

fn ooze_flux_answers(first: ObjectId, second: ObjectId) -> Answers {
    Answers {
        x: Some(3),
        pay_amount: Some(3),
        distribution: Some(vec![
            CounterCostChoice {
                object_id: first,
                counter_type: CounterType::Plus1Plus1,
                count: 2,
            },
            CounterCostChoice {
                object_id: second,
                counter_type: CounterType::Plus1Plus1,
                count: 1,
            },
        ]),
        mana_color: Some(ManaType::Green),
        ..Answers::default()
    }
}

fn assert_one_three_three_ooze(runner: &GameRunner, first: ObjectId, second: ObjectId) {
    assert_eq!(counters_on(runner, first, &CounterType::Plus1Plus1), 0);
    assert_eq!(counters_on(runner, second, &CounterType::Plus1Plus1), 0);
    let oozes = battlefield_named(runner, "Ooze");
    assert_eq!(oozes.len(), 1, "the Ooze survives: it is not 0/0");
    assert_eq!(
        (power(runner, oozes[0]), toughness(runner, oozes[0])),
        (3, 3),
        "Ooze Flux reads its own 3 counters removed"
    );
    assert!(payment_ownership_is_clear(runner));
}

/// C3c: Ooze Flux pays its interactive counter cost (3 from among), then, with
/// its {1}{G} waiting for mana, activates the mana ability Cryptolith Rite grants
/// it (CR 601.2g + CR 605.3a). That mana ability removed no counters, so it must
/// neither read nor strip Ooze Flux's pending count: the Ooze is 3/3.
#[test]
fn a_mana_ability_activated_while_paying_does_not_take_the_pending_count() {
    // An empty pool: Ooze Flux's one mana can't pay {1}{G}, so the payment
    // waits at `ManaPayment` instead of auto-paying.
    let (mut runner, flux, first, second) = ooze_flux_with_granted_mana(&[]);
    let mut answers = ooze_flux_answers(first, second);

    let index = counter_cost_ability(&runner, flux, 0);
    activate(&mut runner, flux, index);
    drive(&mut runner, is_mana_payment, &mut answers);
    assert_eq!(
        pending_activation_count(&runner),
        Some(3),
        "the paused activation owns its 3 counters removed"
    );

    let granted_mana = mana_ability_without_counter_cost(&runner, flux);
    activate(&mut runner, flux, granted_mana);
    drive(&mut runner, is_mana_payment, &mut answers);
    assert!(
        runner.state().objects[&flux].tapped,
        "reach: Ooze Flux tapped for mana with the granted ability"
    );
    assert_eq!(
        pool(&runner, ManaType::Green),
        1,
        "reach: the granted mana ability produced its mana"
    );
    assert_eq!(
        pending_activation_count(&runner),
        Some(3),
        "the mana ability neither read nor stripped the pending count"
    );

    // The rest of {1}{G}.
    let _ = runner.state_mut().add_mana_to_pool(
        P0,
        ManaUnit::new(ManaType::Colorless, ObjectId(0), false, vec![]),
    );
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert!(
        runner.state().players[0].mana_pool.is_empty(),
        "reach: the pool paid {{1}}{{G}}"
    );
    assert_eq!(top_stack_count(&runner), Some(3));

    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_one_three_three_ooze(&runner, first, second);
}

/// C3c (auto-tap): with {G} in the pool, the payment auto-taps Ooze Flux for the
/// {1} with its granted mana ability (CR 605.3a). That mana ability removed no
/// counters either: the stack entry still carries Ooze Flux's own 3.
#[test]
fn an_auto_tapped_mana_ability_does_not_take_the_pending_count() {
    let (mut runner, flux, first, second) = ooze_flux_with_granted_mana(&[ManaType::Green]);
    let mut answers = ooze_flux_answers(first, second);

    let index = counter_cost_ability(&runner, flux, 0);
    activate(&mut runner, flux, index);
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert!(
        runner.state().objects[&flux].tapped,
        "reach: Ooze Flux tapped for mana with the granted ability"
    );
    assert!(
        runner.state().players[0].mana_pool.is_empty(),
        "reach: the pool and the granted mana paid {{1}}{{G}}"
    );
    assert_eq!(top_stack_count(&runner), Some(3));

    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_one_three_three_ooze(&runner, first, second);
}

/// R3: a counter cost nested at any `Composite` depth is still this mana
/// ability's own counter leg — Saprazzan Cove's cost rewritten to
/// `Composite[Composite[{T}, Composite[<remove any number of storage counters>]]]`
/// removes 2 storage counters and adds {U}{U}.
#[test]
fn nested_composite_counter_cost_mana_ability_adds_per_counter_removed() {
    let mut scenario = main_phase();
    let cove = scenario
        .add_land_from_oracle(P0, "Saprazzan Cove", SAPRAZZAN_COVE)
        .id();
    scenario.with_counter(cove, counter("storage"), 3);
    let mut runner = scenario.build();
    let index = counter_cost_ability(&runner, cove, 0);
    let mut def = runner.state().objects[&cove].abilities[index].clone();
    let Some(AbilityCost::Composite { costs }) = def.cost.take() else {
        panic!(
            "structural guard: the parsed cost is composite: {:?}",
            def.cost
        );
    };
    let [tap @ AbilityCost::Tap, remove @ AbilityCost::RemoveCounter { .. }] = costs.as_slice()
    else {
        panic!("structural guard: the parsed cost is {{T}} + a counter removal: {costs:?}");
    };
    def.cost = Some(AbilityCost::Composite {
        costs: vec![AbilityCost::Composite {
            costs: vec![
                tap.clone(),
                AbilityCost::Composite {
                    costs: vec![remove.clone()],
                },
            ],
        }],
    });
    let obj = runner.state_mut().objects.get_mut(&cove).unwrap();
    Arc::make_mut(&mut obj.abilities)[index] = def.clone();
    Arc::make_mut(&mut obj.base_abilities)[index] = def;

    activate(&mut runner, cove, index);
    assert!(
        matches!(
            runner.state().waiting_for,
            WaitingFor::PayAmountChoice { .. }
        ),
        "reach: the nested counter leg was found and prompted: {:?}",
        runner.state().waiting_for
    );
    drive(&mut runner, priority_with_empty_stack, &mut Answers::pay(2));

    assert_eq!(counters_on(&runner, cove, &counter("storage")), 1);
    assert_eq!(pool(&runner, ManaType::Blue), 2);
    assert!(payment_ownership_is_clear(&runner));
}

/// B1-A: Pentad Prism's fixed-count self counter cost is paid by the mana cost
/// cursor's self-contained arm, and its count belongs to that mana activation:
/// it is on the pending mana ability at the colour prompt (CR 605.3b), and
/// "Remove a charge counter from this artifact: Add one mana of any color."
/// removes one counter and adds one mana.
#[test]
fn pentad_prism_fixed_counter_cost_is_owned_by_its_mana_activation() {
    let mut scenario = main_phase();
    let prism = scenario
        .add_artifact_from_oracle(P0, "Pentad Prism", PENTAD_PRISM)
        .id();
    scenario.with_counter(prism, counter("charge"), 2);
    let mut runner = scenario.build();

    let index = counter_cost_ability(&runner, prism, 0);
    activate(&mut runner, prism, index);
    let WaitingFor::ChooseManaColor {
        context: ManaChoiceContext::ManaAbility(pending),
        ..
    } = &runner.state().waiting_for
    else {
        panic!(
            "reach: the mana ability asks for its colour: {:?}",
            runner.state().waiting_for
        );
    };
    assert_eq!(
        pending.cost_removed_counters,
        Some(1),
        "the mana activation owns the one counter its cost removed"
    );
    assert_eq!(
        counters_on(&runner, prism, &counter("charge")),
        1,
        "reach: the cost removed one charge counter"
    );

    runner
        .act(GameAction::ChooseManaColor {
            choice: ManaChoice::SingleColor(ManaType::Red),
            count: 1,
        })
        .expect("the colour is chosen");

    assert!(matches!(
        runner.state().waiting_for,
        WaitingFor::Priority { .. }
    ));
    assert_eq!(pool(&runner, ManaType::Red), 1);
    assert_eq!(runner.state().players[0].mana_pool.total(), 1);
    assert_eq!(counters_on(&runner, prism, &counter("charge")), 1);
    assert!(payment_ownership_is_clear(&runner));
}

/// R1: two activations of one source in flight at once (CR 601.2g + CR 605.3a).
/// Under Opalescence and March of the Machines, Ooze Flux and Black Mana Battery
/// are creatures with a +1/+1 counter, so Experiment Kraj has both of their
/// counter-cost abilities. Kraj activates Ooze Flux's (3 counters from among
/// First and Second), then, while paying its {1}{G}, activates Battery's mana
/// ability (CR 201.5b + CR 700.7: "this artifact" is Kraj, which loses 2 charge counters).
/// Each reads its own count: {B}{B}{B}, then one 3/3 Ooze.
#[test]
fn kraj_nested_same_source_counter_cost_mana_payment_keeps_each_count() {
    let mut scenario = main_phase();
    scenario.add_enchantment_from_oracle(P0, "Opalescence", OPALESCENCE);
    scenario.add_enchantment_from_oracle(P0, "March of the Machines", MARCH_OF_THE_MACHINES);
    let kraj = scenario
        .add_creature_from_oracle(P0, "Experiment Kraj", 4, 6, EXPERIMENT_KRAJ)
        .id();
    // Printed mana costs {3}{G} and {4}: each becomes a 4/4 creature.
    let flux = scenario
        .add_enchantment_from_oracle(P0, "Ooze Flux", OOZE_FLUX)
        .with_mana_cost(ManaCost::Cost {
            shards: vec![ManaCostShard::Green],
            generic: 3,
        })
        .id();
    let battery = scenario
        .add_artifact_from_oracle(P0, "Black Mana Battery", BLACK_MANA_BATTERY)
        .with_mana_cost(ManaCost::Cost {
            shards: vec![],
            generic: 4,
        })
        .id();
    let first = scenario.add_creature(P0, "First", 1, 1).id();
    let second = scenario.add_creature(P0, "Second", 1, 1).id();
    scenario.with_counter(flux, CounterType::Plus1Plus1, 1);
    scenario.with_counter(battery, CounterType::Plus1Plus1, 1);
    scenario.with_counter(first, CounterType::Plus1Plus1, 2);
    scenario.with_counter(second, CounterType::Plus1Plus1, 1);
    scenario.with_counter(kraj, counter("charge"), 2);
    add_mana(&mut scenario, &[ManaType::Green]);
    let mut runner = scenario.build();
    engine::game::layers::evaluate_layers(runner.state_mut());

    let flux_ability = counter_cost_ability(&runner, kraj, 0);
    let battery_ability = counter_cost_ability(&runner, kraj, 1);
    let kraj_abilities = &runner.state().objects[&kraj].abilities;
    assert!(
        matches!(&*kraj_abilities[flux_ability].effect, Effect::Token { .. })
            && matches!(
                &*kraj_abilities[battery_ability].effect,
                Effect::Mana { .. }
            ),
        "reach: Kraj has Ooze Flux's and Battery's counter-cost abilities"
    );

    let mut outer = Answers {
        x: Some(3),
        pay_amount: Some(3),
        distribution: Some(vec![
            CounterCostChoice {
                object_id: first,
                counter_type: CounterType::Plus1Plus1,
                count: 2,
            },
            CounterCostChoice {
                object_id: second,
                counter_type: CounterType::Plus1Plus1,
                count: 1,
            },
        ]),
        ..Answers::default()
    };
    activate(&mut runner, kraj, flux_ability);
    drive(&mut runner, is_mana_payment, &mut outer);
    assert_eq!(
        pending_activation_count(&runner),
        Some(3),
        "the outer activation owns its 3 counters removed"
    );

    activate(&mut runner, kraj, battery_ability);
    drive(&mut runner, is_mana_payment, &mut Answers::pay(2));
    assert_eq!(
        counters_on(&runner, kraj, &counter("charge")),
        0,
        "reach: the inner mana ability removed Kraj's 2 charge counters"
    );
    assert_eq!(
        pool(&runner, ManaType::Black),
        3,
        "the inner mana ability reads only its own 2: {{B}} + {{B}}{{B}}"
    );
    assert_eq!(pool(&runner, ManaType::Green), 1);
    assert_eq!(
        pending_activation_count(&runner),
        Some(3),
        "the inner mana ability left the outer count in place"
    );

    drive(&mut runner, priority_with_empty_stack, &mut outer);
    let oozes = battlefield_named(&runner, "Ooze");
    assert_eq!(oozes.len(), 1, "exactly one Ooze on the battlefield");
    assert_eq!(
        (power(&runner, oozes[0]), toughness(&runner, oozes[0])),
        (3, 3),
        "the outer activation reads its own 3 counters removed"
    );
    assert_eq!(
        pool(&runner, ManaType::Black),
        2,
        "{{1}}{{G}} took the {{G}} and one {{B}}"
    );
    assert_eq!(counters_on(&runner, kraj, &counter("charge")), 0);
    assert_eq!(counters_on(&runner, first, &CounterType::Plus1Plus1), 0);
    assert_eq!(counters_on(&runner, second, &CounterType::Plus1Plus1), 0);
    assert!(payment_ownership_is_clear(&runner));
}

// ---------------------------------------------------------------------------
// R2 — a counter-cost removal deferred by a replacement choice (CR 616.1)
// ---------------------------------------------------------------------------
//
// No printed card replaces a counter removal, so these rows install the typed
// optional replacement `ReplacementDefinition::new(RemoveCounter)
// .mode(Optional { decline: None })` (the fixture of the effect-path tests in
// `effects/counters.rs`). Its applier hands the event back unchanged, so
// accepting and declining both remove what the cost asked for: the replacement
// leg's reading is derived from that applier, not from a card. What the rows
// pin is the payment around it — the removal waits for the choice, the cost is
// still paid (CR 601.2h), and the activation reads the count actually removed
// once it is settled (CR 118.11), never a zero recorded while it waited.

/// Accept the optional replacement at a `ReplacementChoice`.
const ACCEPT: usize = 0;
/// Decline it: the removal happens unreplaced.
const DECLINE: usize = 1;

/// A P0 permanent carrying the optional counter-removal replacement.
fn install_optional_counter_removal_replacement(runner: &mut GameRunner) {
    let holder = runner
        .state()
        .battlefield
        .iter()
        .copied()
        .find(|id| runner.state().objects[id].name == "Steward")
        .expect("the board has the replacement's holder");
    let replacement = ReplacementDefinition::new(ReplacementEvent::RemoveCounter)
        .mode(ReplacementMode::Optional { decline: None });
    let obj = runner.state_mut().objects.get_mut(&holder).unwrap();
    Arc::make_mut(&mut obj.base_replacement_definitions).push(replacement.clone());
    obj.replacement_definitions.push(replacement);
}

/// `main_phase` with the replacement's holder on P0's side.
fn replacement_board() -> GameScenario {
    let mut scenario = main_phase();
    scenario.add_creature(P0, "Steward", 1, 1);
    scenario
}

/// The optional replacement's accept/decline prompt is up (positive reach).
fn assert_optional_replacement_prompt(runner: &GameRunner) {
    assert!(
        matches!(
            runner.state().waiting_for,
            WaitingFor::ReplacementChoice {
                candidate_count: 2,
                kind: ReplacementChoiceKind::OptionalBranch,
                ..
            }
        ),
        "reach: the counter removal waits on the optional replacement: {:?}",
        runner.state().waiting_for
    );
}

fn choose_replacement(runner: &mut GameRunner, index: usize) {
    runner
        .act(GameAction::ChooseReplacement { index })
        .unwrap_or_else(|err| panic!("the replacement choice is accepted: {err:?}"));
}

/// The parked stack-route counter-cost root: its attached payment, record and
/// completion.
fn parked_counter_cost_root(
    runner: &GameRunner,
) -> (
    &PendingCast,
    &PausedCounterCostRemoval,
    &PendingCostMoveCompletion,
) {
    match &runner.state().pending_cost_move_resume {
        Some(PendingCostMoveResume::CounterCostRemovalPayment {
            pending: Some(pending),
            removal,
            completion,
            ..
        }) => (pending, removal, completion),
        other => panic!("the payment is parked on its counter-cost removal: {other:?}"),
    }
}

/// The parked mana-route cursor.
fn parked_mana_cursor(runner: &GameRunner) -> &ManaAbilityCostCursor {
    match &runner.state().pending_cost_move_resume {
        Some(PendingCostMoveResume::ManaAbilityPayment { cursor, .. }) => cursor,
        other => panic!("the mana ability is parked on its cost cursor: {other:?}"),
    }
}

/// R2 stack route, direct pay: Essence Bottle's "Remove all elixir counters"
/// waits on the replacement choice, so the activation pauses with the cost
/// unpaid and no count. After the choice, the stack entry reads the 3 counters
/// actually removed: +6 life. Declining removes the same 3 unreplaced.
#[test]
fn deferred_cost_removal_is_counted_after_its_replacement_choice() {
    for choice in [ACCEPT, DECLINE] {
        let mut scenario = replacement_board();
        let bottle = scenario
            .add_artifact_from_oracle(P0, "Essence Bottle", ESSENCE_BOTTLE)
            .id();
        scenario.with_counter(bottle, counter("elixir"), 3);
        let mut runner = scenario.build();
        install_optional_counter_removal_replacement(&mut runner);
        let life_before = runner.life(P0);

        let index = counter_cost_ability(&runner, bottle, 0);
        activate(&mut runner, bottle, index);
        assert_optional_replacement_prompt(&runner);
        assert!(runner.state().stack.is_empty(), "not activated yet");
        assert_eq!(
            counters_on(&runner, bottle, &counter("elixir")),
            3,
            "the removal waits on the choice"
        );
        let (pending, _, _) = parked_counter_cost_root(&runner);
        assert_eq!(
            pending.ability.context.cost_removed_counter_amount, None,
            "a parked payment holds no count for a removal still waiting"
        );

        choose_replacement(&mut runner, choice);
        assert_eq!(runner.state().stack.len(), 1, "choice {choice}: placed");
        assert_eq!(top_stack_count(&runner), Some(3), "choice {choice}");
        assert_eq!(counters_on(&runner, bottle, &counter("elixir")), 0);

        drive(
            &mut runner,
            priority_with_empty_stack,
            &mut Answers::default(),
        );
        assert_eq!(runner.life(P0) - life_before, 6, "choice {choice}");
        assert!(payment_ownership_is_clear(&runner), "choice {choice}");
    }
}

/// R2 stack route, selected handler with a re-park: Ooze Flux removes 2 from
/// First and 1 from Second; each removal waits on its own choice and is settled
/// before the next one is made. The Ooze is 3/3.
#[test]
fn deferred_removals_from_among_settle_one_at_a_time() {
    let mut scenario = replacement_board();
    let flux = scenario
        .add_enchantment_from_oracle(P0, "Ooze Flux", OOZE_FLUX)
        .id();
    let first = scenario.add_creature(P0, "First", 1, 1).id();
    let second = scenario.add_creature(P0, "Second", 1, 1).id();
    scenario.with_counter(first, CounterType::Plus1Plus1, 2);
    scenario.with_counter(second, CounterType::Plus1Plus1, 1);
    add_mana(&mut scenario, &[ManaType::Colorless, ManaType::Green]);
    let mut runner = scenario.build();
    install_optional_counter_removal_replacement(&mut runner);
    let mut answers = Answers {
        x: Some(3),
        pay_amount: Some(3),
        distribution: Some(vec![
            CounterCostChoice {
                object_id: first,
                counter_type: CounterType::Plus1Plus1,
                count: 2,
            },
            CounterCostChoice {
                object_id: second,
                counter_type: CounterType::Plus1Plus1,
                count: 1,
            },
        ]),
        ..Answers::default()
    };

    let index = counter_cost_ability(&runner, flux, 0);
    activate(&mut runner, flux, index);
    drive(
        &mut runner,
        |state| matches!(state.waiting_for, WaitingFor::ReplacementChoice { .. }),
        &mut answers,
    );
    assert_optional_replacement_prompt(&runner);
    let (pending, removal, _) = parked_counter_cost_root(&runner);
    assert_eq!(pending.ability.context.cost_removed_counter_amount, None);
    assert_eq!(removal.in_flight.removal.object_id, first);
    assert_eq!(removal.remaining.len(), 1, "Second's removal is still owed");

    choose_replacement(&mut runner, ACCEPT);
    assert_optional_replacement_prompt(&runner);
    assert_eq!(counters_on(&runner, first, &CounterType::Plus1Plus1), 0);
    assert_eq!(counters_on(&runner, second, &CounterType::Plus1Plus1), 1);
    let (pending, removal, _) = parked_counter_cost_root(&runner);
    assert_eq!(
        pending.ability.context.cost_removed_counter_amount,
        Some(2),
        "First's settled 2 are credited before Second's removal is made"
    );
    assert_eq!(removal.in_flight.removal.object_id, second);
    assert!(removal.remaining.is_empty());

    answers.replacement_choices.push_back(ACCEPT);
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert!(
        answers.replacement_choices.is_empty(),
        "Second's removal raised its own prompt"
    );
    assert!(
        runner.state().players[0].mana_pool.is_empty(),
        "reach: the activation went on to pay {{1}}{{G}}"
    );
    assert_eq!(top_stack_count(&runner), Some(3));

    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_one_three_three_ooze(&runner, first, second);
}

/// R2 mana route, chosen-count leaf: Saprazzan Cove's "Remove any number of
/// storage counters" (2 of 3) waits on the choice. The cursor holds the removal,
/// has moved past the counter component and holds no count; after the choice
/// the mana ability reads the 2 removed: {U}{U}.
#[test]
fn deferred_mana_cost_removal_is_counted_after_its_replacement_choice() {
    let mut scenario = replacement_board();
    let cove = scenario
        .add_land_from_oracle(P0, "Saprazzan Cove", SAPRAZZAN_COVE)
        .id();
    scenario.with_counter(cove, counter("storage"), 3);
    let mut runner = scenario.build();
    install_optional_counter_removal_replacement(&mut runner);

    let index = counter_cost_ability(&runner, cove, 0);
    activate(&mut runner, cove, index);
    drive(
        &mut runner,
        |state| matches!(state.waiting_for, WaitingFor::ReplacementChoice { .. }),
        &mut Answers::pay(2),
    );
    assert_optional_replacement_prompt(&runner);
    let cursor = parked_mana_cursor(&runner);
    assert!(
        cursor.counter_removal.is_some(),
        "the cursor holds the removal"
    );
    assert!(
        !cursor.remaining.iter().any(cost_removes_counters),
        "the cursor moved past the counter component: {:?}",
        cursor.remaining
    );
    assert_eq!(cursor.cost_removed_counters, None);
    assert_eq!(counters_on(&runner, cove, &counter("storage")), 3);

    choose_replacement(&mut runner, ACCEPT);
    assert_eq!(pool(&runner, ManaType::Blue), 2);
    assert_eq!(counters_on(&runner, cove, &counter("storage")), 1);
    assert!(payment_ownership_is_clear(&runner));
}

/// B1-B, mana route, fixed-count self leaf: Pentad Prism's "Remove a charge
/// counter" is paid by the cursor's self-contained arm. It waits on the choice
/// inside the cursor, and after the choice the mana activation owns the 1
/// counter removed: one mana, charge 2 -> 1. Declining reads the same.
#[test]
fn deferred_fixed_mana_cost_removal_pauses_in_the_cursor() {
    for choice in [ACCEPT, DECLINE] {
        let mut scenario = replacement_board();
        let prism = scenario
            .add_artifact_from_oracle(P0, "Pentad Prism", PENTAD_PRISM)
            .id();
        scenario.with_counter(prism, counter("charge"), 2);
        let mut runner = scenario.build();
        install_optional_counter_removal_replacement(&mut runner);

        let index = counter_cost_ability(&runner, prism, 0);
        activate(&mut runner, prism, index);
        assert_optional_replacement_prompt(&runner);
        let cursor = parked_mana_cursor(&runner);
        assert!(
            cursor.counter_removal.is_some(),
            "the cursor holds the removal"
        );
        assert!(!cursor.remaining.iter().any(cost_removes_counters));
        assert_eq!(
            cursor.cost_removed_counters, None,
            "nothing is credited while the removal waits"
        );
        assert_eq!(counters_on(&runner, prism, &counter("charge")), 2);

        choose_replacement(&mut runner, choice);
        let WaitingFor::ChooseManaColor {
            context: ManaChoiceContext::ManaAbility(pending),
            ..
        } = &runner.state().waiting_for
        else {
            panic!(
                "choice {choice}: the mana ability asks for its colour: {:?}",
                runner.state().waiting_for
            );
        };
        assert_eq!(pending.cost_removed_counters, Some(1), "choice {choice}");
        assert_eq!(counters_on(&runner, prism, &counter("charge")), 1);

        runner
            .act(GameAction::ChooseManaColor {
                choice: ManaChoice::SingleColor(ManaType::Red),
                count: 1,
            })
            .expect("the colour is chosen");
        assert_eq!(runner.state().players[0].mana_pool.total(), 1);
        assert_eq!(counters_on(&runner, prism, &counter("charge")), 1);
        assert!(payment_ownership_is_clear(&runner), "choice {choice}");
    }
}

/// B4-a: Geometric Nexus pays {6} first through the mana-first detour; its
/// residual "{T}, Remove all charge counters" is paid as the ability is pushed
/// and waits on the choice. The count rides the paused activation across both
/// entries into the payment: the Fractal gets the 3 counters removed.
#[test]
fn geometric_nexus_deferred_residual_counter_leg_counts_after_the_choice() {
    let mut scenario = replacement_board();
    let nexus = scenario
        .add_artifact_from_oracle(P0, "Geometric Nexus", GEOMETRIC_NEXUS)
        .id();
    scenario.with_counter(nexus, counter("charge"), 3);
    add_mana(&mut scenario, &colorless(6));
    let mut runner = scenario.build();
    install_optional_counter_removal_replacement(&mut runner);

    let index = counter_cost_ability(&runner, nexus, 0);
    perf_counters::reset();
    activate(&mut runner, nexus, index);
    let reaches = perf_counters::activation_cost_route_snapshot().guard_reaches;
    assert!(
        reaches[ActivationCostGuardSite::PushToStack as usize] > 0
            && reaches[ActivationCostGuardSite::ManaResume as usize] > 0,
        "reach: the mana was paid first and the counter leg as the push's residual: {reaches:?}"
    );
    assert_optional_replacement_prompt(&runner);
    assert!(runner.state().players[0].mana_pool.is_empty(), "{{6}} paid");
    assert!(runner.state().stack.is_empty());
    assert_eq!(counters_on(&runner, nexus, &counter("charge")), 3);
    let (pending, _, _) = parked_counter_cost_root(&runner);
    assert_eq!(pending.activation_cost, None, "no residual is left");
    assert_eq!(pending.ability.context.cost_removed_counter_amount, None);

    choose_replacement(&mut runner, ACCEPT);
    assert_eq!(runner.state().stack.len(), 1);
    assert_eq!(top_stack_count(&runner), Some(3));
    assert_eq!(counters_on(&runner, nexus, &counter("charge")), 0);

    drive(
        &mut runner,
        priority_with_empty_stack,
        &mut Answers::default(),
    );
    let fractals = battlefield_named(&runner, "Fractal");
    assert_eq!(fractals.len(), 1, "the Fractal survives on the battlefield");
    assert_eq!(
        counters_on(&runner, fractals[0], &CounterType::Plus1Plus1),
        3
    );
    assert!(payment_ownership_is_clear(&runner));
}

/// B4-b: Magosi's automatic "{T}, Remove an eon counter" is paid before its
/// selected self-return; the removal waits on the choice, so the root carries
/// the selected return. After the choice the removal is settled first, then
/// Magosi returns to hand, and the activation reads its 1 counter removed.
/// Magosi starts with 2 eon counters so the order is observable: settling after
/// the return would also count the eon counter that ceased to exist when Magosi
/// left the battlefield (CR 122.2) and read 2, not the 1 the cost removed.
/// Unreplaced control first: the return completes in the same action.
#[test]
fn magosi_deferred_counter_leg_completes_its_selected_return_after_the_choice() {
    for replaced in [false, true] {
        let mut scenario = replacement_board();
        let magosi = scenario
            .add_land_from_oracle(P0, "Magosi, the Waterveil", MAGOSI)
            .id();
        scenario.with_counter(magosi, counter("eon"), 2);
        let mut runner = scenario.build();
        runner.state_mut().objects.get_mut(&magosi).unwrap().tapped = false;
        if replaced {
            install_optional_counter_removal_replacement(&mut runner);
        }

        let index = counter_cost_ability(&runner, magosi, 0);
        activate(&mut runner, magosi, index);
        assert!(
            matches!(
                runner.state().waiting_for,
                WaitingFor::PayCost {
                    kind: PayCostKind::ReturnToHand,
                    ..
                }
            ),
            "reach: the self-return is a selected cost: {:?}",
            runner.state().waiting_for
        );
        let mut answers = Answers {
            return_choice: vec![magosi],
            ..Answers::default()
        };
        drive(
            &mut runner,
            |state| {
                priority_with_stack(1)(state)
                    || matches!(state.waiting_for, WaitingFor::ReplacementChoice { .. })
            },
            &mut answers,
        );

        if replaced {
            assert_optional_replacement_prompt(&runner);
            assert!(runner.state().stack.is_empty());
            assert_eq!(runner.state().objects[&magosi].zone, Zone::Battlefield);
            assert_eq!(counters_on(&runner, magosi, &counter("eon")), 2);
            let (pending, _, completion) = parked_counter_cost_root(&runner);
            assert_eq!(pending.ability.context.cost_removed_counter_amount, None);
            assert!(
                matches!(
                    completion,
                    PendingCostMoveCompletion::CompleteSelectedReturnToHand { selected, .. }
                        if selected == &vec![magosi]
                ),
                "the root carries the selected return: {completion:?}"
            );
            choose_replacement(&mut runner, ACCEPT);
        }

        assert_eq!(runner.state().stack.len(), 1, "replaced {replaced}");
        assert_eq!(runner.state().objects[&magosi].zone, Zone::Hand);
        assert_eq!(top_stack_count(&runner), Some(1), "replaced {replaced}");

        drive(&mut runner, priority_with_empty_stack, &mut answers);
        assert_eq!(runner.state().extra_turns.len(), 1, "replaced {replaced}");
        assert!(payment_ownership_is_clear(&runner), "replaced {replaced}");
    }
}

/// Stage a full prevention of the parked counter removal. No RemoveCounter
/// applier yields `Prevented` today (`remove_counter_applier` always modifies
/// the event), so the parked choice is re-pointed at a skip-turn definition on
/// the same holder, whose applier prevents its event outright. This mirrors the
/// staged regeneration-shield producer of the CR 608.3e prevented-entry test in
/// `engine_replacement.rs`. The parked event is still the RemoveCounter, and the
/// resume runs through the real `GameAction::ChooseReplacement` entry.
fn stage_prevented_counter_removal(runner: &mut GameRunner) {
    let holder = runner
        .state()
        .battlefield
        .iter()
        .copied()
        .find(|id| runner.state().objects[id].name == "Steward")
        .expect("the board has the replacement's holder");
    let prevention = ReplacementDefinition::new(ReplacementEvent::BeginTurn);
    let obj = runner.state_mut().objects.get_mut(&holder).unwrap();
    let index = obj.replacement_definitions.len();
    Arc::make_mut(&mut obj.base_replacement_definitions).push(prevention.clone());
    obj.replacement_definitions.push(prevention);
    let pending = runner
        .state_mut()
        .pending_replacement
        .as_mut()
        .expect("a counter removal waits on its replacement choice");
    assert!(
        matches!(pending.proposed, ProposedEvent::RemoveCounter { .. }),
        "reach: the parked event is the counter removal: {:?}",
        pending.proposed
    );
    pending.candidates = vec![ReplacementId {
        source: holder,
        index,
    }];
    pending.is_optional = false;
}

/// Prevented boundary, a removal no cost owns (CR 616.1): Ember Shot's 3 damage
/// to a planeswalker removes loyalty counters (CR 120.3c), and that removal
/// waits on the choice with the spell's "Draw a card." still to resolve. When
/// the removal is prevented outright, no loyalty is removed (CR 614.6) and the
/// spell resumes: its controller draws the card and the spell finishes. Only a
/// cost-move root's removal may leave this drain for the cost-move resume.
#[test]
fn a_prevented_damage_counter_removal_resumes_the_resolving_spell() {
    let mut scenario = replacement_board();
    let walker = scenario
        .add_planeswalker_from_oracle(P1, "Walker", "Gideon", 5, "+1: You gain 1 life.")
        .id();
    let shot = scenario
        .add_spell_to_hand_from_oracle(P0, "Ember Shot", true, EMBER_SHOT)
        .id();
    let mut mana = colorless(6);
    mana.push(ManaType::Red);
    add_mana(&mut scenario, &mana);
    let mut runner = scenario.build();
    install_optional_counter_removal_replacement(&mut runner);

    let card_id = runner.state().objects[&shot].card_id;
    runner
        .act(GameAction::CastSpell {
            object_id: shot,
            card_id,
            targets: vec![],
            payment_mode: CastPaymentMode::Auto,
        })
        .unwrap_or_else(|err| panic!("Ember Shot is cast: {err:?}"));
    drive(
        &mut runner,
        |state| matches!(state.waiting_for, WaitingFor::ReplacementChoice { .. }),
        &mut Answers::default().object(walker),
    );
    assert_optional_replacement_prompt(&runner);
    assert!(
        runner.state().active_ability_continuation().is_some(),
        "reach: the spell's draw waits behind the removal"
    );
    assert!(runner.state().pending_cost_move_resume.is_none());
    assert_eq!(counters_on(&runner, walker, &CounterType::Loyalty), 5);
    let hand_at_choice = runner.state().players[0].hand.len();
    stage_prevented_counter_removal(&mut runner);

    choose_replacement(&mut runner, 0);
    drive(
        &mut runner,
        priority_with_empty_stack,
        &mut Answers::default(),
    );
    assert_eq!(
        counters_on(&runner, walker, &CounterType::Loyalty),
        5,
        "the prevented removal removed nothing"
    );
    assert_eq!(
        runner.state().players[0].hand.len(),
        hand_at_choice + 1,
        "the spell's draw still resolves"
    );
    assert_eq!(runner.state().objects[&shot].zone, Zone::Graveyard);
    assert!(runner.state().active_ability_continuation().is_none());
    assert!(payment_ownership_is_clear(&runner));
}

/// Prevented boundary, a cost-move root's removal: Essence Bottle's "Remove all
/// elixir counters" waits on the choice with the activation parked. When the
/// removal is prevented outright, the cost is still paid (CR 118.11) and no
/// counter was removed (CR 614.6), so the parked root resumes: the ability goes
/// on the stack reading 0 counters removed and gains no life.
#[test]
fn a_prevented_cost_counter_removal_resumes_its_cost_root() {
    let mut scenario = replacement_board();
    let bottle = scenario
        .add_artifact_from_oracle(P0, "Essence Bottle", ESSENCE_BOTTLE)
        .id();
    scenario.with_counter(bottle, counter("elixir"), 3);
    let mut runner = scenario.build();
    install_optional_counter_removal_replacement(&mut runner);
    let life_before = runner.life(P0);

    let index = counter_cost_ability(&runner, bottle, 0);
    activate(&mut runner, bottle, index);
    assert_optional_replacement_prompt(&runner);
    let (pending, _, _) = parked_counter_cost_root(&runner);
    assert_eq!(pending.ability.context.cost_removed_counter_amount, None);
    stage_prevented_counter_removal(&mut runner);

    choose_replacement(&mut runner, 0);
    assert_eq!(
        runner.state().stack.len(),
        1,
        "the parked activation is placed"
    );
    assert_eq!(top_stack_count(&runner), Some(0));
    assert_eq!(counters_on(&runner, bottle, &counter("elixir")), 3);

    drive(
        &mut runner,
        priority_with_empty_stack,
        &mut Answers::default(),
    );
    assert_eq!(runner.life(P0), life_before);
    assert!(payment_ownership_is_clear(&runner));
}

// ---------------------------------------------------------------------------
// C4 — hostiles: the amount belongs to its own activation
// ---------------------------------------------------------------------------

/// C4a: two Vish Kal activations stacked; the top one removed 0 counters and
/// shrinks nothing, the bottom one removed 3.
#[test]
fn stacked_activations_each_read_their_own_removed_count() {
    let mut scenario = main_phase();
    let vish = scenario
        .add_creature_from_oracle(P0, "Vish Kal, Blood Arbiter", 5, 5, VISH_KAL)
        .id();
    scenario.with_counter(vish, CounterType::Plus1Plus1, 3);
    let target = add_victim(&mut scenario, "Target", 5, 5);
    let mut runner = scenario.build();
    let index = counter_cost_ability(&runner, vish, 0);

    activate(&mut runner, vish, index);
    drive(
        &mut runner,
        priority_with_stack(1),
        &mut Answers::default().object(target),
    );
    activate(&mut runner, vish, index);
    drive(
        &mut runner,
        priority_with_stack(2),
        &mut Answers::default().object(target),
    );
    assert_eq!(counters_on(&runner, vish, &CounterType::Plus1Plus1), 0);

    drive(&mut runner, priority_with_stack(1), &mut Answers::default());
    assert_eq!(
        (power(&runner, target), toughness(&runner, target)),
        (5, 5),
        "the second activation removed no counters"
    );
    drive(
        &mut runner,
        priority_with_empty_stack,
        &mut Answers::default(),
    );
    assert_eq!((power(&runner, target), toughness(&runner, target)), (2, 2));
}

/// C4b: a remove-all activation with no counters reads 0, not the previous 3.
#[test]
fn a_later_activation_with_no_counters_reads_zero_not_a_stale_amount() {
    let mut scenario = main_phase();
    let amulet = scenario
        .add_artifact_from_oracle(P0, "Relic Amulet", RELIC_AMULET)
        .id();
    scenario.with_counter(amulet, counter("charge"), 3);
    let first = add_victim(&mut scenario, "First Victim", 1, 5);
    let second = add_victim(&mut scenario, "Second Victim", 1, 5);
    add_mana(&mut scenario, &colorless(4));
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, amulet, 0, Answers::default().object(first));
    assert_eq!(
        damage(&runner, first),
        3,
        "reach: the first activation dealt 3"
    );

    runner.state_mut().objects.get_mut(&amulet).unwrap().tapped = false;
    activate_and_resolve(&mut runner, amulet, 0, Answers::default().object(second));
    assert_eq!(damage(&runner, second), 0);
}

/// C4d: Geistflame Reservoir, chosen count 2 of 3 — "that much" is 2.
#[test]
fn geistflame_reservoir_chosen_count_reads_that_much() {
    let mut scenario = main_phase();
    let reservoir = scenario
        .add_artifact_from_oracle(P0, "Geistflame Reservoir", GEISTFLAME_RESERVOIR)
        .id();
    scenario.with_counter(reservoir, counter("charge"), 3);
    let victim = add_victim(&mut scenario, "Victim", 1, 10);
    add_mana(&mut scenario, &[ManaType::Colorless, ManaType::Red]);
    let mut runner = scenario.build();

    activate_and_resolve(
        &mut runner,
        reservoir,
        0,
        Answers {
            pay_amount: Some(2),
            ..Answers::x(2)
        }
        .object(victim),
    );

    assert_eq!(counters_on(&runner, reservoir, &counter("charge")), 1);
    assert_eq!(damage(&runner, victim), 2);
}

/// C4d: The Astonishing Ant-Man, chosen count 2 — "that many" Insects.
#[test]
fn ant_man_chosen_count_creates_that_many_insects() {
    let mut scenario = main_phase();
    let ant_man = scenario
        .add_creature_from_oracle(P0, "The Astonishing Ant-Man", 1, 1, ANT_MAN)
        .id();
    scenario.with_counter(ant_man, CounterType::Plus1Plus1, 3);
    add_mana(
        &mut scenario,
        &[ManaType::Colorless, ManaType::Colorless, ManaType::Green],
    );
    let mut runner = scenario.build();

    activate_and_resolve(
        &mut runner,
        ant_man,
        0,
        Answers {
            pay_amount: Some(2),
            ..Answers::x(2)
        },
    );

    assert_eq!(counters_on(&runner, ant_man, &CounterType::Plus1Plus1), 1);
    assert_eq!(battlefield_named(&runner, "Insect").len(), 2);
}

/// C4d + C4c: Rasputin, chosen count 2 — "that much {C}" is {C}{C}.
#[test]
fn rasputin_chosen_count_adds_that_much_colorless() {
    let mut scenario = main_phase();
    let rasputin = scenario
        .add_creature_from_oracle(P0, "Rasputin, the Oneiromancer", 4, 1, RASPUTIN)
        .id();
    scenario.with_counter(rasputin, counter("dream"), 3);
    let mut runner = scenario.build();

    activate_and_resolve(
        &mut runner,
        rasputin,
        0,
        Answers {
            pay_amount: Some(2),
            ..Answers::x(2)
        },
    );

    assert_eq!(counters_on(&runner, rasputin, &counter("dream")), 1);
    assert_eq!(pool(&runner, ManaType::Colorless), 2);
    assert!(
        payment_ownership_is_clear(&runner),
        "C4c: no payment state outlives the mana ability"
    );
}

// ---------------------------------------------------------------------------
// C5 — siblings: no counter cost / no reader
// ---------------------------------------------------------------------------

/// C5a: counters removed by an EFFECT keep reading the effect's own count.
#[test]
fn effect_path_removal_still_reads_its_own_count() {
    let mut scenario = main_phase();
    let relic = scenario
        .add_artifact_from_oracle(P0, "Control Relic", EFFECT_PATH_CONTROL)
        .id();
    scenario.with_counter(relic, counter("charge"), 3);
    let mut runner = scenario.build();
    let life_before = runner.life(P0);

    activate(&mut runner, relic, 0);
    drive(
        &mut runner,
        priority_with_empty_stack,
        &mut Answers::default(),
    );

    assert_eq!(counters_on(&runner, relic, &counter("charge")), 0);
    assert_eq!(runner.life(P0) - life_before, 3);
    assert!(payment_ownership_is_clear(&runner));
}

/// C5b: a fixed counter cost with no reader creates exactly one Knight.
#[test]
fn rasputin_fixed_counter_cost_creates_one_knight() {
    let mut scenario = main_phase();
    let rasputin = scenario
        .add_creature_from_oracle(P0, "Rasputin, the Oneiromancer", 4, 1, RASPUTIN)
        .id();
    scenario.with_counter(rasputin, counter("dream"), 3);
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, rasputin, 1, Answers::default());

    assert_eq!(counters_on(&runner, rasputin, &counter("dream")), 2);
    assert_eq!(battlefield_named(&runner, "Knight").len(), 1);
}

// ---------------------------------------------------------------------------
// C6 — Sage of Hours (CR 107.1 + CR 500.7)
// ---------------------------------------------------------------------------

fn sage_extra_turns(counters: u32) -> usize {
    let mut scenario = main_phase();
    let sage = scenario
        .add_creature_from_oracle(P0, "Sage of Hours", 1, 1, SAGE_OF_HOURS)
        .id();
    scenario.with_counter(sage, CounterType::Plus1Plus1, counters);
    let mut runner = scenario.build();

    activate_and_resolve(&mut runner, sage, 0, Answers::default());

    assert_eq!(counters_on(&runner, sage, &CounterType::Plus1Plus1), 0);
    runner.state().extra_turns.len()
}

/// C6: one extra turn for each complete group of five counters removed.
#[test]
fn sage_of_hours_takes_an_extra_turn_per_five_counters_removed() {
    assert_eq!(sage_extra_turns(5), 1);
    assert_eq!(sage_extra_turns(4), 0);
    assert_eq!(sage_extra_turns(12), 2);
}

// ---------------------------------------------------------------------------
// C7 — copies (CR 707.10)
// ---------------------------------------------------------------------------

/// C7a: Relic Amulet copied by Rings of Brighthearth — the copy deals the
/// original's 3, then the original deals 3 (6 total).
///
/// A remove-ALL cost involves no player choice, so the copier rulings' "the
/// copy uses that same information" does not literally name it. The copy still
/// reads 3: CR 707.10 copies "all decisions made for it, including … additional
/// or alternative costs", and the number of counters removed is fixed when the
/// cost is paid during activation (CR 602.2b + CR 601.2h), not on resolution, so
/// it is not a "choice normally made on resolution" that the copy re-makes. The
/// copy cannot pay the cost again (it isn't activated, CR 707.10), so its "that
/// much" is the amount bound to the activation it copies — the same reading
/// Lithoform Engine's ruling gives a copied spell's paid costs ("effects based on
/// any alternative or additional costs that were paid for the original spell are
/// copied as though those same costs were paid for the copy").
#[test]
fn rings_copy_of_relic_amulet_deals_the_originals_amount() {
    let mut scenario = main_phase();
    let rings = scenario
        .add_artifact_from_oracle(P0, "Rings of Brighthearth", RINGS_OF_BRIGHTHEARTH)
        .id();
    let amulet = scenario
        .add_artifact_from_oracle(P0, "Relic Amulet", RELIC_AMULET)
        .id();
    scenario.with_counter(amulet, counter("charge"), 3);
    let victim = add_victim(&mut scenario, "Victim", 1, 10);
    add_mana(&mut scenario, &colorless(4));
    let mut runner = scenario.build();
    let mut answers = Answers {
        copier_last: Some(rings),
        ..Answers::default()
    }
    .object(victim);

    let index = counter_cost_ability(&runner, amulet, 0);
    activate(&mut runner, amulet, index);
    drive(&mut runner, priority_with_no_trigger_waiting, &mut answers);
    assert_eq!(
        runner.state().stack.len(),
        2,
        "reach: the original and its copy wait on the stack"
    );
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert_eq!(
        damage(&runner, victim),
        3,
        "the copy deals the original's 3"
    );
    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_eq!(damage(&runner, victim), 6, "the original deals 3 more");
}

/// C7b: Geistflame Reservoir (2 of 3) copied by Rings — the copy's "that much"
/// uses the original's chosen count (CR 707.10; the copier rulings' "a number of
/// counters to remove, the copy uses that same information"): 2, then 2 more.
#[test]
fn rings_copy_of_geistflame_reservoir_deals_the_chosen_amount() {
    let mut scenario = main_phase();
    let rings = scenario
        .add_artifact_from_oracle(P0, "Rings of Brighthearth", RINGS_OF_BRIGHTHEARTH)
        .id();
    let reservoir = scenario
        .add_artifact_from_oracle(P0, "Geistflame Reservoir", GEISTFLAME_RESERVOIR)
        .id();
    scenario.with_counter(reservoir, counter("charge"), 3);
    let victim = add_victim(&mut scenario, "Victim", 1, 10);
    let mut mana = colorless(3);
    mana.push(ManaType::Red);
    add_mana(&mut scenario, &mana);
    let mut runner = scenario.build();
    let mut answers = Answers {
        copier_last: Some(rings),
        pay_amount: Some(2),
        ..Answers::x(2)
    }
    .object(victim);

    let index = counter_cost_ability(&runner, reservoir, 0);
    activate(&mut runner, reservoir, index);
    drive(&mut runner, priority_with_no_trigger_waiting, &mut answers);
    assert_eq!(
        runner.state().stack.len(),
        2,
        "reach: the original and its copy wait on the stack"
    );
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert_eq!(
        damage(&runner, victim),
        2,
        "the copy deals the original's 2"
    );
    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_eq!(damage(&runner, victim), 4, "the original deals 2 more");
    assert_eq!(counters_on(&runner, reservoir, &counter("charge")), 1);
}

fn baku_with_bracers(oracle: &str) -> (GameRunner, ObjectId) {
    let mut scenario = main_phase();
    let bracers = scenario
        .add_creature(P0, "Illusionist's Bracers", 0, 0)
        .as_artifact()
        .with_subtypes(vec!["Equipment"])
        .from_oracle_text(ILLUSIONISTS_BRACERS)
        .id();
    let baku = scenario
        .add_creature_from_oracle(P0, "Blademane Baku", 1, 1, oracle)
        .id();
    scenario.with_counter(baku, counter("ki"), 3);
    add_mana(&mut scenario, &colorless(1));
    let mut runner = scenario.build();
    attach_to(runner.state_mut(), bracers, baku);
    (runner, baku)
}

/// C7c: Blademane Baku (X = 2) copied by Illusionist's Bracers — the copy uses
/// the original's 2 counters removed (CR 707.10; the copier rulings) and gives
/// +4/+0, then the original gives +4/+0: power 1 → 5 → 9, and only 2 ki counters
/// leave (the copy pays no cost).
#[test]
fn bracers_copy_of_blademane_baku_uses_the_originals_count() {
    let (mut runner, baku) = baku_with_bracers(BLADEMANE_BAKU);
    let mut answers = Answers::x(2);

    let index = counter_cost_ability(&runner, baku, 0);
    activate(&mut runner, baku, index);
    drive(&mut runner, priority_with_no_trigger_waiting, &mut answers);
    assert_eq!(runner.state().stack.len(), 2, "reach: original and copy");
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert_eq!(power(&runner, baku), 5, "the copy gives +4/+0");
    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_eq!(power(&runner, baku), 9, "the original gives +4/+0 more");
    assert_eq!(counters_on(&runner, baku, &counter("ki")), 1);
}

/// C7d: a reader of X itself on a copy still reads the copied X (+2 each).
#[test]
fn bracers_copy_of_an_x_reader_keeps_x() {
    let (mut runner, baku) = baku_with_bracers(X_READER);
    let mut answers = Answers::x(2);

    let index = counter_cost_ability(&runner, baku, 0);
    activate(&mut runner, baku, index);
    drive(&mut runner, priority_with_no_trigger_waiting, &mut answers);
    assert_eq!(runner.state().stack.len(), 2, "reach: original and copy");
    drive(&mut runner, priority_with_stack(1), &mut answers);
    assert_eq!(power(&runner, baku), 3, "the copy gives +X/+0");
    drive(&mut runner, priority_with_empty_stack, &mut answers);
    assert_eq!(power(&runner, baku), 5);
}

// ---------------------------------------------------------------------------
// P — parser shapes (U3)
// ---------------------------------------------------------------------------

fn parsed_json(oracle: &str) -> (String, String) {
    let mut scenario = main_phase();
    let id = scenario
        .add_creature_from_oracle(P0, "Shape Probe", 1, 1, oracle)
        .id();
    let runner = scenario.build();
    let obj = &runner.state().objects[&id];
    (
        serde_json::to_string(&*obj.abilities).unwrap(),
        format!("{:?}", obj.static_definitions.as_slice()),
    )
}

fn previous_effect_amount() -> QuantityExpr {
    QuantityExpr::Ref {
        qty: QuantityRef::PreviousEffectAmount {
            channel: DamageChannel::Total,
            aggregate: AggregateFunction::Sum,
        },
    }
}

/// P1: Sage of Hours repeats once per complete group of five counters removed.
#[test]
fn sage_of_hours_parses_to_groups_of_five() {
    let mut scenario = main_phase();
    let sage = scenario
        .add_creature_from_oracle(P0, "Sage of Hours", 1, 1, SAGE_OF_HOURS)
        .id();
    let runner = scenario.build();
    let def = &runner.state().objects[&sage].abilities[0];
    assert!(
        !serde_json::to_string(def)
            .unwrap()
            .contains("Unimplemented"),
        "reach: the ability is fully parsed"
    );
    assert_eq!(
        def.repeat_for,
        Some(QuantityExpr::DivideRounded {
            inner: Box::new(previous_effect_amount()),
            divisor: 5,
            rounding: RoundingMode::Down,
        })
    );
}

/// P2: "for each <N ≥ 2> …" never reads the bare count — an expression-level
/// consumer divides it, a reference-only consumer fails honestly.
#[test]
fn for_each_group_of_n_never_reads_the_bare_count() {
    // Reference-only consumers of "counters removed this way" with a group size.
    for oracle in [
        "{T}, Remove all +1/+1 counters from this creature: Add {C} for each two counters removed this way.",
        "{T}, Remove all +1/+1 counters from this creature: Create a 1/1 colorless Thopter artifact creature token with flying for each two counters removed this way.",
        "{T}, Remove all +1/+1 counters from this creature: Target creature gets -1/-1 until end of turn for each two +1/+1 counters removed this way.",
        "{T}, Remove all +1/+1 counters from this creature: You gain 1 life for each two counters removed this way.",
        "{T}: You gain 1 life for each two creatures you control.",
        "{T}: Draw a card for each two cards in your hand.",
        "{T}: For each two creatures you control, draw a card.",
    ] {
        let (abilities, _) = parsed_json(oracle);
        let divided = abilities.contains("\"DivideRounded\"") && abilities.contains("\"divisor\":2");
        let unsupported = abilities.contains("Unimplemented") || abilities == "[]";
        assert!(
            divided || unsupported,
            "{oracle}: neither divided nor unsupported: {abilities}"
        );
        assert!(
            divided || !abilities.contains("PreviousEffectAmount"),
            "{oracle}: reads the bare count: {abilities}"
        );
    }

    // "the number of five counters removed this way" is not a grouping
    // construct: the quantity declines honestly as an unparsed quantity (not a
    // bare count, a division, or another "the number of …" count).
    let (abilities, _) = parsed_json(
        "{T}, Remove all +1/+1 counters from this creature: It deals damage to any target equal to the number of five counters removed this way.",
    );
    let abilities_json: serde_json::Value = serde_json::from_str(&abilities).unwrap();
    assert_eq!(
        abilities_json.as_array().map(Vec::len),
        Some(1),
        "reach: THENUM5 parses to one ability: {abilities}"
    );
    assert_eq!(
        (
            &abilities_json[0]["effect"]["type"],
            &abilities_json[0]["effect"]["name"],
        ),
        (
            &serde_json::json!("Unimplemented"),
            &serde_json::json!("unparsed_quantity"),
        ),
        "THENUM5 declines honestly: {abilities}"
    );
    assert!(
        !abilities.contains("PreviousEffectAmount") && !abilities.contains("DivideRounded"),
        "THENUM5 reads no count: {abilities}"
    );

    // A static consumer divides: +1/+1 per complete pair of creatures.
    let (_, statics) = parsed_json("This creature gets +1/+1 for each two creatures you control.");
    let pairs = "value: DivideRounded { inner: Ref { qty: ObjectCount { filter: Typed(TypedFilter { type_filters: [Creature], controller: Some(You), properties: [] }) } }, divisor: 2, rounding: Down }";
    assert!(
        statics.contains(&format!(
            "modifications: [AddDynamicPower {{ {pairs} }}, AddDynamicToughness {{ {pairs} }}]"
        )),
        "STATIC2 divides on both power and toughness: {statics}"
    );

    // N = 1 is unchanged.
    let (abilities, _) =
        parsed_json("{T}: You gain 1 life for each 1 life your opponents have lost this turn.");
    assert!(
        !abilities.contains("Unimplemented") && !abilities.contains("DivideRounded"),
        "ONE_LIFE parses as before: {abilities}"
    );
}
