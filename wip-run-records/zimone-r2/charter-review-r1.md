# Charter review r1: run zimone-r2 (review-engine-plan, charter mode)

**Phase-fit context:** declared. The Sizing / recursive NOT(T1 ∧ T2) consistency check is blocking.
**Artifact:** `<run-root>/phase-charter` (4 phases). Run `BASE_SHA` 8949699 (verified: local HEAD = 8949699, branch card/zimone-all-questioning).
**Inputs read:** CLAUDE.md, review-engine-plan SKILL.md (charter mode), engine-implementer SKILL.md (§Phase-fit gate, §Run limits), chartered.md, review-input.md (including the "NEW since the 19:20Z snapshot" section), triage.md, plan-r1.md, phase-fit (entries 1–6), addenda/phase-1..4 (all empty).
**Mode constraints honoured:** read-only. No repo edits, no cargo, no stash, checkout or restore. Probes were static only: git diff, grep, and Python reads of committed card data and AtomicCards.

**Round result:** 1 decision finding and 8 corrections, so this is a **design round**.

---

## Decision findings

### D-1. Phase 1's recursive check fails in its own worst-case branch (blocking: Sizing / phase-fit)

**Charter text (Phase 1 › Recursive check):**
> "three comment-only doc sites that name the parser predicates, only if the predicates are renamed. Worst case 12, below 13, so T2 is false and NOT(T1∧T2) holds. Keeping the parser names as delegators (D1.1) caps the count at 8."

**Measured.** `rg -n "publishes_chain_created_referent|is_token_creating_effect"` at 8949699, keeping only comment lines. The parser predicate names appear in comments in four files that are not already in Phase 1's definite set:

| File | Lines |
|---|---|
| `crates/engine/src/types/zones.rs` | 111 |
| `crates/engine/src/parser/oracle_effect/tests.rs` | 46470, 46517, 51725 |
| `crates/engine/src/game/morph.rs` | 831, 862 |
| `crates/engine/tests/integration/perpetual_last_created_empty_no_source_fallback.rs` | 4 |

There are further comment sites in `lower.rs` (3968) and `game/effects/mod.rs` (5060), but both files are already counted. The charter's "three" probably missed `parser/oracle_effect/tests.rs`.

**The count, under the run's conventions** (integration and other test code count; comment-only files count in T2 once admitted, per chartered.md "Every extension re-counts T2 over the extended list"):
- 7 definite paths;
- plus `oracle_effect/mod.rs` and `x_reference.rs`;
- plus 4 doc-site files;
- = **13**. With T1 true (3 to 5 units), the T1∧T2 conjunction fires inside Phase 1 on the rename branch.

The fallback figure is also off. With the names kept, the conditional paths are `oracle_effect/mod.rs` (only if callers are switched anyway) and `x_reference.rs`, so the cap is 9, not 8. It is 8 only if keeping the names also means the callers are not switched.

**Why this is a decision.** The phase boundary (U-A + U-C + U-E in one phase) rests on this count. D1.1 currently leaves the rename open ("may keep"). The pre-named fallback seam would turn a predictable worst case into a scope-freeze re-charter.

**Rename hazard.** A rename would also touch a non-comment line outside Phase 1's `effects/mod.rs` subsystem rule: the `use crate::parser::oracle_effect::publishes_chain_created_referent;` import at line 11 and the call at line 5094 in the declined-gate walk.

**Required revision (recommended).** Make D1.1 decisive:
1. The parser keeps `is_token_creating_effect` and `publishes_chain_created_referent` as thin delegators to the new `impl Effect` predicates. No rename; existing parser callers are unchanged.
2. Delete the rename conditional and the `oracle_effect/mod.rs` conditional from Phase 1's scope rule and its recursive check.
3. Restate the worst case in durable form: "the definite paths plus `x_reference.rs` (if a walk-level test is placed there)". Do not restate it as a number.

The alternative is to keep the rename branch and split U-E out now, but that adds a phase for no architectural gain.

---

## Corrections (the orchestrator applies these; no review round)

### C-1. Original task: stale source statement and misattribution

**Old text:**
> "It also takes the minimal CodeRabbit follow-ups that the maintainer marked non-blocking (C2, C4, and the Wild Hypothesis regression guard). The maintainer's non-blocking stances on C3, C4 and C5 are recorded in phase-fit entry 3: C4 must not recurse blindly. review-input.md has no "NEW since 19:20Z" section; that entry is the only source for these stances."

