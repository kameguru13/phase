# Charter review r2: run zimone-r2 (review-engine-plan, charter mode)

**Phase-fit context:** declared. The Sizing / recursive NOT(T1 ∧ T2) consistency check is blocking.
**Artifact:** `<run-root>/phase-charter`, revision r2, 4 phases. Run `BASE_SHA` 8949699 (verified: local HEAD = 8949699 on card/zimone-all-questioning, clean tree).
**Inputs read:** CLAUDE.md; review-engine-plan SKILL.md (charter mode); engine-implementer SKILL.md (§Inputs, §Task scope, §Run limits, §Phase-fit gate); chartered.md; review-input.md (including the "NEW since the 19:20Z snapshot" section); triage.md; plan-r1.md; phase-fit entries 1–8; charter-review-r1.md; `diff phase-charter-r1 phase-charter`; addenda/phase-1..4 (all empty).
**Mode constraints honoured:** read-only. No repo edits, no cargo, no stash, checkout or restore. Probes were static: git diff/show, rg, Python reads of `data/card-data.json` and AtomicCards, and one raw.githubusercontent read of upstream main (`git ls-remote` gave its head).

**Round result: 0 decision findings, 5 corrections. This is a clean round.** The orchestrator applies K-1..K-5, and the charter freezes with no further round. None of the corrections moves a phase boundary, the order, a seam, a deferral attribution or a gate verdict. Each was checked against the recursive count after repair (see the table under "Checks").

---

## Prior-round findings: resolved, none mis-applied

- **D-1 is resolved.** D1.1 now settles the question: thin delegators, no rename, callers unchanged. Phase 1's scope rule drops the `oracle_effect/mod.rs` conditional. The worst case is stated in durable form. The one sentence added here ("comment-only doc sites … stay true") is partly false; see K-3. Phase 1 stays far from 13 even after K-2 and K-3.
- **C-1 to C-8 are all applied as supplied.** I checked each C-3 deletion one by one: the Phase 1, 2, 3 and 4 claim blocks; the D1.1 sentence; the D2.2 bullet; the D2.4 sub-bullet, replaced by the two break conditions; both D2.5 sentences; and the D3.3 clause. C-6's seam-note append reads "and the AI pairing", as supplied. The declined-gate walk is named in the Phase 1 deferral sentence and in the scope rule, so nothing is lost there.
- **Sentences added beyond the supplied text:** the r2 provenance line (harmless), the Phase 1 scope-rule clause "their names are kept and their callers are unchanged (D1.1)" (correct), and the "Not admitted" paragraph (K-3).

---

## Decision findings

None.

I measured every code-state assertion a decision rests on (see "Checks") and found none falsified. In particular:
- 145 is still unshipped upstream. Upstream main today is `40d8589`, and its `crates/lobby-broker/src/protocol.rs` has `pub const PROTOCOL_VERSION: u32 = 144;` (raw read at line 1091).
- D2.5's "observable only by a following instruction" holds (see "Checks").

---

## Corrections (the orchestrator applies these; no review round)

### K-1. The 145 ledger inventory leaves out `client/src/adapter/ws-adapter.ts`

The PR's protocol bump wrote a full-game **145** ledger entry in the client's `PROTOCOL_VERSION` doc block, at `client/src/adapter/ws-adapter.ts:214` ("145 — StaticCondition, TriggerCondition and AbilityCondition gain QuantityQuality …"). That is in addition to the wire-127 note in `client/src/network/protocol.ts:109`.

Evidence: `git diff 6864c59 8949699 -G'145|127|PROTOCOL' --name-only` lists 8 files. The three version-pin tests are excluded as protocol-version surfaces: `p2p-adapter-multiplayer.test.ts`, `network/__tests__/protocol.test.ts` and `openPhaseSocket.test.ts`. That leaves **5** ledger files.

Charter r1's review was wrong to call ws-adapter.ts version-number-only. Under D2.4's own rule ("every ledger entry that enumerates what 145 … carries must also name the sidecar change"), the file is a comment-level edit and counts in T2. It is comment-only, so the scope rule admits it as a standing class. This is therefore not a scope finding; the inventory and the figures are wrong.

