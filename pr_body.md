<!-- claude-projects-attribution: {"github_login":"kameguru13"} -->
_Requested by **Jakob** · [project thread](https://claude.ai/code/project/chan_015qLA6HiCz3fdjbq412agV8?thread=cmsg_015qLA6HiCz3fdjbq412agV89JEiYGLC6QLeXMxdLXSTdU)_

## Summary

Before: an activated ability whose cost removes counters read 0 for "for each … counter removed this way", "for each counter removed" and, for a remove-all cost, "that much". Essence Bottle gained 0 life, Molten Hydra and Relic Amulet dealt 0, Geometric Nexus made a 0/0 Fractal that died, Blademane Baku pumped once, the storage lands (Saprazzan Cove and kin) added 0 and the Mana Batteries added only their base mana. About 27 printed cards were affected.

After: the effect reads the number of counters actually removed to pay the cost (CR 602.2b + CR 601.2h, determined once per CR 608.2h), on both the stack route and the mana-ability route (CR 605.3b). Each activation owns its own count, so a mana ability activated mid-payment from the same source never takes another activation's count (R1). A counter removal that waits on a replacement choice pauses the payment and is credited only once the choice settles (CR 616.1 + CR 118.11), never as a seeded 0 (R2). Nested `Composite` costs are counted at any depth, with no shape guard (R3). A mana ability's colour prompt is sized from the same paid count its production uses, so an "any combination" or "any one color" amount that reads the count offers exactly that many units, and an enclosing payment's leftover amount never sizes the prompt (CR 605.3b + CR 608.2d) (R4). A copy of such an ability (Rings of Brighthearth, Illusionist's Bracers) uses the original's count, per CR 707.10 and the copier rulings ("If the activated ability's cost contains a choice, such as … a number of counters to remove, the copy uses that same information"). Sage of Hours now grants one extra turn per five counters removed, not one per counter: "for each <N ≥ 2> …" parses to floor(count / N), and the ref-level recognizers decline a leading group size instead of silently reading the bare count.

How: the activation owns a removed-counter ledger. `pay_ability_cost_for_activation` takes it as an out-parameter, and every counter leg goes through one payer, `pay_counter_cost_removals`, which credits only completed removals. The stack route carries the count in `SpellContext::cost_removed_counter_amount` on the activation's own `ResolvedAbility`, seeded into the existing `last_effect_amount` channel at depth 0 of `resolve_ability_chain` (no new amount channel or `QuantityRef` variant). The mana route carries it in `ManaAbilityCostCursor` → `PendingManaAbility`, and one scoped binding, `with_bound_mana_activation`, applies it (and restores the enclosing `last_effect_amount`) around both the colour-prompt derivation and `produce_mana_from_ability`. `remove_counter_with_replacement` now returns `CounterRemovalOutcome` (`Removed(n)` / `AwaitingReplacementChoice`); a deferred cost removal parks on a new typed cost-move root, `PendingCostMoveResume::CounterCostRemovalPayment` (stack route), or in the mana cursor's `counter_removal` (mana route), and its resume settles the measured count before finishing the remaining removals and cost through a shared tail guarded by a new `ActivationCostGuardSite::CounterCostTail`. The Prevented arm of the replacement handler sends only a cost-root-owned removal to the cost-move drain; every other counter removal keeps its existing path. Geometric Nexus is covered in both its current `Token.enter_with_counters` shape and a Token → `PutCounter{LastCreated}` shape.

Review round 2 (maintainer F1 / F2 at 2d20628): a counter-cost root now resumes only when its whole replacement subtree has retired, wherever that subtree finishes (CR 616.1 + CR 601.2h + CR 602.2b). The replacement records a typed subtree boundary on the root; `counter_cost_root_subtree()` reports it `Absent`, `Unfinished` or `Retired`, and one action-boundary authority, `resume_parked_counter_cost_root_at_action_boundary`, advances an unfinished subtree and resumes a retired root. So answering an interactive child (Scry, a mana-colour choice, a discard, a sacrifice, an unless payment, an SBA) resumes the activation on every route, not only the replacement boundary (F1). The Prevented arm classifies a prevented removal by that subtree, so a prevented child removal finishes its own `CounterRemovals` queue and its rider before the root resumes, and only the root's own removal settles the payment (F2, and CodeRabbit's owner finding). The activation's announcement owns the trigger events its payment produces: one custody authority in `announcement_triggers` stages, commits and claims them (CR 603.2c: one trigger per occurrence; CR 603.3b: they go on the stack with the activation's batch), and paused observer parks filter already-claimed occurrences, so a dies or discard trigger from a child is put on the stack once, above the activation.