**Why it is wrong:**
- review-input.md now has that section (phase-fit entry 6 records the refresh).
- The maintainer recorded stances only on C3, C4 and C5.
- No maintainer stance on C2 is recorded.
- The Wild Hypothesis guard comes from the maintainer's parse-evidence note ("structural reparenting"), not from CodeRabbit.

**Replacement:**
> "It also takes minimal follow-ups: C4 (an explicit no-descent `EventTime` arm), C2 (a test rename and doc; review-input records no maintainer stance on C2), and a Wild Hypothesis regression guard (from the maintainer's 'structural reparenting' parse-evidence note). The maintainer's stances, from review-input.md's 'NEW since the 19:20Z snapshot' section and phase-fit entry 3, are:
> - C5: 'Nonblocking limitation';
> - C4: 'The state-trigger head is not admitted by the current grammar. Blind recursion is not justified.';
> - C3: 'The malformed-count fallback predates this change. Nonblocking.'"

### C-2. Original task: carry the maintainer's requested controls as task acceptance criteria

SKILL.md §Task scope says to "keep the original requested behavior and acceptance criteria in every handoff". The charter's F1–F4 bullets drop the maintainer's requested controls. Once C-3 removes the claim blocks, nothing in the charter carries them.

**Replacement** for the four bullets under "Original task" (each keeps its current first clause, then appends):
- **F1/M1:** "… Maintainer's fix shape: create tokens for all seats first, keep each seat's created-object refs through pauses, then place counters. Requested control: a real-cast, unequal-power, two-player Oversimplify test (Hardened Scales + Branching Evolution) requiring both Fractals at the first replacement prompt."
- **F2/M2:** "… Fix shape: recurse through ability and trigger conditions with `extract_quantity_features`; keep `Unhandled` on repeated keys. Controls: direct, compound and trigger cases, and both aggregation orders."
- **F3/M3:** "… Fix shape: extend the pre-negation guard via `quantity_expr_missing_resolution_only_referent`. Controls: positive, negated, absent and present."
- **F4/M4:** "… Fix shape: engine-owned publisher classification; pair only when the nearest publisher is `Token`. Controls: ordinary and intervening-Conjure chains."

The phase attribution is already correct in the blocker table (F4 and F3 → 1, F1 → 2 and 3, F2 → 4).

### C-3. Delete the "claims the phase plan must establish" content

Charter mode says these belong in the phase plan. Delete each of the following:
- Phase 1, "Claims the phase plan must establish (by measurement, not assumption)": the whole block.
- D1.1, the sentence "The plan establishes membership by measurement, resolver by resolver, against that criterion." Keep the next sentence (the candidate-member rule), which is a decision.
- D2.2, the bullet "The plan establishes that its attribution key is exact. It includes a hostile fixture …".
- D2.4, the "Claims the plan must establish" sub-bullet with (a), (b) and (c). Keep the two contingency sentences as the decision's break conditions, reworded as: "If 145 has shipped upstream by handoff, re-numbering is a PR-handoff merge concern, not charter drift. If the sidecar shape cannot decode every legacy form additively, D2.3 and D2.4 break: re-charter the remaining phases."
- D2.5, the sentences "The plan establishes this claim." and "The plan also establishes which player-scoped token clauses … from its Oracle text."
- Phase 2, "Claims the phase plan must establish": the whole block.
- D3.3, the clause "; the plan includes a hostile fixture for this".
- Phase 3, "Claims the phase plan must establish": the whole block.
- Phase 4, "Claims the phase plan must establish": the whole block.

Keep D4.4's condition ("only where … the resolver implements the ref and a runtime test proves it"). It is the defective-reference route's own entry condition, not an acceptance row.

### C-4. D1.3's And/Or rationale is false as written

The charter freezes it into the guard's doc ("the guard's doc says why").

**Old text:**
> "`And`/`Or` are not recursed, and the guard's doc says why: each operand evaluates on its own terms."

**Evidence** (effects/mod.rs at 8949699): `Not` checks `subject_dependent_type_condition_has_no_subject` (20114–20127), which recurses only through `Not`. `And` and `Or` evaluate their operands (21063–21069). So for `Not(And([A_absent, B]))`:
1. A evaluates false by its own absent-referent guard;
2. `And` is false;
3. `Not` returns true.

Each operand evaluating on its own terms is exactly what makes the negated compound vacuously true.