**T2 effect:** Phase 2 rises to at most 11, and T1 is false, so the verdict does not change. The Evaluation section's draft-phase-1 figure rises to 15, which still fires, so that verdict does not change either.

Following Check 0, the snapshot count is replaced by the rule:

- **Evaluation section.** Old:
  > "That adds up to four files whose ledger lines change, which makes 14 counted paths for a 2-unit phase, and T1∧T2 fires inside the phase."

  New:
  > "That adds every file in which the PR's protocol bump wrote a 145 or wire-127 ledger entry, which takes a 2-unit phase past 13 counted paths, and T1∧T2 fires inside the phase."
- **D2.4, second bullet.** Old:
  > "…must also name the sidecar change: the lobby-broker version ledger, the server-core v145 test doc, the check-protocol-version.mjs v145 comment lines and the client wire-127 note."

  New:
  > "…must also name the sidecar change: the lobby-broker version ledger, the server-core v145 test doc, the check-protocol-version.mjs v145 comment lines, the client full-game 145 entry in `client/src/adapter/ws-adapter.ts` and the client wire-127 note in `client/src/network/protocol.ts`. The set is every file in which the PR's protocol bump wrote a 145 or wire-127 ledger entry; the phase plan regenerates it from the PR diff (`-G'145|127'`), minus files whose only change is a version number."
- **Phase 2 scope rule, ledger sub-list.** After the `scripts/check-protocol-version.mjs` line, add:
  > "- `client/src/adapter/ws-adapter.ts`, the full-game 145 entry"
- **Phase 2 recursive check.** Old:
  > "**Definite (8):** `game/effects/mod.rs`, `types/game_state.rs`, `game/visibility.rs`, `tests/integration/goaded_creature_under_pacifism_visible.rs`, and the four ledger files. … **Up to 10.**"

  New:
  > "**Definite:** `game/effects/mod.rs`, `types/game_state.rs`, `game/visibility.rs`, `tests/integration/goaded_creature_under_pacifism_visible.rs`, and the ledger files named in the scope rule. **Conditional:** a new integration test file, and the `main.rs` mod line. Below 13 either way."

### K-2. Phase 1's scope rule omits the one re-derived token-creator list that D1.1 forbids

D1.1 freezes the rule "the parser and phase-ai both delegate to these predicates, and no publisher list is re-derived anywhere".

**Predicate:** a `match` arm or `matches!` that lists the token-creator set (`Token`, `CopyTokenOf`, `Populate`) or the publisher set (plus `Manifest`, `ManifestDread`, `Cloak`, `Conjure`) in order to answer *"does this effect create the referent a following anaphor names?"*

**Command:**
```
rg -n -U "Effect::Token \{ \.\. \}[\s|]*\n?[\s|]*Effect::(CopyTokenOf|Populate)|Effect::(Populate|CopyTokenOf \{ \.\. \})[\s|]*\n?[\s|]*Effect::Token" crates/
rg -n -U "Effect::Manifest \{ \.\. \}[\s|]*\n?[\s|]*Effect::ManifestDread" crates/
```
Both were run at 8949699 with test files excluded.

**Every matching site and its disposition:**

| Site | What it answers | Disposition |
|---|---|---|
| `parser/oracle_effect/lower.rs:3886` | the authority itself | Becomes the delegator (already in scope) |
| **`parser/oracle_effect/sequence.rs:9300`** | the "The token(s) are goaded" continuation guard, `Effect::CopyTokenOf \| Effect::Token \| Effect::Populate`; exactly today's token-creator set, and the arm War Games, Rendmaw and Life of the Party parse through | **Re-derived token-creator list: delegate. Not in any phase's scope rule.** |
| `parser/oracle_effect/sequence.rs:9283` | the enters-tapped-attacking modifier `{CopyTokenOf, Token, Meld}` | A different question (permanent entry, includes Meld): keep |
| `parser/oracle_effect/assembly.rs:1228` | `BindGuard` `PermanentCreator` `{CopyTokenOf, Token, ChangeZone, Meld}` | A different question: keep |
| `parser/oracle_effect/mod.rs:257` | `payload_mint` `{Token, CopyTokenOf}` → `PayloadMint::Token` | A different question (typed payload mint; leaves out `Populate`): keep; the phase-1 plan confirms |
| `parser/oracle_effect/mod.rs:43438` | the conditional enters-tapped-attacking patch `{CopyTokenOf, Token, ChangeZone}` | A different question: keep |
| `analysis/ability_graph.rs:1030, 1074`; `game/ability_scan.rs:7117, 7514`; `game/resolution_prompt.rs:502`; `phase-ai/src/policies/redundancy_avoidance.rs:594` | exhaustive classification arms (projection, scan context, census role, choice offer, redundancy) | Different questions: keep |

