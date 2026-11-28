//! Regression tests for instructions that refer back to looked-at or exiled cards.

use std::collections::BTreeSet;

use engine::game::scenario::{GameRunner, GameScenario, P0, P1};
use engine::types::ability::{
    CastingPermission, Duration, Effect, EffectKind, TargetFilter, TargetRef,
};
use engine::types::actions::GameAction;
use engine::types::card_type::{CoreType, Supertype};
use engine::types::game_state::{
    CastPaymentMode, ManaChoice, ManaChoicePrompt, PersistedGameState, WaitingFor,
};
use engine::types::identifiers::TrackedSetId;
use engine::types::mana::{ManaCost, ManaCostShard, ManaType, ManaUnit};
use engine::types::phase::Phase;
use engine::types::player::PlayerId;
use engine::types::zones::Zone;
use engine::types::ObjectId;

const PLACEMENT_SEEDS: [u64; 8] = [4, 19, 42, 93, 101, 109, 211, 495];
const P2: PlayerId = PlayerId(2);
const SVELLA: &str = "{3}, {T}: Create a colorless snow artifact token named Icy Manalith with \"{T}: Add one mana of any color.\"\n{6}{R}{G}, {T}: Look at the top four cards of your library. You may cast a spell from among them without paying its mana cost. Put the rest on the bottom of your library in a random order.";
const ITERATION: &str = "Look at the top three cards of your library. Put one of them into your hand, put one of them on the bottom of your library, and exile one of them. You may play the exiled card this turn.";
const PRISONERS: &str = "Choose one —\n• Break Their Chains — Destroy target artifact.\n• Interrogate Them — Exile the top three cards of target opponent's library. Choose one of them. Until the end of your next turn, you may play that card, and you may spend mana as though it were mana of any color to cast it.";
const GONTI: &str = "Deathtouch\nWhen Gonti enters, look at the top four cards of target opponent's library, exile one of them face down, then put the rest on the bottom of that library in a random order. You may cast that card for as long as it remains exiled, and mana of any type can be spent to cast that spell.";
const MARVEL: &str = "Whenever a permanent you control is put into a graveyard, you get {E} (an energy counter).\n{T}, Pay six {E}: Look at the top six cards of your library. You may cast a spell from among them without paying its mana cost. Put the rest on the bottom of your library in a random order.";
fn scenario(players: u8, seed: u64) -> GameScenario {
    let mut board = GameScenario::new_n_player(players, seed);
    board.at_phase(Phase::PreCombatMain);
    board
}

fn mana(board: &mut GameScenario, types: impl IntoIterator<Item = ManaType>) {
    board.with_mana_pool(
        P0,
        types
            .into_iter()
            .map(|kind| ManaUnit::new(kind, ObjectId(0), false, vec![]))
            .collect(),
    );
}

fn spell(
    board: &mut GameScenario,
    player: PlayerId,
    name: &str,
    instant: bool,
    mv: u32,
) -> ObjectId {
    board
        .add_spell_to_library_top(player, name, instant)
        .with_mana_cost(ManaCost::generic(mv))
        .from_oracle_text("You gain 1 life.")
        .id()
}

fn land(board: &mut GameScenario, player: PlayerId, name: &str) -> ObjectId {
    board.add_land_to_library_top(player, name).id()
}

fn same_cards(actual: &[ObjectId], expected: &[ObjectId], context: &str) {
    let mut actual = actual.to_vec();
    let mut expected = expected.to_vec();
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected, "{context}");
}

// CR 608.2c: only pass while the stack is still resolving. A priority
// window after cleanup is observable before the selected spell resolves.
fn pass_until_prompt(runner: &mut GameRunner) {
    for _ in 0..24 {
        if !matches!(runner.state().waiting_for, WaitingFor::Priority { .. })
            || runner.state().stack.is_empty()
        {
            return;
        }
        runner
            .act(GameAction::PassPriority)
            .expect("pass during resolution");
    }
    panic!("resolution did not reach a prompt or finish");
}

fn open_optional(runner: &mut GameRunner) {
    if matches!(
        runner.state().waiting_for,
        WaitingFor::OptionalEffectChoice { player: P0, .. }
    ) {
        runner
            .act(GameAction::DecideOptionalEffect { accept: true })
            .expect("open the optional cast window");
    }
}

