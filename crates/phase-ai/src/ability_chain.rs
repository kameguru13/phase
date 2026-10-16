//! Shared helpers for walking an `AbilityDefinition`'s effect tree.
//!
//! `AbilityDefinition` composes a primary `effect` with an optional
//! `sub_ability` that itself has an `effect` plus another optional
//! `sub_ability`, forming a single-linked list. Feature detectors and
//! policies both need to classify the *set* of effects produced by an
//! ability (e.g., "does this ability both search the library and put a
//! land onto the battlefield?"), so they collect the chain into a flat
//! slice and iterate with `matches!`.
//!
//! Two branches of that definition are *conditional* rather than part of the
//! unconditional chain: `else_ability` (the CR 608.2c "Otherwise, ..." leg) and
//! `mode_abilities` (the CR 700.2 modal alternatives). Whether they belong in
//! the walk depends on the question being asked, which is what [`AbilityScope`]
//! names — see its docs. [`collect_scoped_effects`] is the single authority for
//! both walks; `collect_chain_effects` is the unconditional shorthand.
//!
//! A created-token placement is the one cross-effect reading this module owns:
//! a `PutCounter { target: LastCreated }` that follows an `Effect::Token` in
//! the same unconditional chain puts its counters on the token(s) that chain
//! just created, so a consumer reads it as part of that token's creation, the
//! way it reads `Token { enter_with_counters }`.
//! [`created_token_placement_source`] answers the pairing question and
//! [`AbilityWalk::without_created_token_placements`] is the filtering view
//! every consumer uses; neither ever pairs across abilities or into an
//! `else_ability` / mode branch.
//!
//! Keep this module small — it is a single building block shared across
//! `features/*` and `policies/*`.

use engine::types::ability::{AbilityDefinition, Effect, TargetFilter};

/// Which part of an ability tree a classification question is asking about.
///
/// The distinction is a decision-boundary one, not a convenience one:
///
/// * [`AbilityScope::Unconditional`] answers *"does resolving this ability as
///   already announced produce effect X?"* — the walk a LIVE per-action policy
///   needs. CR 601.2b makes mode selection a distinct step of announcing a
///   spell, so at `CastSpell` time no mode has been chosen yet and a modal
///   branch must not be credited to the cast.
/// * [`AbilityScope::Potential`] answers *"can this card ever produce effect
///   X?"* — the walk DECK-TIME detection needs, where every branch the card
///   could take is in scope.
///
/// A typed scope rather than a `bool` so each call site states which question
/// it is asking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AbilityScope {
    /// `effect` plus the `sub_ability` chain — everything that happens no
    /// matter which branch or mode is taken.
    Unconditional,
    /// The full tree: `Unconditional` plus the `else_ability` branch (CR
    /// 608.2c) and every entry of `mode_abilities` (CR 700.2), recursively.
    Potential,
}

/// Walk an ability tree at `scope`, returning borrowed effect references in
/// traversal order.
///
/// This is the mode-aware authority both deck-time detection and live policy
/// classification share; they differ only in the `scope` they pass, so the two
/// can never drift apart on which branches exist.
pub(crate) fn collect_scoped_effects(
    ability: &AbilityDefinition,
    scope: AbilityScope,
) -> Vec<&Effect> {
    let mut effects: Vec<&Effect> = Vec::new();
    push_scoped_effects(ability, scope, &mut effects);
    effects
}

fn push_scoped_effects<'a>(
    ability: &'a AbilityDefinition,
    scope: AbilityScope,
    out: &mut Vec<&'a Effect>,
) {
    out.push(&ability.effect);
    if let Some(sub) = &ability.sub_ability {
        push_scoped_effects(sub, scope, out);
    }
    if scope == AbilityScope::Unconditional {
        return;
    }
    // CR 608.2c: the "Otherwise, ..." leg is one of two mutually exclusive
    // outcomes, so it is potential rather than unconditional.
    if let Some(other) = &ability.else_ability {
        push_scoped_effects(other, scope, out);
    }
    // CR 700.2: each mode is an alternative the controller may choose.
    for mode in &ability.mode_abilities {
        push_scoped_effects(mode, scope, out);
    }
}

/// Walk `ability.effect` plus each `sub_ability.effect` in turn, returning
/// borrowed references in chain order. Shorthand for
/// [`collect_scoped_effects`] at [`AbilityScope::Unconditional`].
pub(crate) fn collect_chain_effects(ability: &AbilityDefinition) -> Vec<&Effect> {
    collect_scoped_effects(ability, AbilityScope::Unconditional)
}

