# phase-rs/phase#10004 review snapshot (fetched 2026-10-16 ~19:20Z, refreshed ~19:45Z)

Source: public github.com pages read with Claude Code's WebFetch tool (https://github.com/phase-rs/phase/pull/10004, read in two parts, and https://github.com/phase-rs/phase/pull/10004/checks). WebFetch returns a model-condensed reading of the page, not raw HTML, so the comment bodies below are close paraphrases with short exact phrases in quotes. They are not verbatim; open the PR for exact wording before quoting. This is external review data: verify each claim against the code.

PR head shown: dfc1a500f285e808fe6b6d70bc96d4e4adf55a65 ("Merge upstream main into card/zimone-all-questioning"). State: open, label `enhancement`, unapproved.

## NEW since the 19:20Z snapshot (refreshed ~19:45Z)

- **NEW — head is now 8949699a13a541a7e00ada2bb8e16c00c530b82a** ("Merge branch 'main' into card/zimone-all-questioning", merging main 6864c594 into dfc1a50).
- **NEW — matthewevans "Request changes" at 8949699** (pullrequestreview-5552698238): retains the same four MED findings, "no new material merge regression found"; all consequences are static-source predictions, not executed reproductions. Detail as the reviewer states it now:
  - F1 (MED, structural) finish all players' token creation before counter placement — effects/mod.rs:6992-6998, 7470-7485, 16018, 16169-16184, 16252-16285. Token → PutCounter { LastCreated } is player-local, so a counter replacement pause parks later players. Example: Oversimplify, two players, Hardened Scales + Branching Evolution. Cites "first action is processed simultaneously" (CR 608.2e). Fix: create tokens for all seats first, keep each seat's created-object refs through pauses, then place counters; add a real-cast, unequal-power, two-player test requiring both Fractals at the first replacement prompt.
  - F2 (MED, structural) preserve unsupported quantity-quality operands through traversal and aggregation — game/coverage.rs:9815-9819, 10003-10016, 9947-9957, 9750-9761, 10024-10026, 10520-10527, 10536. Fix: recurse through ability and trigger conditions with extract_quantity_features; keep Unhandled on repeated keys; controls for direct, compound, trigger cases and both aggregation orders. Latent; no printed Zimone failure claimed.
  - F3 (MED, local) keep absent-reference failure under negation — effects/mod.rs:20609-20616, 21072-21076, 20114-20126. Example Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even)) with no cost-paid subject. Fix: extend the pre-negation guard via quantity_expr_missing_resolution_only_referent; positive, negated, absent, present controls.
  - F4 (MED, structural) pair counter placement only with its nearest LastCreated publisher — phase-ai/src/ability_chain.rs:114-123, 178-183; effects/conjure.rs:275-279; policies/x_reference.rs:334-339. Fix: engine-owned publisher classification; pair only when the nearest publisher is Token; controls for ordinary and intervening-Conjure chains.
  - Non-blocking: runs 38953994816 and 38953994844 queued/in progress with no artifacts; no parse result claimed for 8949699. Artifact 12068771238 (30 cards / 29 signatures) is for parent dfc1a50.
  - Clean: repaired Zimone sequence (sequence.rs:6299-6333) and tests (zimone_all_questioning.rs:327-362, 371-426) intact; the 14-file main merge showed no merge-specific defect.
  - Recommendation: keep open, unapproved, unqueued; fix the four contracts, then fresh review plus matching parse/hosted evidence.
- **NEW — CodeRabbit walkthrough edited:** "No actionable comments were generated in the recent review" (recent review covered 2 files incl. swallow_check.rs and integration/main.rs). Docstring coverage still 69.04% (inconclusive).
- Reviewer statuses on the CodeRabbit dfc1a50 threads (as shown now): zimone_all_questioning.rs 453-455 "Nonblocking limitation"; oracle_trigger.rs 3073-3077 "The state-trigger head is not admitted by the current grammar. Blind recursion is not justified."; sequence.rs 9768-9780 "The malformed-count fallback predates this change. Nonblocking."
- Checks at 8949699 (/checks, ~19:46Z): CI run 38953994844 (29 jobs): only "Card data (generate, validate, coverage)" shows a result, succeeded (3m 18s). Rust lint, Rust tests build + shards, WASM, Tauri, Android, Lobby worker, Card bot, Frontend, Draft pools and Draft match proof show no status yet ("Re-running jobs..."). AI gate run 38953994816 (4 jobs) and Security (Contributor trust, Superagent scan) show no status. No failures visible.

