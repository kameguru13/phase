# Phase D plan review, round 3 (focused, final)

Plan: `pr-phase-D-plan.md` (r3), diffed against the r2 copy. Code read at 4483f4b. The review was read-only: no cargo, no probes.

Scope as requested:
1. X3 on `SubtreePosition`.
2. T-D: the loop moved inside `resume_pending_continuation_if_priority`.
3. T-A: keying the enclosing frame by `PostReplacementFrameId`.
4. The T10 reach premise.

## Verdict: no blocking findings. The plan is clean to implement

### 1. X3: adopted correctly

- The exit condition is the r2 replacement verbatim: `!after.is_strictly_inside(&before)`, with `is_strictly_inside` the lexicographic `<` on ℕ×ℕ.
- The measure moved to a private `SubtreePosition (len, active-frame drain count)`, which is positional. That is fine: termination needs only well-foundedness, not that the measure names the subtree.
- `before` and `after` are both sampled around exactly one `resume_pending_continuation_pass`. Any pass that does not strictly decrease the measure exits.

### 2. T-D: the loop inside `resume_pending_continuation_if_priority` is sound

**Exactly one pass with no live subtree.** The exit test runs after every pass. With no `Unfinished` counter-cost root (`Absent`/`Retired`), the second disjunct exits after pass 1. A root that a pass creates or re-parks leaves a `ReplacementChoice`, and the first disjunct exits. No caller among the ~50 sees an extra pass unless a counter-cost subtree is still live.

**Re-entrancy.**
- `RandomDiscardUnlessPayment` is not admitted at `PriorityBoundary` (engine.rs 8783–8791 + D3). So the pass's own cost drain never enters `resume_random_discard_unless_payment` (engine_payment_choices.rs 2595) or `finish_successful_unless_payment` (1970 → 2040). That recursion happens only at the replacement boundaries, as at the base.
- A nested call that is reached through a root resumed inside the pass (`CounterCostRemovalPayment`/`ManaAbilityPayment` resume → … → `resume_pending_continuation_if_priority`) runs after `pending_cost_move_resume.take()`. It therefore reads `Absent` and makes one pass.
- So the recursion depth is the base's, and each invocation is bounded.

**Error propagation.** Pass 1 behaves as the base: `?` propagates. An `Err` on a later pass surfaces in the same action instead of at the next boundary. Callers such as `pass_priority_once_with_pipeline` (engine.rs 9291 at 2d20628; 9441 per the plan) restore their snapshot on `Err`, so nothing is half-applied. The `.expect` exposure is residual 4, unchanged.

### 3. T-A: identity keying

**Stamping.** `begin_active_post_replacement_dispatch` (resolution.rs 2883–2913) is the only production `begin_dispatch` caller; the other at 6069 is a test, and the game_state.rs 338xx calls are unit tests. It stamps at first dispatch through `stamp_frame_id` (game_state.rs ~25160), which stamps at most once. So every frame that holds a `Dispatching` or `Paused` drain is stamped. That covers the enclosing frame P in the case that matters: a paused outer dispatch, which is the T10 and `[.., P, C]` shape.

**`Retired` is exact for a stamped P.**
- `len <= depth` must hold, and so must P's drain count by identity (resolution.rs 2939 `post_replacement_frame` → `FrameVec::by_id`) being at most the recorded count.
- The count cannot fall early under a live nested child:
  - `finish_paused_dispatch` (game_state.rs 25082) pops only the innermost `Paused` drain;
  - `finish_dispatch` (25067) requires `Dispatching`, and the outer drain is `Paused` while it waits on the payment prompt.
- The `Less` short-circuit is gone.

**Non-blocking text correction.** The `EnclosingPostReplacementFrame.frame` doc says "`None` only for a frame restored from a pre-identity payload". The type's own doc (game_state.rs 25128–25132) says an unstamped frame is also "a freshly minted sibling frame, and a journal-replayed frame", and that includes any frame never dispatched. Each of these falls back to the positional read.
- Reword the doc to: "`None` for an unstamped frame: never dispatched, journal-replayed, or from a pre-identity payload".
- Widen residual 7 to match.

This is not blocking: the inexact `[P, C]` case needs a P whose outer dispatch has paused, and such a P is stamped in a live session.

### 4. T10 reach premise is plausible, with a working fallback

- `AbilityDefinition.unless_pay: Option<UnlessPayModifier>` exists (ability.rs 27937; `UnlessPayModifier` 15639).
- `ActivateAbility` at `WaitingFor::UnlessPayment` routes to `handle_unless_payment_activate_ability` (engine.rs 13447–13456; CR 118.12).
- The Drawer's chain has no later instruction, so the stack at the prompt is `[P]` and the Cove's record holds P (stamped) as its enclosing frame.
- At entry 2's decline: the Execute arm's 1360 drain runs the child rider, and `finish_active_paused_post_replacement_dispatch` pops the child, P's innermost `Paused` drain (game_state.rs 27658), not the outer unless-prompt drain. D6(a) then sees `Retired` and resumes the Cove, which returns the `UnlessPayment` prompt.
- Whether a post-replacement `Template` with `unless_pay` raises `UnlessPayment` with a `Paused` drain (P6) is unprobed. It is guarded by T10's first assertion, with the stated drop-to-U2/U3 fallback.
- At the base, T10 is red through the T3 mechanism.
