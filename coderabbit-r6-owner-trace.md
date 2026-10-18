# CodeRabbit r6 — "Match the pending removal to its cost owner" (engine_replacement.rs:1627)

HEAD 2d20628 (branch engine/cost-removed-counters), base 3b828ac. Read-only trace (git/grep/read only).

## Verdict

**Real in structure, introduced by this PR (commit 1cbd2df), not reachable in production.**
At HEAD, no game state can reach the Prevented arm with a `RemoveCounter` pending
replacement: only synthetic test staging gets there, and no printed or parsed card has a
`RemoveCounter` replacement. The guard still picks the owner from the wrong signal:
"any cost root exists" instead of "the cost root owns this removal". So a child removal
is deferred behind its parent cost and settles in the wrong order. The base had a
different defect on the same path: it stranded the parent root. Worth a small fix,
because the PR's own tests already reach this arm through staging.

## 1. How ownership is represented

- `pending_was_counter_removal` (engine_replacement.rs:232-235): a bool. It is `true` iff the
  pending record being answered has `proposed == ProposedEvent::RemoveCounter`. It records
  nothing about the owner.
- Effect-path removals: every `Effect::RemoveCounter` goes through
  `counters::resolve_remove` (counters.rs:3149-3260) → `push_counter_removals(PendingCounterRemovalQueue)`,
  a `ResolutionFrame::CounterRemovals` frame on `resolution_stack` (resolution.rs:432, 1651-1697).
  `drain_pending_counter_removals` (counters.rs:3446) stores the removal that waits on a
  choice in `queue.in_flight: Option<PendingCounterRemovalInFlight>` (game_state.rs:6049-6065).
  That `in_flight` is the effect owner's typed marker.
- Cost-path removals: `costs::pay_counter_cost_removals` (costs.rs:786-822) returns
  `AwaitingReplacementChoice(PausedCounterCostRemoval { in_flight, remaining })` (game_state.rs:6036-6046).
  The record is held by:
  - the stack route: `PendingCostMoveResume::CounterCostRemovalPayment { removal, .. }` (game_state.rs:8294; parked at costs.rs:1906-1915).
  - the mana route: `ManaAbilityCostCursor.counter_removal` inside `PendingCostMoveResume::ManaAbilityPayment { cursor, .. }` (game_state.rs:8112; mana_abilities.rs:4138-4155).
  No queue frame is pushed for a cost removal.
- Non-queue, non-cost removals call `remove_counter_with_replacement` (counters.rs:1476) directly and keep no record:
  - damage to a planeswalker or battle: deal_damage.rs:703/716.
  - resolution-time `PaymentScope::Resolution` cost: costs.rs:1918-1923 (no root).
  - the negative loyalty cost: costs.rs:1791, which ignores the outcome.
  - the unless-payment removals: engine_payment_choices.rs:1718/1733.
  The owner of these is the paused `AbilityContinuation`.
- The Execute arm checks no owner. It runs in a fixed order:
  1. `active_counter_removals()` queue drain (1150-1158).
  2. The continuation drain, skipped while a `ManaAbilityPayment` root is parked (1276-1296).
  3. The cost-root drain `ReplacementDelivered` (1345-1355).
  4. The continuation again (1360-1369).

  So a child queue always settles before its parent cost root.
- The Prevented arm (1624-1635) is the only place that chooses an owner. It uses
  `pending_cost_move_resume.is_none()`, a proxy for "a root exists", not "the root owns this".
  The ownership data the arm could compare against does exist:
  - `active_counter_removals().in_flight`;
  - `CounterCostRemovalPayment.removal`;
  - `ManaAbilityPayment.cursor.counter_removal`.

## 2. Paths with a root parked and a NON-cost removal reaching a choice

The only structural route: the root parks on a replacement choice. The answer is accepted
(Execute), or prevented by a substitute that carries a post-effect. The replacement's
post-effect then runs:
- `apply_pending_post_replacement_effect`: 1022-1040 in the Execute arm.
- the Prevented-arm nested return: 1507-1512.
- `apply_post_replacement_effect`: 2944.