fn select(runner: &mut GameRunner, cards: Vec<ObjectId>) {
    runner
        .act(GameAction::SelectCards { cards })
        .expect("select the resolution cards");
}

fn assert_priority(runner: &GameRunner, stack_size: usize) {
    assert!(
        matches!(runner.state().waiting_for, WaitingFor::Priority { .. }),
        "expected priority after the chain, got {}",
        runner.state().waiting_for.variant_name()
    );
    assert_eq!(
        runner.state().stack.len(),
        stack_size,
        "unexpected number of spells left on the stack"
    );
}

fn assert_bottom(
    runner: &GameRunner,
    owner: PlayerId,
    prefix: &[ObjectId],
    bottom: &[ObjectId],
) -> Vec<ObjectId> {
    let library: Vec<_> = runner.state().players[owner.0 as usize]
        .library
        .iter()
        .copied()
        .collect();
    assert!(
        library.starts_with(prefix),
        "unseen cards must stay on top: {library:?}, expected prefix {prefix:?}"
    );
    same_cards(
        &library[prefix.len()..],
        bottom,
        "exactly the unchosen cards must reach this library's bottom",
    );
    library[prefix.len()..].to_vec()
}

// CR 608.2g: the accepted spell stays on the stack while the parent
// ability finishes random bottom placement.
fn peek_cast(marvel: bool, accept: bool) {
    let mut orders = BTreeSet::new();
    for seed in PLACEMENT_SEEDS {
        let mut board = scenario(2, seed);
        let source = if marvel {
            board
                .add_artifact_from_oracle(P0, "Aetherworks Marvel", MARVEL)
                .id()
        } else {
            board
                .add_creature_from_oracle(P0, "Svella, Ice Shaper", 2, 4, SVELLA)
                .id()
        };
        if !marvel {
            mana(
                &mut board,
                [ManaType::Colorless; 6]
                    .into_iter()
                    .chain([ManaType::Red, ManaType::Green]),
            );
        }
        let energy_setup = marvel.then(|| {
            let pit = board
                .add_artifact_from_oracle(
                    P0,
                    "Spawning Pit",
                    "Sacrifice a creature: Put a charge counter on this artifact.",
                )
                .id();
            let fodder = [
                "Grizzly Bears",
                "Runeclaw Bear",
                "Bear Cub",
                "Forest Bear",
                "Cylian Elf",
                "Glory Seeker",
            ]
            .map(|name| board.add_creature(P0, name, 2, 2).id());
            (pit, fodder)
        });
        let deeper = spell(&mut board, P0, "Unseen spell", true, 1);
        let first = spell(&mut board, P0, "Looked-at instant", true, 1);
        let second = spell(&mut board, P0, "Looked-at sorcery", false, 2);
        let mut rest: Vec<_> = (0..if marvel { 4 } else { 2 })
            .map(|i| land(&mut board, P0, &format!("Looked-at land {i}")))
            .collect();
        let mut runner = board.build();
        if let Some((pit, fodder)) = energy_setup {
            for creature in fodder {
                runner.activate(pit, 0).pay_with(&[creature]).resolve();
            }
            assert_eq!(runner.state().players[0].energy, 6);
        }

        runner
            .activate(source, if marvel { 0 } else { 1 })
            .stop_at_optional()
            .resolve();
        open_optional(&mut runner);
        let WaitingFor::EffectZoneChoice {
            player: P0,
            zone: Zone::Library,
            cards,
            ..
        } = &runner.state().waiting_for
        else {
            panic!(
                "expected a private library cast choice, got {}",
                runner.state().waiting_for.variant_name()
            );
        };
        same_cards(
            cards,
            &[first, second],
            "only looked-at spells should be offered",
        );
        select(&mut runner, if accept { vec![second] } else { vec![] });
        assert_priority(&runner, usize::from(accept));
        rest.push(first);
        if accept {
            assert_eq!(
                runner.state().objects[&second].zone,
                Zone::Stack,
                "chosen spell must stay on the stack during library cleanup"
            );
        } else {
            rest.push(second);
        }
        orders.insert(assert_bottom(&runner, P0, &[deeper], &rest));
    }
    assert!(
        orders.len() > 1,
        "bottom placement must vary across deterministic seeds"
    );
}

/// CR 608.2g: "You may cast a spell from among them without paying its mana cost."
#[test]
fn svella_accepts_one_looked_at_spell() {
    peek_cast(false, true);
}