**Corpus probe** (zimone-f1 head card data, every `Not` whose operand is `And`/`Or`): 6 cards.
- Ability-condition sites: Dose of Dawnglow `Not(And(CurrentPhaseIs, IsYourTurn))` and Twinned Vision `Not(And(WasCast, SourceMatchesFilter))`. Neither has a subject-dependent operand.
- The rest are trigger or static conditions: Chorale of the Void, Skullcage, Intrepid Ace, Tromokratis.

No printed card reaches the gap, so the decision not to recurse can stand. Only the rationale is wrong.

**Replacement:**
> "`And`/`Or` are not recursed. The guard covers a negated subject-dependent leaf. A negated compound with a subject-dependent operand (for example `Not(And(QuantityQuality{absent}, …))`) remains vacuously true, and the guard's doc records this as a known limitation that no printed card reaches."

### C-5. D1.6 states a code-state outcome as fact

**Old text:**
> "A cast-pipeline test locks in today's correct reading: counters on the Fractal before the surveil prompt, and Surveil still resolves. It is green at the phase base by design and must stay green through phase 3."

**Replacement:**
> "A cast-pipeline test guards Wild Hypothesis's reading derived from its Oracle text: counters on the Fractal before the surveil prompt, and Surveil still resolves. It must stay green through phase 3. If it is red at the phase base, the defect takes the pre-existing-report route."

Static evidence that this is low-risk: the head parse is `Token → PutCounter{X, LastCreated} → Surveil(2, SequentialSibling)`, with no `player_scope` on any node.

### C-6. Phase 1 deferral sentence is inaccurate

**Old text:**
> "In phase 1 the predicates reach production only through parser delegation, which keeps the tree free of dead code."

The publisher predicate also reaches production through:
- phase-ai's pairing (D1.2, Phase 1's own consumer);
- the game-layer declined-gate walk (`game/effects/mod.rs:5094`, through the parser name).

**Replacement:**
> "In phase 1 the predicates reach production through parser delegation (and, for the publisher predicate, through the AI pairing and the declined-gate walk), which keeps the tree free of dead code."

The 1→2 seam note ("the phase-1 predicates have production callers through parser delegation") is true but incomplete. Append "and the AI pairing".

### C-7. Phase 1's `effects/mod.rs` scope sentence omits the existing game-layer publisher consumer

`game/effects/mod.rs` imports the parser predicate at line 11 and calls it at line 5094 in `declined_gate_surviving_instructions`. That makes it a third consumer of the classification D1.1 centralizes. The file is already counted, so there is no T2 effect.

**Old text:**
> "In `crates/engine/src/game/effects/mod.rs`, only the condition evaluator's no-subject guard and its inline tests."

**Replacement:**
> "In `crates/engine/src/game/effects/mod.rs`, only the condition evaluator's no-subject guard and its inline tests, and the declined-gate walk's publisher call (line-level: it may switch from the parser delegator to the engine predicate)."

### C-8. "Outside the run": the C3 and C5 rationales are unverified or code-state claims

No phase rests on them, since C3 is dropped and C5 is unchanged.

**C3 old text:**
> "The Roaring Toeclaws' count-less 'put +1/+1 counters on it equal to its power' means a correct fix is grammar work …"

The card text is verified (AtomicCards). But the fallback lives in `try_parse_put_counters_on_token_followup` (sequence.rs:9733–9780), a follow-up to a created token. The Roaring Toeclaws creates no token and cannot reach it. Corpus check: only 2 cards print a count-less "put +1/+1 counters on it/that creature", Roaring Toeclaws and Experiment Twelve, and neither creates a token.

**C3 replacement:**
> "It predates the PR, and the maintainer marked it non-blocking ('The malformed-count fallback predates this change. Nonblocking.'). Follow-up only."

**C5:** replace the two code-state sentences with:
> "Maintainer stance: 'Nonblocking limitation'. Reply material is in triage.md §C5."

---

## Checks requested by the orchestrator, with measured evidence