In phase-ai, every created-token pairing goes through `ability_chain` (`AbilityWalk::without_created_token_placements` / `created_token_placement_source`): `cast_facts.rs:529`, `features/plus_one_counters.rs:236`, `policies/x_reference.rs:339`, `anti_self_harm.rs`, `redundancy_avoidance.rs:218`. No other phase-ai site re-derives the list.

`sequence.rs:9300` is a non-comment parser edit, so it is not a standing class. It needs a literal scope entry. The repair is a scope entry and a figure; no phase boundary moves, because Phase 1's worst case stays at or below 10 even with K-3.

- **Phase 1 scope rule.** After the `oracle_effect/lower.rs` bullet, add:
  > "- In `crates/engine/src/parser/oracle_effect/sequence.rs`, only the tokens-goaded continuation arm's token-creator guard, delegation only: the arm calls the engine token-creator predicate instead of listing its members."
- **Phase 1 recursive check, Definite.** Add `parser/oracle_effect/sequence.rs` to the list.
- **Seam notes, first bullet.** Append:
  > "`crates/engine/src/parser/oracle_effect/sequence.rs` is shared by phase 1 (the tokens-goaded guard's delegation) and phase 3 (the placement continuation lowering), in different arms."

### K-3. Phase 1's "Not admitted" sentence is partly false

**Charter text:**
> "The comment-only doc sites elsewhere that name the parser predicates stay true, so the phase admits none of them."

**Measured** (`rg -n "publishes_chain_created_referent|is_token_creating_effect"`, comment lines outside the definite files). Three of the four sites stay true once the parser function is a delegator:
- `game/morph.rs:831` and `:862`: "the parser admits", "the parser side keys on";
- `parser/oracle_effect/tests.rs:46470`, `:46517` and `:51725`;
- `tests/integration/perpetual_last_created_empty_no_source_fallback.rs:4`.

`types/zones.rs:111` does not. It calls the parser function "the parser's admission authority", one of "two halves of one contract [that] have to be changed together". After D1.1 the membership authority is the `impl Effect` predicate, so the comment would send a maintainer to edit a delegator.

**T2 effect:** at most +1. Phase 1's worst case becomes the 8 definite paths (with K-2) plus `x_reference.rs` plus `types/zones.rs`, that is 10, below 13. No decision changes.

If *every* doc site were admitted, the count would reach 13 and fire. The durable form below keeps the admission to the one site that goes stale.

**Old:**
> "The comment-only doc sites elsewhere that name the parser predicates stay true, so the phase admits none of them."
> "**Worst case:** the definite paths plus `x_reference.rs` (if a walk-level test is placed there)."

**New:**
> "**Conditional, comment-only:** `types/zones.rs`, whose doc names the parser function as the membership authority to change together with the engine marks; after D1.1 that authority is the `impl Effect` predicate. The other comment sites that name the parser predicates (`rg -n 'publishes_chain_created_referent|is_token_creating_effect'` outside the definite files) still describe a delegator truthfully and are not admitted."
> "**Worst case:** the definite paths plus `x_reference.rs` (if a walk-level test is placed there) plus `types/zones.rs` (comment-only)."

### K-4. D1.1 does not say which predicate a candidate member joins

D1.1 defines the token-creator predicate as "the set the parser calls token-creating today". The "one runtime criterion" (the resolver assigns the `LastCreated` referent) defines the publisher superset. The candidate sentence ("added only if its resolver is shown to meet the criterion") does not say which predicate a candidate joins.

