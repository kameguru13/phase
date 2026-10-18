# Phase D implementation review r4 (384bb92)

Review Head: 384bb92. The primary diff `8f18b5e..384bb92` was reproduced exactly. The Phase D span `4483f4b..384bb92` was read only where it meets the r3 fixes.

Completion Gate: not assessed. This was a read-only review: no builds, tests or probes were run, and the executor's red/green claims were not re-run. Under the deferred-verification policy, the full `test-engine` run is still owed.

Maintainer-Simulation Gate: not assessed (no matrix supplied). The F5 sacrifice row is `#[ignore]`d (§D).

Verdict: **behavior=4 text=1 machinery=0**.
- **Blocking:**
  - R4-F1 (MED). The plan's F5 row depends on it, and the r3 F1 custody fix does not hold on any completion that resumes before it collects.
  - R4-F2 (MED, latent). The orchestrator may name it as a residual instead, but the fix is about 10 lines.
- **Ride along:** R4-F3, R4-F4 (LOW behavior) and R4-F5 (LOW text).

CR numbers grep-verified for this review: 101.4, 117.3c, 118.12a, 601.2h, 602.2b, 603.2c, 603.3, 605.3b, 608.2c, 608.2e, 704.3. Every CR annotation the diff adds resolves and describes its site.

---

## A. Verification of the r3 fixes

### F1 (custody routing): correct for what it routes
- **`park_observer_triggers_if_paused`** (`triggers.rs:8400-8436`). It takes custody after the Priority early return, then collects `events[..owned.end]` through `filter_consumed_trigger_events_from`.
- **`collect_and_drain_observer_triggers_if_settled`** (`:8443-8475`) does the same. Its OpponentMayChoice callers (`engine_payment_choices.rs:419-420`, `:566-567`) resume first. They are safe even when the resume commits the announcement, because the commit claims `AllCollectors` and these helpers filter by the ledger (contrast R4-F1).
- **`park_discard_batch`** (`discard.rs:179-210`). It now copies `events[events_before_self..]` itself and routes through `defer_observer_triggers_for_paused_choice`.
  - `preceding_events` is only the CR 608.2i count window (`effects/mod.rs:14803`, `:14824`), never a trigger source, so the copy and the custody claim do not interact.
  - `defer_observer_triggers_from`, and its false premise, are gone.

### F2 (authority runs for any parked root): the gating holds
- The authority only runs `resume_pending_continuation_if_priority`. Inside that loop, the root is resumed only by `resume_due_counter_cost_root`, which still requires `counter_cost_root_is_due()` (`engine.rs:8999-9010`). Running the loop for an `Unfinished` root therefore cannot resume the root early. It advances only the subtree's top frames.
- Gate (a) (`effects/mod.rs:1213`) is still a due-only stop. It does not need to fire for `Unfinished`, because without a positional swap the top of an `Unfinished` subtree is subtree work.
  - The one exception is the `enclosing` case, where the top is the enclosing frame holding the child's drain. Its `:1815` retire is the child's own drain.
  - R4-F2 is the remaining way an outer frame can reach the top.
- **The progress test cannot loop.** It is a single comparison after a bounded loop; the loop is bounded by `SubtreePosition::is_strictly_inside`.
- **What the progress test misses.** It misses partial progress: frames retired, root still parked, wait unchanged. That case returns `None` after mutating state and appending events (R4-F4).
- **"Root left its slot" covers the case where the wait is unchanged.** If the root resumes, its activation is pushed and `Priority{P0}` is returned again, so the wait looks the same. The `Absent` disjunct catches this.
- `sync_waiting_for` is now called for any parked root under a Priority result. That is harmless: `finish_action_boundary` re-syncs anyway, and site 2 restores the reducer's wait around `end_action`.

### F3 guard: the arithmetic is right, but only one of the two promote callers is guarded
- **The guard is equivalent to "continuation index < boundary".** Once the draw is popped, the PR is the top, at `len-1`. The candidate continuation sits at `len-2`, and `len-2 < depth` exactly when `len < depth+2` (`game_state.rs:27735-27742`).
- **The `enclosing` case is also handled correctly.** There the PR is the enclosing frame at `depth-1`, and anything below it is outer, so the guard skips the promote.
- **No legitimate tail is blocked.** Everything the child's dispatch parks is installed after the boundary was recorded, so it sits at an index ≥ `depth`. The cost payment has no resolution-stack tail of its own: the root is the continuation.
- **The second caller is not guarded.** `finish_active_paused_post_replacement_dispatch` (`game_state.rs:27802-27821`) calls the same positional promote (R4-F2).

