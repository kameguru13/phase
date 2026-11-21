# Task: reconcile PR phase-rs/phase#10639 with main's #9670 (one authority per seam)

## Original request

Jakob (Command Zone #159, 2026-11-17): "We rework it." The maintainer (matthewevans) asked that #10639
compose with #9670 (Soul-Scar Mage / Szadek damage substitution, merged to main 2026-11-16 as
`ed6fd9aef4`) with **one authority per seam, built on main's #9670 shapes**. His note is verbatim in
`maintainer-hold-2026-11-17.md` beside this file. The PR body (what #10639 claims and fences) is in
`pr-body.md`.

## Refs

- PR head / BASE_SHA: `2fc5a9188e` (branch `engine/unpreventable-damage-riders`, worktree
  `/mnt/data/dev/phase/.claude/worktrees/bridge-cse_01KG1sqdiB2vcrN2pv7X3nVa`).
- Main to merge: `upstream/main` = `91639fdf02` (contains #9670 `ed6fd9aef4`, the `LossReason`
  elimination change #10810, the GameState ceiling raise to 16,384 in `game_state_size.rs`).
- `git merge-tree --write-tree --name-only 2fc5a9188e upstream/main` conflicts in 11 files:
  combat_damage.rs, effects/deal_damage.rs (24 hunks, ~1,000 lines), effects/mod.rs,
  effects/win_lose.rs, elimination.rs, engine.rs, quantity.rs, replacement.rs (7 hunks, ~900 lines),
  types/game_state.rs, types/game_state_size.rs, scripts/post-replacement-continuation-baseline.txt.
- #9670's own engine diffs: `git show ed6fd9aef4 -- <path>` (its commit message documents its design
  round by round; read it).

## The four design seams (maintainer's recommendation: build on #9670's shapes)

1. **Replaced-event amount.** Both sides add `PostReplacementDrain::event_amount`. #10639: every
   replacement/prevention applier stamps it (`REPLACEMENT_AMOUNT_STAMP`, read via
   `replacement_action_amount`). Main: set only for damage substitutions, read via
   `post_replacement_event_amount()`, installed from a `ReplacedEventContext`. One meaning, one reader.
2. **Damage substitution (CR 614.1a).** #10639: `ShieldKind::Substitution`, substitutes held until the
   batch's damage is dealt. Main: `is_damage_substitution` requires `ShieldKind::None` and drains each
   substitute immediately. The predicates exclude each other: fold into one substitution predicate.
3. **Combat replacement choices.** #10639 settles combat order/rider prompts automatically (fenced as
   `DamageReplacementChoiceUnaskedInCombat`). #9670 parks the combat batch and asks the owing player in
   APNAP order. Keep main's asked order choice.
4. **Multi-recipient noncombat pause and tail ownership.** #10639's `run_damage_batch` /
   `Effect::ResumeDamageBatch` vs main's `DamageTail::{Stashed, Caller}` +
   `TailOwnership::Transferred`. Only one can stand; build on main's.

Mechanical: import unions; thread main's `LossReason` through `lose_by_effect` / `win_by_effect` /
`settle_event_outcomes` (EffectStatedLoss for effect losers and their CR 810.8a shared-life teammates,
OpponentWon for a winner's opponents, via `eliminate_players(.., DepartureTriggers::ScannedByCaller, ..)`);
`game_state_size.rs` takes main's 16,384 ceiling (keep this branch's boxed `damage_batch_ledger` note
only if the field survives); regenerate the post-replacement continuation census baseline.

## Acceptance

- One authority per seam above; no parallel mechanism survives.
- Main's #9670 suites pass unchanged in intent: soul_scar_mage_damage_to_counters,
  szadek_combat_damage_to_counters, undead_alchemist_combat_damage_to_mill,
  adamantoise_vigor_combat_order, combat_hit_bookkeeping_follows_delivery, awe_strike_prevention,
  ultimate_magic_meteor_per_opponent_destroy, swans_prevention_followup.
- #10639's suites pass: damage_substitution_replacements, unpreventable_damage_prevention_effects,
  Ironscale Hydra and the PR's other integration and unit tests. Where a #10639 test encodes a design
  #9670 replaces (e.g. a combat prompt this branch settled automatically and main now asks), the plan
  names the test, the card's Oracle text and CR, and the corrected expectation; never weaken a test to
  get green.
- Everything #10639 claims and keeps must still hold (see pr-body.md): held riders behind the
  simultaneous event (CR 614.6 + CR 615.5), effect-stated outcomes settled together (CR 104.3f,
  CR 104.4a, CR 810.8a), prevention-disabled ordering (CR 615.12), "while" guards, capped-redirect and
  shared-shield fences, substitution-choice fences. Fences whose gap main's #9670 now closes (combat
  choice asked) are removed with the claim updated.
- Coverage/parse impact: no card that #10639 or main supports regresses.
- Verification at the merged head: workspace clippy, phase-engine (all shards, default test stack),
  phase-parser, phase-server, phase-ai --lib, wasm32 check, engine-authorities gates, the Kathril
  stack test within 16 MiB (`kathril_reaches_matching_counter_and_tail_past_false_earlier_gates`),
  GameState under main's ceiling.

## Constraints

- CLAUDE.md design principles (idiomatic Rust, CR-annotated with grep-verified numbers, build for the
  class, engine owns logic, nom combinators for parser code).
- Never write model identifiers into code, commits or PR text. No `git stash`. Don't touch
  `.claude/skills/**`.
- Budget: standard. Ship as a merge of main into the PR branch plus follow-up commits, pushed once
  after verification and an independent implementation review.