That matters downstream:
- the token-creator set gates Phase 2's union publication (D2.1);
- through the parser delegator, it also gates tracked-set and aggregate publication (`oracle_effect/mod.rs:35704`, `:35801`);
- after K-2, it gates the tokens-goaded continuation as well.

Evidence that the question is live: `copy_token_blocking.rs:87` assigns `last_created_token_ids`.

**Old:**
> "Each candidate member the draft names (`CopyTokenBlockingAttacker`, `EachPlayerCopyChosen`) is added only if its resolver is shown to meet the criterion."

**New:**
> "Each candidate member the draft names (`CopyTokenBlockingAttacker`, `EachPlayerCopyChosen`) joins the publisher predicate only if its resolver is shown to meet the criterion, and joins the token-creator predicate only if, in addition, the object it publishes is a token it creates (CR 111.1). Any change to the token-creator set is a parse-diff change the phase plan derives card by card."

CR 111.1 was verified: "Some effects put tokens onto the battlefield."

### K-5. The seam notes' parse-diff expectations are predicted outcomes, not architecture

**Charter text** (Seam notes › Integration review consistency checks):
> "Parse-diff expectations across the run span: phase 1: zero cards, or the cards D1.1 derives; phase 2: zero cards (runtime only); phase 3: exactly Oversimplify's placement scope; phase 4: none (coverage only)."

Charter mode says acceptance rows and claims a phase must establish belong to the phase plan. "Exactly Oversimplify" is a corpus outcome stated as fact.

My census found Oversimplify to be the only player-scoped `Token → PutCounter{LastCreated}` chain. But that census ran on `data/card-data.json` generated at 17:08Z, before BASE (committed 19:18Z), so it is a lead, not a measurement at base. No decision rests on the sentence.

**New:**
> "Parse diff: each phase plan derives its expected parse diff from the Oracle text of every changed card. The integration review checks that the run-span parse diff is the union of those derived sets, and that phases 2 and 4, which change no parser file, contribute none."

---

## Checks requested by the orchestrator, with measured evidence

### Phase seams are real green-tree seams

- **1→2.** After D1.1, the `impl Effect` predicates have production callers through:
  - the parser delegators (`lower.rs:3874`, `3979`, `4444`, `5139`; `oracle_effect/mod.rs:26816`, `27048`, `35704`, `35801`; and `sequence.rs:9300` after K-2);
  - the declined-gate walk (`effects/mod.rs:5094`);
  - phase-ai's `created_token_placement_source`.

  They are `pub` methods on a `pub` type, so there is no dead-code lint either way. Phase 2's driver is the deferred consumer, recorded as `DEFERRED(phase 2)`.
- **2→3.**
  - Oversimplify's local pairing is still present at `effects/mod.rs:6987–6998` (`is_player_scope_local_continuation`).
  - Its parse carries `player_scope: All` on the Token node only, with the `PutCounter{LastCreated}` child unscoped (card data).
  - Phase 2 publishes the union at clause completion. Oversimplify has no instruction after the placement, and the paused publication point ("last seat leg drains") comes after each seat's local placement, so Oversimplify's readings cannot move.
  - **D2.5's premise is measured:**
    - `last_created_token_ids` is a most-recent-producer slot (`game_state.rs:23099`, "Consumed by sub_abilities … via TargetFilter::LastCreated").
    - Producers reassign it (`token.rs:660`, `756`, `1230`; `token_copy.rs:237`; `morph.rs:885`), and `engine.rs:7010` clears it in frame bookkeeping.
    - It is read only by `LastCreated` resolution (`targeting.rs:780`, `830`, `1944`; `filter.rs:6126`) and by the replacement restore (`engine_replacement.rs:2492–2514`).
    - A read across resolutions would already be a stale-referent defect, independent of this run.
  - Phase 3's detached placement becomes the splitter's `after_scope` tail (`split_player_scope_chain` → `detach_after_player_scope_local_chain`, 7380–7505). Phase 2's paused path parks that tail in the generalized sidecar, so nothing is foreclosed.
  - Stamping the subject's scope on the placement (D3.1) does not trip the co-scoped anaphoric-consumer arm:
    - `effect_introduces_per_iteration_referent` covers only `RevealTop` and the reveal-card choice (7277–7296);
    - `effect_has_iteration_bound_recipient` does not cover `PutCounter`.

    Removing the local pairing therefore actually detaches the placement.
