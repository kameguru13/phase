# Runner report: impl-review r1 fixes (618fb57, parent fad0bf7)

Pushed to engine/cost-removed-counters-wip; pre-push hook passed in full (coverage regression 0 flipped/0 gained). Per deferred-verification policy only fmt, CR grep and targeted rows were run. CR in diff: 601.2h, 602.2b, 603.3, 605.3b, 608.2c, 616.1 (all verified). Head: cost_removed_counter_amount 63 passed / 0 failed / 1 ignored (T12b).

Files: types/game_state.rs (`counter_cost_root_is_due`); effects/mod.rs (gates (a),(b),(c) in `drain_pending_continuation`, gate 3 in `resume_resolution_frames`); engine.rs (`resume_due_counter_cost_root` called twice in the pass; D5 doc; `resume_due_counter_cost_root_at_action_boundary` next to `owed_spell_final_part_is_due`; site 1 before the shared pipeline in `apply_non_priority_pass_action`; site 2 in `apply_action_boundary_core`, scan_from moved above the resume and convergence condition widened); engine_replacement.rs (D6(a) comment); tests (`stage_prevented_pending_event` generalization, `stage_prevented_counter_removal` wrapper).

## Red/green (base = fad0bf7 code + new tests; each column = 618fb57 with one revert)
| Row | base | head | -(a) | -(b) | -(c) | -(3+pass helper) | -both authority sites | -D4(c) |
|---|---|---|---|---|---|---|---|---|
| T10a an_outer_tail_waits_for_the_mana_root_paying_its_unless_cost | RED | green | green | RED | green | green | green | green |
| T10b a_prevented_last_child_entry_leaves_the_outer_unless_post_effect_paused | RED | green | green | RED | green | RED | green | green |
| T10c an_outer_tail_below_the_boundary_waits_for_the_mana_root | green | green | green | green | green | green | green | green |
| T10d a_queued_child_rider_leaves_the_outer_unless_post_effect_paused | RED | green | green | RED | green | RED | green | green |
| T10e an_outer_repeat_waits_for_the_mana_root_paying_its_unless_cost | RED | green | green | green | RED | green | green | green |
| T11 a_prevented_non_counter_child_event_finishes_its_rider_before_the_cost_root | green | green | green | green | green | green | green | RED |
| T12 a_mana_choice_child_finishes_and_places_the_activation_in_the_same_action | RED | green | green | green | green | green | RED | green |
| T12b a_trigger_from_the_childs_payment_goes_on_the_stack_above_the_activation | RED | RED(ignored) | RED | RED | RED | RED | RED | RED |

Gate (a): NO discriminating row. T10a goes red under -(b) instead: cascade stopped by (b) before the recursive call (a) would catch.

## Deviations / findings
1. "C" is not a frame in T10a/T10c. At the unless prompt stack is [PostReplacement] (T10a) or [] (T10c); the outer tail is carried in WaitingFor::UnlessPayment.pending_effect.sub_ability, not a parked AbilityContinuation. Reach replaced with `unless_prompt_holds_its_tail`.
2. Paying the unless cost skips the whole pending effect including its sub_ability tail (pre-existing). T10a/T10c decline instead (-4 = unless loss then tail).
3. T10c green at base (no enclosing frame; root resumes at D6(a) before tail). Kept as control.
4. T11 fixture: after prevention the Bottle's +6 meets the optional GainLife replacement, final drive answers DECLINE. P7 holds.
5. T12b BLOCKED. Witness ("Whenever you lose life, you gain 1 life.") triggers MORE THAN ONCE from the child's single life loss. With both sites, stack after the colour answer: [Witness#10, Bottle activation#11, Witness#12] — one copy BELOW the activation (no StackPushed for #10 in the action's events) plus a copy above. With site 1 removed: [W#10, B#11, W#12, W#13]. Action events contain exactly one LifeChanged(-1). So the life-loss event is collected by more than one trigger pass once the root resumes inside the action. Not checked whether duplicate collection exists in other rows.
6. T10d P10 holds (Clash, empty libraries, queues tail without prompting). T10e P9 holds (RepeatFor frame parked under iteration 1's UnlessPayment).
