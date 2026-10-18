# Draft replies for PR #10036 (not posted; for Jakob's approval)

## To matthewevans (review at de01d1e8): B1 / B2

Thanks. Both blockers are fixed at a0e023a7.

**B1 (pre-entry population).**
- `BattlefieldEntrantProjection::new` now pins every population-dependent static condition to its value on the board before the entry (`layers::pin_entry_population_conditions`). This covers devotion, artifact and enchantment counts, and zone membership.
- The pin applies to printed, copied and transient statics, each evaluated with its own controller as "you". The entrant's own statics still apply to it.
- Tests in `conditional_entry_gate_projection.rs`:
  - Recommission on Heliod at four pre-entry devotion now gives 0 counters, and at five gives 1.
  - A separate observer row confirms Heliod is still a 5/5 creature after entry, so Elemental Bond draws.
  - Rusted Relic (metalcraft), Starfield of Nyx (including a copied Starfield and a command-zone static) and a stolen entrant counted under its new controller cover the rest of the class.
- The same "before the entry" reading now holds across a multi-object move. Every member's gates are resolved before the first member moves, so co-entrants don't feed each other's conditions (CR 603.6a). The protocol is bumped to 148/130 for the carrier change.

**B2 (keyword-kind on the observation path).**
- `HasKeywordKind` / `WithoutKeywordKind` now go through `keywords::effective_keyword_kind_of(state, obj, held, kind)`.
- Every `filter_inner_for_object` caller declares `HeldObject::{Live, Detached}`. The observation, record and projection doors read the supplied object; live and owner-zone reads keep off-zone grants.
- Tests cover both polarities beside the existing observed-keyword fixture, including a departed entrant.

Also addressed: the `drive_to_empty` completion assertion (L1) and the multi-type rider ("if it's an artifact or creature"), which now folds into an any-of entry gate.

The onset deferral for "gains/gets/becomes" riders (CR 611.2e) moves to a follow-up PR, as noted in the description.

## To CodeRabbit: filter.rs no-observation incarnation fallback

Fixed. With no observation, the live object is used only when its incarnation matches `record.entered_incarnation`; otherwise the zone-change record answers. A zone change always bumps the incarnation (CR 400.7), so a separate battlefield-zone check would add nothing. Test: `entry_observation_absent_rejects_later_incarnation`.

## To CodeRabbit: effects/mod.rs ~1351-1357, entry observations across reducer actions

Checked; no change. Only enters-trigger matching consumes observations, and triggers for an action's events are collected at the end of that same action:
- A paused resolution goes through `collect_triggers_into_deferred` in `engine_priority.rs`.
- A partly-moved multi-object loop collects its segment at the pause point.

At that point nothing later has changed the entrant, so the live fallback reads what an observation would have recorded. We found no scenario with a missed or wrong trigger. (This matches matthewevans' reconciliation.)

## To CodeRabbit: parser-test nitpicks (tests.rs ~87956 / ~88002)

Not applied. On the current parse, the later `PutCounter`'s condition is `RevealedHasCardType`, not `TargetMatchesFilter`, and the token rider is `Unimplemented`, so both suggested assertions would fail. Neither shape comes from this PR. Both tests already carry positive reach guards (the `PutCounter` effect assertion and the sub-ability `expect`), so the negative assertions aren't vacuous.

## To CodeRabbit: C1 / blanket deferral (effects/mod.rs ~4800)

Agreed the deferral should key on the rider's onset. That needs typed onset metadata from lowering. It moves to a dedicated follow-up PR, as matthewevans' reconciliation also classed it as non-blocking. This PR keeps the conservative deferral, which falls back to today's live reading.

## To CodeRabbit: double projection (change_zone.rs)

Kept as documented: two projections arise only for an entrant carrying both gates, which no printed card does.