/// CR 608.2c: "Put the rest on the bottom of your library in a random order."
#[test]
fn svella_declines_and_bottoms_the_whole_peek() {
    peek_cast(false, false);
}

/// CR 608.2g: "You may cast a spell from among them without paying its mana cost."
#[test]
fn marvel_accepts_one_looked_at_spell() {
    peek_cast(true, true);
}

/// CR 608.2c: "Put the rest on the bottom of your library in a random order."
#[test]
fn marvel_declines_and_bottoms_the_whole_peek() {
    peek_cast(true, false);
}

fn resolve_targeted_etb(runner: &mut GameRunner, permanent: ObjectId, target: PlayerId) {
    runner.cast(permanent).commit();
    pass_until_prompt(runner);
    let WaitingFor::TriggerTargetSelection {
        player: P0,
        target_slots,
        selection,
        ..
    } = &runner.state().waiting_for
    else {
        panic!(
            "expected an ETB target choice, got {}",
            runner.state().waiting_for.variant_name()
        );
    };
    // CR 115.1d: the chosen opponent must be a legal target of the ETB.
    let opponent = TargetRef::Player(target);
    assert!(
        target_slots[selection.current_slot]
            .legal_targets
            .contains(&opponent),
        "requested opponent must be a legal trigger target"
    );
    runner
        .act(GameAction::ChooseTarget {
            target: Some(opponent),
        })
        .expect("choose ETB opponent");
    pass_until_prompt(runner);
}

fn assert_dig(runner: &GameRunner, expected: &[ObjectId]) {
    let WaitingFor::DigChoice {
        player: P0, cards, ..
    } = &runner.state().waiting_for
    else {
        panic!(
            "expected a dig choice, got {}",
            runner.state().waiting_for.variant_name()
        );
    };
    same_cards(
        cards,
        expected,
        "dig must offer exactly these looked-at cards",
    );
}

/// CR 608.2c: "Put one of them into your hand, put one of them on the bottom
/// of your library, and exile one of them. You may play the exiled card this turn."
enum IterationPath {
    Fresh,
    EarlierExile,
    LegacyRestore,
}

