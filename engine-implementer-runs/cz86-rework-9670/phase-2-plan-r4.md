# Phase 2 plan, revision 4 — addendum: U4, an answer to a parked combat batch converges once (cz86-rework-9670)

Mode: `engine-planner` **phase-plan mode** for charter phase **P2** ("Every combat pause asked").
This is an addendum plan. It specifies one more P2 unit, **U4**, and the Sizing for the whole phase.
U1–U3 (plan r3 §§4–17) are delivered and unchanged. Nothing below rewrites them.

Inputs read: `task.md`; `phase-charter` (frozen; P2 entry, seam notes); `addenda/phase-2`;
`phase-inputs/phase-2`; `phase-1-accepted-summary.md`; `phase-2-plan-r3.md` (clean, delivered);
`phase-2-impl-review-r1.md` (fixed by `4d37cfed3d`); `phase-2-impl-review-r2.md` (open);
phase-fit entries 1–35; the fix executor's stop-and-return (entry 35).

Attempt history (from the phase-fit record):
- P2 plan loop: r1 1 text (applied, clean); executor stop-and-return → r2 (design round 1);
  r2 review 2 behavior + 1 text (design round 2); r3 1 text (applied, clean). The executor
  stop-and-return of entry 35 is **design round 3** of this loop (resumed count). It is skipped by
  the lowering comparison and counts toward the fifth-round limit.