/// CR 608.2c + CR 111.1: an ability's instructions resolve in order, so a
/// `PutCounter { target: LastCreated }` that follows an `Effect::Token` in the
/// same unconditional chain puts its counters on the token(s) that chain just
/// created (CR 122.6: counters put on an object after it entered). Returns the
/// chain index of the nearest preceding `Token`. Counter-type agnostic.
///
/// `chain` must be one ability's unconditional chain
/// ([`collect_chain_effects`]), never a concatenation of several abilities'
/// walks or a walk that includes `else_ability` / mode branches. A placement
/// with no preceding `Token` (a Conjure, Manifest or copy-token publisher)
/// returns `None`, so every consumer keeps its existing reading of it.
pub(crate) fn created_token_placement_source(chain: &[&Effect], index: usize) -> Option<usize> {
    match chain.get(index)? {
        Effect::PutCounter {
            target: TargetFilter::LastCreated,
            ..
        } => chain[..index]
            .iter()
            .rposition(|effect| matches!(effect, Effect::Token { .. })),
        _ => None,
    }
}

/// One ability's effects as a consumer walks them, plus the ability they came
/// from. The ability's unconditional chain, the only sequence the created-token
/// predicate reads, is built on demand, never at construction.
pub(crate) struct AbilityWalk<'a> {
    ability: Option<&'a AbilityDefinition>,
    effects: Vec<&'a Effect>,
}

impl<'a> AbilityWalk<'a> {
    /// `effects` must be a walk of `ability` (any of the existing walkers), so
    /// that each entry is a node of `ability`'s tree and identity comparison
    /// against the chain is meaningful.
    pub(crate) fn of(ability: &'a AbilityDefinition, effects: Vec<&'a Effect>) -> Self {
        Self {
            ability: Some(ability),
            effects,
        }
    }

    /// The unconditional chain as the walk ([`collect_chain_effects`]).
    pub(crate) fn unconditional(ability: &'a AbilityDefinition) -> Self {
        Self::of(ability, collect_chain_effects(ability))
    }

    /// A walk with no `AbilityDefinition` behind it (a `ResolvedAbility` walk):
    /// `ability` is `None`, so the conservative default holds for every entry.
    pub(crate) fn unpaired(effects: Vec<&'a Effect>) -> Self {
        Self {
            ability: None,
            effects,
        }
    }