Upstream sync: merges main d3f002d, including #9941's activation cost-span custodian. The two designs were reconciled to one authority: the push commits through `commit_activation_announcement`, and #9941's park stand-down, paid-cost commit flag and sacrifice-settle carrier write are kept. An activated mode prompt now counts as a live announcement at action entry (`announcement_live_at_action_entry`; CR 602.2a + CR 700.2a), so a modal activation's targeting and cost triggers survive a later cost prompt (Professor Hojo row).

Protocol: the new cost-move root and fields are serialized, so this bumps the full-game protocol to 153 and the P2P wire protocol to 135. Upstream main is at 151 / 133; 152 / 134 are reserved for #10004, which is meant to land first (if this one lands first, or #10004 takes other numbers, every surface is renumbered). Surfaces: `lobby-broker`, `server-core`, `check-protocol-version.mjs`, the client constants and their handshake tests.

## Files changed

- `crates/engine/src/game/costs.rs`
- `crates/engine/src/game/casting_costs.rs`
- `crates/engine/src/game/casting.rs`
- `crates/engine/src/game/mana_abilities.rs`
- `crates/engine/src/game/effects/counters.rs`
- `crates/engine/src/game/effects/mod.rs`
- `crates/engine/src/game/engine.rs`
- `crates/engine/src/game/engine_replacement.rs`
- `crates/engine/src/game/announcement_triggers.rs`
- `crates/engine/src/game/engine_resolution_choices.rs`, `engine_payment_choices.rs`, `triggers.rs`, `effects/discard.rs`, `effects/copy_spell.rs` (subtree resume and observer custody)
- `crates/engine/src/types/resolution.rs` (replacement subtree boundary)
- `crates/engine/src/game/visibility.rs`
- `crates/engine/src/game/keywords.rs`
- `crates/engine/src/game/planeswalker.rs`
- `crates/engine/src/ai_support/payment_continuation.rs`
- `crates/engine/src/types/ability.rs`
- `crates/engine/src/types/game_state.rs`
- `crates/engine/src/parser/oracle_quantity.rs`
- `crates/engine/src/parser/oracle_nom/quantity.rs`
- `crates/engine/src/parser/oracle_effect/lower.rs`
- Test modules updated for the new payer signature or fields: `crates/engine/src/game/casting_tests.rs`, `crates/engine/src/game/effects/collect_evidence.rs`, `crates/phase-ai/src/search.rs`
- `crates/engine/tests/integration/cost_removed_counter_amount.rs` (new)
- `crates/engine/tests/integration/main.rs`
- `crates/engine/tests/integration/mana_payment_class_rows.rs` (Mage-Ring Network row re-derived: castable `true`, reading `ff/H` by parity with Rasputin)
- `crates/engine/tests/integration/activation_cost_work_census.rs`, `activation_cost_target_settlement.rs` (new `CounterCostTail` guard site), `issue_2862_teferi_loyalty.rs`, `ai_multi_target_selection.rs`
- Protocol 153 / wire 135: `crates/lobby-broker/src/protocol.rs`, `crates/server-core/src/protocol.rs`, `scripts/check-protocol-version.mjs`, `client/src/network/protocol.ts`, `client/src/adapter/ws-adapter.ts`, and the client tests `protocol.test.ts`, `p2p-adapter-multiplayer.test.ts`, `openPhaseSocket.test.ts`

## Track

Developer

## LLM

Model: claude-opus-5-5
Tier: Frontier
Thinking: High

## Implementation method (required)

Method: /engine-implementer

## CR references

CR 602.2b, CR 601.2f, CR 608.2d, CR 601.2g, CR 601.2h, CR 605.3a, CR 605.3b, CR 608.2c, CR 608.2h, CR 608.3e, CR 614.6, CR 616.1, CR 118.3, CR 118.11, CR 118.12, CR 120.3c, CR 122.1, CR 122.2, CR 700.7, CR 702.49a, CR 707.10, CR 107.1, CR 107.1c, CR 107.3a, CR 201.5b, CR 500.7 ; review round 2 adds CR 101.4, CR 117.3c, CR 117.5, CR 118.12a, CR 601.2i, CR 602.2a, CR 603.2, CR 603.2c, CR 603.3, CR 603.3b, CR 603.7b, CR 605.4a, CR 608.2e, CR 700.2a, CR 704.3, CR 733.1 (all grep-verified against MagicCompRules.txt).

## Verification

- [x] Required checks ran clean, or the exact CI-owned alternative is stated below.
- [x] Gate A output below is for the current committed head.
- [x] Final review-impl below is clean for the current committed head.
- [x] Both anchors cite existing analogous code at the same seam.

