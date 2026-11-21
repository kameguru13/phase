# Task: restrictions created by resolving spells/abilities survive a later "lose all abilities" effect

Worktree: X:/Users/jakob/Documents/dev/phase-wt/resolved-restrictions (detached at upstream main; branch
name pending Jakob's confirmation, proposed engine/resolved-restrictions-survive-ability-removal).
BASE_SHA = 7e2a538799e672af23caca367ba938bc24194d66 (upstream phase-rs/phase main, 2026-11-21).
Budget: standard. Jakob chose (Command Zone #213) to ship this as its own engine PR, ahead of the
Immortal Obligation PR (phase-rs/phase#9608, branch card/immortal-obligation, which will stack on it).

## Defect (found by the Immortal Obligation phase-2 implementation review, measured)

A resolving spell or ability that puts a combat restriction (or goad) on a permanent "for as long as"
/ "this turn" is represented by a transient continuous effect whose modifications graft statics onto
the permanent (`ContinuousModification::AddStaticMode { .. }` and/or `GrantStaticAbility`). At layer 6
the `ContinuousModification::RemoveAllAbilities` arm in crates/engine/src/game/layers.rs (~13310-13330
on BASE) does `obj.static_definitions.clear()` (it already keeps resolution-installed replacements via
`obj.replacement_definitions.retain(|d| d.is_resolution_installed())`, citing CR 613.1f + CR 611.2c).
So a later-timestamped "creatures lose all abilities" effect (Dress Down, Turn to Frog, Humility-style)
erases the restriction.

Measured on the Immortal Obligation branch (50dc4841): cast Immortal Obligation, then Dress Down, by
production casts; the returned creature stays goaded (goad is read off the live TCE by
`goading_players_for_creature_gated`), but its `static_definitions` become [] and
`declare_blockers([(returned, caster's attacker)])` returns Ok — it should be refused.

Rules: a restriction created by a resolving spell is an effect, not an ability of the object
(CR 611.2, CR 611.2c; CR 509.1b / CR 508.1c restrictions; CR 613.1f layer 6 removes abilities only).
Abilities granted by resolving effects ("gains flying until end of turn", "gains '<quoted ability>'")
ARE abilities and must still be removed by a later ability-removal effect in timestamp order
(CR 613.1f, CR 613.7). The fix must keep that distinction. Grep-verify every CR number.

## Cards (verify verbatim Oracle text from data/mtgjson/AtomicCards.json or data/card-data.json)

- Immortal Obligation: "Return target creature card from an opponent's graveyard to the battlefield under their control with a duty counter on it. For as long as that creature has a duty counter on it, it is goaded, can't attack you or a permanent you control, and can't block creatures you control." (its parser support lands in the stacked PR; this PR need not parse it)
- Promise of Loyalty, Craterous Stomp (named by the reviewer as affected), plus whatever the class census finds
  (resolving-effect "target creature can't block/attack [this turn]" etc.).
- Dress Down: "Flash / When this enchantment enters, draw a card. / Creatures lose all abilities. / At the beginning of the end step, sacrifice this enchantment."
- Turn to Frog (verify text) as a second ability-removal witness.
- Related pre-existing, static-line case: Psychic Paper ("Equipped creature has ward {1}, it can't be blocked, ...")
  parses its "can't be blocked" as AddStaticMode{CantBeBlocked} inside a static Continuous def; filed as
  Command Zone follow-up #282. Include it only if the chosen design covers it naturally and the parse
  diff stays honest; otherwise give a one-line reason it is out of scope.

## Acceptance criteria

1. After a later "lose all abilities" effect, a restriction installed by a resolving effect on the
   permanent still applies (block and/or attack declaration refused), while abilities granted by
   resolving effects (keywords, quoted abilities) are still removed. Goad is already preserved; keep it so.
2. Production-path tests (real casts, real declare-attackers/blockers) for at least two printed cards in
   the class that exist and parse on BASE (e.g. Craterous Stomp, Promise of Loyalty), each failing at
   BASE for this reason, with paired positives and a hostile row proving a granted keyword/ability is
   still removed by the same ability-removal effect.
3. Engine-owned fix at one authority (the layer system or the combat consumers), building for the class,
   not per card. No parser change unless strictly required. Grep siblings (other consumers reading
   restriction statics: attack/block legality, CantUntap, CantBeBlocked, etc.) and give each the same
   treatment or a one-line reason.
4. Parse diff vs BASE: none expected (engine-only). Card-data unchanged unless the design says otherwise.
5. CR annotations grep-verified in docs/MagicCompRules.txt; real printed cards with verbatim text in tests.

## Attempt history

New task (loops start at 0). Context: the same bug class at static-line level was maintainer finding
F1 on phase-rs/phase#11229 (fixed there by hosting Burden of Proof's restriction on the Aura; merged).
No scouts run yet.
