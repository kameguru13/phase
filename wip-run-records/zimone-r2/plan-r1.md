# Plan r1: PR #10004 (card/zimone-all-questioning) review blockers F1–F4

BASE_SHA 8949699a13a541a7e00ada2bb8e16c00c530b82a. Line numbers were read at dfc1a50. dfc1a50..8949699 touches none of the files below, so the numbers still hold; re-anchor by symbol name anyway.

Scope: the four maintainer blockers M1/F1, M2/F2, M3/F3 and M4/F4 are all in this PR, as the user confirmed. C2–C5 are non-blocking; how each is handled is under U-E. The Wild Hypothesis surveil question is answered under U-E as well.

All CR numbers below were grepped in `docs/MagicCompRules.txt`: 101.4, 111.2, 115.10a, 122.1, 305.1, 400.7, 603.4, 603.8, 608.2c, 608.2e, 608.2f, 609.3, 616.1, 700.11, 701.15b, 701.57c.

Card texts were verified from `data/mtgjson/AtomicCards.json`:
- **Oversimplify:** "Exile all creatures. Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it equal to the total power of creatures they controlled that were exiled this way."
- **The War Games, ch. I:** "Each player creates three tapped 1/1 white Warrior creature tokens. The tokens are goaded for as long as this Saga remains on the battlefield."
- **Rendmaw:** "... each player creates a tapped 2/2 black Bird creature token with flying. The tokens are goaded for the rest of the game."
- **Wild Hypothesis:** "Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it. Surveil 2."
- **Tunnel Tipster:** "... if a face-down creature entered the battlefield under your control this turn, put a +1/+1 counter on this creature."
- **The Roaring Toeclaws:** "... put +1/+1 counters on it equal to its power." This bears on C3.
- **Also verified:** Hardened Scales, Branching Evolution, Path to Exile, Zimone.

---

## Pattern Coverage

| Unit | Class (not card) | Corpus members (measured on head card-data) |
|---|---|---|
| U-A (M4) | An AI pairing of a `PutCounter{LastCreated}` placement with its created-token source. The pairing is valid only when the **nearest** publisher of the chain-created referent is a token creator. | Every chain where a `Token` comes before `PutCounter{LastCreated}`: the Fractal family (Wild Hypothesis, Oversimplify, Body of Research, Fractal Harness…) plus every "create … put counters on it" card. The Conjure/Manifest/Cloak publishers interleave rarely, but the shape is legal. |
| U-B (M1) | A player-scoped multi-action instruction (CR 608.2e) whose first action creates tokens for each player and whose later action refers to "it"/"the tokens". Two subclasses: (a) a **co-scoped** second action ("each player … and puts counters on it"), done per player and bound to that player's own token; (b) an **unscoped** second action ("The tokens are goaded"), done once on the union of every player's tokens. | (a) Oversimplify. A corpus scan found no other player-scoped node that reads LastCreated. (b) The War Games ch. I, Rendmaw, and any future "each player creates … the tokens …". The fix is the generic driver and ledger rule, not a card match. |
| U-C (M3) | A negated subject-dependent condition whose subject is a resolution-only referent that is absent: `Not(X)` where X reads a referent the resolution never bound. | `QuantityQuality` and `QuantityCheck` over any `quantity_expr_missing_resolution_only_referent` ref (`CostPaidObject`, etc.), plus the existing `RevealedHasCardType` arm. The corpus has no printed member today; the only `Not(QuantityCheck|QuantityQuality)` card is Ravenous Necrotitan, with no resolution-only scope. This is a fix for the building block. |
| U-D (M2) | Coverage honesty for quantities nested in conditions: every QuantityRef reachable from an ability or trigger condition is classified, and an Unhandled verdict cannot be overwritten by a later Handled insert of the same key. | Every trigger-condition card with a QuantityComparison/QuantityQuality operand. A walk prediction reaches 75 cards: werewolves (SpellsCastLastTurn), descend cards (DescendedThisTurn) and Tunnel Tipster (BattlefieldEntriesThisTurn with a non-evaluable FaceDown filter). |
| U-E | The minor follow-ups listed under U-E. | n/a |

## Sizing

**Units: 5.**
- U-A is M4: the publisher authority and the AI pairing.
- U-B is M1: the CR 608.2e two-action player scope and the created-token union.
- U-C is M3: the negated-condition subject guard.
- U-D is M2: coverage recursion plus sticky Unhandled.
- U-E is minor: the C2 rename, the C4 explicit arm, and a Wild Hypothesis runtime test.

**Dependency edges:**
- U-A → U-B. U-B uses the engine-public `Effect::creates_tokens()` that U-A introduces.
- U-C, U-D and U-E are independent of each other and of U-A/U-B.
- U-D's reclassification needs the runtime evidence tests that are in U-D itself.

**Registration surfaces:**
- `impl Effect` predicates in types/ability.rs (U-A).
- The serialized `PendingContinuation` sidecar field and its type in types/game_state.rs (U-B). This is a serialized shape change; see the protocol note.
- The coverage feature map (U-D).
- No new `Effect`, `WaitingFor` or `GameAction` variant.
- No new enum variant anywhere except that the sidecar struct gains fields.

**Expected changed paths, before exclusion:**