### F6: correct
- `owned` is `start..max(start, scope_start).min(len)`, so `owned.is_empty()` means the announcement owns the whole span, and `owned.end < len` means the span is non-empty.
- With no live announcement, `owned.end == len`, so `announced` is false.

### Rows
- **Double-Scry row.** It is the right replacement for T12c. It is nested because `sub_ability` replaces (`scry_one().sub_ability(lose(1).sub_ability(scry_one()))`). It asserts an empty stack between the two answers.
- **T10f / T10f′.** F4 is fixed: `p1_leg_is_the_only_continuation` requires exactly one `AbilityContinuation`, with a `LoseLife` chain whose controller is P1. R4-F3 covers the trailing assertion.
- **T12f.** The row is adequate.

---

## B. Deviation 2 (T10f: `UnlessPayment.player` reads P0 for P1's leg)

**Classification: not caused by this PR, and not an engine defect for the real card class. It is a fixture shape divergence.**
- The fixture's unless modifier is `lose_unless_two`, with payer `TargetFilter::Controller` (`cost_removed_counter_amount.rs:2729-2736`).
- `resolve_unless_payer` maps `Controller => Some(state.active_player)` (`effects/mod.rs:21436`). Upstream main has the identical line (`upstream/main` 39fb1eb, `effects/mod.rs:21391`). The same line is at 4483f4b.
- So every leg's prompt goes to the active player, P0, while `pending_effect.controller` is the scoped P1.
- Real "each player/opponent … unless that player pays" cards parse to `payer: ScopedPlayer`, which `resolve_effect_player_ref` binds per `player_scope` iteration (`effects/mod.rs:21491-21500`). Examples from `data/card-data.json`, 19 nodes with `player_scope` + `unless_pay`:
  - Bellowing Mauler (`All`);
  - Court of Ambition;
  - Ob Nixilis, the Adversary.
- The T10f path does not touch payer resolution, so the PR cannot cause this.
- The row's last assertion checks only `pending_effect.controller == P1`, so it passes despite the wrong seat. The doc comment's "P1 is asked" claims more than the assertion checks (R4-F3).
- Whether `Controller => active_player` is itself wrong, for an "unless you pay" on an ability controlled by the non-active player, is listed under Pre-existing.

---

## C. The F3 exposure at the second promote caller

`finish_active_paused_post_replacement_dispatch` (`game_state.rs:27802-27821`) works like this:
1. It finishes the active frame's resident drain.
2. If the frame is not empty afterwards, it calls `promote_ability_continuation_after_post_replacement_draw` unguarded. That call swaps any `AbilityContinuation` directly below a PR whose new resident is `Paused`.

Under a parked counter-cost root this is the T10f shape, reached by a different route:
- The child's PR frame sits at index `depth`. The outer `player_scope` leg sits at `depth-1`.
- The child's own drain is `Paused` beneath a nested drain. The nested drain was installed into the child's frame when one of the child's own events was replaced (CR 616.1g nesting, `install_post_replacement_drain`).
- When that nested drain retires, the call site is any of these: `effects/mod.rs:1404`, `:1815`, `draw.rs:865`, `engine_replacement.rs:2168/2325/2702/2804/2815` or `engine_resolution_choices.rs:6362/6620`.
- The frame still holds the child's `Paused` drain, so the promote lifts P1's leg above it. The stack length is unchanged, so the root reads `Unfinished`.
- Gate (a) is due-only, so `drain_pending_continuation` runs P1's leg inside the Cove's activation. This is exactly the r3-F3 violation (CR 608.2e + CR 101.4 + CR 118.12a).

The `draw.rs` path is now safe, because the guarded first promote leaves `[AC(P1), PR]`. `finish_…` then empties and pops the PR, and its own promote then errors harmlessly on an AC top.

The r3 recommendation was "guard … (or as a predicate passed to the promote)". The guard was put in only one of the promote's two callers.

---

## D. The blocked sacrifice row: resume-before-collect completions