- **D3.4 (no new serialized shape) is consistent with the driver.**
  - On a pause, the driver builds every remaining seat's leg eagerly as a concrete `ResolvedAbility` (`bind_scoped_seat`, 16247–16265) and parks it in the continuation.
  - A per-seat `LastCreated → ParentTarget` bind computed at loop start is therefore serialized inside existing `ResolvedAbility` targets.
  - D3.4's ride-145 fallback covers the case where the phase plan finds otherwise.
- **3→4.** Independent. Phase 4 touches only `coverage.rs`, plus `triggers.rs` and `quantity.rs` if needed. Nothing in phases 1–3 reads coverage.

### Recursive NOT(T1 ∧ T2) per phase, after K-1..K-3

Integration tests count; comment-only files count; version-pin-only files and regenerated card data are excluded.

| Phase | T1 | Worst-case T2 | Verdict |
|---|---|---|---|
| 1 | true (3–5 units) | 7 original definite + `sequence.rs` (K-2) + `x_reference.rs` + `types/zones.rs` (K-3) = **10** | single |
| 2 | false (1 unit) | 4 + 5 ledger files (K-1) + new test + `main.rs` = **11** | single |
| 3 | false | sequence.rs, effects/mod.rs, oversimplify test + assembly.rs, new test, `main.rs` = **6** | single |
| 4 | false | coverage.rs + triggers.rs + quantity.rs = **3** | single |

Margin for Phase 1: 3. An AI baseline refresh (`crates/phase-ai/baselines/suite-baseline.json`) would be regenerated data, which is excluded.

`phase-ai/src/policies/tests/created_token_placement_parity.rs` is the one plausible further test site. With it, the count is 11, still single.

### Deferral allowlists foreclose nothing

- Phase 1 leaves the token-creator set exactly as the parser has it today (`lower.rs:3883–3888`: `Populate | Token | CopyTokenOf`), which is what Phase 2's union keys on.
- D1.2 pairs on the definition chain (`collect_chain_effects` ignores `player_scope`), so D3.1's stamp leaves the AI reading of Oversimplify unchanged.
- Phase 2's `DEFERRED(phase 3)` rows each name phase 3.
- No phase defers to Phase 4, and Phase 4 defers nothing.

### The protocol decision (D2.4) is sound

- **145 is still unshipped.**
  - At the PR's main merge base `6864c59`, `PROTOCOL_VERSION` is 144.
  - Today's upstream main `40d8589` (`git ls-remote upstream refs/heads/main`) is still 144: its `lobby-broker/src/protocol.rs:1091` was read via raw.githubusercontent, and its newest ledger entry is "144 —".
- **Floor and enforcement.** The full-game floor is current-only (`MIN_SUPPORTED_PROTOCOL = PROTOCOL_VERSION`; `full_game_floor_is_current_only_not_a_rollout_window`). `check-protocol-version.mjs` pins numbers and carries ledger comments only; it has no schema fingerprint.
- **The sidecar is additive and legacy-decodable.**
  - It is `pub(crate) struct PendingPlayerScopeLinkedExile { source_id, after_scope, #[serde(default, skip_serializing_if = "Vec::is_empty")] batch }` (`game_state.rs:2819`), carried on `PendingContinuation.player_scope_linked_exile` with `#[serde(default, skip_serializing_if = "Option::is_none")]` ("Legacy saves default to `None`").
  - There is no TS binding, since the type is `pub(crate)`.
  - Every construction and read site is in `effects/mod.rs` or `game_state.rs`. `after_scope_needs_linked_exile` has 12 uses in `effects/mod.rs` and 1 in `game_state.rs` (`PendingDiscardFanOut`).
  - The resolution-frame legacy readers decode `PendingContinuation` through serde (`types/resolution.rs` v1–v5), so an optional serde-default channel needs no `RESOLUTION_STATE_WIRE_VERSION` bump.