| # | Path | Unit | Counted? |
|---|---|---|---|
| 1 | crates/engine/src/types/ability.rs | U-A | yes |
| 2 | crates/engine/src/parser/oracle_effect/lower.rs | U-A | yes |
| 3 | crates/engine/src/parser/oracle_effect/mod.rs | U-A | yes. Call-site switch at :27 import, :26816, :27048. |
| 4 | crates/engine/src/game/effects/mod.rs | U-A, U-B, U-C | yes |
| 5 | crates/phase-ai/src/ability_chain.rs | U-A | yes. Its inline tests are in the same file. |
| 6 | crates/engine/src/parser/oracle_effect/sequence.rs | U-B | yes |
| 7 | crates/engine/src/types/game_state.rs | U-B | yes. Serialized type change. |
| 8 | crates/engine/src/game/visibility.rs | U-B | yes. Comment-only standing class: the doc names `resolving_player_scope_linked_exile`. |
| 9 | crates/engine/src/game/coverage.rs | U-D | yes |
| 10 | crates/engine/src/game/triggers.rs | U-D (runtime evidence tests), U-E (C2 rename) | yes |
| 11 | crates/engine/src/parser/oracle_trigger.rs | U-E (C4 arm) | yes |
| 12 | crates/engine/src/game/quantity.rs | U-D | conditional. Only if the executor adds the CR 700.11 annotation at the DescendedThisTurn resolver (6620) and it is missing there. Expected: no change. |
| 13 | crates/engine/tests/integration/oversimplify_per_player_fractal.rs | U-B | integration test |
| 14 | crates/engine/tests/integration/goaded_creature_under_pacifism_visible.rs | U-B | integration test |
| 15 | crates/engine/tests/integration/created_token_counter_sequence.rs | U-E | integration test |
| 16 | crates/phase-ai/src/policies/x_reference.rs | U-A | conditional. Only if a walk-level test is placed there and not in ability_chain.rs. Plan: place it in ability_chain.rs, so no change. |

**Excluded:**
- Regenerated `data/card-data.json` and coverage output. These are uncommitted pipeline data.
- The protocol version. The PR already carries protocol 145. If the executor bumps it again for the sidecar shape, `crates/lobby-broker/src/protocol.rs` is a protocol-version surface.
- Whether the integration-test files count as "test fixtures" is for the adjudicator. Counts are given both ways.

**Counted scope-path totals:**
- 11 non-test source paths (rows 1–11), with integration tests excluded.
- 14 with the three integration-test files counted.
- Conditional rows 12 and 16 are expected unchanged.
- There are no generated-artifact groupings and no translation mirrors.

**Discriminating tests per unit** (detail in the Verification Matrix):
- U-A: 6 unit rows.
- U-B: 7 integration rows, plus 2 updated rows and 1 rewritten splitter unit test.
- U-C: 5 unit rows.
- U-D: 9 unit rows and 2 runtime evidence rows.
- U-E: 1 integration row and 1 renamed test.

**Suggested partition** (for the adjudicator; this is not a verdict):
- Phase 1 = U-A + U-B. Paths 1–8 and 13–14, which are 8 counted source paths.
- Phase 2 = U-C + U-D + U-E. Paths 4, 9–11 and 15.

F2 (U-D) can split into its own phase without touching phase 1. U-C shares effects/mod.rs with phase 1, so put it in phase 2, after phase 1 lands.

## Building Blocks (reuse, do not duplicate)

- **Player-scope driver** (`resolve_ability_chain` player_scope branch, effects/mod.rs ~15930–16330), with:
  - `split_player_scope_chain` (~7494)
  - `detach_after_player_scope_local_chain` (~7380)
  - `is_player_scope_local_continuation` (~6942)
  - `bind_scoped_seat` (19589)
  - `apnap_order_from` (players.rs:281)
  - `matching_players`
- **Paused fan-out sidecar:**
  - `PendingPlayerScopeLinkedExile` (game_state.rs 2819) on `PendingContinuation.player_scope_linked_exile`
  - `state.resolving_player_scope_linked_exile`
  - `park_player_scope_queue_end`
  - The drain at effects/mod.rs ~1250–1365, including `linked_exile_batch_from_events` + `extend_linked_exile_batch`
  - The resolution-time batch extension at ~17668
  - The park gate at ~18075
- **Clause publication authority:** `publish_player_scope_clause_results` (~13569), described as "one action ⇒ one terminal result" (CR 608.2f).
- **LastCreated → ParentTarget snapshot precedent:** `rebind_last_created_to_parent_target` (delayed_trigger.rs:1022, logic 517–602), plus `Effect::target_filter()` (ability.rs:23304) and `target_filter_mut()` (23954).
- **Counter placement through ParentTarget object targets:** `resolve_defined_or_targets` → `live_object_targets` (counters.rs ~2440–2500).
- **Token creation event:** `GameEvent::TokenCreated { object_id, name, source_id }` (events.rs:1381). Its single emitter is `push_committed_token_entry_events` (token.rs).
- **Ledger:** `last_created_token_ids: Vec<ObjectId>` (game_state.rs 23102). The single-id publisher is `record_last_created_token` (token.rs 2405).
- **Incarnation pins:** `ObjectIncarnationRef` (identifiers.rs:171).
- **Publisher predicates:** `is_token_creating_effect` (lower.rs 3883) and `publishes_chain_created_referent` (lower.rs 3917). U-A moves both to `impl Effect`.
- **Absent-referent detection:** `quantity_expr_missing_resolution_only_referent` (quantity.rs 2243) and `condition_awaits_resolution_only_referent` (effects/mod.rs ~14894).
- **Coverage:** `FeatureSupport`, `quantity_ref_feature` (coverage.rs 10242), `extract_quantity_features` (10021), `extract_static_condition_features` (9737; its And/Or/Not recursion is the shape to copy), `card_face_gaps`, `check_resolver_features` (~8237).
- **Condition-walker recursion shape** (reference): types/ability_visit.rs ~1315–1360.

## Logic Placement

- All rules logic stays in `engine`. The AI change (U-A) consumes an engine-public predicate, `Effect::publishes_created_referent()`, and no longer re-derives a publisher list in phase-ai. That is the maintainer's "engine-owned publisher classification" point (ability_chain.rs:178–183, x_reference.rs:334–339).
- The per-seat binding and the union publication live in the player-scope driver and sidecar, the single authority for fan-out. The token resolver and counter resolver stay unchanged.
- The parser change in U-B is limited to propagating `player_scope` onto the placement node. "Each player … puts" is the same grammatical subject.
- Frontend: no change.

## Rust Idioms

