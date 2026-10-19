# Final PR review r8: phase-rs/phase#10039 (engine/cost-removed-counters-wip)

Review Head: de714c72f29a3de0fa0dac8f3eab5e3c6a0f5608
Previous full-head PASS: r7 at 375a58f8189652b46da5a503bfd0f1aa755d90f5.
Delta reviewed:
- 341a17f, which merges upstream main b6a83a8 (6 upstream commits since d3f002d) and moves the protocol bump to 154 / wire 136;
- de714c7, the CodeRabbit fix for the two paused observer collectors.

Mode: read-only. No cargo or pnpm was run. The one script run was `node scripts/check-protocol-version.mjs`, which returned rc=0.

## Verdict

behavior=0 text=0 machinery=0

No findings. §4 lists observations, which are not findings.

---

## A. Merge resolution: protocol surfaces

**Merge shape.** `git show --cc 341a17f -- crates/engine` is empty, so the merge has no evil hunks in engine code. Every engine file auto-merged cleanly. Conflict resolution touched only the protocol files and the phase-server literal.

**The same numbers appear on every surface** (full-game 154, MIN 153 / 154, wire 136, with 153/135 reserved for #10004):

| Surface | Value at head | Check |
|---|---|---|
| `crates/lobby-broker/src/protocol.rs` `PROTOCOL_VERSION` | 154 | `MIN_SUPPORTED_PROTOCOL = PROTOCOL_VERSION.saturating_sub(1)`, which is 153. The test asserts 154 / 153. |
| `crates/server-core/src/protocol.rs` | `MIN_SUPPORTED_PROTOCOL = PROTOCOL_VERSION` (154) | The test was renamed to `protocol_version_is_154_for_counter_cost_removal_payment`, and the REVERT-PROBE reference was renamed with it. |
| `client/src/adapter/ws-adapter.ts` | `PROTOCOL_VERSION = 154` | `MIN_SUPPORTED_SERVER_PROTOCOL = PROTOCOL_VERSION`; `LOBBY_MIN… = PROTOCOL_VERSION - 1`, which is 153 and agrees with the broker. |
| `client/src/network/protocol.ts` | `WIRE_PROTOCOL_VERSION = 136` | protocol.test.ts pins v136. |
| `scripts/check-protocol-version.mjs` | 71 + 83 = 154; 54 + 82 = 136 | Each reserved step (+82 / v153, +81 / wire 135) carries a comment. rc=0. |
| p2p gate (`p2p-adapter-multiplayer.test.ts`) | The history `it.each` gains 132, 133 and 134 (upstream's current wire). Refuses v135, admits v136. | The Bestow reconnect and host rows keep upstream's refuse-133 literal and admit 136. The comment's revert arithmetic (136 → 135) matches. |
| `openPhaseSocket.test.ts` | Rejects v152 (new row), keeps the v151 and v150 rejects, accepts v154 | Upstream's v152-accept row was converted to v154-accept, and its title covers both feature sets. |
| `crates/phase-server/src/main.rs` handshake | `classify(153)` gives RejectProtocol{153, 154}; `classify(154)` gives Accept | This follows the current version, as upstream bumps it. |

**Doc entries.**
- In ws-adapter, protocol.ts, lobby-broker and server-core, the 154 / 136 entry sits directly above upstream's 152 / 134 entry. Upstream's 152 / 134 text is intact everywhere.
- Each new entry names the 153 / 135 reservation for #10004.
- "A v153 peer cannot parse the tag" is accurate: v153 will be #10004's build, which lacks our variant.

**Stale references.** `git grep` for v153, v135, "protocol 153", "wire 135", "153/135" and `_153_` finds only:
- the reservation notes;
- the wire-revert comment, which correctly names v135 as the version one below ours.

No surface claims 153 or 135 as this PR's own number.

**Nothing from upstream's 152 work was lost.** `git diff b6a83a8 de714c7` on these files is purely additive around the 152 entries and literals.

## B. The upstream commits (d3f002d..b6a83a8) and our machinery

Upstream engine changes:

| Commit | Engine change | Interaction with this PR |
|---|---|---|
| 57337ab (Fold into Aether) | `sub_has_independent_object_target_slot` gains an `effect_refs_parent_target_metadata` disjunct (effects/mod.rs) | Target-slot classification only. It touches no trigger collection, resume or cost path. It auto-merged next to our `drain_pending_continuation` / `resolve_ability_chain_step` hunks without textual or semantic overlap. |
| 598c7b8 (Way of the Pyromancer) | Parser: a granted quoted loyalty ability gets loyalty timing (oracle.rs, oracle_static/grammar.rs) | A granted `[+1]` adds counters. A granted `[−N]` would take the existing negative-loyalty path, which is already disclosed as residual 4 (no root, no count). Unchanged. |
| b6a83a8 (#10098 bounded drain shortcut) | analysis/resource.rs, decision_template.rs, loop_check.rs, game/engine.rs (`bounded_offer_tail`, `drive_one_shortcut_cycle` + `DepartureStop`, `materialize_fixed_shortcut`, `handle_declare_shortcut`), interaction.rs, visibility.rs, candidates.rs | All loop-shortcut certification and projection. Its "announcement" vocabulary means the shortcut's trigger-target aims (`leading_announcement_aims` reads the top stack entry's targets). It is not activation-announcement custody. |
| 9e9a166, e453397, e0e3536 | AI baseline, client i18n, changelog | None. |

What I checked:
- **Collectors and resume sites in the upstream delta.** `git diff d3f002d b6a83a8 -- crates/engine` adds no use of `process_triggers`, `collect_*triggers*`, `deferred_triggers`, `consumed_before_priority_trigger_events`, `take_inline_trigger_span`, `stage_announcement_trigger_events`, `drain_pending_*`, `pending_cost_move_resume` or `run_post_action_pipeline`. So no new collector or resume site needs custody or ledger treatment.
- **The shortcut drive.** `drive_one_shortcut_cycle` runs ordinary priority-pass pipelines on a clone, and it now continues past a seat departure under `CompletesRepetition`. That reaches our machinery only through the same public apply paths it already used.
- **State our PR reshaped.** No added upstream line constructs or reads `SpellContext`, `PendingManaAbility`, `ManaAbilityCostCursor`, `PendingCast`, `ResolvedAbility` literals, `PendingCostMoveResume`, `cost_removed_*` or `last_effect_amount`. The only struct-literal additions are `leading_aims: Vec::new()` / `suggested_declaration: None` on shortcut types in the visibility.rs and game_state.rs tests.
- **The loop prover's belt** (resource.rs `!state.deferred_triggers.is_empty()`) predates the delta and is untouched.
- **Auto-merged `tests/integration/main.rs`** carries our `cost_removed_counter_amount` and upstream's `granted_loyalty_ability_timing`, `three_opponent_drain_shortcut` and `fold_into_aether_counter_put`. That last name refers to putting a creature onto the battlefield, not counters.

## C. de714c7: `unclaimed_observer_events` in the two paused collectors

**The contract holds.** `take_inline_trigger_span` (announcement_triggers.rs:197–208) returns `start..events.len()` with no live announcement, and `start..owned_from.min(len)` with one. Either way `owned.start == start`, so these two calls are equivalent to the previous slice bounds:
- `unclaimed_observer_events(state, events, owned.start, owned.end)` at :854;
- the same call at :8575.

The helper also handles an inverted or empty range by returning empty, which matches the old empty slice.

**The filter only narrows.** `filter_consumed_trigger_events_from(&events[..end], start, Ordinary, ledger)` drops only occurrences already in the ledger. The occurrence identity is computed over a prefix of the action buffer, which agrees with every writer, because they all claim against the full buffer and the buffer is only appended to.

I re-walked the writers at head: triggers.rs:1174, :11730, :11857 and :14333; engine.rs:10764 (`collect_copy_cast_observers`); announcement_triggers.rs `claim_indices` (commit, `claim_ordinary`, `claim_ordinary_collected_span`); and stack.rs:4296 (clone). Each one claims an occurrence that some collector has collected or will collect. Two cases deserve a note:
- `stage_announcement_trigger_events` claims its whole span `OrdinaryCollectorsOnly`, including cost-owner-retained events that it did not carry. Those retained events are collected by their owner from its durable `deferred_cost_events` copy, which is not ledger-filtered (`collect_completed_mana_frame_events` takes `deferred` unfiltered). So the claim is paired, and suppressing them here prevents a second collection rather than losing one.
- Upstream added no writer in the delta.

So the change can only remove re-collections.

**The `Ordinary` requester is correct.** `collect_triggers_into_deferred` delegates to `collect_pending_triggers`, which is ordinary-only. That matches the sibling keeper and batch collectors (:797, :809, :6094, :8446, :8664, :8765) and the triggers.rs park and collect (:8512, :8553). Delayed triggers are not collected here, so an `OrdinaryCollectorsOnly` claim correctly suppresses only this collector.

**How realistic is CodeRabbit's route?**
- **Its specific premise** is that a parked counter-cost root claims events before either collector runs. I found it hard to reach:
  - The root resumes only in `resume_due_counter_cost_root` (engine.rs:9191), inside `resume_pending_continuation_pass`, or at the action-boundary authority.
  - `effects::drain_pending_continuation` returns early while the root is due. So at engine_replacement.rs:1281, which runs right after `drain_pending_continuation`, no root resume or push commit has happened.
  - At :1271, at discard.rs:205 (`park_discard_batch`, called from `resolve`) and at :6012, the action has not reached a root resume.
- **A more general route makes the filter meaningful.** `stage_announcement_trigger_events` advances `AnnouncementEventScope::From{start}` to `events.len()`. A later `take_inline_trigger_span` in the same action whose `start` precedes an earlier inline stage then gets back `start..watermark` as unowned. That range includes occurrences the earlier stage already carried and claimed `OrdinaryCollectorsOnly`.
  - The raw slice would collect those occurrences into `deferred_triggers` as well as the carrier, which violates CR 603.2c.
  - The filtered form drops them.
  - This is the R2 watermark shape. Under this PR, replacement children that pause (discard, sacrifice) do run under a live counter-cost announcement, so the route is plausible, though I did not prove it reachable.
- **Either way the change is a strict improvement and brings the last two take-span collectors in this file into line with their siblings.**

**CR annotations.** I grep-verified CR 603.2c: "An ability triggers only once each time its trigger event occurs." That fits "an event another collector already claimed this action is not collected again", and it is the same citation the sibling collectors use. The surrounding 601.2h and 603.3 annotations are unchanged and verified in r7.

## 4. Observations (not findings)

1. **One raw-slice take-span collector remains**, in a different file: `drain_pending_change_zone_iteration` (effects/mod.rs:2537–2551; this PR introduced the custody call there).
   - It still reads `events[owned]`.
   - I found no claim that can fall in its range on a reachable path. The range is `events_before_drain..`, captured inside the function, and nothing between that point and the collector stages announcement events.
   - The logical-owner marks there claim only `ZoneChanged`, which this collector already filters out.
   - So it is equivalent to the filtered form today. Routing it through the same filter (the helper would need to be made `pub(crate)`) would make the take-span collectors uniform. That is optional.
   - The ChooseManaColor raw slice (engine.rs:13481) stays as listed under Pre-existing in r7.
2. **de714c7 adds no discriminating row.** That is consistent with r5/r7's handling of the sibling filter sites. The PR body can say the change is defensive and covers the watermark route (§C).
3. **Update the PR body for the renumbered reservation.** r7 §7 item 14 now reads: 153 / wire 135 reserved for #10004, and ours is 154 / wire 136, with the script at +83 / +82. Add phase-server's `bestow_payload_full_game_refuses_previous_and_accepts_current` literals (153 / 154) to the list of things to renumber if #10004 lands at other numbers or after this PR.
4. **Placement of the v154 note in the check script.** The free-form v-note list in `check-protocol-version.mjs` (lines 70–97) was already out of numeric order upstream (v151 before v141, v134 after v147). Our v154 line sits where the v153 line sat at r7, before upstream's v134 / v147–v152 lines. This is harmless.

## Evidence owned by the orchestrator

- CI status at de714c7, including the phase-server handshake test that failed before the literal bump, and the client vitest for the renumbered rows. Not observable from this session.
- The CodeRabbit thread resolution and the parse-diff sticky at this head.
- r7's other completion evidence (the counter-cost suite at 74/0/0, Gate A/G) predates the merge. Re-confirm the integration run on the merged tree, since the merge added three upstream integration modules.

Final review-impl PASS head=de714c72f29a3de0fa0dac8f3eab5e3c6a0f5608
