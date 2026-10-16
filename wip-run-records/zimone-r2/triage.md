# PR #10004 review triage at dfc1a50 (read-only verification)

Base for "introduced by PR": 33aa23a. Corpus checks: head card data `zimone-f1/head-data-merged/card-data.json`.

## Summary

| Item | Verdict | Introduced by PR? | Seam | Size |
|---|---|---|---|---|
| M1 per-player Token+PutCounter scheduling | PARTIAL. Later seats are deferred, not lost. The CR 608.2e step order really differs, but the only visible effect is prompt order. | No. Base ran Token{enter_with_counters} per seat and paused the same way. The PR adds the explicit local-continuation arm. | Rules-faithful fix: two fan-outs (all seats create, then all seats place) plus a per-seat created-token ledger | Large (6-8 files). **Needs user decision** |
| M2 / C1 coverage QuantityQuality operands and key overwrite | PARTIAL. All three behaviours exist, but no corpus card is mis-reported. | No. Base had them for QuantityCheck, all trigger conditions, and every QuantityRef key. The PR copies the pattern. | `coverage.rs`: sticky-Unhandled insert helper, recursion through And/Or/Not, a trigger-condition walker | 1 src file, but coverage across the corpus may shift. **Needs user decision** |
| M3 Not(QuantityQuality) with absent referent | REAL in code, unreachable from printed cards | The variant is new. QuantityCheck has the same gap from before. | `subject_dependent_type_condition_has_no_subject` (effects/mod.rs) | Small (1 file + test) |
| M4 / C6 created_token_placement_source pairs with the wrong publisher | REAL in code; no corpus card reaches it | Yes (2300adb) | Pair with the nearest LastCreated publisher only if it is `Token`. Reuse the parser's `publishes_chain_created_referent`, promoted to engine-public. | Small (2-3 files) |
| C2 triggers.rs constructed-scope test | PARTIAL. The unit test does bypass production, but the integration test already covers the production path. | Yes (test only) | Rename, or add a process_triggers→resolve case | Tiny |
| C3 VariableX fallback | REAL as code smell; unreachable from printed text | No. Base had the same `(rest, None)`→X fallback in both functions. | `.ok()?` in sequence.rs | Tiny |
| C4 positive_count_gate_qty lacks EventTime arm | Technically REAL; zero corpus cards reach it | Yes (function added in 41cc509) | Explicit `EventTime` arm plus doc | Tiny |
| C5 hand-stamped opponent land entry | PARTIAL. The stamp calls the same function production uses. The suggested `PlayLand` by P1 is illegal on P0's turn (CR 305.1). | Yes (test only) | Optional: route through a real effect (Path to Exile) | Tiny |
| Wild Hypothesis Surveil "removed" | REFUTED as a loss. It is reparenting. | n/a | none | none |
| "Primo, the Indivisible" token | REFUTED as an error. It matches Zimone's Oracle text. | n/a | none | none |

## Per-item evidence

### M1. Per-player Token + PutCounter{LastCreated}

**Card text.** Oversimplify's Oracle reads: "Exile all creatures. Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it equal to the total power of creatures they controlled that were exiled this way." The connective is "and puts", not "then puts".

**Parse shape.**
- Base: `ChangeZoneAll` → `Token{enter_with_counters:[P1P1, Sum Power(ScopedPlayer ∧ ExiledBySource)]}` with `player_scope: All`.
- Head: `Token` → ContinuationStep `PutCounter{count: same Sum, target: LastCreated}`, under the same `player_scope: All`.

**Code at head.**
- effects/mod.rs:6987-6998. The new arm `(Token, PutCounter{LastCreated})` in `is_player_scope_local_continuation`.
- :7470-7485. `next_is_local_continuation` keeps the placement inside the seat's template and clears its `player_scope`.
- The driver loop is at :16169-16285. Each seat resolves the whole template (Token, then PutCounter).
- If `waiting_for` changes, the remaining seats are re-bound and appended to `pending_continuation`, then the loop breaks. A counter replacement prompt triggers this: `counters.rs:266-270` sets `ReplacementChoice` when `NeedsChoice` is returned.
- After the choice, `engine_replacement.rs:535-563` applies the AddCounter and the handler drains `pending_continuation` (:1227/1256/1286/1359 and others).
- Result: the remaining seats run afterwards. Nothing is lost.