- Exhaustive `match` on `Effect` inside `publishes_created_referent()` / `creates_tokens()` would be very large. Use `matches!` with an explicit positive list, which mirrors the current lower.rs predicates. A doc comment names the runtime-writer criterion: the effect assigns `last_created_token_ids`. This keeps the predicates as the single authority.
- New sidecar fields are `Option<Vec<ObjectIncarnationRef>>`, not bools. `None` means "this sidecar does not carry that channel".
- Coverage sticky merge goes in one `fn record_feature(map: &mut …, key, support)` that uses `entry().and_modify(|s| if support == Unhandled { *s = Unhandled }).or_insert(support)`. All 7 `features.insert(` sites route through it.
- Trigger-condition and ability-condition quantity walkers are recursive `fn`s with exhaustive matches over `AbilityCondition` / `TriggerCondition`.
  - Arms that carry no quantity go in a single grouped arm that lists each variant, with no `_ =>` wildcard.
  - If the variant count makes that impractical, use a wildcard with a comment. The existing walkers' convention decides; the executor matches it.

## Nom Compliance

- No new parser text-matching is introduced. U-B's parser change is structural: it copies `previous.player_scope` in `sequence.rs` at the `ContinuationAst::PutCountersOnLastCreated` lowering (~6299–6330). It adds no combinators and no string dispatch.
- U-E's C4 arm is a match-arm addition in `positive_count_gate_qty` (oracle_trigger.rs 3067) and does no text parsing.
- C3 (the parser fallback) is dropped, so no parser text work is left in this plan.

## Extension vs Creation

| Need | Extend | Create (justified) |
|---|---|---|
| Union of a paused fan-out's created tokens | Generalize the existing linked-exile sidecar (`PendingPlayerScopeLinkedExile` → `PendingPlayerScopeTail`). It already defers `after_scope` until every seat leg drains and already accumulates a batch from events. | The `created_tokens` field only. |
| Per-seat "it" binding | Extend the driver's existing per-seat bind (`bind_scoped_seat`) and the `rebind_last_created_to_parent_target` snapshot idiom. | A small helper, `bind_seat_created_referent(template, seat, union) -> Option<ResolvedAbility>`. |
| Publisher classification | Move the existing lower.rs predicates into `impl Effect`. | None. |
| Coverage recursion | Copy `extract_static_condition_features`' recursion. | `extract_trigger_condition_quantity_features`, plus `record_feature`. |
| Negated-condition guard | Extend `subject_dependent_type_condition_has_no_subject` by renaming it and adding arms. | None. |

## Analogous Trace

1. **Linked-exile player-scope sidecar (Path to Exile–style "each player … exiled this way" tails):**
   - Flow: `after_scope_needs_linked_exile` → pause → `park_player_scope_queue_end` → `frame.pending.player_scope_linked_exile = Some(..)` (~16315) → the drain (~1250–1365) extends the batch from events and swaps `resolving_player_scope_linked_exile` → when legs are drained and nothing waits, `bind_resolution_exile_batch_paths(after_scope, batch)` and resolve it.
   - The non-paused path does the same inline after `publish_player_scope_clause_results`.
   - U-B rides this machinery exactly. It adds a second channel, `created_tokens`, alongside `linked_exile`.
2. **Windfall co-scoped tail:** a detached tail that keeps `player_scope` re-enters the driver as its own all-seats loop, after the first clause finished for every seat. That is the CR 608.2e "action, then next action" order.
   - Oversimplify's placement becomes exactly this once it carries `player_scope` and is no longer a local continuation.
3. **Delayed-trigger LastCreated snapshot** (delayed_trigger.rs 517–602 / 1022): snapshot `last_created_token_ids` into `TargetRef::Object` targets plus incarnation pins, and rewrite `TargetFilter::LastCreated` → `ParentTarget`. U-B's per-seat bind does the same, filtered to the seat's own tokens.
4. **War Games test today** (`war_games_registered_chapter_token_tracks_saga_presence`): it asserts "at least one" goaded recipient. That passes with the last-seat-only defect, so it is not discriminating; U-B tightens it.

## Variant Discoverability

- `data/engine-inventory.json` is gitignored. The executor runs `cargo engine-inventory` before editing, then:
  - greps `"creates_tokens"`, `"publishes_created_referent"` and `"PendingPlayerScope"` to confirm no existing equivalents;
  - checks `QuantityRef` for SpellsCastLastTurn and DescendedThisTurn and any sibling-cluster smell.
- A repo grep found no existing `fn creates_tokens` or `fn publishes_created_referent` in types/ability.rs.
- No new enum variant is proposed. The sidecar rename is a struct, not an enum sibling.

## Identity/Provenance Contract

- **Created-token union:** tokens attributed by `TokenCreated.source_id == clause template source_id`. This includes tokens added by replacement effects (Parallel Lives / Anointed Procession), which share the creating effect's source_id; the U-B phase-1 pause test reach-guards this.
  - Each entry is stored as an `ObjectIncarnationRef` pinned when the event is seen (CR 400.7).
  - At publication only entries whose object is still live at the same incarnation are written to `last_created_token_ids`.
- **Per-seat subset:** union entries whose object's `owner == seat` (CR 111.2: the creator is the owner). Under `bind_scoped_seat`, the Token's `owner: Controller` resolves to the seat. Recognize only Token templates whose `owner` is `Controller` or `ScopedPlayer`; any other owner filter leaves the clause on the existing path (no expansion), documented.
- **Bound targets:** each seat's PutCounter gets `TargetRef::Object(id)` targets with incarnation pins and `target: ParentTarget`. `live_object_targets` then drops anything that changed zones before placement (CR 400.7 / CR 609.3).
- **CR 115.10a:** these are affected objects, not targets. The binding is a referent snapshot and adds no targeting legality checks. Note this in a comment, as the delayed-trigger precedent does.

## Step-by-step plan

### U-A: engine-owned created-referent publisher classification (M4/F4)