    /// The unconditional chain, built on each call by [`collect_chain_effects`];
    /// empty when there is no ability behind the walk.
    pub(crate) fn chain(&self) -> Vec<&'a Effect> {
        self.ability.map(collect_chain_effects).unwrap_or_default()
    }

    pub(crate) fn effects(&self) -> &[&'a Effect] {
        &self.effects
    }

    pub(crate) fn into_effects(self) -> Vec<&'a Effect> {
        self.effects
    }

    /// The filtering view: this walk without its created-token placements. An
    /// entry is dropped iff it is (by `std::ptr::eq`) a chain entry whose
    /// [`created_token_placement_source`] is `Some`. A placement reached only
    /// through an `else_ability` or mode branch is not a chain entry, so it is
    /// kept. Builds the chain once per call.
    pub(crate) fn without_created_token_placements(&self) -> Vec<&'a Effect> {
        let chain = self.chain();
        let placements: Vec<&Effect> = (0..chain.len())
            .filter(|&index| created_token_placement_source(&chain, index).is_some())
            .map(|index| chain[index])
            .collect();
        self.effects
            .iter()
            .copied()
            .filter(|effect| {
                !placements
                    .iter()
                    .any(|placement| std::ptr::eq(*placement, *effect))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cast_facts::collect_definition_effects;
    use crate::policies::tests::created_token_placement_parity::{
        effect_chain, last_created_placement, token_effect,
    };
    use engine::types::ability::{
        AbilityCondition, AbilityKind, ConjureCard, ConjureSource, QuantityExpr, SubAbilityLink,
        TypedFilter,
    };
    use engine::types::counter::CounterType;
    use engine::types::zones::Zone;

    fn one() -> QuantityExpr {
        QuantityExpr::Fixed { value: 1 }
    }

    fn token() -> Effect {
        token_effect(0, 0, vec![])
    }

    fn placement() -> Effect {
        last_created_placement(CounterType::Plus1Plus1, one())
    }

    fn put_counter_on(target: TargetFilter) -> Effect {
        Effect::PutCounter {
            counter_type: CounterType::Plus1Plus1,
            count: one(),
            target,
        }
    }

    fn conjure() -> Effect {
        Effect::Conjure {
            cards: vec![ConjureCard {
                source: ConjureSource::Named {
                    name: "Hangarback Walker".to_string(),
                },
                count: one(),
            }],
            destination: Zone::Battlefield,
            tapped: false,
            library_position: None,
            library_players: None,
        }
    }

    fn source_at(effects: &[Effect], index: usize) -> Option<usize> {
        let chain: Vec<&Effect> = effects.iter().collect();
        created_token_placement_source(&chain, index)
    }

    /// CR 608.2c + CR 111.1: the placement pairs with the NEAREST preceding
    /// `Token` of its own chain, whatever sits between them and whatever the
    /// counter type.
    #[test]
    fn placement_pairs_with_the_nearest_preceding_token() {
        assert_eq!(source_at(&[token(), placement()], 1), Some(0));
        assert_eq!(source_at(&[token(), token(), placement()], 2), Some(1));
        assert_eq!(
            source_at(&[Effect::Investigate, token(), placement()], 2),
            Some(1)
        );
        let deathtouch = last_created_placement(CounterType::Generic("deathtouch".into()), one());
        let mill = Effect::Mill {
            count: QuantityExpr::Fixed { value: 2 },
            target: TargetFilter::Controller,
            destination: Zone::Graveyard,
        };
        assert_eq!(source_at(&[token(), mill, deathtouch], 2), Some(0));
    }

    /// Conservative default: anything other than a `LastCreated` placement after
    /// a `Token` keeps today's reading.
    #[test]
    fn non_token_publishers_and_other_targets_do_not_pair() {
        // A Conjure is a `LastCreated` publisher but not a token creation.
        assert_eq!(source_at(&[conjure(), placement()], 1), None);
        assert_eq!(
            source_at(&[token(), put_counter_on(TargetFilter::SelfRef)], 1),
            None
        );
        assert_eq!(
            source_at(
                &[
                    token(),
                    put_counter_on(TargetFilter::Typed(TypedFilter::creature()))
                ],
                1
            ),
            None
        );
        // The token must come first in resolution order.
        assert_eq!(source_at(&[placement(), token()], 0), None);
        // Out of range, and a non-placement index.
        assert_eq!(source_at(&[token(), placement()], 2), None);
        assert_eq!(source_at(&[token(), placement()], 0), None);
    }

    /// The filtering view drops exactly the chain's created-token placements.
    #[test]
    fn filtering_view_drops_only_created_token_placements() {
        let ability = effect_chain(
            AbilityKind::Spell,
            SubAbilityLink::ContinuationStep,
            vec![token(), placement(), put_counter_on(TargetFilter::SelfRef)],
        );
        let walk = AbilityWalk::unconditional(&ability);
        assert_eq!(walk.effects().len(), 3);
        let kept = walk.without_created_token_placements();
        assert_eq!(kept.len(), 2);
        assert!(matches!(kept[0], Effect::Token { .. }));
        assert!(matches!(
            kept[1],
            Effect::PutCounter {
                target: TargetFilter::SelfRef,
                ..
            }
        ));
        // A walk with no ability behind it keeps every entry.
        let unpaired = AbilityWalk::unpaired(collect_chain_effects(&ability));
        assert!(unpaired.chain().is_empty());
        assert_eq!(unpaired.without_created_token_placements().len(), 3);
    }

    /// CR 608.2c: an "otherwise" leg replaces the chain it guards, so a
    /// placement there never resolves after the root `Token`. It is not in the
    /// unconditional chain, and the branch-inclusive walk keeps it.
    #[test]
    fn else_branch_placement_is_not_paired_with_the_root_token() {
        let mut ability = AbilityDefinition::new(AbilityKind::Spell, token());
        ability.condition = Some(AbilityCondition::IsYourTurn);
        ability.else_ability = Some(Box::new(AbilityDefinition::new(
            AbilityKind::Spell,
            placement(),
        )));
        assert_eq!(collect_chain_effects(&ability).len(), 1);
        let walk = AbilityWalk::of(&ability, collect_definition_effects(&ability));
        assert_eq!(walk.effects().len(), 2);
        assert_eq!(walk.without_created_token_placements().len(), 2);

        // Control: the same placement in the root sub chain is paired.
        let mut chained = AbilityDefinition::new(AbilityKind::Spell, token());
        chained.condition = Some(AbilityCondition::IsYourTurn);
        chained.sub_ability = Some(Box::new(AbilityDefinition::new(
            AbilityKind::Spell,
            placement(),
        )));
        let walk = AbilityWalk::of(&chained, collect_definition_effects(&chained));
        assert_eq!(walk.without_created_token_placements().len(), 1);
    }

    /// CR 700.2: modes are alternatives, so a `Token` in one mode never pairs
    /// with a placement in another.
    #[test]
    fn placements_in_another_mode_are_not_paired() {
        let mut ability = AbilityDefinition::new(AbilityKind::Spell, Effect::NoOp);
        ability.mode_abilities = vec![
            AbilityDefinition::new(AbilityKind::Spell, token()),
            AbilityDefinition::new(AbilityKind::Spell, placement()),
        ];
        let walk = AbilityWalk::of(&ability, collect_definition_effects(&ability));
        assert_eq!(walk.effects().len(), 3);
        assert_eq!(walk.without_created_token_placements().len(), 3);
    }
}