### Mechanism (confirmed by reading; it matches the executor's [B, W, W])
1. In the EffectZoneChoice sacrifice completion, `resume_with_error_propagation` (`engine_resolution_choices.rs:6630`) runs the D5 loop.
2. The child's paused drain retires. `resume_due_counter_cost_root` resumes the root, and `push_ability_entry` calls `commit_activation_announcement`.
   - That reads `unclaimed_events(state, events, 0)`, because `begin_action` gave the live activation `From{0}`.
   - So it collects the sacrifice's `ZoneChanged`, giving W.
   - It claims `0..len` as `AllCollectors` (`announcement_triggers.rs:369-435`).
   - It also `mem::take`s the scope to `Unowned`.
3. `batch_or_drain_observer_triggers` (`:6631`) then calls `take_inline_trigger_span`, which finds no live announcement and returns the whole span. B1 builds `trigger_events` from `events[before..after]` with no ledger filter (`:775-793`) and collects W again.

The r3 custody routing put `take_inline_trigger_span` at the collector's call time. Every completion that resumes first evaluates custody after the resume has already committed the announcement. So the routing never applies on those completions, and nothing dedupes against the commit's claim.

### (a) Reachability
- Within Phase D's class it is reached by every interactive child whose completion handler resumes and then collects through an unfiltered body. That covers sacrifice, a "discard a card" choice (the DiscardChoice pair loop), the ring-bearer choice and the room-door choice.
- The whole Phase D class is synthetic: plan Residuals, "no printed RemoveCounter replacement exists". So by the calibration rule this is latent, but at least MED.
- **ManaAbilityPayment roots.** The resume completes a mana ability, which owns no announcement, so it commits nothing and there is no duplicate.
- **Cast{FinalizeDelvedCast} roots.** These are eligible at `PriorityBoundary` on upstream. Their resume finalizes the cast, and `commit_spell_announcement` collects and claims, so the same duplicate follows. It needs a choice-requiring replacement of a delve exile whose child is one of these interactive choices; no printed card is known. See Pre-existing.

### (b) Pre-existing or introduced
- **The activation instance is introduced inside the PR's Phase D.**
  - At 4483f4b, `PriorityBoundary` admitted only `Cast{FinalizeDelvedCast}` and `ManaAbilityPayment` (4483f4b `engine.rs:8783-8791`). A `CounterCostRemovalPayment` root never resumed inside `resume_pending_continuation_if_priority`.
  - 59247b7/618fb57 (the D5 resume) made it resume there. 8f18b5e (`commit_activation_announcement`) made that resume collect and claim.
  - The executor's "at base" is 8f18b5e, which already contains both.
- **The spell (delve) instance is pre-existing on upstream** (latent; see Pre-existing).

### (c) Fix recommendation: filter by the ledger in the shared collector bodies, rather than collecting before the resume
- **Why not collect before the resume.**
  - It would have to restructure every listed site.
  - It would also move the B1/B2 decision ahead of the continuation. That decision depends on the post-resume wait, which matters for the continuation-pause batching of issues #1793 and #4245.
  - And it would reorder the triggers of flows with no announcement.
- **Why the ledger filter is enough.** The module's own invariant (`commit_activation_announcement` doc) is that every ledger writer pairs its claim with a collection. So filtering removes only re-collections, and it is a no-op for flows with no announcement.
- **The idiom is already in use in the sibling collectors:**
  - `collect_search_observer_triggers` (`:862-873`, via `filter_already_collected_trigger_events_from`);
  - `park_observer_triggers_if_paused` and `collect_and_drain_observer_triggers_if_settled` (via `filter_consumed_trigger_events_from`).

Predicate: a completion that calls `resume_pending_continuation_if_priority` (directly, via `resume_with_error_propagation`, or via `finish_with_continuation`) or `handle_replacement_choice`, and then collects a slice that began before that call into `deferred_triggers` without consulting `consumed_before_priority_trigger_events`.

Command:
```
rg -n "resume_with_error_propagation\(state, events\)\?;|finish_with_continuation\(|resume_pending_continuation_if_priority\(|batch_or_drain_observer_triggers\(|take_inline_trigger_span\(" crates/engine/src/game/engine_resolution_choices.rs crates/engine/src/game/engine_payment_choices.rs
```

Each matching site and what to do with it:

| Site | Shape | Disposition |
|---|---|---|
| `batch_or_drain_observer_triggers` body `:769-793` | unfiltered slice + `PlayerPerformedAction` completion chain | **change**: build `trigger_events` from `filter_consumed_trigger_events_from(&events[..slice_end], event_slice_start, Ordinary, &state.consumed_before_priority_trigger_events)` (and the same for the `completion_events` chain), then apply the existing PhaseChanged/ZoneChanged filters. One change covers these callers: |
| ↳ `:6365-6366` player-scope sacrifice completion | resume → B1 | covered |
| ↳ `:6630-6631` EffectZoneChoice sacrifice (the blocked row) | resume → B1 | covered |
| ↳ `:7542` → `:7566` EffectZoneChoice Sacrifice/ChangeZone/BounceAll tail (`true` flag drops `ZoneChanged`, not the rest) | resume → B1 | covered |
| ↳ `:7991-7995` ChooseRingBearer, `:8024-8029` ChooseRoomDoor | `finish_with_continuation` → B1 | covered |
| ↳ `:6016`, `:7423` MayCost accept | `handle_replacement_choice` → B1 | covered (it can resume roots at `ReplacementDelivered`) |
| ↳ `:8050`, `:8086` dungeon | B1 before `finish_with_continuation` | unaffected (custody applies) |
| DiscardChoice pair loop `:6047-6066` (after `finish_with_continuation` `:6028`) | unfiltered two segments | **change**: the same ledger filter per segment. This is the "discard a card" child. |
| Keeper choices `:8611-8632`, `:8708-8729` B2 arms | resume → unfiltered park when paused | **change** (latent: needs a commit followed by an outer pause). Same filter; cheapest as one shared helper with the two above. |
| `collect_search_observer_triggers` after `:1527`, `:5048`, `:5120` | resume → ledger-filtered | already safe |
| `engine_payment_choices.rs:419-420`, `:566-567` | resume → `collect_and_drain…` (ledger-filtered) | already safe |
| all other resume sites in the command output | no collector follows | unaffected |

- **Row.** Un-ignore the F5 sacrifice row (exactly [B, W]).
- **Add a DiscardChoice child row:** a "discard a card" child with a Discarded-witness, exactly [B, W]. Revert check: drop the filter in each body, and that row goes red.

---

## Findings

**[MED] [behavior] R4-F1. Completions that resume and then collect re-collect what the resumed root's announcement commit already collected and claimed. As a result, an interactive sacrifice or discard child of a counter-cost replacement puts its observer trigger on the stack twice ([B, W, W]), and the plan's F5 row is `#[ignore]`d.**
- Evidence:
  - `engine_resolution_choices.rs:6630-6631`, `:6365-6366`, `:7542/7566`, `:7991-7995`, `:8024-8029`;
  - the pair loop `:6028` and `:6047-6066`;
  - the B1 body `:769-793`;
  - `announcement_triggers.rs:369-435` (the commit claims `AllCollectors` and ends the scope);
  - `cost_removed_counter_amount.rs`, the ignored row;
  - §D.
- Why it matters:
  - This is the r3-F1 class (CR 603.2c, CR 603.3), introduced in Phase D (D5 resume + 8f18b5e commit).
  - The r3 custody routing is evaluated after the resume, so it never applies on these completions.
- Suggested fix:
  - Ledger-filter the slices in `batch_or_drain_observer_triggers`, the DiscardChoice pair loop and the two keeper B2 arms (table in §D).
  - Un-ignore the sacrifice row and add a discard-child row.

**[MED] [behavior] R4-F2. The F3 guard covers only `take_completed_multi_draw_frame`. `finish_active_paused_post_replacement_dispatch` calls the same positional promote unguarded, so a nested drain retiring inside the child's frame still lifts an outer `player_scope` leg above the child's `Paused` drain.**
- Evidence:
  - `game_state.rs:27731-27748` (guarded) vs `:27814-27821` (unguarded);
  - `resolution.rs:3295-3316`;
  - the callers listed in §C.
- Why it matters: P1's unless leg runs inside the Cove's activation. This is the r3-F3 violation (CR 608.2e + CR 101.4 + CR 118.12a), for children whose own event is replaced with an interactive post-effect. It is latent and synthetic, as the whole class is.
- Suggested fix:
  - Hoist `continuation_is_outer` into one `GameState` method, for example `promote_paused_drain_tail_within_counter_cost_subtree()`. It returns without promoting when `len < depth + 2`; the predicate is the same, because the stack is `[.., C, PR]` at both callers.
  - Call it from both callers.
  - Optional row: the T10f board with a child whose own draw is replaced by a nested replacement with an interactive post-effect.