If that post-effect contains `Effect::RemoveCounter`, it pushes a queue frame. Its removal
reaches a `RemoveCounter` choice and parks a new `pending_replacement`. `waiting_for` is then
not Priority, so the root drain at 1345 is skipped and the root stays parked. The child is
then answered.
- The parent root can be any variant: `Cast`, `ActivationMillPayment`, `ManaAbilityPayment`,
  `CounterCostRemovalPayment`, and so on. The parent's own event need not be a counter removal.
  A Moved, Damage or other replacement whose post-effect removes counters works too, for
  example the post-effect shape of Phantom Centaur, Protean Hydra or Unbreathing Horde.
- Other candidate routes checked, none of which give a root plus a child choice:
  - mana-ability effects: a mana ability whose cost paused has not resolved its effect yet;
  - triggers: not resolved during payment;
  - statics: do not remove counters;
  - resolution-time costs: no root exists while a spell or ability resolves.

Reachability limits:
- The child choice needs a `RemoveCounter` replacement that surfaces a choice: two or more
  applicable, or an optional one.
  - The parser emits no `ReplacementEvent::RemoveCounter`. Grep of crates/engine/src/parser finds none.
  - client/public/card-data.json has 0 `"event":"RemoveCounter"`. Its tally includes Moved 2363,
    DamageDone 240, AddCounter 34, and so on.
  - Only `database/forge/replacement.rs:95` maps the string, and no shipped card uses it.
- Prevented specifically is unreachable even with synthetic `RemoveCounter` replacements.
  `remove_counter_applier` (replacement.rs:4697-4704) always returns `Modified`. The prevention
  sources are the gain/lose-life, add-counter and create-token appliers, and the skip-combat
  check at replacement.rs:9724. The PR's tests reach the arm only by re-pointing the candidate
  at a `BeginTurn` definition (`stage_prevented_counter_removal`, tests/integration/cost_removed_counter_amount.rs:1608-1636).

## 3. What happens on that path

**HEAD** (child queue removal Prevented while any root is parked):
- The 1627 guard is false, so control falls through:
  - copy-token and batch checks;
  - Priority reset (1656);
  - `drain_pending_cost_move_resume(ReplacementPrevented)` (1667).
- The parent root settles first, which is the wrong order:
  - an activation is placed on the stack;
  - a cast is finalized;
  - mana is added;
  - for `CounterCostRemovalPayment`, its own `remaining` removals are paid.
- The child `CounterRemovals` frame stays on `resolution_stack` with `in_flight` still `Some`,
  and the post-effect's continuation stays below it. Neither is dropped and nothing panics.
- They drain at the next drain point:
  - the next Execute arm (1150);
  - any `drain_pending_continuation` (effects/mod.rs:1209-1212);
  - `resume_resolution_frames` at the next priority pass (engine.rs:9291 → effects/mod.rs:1696).
- Mana route: the tail's continuation drain (1673-1682) uses `active_ability_continuation()`,
  which is top-only. It sees the `CounterRemovals` frame on top and does nothing.
- Side effect: `settled_in_flight_counter_removal` measures a count delta (counters.rs:3427).
  If the root's later removals, or another removal, touch the same object and counter type
  before the stale queue settles, its `applied_total` and `last_effect_count` absorb them. A
  "that many" rider then reads the wrong number.

**Base 3b828ac**: the guard was an unconditional `if pending_was_counter_removal`
(`git show 3b828ac:crates/engine/src/game/engine_replacement.rs` 1623-1632).
- The child queue and continuation drained first (right order), then the arm returned.
  The root was never drained.
- `PriorityBoundary` (engine.rs:8686-8694) resumes only `FinalizeDelvedCast` and `ManaAbilityPayment`.
  A `Cast`, `ActivationMillPayment` or other root would stay parked with `pending_cast` stuck:
  a stranded root, a pre-existing defect on the same unreachable path.
- The base had no `CounterCostRemovalPayment`, so the PR's own root did not exist yet.

**History**: 7d6397a introduced `&& state.active_counter_removals().is_some()`, which gave the
right owner for queue children but sent damage and resolution-time removals to the cost drain.
1cbd2df replaced it with `&& state.pending_cost_move_resume.is_none()`, which fixed those but
introduced this misrouting. **This PR introduced the ordering inversion; the stranding is pre-existing.**

## 4. Proposed fix (smallest correct)

Choose the owner from the typed records, child first, and mirror the Execute arm's order.
Touches `crates/engine/src/game/engine_replacement.rs` and one helper in
`crates/engine/src/types/game_state.rs`.

