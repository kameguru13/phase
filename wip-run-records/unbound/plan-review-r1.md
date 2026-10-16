## Plan review: Unbound Flourishing, plan-r1 (Step 2, ordinary mode with phase-fit context)

**Verdict: not clean.** There are 6 blocking findings: 4 `behavior` and 2 `text`. Most of the plan's architecture holds up. Constraints for the phase planners follow the findings.

Read-only review. I ran no cargo and no probes. Parser files were read at HEAD 6295aa7.

### What I checked and found sound (use these as constraints)
- **Folding the intervening-if into `valid_card` (CR 603.4)** is rules-equivalent.
  - The predicate reads the mana cost or activation cost. That is fixed by CR 601.2f / CR 602.2b. Last-known information (CR 608.2h) answers the resolution recheck the same way, which matches ruling 1.
  - The engine reason also holds. `pending_activation_cost_has_x` (casting_costs.rs:17127) reads `pending_activations`, so a rechecked leaf would fail at resolution.
  - `ability_cost_has_x` only counts `AbilityCost::Mana` legs. So "pay X life", "remove X counters" and loyalty −X are not copied, which is correct per ruling 3.
- **X channels.**
  - Ballista's "enters with X" is a `PutCounter(CostXPaid)` replacement, so it reads `obj.cost_x_paid`.
  - Intrinsic ETB counters read `stack_paid_facts.x_value`, falling back to `cost_x_paid` (stack.rs:2553).
  - Mana value reads `mana_value_with_x(zone, cost_x_paid)`.
  - Writing `cost_x_paid`, `stack_paid_facts[id].x_value` and the spell ability's `chosen_x` together covers every reader.
  - The arithmetic is right: Ballista X=2 doubled gives 4 counters and mana value 8 (CR 202.3e); Krasis X=4 doubled enters with 8.
- **Ruling 4 (cast trigger that reads X).** Krasis's `Variable("X")` has no `chosen_x`. It reads the trigger event's spell `cost_x_paid` live (quantity.rs:5023), so trigger order really changes the result. U4 is required, and V13's numbers (4/4 vs 2/2) are correct.
- **"Copy even if countered" for abilities.**
  - Rings of Brighthearth uses `ParentTarget` with no targets. It reaches `triggering_activation` through the final untargeted fallback in `copy_source_entry` (copy_spell.rs ~743), so U5 fixes Rings too.
  - Both Stifle-class counter sites (counter.rs:115, 488) go through `remove_nonresolving_stack_entry_at`, so recording there catches the countered case.
  - Ability entry ids come from a monotonic counter, so keying by entry id without an incarnation is fine.
- **CR citations.** I grep-verified every cited rule: 107.3a/c/e/i/k/m, 113.7a, 202.3e, 400.7, 601.2f/i, 602.2a/b, 603.2/2a/2h/3b/4, 605.3b, 608.2h, 701.6a, 701.10 (no X sub-rule, as the plan says), 701.10d/e/f, 707.10, 707.10c.

### Blocking findings

**F1 `behavior`: the new `departed_stack_abilities` map would leak hidden information.**
- Plan Step 9 says: "Activated-ability entries are public, so the projection copies the map unchanged." That is false. The viewer filter redacts live activated-ability stack entries:
  - visibility.rs:1293–1296 sets `ability.activation_record = None` for every `StackEntryKind::ActivatedAbility` on the stack.
  - visibility.rs:1701+ redacts each stack and resolving entry's creation-lookback events (`for_each_creation_lookback_event_mut`).
- A countered entry stored in the new map would bypass both redactions and reach clients.
- Required revision:
  - Step 9 and U5 must apply the same redactions to every entry in `departed_stack_abilities`, in `filter_state_for_viewer` and the unseated variant, and in `project_paid_cast_cleanup_authority` if it applies.
  - Remove the "(only if a GameState destructure forces it)" condition on visibility.rs.
  - Add a visibility unit test: a departed ability with `activation_record: Some(..)` comes out `None` in the viewer projection.

**F2 `behavior`: "no protocol-version surfaces" contradicts repo policy.**
- The new `Effect::ModifyAnnouncedX` and `XModification` tags travel inside `GameState` (in ability definitions). `crates/lobby-broker/src/protocol.rs` treats new serialized tags as one-way parse breaks that need a bump. Entry 64(a) did it for `Effect::OpenBoosterPack`; entries 139, 140, 141 and 144 did it for other new tags. HEAD 6295aa7 itself bumped to 144 / wire 126 for `NumberQuality`.
- The new `GameState` field is the same class of change as 139 (`crew_history_this_turn`) and 45, both of which bumped. A peer re-encoding state across a P2P handoff would drop the departed record.
- Required revision:
  - Each phase that changes the serialized shape bumps `PROTOCOL_VERSION`, adds its doc entry, and moves the P2P wire version in lockstep: Phase C (Effect tag) for certain, and Phase B (state field) unless the plan cites a policy entry that exempts additive fields.
  - Files, following 6295aa7's set: `crates/lobby-broker/src/protocol.rs`, `client/src/network/protocol.ts` (`WIRE_PROTOCOL_VERSION`), `scripts/check-protocol-version.mjs` (`EXPECTED_PROTOCOL_VERSION`), `client/src/adapter/ws-adapter.ts`, `client/src/network/__tests__/protocol.test.ts`, `client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`, `client/src/services/__tests__/openPhaseSocket.test.ts`.
  - These go into Sizing (see F5).