fn iteration(library_count: usize, path: IterationPath) {
    let mut board = scenario(2, 42);
    mana(&mut board, [ManaType::Blue, ManaType::Red]);
    let iteration = board
        .add_spell_to_hand_from_oracle(P0, "Expressive Iteration", false, ITERATION)
        .with_mana_cost(ManaCost::Cost {
            generic: 0,
            shards: vec![ManaCostShard::Blue, ManaCostShard::Red],
        })
        .id();
    let deeper = (library_count == 4).then(|| land(&mut board, P0, "Unseen land"));
    let exile = (library_count >= 3).then(|| spell(&mut board, P0, "Remaining spell", false, 1));
    let bottom = (library_count >= 2).then(|| land(&mut board, P0, "Bottom land"));
    let hand = land(&mut board, P0, "Hand land");
    let seen: Vec<_> = [Some(hand), bottom, exile].into_iter().flatten().collect();

    let earlier = if matches!(path, IterationPath::EarlierExile) {
        mana(
            &mut board,
            [
                ManaType::Blue,
                ManaType::Red,
                ManaType::Red,
                ManaType::Colorless,
            ],
        );
        let resolve = board
            .add_spell_to_hand_from_oracle(
                P0,
                "Wrenn's Resolve",
                false,
                "Exile the top two cards of your library. Until the end of your next turn, you may play those cards.",
            )
            .with_mana_cost(ManaCost::Cost {
                generic: 1,
                shards: vec![ManaCostShard::Red],
            })
            .id();
        Some((
            resolve,
            [
                land(&mut board, P0, "Earlier A"),
                land(&mut board, P0, "Earlier B"),
            ],
        ))
    } else {
        None
    };
    let mut runner = board.build();
    let earlier = earlier.map(|(resolve, cards)| {
        runner.cast(resolve).resolve();
        assert_priority(&runner, 0);
        cards.map(|id| {
            let object = &runner.state().objects[&id];
            assert_eq!(object.zone, Zone::Exile);
            assert!(!object.casting_permissions.is_empty());
            assert!(runner
                .state()
                .tracked_object_sets
                .values()
                .any(|set| set.contains(&id)));
            (id, object.casting_permissions.clone())
        })
    });
    runner.cast(iteration).commit();
    pass_until_prompt(&mut runner);
    // A mandatory singleton may resolve without a prompt.
    if library_count > 1 || !matches!(runner.state().waiting_for, WaitingFor::Priority { .. }) {
        assert_dig(&runner, &seen);
        select(&mut runner, vec![hand]);
    }
    if let WaitingFor::EffectZoneChoice {
        player: P0,
        zone: Zone::Library,
        effect_kind: EffectKind::PutAtLibraryPosition,
        cards,
        ..
    } = &runner.state().waiting_for
    {
        same_cards(
            cards,
            &seen[1..],
            "the bottom choice excludes the card in hand",
        );
        if matches!(path, IterationPath::LegacyRestore) {
            runner = restore_legacy_iteration(runner, hand);
        }
        select(&mut runner, vec![bottom.expect("a bottom candidate")]);
        if let Some(exile) = exile.filter(|_| !matches!(path, IterationPath::LegacyRestore)) {
            let WaitingFor::ChooseFromZoneChoice {
                player: P0,
                cards,
                count: 1,
                up_to: false,
                ..
            } = &runner.state().waiting_for
            else {
                panic!("expected the mandatory exile choice");
            };
            assert_eq!(cards, &[exile]);
            select(&mut runner, vec![exile]);
        }
    } else {
        assert!(exile.is_none(), "two candidates require a bottom prompt");
    }
    assert_priority(&runner, 0);
    assert_eq!(
        runner.state().objects[&hand].zone,
        Zone::Hand,
        "first selection must go to hand"
    );
    if let Some(bottom) = bottom {
        let object = &runner.state().objects[&bottom];
        assert_eq!(
            (object.zone, object.casting_permissions.is_empty()),
            (Zone::Library, true),
            "the bottom card must remain in the library without play permission"
        );
    }
    assert!(runner.state().objects[&hand].casting_permissions.is_empty());
    assert_bottom(&runner, P0, deeper.as_slice(), bottom.as_slice());
    let Some(exile) = exile else {
        let expected: Vec<_> = earlier
            .iter()
            .flatten()
            .map(|(id, permissions)| {
                assert_eq!(&runner.state().objects[id].casting_permissions, permissions);
                *id
            })
            .collect();
        let actual: Vec<_> = runner
            .state()
            .objects
            .values()
            .filter(|object| object.zone == Zone::Exile)
            .map(|object| object.id)
            .collect();
        same_cards(&actual, &expected, "Iteration must leave exile unchanged");
        return;
    };
    assert_eq!(
        runner.state().objects[&exile].zone,
        Zone::Exile,
        "third looked-at card must be exiled"
    );
    assert!(
        runner.state().objects[&exile]
            .casting_permissions
            .iter()
            .any(|permission| matches!(
                permission,
                CastingPermission::PlayFromExile {
                    granted_to: P0,
                    duration: Duration::UntilEndOfTurn,
                    ..
                }
            )),
        "the exiled card must receive the play-this-turn permission"
    );
}

fn restore_legacy_iteration(mut runner: GameRunner, hand: ObjectId) -> GameRunner {
    let pending = &mut runner
        .state_mut()
        .active_ability_continuation_frame_mut()
        .expect("bottom choice carries a continuation")
        .pending;
    let chooser = &mut pending.chain;
    assert!(matches!(chooser.effect, Effect::ChooseFromZone { .. }));
    let mut old_exile = chooser.sub_ability.take().expect("exile follows choice");
    let Effect::ChangeZone {
        origin,
        target: TargetFilter::ParentTarget,
        ..
    } = &mut old_exile.effect
    else {
        panic!("expected the selected card's exile step");
    };
    *origin = None;
    old_exile.set_unpinned_targets(vec![TargetRef::Object(hand)]);
    let Effect::GrantCastingPermission { target, .. } = &mut old_exile
        .sub_ability
        .as_mut()
        .expect("grant follows exile")
        .effect
    else {
        panic!("expected play permission");
    };
    *target = TargetFilter::TrackedSet {
        id: TrackedSetId(0),
    };
    pending.chain = old_exile;
    let saved = serde_json::to_vec(&PersistedGameState::capture(runner.state().clone())).unwrap();
    let restored: PersistedGameState = serde_json::from_slice(&saved).unwrap();
    GameRunner::from_state(restored.into_game_state().expect("restore legacy chain"))
}