### Every F1–F4 maintainer-requested control has a phase

| Blocker | Phase and decision |
|---|---|
| F1 | Phase 2 (union through synchronous and paused fan-out); Phase 3 (all seats finish action 1 first; per-seat refs through pauses; the real-cast, unequal-power, two-player Oversimplify test with Hardened Scales + Branching Evolution, with both Fractals at the first prompt) |
| F2 | Phase 4: D4.1 sticky `Unhandled` under both aggregation orders; D4.2 ability compounds; D4.3 trigger conditions; direct cases |
| F3 | Phase 1, D1.3: positive, negated, absent, present. The helper `quantity_expr_missing_resolution_only_referent` exists in `game/quantity.rs` and is already used at `effects/mod.rs:14901`, `20592` and `20613` |
| F4 | Phase 1, D1.1 + D1.2: ordinary and intervening-Conjure chains; the nearest publisher must be `Token` |

The controls themselves are carried in the Original task section (C-2 from r1).

### No decision rests on an unverified code-state assertion

Each was measured at 8949699 unless stated otherwise:
- D1.1's "today's set": `lower.rs:3883`.
- D1.3:
  - the `Not` guard recurses only through `Not` (`effects/mod.rs:20114–20127`);
  - the PR's `QuantityQuality` arm cites CR 701.57c (20610);
  - the `QuantityCheck` arm's 701.57c citation predates the PR (present at `6864c59`, line 20533), so D1.3 rightly scopes the citation fix to the PR-added arm;
  - the And/Or "no printed card reaches" claim rests on r1's corpus probe.
- D1.4: `positive_count_gate_qty` (`oracle_trigger.rs:3067–3086`) has no `EventTime` arm; `TriggerCondition::EventTime` exists (`types/ability.rs:31203`).
- D2.1: one publication authority, `publish_player_scope_clause_results` (13569). Its callers are 14690, 16082 and 16288, all in `effects/mod.rs`.
- D2.4: as above.
- D2.5: as above.
- D3.2: the local pairing (6987–6998) and the Windfall co-scoped path (7415–7442).
- D3.3: the delayed-trigger snapshot precedent, as r1 verified.
- Premise table: the Oracle text of all six cards matches AtomicCards. So do Hardened Scales and Branching Evolution, the F1 control's cards.
- CR numbers: 101.4, 111.1, 111.2, 115.10a, 400.7, 603.4, 608.2c, 608.2e, 608.2f, 609.3, 616.1, 700.11, 701.15b and 701.57c all exist in `docs/MagicCompRules.txt` with the meanings used.

---

## Residual assumptions (for the phase-plan reviewers, not findings)

- **Phase 2's class has a third printed member: Life of the Party.** Its Oracle text (AtomicCards): "each opponent creates a token that's a copy of it. The tokens are goaded for the rest of the game". It parses as player-scoped `CopyTokenOf` → unscoped `GenericEffect{LastCreated}` (card data, which predates BASE). D2.1's class rule covers it because `CopyTokenOf` is in the token-creator set. The Phase 2 plan should derive its row alongside War Games and Rendmaw.
- **D2.3 channel keying.** r1's note stands. The linked-exile channel has no presence key of its own: `batch` is skipped when empty. Of the two queue-end parks, the driver's (`effects/mod.rs:16309`) sits inside the `after_scope_needs_linked_exile` branch, and the drain's (`18078`) is keyed on sidecar presence. Phase 2 must give the union-only sidecar its own key without misrouting the linked-exile drain readers. That means a typed marker whose legacy default means linked exile, or a need derived from `after_scope`, and not a `bool` (CLAUDE.md).
- **Phase 4 test sites.** `created_token_counter_sequence.rs` asserts coverage gaps, so a derived Phase 4 flip may need an edit there. That is ordinary addendum drift.

## Pre-existing

Nothing new. r1's single item (a negated compound with an absent subject-dependent operand) is not repeated.