**F3 `behavior`: the cast-or-activate splitter breaks once-per-turn abilities (CR 603.2h).**
- Splitting one printed ability into two `TriggerDefinition`s gives each half its own OncePerTurn counter, because OncePerTurn is keyed per `TriggerDefinitionRef`. Two lines in the splitter's class carry this constraint:
  - Bill Potts: "This ability triggers only once each turn."
  - Glava: "Do this only once each turn."
- The plan accepts this as residual 2, relying on those halves "parsing to Unknown" today. That has not been measured, and it leaves a building block that will ship wrong semantics as soon as either half parses.
- Required revision:
  - The splitter declines (returns `None`, leaving the line honestly red) when either lowered half carries a per-ability constraint or once-each-turn instruction.
  - Add a V4 test row: Bill Potts' verbatim text produces no split and stays red, paired with Unbound splitting as the reach guard.
  - Keep the `or_trigger` follow-up as the long-term fix.

**F4 `behavior`: the fold does not say what happens to an intervening-if it cannot parse.**
- Step 2 defines two outcomes: the fold succeeds, or it parses but the subjects mismatch (decline). It does not cover an effect that starts with "if " and that the fold combinator rejects.
- Vazi has exactly that: "Whenever an opponent casts a spell or activates an ability, if mana from a Treasure was spent to cast it or activate it, …". The plan would copy the whole "if … cast it or activate it" into both halves, where "activate it" has no referent on the spell half and "cast it" none on the ability half.
- Required revision:
  - Add the rule: if the effect matches `tag("if ")` and the fold combinator fails, the splitter declines.
  - V4 asserts that Vazi either stays unsplit or has its condition present in each half, never dropped.

**F5 `text`: the Sizing scope list is incomplete.**
- §Sizing "Authored product files" omits:
  - `crates/engine/src/analysis/resource.rs`. It calls `retain_trigger_referenced_departed_spells` at lines 5710–5711, and Step 5 says to call the new sibling "wherever retain_trigger_referenced_departed_spells is called".
  - The protocol files from F2.
- It also marks visibility.rs as conditional, which F1 makes required.
- Replacement for line 123 (`20. crates/engine/src/game/visibility.rs (only if a GameState destructure forces it)`): `20. crates/engine/src/game/visibility.rs (required: viewer redaction of departed ability entries, F1)`
- Append after it:
  - `20a. crates/engine/src/analysis/resource.rs (retain_trigger_referenced_departed_abilities call beside the existing retain call)`
  - `20b–20h. protocol bump set: crates/lobby-broker/src/protocol.rs, scripts/check-protocol-version.mjs, client/src/network/protocol.ts, client/src/adapter/ws-adapter.ts, client/src/network/__tests__/protocol.test.ts, client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts, client/src/services/__tests__/openPhaseSocket.test.ts (per phase that changes wire shape)`
- Replace "**About 22–24 paths, 5 units.**" with "**About 31–33 paths, 5 units; regenerate with the phase-fit counting rule after charter partition.**" Both gates still fire.

**F6 `text`: two rulings the plan claims have no test rows.**
- The Identity row for "that spell's mana cost … contains {X}" names "X only in an **additional** cost → no trigger (ruling 8)". Ruling 7's "copied twice with two Unbounds" is also not covered. Neither has a Verification Matrix row.
- Add after V16:
  - `| V17 | {X} only in an additional cost does not trigger either line (ruling 8) | HasXInManaCost via U1/U2 | runner.cast(..) of a spell whose X is only in an additional cost (verbatim Oracle) | integration :: unbound_ignores_x_in_additional_cost | no Unbound trigger on the stack | Reach guard: V6/V9 positive in the same file | match_spell_cast valid_card |`
  - `| V18 | Two Unbounds copy an X instant twice (ruling 7) | U2 | Stroke X=2 with two Unbounds | integration :: two_unbounds_copy_x_instant_twice | P0 draws 6 | Control: V9 | — |`

### Non-blocking notes (for the phase planners)
1. **CR 733 journal.** `set_stack_object_announced_x` writes persistent state raw: `cost_x_paid`, `stack_paid_facts`, and the stack entry's ability. The journal (`types/resolved_commands.rs`) only replays typed commands. Step 12 leaves the choice between journaling it and documenting an unjournaled seam to the executor. The Phase C plan should make that decision, not defer it. `change_targets.rs` has no journaling, so it is precedent for the unjournaled option.
2. **Code no parse can reach.** No card reaches `ModifyAnnouncedX`'s non-`TriggeringSource` branch or the `ActivatedAbility` arm of `set_stack_object_announced_x`.
   - The ActivatedAbility arm also sits beside a separate X channel, `state.announced_source_x`, which is stamped onto triggers at triggers.rs:17160.
   - Either drop both until a consumer exists, or give each its own test.
3. **Mana-ability residual.** Residual 1 is written `DEFERRED(follow-up)` with no landing phase. In phase-plan mode that row would be rejected, so the charter should assign it or declare it out of scope.
4. **V8's assertion is weak.** "Graveyard card `cost_x_paid` unchanged" barely tells a correct resolver from a broken one. Also assert that no other stack object's X changed, using a second X spell on the stack.
5. **U4 over-prompts.** U4 marks every `Variable("X")` as an event-live read, including triggers whose `chosen_x` was set at activation. That only adds extra ordering prompts, never wrong behavior; Step 18's parity sweep should catch any pinned-census fallout.

### Pre-existing (not blocking)
- Vazi, Keen Negotiator's current card-data parse has `condition: null`. The Treasure-mana intervening-if is silently dropped today, independent of this plan.