1. **types/ability.rs**: add to `impl Effect`.
   - `pub fn creates_tokens(&self) -> bool` matches `Token | CopyTokenOf | Populate`. This is today's `is_token_creating_effect` set.
   - `pub fn publishes_created_referent(&self) -> bool` = `creates_tokens()` ∪ `Manifest | ManifestDread | Cloak | Conjure | CopyTokenBlockingAttacker | EachPlayerCopyChosen`.
   - The doc on `publishes_created_referent` says the criterion: the resolver assigns `last_created_token_ids`, the referent `TargetFilter::LastCreated` reads.
   - The two new members (`CopyTokenBlockingAttacker` in copy_token_blocking.rs, `EachPlayerCopyChosen`) are runtime ledger writers the parser list missed. The executor confirms each writes the ledger by grepping its resolver for `last_created_token_ids` before adding it. Drop any that does not.
   - Annotate with CR 111.1 (token creation) only if the grep confirms the wording. No CR is required for a classification predicate.
2. **parser/oracle_effect/lower.rs**:
   - `is_token_creating_effect` and `publishes_chain_created_referent` become thin wrappers that delegate to the methods. Or delete them and switch callers; prefer deleting.
   - Callers: lower.rs, `oracle_effect/mod.rs` (:27 import, :26816, :27048), `game/effects/mod.rs:11` import and ~5094. Update the doc mentions in `types/zones.rs:111` and `morph.rs:831` only if they name the function. Those are comment-only and **outside SCOPE**: if they name it, the executor stops and returns them as comment-only standing-class paths. Prefer keeping a `pub(crate)` wrapper with the old name so the docs stay true; then they need no edit.
   - **Predicted parse diff: zero cards.** Unprobed. Widening the set matters only where a parser call site sees `CopyTokenBlockingAttacker` or `EachPlayerCopyChosen` followed by a LastCreated anaphor:
     - Mirror Match's sub is already `CreateDelayedTrigger{ChangeZone LastCreated}`.
     - Caught in a Parallel Universe and Human — Time Lord Meta-Crisis have no sub.
     - Verify with the parse-diff command below.
3. **phase-ai/src/ability_chain.rs** `created_token_placement_source` (114):
   ```rust
   Effect::PutCounter { target: TargetFilter::LastCreated, .. } => {
       let j = chain[..index].iter().rposition(|e| e.publishes_created_referent())?;
       matches!(chain[j], Effect::Token { .. }).then_some(j)
   }
   ```
   - Doc comment: the placement's referent is the nearest publisher (it overwrites the ledger). It pairs as a created-**token** placement only when that publisher is a `Token`.
   - Keep `Token`-only, not `creates_tokens()`. `CopyTokenOf` and `Populate` copies carry their own P/T and the existing AI semantics are Token-specific; say so in the doc. The executor checks `plus_one_counters.rs:236` and `x_reference.rs:339` still read correctly and adds no logic there.
4. Run `cargo ai-gate` if the AI baselines change. A paired-seed report is required to refresh them. Expected: no score change on corpus decks, because no corpus chain interleaves a publisher between Token and placement.

### U-B: CR 608.2e two-action player scope and created-token union (M1/F1, War Games/Rendmaw)

Card-derived expected behavior:
- **Oversimplify:** CR 608.2e. First, each player creates a Fractal: all seats, APNAP order (CR 101.4). Then each player puts counters on their own Fractal: all seats, APNAP order. Replacement-order choices are made by the affected object's controller (CR 616.1), in APNAP order.
- **The War Games / Rendmaw:** "The tokens" is every token created by the "each player creates" action. That is one action with one result (CR 608.2f), so all of them are goaded (CR 701.15b).

5. **parser/oracle_effect/sequence.rs** (~6299–6330, `ContinuationAst::PutCountersOnLastCreated` lowering): set `placement.player_scope = previous.player_scope.clone()`. CR 608.2e: the sentence's subject "each player" governs both verbs.
   - **Unprobed:** whether `previous` (the Token def) already carries `player_scope` when the continuation applies. Continuations apply via assembly.rs `apply_clause_continuation` at 2141/2733/2882/3911.
   - The executor first adds the parse assertion: Oversimplify's placement `player_scope == Some(All)`, in oversimplify_per_player_fractal.rs.
   - If `previous.player_scope` is `None` at that moment, find where the subject scope is stamped and stamp the placement there instead. That site must be in sequence.rs or assembly.rs. If assembly.rs, it is out of SCOPE: stop and return it.
   - Single-player chains ("Create … Put X counters on it", like Wild Hypothesis) have `player_scope None` and are unchanged.
   - **Predicted parse diff:** exactly 1 card (Oversimplify), 1 signature change: the placement gains `player_scope: All`.
6. **game/effects/mod.rs** `is_player_scope_local_continuation` (~6942): delete the PR-added `(Effect::Token { .. }, Effect::PutCounter { target: TargetFilter::LastCreated, .. })` arm and its comment.
   - The co-scoped placement now detaches as a tail that keeps its scope, via `detach_after_player_scope_local_chain` (Windfall path).
   - Confirm in the splitter unit test that `effect_introduces_per_iteration_referent` (Token is not an introducer) does not pull it back in.
7. **types/game_state.rs:** generalize the sidecar.
   - Rename `PendingPlayerScopeLinkedExile` → `PendingPlayerScopeTail`:
     ```rust
     pub struct PendingPlayerScopeTail {
         pub source_id: ObjectId,
         pub after_scope: Box<ResolvedAbility>,
         /// CR 608.2c: objects the fan-out exiled, for an "exiled this way" tail.
         #[serde(default, skip_serializing_if = "Option::is_none")]
         pub linked_exile: Option<Vec<ObjectIncarnationRef>>,
         /// CR 608.2f + CR 111.2: tokens every seat's leg created, for a tail that reads "the tokens"/"it".
         #[serde(default, skip_serializing_if = "Option::is_none")]
         pub created_tokens: Option<Vec<ObjectIncarnationRef>>,
     }
     ```
   - Rename the fields `PendingContinuation.player_scope_linked_exile` → `player_scope_tail` and `GameState.resolving_player_scope_linked_exile` → `resolving_player_scope_tail`. Add `#[serde(alias = "player_scope_linked_exile")]` / `alias = "resolving_player_scope_linked_exile"`.
   - The old `batch: Vec<_>` maps to `linked_exile: Some(batch)`. This is not alias-compatible because the shape differs. **Protocol policy:** the executor checks whether `PendingContinuation` is persisted across versions (saved games, server resume). If it is, either keep a custom `Deserialize` shim or bump the protocol per the repo policy. The PR already bumped to 145, so one bump per PR may cover it; the executor confirms with the project-reference skill.
   - **Alternative if the rename churn is judged too large:** keep the type and field names, make `batch` → `linked_exile: Option<Vec<…>>`, and add `created_tokens`, with the doc updated to "player-scope tail sidecar". The executor picks the rename unless a persistence constraint blocks it, and states which.
   - Update visibility.rs:3864 (comment-only).