- **Phase seams are real green-tree seams** (infrastructure → consumer):
  - **1→2:** Phase 1's predicates have live production callers once the parser delegates. Verified at `lower.rs` 3874 and 3979/4444/5139, `oracle_effect/mod.rs` 26816/27048/35704/35801, `effects/mod.rs` 5094, plus phase-ai through D1.2. Phase 2 consumes the token-creator predicate in the driver.
  - **2→3:** Phase 3's per-seat binding reads Phase 2's published union. Oversimplify's local-continuation arm is still present until Phase 3 (verified at `is_player_scope_local_continuation`, effects/mod.rs ~6987–6998), so its rows are honestly `DEFERRED(phase 3)`.
  - **3→4:** independent.
  - Phase 3's detached co-scoped placement is the splitter's `after_scope` tail (effects/mod.rs 7425–7495), so Phase 2's paused-union routing ("after-scope needs the created union") serves Phase 3's tail as well. There is no foreclosure.
- **Recursive NOT(T1 ∧ T2) per phase:**
  - Phase 1 fails in its worst case (D-1).
  - Phase 2: 1 unit, ≤10 paths. The four ledger files correctly count, since a comment naming a serialized change is not a "protocol-version surface". Consistent.
  - Phase 3: 1 unit, ≤6. Consistent. The class-table test `created_token_counter_sequence.rs` asserts only that "Token is directly followed by a ContinuationStep PutCounter{LastCreated}" and coverage gaps. Stamping `player_scope` on the placement keeps that shape. The assembly snapshot `…assembly_create_token_put_counter.snap` is a non-player-scoped Soldier, so it is unaffected.
  - Phase 4: 1 unit, ≤3. Consistent.
  - Exclusions are applied as directed: integration tests count; regenerated card data is excluded; version-number-only files are excluded (the PR's client version-pin tests and `ws-adapter.ts` are not in any phase's count, which is correct).
- **Deferral allowlists do not foreclose later phases.** Phase 1's token-creator predicate is the set Phase 2 needs (`Token | Populate | CopyTokenOf` today, at `lower.rs:3883`). D1.2 pairs Oversimplify on the definition chain, which Phase 3 does not restructure (D3.1 only stamps scope). Phase 2 keeps Oversimplify unchanged for Phase 3. Phase 4 shares no subsystem.
- **The protocol decision (D2.4) is sound for this repo's policy:**
  - `git diff 6864c59 8949699` shows the PR moving `PROTOCOL_VERSION` 144→145 and `WIRE_PROTOCOL_VERSION` 126→127, with ledger lines in lobby-broker, server-core, `check-protocol-version.mjs` and `protocol.ts`. At the merged main 6864c59 the constant is 144, so 145 was unshipped there. Upstream main beyond 6864c59 could not be read: GitHub access is not enabled for this session, and the local `upstream/main` ref (33aa23a) is older. D2.4's handoff contingency covers that.
  - The full-game floor is current-only: `server-core/src/protocol.rs:31` `MIN_SUPPORTED_PROTOCOL = PROTOCOL_VERSION`, and the test is `full_game_floor_is_current_only_not_a_rollout_window`.
  - Repo practice bumps the number even for serde-additive `GameState` fields (v142 `LinkedExileSnapshot.incarnation`; the server-core v114 doc: "a v113 peer would drop it silently, so it must be refused"). So the sidecar field does need protocol coverage. One unshipped number covering several changes from one PR is established practice (v144 = boons + P/T retype).
  - `check-protocol-version.mjs` has no schema fingerprint, so riding 145 needs only the ledger comment lines.
  - The sidecar does ride the client/P2P wire: `client_state_wire_value` (derived_views.rs ~981) does not strip `resolution_stack`. So the wire-127 note entry is legitimate.
  - Persisted saves decode through resolution-frame migration (`RESOLUTION_STATE_WIRE_VERSION` 5, with v1–v4 legacy readers). `player_scope_linked_exile` already documents "Legacy saves default to `None`", so an optional serde-default channel is the established no-resolution-bump pattern. (The v4 bump was for a *required* field.)
- **Phase 4 matches the maintainer's F2 fix shape:**
  - D4.1 is sticky `Unhandled` across every `features.insert`. That covers the maintainer's sites 9750–9761, 9818, 10024–10026; the static walker shares the insert, so it is in scope.
  - D4.2 recurses ability conditions, after the static walker's And/Or/Not recursion at 9737–9785 (verified).
  - D4.3 adds a trigger-condition walker; `extract_trigger_features` (9947–9957) today emits only `structural:trigger_condition`.