Head 375a58f merges upstream main d3f002d (b133e67). Run on a clean worktree, no Tilt, `CARGO_INCREMENTAL=0`, at 375a58f:
- `cargo fmt --all --check`; `node scripts/check-protocol-version.mjs`; `./scripts/check-parser-combinators.sh d3f002d` (Gate A PASS, Gate G PASS); `cargo clippy -D warnings` on phase-engine (`--all-targets --features engine/proptest`), server-core and lobby-broker (0 warnings).
- Tests: engine lib 24379 passed / 0 failed / 8 ignored; engine integration 12022 / 0 / 5 (including `cost_removed_counter_amount` 74 / 0 / 0, `announcement_trigger_collection`, `activation_cost_target_settlement`, `delayed_trigger_continuation`, `integration_bending`); phase-ai 2943 / 0 / 21; server-core 564 / 0; lobby-broker 292 / 0.
- Client: `pnpm run type-check`; `pnpm lint` (0 errors); `pnpm vitest run` (9838 passed), which covers the protocol, P2P and socket handshake tests.
- Parser output vs d3f002d with the same pinned MTGJSON data: card data differs only in `sage of hours` (`Ref(PreviousEffectAmount)` → `DivideRounded{Ref(PreviousEffectAmount), 5, Down}`).
- Review round 2 rows (each fix has a row that goes red with that fix reverted, except latent arms noted in the plan): Scry child on the stack and mana routes (F1); a two-entry prevented child with a trailing rider, an unrelated mill root parked, a same-subject child, and the root's own prevented removal (F2); mana-colour, discard, sacrifice, unless-payment (both players' legs) and SBA children; a dies trigger and a discard trigger from a child put on the stack once above the activation; and a modal activation's target trigger surviving its sacrifice prompt (Professor Hojo). The #9941 stand-down row pins where the trigger waits during the pause (custody); the trigger batch is the same either way.

## Gate A

Gate A PASS head=375a58f8189652b46da5a503bfd0f1aa755d90f5 base=d3f002dae86dc6207486331b6e8b7a221fcbe584

## Anchored on

- crates/engine/src/game/costs.rs:707 — `pay_cost_paid_life_amount` cost-paid amount stamped into `last_effect_amount` (same consumer channel)
- crates/engine/src/types/game_state.rs:8349 — `PendingCostMoveResume::ActivationMillPayment`, the typed cost-move root a paused activation cost parks on (same pause/resume seam as `CounterCostRemovalPayment` at :8361)

## Final review-impl

Final review-impl PASS head=375a58f8189652b46da5a503bfd0f1aa755d90f5

## Claimed parse impact

Sage of Hours

## Scope Expansion

Sage of Hours ("For each five counters removed this way, take an extra turn after this one.") parsed as a bare count, dropping "five". With the runtime fix alone it would have granted one extra turn per counter, so the parser fix ships with it.

## Validation Failures

None. Known limits, none reachable by a printed card (no card has a counter-removal replacement, so the subtree paths are exercised by a typed `RemoveCounter` replacement fixture):
1. Non-counter cost roots (`Cast`, `ActivationMillPayment`, a mana root without a counter leg) record no subtree boundary, so a paused post-effect of their own replacement may finish after the root resumes.
2. If the subtree cannot make progress, the root stays parked under Priority; a spell's final part does not consult a parked cost root.
3. An `Err` from `resume_counter_cost_removal_payment` at the priority boundary panics at the existing `finish_with_continuation` `.expect` sites (synthetic only; the mana root already has the same exposure).
4. Resolution-time PayCost and negative-loyalty counter removals keep no root and no count.
5. A `Replace` install evicts an undispatched enclosing drain, so the boundary can read retired early.
6. An unstamped enclosing frame (never dispatched, journal-replayed, or from an old payload) is read by position.
7. `ManaAdded`-mode triggered mana abilities under a live announcement wait in the carrier until the drain (CR 605.4a).
8. Announcement scope: a same-action stager starting after a stood-down span would skip it, and the commit's delayed scan starts at the scope watermark. Both are held by an invariant (a stand-down ends the action on a non-Priority wait), not by construction; the follow-up splits the scope into an inception point and a staged-through point.
9. The owner-based (`activation_announcement_is_live`) and scope-based (`announcement_owned_from`) predicates agree through the shared `begin_action` invariant.
10. Spell-carrier events staged during a mana-ability child are dropped if the spell is cancelled (CR 733.1), as for other spell-carrier events at base.
11. A park while a root's `pending` slot is still empty goes public, which can split the trigger batch (ordering only; nothing is lost or doubled).
12. The automatic counter leg is not marked as a committed cost; only a selected counter cost refuses `CancelCast`.

Pre-existing on main (not changed here): the `LoyaltyActivation` cost-move root has no carrier, so its paused action isn't staged; `resolve_unless_payer` maps `Controller` to the active player; a self-contained `PutCounter` mana cost that waits on a replacement choice hits `unreachable!` in `mana_abilities.rs` (Wall of Roots; separate PR).

## CI Failures

None.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01QXX9iykdoVWSPVTzWcoDWU
