<!-- pr-review-held -->
**Held at exact head `2fc5a9188eed7eb15c6c1694917885a2db8d0f8c`: your content approval stands, but #9670 (Soul-Scar Mage / Szadek damage substitution) merged to `main` at 2026-11-16 10:15Z, after our last merge into this branch, and it built a second mechanism on the same seams. This is our doing, not yours.**

External condition: waiting for a decision on how this PR composes with #9670. A trial merge of `main` `894cf094ab` conflicts in 10 files, and four of the conflicts are design choices rather than keep-both:

1. **Replaced-event amount.** Both sides add `PostReplacementDrain::event_amount`. Here every replacement/prevention applier stamps it (`REPLACEMENT_AMOUNT_STAMP`, read via `replacement_action_amount`); on `main` it is set only for damage substitutions and read via `post_replacement_event_amount()`, installed from a `ReplacedEventContext`.
2. **Damage substitution (CR 614.1a).** This PR uses `ShieldKind::Substitution` with substitutes held until the batch's damage is dealt; `main`'s `is_damage_substitution` requires `ShieldKind::None` and drains each substitute immediately. The two predicates exclude each other.
3. **Combat replacement choices.** This PR settles combat order/rider prompts automatically and notes a combat-prompt follow-up; #9670 is that follow-up: the combat batch parks and asks the owing player in APNAP order.
4. **Multi-recipient noncombat pause and tail ownership.** This PR's `run_damage_batch` / `Effect::ResumeDamageBatch` and `main`'s `DamageTail::{Stashed, Caller}` + `TailOwnership::Transferred` each guarantee the spell's tail runs once; only one can stand.

The rest (import unions, threading `main`'s `LossReason` through `lose_by_effect`/`win_by_effect`, the `game_state_size.rs` ceiling now 16,384 on `main`, the continuation baseline) is mechanical. The red CI at this head is the `GameState` size assertion (`game_state_size.rs:142`) tripping after our merge `de3c77a4f7`; `main`'s raised ceiling should clear it once merged.

Next step: we recommend one authority per seam, built on `main`'s #9670 shapes (its combat parking, `DamageTail` ownership and `event_amount` meaning), with `ShieldKind::Substitution` folded into a single substitution predicate. You are welcome to do that rework and push; a maintainer may also take it, or split out the "while" guards and parser rows to land first. Either way, once a merged head is pushed and CI has run, we will review the resolution (`git show --remerge-diff`) and then re-approve and enqueue.