#[test]
fn expressive_iteration_one_card_goes_to_hand() {
    iteration(1, IterationPath::Fresh);
}

#[test]
fn expressive_iteration_two_cards_go_to_hand_and_bottom() {
    iteration(2, IterationPath::Fresh);
}

#[test]
fn expressive_iteration_three_cards_go_to_hand_bottom_and_exile() {
    iteration(3, IterationPath::Fresh);
}

#[test]
fn expressive_iteration_four_cards_leaves_the_unseen_card_on_top() {
    iteration(4, IterationPath::Fresh);
}

#[test]
fn expressive_iteration_ignores_the_earlier_exile_set() {
    iteration(2, IterationPath::EarlierExile);
}

#[test]
fn expressive_iteration_resumes_a_legacy_bottom_choice() {
    iteration(3, IterationPath::LegacyRestore);
}

/// CR 608.2c: "Exile the top three cards of target opponent's library. Choose one of them."
#[test]
fn prisoners_only_offers_the_newly_exiled_cards_and_grants_the_chosen_one() {
    let mut board = scenario(2, 42);
    let prisoners = board
        .add_spell_to_hand_from_oracle(P0, "You Find Some Prisoners", true, PRISONERS)
        .with_mana_cost(ManaCost::Cost {
            generic: 1,
            shards: vec![ManaCostShard::Red],
        })
        .id();
    let swords = board
        .add_spell_to_hand_from_oracle(
            P0,
            "Swords to Plowshares",
            true,
            "Exile target creature. Its controller gains life equal to its power.",
        )
        .with_mana_cost(ManaCost::Cost {
            generic: 0,
            shards: vec![ManaCostShard::White],
        })
        .id();
    mana(
        &mut board,
        [ManaType::White, ManaType::Red, ManaType::Colorless],
    );
    let older_opponent = board.add_creature(P1, "Grizzly Bears", 2, 2).id();
    let deeper = land(&mut board, P1, "Unseen land");
    let a = land(&mut board, P1, "Just exiled land");
    let b = spell(&mut board, P1, "Just exiled sorcery", false, 1);
    let c = spell(&mut board, P1, "Just exiled instant", true, 1);
    let mut runner = board.build();
    runner
        .cast(swords)
        .target_object(older_opponent)
        .commit()
        .resolve();
    assert_eq!(runner.state().objects[&older_opponent].zone, Zone::Exile);
    runner
        .cast(prisoners)
        .modes(&[1])
        .target_player(P1)
        .commit()
        .stop_at_optional()
        .resolve();
    open_optional(&mut runner);
    // CR 608.2c: "choose one of them" is restricted to the cards exiled by mode 2.
    let cards = match &runner.state().waiting_for {
        WaitingFor::ChooseFromZoneChoice {
            player: P0, cards, ..
        }
        | WaitingFor::EffectZoneChoice {
            player: P0,
            zone: Zone::Exile,
            cards,
            ..
        } => cards,
        other => panic!(
            "expected a choice from the newly exiled cards, got {}",
            other.variant_name()
        ),
    };
    same_cards(
        cards,
        &[a, b, c],
        "only the three newly exiled cards are offered",
    );
    select(&mut runner, vec![b]);
    open_optional(&mut runner);
    assert_priority(&runner, 0);
    assert_eq!(runner.state().objects[&deeper].zone, Zone::Library);
    for card in [a, b, c, older_opponent] {
        let object = &runner.state().objects[&card];
        assert_eq!(object.zone, Zone::Exile);
        assert_eq!(
            object
                .casting_permissions
                .iter()
                .any(|p| matches!(p, CastingPermission::PlayFromExile { granted_to: P0, .. })),
            card == b,
            "only P0's chosen card gets play permission: {card:?}"
        );
    }
}