- P2 implementation-review loop: r1 1 behavior (MED custody on a re-parking answer; fixed in
  `4d37cfed3d`); r2 1 behavior (MED Zur's Weirding, open). r2 did not lower r1. The next
  implementation review must report **0 behavior findings**, or the loop stops (two consecutive
  non-lowering design rounds).
- Budget: standard. No expansion case is open.

State: branch `engine/unpreventable-damage-riders`, HEAD `ff069773b5` (P2 commits `8b16c4a374`,
`ebacebdf60`, `4d37cfed3d`, and the red witness `ff069773b5`). `PHASE_BASE_SHA` = `00396db22d`.
**U4 base** = `ff069773b5`. Every "today" claim below was read at `ff069773b5`.

**Probe status.** The planning worktree is read-only and nothing was built. **No claim in this
revision was probed by running code.** Each runtime claim is read from source and named by
function, or is listed in §3 as a claim the executor establishes before the edit that depends on
it.

---

## 0. Premise (Step 0)

Oracle text was read from MTGJSON `data/mtgjson/AtomicCards.json`, version `5.3.0+20261021` (the
charter's version), not from the brief. The task's card premises hold.

- **Zur's Weirding**: "Players play with their hands revealed. / If a player would draw a card, they
  reveal it instead. Then any other player may pay 2 life. If a player does, put that card into its
  owner's graveyard. Otherwise, that player draws a card." Rulings: "If a spell or ability causes
  more than one card to be drawn, draw the cards one at a time and deal with this replacement
  before drawing the next card." "If someone pays the cost, then the card was never drawn or
  discarded. It just gets put into the graveyard directly from the library."
- **Swans of Bryn Argoll**: "Flying / If a source would deal damage to this creature, prevent that
  damage. The source's controller draws cards equal to the damage prevented this way." Ruling:
  "The source of combat damage is the creature that deals it."
- **Psychosis Crawler**: "Psychosis Crawler's power and toughness are each equal to the number of
  cards in your hand. / Whenever you draw a card, each opponent loses 1 life."
- **Suntail Hawk**: "Flying". **Grizzly Bears**: no rules text.
- **Dralnu, Lich Lord**: "If damage would be dealt to Dralnu, sacrifice that many permanents
  instead. / {T}: Target instant or sorcery card in your graveyard gains flashback until end of
  turn. …" Rulings: "Dralnu may be one of the permanents you sacrifice while applying its
  replacement effect." "If any of the sacrificed permanents would deal or be dealt damage at the
  same time as Dralnu, … abilities of other permanents that trigger on that damage do."
- **Rest in Peace**: "When this enchantment enters, exile all graveyards. / If a card or token would
  be put into a graveyard from anywhere, exile it instead."
- **Darksteel Colossus**: "Trample … / Indestructible … / If Darksteel Colossus would be put into a
  graveyard from anywhere, reveal Darksteel Colossus and shuffle it into its owner's library
  instead."
- **Juri, Master of the Revue**: "Whenever you sacrifice a permanent, put a +1/+1 counter on Juri. /
  When Juri dies, it deals damage equal to its power to any target." (parsed: `Sacrificed` trigger)
- **Exquisite Blood**: "Whenever an opponent loses life, you gain that much life." (parsed:
  `LifeLost` trigger; used by `exquisite_blood_routing.rs`)
- **Syr Konrad, the Grim**: "Whenever another creature dies, or a creature card is put into a
  graveyard from anywhere other than the battlefield, or a creature card leaves your graveyard, Syr
  Konrad deals 1 damage to each opponent. / {1}{B}: Each player mills a card." (parsed:
  `ChangesZone`; used by `issue_3299_syr_konrad_duplicate_damage.rs`)
- **Breathstealer's Crypt**: "If a player would draw a card, instead they draw a card and reveal it.
  If it's a creature card, that player discards it unless they pay 3 life." (§19, triage)

CR numbers used in this plan, each grep-verified in `docs/MagicCompRules.txt`: 101.4, 119.4, 510.2,
510.3, 510.3a, 603.2, 603.2c, 603.3, 603.3b, 603.7, 605.4a, 608.2d, 614.6, 615.5, 616.1, 616.1f,
701.21a, 704.3, 800.4a. The ones that carry the design:
- CR 510.2: "all combat damage that's been assigned is dealt simultaneously. This turn-based action
  doesn't use the stack. No player has the chance to cast spells or activate abilities between the
  time combat damage is assigned and the time it's dealt."
- CR 510.3a: "Any abilities that triggered on damage being dealt or while state-based actions are
  performed afterward are put onto the stack before the active player gets priority …"
- CR 603.3: "… its controller puts it on the stack … the next time a player would receive priority."
- CR 603.3b: "If multiple abilities have triggered since the last time a player received priority,
  … each player, in APNAP order, puts each triggered ability they control …"
- CR 704.3: "Whenever a player would get priority …, the game checks for any of the listed
  conditions for state-based actions …"
- CR 615.5: "… The prevention takes place at the time the original event would have happened; the
  rest of the effect takes place immediately afterward."
- CR 119.4: "If a player pays life, … the player loses that much life."

---

## 1. Pattern Coverage

**Charter class attribution, cited, not re-run** (charter §"Carried survival steps", Step 0.5):
class C, CR-defined combat replacement and prevention choices (CR 510.2 + CR 616.1), A ∪ B minus
three noncombat-only cards = 48 cards (`class/union.txt`), MTGJSON `5.3.0+20261021`, `jq-1.8.1`.
P2 owns its aggregate and in-leg rider asks. U4 covers the same class. It does not grow the card
list: it makes every answer to a prompt raised inside that class's combat batch finish the batch
once.

U4's own axis is **the answer arm**, not the card. A rider's work, or a replacement that modifies
it, can raise any prompt the engine has. The reducer answers each prompt in one arm of
`engine::apply_non_priority_pass_action`. Some arms fall through to that function's epilogue,
which already resumes a parked batch (plan r3's D5). Others return an `ActionResult` directly and
skip it. Printed cards reach both kinds:

| Answer arm (reducer) | Returns | Reached inside a class-C combat batch by | Reachable |
|---|---|---|---|
| `OptionalEffectChoice` / `DecideOptionalEffect` | falls through | Abundance on Swans of Bryn Argoll's or New Way Forward's draw; Channel Harm's "may" | yes (delivered rows) |
| `EffectZoneChoice` (resolution choice), settled | falls through (`WaitingForWithInlineTriggers`) | Dralnu, Lich Lord's sacrifice | yes |
| `EffectZoneChoice` (resolution choice), sacrifice paused for a replacement | **direct** (`ResolutionChoiceOutcome::ActionResult`, the `PausedForReplacement` arm) | Dralnu sacrificing Darksteel Colossus under Rest in Peace | yes |
| `ReplacementChoice` / `ChooseReplacement` | falls through (`engine_replacement::handle_replacement_choice` resumes itself) | the batch's own CR 616.1 order; Darksteel Colossus vs Rest in Peace | yes (main's #9670 authority, unchanged) |
| `OpponentMayChoice` / `DecideOptionalEffect` | **direct** | Zur's Weirding on any draw rider (Swans, New Way Forward) | yes |
| `UnlessPayment` / `PayUnlessCost` | **direct** | Breathstealer's Crypt on a draw rider | in principle; its parse is wrong today (§19) |
| `UnlessPaymentChooseCost`, `ChooseManaColor`, mana payment arms, `ReturnAsAuraTarget`, `ChooseTokenHost`, `ProliferateChoice`, shortcut arms, `SetAutoPass` | **direct** | no class-C rider reaches them: no rider in `class/prevention_rider_class.json` or `class/substitution_class.json` pays mana, moves an Aura or token onto the battlefield, proliferates or offers a shortcut, and `SetAutoPass` needs a `Priority` wait | no |

So the class is "every arm", and the design must not depend on which arm returns how. U4 closes
it at the reducer's one exit, which every arm passes (§4, D9).

---

## 2. What the U4 base does today (read at `ff069773b5`, named by function)

- **The one resume site.** `apply_non_priority_pass_action` takes
  `combat_damage::ParkedCombatDamageAnswer::begin(state, events.len())` before its `match`. Its
  epilogue calls `combat_damage::resume_parked_combat_damage_after_answer` after
  `resume_parked_counter_cost_root_at_action_boundary` and before the post-action pipeline. That
  call either resumes the batch (`Priority`) or, for an answer that leaves another prompt, folds
  the answer's events into `PendingCombatLifelink::batch_events` through
  `take_answer_events_into_batch` and claims them (`4d37cfed3d`).
- **Direct returns skip it.** The `OpponentMayChoice` arm is
  `return engine_payment_choices::handle_opponent_may_choice(..)`. The resolution-choice arm returns
  `ResolutionChoiceOutcome::ActionResult(result)` as `return Ok(result)`. Every other arm in §1's
  "direct" rows returns from inside the `match` the same way. `apply_non_priority_pass_action`'s
  only caller is the tail of `engine::apply_action`.
- **The direct handler settles on its own.** `handle_opponent_may_choice_inner`'s accept path and
  `advance_opponent_may_offer` (its decline and all-declined path) both end with
  `resume_pending_continuation_if_priority` and then
  `triggers::collect_and_drain_observer_triggers_if_settled`. At `Priority` that helper collects the
  answer's observers into `deferred_triggers` and drains them onto the stack. It does not claim
  them: the boundary comment in `apply_action_boundary_core` says so ("`handle_opponent_may_choice`,
  which collects into `deferred_triggers` without recording them in
  `consumed_before_priority_trigger_events`"). Away from `Priority` it parks them
  (`park_observer_triggers_if_paused`).
- **So today a declined Zur's offer** draws, puts Psychosis Crawler's trigger on the stack and
  returns `Priority` with `pending_combat_lifelink` still `Some`. That is r2's finding, and
  `zurs_weirding_offer_inside_the_combat_batch_completes_it` (committed red) shows it. The batch
  finishes only at a later pass, through the departure re-entry at the top of
  `combat_damage::resolve_combat_damage`.
- **Resolution-choice handlers queue directly.** `engine_resolution_choices` queues answer-local
  observers through `triggers::collect_action_end_triggers_into_deferred` without the four
  observer helpers: `batch_or_drain_observer_triggers` (Dralnu's settled sacrifice, the
  `Completed` arm of the `EffectZoneChoice` sacrifice), `defer_observer_triggers_for_paused_choice`,
  `collect_search_observer_triggers`, and the keep-exact and discard arms (10 call sites,
  `grep -n "collect_action_end_triggers_into_deferred(" engine_resolution_choices.rs`).
  `take_answer_events_into_batch` excludes an event a collector *claimed* and a `ZoneChanged` a
  collector *queued*. A queued non-zone event (a sacrifice) is in neither set (claim E-U4-4).
- **Handler-internal pipelines.** `engine_payment_choices::handle_unless_payment` and its resumes
  call `engine_priority::run_post_action_pipeline_from` themselves. Every pipeline wrapper reaches
  `run_post_action_pipeline_from_with_policy`, which runs state-based actions, scans and drains.
- **Zone moves queue and may drain.** Zone moves inside a rider's work collect their own observers
  at the move (`effects/change_zone.rs`, `zone_pipeline.rs`, CR 603.10a look-back), and
  `effects/change_zone.rs` and `effects/mod.rs` call `triggers::drain_deferred_trigger_queue`
  (4 call sites each). Every drain is gated by `triggers::can_drain_deferred_triggers`
  (`should_drain_deferred_triggers_now` → `can_drain_deferred_triggers`).
- **The precedent stand-down.** `triggers::triggered_mana_sidecar_owns_local_collection` (CR 605.4a)
  is consulted first by the four observer helpers
  (`collect_and_drain_entry_observer_triggers_if_settled`,
  `park_observer_triggers_if_paused_matching`, `park_delayed_observer_triggers_if_paused`,
  `collect_and_drain_observer_triggers_if_settled`): "Both observer helpers fail closed on this
  predicate *before* collecting or draining … the fixed point — not a helper — owns every emitted
  event of an accepted occurrence." The sidecar's own drain gate sits in
  `resolution_completion_can_settle`.
- **The record's lifetime.** `pending_combat_lifelink` is written only at the park points in
  `combat_damage.rs` (the `state.pending_combat_lifelink = Some(..)` sites). The resume takes it
  first (`resume_pending_combat_lifelink`: `state.pending_combat_lifelink.take()?`). While the
  batch runs, the record is out of the state (claim E-U4-2).
- **The fold's retained set duplicates an existing authority.** `take_answer_events_into_batch`
  builds `retained_logical_zone_events + retained_cost_events` and excludes
  `deferred_entry_events` inline. That is exactly `triggers::paused_exit_retained_events` (private
  today).

---

## 3. Claims the executor establishes first (before the edit that depends on each)

Each is measured with a throwaway probe (a `#[test]` or `eprintln!` at the seam, removed before the
checkpoint) in an isolated `CARGO_TARGET_DIR`, against the named row. Each probe prints a positive
marker beside its verdict.

- **E-U4-1** (before D8). At `ff069773b5`, the decline of T-1 reaches
  `collect_and_drain_observer_triggers_if_settled` with `pending_combat_lifelink.is_some()`, and
  that call drains Psychosis Crawler's trigger. Marker: `deferred_triggers.len()` before and the
  stack length after.
- **E-U4-2** (before D8). Over every test in the three P2 test files and
  `combat_lifelink_replacement_ordering.rs`, a counter at each of D8's consumers that fires while
  `pending_combat_lifelink.is_some()` fires only (a) inside an answer to the parked batch (the
  action began with the record parked), or (b) in N-4's departure window, or (c) at the park
  action in the post-pass pipeline at a non-`Priority` wait. If a hit falls outside (a)–(c), stop
  and return it with its stack trace.
- **E-U4-3** (before D9). Every `ActionResult` that `reduce_non_priority_pass_action` returns
  while a batch was parked at the action's start has `action_event_start <= events.len()` (the
  `debug_assert!` of D9, run over the three P2 test files).
- **E-U4-4** (before deciding T-4a's base expectation). At `ff069773b5`, T-4a (Dralnu's settled
  sacrifice with Juri) counts Juri's triggers. If it counts 4 (each sacrifice queued by
  `batch_or_drain_observer_triggers` and folded again), T-4a is red at the U4 base. If it counts 2,
  T-4a is a derived-value guard whose discriminating power is MP-3.
- **E-U4-5** (before D8's drain gate). In T-5's accept, name the drain that runs at `Priority` with
  the batch parked (`effects/change_zone.rs`, `effects/mod.rs`, or the helper). Marker: a hit
  counter inside `can_drain_deferred_triggers` that fires while the record is parked.
- **E-U4-6** (after D8–D9). N-4 is green, and its concession still finishes the batch within its
  16-action bound.

---

## 4. The U4 decisions

### D8 — While a combat batch is parked, it owns the answer's collection (CR 510.2 + CR 510.3a + CR 603.3 + CR 704.3)

**One predicate.** `combat_damage::parked_combat_damage_batch_owns_collection(state: &GameState)
-> bool`, true iff `state.pending_combat_lifelink.is_some()`. It sits beside
`ParkedCombatDamageAnswer` in `combat_damage.rs`, which owns the record and the fold. It stores
nothing and recognizes no prompt, card or event.

Why the state alone is enough: the record is in the state only while the batch is parked
(§2, E-U4-2). Then no player may receive priority (CR 510.2). Whatever an action does in that
time belongs to the batch's one trigger batch, which completes before the active player gets
priority (CR 510.3a). State-based actions are checked only when a player would get priority
(CR 704.3). So a local collector or settlement that runs while the record is parked defers to the
batch. Every route out of the parked state reaches a convergence that collects what it deferred:
the answer exit (D9), or the departure re-entry in `resolve_combat_damage` followed by that pass's
pipeline (N-4).

**Its consumers** (each consults the predicate before it does any work):
1. **The four observer helpers** (`triggers.rs`), the sidecar precedent's exact placement. A new
   private `fn local_collection_is_owned(state: &GameState) -> bool` returns
   `triggered_mana_sidecar_owns_local_collection(state) ||
   combat_damage::parked_combat_damage_batch_owns_collection(state)`. The four helpers call it in
   place of their sidecar call. They are the answer-local collectors of every direct handler
   (`engine_payment_choices`: Opponent-may, unless payment, Tribute, Aura host; `engine.rs` Aura
   and token host). Each does ordinary and delayed collection and drains, so one early return
   covers all three.
2. **The action-end queue**, `triggers::collect_action_end_triggers_into_deferred`: an early return
   while parked. Every resolution-choice completion queues through it directly (§2), and so does
   `collect_answer_slice_matching` beneath the helpers. One gate there covers all ten
   `engine_resolution_choices` sites without editing that file. It consults the combat predicate
   only. The sidecar never stood this primitive down, and U4 does not change the sidecar.
3. **The drain gate**, `triggers::can_drain_deferred_triggers`: return `false` while parked, beside
   its CR 608.2n final-part clause. Zone moves inside a rider's work keep collecting their own
   observers at the move (CR 603.10a look-back; unchanged). What waits is their placement
   (CR 603.3). `drain_deferred_trigger_queue`'s callers in `effects/change_zone.rs`,
   `effects/mod.rs` and the helpers are all behind this gate. The executor's proposal put this
   clause in `resolution_completion_can_settle`. That predicate also feeds
   `turns::phase_transition_requires_settlement`, `engine::resolution_instructions_are_done`,
   `stack.rs` and `engine_priority::settle_pending_resolution_completion`. U4 needs none of those
   widened. The final-part clause is the code's own precedent for a drain-only gate: its comment
   says it is "Not part of `resolution_completion_can_settle`: that predicate feeds
   `engine::resolution_instructions_are_done`".
4. **The paused exit**, `triggers::collect_paused_exit_triggers_matching` (the body both
   `collect_paused_exit_triggers` entry points use): an early return while parked. Its callers are
   the epilogue's staged-payment exit, `engine_payment_choices`, `payment_transaction.rs` and
   `effects/token.rs`. The epilogue's exit block in the reducer body also clears
   `consumed_before_priority_trigger_events` right after that call. While the batch owns
   collection, that clear would erase the claims the batch made when it re-parked inside this
   action, and D9's fold, which runs after the body, would take those events a second time. So the
   whole block (the collect and the clear) is skipped while the batch owns collection: one added
   conjunct on its `if`. It is reached only by a payment of draws paused inside the batch
   (Breathstealer's Crypt, §19) or a staged payment, so no row reaches it today.
5. **The settlement pipeline**, `engine_priority::run_post_action_pipeline_from_with_policy` (the
   core under every wrapper): when `default_wf` is `Priority` and the record is parked, return
   `Ok(default_wf.clone())` before staging, state-based actions, scanning or draining, and leave
   `announcement_event_scope` unadvanced. A pipeline asked to settle a non-`Priority` wait (the
   park action's post-pass pipeline, claim E-U4-2(c)) is unchanged. This covers the epilogue's
   pipeline (D9 needs it) and every handler-internal pipeline
   (`engine_payment_choices::handle_unless_payment`, the sidecar's settled wrapper).

**Not consumers:** the batch's own collection (`process_combat_damage_triggers`,
`collect_triggers_for_batch`), which runs with the record taken; elimination's departure
collectors (`collect_departure_triggers_into_deferred`, `collect_unclaimed_triggers_into_deferred`),
whose triggers wait in `deferred_triggers` behind consumer 3; and the zone authorities'
move-time collection (CR 603.10a).

**One retained-events authority.** `take_answer_events_into_batch` excludes
`triggers::paused_exit_retained_events(state)` (made `pub(crate)`) in place of its inline copy. The
set is the same: logical zone events, cost events and deferred entry events.

### D9 — Every answer arm converges at the reducer's one exit (CR 510.2 + CR 510.3a + CR 615.5)

`engine::apply_non_priority_pass_action` becomes the convergence wrapper. Its body is renamed
`reduce_non_priority_pass_action`, same signature, and is unchanged except for two deletions:
- the `ParkedCombatDamageAnswer::begin` binding before its `match`, and
- the epilogue's `resume_parked_combat_damage_after_answer` block (between
  `resume_parked_counter_cost_root_at_action_boundary` and the post-action pipeline).

The wrapper:

```rust
fn apply_non_priority_pass_action(
    state: &mut GameState,
    actor: PlayerId,
    action: GameAction,
    events: Vec<GameEvent>,
    semantic_actor: PlayerId,
) -> Result<ActionResult, EngineError> {
    // CR 510.2: a combat damage batch parked as this action begins is waiting on
    // the prompt it answers.
    let parked_answer =
        super::combat_damage::ParkedCombatDamageAnswer::begin(state, events.len());
    let mut result = reduce_non_priority_pass_action(state, actor, action, events, semantic_actor)?;
    if let Some(answer) = parked_answer {
        converge_parked_combat_damage_answer(state, answer, &mut result)?;
    }
    Ok(result)
}
```

`converge_parked_combat_damage_answer(state, answer, result: &mut ActionResult) ->
Result<(), EngineError>`, beside `run_boundary_convergence_pipeline`:
- A result that is not `disposition.is_applied()` is returned untouched (the boundary restores
  its snapshot).
- `debug_assert!` that the answer's anchor is `<= result.events.len()` (E-U4-3). `ParkedCombatDamageAnswer`
  gets a `pub(crate) fn action_event_start(self) -> usize` accessor for it.
- Call `resume_parked_combat_damage_after_answer(state, answer, &result.waiting_for, &mut result.events)`.
  - `None`: either the answer left another prompt and the call folded its events into the batch
    and claimed them, or nothing is parked any longer (`handle_replacement_choice` resumed it).
    The result stands.
  - `Some(wait)` with `wait` at `Priority`: set `result.waiting_for = wait` and call
    `run_boundary_convergence_pipeline(state, result, 0)`. That pipeline is the epilogue's own
    (`run_post_action_pipeline_from` from 0, no inline skip, drain allowed), and it writes both
    waits. The batch's events were claimed for ordinary collectors as the batch completed, so the
    pipeline places only what the batch did not collect (delayed triggers, CR 603.7; zone-move
    observers queued at the move) and runs state-based actions once.
  - `Some(_)` with `state.waiting_for` at `GameOver` (a CR 104.1 stop inside the batch): call
    `match_flow::handle_game_over_transition(state)` and copy `state.waiting_for` into the result,
    as the epilogue's game-over branch does.
  - `Some(wait)` otherwise (the batch parked again on a later prompt, or an `OrderTriggers` prompt
    from its trigger batch): write `wait` to both `state.waiting_for` and `result.waiting_for`.

Why this works for both arm kinds:
- A fall-through arm reaches the epilogue with the batch still parked. Its pipeline stands down
  (D8.5) and it returns its wait unchanged. Then the wrapper resumes and settles, exactly as the
  old epilogue call did, one level up.
- A direct arm's handler collected and settled nothing while the record was parked (D8.1–D8.4).
  The wrapper is the first collector its events meet.
- `handle_replacement_choice` (main's #9670 `ChooseReplacement` wrapper) keeps its own resume. It
  leaves nothing parked at `Priority`, so the wrapper then finds nothing to resume. If its resume
  re-parks, the batch has already published and claimed its events, and the wrapper's fold finds
  them claimed and adds nothing. The old epilogue call behaves the same way at `ff069773b5`.
- An action that parks the batch for the first time (the pass that enters the combat damage step,
  an `AssignCombatDamage` answer) began with nothing parked. It gets no `ParkedCombatDamageAnswer`,
  as today.

**What changes for the departure window.** After a concession leaves `Priority { next }` with the
batch parked (main's #9670 departure path), D8 also defers that window's settlement. A
`PassPriority` there settles nothing until the final pass re-enters `resolve_combat_damage` and
finishes the batch; that pass's pipeline then settles everything (N-4). Any other action in the
window reaches D9's wrapper and finishes the batch in its own action, with its events folded into
the batch's trigger batch. The window itself is pre-existing (§19).

**Rejected alternatives** (the executor's design, evaluated against the source):
- *A labelled funnel replacing each direct return.* About 19 `return Ok(..)` sites in the body
  would need to change. An arm added later would escape the funnel. The wrapper is reached by
  every arm by construction, because `apply_action` calls the body only through it.
- *Converging at `apply_action_boundary_core`'s follow-up*, next to the counter-cost root's
  boundary resume. `apply_action` is re-entered by the auto-pass and shortcut loops, and the
  in-function point is reached by all of those re-entries. A concession would also be routed
  through it, which would change main's departure path.
- *Keeping the epilogue call and adding the wrapper for direct returns.* That gives two resume
  sites. The re-park fold would run twice on fall-through arms.

### D10 — Documents (comment-only)

- `combat_damage::resume_parked_combat_damage_after_answer` and `ParkedCombatDamageAnswer`: their
  one caller is `engine::converge_parked_combat_damage_answer`, for every answer arm.
- `combat_damage::resolve_combat_damage`'s re-entry comment: an answer finishes the batch in its
  own action through `engine::apply_non_priority_pass_action`, direct-return arms included; the
  re-entry is the departure path.
- The epilogue comment above the post-action pipeline in `reduce_non_priority_pass_action`: while
  a batch is parked the pipeline defers, and the wrapper finishes the batch.
- `triggered_mana_sidecar_owns_local_collection`'s doc ("Both observer helpers fail closed on this
  predicate"): it is now one of the two owners `local_collection_is_owned` consults.
- `engine_replacement::handle_replacement_choice`'s doc names
  `engine::apply_non_priority_pass_action` as the other answers' boundary. That stays true, so no
  edit is needed.

---

## 5. Sizing

| Unit | Behavior | Registration surfaces | Discriminating tests |
|---|---|---|---|
| U1 Every combat rider pause asked (delivered) | plan r3 D1–D3, D5, D6 | plan r3 §5 | plan r3 §5 |
| U2 Guards read once across a park (delivered) | plan r3 D4 | plan r3 §5 | N-3 |
| U3 A replacement action's "may" keeps its amount (delivered) | plan r3 D7 | plan r3 §5 | N-5…N-8, UT-1, UT-2 |
| **U4 An answer to a parked combat batch converges once** | D8 (one owner predicate, five consumers), D9 (the reducer's exit), D10 (docs) | `combat_damage.rs` (predicate, accessor, fold reuse), `triggers.rs` (helpers, action-end queue, drain gate, paused exit, `paused_exit_retained_events` visibility), `engine_priority.rs` (pipeline core), `engine.rs` (wrapper, converge, two deletions, the paused-exit guard) | T-1 (committed), T-2, T-3, T-4a/b, T-5, N-4 (departure), plus the V1 invariant in `fight_driving` over every combat row |

Dependency edges:
- **U4 → U1.** U4 consumes U1's live rider park: its rows need the prompts U1 asks.
- U4 has no edge with U2 or U3.

Order: U2, U1, U3 delivered; U4 as Commit E on `ff069773b5`.

Scope paths under the phase-fit counting rule (directories expanded; fixtures, regenerated data
and protocol surfaces excluded; standing-class paths counted):
- r3's 28 counted paths (plan r3 §5, 1–28), unchanged.
- `crates/engine/src/game/triggers.rs` is r3's path 22. It was admitted only for a
  compiler-forced test literal; U4 makes production changes in it. That is a new kind of change in
  the file, so this plan admits it. It is not a new path.
- **New: `crates/engine/src/game/engine_priority.rs`** (path 29).
- U4 edits in paths already counted: `engine.rs` (5), `combat_damage.rs` (2),
  `unpreventable_damage_prevention_effects.rs` (8).
- Not edited by U4: `engine_payment_choices.rs`, `engine_resolution_choices.rs`,
  `engine_replacement.rs`, `types/game_state.rs`, `damage_substitution_replacements.rs`. Their
  collectors stand down through D8's consumers. If E-U4-2 shows a collector in one of them that
  bypasses D8's consumers, that is a stop-and-return, not an in-round edit.
- No serialized type changes, so no protocol record (§14).

**Count: 29 counted paths. T1: 4 units. T2: 29 ≥ 13. The T1∧T2 conjunction fires.**
- It fires without U4: r3 already fired it (3 units, 28 paths).
- Without U3's compiler-forced literal-only files (16–28, except `triggers.rs`), the count is 17.
- U4 alone is 1 unit over 5 paths (`engine.rs`, `combat_damage.rs`, `triggers.rs`,
  `engine_priority.rs`, the test file). It does not fire.
- Routing is the orchestrator's (§19).

---

## 6. Building Blocks

- `combat_damage::ParkedCombatDamageAnswer::begin`, `resume_parked_combat_damage_after_answer`,
  `take_answer_events_into_batch`, `claim_combat_lifelink_batch_events_for_ordinary_collection`
  (all delivered in P2): the resume and the custody fold, reused unchanged except for the retained
  set.
- `triggers::triggered_mana_sidecar_owns_local_collection`: the precedent owner predicate, kept
  as one of `local_collection_is_owned`'s two owners.
- `triggers::paused_exit_retained_events`: the single retained-events authority, now shared by the
  paused exit and the fold.
- `triggers::can_drain_deferred_triggers`: the single drain gate (final-part clause precedent).
- `engine::run_boundary_convergence_pipeline`: the existing "converge a result onto the pipeline
  and write both waits" helper, reused by D9.
- `match_flow::handle_game_over_transition`: the epilogue's game-over step, reused by D9.
- Proposed new helpers, with the closest existing authority:
  - `parked_combat_damage_batch_owns_collection` is to the batch what
    `triggered_mana_sidecar_owns_local_collection` is to the sidecar. It is not merged into the
    sidecar predicate, because its reach differs: the sidecar stands down only the four helpers;
    the batch also stands down the action-end queue, the drain gate, the paused exit and the
    pipeline.
  - `local_collection_is_owned` is the one place the four helpers ask "does an owner hold these
    events". It replaces four copies of the sidecar call.
  - `converge_parked_combat_damage_answer` sits at the only place that sees every arm's result.
    `run_boundary_convergence_pipeline` cannot be called by itself there, because the resume must
    run first.

## 7. Logic Placement

| Behavior | Owning function | Delegating callers |
|---|---|---|
| "The batch owns this action's events" | `combat_damage::parked_combat_damage_batch_owns_collection` | `triggers::local_collection_is_owned`, `triggers::collect_action_end_triggers_into_deferred`, `triggers::can_drain_deferred_triggers`, `triggers::collect_paused_exit_triggers_matching`, `engine_priority::run_post_action_pipeline_from_with_policy` |
| Finishing the batch after any answer, then settling once | `engine::converge_parked_combat_damage_answer` (via the `apply_non_priority_pass_action` wrapper) | none: `apply_action` reaches the body only through the wrapper |
| What the batch takes from an answer | `combat_damage::take_answer_events_into_batch` (unchanged rule, shared retained set) | `resume_pending_combat_lifelink`, `resume_parked_combat_damage_after_answer` |
| The batch's own CR 616.1 order and a rider's `ChooseReplacement` | `engine_replacement::handle_replacement_choice` (main's, unchanged) | the `ReplacementChoice` arms |

Consumers whose behavior changes with the same signature (cardinality and timing):
`handle_opponent_may_choice`, `handle_unless_payment`, `handle_tribute_choice`, the Aura and token
host arms, every resolution-choice completion that queues through the action-end primitive, and
`run_post_action_pipeline_from` in the departure window. Each defers while a record is parked.
None changes when nothing is parked: the predicate is false, and every consumer runs exactly as at
`ff069773b5`.

## 8. Rust Idioms

- A `bool` predicate function, not a stored flag. The fact is already in the state
  (`Option<Box<PendingCombatLifelink>>`).
- `if let Some(answer) = parked_answer`; `let Some(wait) = … else { return Ok(()) };`; the waits
  matched with `matches!`. The `GameOver` check reads `state.waiting_for`, as the epilogue does.
- The `debug_assert!` documents the anchor's construction invariant. No clamping silently hides a
  violation.
- No new type, no enum variant, no serialized field.

## 9. Nom Compliance

No file under `crates/phase-parser/src/parser/` changes.

## 10. Extension vs Creation

Extension. D8 extends the sidecar stand-down pattern with a second owner. D9 moves the existing
resume site from the epilogue to the wrapper around the body, so it covers every arm. The only new
function bodies are a predicate, a two-owner OR, the wrapper and its converge step.

## 11. Analogous Trace (Step 2)

- **Traced: the triggered-mana sidecar stand-down** (CR 605.4a).
  `triggers::triggered_mana_sidecar_owns_local_collection` → its four consumers in `triggers.rs`
  (`collect_and_drain_entry_observer_triggers_if_settled`,
  `park_observer_triggers_if_paused_matching`, `park_delayed_observer_triggers_if_paused`,
  `collect_and_drain_observer_triggers_if_settled`) → its drain gate in
  `triggers::resolution_completion_can_settle` (`pending_triggered_mana_resume.is_some()`) → the
  owner's convergence in `engine_payment_choices::handle_opponent_may_choice`
  (`with_accepted_triggered_mana_action_scope` → `finish_accepted_triggered_mana_action` →
  `engine_priority::run_post_action_pipeline_from_settled_priority` for a direct-return arm that
  skips the epilogue).
  - Copied: the owner predicate consulted first by local collectors, the owner's own convergence,
    and settlement for a direct-return arm.
  - Not copied: the drain gate's placement (D8.3 explains why) and the per-handler wrapper (D9
    replaces it with one reducer-level wrapper).
- **Traced: the parked counter-cost root**. `engine::resume_parked_counter_cost_root_at_action_boundary`
  is called both in the epilogue and in `apply_action_boundary_core`'s follow-up, and the
  follow-up converges through `run_boundary_convergence_pipeline`. D9 reuses that pipeline helper.
  It does not copy the two call sites (§4, rejected alternatives).
- **Traced: plan r3's D5** (`8b16c4a374`) and its custody fold (`4d37cfed3d`):
  `ParkedCombatDamageAnswer::begin` → `resume_parked_combat_damage_after_answer` →
  `resume_pending_combat_lifelink` / `take_answer_events_into_batch` →
  `finish_combat_damage_sub_step` → `process_combat_damage_triggers` →
  `claim_combat_lifelink_batch_events_for_ordinary_collection`.

## 12. Verification Matrix (with Identity / Provenance)

Production entry for every runtime row: `GameRunner::act` through the public boundary, driven by
`fight_driving` / `fight_answering` (`unpreventable_damage_prevention_effects.rs`). The completion
boundary is the answering action's result (`Answered`) and the end of combat.
- `fight_driving` asserts V1 after every answer: if the answer left `Priority`, then
  `pending_combat_lifelink.is_none()`.
- Each row's negative assertion is paired with a reach guard: the prompt was asked, or the damage
  was prevented.

| ID | Claim | Seam | Row(s) | Revert-failing assertion | Reach guard | Siblings / hostile | Provenance |
|---|---|---|---|---|---|---|---|
| V17 | A direct-return answer that settles finishes the batch in its own action (CR 510.2, CR 510.3a) | D9 wrapper; D8.1 | T-1 (both answers) | after the answer, `!answer.batch_parked`; Crawler on the stack once (decline) | `offered == [P1]`, `prevented == 1` | accept (no draw; P1 −2) | anchor = `events.len()` at the wrapper's start, a local; never stored |
| V18 | A direct-return answer that leaves the next offer keeps its events in the batch (CR 603.2, CR 603.3) | D9 → `resume_parked_combat_damage_after_answer` re-park branch; D8.1 | T-2 | after the first decline: batch parked, stack empty, `cards_drawn(record.batch_events, P0) == 1`; after the second: Crawler ×2 on the stack, none earlier; a save and load at the second offer keeps the count | two offers to P1, two prevented | accept-first variant: Crawler ×1, P1 −2 | `PendingCombatLifelink::batch_events` (serialized; delivered in `4d37cfed3d`) |
| V19 | A multiplayer re-offer after an acceptance keeps the payment's life loss in the batch (CR 101.4, CR 119.4, CR 603.3b) | as V18 | T-3 | after P1's acceptance: batch parked, P2 offered, stack empty, no Exquisite Blood trigger anywhere but the record; after P2's decline: Exquisite Blood once in the step's trigger batch | `offered == [P1, P2]`; P1 lost 2 | P2 accepts too (P2 −2 and a second Exquisite Blood trigger) | as V18 |
| V20 | A resolution-choice completion's queued observers reach the batch once (CR 603.2c, CR 701.21a) | D8.2 (the action-end queue) | T-4a | Juri's `Sacrificed` triggers == 2 in the combat step's trigger batch; Juri ends with 2 counters | `EffectZoneChoice` asked of P1 with count 2 | T-4b (direct return: the sacrifice paused for Rest in Peace vs Darksteel Colossus) | the queue's stand-down holds while the record is parked; the fold is the only collector |
| V21 | A direct-return re-park whose partial events a paused owner already holds collects them once (CR 616.1, CR 603.2c) | D9 re-park branch; D8.6 retained set | T-4b | after the sacrifice answer: batch parked on P1's `ReplacementChoice`, stack empty; after `ChooseReplacement`: Juri ×2 once, the land exiled, the Colossus in the zone P1 chose | the `ReplacementChoice` offers Rest in Peace and Darksteel Colossus | both choices (exile, library) | the batch delivery's `deferred_events` own the partial events (`zone_pipeline` drains them out of the buffer) |
| V22 | A zone move's observer queued during an answer is not placed before the batch's triggers (CR 603.3b APNAP) | D8.3 (the drain gate) | T-5 | after the answer: P0's Exquisite Blood trigger is the bottom of the stack, and both of P1's Syr Konrad triggers are above it or in P1's order prompt; none of them is below it | P1 accepted (paid 2); the revealed card is a creature card in P0's graveyard | decline variant: no Konrad trigger for the card, Crawler-free board | E-U4-5 names the drain the gate blocks |
| V23 | The departure path still finishes the batch, once (CR 800.4a) | D8 consumers in the window; `resolve_combat_damage` re-entry | N-4 (existing) | unchanged assertions; added: `deferred_triggers.is_empty()` and `stack.is_empty()` once the record drains | P1 was asked the order | — | — |
| V24 | No consumer changes when no batch is parked | D8 (predicate false) | every non-combat engine test (the full suite) | — | — | — | — |

Coverage impact: none. No parse changes, no coverage fence changes.

## 13. Reference Readings

- **T-1 … T-5**: each expected value is derived from the Oracle text and the CR (§16), not from
  a base or sibling reading.
- **N-4**: its expectations were derived in plan r3 (V9: P2's life 20, P0's life 20; CR 800.4a).
  U4 adds only "nothing left deferred or on the stack". That too is derived: Awe Strike's gain is
  P1's, P1 left, and no other trigger exists on the board.
- **T-4a at the U4 base** (E-U4-4). If the base gives 2, the row is still derived ("Whenever you
  sacrifice a permanent" for two sacrificed permanents is two triggers, CR 603.2c), not parity.

## 14. Serialized surfaces

None. No type, field or variant changes. `PendingCombatLifelink::batch_events` (delivered,
`a26a16db` record) already carries the custody that T-2 reloads. No protocol record.

## 15. Skills and checklist steps (Step 1)

- `/add-interactive-effect`: no new `WaitingFor`/`GameAction`.
  - Phase 3, engine handler: D9 at the reducer's exit.
  - Phases 4–6, AI, routing and frontend: unchanged prompts and answers.
  - Phase 7, visibility: `pending_combat_lifelink` is already filtered.
  - Phase 8, tests: §16. They cover the decline and accept paths, the re-park, a save and load at
    the second prompt, multiplayer, and departure.
- `/casting-stack-conditions`: the `WaitingFor`/`GameAction` state machine's exit changes (D9).
  No new casting step.
- `/card-test`: production entry and completion boundary per §12.
- `/add-replacement-effect`, `/add-engine-variant`, `/add-card-data-pipeline`: not applicable.
  No replacement definition, variant or card data changes.

## 16. Tests

All new rows go in `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs`,
in the Zur's Weirding section beside T-1 and the Dralnu section beside
`dralnus_sacrifice_choice_is_asked_inside_the_batch`. They reuse `rider_board`,
`swans_of_bryn_argoll`, `fight_answering` / `fight_driving`, `cards_drawn`, `prevented`, `life`,
`hand_size`, `bolt_at` and `assert_batch_settled`. Add the Oracle constants verbatim from §0:
`EXQUISITE_BLOOD`, `JURI_MASTER_OF_THE_REVUE`, `REST_IN_PEACE`, `DARKSTEEL_COLOSSUS` and
`SYR_KONRAD_THE_GRIM` (reuse a file-local constant where one exists).

- **T-1** `zurs_weirding_offer_inside_the_combat_batch_completes_it`: committed in `ff069773b5`
  and red there. Unchanged.
- **T-2** `zurs_weirding_second_offer_keeps_the_first_draw_in_the_combat_batch`.
  - Board: P0's Grizzly Bears (2/2) attacks; P1's Swans of Bryn Argoll blocks; Zur's Weirding is
    P1's; P0 has Psychosis Crawler and one card in hand.
  - Swans prevents 2 (CR 615.5), so P0 draws 2. By Zur's ruling each draw is replaced one at a
    time, so P1 is offered twice.
  - Variants:
    - decline, decline: P0 draws both. Crawler triggers twice (CR 603.2c). After the first answer
      the batch is parked on the second offer with the first draw in `batch_events` and the stack
      empty. After the second answer it is not parked, and the Crawler is on the stack twice.
      P1 loses 2.
    - accept, decline: P1 pays 2 (CR 119.4) and the first card goes to P0's graveyard. P0 draws
      one card, the Crawler triggers once, and P1 loses 3 in all.
    - each with a `serde_json` save and load at the second offer, as `new_way_forwards_draws_ask_abundance`
      does.
  - Swans deals 4 to the Bears, which dies.
- **T-3** `zurs_weirding_offer_passed_to_the_next_player_keeps_the_payment_in_the_combat_batch`.
  - Board: `GameScenario::new_n_player(3, seed)` with eight library cards each, as in N-4. P0's
    Suntail Hawk attacks P1; P1's Swans blocks; Zur's Weirding is P2's; P0 has Exquisite Blood.
  - Swans prevents 1, so P0 draws 1. "Any other player" is P1 then P2 (CR 101.4).
  - Variants:
    - P1 accepts: P1 loses 2 (CR 119.4) and the revealed card goes to P0's graveyard. The engine
      re-offers the poll after a `PayCost` acceptance (`acceptance_leaves_the_option_open`), so P2
      is offered; this answer returns directly and re-parks. After it: batch parked, stack empty,
      no priority. P2 declines: no draw (the card was already put into the graveyard). The batch
      completes in P2's answer, and Exquisite Blood's trigger for P1's 2 is on the stack once with
      the step's triggers (CR 510.3a). On resolution P0 gains 2.
    - P1 and P2 both accept: two triggers, P0 gains 4.
- **T-4** `dralnus_sacrifice_triggers_join_the_combat_batch_once`.
  - Board: P0's Grizzly Bears attacks; P1's Dralnu, Lich Lord blocks; P1 has Juri, Master of the
    Revue and two lands. The Bears' 2 becomes P1's sacrifice of 2 permanents (CR 701.21a,
    CR 608.2d).
  - (a) P1 sacrifices the two lands. The `EffectZoneChoice` settles: the fall-through arm through
    `batch_or_drain_observer_triggers`. Juri triggers twice (CR 603.2c) in the combat step's
    trigger batch and ends with two +1/+1 counters. Dralnu's 3 kills the Bears.
  - (b) P0 controls Rest in Peace; P1 also has Darksteel Colossus. P1 sacrifices the Colossus and
    a land. The Colossus's move meets Rest in Peace and its own replacement, so P1 chooses
    (CR 616.1). The sacrifice answer returns directly and leaves the batch parked on that
    `ReplacementChoice` with the stack empty. Both choices are covered: exile, or shuffled into
    the library. The land is exiled (Rest in Peace). Juri triggers twice, once each, after the
    `ChooseReplacement` answer completes the batch. That is the executor's "deferred partial
    events aren't collected twice" row.
- **T-5** `zurs_weirding_accepted_offer_keeps_its_graveyard_trigger_above_the_active_players`.
  - Board: two players. A creature card is on top of P0's library (a vanilla creature card built
    with the scenario's library builder). P0's Suntail Hawk attacks; P1's Swans blocks; Zur's
    Weirding is P1's; P0 has Exquisite Blood; P1 has Syr Konrad, the Grim.
  - P1 accepts: P1 loses 2, so Exquisite Blood (P0, the active player) triggers. The revealed
    creature card goes from P0's library to P0's graveyard, so Syr Konrad (P1) triggers. Swans
    deals 4 to the Hawk, which dies, so Syr Konrad triggers again.
  - CR 603.3b: P0's trigger is put on the stack first and P1's above it. After the answer:
    Exquisite Blood is the bottom stack entry, and both Syr Konrad triggers are above it or in
    P1's `OrderTriggers` prompt.
- **N-4** `a_phase_d_rider_prompt_whose_player_leaves_lets_the_batch_finish`: unchanged, plus
  `assert!(state.deferred_triggers.is_empty() && state.stack.is_empty())` once the record drains.
  It is the departure-path row: `resolve_combat_damage` meets the batch still parked at `Priority`.
- **Regression set, unchanged and green:** every row of the three P2 test files (their
  `fight_driving` V1 check now covers direct-return arms too), `combat_lifelink_replacement_ordering`,
  main's #9670 suites named in `task.md`, `exquisite_blood_routing`,
  `issue_3299_syr_konrad_duplicate_damage`, `optional_effect_remember_legal_actions`, the Dash Hopes
  and Browbeat opponent-may rows (`grep -rln "OpponentMayChoice" crates/engine/tests/integration`),
  and the sidecar rows (`grep -rln "triggered_mana" crates/engine/tests/integration`).

### Red-at-base plan

Run T-2, T-3, T-4a, T-4b and T-5 at the U4 base `ff069773b5` before any D8/D9 edit, and record
each failing assertion:
- T-1, T-2, T-3, T-5: the V1 check in `fight_driving` (`Priority` with a parked batch) or the
  stack-content assertion.
- T-4a: E-U4-4 decides whether it is red there.
- T-4b: expected red only if the re-park drops custody; otherwise it is a guard for MP-1 and
  MP-3.

The rows are also red at `PHASE_BASE_SHA` `00396db22d` (no rider prompt is asked there), but that
redness is not U4's discrimination.

### Mutation probes (each restored after; record which rows go red)

- **MP-1**: the wrapper skips `converge_parked_combat_damage_answer` → T-1, T-2, T-3, T-5 red
  (V1), and the fall-through rows red too (the deleted epilogue call is gone).
- **MP-2**: `local_collection_is_owned` returns only the sidecar predicate → T-1 decline red
  (Crawler collected twice, or on the stack before the batch completes).
- **MP-3**: the action-end queue's stand-down removed → T-4a red (Juri ×4) when E-U4-4 found the
  double; otherwise T-4b.
- **MP-4**: the pipeline core's stand-down removed → a fall-through row whose answer settles with
  the batch parked goes red (`swans_of_bryn_argolls_draws_ask_abundance` or
  `new_way_forwards_draws_ask_abundance`: Crawler count, or a trigger placed before the batch's).
- **MP-5**: the drain gate removed → T-5 red (a Syr Konrad trigger below Exquisite Blood).

A probe that turns no row red means the site is not load-bearing for any row. The executor returns
it with the probe's output. It does not delete the site on its own judgment.

## 17. Step-by-step plan (Commit E, on `ff069773b5`)

1. **E0 (claims).** Establish E-U4-1, E-U4-2, E-U4-4 and E-U4-5 (§3). Run T-2…T-5 at `ff069773b5`
   and record the red-at-base results.
2. **E1 — `crates/engine/src/game/combat_damage.rs`.**
   - Add `pub(crate) fn parked_combat_damage_batch_owns_collection(state: &GameState) -> bool`
     beside `ParkedCombatDamageAnswer`. Doc: CR 510.2 + CR 510.3a + CR 603.3 + CR 704.3, as D8
     words it.
   - Add `pub(crate) fn action_event_start(self) -> usize` on `ParkedCombatDamageAnswer`.
   - In `take_answer_events_into_batch`, replace the inline retained set and the
     `deferred_entry_events` check with `triggers::paused_exit_retained_events(state)`.
   - D10's doc lines for this file.
3. **E2 — `crates/engine/src/game/triggers.rs`.**
   - Add `fn local_collection_is_owned`. The four helpers call it in place of
     `triggered_mana_sidecar_owns_local_collection` (comment: CR 605.4a for the sidecar, CR 510.2 +
     CR 603.3 for the batch).
   - Early returns in `collect_action_end_triggers_into_deferred` and
     `collect_paused_exit_triggers_matching` when the batch owns collection (CR 510.2 + CR 603.2 +
     CR 603.3b).
   - `can_drain_deferred_triggers`: `false` while the batch owns collection, beside the final-part
     clause, with the comment from D8.3 (CR 510.2 + CR 510.3a + CR 603.3).
   - `paused_exit_retained_events` becomes `pub(crate)`.
   - D10's doc line on the sidecar predicate.
4. **E3 — `crates/engine/src/game/engine_priority.rs`.** At the top of
   `run_post_action_pipeline_from_with_policy`, before the perf counter, staging and the core:
   - `if matches!(default_wf, WaitingFor::Priority { .. }) && super::combat_damage::parked_combat_damage_batch_owns_collection(state) { return Ok(default_wf.clone()); }`
   - Comment: CR 510.2 + CR 704.3 + CR 603.3. The batch's answer exit (or the departure re-entry)
     settles.
5. **E4 — `crates/engine/src/game/engine.rs`.**
   - Rename the body to `reduce_non_priority_pass_action`.
   - Delete its `parked_combat_damage_answer` binding and the epilogue's resume block.
   - Add `&& !super::combat_damage::parked_combat_damage_batch_owns_collection(state)` to the `if`
     of the epilogue's paused-exit block (the `collect_paused_exit_triggers` call and the
     `consumed_before_priority_trigger_events.clear()`), per D8.4.
   - Add the wrapper `apply_non_priority_pass_action` and `converge_parked_combat_damage_answer`
     (D9), with CR 510.2 + CR 510.3a + CR 615.5 annotations.
   - Update the epilogue comment (D10).
6. **E5 — tests** (§16): T-2 … T-5 and N-4's added assertion. Run MP-1 … MP-5 and restore each.
7. **E6.** `cargo fmt --all`; commit E as one commit:
   `fix(engine): every answer to a parked combat damage batch finishes it once`.

### Verification (Tilt first; never both Tilt and direct cargo)

- Detect Tilt with `tilt get uiresource clippy`. With Tilt: `./scripts/tilt-wait.sh clippy
  test-engine` (exit 3 = cannot answer, not a failure). Without Tilt: direct cargo in the isolated
  target dir.
- Focused first: the three P2 test files and the regression set (§16).
- Then the full engine suite. U4 touches the shared pipeline core and the drain gate, so V24 is
  the full suite, not a sample.
- Deferred to run-level verification (charter): workspace clippy, all shards, phase-ai `--lib`,
  wasm32, the Kathril stack test, and the GameState size (unchanged: no field).

## 18. Unprobed assertions (each settled at implementation by the named check)

- E-U4-1 … E-U4-6 (§3).
- After the first decline, the second Zur's offer is raised inside the same answer
  (`resume_pending_continuation_if_priority` → the next draw of Swans's `Draw` → Zur's
  replacement), as Abundance's is in `swans_of_bryn_argolls_draws_ask_abundance` (T-2).
- In T-4b the land's sacrifice completes before the Colossus's move pauses, and its events are
  drained into the parked batch delivery's `deferred_events` (`zone_pipeline`), not left in the
  buffer (T-4b).
- In T-3 the poll offers P1 before P2. The opponent-may fan-out orders seats by turn order from the
  drawing player; UT-2(b) in plan r3 read the same order for `AnyPlayer`.
- `process_combat_damage_triggers` places what the batch collected. Whether that placement also
  takes `deferred_triggers` queued at a zone move in the same CR 603.3b batch is not asserted.
  T-5's assertion holds either way (§16).
- `default_wf` is the settlement's wait at every handler-internal pipeline call
  (`handle_unless_payment`'s resumes pass `state.waiting_for`); E3's check reads it.

## 19. For the orchestrator / human

- **Addendum (scope).** One line for `addenda/phase-2`.
  - Decision touched: none broken. Charter decision 3 and plan r3's D5 stand: the record parks like
    the lifelink-gain pause and resumes through `resume_pending_combat_lifelink`. D9 moves D5's
    call site to the reducer's exit, so every answer arm passes it.
  - Paths: `crates/engine/src/game/engine_priority.rs` (new), and production changes in
    `crates/engine/src/game/triggers.rs` (already listed, path 22, for a test literal).
  - Card: Zur's Weirding ("Then any other player may pay 2 life") on Swans of Bryn Argoll's draw.
    Its `OpponentMayChoice` answer returns directly and leaves priority open over the parked batch
    (CR 510.2).
- **Phase-fit.** The conjunction fires: T1 4 units, T2 29 paths (§5). It already fired at r2/r3
  without U4, and entry 27 routed it in-phase through the defective-reference route. That route
  does not cover U4:
  - U4 closes a behavior finding the change introduced (impl review r2: "Introduced by P2's live
    rider park; contradicts V1"). It is not a pre-existing reference defect.
  - SKILL.md routes an introduced finding through Step 6 ("a finding the change introduces is never
    a clean residual … it closes through Step 6"). Its out-of-list sites return to this plan step,
    and chartered.md sizes the revised scope as drift: "an addendum when every charter decision
    still stands." Every charter decision still stands.
  - chartered.md's scope rule then says: "Every extension re-counts T2 over the extended list and
    re-evaluates the full T1 AND T2 conjunction … Only when the conjunction fires does the
    extension take the in-phase route above." The route above, for a phase with landed candidates,
    is: "With landed candidates requiring a new split, stop and propose a re-charter through an
    expansion case rather than restarting or truncating the phase." P2 has landed candidates.
  - **Evidence that no split helps.** A phase that takes U1–U3 without U4 cannot be accepted:
    `zurs_weirding_offer_inside_the_combat_batch_completes_it` (committed) and `fight_driving`'s V1
    check over T-2/T-3/T-5 stay red, and impl review r2's MED finding stays open. A U4-only phase
    after it would re-review the same seam. So a re-charter could only reproduce one P2 phase.
    This is feasibility-exit evidence in the form chartered.md names ("every candidate split point
    named and shown to leave … tests red"). The orchestrator may record it if it takes the
    charter route.
  - **Decision for the orchestrator:** (a) record this evidence as P2's feasibility exit for the
    U4 firing and continue in-phase; or (b) under budget standard, pose the landed-candidate
    re-charter as an expansion case to Jakob. Either way, the next implementation review must
    return 0 behavior findings, or the P2 implementation loop stops (r1 = 1, r2 = 1).
- **Process effort** (entry 35's note stands). P2 has run plan r1–r3, impl r1–r2 and two
  stop-and-returns, with `ff069773b5` landing a red witness. Tell Jakob.
- **Breathstealer's Crypt (pre-existing): file an issue.**
  - Oracle text: "If a player would draw a card, instead they draw a card and reveal it. If it's a
    creature card, that player discards it unless they pay 3 life."
  - Measured parse (`client/public/card-data.json`, built 2026-10-22, before this run): the
    replacement executes `Draw { 1, ParentTarget }` → `RevealTop { player:
    PostReplacementDamageTarget, count: 1 }` → `DiscardCard { count: 1, target: ParentTarget }`,
    gated `RevealedHasCardType [Creature]` with `unless_pay: PayLife 3, payer: Player`.
  - Defects:
    - it reveals the library's *new* top card, not the drawn card ("reveal it");
    - it discards a chosen card rather than "it", the drawn card;
    - the reveal names a damage-replacement player reference on a draw replacement.
    The card carries no `Unimplemented` marker, so coverage likely counts it as supported.
  - Why file, not fix or drop:
    - U4 names no Breathstealer's row and asserts nothing about it, so every case U4 claims works
      with the defect present.
    - The `UnlessPayment` arm is closed structurally by D9.
    - Printed text does reach the arm, so it cannot be dropped.
    - The fix is parser work, outside P2's charter scope.
  - File after a duplicate search, with the parse excerpt above.
- **Departure window (pre-existing, main's #9670): recommend file.** After a concession by the seat
  that owed the batch's answer, elimination writes `Priority { next }` with the batch parked. That
  opens a priority window inside combat damage (CR 510.2) until a pass re-enters
  `resolve_combat_damage`. Any legal action in a multiplayer game reaches it. U4 does not close it:
  D8 defers settlement there, and D9 finishes the batch on any non-pass action. N-4 keeps it green.
  Closing it means finishing the batch inside the concession action. That changes main's
  departure path, so it is out of P2.
- **Loop accounting.** This revision is design round 3 of the P2 plan loop (an executor
  stop-and-return). Its review is the next design-round comparison point. The fifth-round limit
  leaves two further design rounds.