8. **effects/mod.rs, sidecar consumers.** Every site that today assumes "sidecar present ⇒ linked exile" now gates on `linked_exile.is_some()`:
   - (a) the drain ~1250–1365, which extends the batch via `linked_exile_batch_from_events` and runs `bind_resolution_exile_batch_paths` at completion;
   - (b) the resolution-time extension ~17668;
   - (c) the park gate ~18075 (`park_player_scope_queue_end` when the resolving sidecar is present). That stays keyed on presence, because both channels need the tail deferred to queue end;
   - (d) creation at ~16315.

   The `created_tokens` channel is extended, with incarnation dedup, from `TokenCreated { source_id == sidecar.source_id }` events at the same two points as the linked batch: the drain before the frame is taken, and the resolution-time site ~17668. That way tokens from legs that resolved before a nested re-pause survive (`PendingContinuation::new` clones the resolving sidecar into nested frames).
9. **effects/mod.rs, driver** (~15930–16330):
   - Compute `after_scope_needs_created_union = after_scope.is_some() && scoped_template.effect.creates_tokens()`.
   - On pause, route to the sidecar when `after_scope_needs_linked_exile || after_scope_needs_created_union`. Today's `tail = if after_scope_needs_linked_exile { None } else { after_scope.clone() }` becomes keyed on the disjunction.
   - Seed `created_tokens` from the clause's pre-pause events.
   - **Sync completion:** `publish_player_scope_clause_results` gains a created-token step. When `scoped_template.effect.creates_tokens()`, set `state.last_created_token_ids` to the deduped, live-filtered ids of `TokenCreated{source_id == scoped_template.source_id}` in `scoped_events`, in event order.
     - Annotate: "CR 608.2f: 'each player creates' is one action with one result; the clause's created-token referent is every token it created, not the last seat's overwrite."
     - This fixes War Games and Rendmaw: the unscoped tail reads LastCreated = the union.
   - **Pause completion** (the drain, when legs are drained and nothing waits): when `created_tokens.is_some()`, write the live-filtered union to `last_created_token_ids` before resolving `after_scope`. Also bind the linked batch when `linked_exile.is_some()`.
   - **Third completion point:** the resumed `drain_pending_discard_batch` publishes through `publish_player_scope_clause_results`. The executor confirms a Token template cannot reach it (discard-batch only). If it can, it gets the same created-token step through the shared function automatically.
10. **effects/mod.rs, per-seat binding of a co-scoped created-referent consumer.**
    - This applies when the driver begins a player-scope loop whose template effect has `target_filter() == Some(TargetFilter::LastCreated)`. That is the re-entry of the co-scoped Oversimplify tail, at both completion points, because each one calls `resolve_ability_chain(after_scope)`.
    - Snapshot the current ledger (the union just published) once at loop start.
    - For each seat in the driver's APNAP order, after `bind_scoped_seat`, call `bind_seat_created_referent`:
      - filter the snapshot to objects with `owner == seat` (CR 111.2);
      - if the subset is empty, skip the seat (CR 609.3);
      - otherwise set `targets` to `TargetRef::Object` with incarnation pins, and rewrite `target_filter_mut()` LastCreated → `ParentTarget`. Cite the delayed_trigger precedent in the comment.
    - Pauses in this second action (Hardened Scales / Branching Evolution order prompts, CR 616.1) ride the driver's existing leg continuation. The snapshot is already baked into each leg's targets, so the ledger being overwritten mid-resume is harmless.
    - Scope guard: only `LastCreated`-reading templates under `player_scope`. A corpus scan found no other player-scoped node that reads LastCreated, so the change in corpus behavior is Oversimplify only.
11. **Tests:** see the Verification Matrix.
    - Rewrite the PR-added unit test `split_player_scope_keeps_created_token_counter_placement_in_iteration` (effects/mod.rs) as `split_player_scope_detaches_co_scoped_created_token_placement`. It asserts that the placement is in the detached co-scoped tail with `player_scope` intact, and that the template is Token alone.

### U-C: negated subject-dependent condition guard (M3/F3)

12. **effects/mod.rs** `subject_dependent_type_condition_has_no_subject` (20114) → rename to `subject_dependent_condition_has_no_subject` and add arms:
    - `QuantityCheck { lhs, rhs, .. } => quantity_expr_missing_resolution_only_referent(lhs, state, ability) || quantity_expr_missing_resolution_only_referent(rhs, state, ability)`. Match the real signature at quantity.rs 2243.
    - `QuantityQuality { quantity, .. } => quantity_expr_missing_resolution_only_referent(quantity, …)`
    - Keep the `RevealedHasCardType` and `Not` arms.
    - **Do not recurse And/Or.** An `And`/`Or` under `Not` with one absent operand has defined semantics through each operand's own evaluation. Document that this guard covers only conditions whose subject is a single resolution-only referent.
    - Doc: "CR 608.2c: a condition about an object the resolution never bound has no subject; its negation must not be vacuously true."
13. Fix the PR-added QuantityQuality arm's citation: "CR 701.57c: same absent-referent guard" → "CR 608.2c: …". CR 701.57c is Discover's discovered card, which fits only the QuantityCheck site at ~20576. Leave that one.

### U-D: coverage recursion and sticky Unhandled (M2/F2)