/// CR 406.3 + CR 608.2c: "exile one of them face down, then put the rest on the
/// bottom of that library in a random order."
#[test]
fn gonti_choices_and_bottom_placement_belong_to_the_second_opponent() {
    let mut orders = BTreeSet::new();
    for seed in PLACEMENT_SEEDS {
        let mut board = scenario(3, seed);
        let gonti = board
            .add_creature_to_hand(P0, "Gonti, Lord of Luxury", 2, 3)
            .with_mana_cost(ManaCost::Cost {
                generic: 2,
                shards: vec![ManaCostShard::Black; 2],
            })
            .from_oracle_text_with_keywords(&["Deathtouch"], GONTI)
            .id();
        mana(
            &mut board,
            [ManaType::Black; 2]
                .into_iter()
                .chain([ManaType::Colorless; 2]),
        );
        let own = land(&mut board, P0, "P0 land");
        let first_opponent = land(&mut board, P1, "P1 land");
        let deeper = land(&mut board, P2, "P2 unseen land");
        let a = land(&mut board, P2, "P2 land A");
        let b = spell(&mut board, P2, "P2 sorcery B", false, 1);
        let c = land(&mut board, P2, "P2 land C");
        let d = land(&mut board, P2, "P2 land D");
        let mut runner = board.build();
        resolve_targeted_etb(&mut runner, gonti, P2);
        open_optional(&mut runner);
        assert_dig(&runner, &[a, b, c, d]);
        select(&mut runner, vec![b]);
        open_optional(&mut runner);
        assert_priority(&runner, 0);
        // CR 406.3 + CR 608.2c: the chosen card is exiled face down and
        // the remaining looked-at cards go to the targeted player's library.
        let object = &runner.state().objects[&b];
        assert_eq!(object.zone, Zone::Exile, "Gonti exiles the selected card");
        assert!(object.face_down, "Gonti exiles the selected card face down");
        orders.insert(assert_bottom(&runner, P2, &[deeper], &[a, c, d]));
        assert_bottom(&runner, P1, &[first_opponent], &[]);
        assert_bottom(&runner, P0, &[own], &[]);
    }
    assert!(
        orders.len() > 1,
        "Gonti's bottom placement must vary across deterministic seeds"
    );
}

/// CR 111.3 + CR 205.4a: "Create a colorless snow artifact token named Icy Manalith".
#[test]
fn svella_creates_a_snow_artifact_token() {
    let mut board = scenario(2, 42);
    let svella = board
        .add_creature_from_oracle(P0, "Svella, Ice Shaper", 2, 4, SVELLA)
        .id();
    mana(&mut board, [ManaType::Colorless; 3]);
    let mut runner = board.build();
    runner.activate(svella, 0).resolve();
    assert_priority(&runner, 0);

    let token = runner
        .state()
        .objects
        .values()
        .find(|object| object.zone == Zone::Battlefield && object.name == "Icy Manalith")
        .expect("Svella creates Icy Manalith on the battlefield");
    assert!(token.is_token);
    assert!(token.card_types.core_types.contains(&CoreType::Artifact));
    assert!(token.card_types.supertypes.contains(&Supertype::Snow));
    // CR 105.2c: A colorless object has no color.
    assert!(token.color.is_empty(), "Icy Manalith must be colorless");

    let token = token.id;
    let pool_before = runner.state().players[0].mana_pool.total();
    let chosen = ManaType::Blue;
    let chosen_before = runner.state().players[0].mana_pool.count_color(chosen);
    assert!(!runner.state().objects[&token].tapped);

    runner
        .act(GameAction::ActivateAbility {
            source_id: token,
            ability_index: 0,
            payment_mode: CastPaymentMode::Auto,
        })
        .expect("activate Icy Manalith's mana ability");
    let WaitingFor::ChooseManaColor {
        player: P0,
        choice: ManaChoicePrompt::SingleColor { options },
        ..
    } = &runner.state().waiting_for
    else {
        panic!("expected Icy Manalith to offer a mana color choice");
    };
    // CR 105.1 + CR 105.4: Any color offers all five colors.
    assert_eq!(options.len(), 5);
    for color in [
        ManaType::White,
        ManaType::Blue,
        ManaType::Black,
        ManaType::Red,
        ManaType::Green,
    ] {
        assert!(options.contains(&color), "missing mana color {color:?}");
    }
    runner
        .act(GameAction::ChooseManaColor {
            choice: ManaChoice::SingleColor(chosen),
            count: 1,
        })
        .expect("choose blue mana");

    // CR 107.5 + CR 106.4 + CR 605.3b: The mana ability taps its source
    // and adds mana immediately, leaving the stack empty.
    assert_priority(&runner, 0);
    assert!(runner.state().objects[&token].tapped);
    let pool = &runner.state().players[0].mana_pool;
    assert_eq!(pool.total(), pool_before + 1);
    assert_eq!(pool.count_color(chosen), chosen_before + 1);
}