**[LOW] [behavior] R4-F3. T10f's last assertion does not check who is asked. Its fixture's payer also differs from the real card class, so the doc comment's "P1 is asked" is not proved.**
- Evidence:
  - `cost_removed_counter_amount.rs` `outer_player_scope_seat_waits_for_the_mana_root`: the doc says "P0 then declines and P1 is asked", and the assertion matches only `pending_effect.controller == P1`;
  - `lose_unless_two` payer `TargetFilter::Controller` (`:2729-2736`) resolves to `state.active_player` (`effects/mod.rs:21436`);
  - real cards use `ScopedPlayer` (§B).
- Why it matters: this is fixture path divergence. The row would stay green if P1's leg were prompted to the wrong seat.
- Suggested fix:
  - In the helper, set `each_player.unless_pay` payer to `TargetFilter::ScopedPlayer`, which is what the parser emits for "unless that player pays".
  - Add `player == P1` to the post-decline match (and `player == P0` is already asserted on the first prompt).

**[LOW] [behavior] R4-F4. When the F2 authority makes partial progress (some subtree frames retire, the root stays parked, the wait is unchanged), it returns `None` after mutating state and appending events. At site 2 those events are then neither staged nor collected.**
- Evidence:
  - `engine.rs:1747-1762` (`progressed` ignores stack/event progress);
  - site 2 `engine.rs:1662-1693` (no convergence on `None`);
  - `announcement_triggers::end_action` stages only for non-Priority waits.
- Why it matters: triggers from the advanced frames are lost (CR 603.2, CR 603.3). This is latent, and only within residual 3's "loop cannot finish" case. Before r3 the same work ran inside the next pipeline pass, which stages.
- Suggested fix:
  - On the `None` path, with the root still parked, stage the appended span into the carrier: `stage_announcement_trigger_events(state, events, before_len)`.
  - Or treat `events.len() > before_len` as progress, but that also runs SBAs mid-activation, so prefer staging.

**[LOW] [text] R4-F5. The authority's name and its two call-site comments still say "due", but it now runs for any parked root.**
- `engine.rs:1747`, `fn resume_due_counter_cost_root_at_action_boundary`. Rename to `resume_parked_counter_cost_root_at_action_boundary`, and update the calls at `:1662` and `:16418`.
- `engine.rs:1662`, `let resumed_due_root`. Rename to `resumed_parked_root` (and its use at `:1687`).
- `engine.rs:1659`, "a counter-cost root this action made due resumes before the action returns, and its events join the convergence pass below." Replace with: "a parked counter-cost root's subtree is finished and the root resumed before the action returns; when that made progress, its events join the convergence pass below."
- `engine.rs:16411`, "a counter-cost root this handler made due resumes before the pipeline below". Replace with: "a parked counter-cost root's subtree is finished and the root resumed before the pipeline below".

## Pre-existing

- **`resolve_unless_payer` maps `TargetFilter::Controller` to `state.active_player`, not to the ability's controller.**
  - Location: `effects/mod.rs:21436`; upstream 39fb1eb `:21391`.
  - For an "unless [you] pay" on an ability controlled by the non-active player, the prompt would go to the active player (CR 118.12a). This is unprobed.
  - It is the root of T10f's Deviation 2, but real `player_scope` unless cards use `ScopedPlayer`, and the PR does not depend on this mapping.
- **On upstream, a `Cast{FinalizeDelvedCast}` root can also be resumed at `PriorityBoundary` inside these completions.**
  - Inside the same resume-then-collect completions (`resume_with_error_propagation` → `drain_pending_cost_move_resume(PriorityBoundary)`), `commit_spell_announcement` collects and claims the move's events, and the unfiltered B1 collects them again.
  - Upstream: `engine.rs:8779-8786`, `:8946-8951`; `announcement_triggers.rs:300-350`.
  - This was found by reading only; no printed card is known that reaches it (it needs a choice-requiring replacement of a delve exile with an interactive child). R4-F1's ledger filter closes it too.