14. **coverage.rs:** add `record_feature` with sticky Unhandled, and route all 7 insert sites through it: 9733 (`emit_structural`), 9746, 9755, 9780, 9818, 9842, 10026.
15. **`extract_condition_quantity_features`** (10003): recurse through `And`, `Or`, `Not`, `ConditionInstead`, plus `PreviousEffectAmount { rhs }`, alongside the existing `QuantityCheck` / `QuantityQuality` operands. Every operand goes through `extract_quantity_features`.
16. **New `extract_trigger_condition_quantity_features`:** walk `QuantityComparison { lhs, rhs }`, `QuantityQuality`, `And`, `Or`, `Not`, and **`EventTime`**. Call it from `extract_trigger_features` (9947).
    - Coverage descent into EventTime is about whether the quantity resolves at all. It is not the C4 recheck question; C4 concerns `positive_count_gate_qty` in the parser. State this in a comment so a reviewer does not conflate the two.
17. **Defective-reference route** (record in the phase-fit record, per SKILL):
    - `quantity_ref_feature` (10242) classifies `SpellsCastLastTurn` and `DescendedThisTurn` as Unhandled. The resolver implements both:
      - quantity.rs 6642: `state.spells_cast_last_turn.map_or(0, i32::from)`;
      - quantity.rs 6620: DescendedThisTurn.
    - Reclassify both as Handled, with CR annotations:
      - `CR 700.11: descended this turn — a permanent card was put into the player's graveyard from anywhere this turn` for DescendedThisTurn.
      - The werewolf day/night transform condition for SpellsCastLastTurn. The executor greps the transform/day-night CR text before citing; if no clean fit, write no CR annotation and flag it "needs manual verification".
    - Evidence tests (step 18). The class covered: every route that drops these refs, i.e. both refs, through both the ability-condition and the trigger-condition walker.
    - `ChosenNumber` and `DungeonsCompleted` stay Unhandled. Re-verify they are not implemented. If they are, the defective-reference route extends to them in the same way.
18. **Coverage tests** (inline in coverage.rs): see the Verification Matrix.

**Predicted coverage flips:**
- Werewolves and descend cards: stay supported, because their refs are reclassified Handled.
- Tunnel Tipster: supported → unsupported, if it is currently supported. Unprobed whether it is supported today; the full coverage diff answers it.
- Every other card: no flip.
- Molten Collapse's DescendedThisTurn sits in modal constraints, which are not walked; it is unchanged.

A **full coverage diff is required:** base vs head `cargo coverage` output. Any flip not in this prediction stops the executor for a design decision.

### U-E: minor

19. **C2:** rename triggers.rs test `quantity_quality_rechecked_at_resolution` (35201) → `bind_triggered_resolution_scope_rechecks_quantity_quality`. Add a doc line pointing to the production recheck coverage in `zimone_all_questioning.rs` (~224). No logic change.
20. **C3: DROPPED.**
    - It predates the PR and the maintainer marked it non-blocking.
    - The Toeclaws grammar ("put +1/+1 counters on it equal to its power") shows the count-less "equal to" form is real. A correct fix needs `opt(count-prefix)` plus a required `equal to` suffix in `try_parse_put_counters_on_token_followup` (sequence.rs ~9780) and `try_parse_token_enters_with_counters` (9607). That is parser grammar work for a follow-up, not this PR.
21. **C4:** in `positive_count_gate_qty` (oracle_trigger.rs 3067), add an explicit `TriggerCondition::EventTime { .. } => None` arm before the wildcard. Comment: "CR 603.4 + CR 603.8: an event-time gate is not rechecked at resolution, so re-reading its quantity then need not give the number that held when the event occurred. It is not a positive-count gate." **No descent**, per the maintainer.
22. **C5: KEPT as is.** The `put_land` + `record_battlefield_entry` hand-stamp in zimone_all_questioning.rs ~453 uses the production recorder, and P1's PlayLand on P0's turn is illegal (CR 305.1). Production recheck is covered at ~224. No change; the plan records the reason.
23. **Wild Hypothesis surveil:**
    - The reparenting is **correct**. A head probe (real cast, X=2, library of 3) showed the Fractal holding 2 counters at the SurveilChoice prompt, and graveyard = 3 after surveil (2 surveilled plus the spell).
    - **No existing test covers it.** Add a runtime test to `created_token_counter_sequence.rs` (row E1).

## Verification Matrix

Each row has three parts:
- **Reach-guard:** proves the code path ran, so a pass is not vacuous.
- **Discriminating assertion.**
- **Revert-failing:** what reverting the unit's source change does to it.

### U-A (ability_chain.rs inline tests, using constructed `Effect` chains)

| ID | Input chain | Assert | Reach-guard | Revert-failing |
|---|---|---|---|---|
| A1 | [Token, PutCounter{LastCreated}] | `created_token_placement_source(.,1) == Some(0)` | n/a (pure function) | stays green (control) |
| A2 | [Token, Mill, PutCounter{LastCreated}] | `Some(0)` (a non-publisher in between) | — | control |
| A3 | [Token, Conjure, PutCounter{LastCreated}] | `None` | — | **fails at base** (base rposition finds Token → Some(0)) |
| A4 | [Token, CopyTokenBlockingAttacker, PutCounter{LastCreated}] | `None` | — | **fails at base** |
| A5 | [Conjure, Token, PutCounter{LastCreated}] | `Some(1)` | — | control |
| A6 | Walk-level: an `AbilityWalk` over the A3 chain | `without_created_token_placements()` keeps the PutCounter (len 3), and the `effect_matches_in_walk` path does not pair it | the walk has 3 effects | **fails at base** |

The existing `created_token_placement_parity.rs` stays green.

### U-B (integration; real cast via GameScenario unless noted)