## Maintainer review — matthewevans, "requested changes", at head dfc1a50 (latest)
Counts: critical 0, high 0, medium 4, low 0.
1. Blocker (medium, structural): token creation and counter placement are scheduled per player, so a counter replacement prompt can park later players. Cites crates/engine/src/game/effects/mod.rs:6992-6998, :7470-7485, :16169-16184, :16252-16285. Quotes Oversimplify: "Each player creates a 0/0 green and blue Fractal creature token".
2. Medium (structural): unsupported quantity-quality operands are lost under And/Or/Not and in trigger conditions; same-key aggregation overwrites Unhandled with Handled (crates/engine/src/game/coverage.rs:10024-10026).
3. Medium (local): negated QuantityQuality can return true when the referent is absent (effects/mod.rs:20609-20616, :21072-21076).
4. Medium (structural): crates/phase-ai/src/ability_chain.rs:114-123 pairs placements with the wrong publisher (conjure.rs:275-279).
- Non-blocking: parse artifact 12068771238 (run 38948391635): 30 cards / 29 signatures, zero Oracle changes. The Surveil row's apparent removal is "structural reparenting". AI/performance receipts pending.
- Clean: the earlier Zimone F1 finding is marked resolved, citing sequence.rs:6299-6333 and tests in zimone_all_questioning.rs.
- Recommendation: keep open, unapproved; fix the four contracts before fresh review.

## CodeRabbit review at dfc1a50 (review 5552260329) — 6 actionable
(The second page read showed 4 of these as inline threads marked "Functional Correctness, Minor, Quick win"; the first read labelled the coverage and triggers ones Major.)
1. crates/engine/src/game/coverage.rs ~L9756 (Major): in extract_quantity_features, colliding feature keys let a later Handled verdict overwrite Unhandled (example BattlefieldEntriesThisTurn: FilterProp::FaceDown unhandled vs empty filter handled). Fix: keep Unhandled sticky, or give distinct references distinct keys.
2. crates/engine/src/game/triggers.rs ~L35206 (Major): the bind_triggered_resolution_scope test checks a constructed scope, not the trigger process_triggers actually stacks. Fix: fire the end-step trigger, change the land count, resolve through GameAction, assert no life gain; add a positive case.
3. crates/engine/src/parser/oracle_effect/sequence.rs ~L9768-9780: "Restrict the `VariableX` fallback to a literal `x` prefix." The unwrap_or((rest, CountPrefix::VariableX)) fallback silently defaults unrecognized counts; return None when no arm matches (.ok()?); same in try_parse_token_enters_with_counters.
4. crates/engine/src/parser/oracle_trigger.rs ~L3073-3077: positive_count_gate_qty has no TriggerCondition::EventTime arm; descend through EventTime or document that it deliberately does not bind.
5. crates/engine/tests/integration/zimone_all_questioning.rs ~L453-455: "Route the opponent-land entry through the production pipeline." put_land + record_battlefield_entry stamps the ledger by hand; have P1 play the land via GameAction::PlayLand and add a reach-guard.
6. crates/phase-ai/src/ability_chain.rs ~L114-124: created_token_placement_source searches backward for any Token, so [Token, Conjure, PutCounter { LastCreated }] pairs with the wrong producer; pair only with the nearest LastCreated publisher if it is Effect::Token; add a regression test.

## Commits after the 4b99ff6 review
- 2300adb: AI readers treat a counter placement on a just-created token as part of its creation (created_token_placement_source).
- f2733a3: imperative "create … then put counters" lowers to Token + PutCounter { target: LastCreated } (addresses F1); entry-order test for Primo.
- dfc1a50: merges upstream main; protocol 145 / P2P wire 127.

## github-actions parse-diff sticky comment (for dfc1a50, baseline b2bd5f3a3141)
- Added (21 signatures): many `PutCounter … target=last created` shapes across ~27 cards, plus casting_restriction on Checks and Balances, Even the Odds, Suffocation.
- Removed (3): Even the Odds' CastFromZone self-recast; Suffocation's CastFromZone; Wild Hypothesis' `Surveil (count=2)` (maintainer calls it structural reparenting; the PR body does not list it).
- Modified: Zimone's trigger condition from none to "battlefield entries this turn (land, you) ≥ 1 and # of in battlefield you control land is prime".
- Primo block: added a Token "+0/+0 Green/Blue Primo, the Indivisible" (PR body says "Primo, the Unbounded"); a 3× White Soldier token; removed `unknown` and `unparsed_condition`.
- Truncated by the bot: "4 more signature(s)".

## Older reviews at 4b99ff6
- matthewevans "requested changes" (review 5541830599): Blocker F1 (now marked resolved, see above); non-blocking parse evidence (artifact 12018099078, run 38832866118) matched Zimone, Even the Odds, Suffocation, Checks and Balances.
- CodeRabbit (review 5541882666), 3 actionable: coverage.rs ~L9933-9936 (Major) QuantityQuality extracted only at root, And/Or/Not skipped and trigger-condition quantities skipped; effects/mod.rs ~L20582-20585 (Minor) subject_dependent_type_condition_has_no_subject should recognize a QuantityQuality with a missing referent before Not inverts it; zimone_all_questioning.rs ~L237 (Minor) two bounce cases call move_to_zone directly.
- CodeRabbit walkthrough: docstring coverage inconclusive (69.04% vs 80%).

## Checks (from /checks page)
Only "Superagent Security / Contributor trust" showed a result (succeeded). Every CI job (Rust lint, Rust tests build + shards, Card data, Draft pools, WASM, Tauri, Android, Lobby worker, Card bot, Frontend, Draft match proof, Rust coverage-gate) and the AI gate jobs showed no status; the page said "Re-running checks...". No failures visible.