1. game_state.rs: add an exhaustive helper on `PendingCostMoveResume`:
   ```rust
   /// CR 118.3 + CR 616.1: the counter-cost removal this root holds while its replacement choice is pending.
   pub(crate) fn paused_counter_removal(&self) -> Option<&PausedCounterCostRemoval> {
       match self {
           Self::CounterCostRemovalPayment { removal, .. } => Some(removal),
           Self::ManaAbilityPayment { cursor, .. } => cursor.counter_removal.as_deref(),
           Self::Cast { .. } | ... => None,   // exhaustive
       }
   }
   ```
2. engine_replacement.rs, Prevented arm, replacing 1624-1635:
   ```rust
   // CR 616.1: the removal's owner is the innermost record holding it in flight —
   // an effect queue (a replacement's child) before a parked cost root.
   let effect_queue_owns_removal = state
       .active_counter_removals()
       .is_some_and(|queue| queue.in_flight.is_some());
   let cost_root_owns_removal = !effect_queue_owns_removal
       && state.pending_cost_move_resume.as_ref()
           .is_some_and(|root| root.paused_counter_removal().is_some());
   if pending_was_counter_removal && !cost_root_owns_removal {
       state.waiting_for = WaitingFor::Priority { player: state.active_player };
       effects::counters::drain_pending_counter_removals(state, events);
       let mana_root = matches!(state.pending_cost_move_resume,
           Some(PendingCostMoveResume::ManaAbilityPayment { .. }));
       if matches!(state.waiting_for, WaitingFor::Priority { .. }) && !mana_root {
           effects::drain_pending_continuation(state, events);
       }
       // A parent cost root resumes only after its child settles (Execute-arm order).
       if !(matches!(state.waiting_for, WaitingFor::Priority { .. })
           && state.pending_cost_move_resume.is_some())
       {
           return Ok(state.waiting_for.clone());
       }
       // fall through to the existing typed cost-move drain + mana continuation tail (1656-1686)
   }
   ```
   The tail at 1656-1686 then resumes the parent root and, for a mana root, the rider.
   This is the same sequence as Execute: queue → continuation (non-mana) → root → continuation (mana).
   It also fixes the base's stranded-root problem for `Cast` and other roots.

   The fall-through skips the copy-token and batch blocks at 1636-1653 only if the code
   returns before them. Put the child block where it is now and fall through as written; those
   blocks are guarded by their own frames and are inert here.

Residual gap: an owner marker would be needed for a NON-queue child under a counter-cost root
whose own choice was already answered. Examples: a post-effect dealing damage to a planeswalker,
or a resolution-scope PayCost removal. The root's `in_flight` stays `Some` until the root drains,
so the helper cannot tell "own choice pending" from "own choice answered". An exact fix needs a
typed state on `PausedCounterCostRemoval`, e.g. `choice: CounterCostChoice { Pending, Settled }`,
set to `Settled` when the root's own event reaches Execute or Prevented. That is serde plus both
routes. It is not worth it for a path that needs an unprinted replacement, a staged prevention
and a post-effect that removes counters through damage. Record it as a known gap, or reply on
the thread that the queue marker covers every effect-path child.

## Test reachability

Reachable through public actions with the PR's existing staging, in tests/integration/cost_removed_counter_amount.rs:
1. Install an optional `ReplacementDefinition::new(RemoveCounter)` whose `execute` is an
   `Effect::RemoveCounter` from another object, e.g. a 1/1 with counters.
2. Activate Essence Bottle. The root `CounterCostRemovalPayment` parks.
3. `ChooseReplacement(ACCEPT)`. The post-effect child queue removal hits the same optional
   replacement, which is a new event, so CR 614.5 allows it to apply again.
4. Assert the child prompt is up. Assert `active_counter_removals().in_flight.is_some()` and that
   `pending_cost_move_resume` is `CounterCostRemovalPayment`.
5. Run `stage_prevented_counter_removal`, then `ChooseReplacement(0)`.
6. Assert:
   - `active_counter_removals().is_none()`, meaning the child settled;
   - the child's `EffectResolved(RemoveCounter)` precedes the activation's stack placement in `events`;
   - the stack holds the activation with the parent's count.

At HEAD the stale frame is still present after step 5.

A sibling row with a `Cast` or `ActivationMillPayment` parent would pin the base's stranding fix.