| ID | Scenario | Assert | Reach-guard | Revert-failing |
|---|---|---|---|---|
| B1 (acceptance row) | Oversimplify, 2 players. P0 and P1 each control Hardened Scales + Branching Evolution. P0 has a 2/2, P1 a 3/3. P0 casts. | At the **first** counter-replacement-order prompt: chooser P0, and **both** Fractals (P0's and P1's) are on the battlefield with 0 counters. Second prompt chooser = P1. Finals: P0 Fractal 6 counters, P1 Fractal 8 (choose Hardened Scales first: (2+1)×2, (3+1)×2). Each Fractal's owner and controller is its creator. | the prompt is the replacement-order `WaitingFor` and its chooser is asserted | **fails at base:** the probe showed prompt #1 with only [(P0,0)] |
| B2 | Oversimplify phase-1 pause. P0 controls Parallel Lives + Anointed Procession, so creating the token raises a replacement-order prompt. P0 2/2, P1 3/3. | After resolving: P0 has 4 Fractals, each with 2 counters; P1 has 1 Fractal with 3. No P1 token got P0's count and vice versa. | **Unprobed:** the executor first asserts the engine actually prompts for Parallel Lives + Anointed Procession ordering (a `WaitingFor` replacement choice). If it does not prompt, substitute another token-creation replacement pair that does, and say which. | **fails at base** if the base sidecar path is not taken: the per-seat binding is absent, so placements read the whole ledger. If it passes at base, record that and keep it as a control for the sidecar route. |
| B3 | The War Games ch. I, 3 players (real chapter trigger) | All 9 Warriors goaded, 3 per player | 9 Warriors exist | **fails at base:** the probe showed 3 goaded, P2's only |
| B4 | The War Games ch. I, 3 players, P1 controls Parallel Lives + Anointed Procession (pause during P1's leg) | All created Warriors goaded: 3 + 12 + 3 = 18 | the replacement prompt was reached (same reach-guard caveat as B2) | **fails at base** |
| B5 | Rendmaw trigger, 3 players | All 3 Birds goaded | 3 Birds exist | **fails at base** (only P2's goaded) |
| B6 | Update `oversimplify_per_player_fractal.rs` | The parse-shape assertion becomes: the placement is `PutCounter{LastCreated}` with `player_scope == Some(All)`. Existing P0 5 / P1 5 (stolen 2/2) results still hold. | the existing direct `resolve_ability_chain` call | the shape assertion **fails at base** |
| B7 | Fix `registered_warrior_direct_regoad_uses_each_goader_original_lifetime` (goaded_creature_under_pacifism_visible.rs) | Pick an explicit recipient whose controller is not P0 by filtering, not "first registered". Tighten `war_games_registered_chapter_token_tracks_saga_presence` from "at least one" to every Warrior recipient (9 in 3p / 6 in 2p) | the recipient count is asserted | the tightened assertion **fails at base** |
| B8 | The effects/mod.rs splitter unit test (rewritten, step 11) | The placement is in the co-scoped tail and the template is Token only | — | **fails at base** (the base arm keeps the placement local) |

### U-C (effects/mod.rs inline, through `resolve_ability_chain`)

The parent is a `GainLife 3` node gated by the condition, on an ability whose `ObjectManaValue{CostPaidObject}` subject is absent or present.

| ID | Condition | Subject | Assert | Revert-failing |
|---|---|---|---|---|
| C1 | Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even)) | absent | no life gained | **fails at base** (Not of false ⇒ gain) |
| C2 | same | present, MV 3 (odd) | gains 3 | control |
| C3 | same | present, MV 2 | no gain | control |
| C4 | QuantityQuality(…, Even) (not negated) | present, MV 2 | gains 3 | control |
| C5 | Not(QuantityCheck{lhs ObjectManaValue(CostPaidObject), GE, Fixed 1}) | absent | no gain | **fails at base** |

Reach-guard: life total read before and after, plus a positive control in the same test (C2) proving the node resolves.

Construction is unit-level because the only parser producer of `QuantityQuality`, `parse_control_count_quality` (oracle_nom/condition.rs ~5164), yields `ObjectCount` only. No printed card reaches this.

### U-D (coverage.rs inline)

| ID | Input | Assert | Revert-failing |
|---|---|---|---|
| D1 | Ability condition `QuantityCheck` on an Unhandled ref (ChosenNumber) | Unhandled recorded | control (already root-walked) |
| D2 | `And[Fixed-check, QuantityCheck(ChosenNumber)]` ability condition | Unhandled | **fails at base** (root-only) |
| D3 | `Not(QuantityQuality(ChosenNumber))` | Unhandled | **fails at base** |
| D4 | Trigger condition `QuantityComparison(ChosenNumber)` | Unhandled | **fails at base** (triggers emit only the structural feature) |
| D5 | Trigger `EventTime { QuantityComparison(ChosenNumber) }` | Unhandled | **fails at base** |
| D6 | Same key `BattlefieldEntriesThisTurn`, non-evaluable filter inserted first, then evaluable | Unhandled | control or fail depending on order |
| D7 | Same key, evaluable first, then non-evaluable | Unhandled | **fails at base** (overwrite) |
| D8 | Evaluable-filter-only controls for D2–D5 (each with an evaluable ref) | Handled | control: proves the walk does not over-report |
| D9 | `card_face_gaps` on Tunnel Tipster's verbatim Oracle text, parsed | reports the gap (unsupported) | **fails at base** |
| D10 | `card_face_gaps` on a verbatim single-face werewolf (SpellsCastLastTurn trigger) and a descend card (DescendedThisTurn) | no gap (supported) | control. Fails if step 17 is skipped. |

**Runtime evidence for the reclassification**, in triggers.rs inline, reusing the `end_step_quality_trigger_stack_len`-style harness or a GameScenario:
- **R1:** a SpellsCastLastTurn-gated upkeep trigger fires when 0 spells were cast last turn and does not fire after 1 spell.
- **R2:** a DescendedThisTurn-gated trigger fires after a permanent card goes to the graveyard and not before (CR 700.11).

If equivalent tests already exist (the executor greps for `spells_cast_last_turn` / `descended` in triggers.rs tests), cite them instead of adding.

### U-E

| ID | Scenario | Assert | Revert-failing |
|---|---|---|---|
| E1 | Wild Hypothesis real cast, X=2, library of 3 | At the SurveilChoice `WaitingFor`, the Fractal has 2 +1/+1 counters. After choosing both to the graveyard, graveyard = 3. | Locks in correct behavior (green at base); a regression guard for the U-B splitter change. |
| E2 | The renamed C2 test | unchanged assertions | n/a |