- **No conflict with the maintainer's C4 stance.** The C4 thread (oracle_trigger.rs 3073–3077) sits inside the parser's `positive_count_gate_qty` (3067–3086), which binds a "that many" anaphor. D1.4 keeps it non-descending, as the maintainer asked. D4.3's descent is in `coverage.rs`, a different function answering a different question. Confirmed.
- **Phase 4 CI feasibility (checked because derived flips are expected).** The blocking "Coverage regression check (engine vs parser-honesty)" (`ci.yml` ~568, `--fail-on-engine`) fails only when a flipped card loses a handler that was a supported `parse_details` item. Resolver-feature gaps enter `missing` as `ResolverFeature:<key>` (`check_resolver_features`, coverage.rs 8237), not as parse-item handlers. A Phase 4 flip such as Tunnel Tipster therefore lands in the non-fatal honesty bucket.
- **Every maintainer-requested F1–F4 control is assigned to a phase:**
  - F1: Phase 2 (union through pauses) and Phase 3 (both Fractals at the first prompt; per-seat refs through pauses).
  - F2: Phase 4 (direct, compound, trigger; sticky in both orders).
  - F3: Phase 1, D1.3 (positive, negated, absent, present).
  - F4: Phase 1, D1.1 and D1.2 (ordinary and intervening-Conjure).
  - The control lists themselves need C-2 so they survive C-3.
- **Code-state assertions that decisions rest on**, each measured at 8949699:
  - D1.3: the PR's `QuantityQuality` arm cites CR 701.57c (effects/mod.rs ~20612), and the revealed-card-type arm exists.
  - D2.1: there is a single publication authority (`publish_player_scope_clause_results`, 13569). It is also used by the resumed discard batch, so the "both completion points" wording undercounts, harmlessly.
  - D2.3:
    - every sidecar reader and writer is in `game_state.rs` or `effects/mod.rs`; `visibility.rs` 3864 is comment-only;
    - the queue-end park is keyed on presence (effects/mod.rs 18075);
    - linked-exile handling at drain is keyed on presence (1253–1365).
  - D3.1: Oversimplify's placement carries no `player_scope` today (head data: `player_scope: All` on the Token node only).
  - D3.2: the local-continuation pairing exists (~6987–6998) and the Windfall co-scoped tail path exists (7415–7442).
  - D3.3: the delayed-trigger snapshot precedent is at `effects/delayed_trigger.rs` 517–533.
  - D1.5: `resolution_recheck_and_live_count` is at `zimone_all_questioning.rs:224` and `quantity_quality_rechecked_at_resolution` at `triggers.rs:35201`.
  - Premise table: the Oracle text of all six cards matches AtomicCards.
  - CR numbers: 101.4, 111.2, 115.10a, 305.1, 400.7, 603.4, 608.2c, 608.2e, 608.2f, 609.3, 616.1, 700.11, 701.15b and 701.57c all exist in `docs/MagicCompRules.txt` with the meanings the charter uses.
  - Assertions with no decision resting on them are covered by C-1, C-5, C-6 and C-8. The only falsified decision-bearing figure is D-1's.

## Residual assumptions (for the phase-plan reviewers, not findings)

- **D2.3 channel keying.** The linked-exile channel has no presence key of its own: `batch` is `skip_serializing_if = Vec::is_empty`, and today "linked exile" means "sidecar present". To key linked-exile consumers "on the channel they need" (`mark_exile_choice_tracks_by_source`, `bind_resolution_exile_batch_paths` at drain) without misfiring on a union-only sidecar, the Phase 2 plan must do one of two things:
  - derive the linked-exile need from `after_scope` at drain; or
  - add a typed marker whose legacy default means linked exile.

  A `bool` is disfavoured by CLAUDE.md. Either way, D2.3's "decoded without it behaves exactly as today" must hold.
- **Phase-ai parity suite.** `crates/phase-ai/src/policies/tests/created_token_placement_parity.rs` is outside Phase 1's scope rule. Per triage, no corpus chain has an intervening publisher, so D1.2 should not move it. If it does, that is a plan-step return or an addendum, not a foreclosure.
- **Phase 4 test sites.** Runtime evidence for a reclassified ref, or a derived class-member gap change in `created_token_counter_sequence.rs`, may need a test file outside Phase 4's scope rule. That is ordinary addendum drift.

## Pre-existing (untagged, non-blocking; listed once)

- `Not(And/Or(…))` with a subject-dependent operand whose referent is absent evaluates vacuously true. This applies to `RevealedHasCardType` and `QuantityCheck` at base, and to `QuantityQuality` at head. No printed card reaches it (corpus probe in C-4). The change does not depend on it.