**Base behaved the same.** In base, the Token's `enter_with_counters` called `add_counter_with_replacement` (token.rs:1084-1130). On a prompt it parks `FinalizeTokenEntry`/`ContinueTokenCreation`, and the same driver parked the later seats. The PR moves the pause point from Token to PutCounter. The per-seat interleaving itself is unchanged.

**Rules.**
- CR 608.2e: "the choices for the first action are made in APNAP order, and then the first action is processed simultaneously. Then the choices for the second action…" (verified).
- CR 101.4 (APNAP) and CR 616.1 (the affected object's controller orders replacements) also apply.
- Read strictly, all players create their tokens first. Counter-replacement choices come next, in APNAP order. The engine instead runs AP: create → AP's counter choice → NAP: create → NAP's choice.

**Observable difference.** The only one I can construct for Oversimplify is prompt order. For example, NAP's token-creation replacement choice comes after AP's counter-ordering choice. Counter counts are fixed snapshots (ExiledBySource) and do not change. Entry-time triggers see 0/0 tokens either way. Triggers go on the stack after resolution either way. I found no printed board where the totals differ.

**PR claims and tests.** The PR comment cites CR 608.2c + CR 111.1 for keeping the placement inside the iteration. The maintainer's reading (CR 608.2e) is more faithful. No test covers a multi-seat pause. `replacement_order_pause_leaves_a_surviving_0_0_primo` covers only single-controller Zimone.

**Corpus.** Oversimplify is the only player-scoped `Token → PutCounter{LastCreated}`.

**Related pre-existing issue (suspected, not run).** The War Games (ch. I) and Rendmaw, Creaking Nest are player-scoped `Token` chains followed by an unscoped `GenericEffect{LastCreated}` ("The tokens are goaded…"). `Token` assigns `last_created_token_ids` per seat (token.rs:990/1098/1230), and the driver does not accumulate across seats. The tail therefore probably sees only the last seat's tokens. The per-seat created-token ledger in the M1 fix would fix both.

**Fix options.**
- (a) Rules-faithful. Remove the local-continuation arm and fan out PutCounter as its own clause after all Token seats. Each seat's placement reads "the token this seat created". That needs a clause-scoped, per-seat created-token ledger: today `last_created_token_ids` is overwritten per seat, so a second fan-out would read only the last seat's token. Paths: `types/game_state.rs` (ledger), the effects/mod.rs driver plus `publish_player_scope_clause_results`, token.rs publish, `filter.rs` (LastCreated bound to the scoped seat, or a new `TargetFilter` scope parameter, not a sibling), the parser lowering, and tests. Roughly 6-8 files. Run the `add-engine-variant` gate first if a filter variant or parameter is added.
- (b) Keep the current design and answer the maintainer with the evidence above: not lost, pre-existing, no observable difference beyond prompt order. Add a multi-seat pause test: AP holds Hardened Scales + Branching Evolution, cast Oversimplify, answer AP's ChooseReplacement, then assert NAP's Fractal exists with the right counters.

**User decision required.** The maintainer marked this a blocker.

### M2 / C1. coverage.rs QuantityQuality extraction

**Code.**
- `extract_condition_quantity_features` (coverage.rs:10003-10018) reads quantities only at the root `QuantityCheck`/`QuantityQuality`. `condition_feature` emits `And`/`Or`/`Not` as Handled (:10217-10222) and never descends into the leaves.
- Trigger conditions get only `structural:trigger_condition` (:9955-9957). They are never walked, and the comment at :10500-10509 documents this (Tunnel Tipster ledger item).
- `extract_quantity_features` uses `features.insert` (:10021-10027). A later Handled overwrites an earlier Unhandled for the same `quantity_ref:<Name>` key. The only filter-dependent classification is `BattlefieldEntriesThisTurn` (:10520).

**Pre-existing.** Base 33aa23a had a root-only `if let QuantityCheck`, the same trigger-condition skip, and the same `insert`. The PR adds QuantityQuality arms in the same pattern.

**Corpus.**
- Zimone is the only card with QuantityQuality at head, and it sits inside a trigger-condition `And` (`triggers[0].condition.conditions[1]`). Coverage therefore never sees it.
- Its operands are ObjectCount (Handled) and `BattlefieldEntriesThisTurn{Land}` (an evaluable filter, so Handled). No mis-report today.
- No card has a QuantityQuality in an ability or static condition, and none has a colliding key with a different verdict caused by this PR.

**Overlap.** C1 is the same as the older CodeRabbit finding (coverage.rs ~L9933). M2 covers both.

**Fix seam (coverage.rs only).**
- A `record_feature(features, key, support)` helper that never downgrades Unhandled to Handled, used at every insert site.
- Recurse `extract_condition_quantity_features` through And/Or/Not, and emit leaf `condition_feature` for the operands.
- A `extract_trigger_condition_features` that mirrors `extract_static_condition_features`, including Not/EventTime.

**Risk.** All three changes are coverage-wide and can flip currently-"supported" cards. Tunnel Tipster is the documented candidate. Measuring that needs a full `cargo coverage` diff.

**User decision.** Do this in this PR or as a follow-up.

### M3. Not(QuantityQuality) with an absent referent

**Code.**
- `evaluate_condition` QuantityQuality (effects/mod.rs:20609-20623) returns false when `quantity_expr_missing_resolution_only_referent` is true.
- The Not arm (:21072-21077) skips negation only when `subject_dependent_type_condition_has_no_subject` (:20114-20127) is true, and that function handles only `RevealedHasCardType`.
- So `Not(QuantityQuality{ObjectManaValue{CostPaidObject}})` with no referent evaluates `!false = true`.

**Reachability.** The only parser producer is `parse_control_count_quality` (oracle_nom/condition.rs:5164-5188), and it always yields `ObjectCount`. ObjectCount never has a resolution-only referent (quantity.rs:2260-2278). No printed card reaches this.

**Pre-existing analogue.** `Not(QuantityCheck{absent})` behaves the same way and also predates this PR.

**Overlap.** Same as the older CodeRabbit effects/mod.rs ~L20582 finding.

**Fix.** Add an arm to `subject_dependent_type_condition_has_no_subject` for `QuantityQuality` (and arguably `QuantityCheck`, which would be a behaviour change outside the PR's scope) that calls `quantity_expr_missing_resolution_only_referent`. This follows the issue #2871 precedent (CR 608.2c). Add a unit test next to `quantity_quality_ability_condition_evaluates_at_resolution`.

**Annotation note.** The guard's comment cites CR 701.57c (Discover). That text was copied from QuantityCheck's discover rationale and is a weak fit for a generic quality gate; CR 608.2c is the better citation.

### M4 / C6. AI created_token_placement_source

**Code.** phase-ai/ability_chain.rs:114-123. For `PutCounter{LastCreated}` it returns `rposition(Token)`, which can skip over intervening LastCreated publishers.

**Publishers in the engine (runtime).** These effects assign `last_created_token_ids`:
- Token (token.rs)
- CopyTokenOf, with copy_token_blocking (token_copy.rs)
- Populate
- Conjure (conjure.rs:279)
- Manifest, ManifestDread and Cloak, via `morph::publish_face_down_entry_referent` (morph.rs:867)

**Existing predicate.** The parser already has one: `publishes_chain_created_referent` (parser/oracle_effect/lower.rs:3917), built on `is_token_creating_effect`. It is `pub(crate)`.

**Corpus scan.** I checked every unconditional chain in head data with a `PutCounter{LastCreated}` that has a preceding Token. The effects between them are always empty, except Grist (a `Mill`, which is not a publisher). No printed card has `[Token, X-publisher, PutCounter{LastCreated}]`.

**Status.** Latent, introduced by 2300adb.

**Fix.**
- Promote the publisher predicate to an engine-public authority, for example `Effect::publishes_created_referent()` in types/ability.rs. Have lower.rs delegate to it so there is still one authority.
- In ability_chain.rs, take `rposition(publishes)` and return `Some(j)` only if `chain[j]` is `Token`.
- Add a regression test `[Token, Conjure, PutCounter{LastCreated}] → None`.
- 2-3 files.

M4 and C6 are the same item.

### C2. triggers.rs:35201 `quantity_quality_rechecked_at_resolution`

**What it does.** It calls `stack::bind_triggered_resolution_scope` (stack.rs:1540) with a hand-built `TriggeredResolutionScope`. That is a constructed-scope test.

**Production coverage already exists.** `zimone_all_questioning.rs:224-244 resolution_recheck_and_live_count`(b):
1. A real PlayLand.
2. The real end-step trigger fires with 3 lands.
3. A 4th land is added.
4. The stack drains, and the test asserts no Primo.

That case exercises process_triggers → stack → resolution recheck.

**Fix.** Rename or doc the unit test as a bind-function test. Optionally add a process_triggers + resolve case to the same unit test.

### C3. sequence.rs:9767-9780 `.unwrap_or((rest, CountPrefix::VariableX))`

**What reaches it.** The fallback fires only when none of `x `/`that many `/`a `/`an `/number matches. The very next step requires `+1/+1 ` or `-1/-1 `. The only input that gets through is therefore a count-less "put +1/+1 counters on it", which does not occur in printed text. "another" and "an additional" fail at the counter-type step and return None.

**Pre-existing.** Base 33aa23a had `(rest, None)`, which falls back to `Variable("X")`. `try_parse_token_enters_with_counters` (:9617-9632) is unchanged from base and has the same fallback.

**Oversimplify at head.** Its count is `PropertyAggregate Sum`, not VariableX. Form 1 handles it.

**Fix.** Use `.ok()?` in the followup. Optionally do the same in enters-with: the parse_number else-branch becomes `?`. Confirm with a parse diff showing zero card changes.

### C4. oracle_trigger.rs:3067-3086 positive_count_gate_qty

**Code.** `EventTime` falls to `_ => None`. EventTime wraps attack-while gates, state-trigger gates and count-qualifier event gates (types/ability.rs:31186-31203).

**Corpus.** No card has an EventTime that contains HadCounters or QuantityQuality.

**Fix.** Add an explicit `TriggerCondition::EventTime { condition } => positive_count_gate_qty(condition)`, or an explicit `None` with a doc line. Either is tiny. Descending is consistent with how `condition_contains_source_counter` (:3050) treats EventTime.

### C5. zimone_all_questioning.rs:453-455

**What the test does.** `put_land` followed by `record_battlefield_entry`.
- `record_battlefield_entry` is the public, pre-existing function that production's `record_zone_change` calls (restrictions.rs:428, :661), so the ledger record is built the same way.
- The suggested `GameAction::PlayLand` by P1 cannot work on P0's turn: CR 305.1 allows a land play only "during a main phase of their turn".

**Optional production route.** P0 casts Path to Exile on a P1 creature. Its Oracle text: "Its controller may search their library for a basic land card, put that card onto the battlefield tapped". Then reach-guard the ledger entry. Low value.

### Parse-diff notes

**Wild Hypothesis.** Base: `Token → Surveil(2, SequentialSibling)`. Head: `Token → PutCounter{X, LastCreated} → Surveil(2, SequentialSibling)`. Surveil is still present, independently linked, and always resolves. This is reparenting, and the PR body should mention it.

**"Primo, the Indivisible".** This is Zimone's token. Zimone's Oracle reads "create Primo, the Indivisible, a legendary 0/0 green and blue Fractal…", so the head name is correct.
- "Primo, the Unbounded" is a different, real card in the class table (created_token_counter_sequence.rs). Its trigger lowers to `Token → PutCounter{EventContextAmount, LastCreated}`.
- If the PR body calls Zimone's token "Unbounded", fix the body. I could not see the PR body.
- The "3× White Soldier" row is Even the Odds' token, now split out next to its `casting_restriction` gap.

## Proposed implementation units

**U1. Local correctness fixes (PR-scoped, no decision needed).** M4/C6, M3, C3, C4.
- Paths: phase-ai/ability_chain.rs, types/ability.rs (or another engine-public home for the publisher predicate) plus the delegating edit in parser/oracle_effect/lower.rs, effects/mod.rs, parser/oracle_effect/sequence.rs, parser/oracle_trigger.rs.
- About 6 counted paths. Tests go inline in those files.
- Gate: parser combinator check plus a card-data parse diff, which should show zero changes.

**U2. Test hardening (PR-scoped).**
- C2: rename the unit test, or add a production case, in triggers.rs.
- C5: optional Path to Exile route in zimone_all_questioning.rs.
- M1(b): the multi-seat Oversimplify pause test in oversimplify_per_player_fractal.rs.
- About 3 paths.

**U3. Coverage aggregation (M2/C1). Pre-existing; needs user decision.**
- coverage.rs only (1 path, plus tests).
- Needs a full coverage diff, because supported counts may drop (Tunnel Tipster). Recommend a separate follow-up PR, with a reply to the maintainer that the PR copies existing behaviour and no corpus card is mis-reported.

**U4. M1 rules-faithful restructure. Needs user decision.**
- Two-action fan-out plus a per-seat created-token ledger. About 6-8 paths across types, game_state, effects/mod.rs, token.rs, filter.rs, the parser lowering and tests.
- It would also fix the suspected War Games / Rendmaw tail bug.
- Alternative: U2's test plus a written rebuttal. Since the maintainer called this a blocker, the user must choose between the rebuttal and the restructure, and whether the restructure belongs in this PR.