## Regenerating commands (executor runs; via Tilt where a resource exists)

- `cargo fmt --all`. Always run directly.
- Parse diff: regenerate card-data at base and head with the repo's card-data command (see the project-reference skill), then diff per card.
  - Expected: Oversimplify only, from U-B step 5.
  - Zero cards from U-A widening.
- Full coverage diff: `cargo coverage` at base and head, diffing the supported/unsupported sets. Expected flip: Tunnel Tipster only.
- `cargo engine-inventory` before editing.
- `cargo ai-gate` for U-A. Refresh baselines only with the paired-seed report.
- Tilt resources: `clippy`, `test-engine`, `test-ai`, `card-data`, `wasm`. Run `wasm` because a serialized type changed.

## Reference Readings

- CR 608.2c, 608.2e, 608.2f, 101.4, 616.1, 111.2, 400.7, 115.10a, 609.3, 701.15b, 603.4, 603.8, 700.11, 305.1, 701.57c (verified line numbers: 2799, 2804, 2806, 235, 3179, 647, 1955, 890, 2853, 3402, 2598, 2637, 3266, 1693, 3783).
- `.claude/skills/engine-planner/SKILL.md`, `.claude/skills/engine-implementer/SKILL.md` (phase-fit), `.claude/skills/add-engine-variant` (not triggered: no new variant), `project-reference` (commands, protocol policy).
- The sidecar trace sites listed under Building Blocks; delayed_trigger.rs 517–602 / 1022.

## Maintainer-simulation matrix

| Maintainer point (cited site) | What they will check | Plan answer | Row |
|---|---|---|---|
| F1: Oversimplify violates CR 608.2e (one seat's create+place before the next seat creates) | Both Fractals exist at the first counter-replacement prompt | Steps 5, 6, 9, 10: create for all seats, then place for all seats, each seat bound to its own token | B1, B2 |
| F1: keep each seat's created refs through pauses | Phase-1 pause does not lose or mix tokens | `created_tokens` sidecar channel, extended at drain and resolution time, with incarnation pins | B2, B4 |
| F1: choices in APNAP order | Prompt chooser order P0 then P1 | The driver iterates `matching_players` / APNAP | B1 |
| F1: War Games / Rendmaw same mechanism | Every player's tokens goaded | Union publication (CR 608.2f), sync and pause | B3, B4, B5, B7 |
| F2: coverage.rs ~9750–9761, 9815–9819 (insert overwrite) | Unhandled is sticky regardless of order | `record_feature` at all 7 sites | D6, D7 |
| F2: 9947–9957 (trigger conditions not walked) | Trigger quantities classified | `extract_trigger_condition_quantity_features` | D4, D5, D9 |
| F2: 10003–10016 (root-only ability-condition walk) | Compound conditions walked | Recursion through And/Or/Not/ConditionInstead | D2, D3 |
| F2: 10024–10026 / 10520–10536 (quantity extraction, ref table) | No stale Unhandled refs mask real support; flips predicted | Defective-reference reclassification plus a full coverage diff | D10, R1, R2 |
| F3: Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even)) with no subject | Negation not vacuously true; QuantityCheck too; CR cite | Step 12–13 | C1–C5 |
| F4: ability_chain.rs:178–183, x_reference.rs:334–339 | Pairing uses the nearest publisher from an engine-owned set | Steps 1–3 | A1–A6 |
| C2 test name | Name says what it tests | Renamed | E2 |
| C3 parser fallback | — | Dropped, with reason (predates PR; Toeclaws grammar) | — |
| C4 EventTime | No descent | Explicit `None` arm with CR 603.4/603.8 reason | — |
| C5 hand-stamp | — | Kept, with reason (production recorder; CR 305.1) | — |
| Wild Hypothesis reparenting | Correct and tested | Correct by probe; test added | E1 |

## Per-unit acceptance rows

- **U-A accepted when:**
  - A3, A4 and A6 fail at base and pass at head; A1, A2 and A5 pass at both.
  - The parse diff from widening the publisher set is zero cards.
  - No publisher list remains in phase-ai.
  - `test-ai` is green, and `cargo ai-gate` shows no unexplained delta.
- **U-B accepted when:**
  - **B1 passes:** both Fractals on the battlefield at the first counter-replacement prompt, finals 6 and 8.
  - B2–B8 pass, with B2's and B4's reach-guards demonstrated.
  - The parse diff is exactly Oversimplify.
  - The PR-added local-continuation arm is gone.
  - Every sidecar consumer gates on its channel.
  - The serialized-shape and protocol decision is stated.
  - `test-engine`, `clippy` and `wasm` are green.
- **U-C accepted when:**
  - C1 and C5 fail at base and pass at head; C2–C4 pass at both.
  - The QuantityQuality arm cites CR 608.2c.
- **U-D accepted when:**
  - D2–D5, D7 and D9 fail at base and pass at head; D1, D6, D8 and D10 pass.
  - R1 and R2 (or cited existing tests) pass.
  - The full coverage diff matches the prediction: Tunnel Tipster only, or no flip if Tipster is unsupported today. Any other flip stops the run.
- **U-E accepted when:**
  - E1 passes.
  - The C4 arm is present with no descent.
  - The C2 rename is done.
  - The C3 and C5 decisions are recorded in the PR description.

## Unprobed items (executor must close before relying on them)

1. Whether the Token def carries `player_scope` when `PutCountersOnLastCreated` lowering runs (step 5).
2. Whether the engine prompts for Parallel Lives + Anointed Procession ordering (B2, B4 reach-guards).
3. Whether Tunnel Tipster is currently coverage-supported (U-D flip prediction).
4. The zero parse diff from widening the publisher set (U-A step 2).
5. Whether `CopyTokenBlockingAttacker` and `EachPlayerCopyChosen` resolvers write `last_created_token_ids` (U-A step 1).
6. Whether a Token template can reach the `drain_pending_discard_batch` completion point (step 9).
7. Whether `PendingContinuation` is persisted across versions, which decides the protocol/serde shim (step 7).
