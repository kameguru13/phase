# Phase 1 plan r1: run zimone-r2 (engine-owned created-referent authority, F3 guard, minor follow-ups)

Mode: `engine-planner` phase-plan mode. Phase index 1 of 4. `PHASE_BASE_SHA` = run `BASE_SHA` = `8949699a13a541a7e00ada2bb8e16c00c530b82a` (branch `card/zimone-all-questioning`, PR phase-rs/phase#10004). Units: U-A (F4), U-C (F3), U-E (C2, C4, Wild Hypothesis guard). Charter decisions D1.1–D1.6 are carried out as written; nothing here relitigates them.

Evidence discipline. This plan was produced read-only: no cargo build ran (the box has ~4.6 GB free on `/`, Tilt is not installed, and the instructions forbid building in the main checkout). Every code claim below was read at `8949699`. Corpus claims come from static Python censuses over `data/card-data.json`, which was generated at 17:08Z from the PR head before `BASE` merged main (`dfc1a50..8949699` touches only loyalty/static-parser files, none of the sites below). They are labelled **lead (static census)** and each names the measurement the executor runs to confirm it. Census scripts: `/tmp/claude-0/-home-user-phase-personal/3ffacad5-7db0-5e02-a1a7-192aa01f873f/scratchpad/p1/census_*.py`.

CR numbers cited in this plan, each grepped in `docs/MagicCompRules.txt` (line): 101.4 (235), 111.1 (645), 111.2 (647), 111.7 (661), 111.10f (679), 122.6 (1210), 603.4 (2598), 603.8 (2637), 608.2c (2799), 608.2e (2804), 608.2f (2806), 608.2h (2812), 609.3 (2853), 616.1 (3179), 700.1 (3206), 701.16a (3410), 701.25a (3516), 701.40a (3638), 701.47a (3697), 701.57c (3783), 701.58a (3787), 701.62a (3827), 702.116a (4859), 702.141a (5053), 704.3 (5506), 704.5f (5523), 707.2 (5633). Conjure and Empower Jace are digital-only and have no CR entry; their arms carry no CR number.

---

## Step 0 — Premise verification (Oracle text, `data/mtgjson/AtomicCards.json`)

| Card | Verbatim clause the phase depends on | Used by |
|---|---|---|
| Wild Hypothesis {X}{G} | "Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it.\nSurveil 2. (Look at the top two cards of your library, then put any number of them into your graveyard and the rest on top of your library in any order.)" | E1, E1b |
| Oversimplify | "Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it …" | seam only (D1.2 must still pair it; phases 2–3) |
| Mirror Match | "For each creature attacking you or a planeswalker you control, create a token that's a copy of that creature and that's blocking that creature. Exile those tokens at end of combat." | D1.1 member derivation |
| Panther Pounce | "Target player investigates. Target creature gets +1/+0 and gains flying until end of turn. Untap it." | parse-diff derivation |
| Disorder in the Court | "Exile X target creatures, then investigate X times. Return the exiled cards to the battlefield tapped …" | token-creator non-widening reason |
| Elemental, My Dear | "Investigate X times, then create an 0/0 Blue Elemental creature token. Put +1/+1 counters on that token equal to the number of Clues you control." | parse-diff derivation, A-rows |
| Zimone, All-Questioning | "… create Primo, the Indivisible, … then put that many +1/+1 counters on it." | C2 doc pointer (`resolution_recheck_and_live_count`) |

F3 and F4 have no printed member (the maintainer's own examples are constructed: `Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even))`, `[Token, Conjure, PutCounter{LastCreated}]`). Lead (static census, `census_not.py`, `census_ai.py`): the only `AbilityCondition::Not(QuantityCheck|QuantityQuality)` in the corpus is Ravenous Necrotitan, whose operand reads no resolution-only scope; no corpus chain has a publisher between a `Token` and a `PutCounter{LastCreated}`. Both fixes are building-block fixes; their tests are constructed by necessity and drive production functions.

## Step 1 — Applicable skills and checklist skeleton

| Skill | Applies to | Checklist disposition in this phase |
|---|---|---|
| `/casting-stack-conditions` (§Sub-Ability Chain Resolution, §Condition Systems, §Common Pitfalls) | U-C: `evaluate_condition`'s `Not` arm, reached from `resolve_ability_chain`'s top-level condition gate | Condition evaluated at resolution (CR 608.2h) — unchanged. No new `WaitingFor`, no `PendingCast`/`StackEntry` field, no serde change, no intervening-if change. Verification block: unchanged references. |
| `/oracle-parser` §1 rules, §9a Phase 6, Self-Review | U-A delegation in `lower.rs` / `sequence.rs`; U-E C4 arm in `oracle_trigger.rs` | §9a Phases 1–5 (new pattern, subject, chain composition, routing): N/A — no Oracle text is newly accepted or newly matched; no combinator is added. Phase 6: no parser unit test is needed for a delegation (shape unchanged); the parse diff (expected zero) and the combinator gate are the verification. |
| `/card-test` | E1/E1b Wild Hypothesis real cast | `GameScenario` + `GameRunner::cast(..).x(..)`, verbatim Oracle, assertions via `CastOutcome`/runner accessors; the prompt-stop is driven manually because the default driver auto-answers `SurveilChoice` (keep-on-top). |
| `/add-engine-variant` | — | Not triggered: no enum variant is added (two `impl Effect` methods only). |
| `/add-ai-feature-policy` | — | Not triggered: no `DeckFeatures` axis or policy is added; `created_token_placement_source` only consumes the engine predicate. CLAUDE.md's `cargo ai-gate` rule applies (V-AI). |

`DEFERRED(phase 2)`: the player-scope driver's consumption of the token-creator predicate (D2.1). `DEFERRED(phase 2/3)`: every Oversimplify / War Games / Rendmaw behaviour row. `DEFERRED(phase 4)`: coverage.

## Step 2 — Analogous traces

1. **The parser's publisher authority, end to end** (the thing D1.1 moves): `parser/oracle_effect/lower.rs::is_token_creating_effect` / `publishes_chain_created_referent` → parser consumers `lower.rs::resolve_populated_token_anaphors` (nearest token creator), `relink_gated_token_referent_consumers`, `clone_would_transplant_gated_referent`, the token-cleanup gate (`is_token_creating_effect(&inner.effect)`), `oracle_effect/mod.rs::chain_prior_referent_is_created_token`, `bare_they_player_declaration`, `publishes_tracked_set_from_resolution`, `classify_bare_card_aggregate_publisher`, `sequence.rs` tokens-goaded arm → game consumer `game/effects/mod.rs::declined_gate_surviving_instructions` (imports the parser function) → AI consumer `phase-ai/src/ability_chain.rs::created_token_placement_source` (re-derives `Token` only) → runtime referent `GameState::last_created_token_ids`, read by `TargetFilter::LastCreated` (`game/targeting.rs`, `game/filter.rs`).
2. **The engine-owned AST classification idiom**: `types/ability.rs::Effect::moves_card_to_or_from_library` — a node-only, static-AST predicate on `impl Effect` with a **wildcard-free** match ("a new `Effect` variant is a compile error here, forcing its author to answer … at the one place that owns the answer"). U-A's publisher predicate follows it exactly; that is what stops the next missed member (the parser list already misses six measured writers, see D1.1 membership).
3. **The existing absent-referent guard**: `game/effects/mod.rs::subject_dependent_type_condition_has_no_subject` (issue #2871, `RevealedHasCardType`, recursion through `Not` only) ← called by `evaluate_condition`'s `Not` arm; the leaf test it needs already exists as `game/quantity.rs::quantity_expr_missing_resolution_only_referent(state, expr, ability)`, used by the `QuantityCheck`/`QuantityQuality` arms of `evaluate_condition` and by `condition_awaits_resolution_only_referent`. Integration evidence for the revealed arm: `tests/integration/issue_2871_currency_converter.rs`.
4. **Wild Hypothesis runtime path**: `created_token_counter_sequence.rs` runtime rows (`moved_placement_keeps_its_x_count`, Fractal Summoning X=3) → `game/scenario.rs` `SpellCast::commit` / `GameRunner::resolve_top` / `act`; `WaitingFor::SurveilChoice` handled in `engine_resolution_choices.rs` (`SelectCards { cards }` = the ordered keep-on-top set; every other looked-at card goes to the graveyard).

## Step 3 — Files read

`types/ability.rs` (`impl Effect`, `moves_card_to_or_from_library`, `TriggerCondition::EventTime` doc, `AbilityCondition` helpers), `parser/oracle_effect/lower.rs` (3860–4000, 4415–4470, 5139), `parser/oracle_effect/sequence.rs` (8432–8520, 9265–9330, 9477–9500), `parser/oracle_effect/mod.rs` (imports 20–60, 26740–26860, 27030–27060, 35680–35810), `game/effects/mod.rs` (imports, 5040–5115, 14880–14925, 15360–15400, 16385–16420, 18635–18665, 20095–20135, 20560–20630, 21060–21085, test 37589), `game/quantity.rs` (2160–2300), every non-test writer of `last_created_token_ids` (see D1.1 membership), `phase-ai/src/ability_chain.rs` (whole file), `phase-ai/src/policies/x_reference.rs` (263–360), `phase-ai/src/features/plus_one_counters.rs` (215–245), `parser/oracle_trigger.rs` (3038–3086, 2360–2385), `game/triggers.rs` (35170–35250), `tests/integration/created_token_counter_sequence.rs` (whole), `tests/integration/zimone_all_questioning.rs` (224–245), `tests/integration/perpetual_last_created_empty_no_source_fallback.rs`, `types/zones.rs` (95–125), `game/morph.rs` (825–890), `game/scenario.rs` (1895–1906, 2595–2610, 3662–3730).

## Step 3.5 — Probes

Static only (no build was possible within the constraints). Probed by reading/census, each with a positive control:
- Ledger writers: `rg -n "last_created_token_ids\s*=|last_created_token_ids\.(push|extend|clear)|record_last_created_(copy_batch_)?token\(|publish_face_down_entry_referent\(|begin_face_down_referent_production\(" crates/engine/src/game`, then callers of `token::resolve`, `token_copy::resolve`, `drive_copy_token_batches`, `apply_create_token_after_replacement*`, traced to their `Effect` dispatch arm (positive control: the seven parser members all appear).
- AI pairing over the corpus under the widened set: `census_ai.py` — 51 base-paired placement prefixes, **0** classification changes.
- Parser sites that could see a newly admitted publisher: `census_members.py` / `census_anaphor.py` / `census_gate.py` (positive control: Elemental, My Dear's `LastCreated` reader is found).
- Negated quantity conditions: `census_not.py` (positive control: 37 negated quantity conditions found, one `AbilityCondition`).

Not probed (each closes in the executor's first build; see §Unprobed): the exact compile form of the `sequence.rs` guard, the Wild Hypothesis prompt-stop sequence, whether a faceless `Conjure{Named}` battlefield object accepts a counter, and every "fails at base" claim (derived from code, not run).

---

## Pattern Coverage

| Unit | Class | Members |
|---|---|---|
| U-A | Every chain in which a `PutCounter{target: LastCreated}` follows one or more effects that publish the `LastCreated` referent. The runtime referent is the **nearest** publisher's product (each publisher *assigns* the slot); the AI reads the placement as part of a token's creation only when that publisher is `Effect::Token`. And every engine/parser site that asks "does this effect publish the referent a later `LastCreated` read names?" | 15 publisher variants (D1.1 membership). Corpus: 51 paired prefixes keep their reading (lead); the class rows cover every non-`Token` publisher interleaved. Charter class attribution: F4 (D1.1/D1.2), plus phase 2's union consumer. |
| U-C | A negated subject-dependent condition whose single resolution-only subject is absent: `Not(X)` with X ∈ {`RevealedHasCardType`, `QuantityCheck`, `QuantityQuality`} over any `ObjectScope` that `quantity_expr_missing_resolution_only_referent` classifies (CostPaidObject, Target, EventSource, Anaphoric/Demonstrative, EventTarget, OtherRevealedCard, OwnedLinkedExileCard, AmassedArmy, ChainRootTarget, GrantingObject, BatchSource) | No printed member today (lead). Building-block fix (charter F3). |
| U-E | C2 test honesty, C4 explicit no-descent arm, and a regression guard for the "create, put counters on it, then an independent instruction" chain shape (Wild Hypothesis) that phase 3's splitter change must not disturb | Wild Hypothesis, plus every single-player Token → PutCounter{LastCreated} → independent tail. |

Not 1-card: U-A covers a 15-variant class and every chain shape; U-C covers 3 condition shapes × 11 scopes.

## Sizing

**Units: 3** (U-A, U-C, U-E). A reviewer splitting U-E into C2 / C4 / Wild Hypothesis gets 5. T1 is true either way.

| Unit | Behaviour | Registration surfaces | Discriminating tests |
|---|---|---|---|
| U-A | Engine-owned publisher + token-creator predicates; AI pairs only with the nearest publisher when it is `Token`; parser and game layer delegate | 2 `pub fn` on `impl Effect` (no variant, no serialized surface, no `WaitingFor`/`GameAction`) | A2, A3, A4, A6 (fail at base) |
| U-C | `Not` over an absent resolution-only subject is false, for `QuantityCheck` and `QuantityQuality` (the revealed arm unchanged) | none (private fn) | C1, C5, C6 (fail at base) |
| U-E | C2 rename/doc; C4 explicit `EventTime => None`; Wild Hypothesis guard | none | none by construction (C2 and C4 are behaviour-identical; E1/E1b are guards green at base) |

**Dependency edges:** none inside the phase (U-A, U-C, U-E are independent). Outgoing: U-A → phase 2 (`Effect::is_token_creator` is the driver's union key, D2.1).

**Expected changed paths** (phase-fit counting; integration tests count; nothing excluded applies — no fixtures, no regenerated data committed, no protocol surface):

| # | Path | Unit | Kind |
|---|---|---|---|
| 1 | `crates/engine/src/types/ability.rs` | U-A | definite |
| 2 | `crates/engine/src/parser/oracle_effect/lower.rs` | U-A | definite (delegators + doc move) |
| 3 | `crates/engine/src/parser/oracle_effect/sequence.rs` | U-A | definite (goaded arm delegation only) |
| 4 | `crates/phase-ai/src/ability_chain.rs` | U-A | definite (incl. inline tests) |
| 5 | `crates/engine/src/game/effects/mod.rs` | U-A (declined-gate call + import), U-C | definite |
| 6 | `crates/engine/src/game/triggers.rs` | U-E (C2) | definite |
| 7 | `crates/engine/src/parser/oracle_trigger.rs` | U-E (C4) | definite |
| 8 | `crates/engine/tests/integration/created_token_counter_sequence.rs` | U-E (E1/E1b), U-A (R1) | definite |
| 9 | `crates/engine/src/types/zones.rs` | U-A | comment-only (charter conditional; this plan edits it) |
| — | `crates/phase-ai/src/policies/x_reference.rs` | — | **not touched**: every walk-level row lives in `ability_chain.rs` |

**Count: 9** (worst case 10 if a reviewer moves a walk row into `x_reference.rs`). T2 false → NOT(T1∧T2) → single phase. No generated-artifact grouping (`ChainReferentIntent` has no TS binding: `rg -l ChainReferentIntent` finds Rust only).

## Building Blocks

- `game::quantity::quantity_expr_missing_resolution_only_referent(state, expr, ability)` — the single absent-referent authority (the maintainer's named fix shape). Used as-is.
- `Effect::moves_card_to_or_from_library` — the wildcard-free classification idiom copied for the publisher predicate (its variant list is the copy source for the `false` arm).
- `phase-ai::ability_chain::{collect_chain_effects, AbilityWalk, created_token_placement_source}` and `features::plus_one_counters::ability_creates_token_with_plus_one_counters` (production consumer used by tests).
- `phase-ai::policies::tests::created_token_placement_parity::{token_effect, last_created_placement, effect_chain}` — existing fixture builders.
- `engine::game::effects::{conjure, resolve_ability_chain}`, `GameScenario`/`GameRunner` (`commit`, `resolve_top`, `act`, `graveyard_count`, `counters`).
- No new helper is introduced.

## Logic Placement

- Both predicates: `types/ability.rs` `impl Effect`. They are static-AST, node-only classifications consumed by three crates' layers (parser, game, phase-ai); `moves_card_to_or_from_library` sets the precedent. The parser keeps its names as delegators (D1.1) so no parser caller moves.
- `declined_gate_surviving_instructions` (game layer) switches from the parser import to `node.effect.publishes_created_referent()`: the game layer should not depend on a parser function, and the charter permits the line-level switch.
- AI: `phase-ai/src/ability_chain.rs` consumes the engine predicate and adds no classification of its own beyond D1.2's `Token` check.
- U-C: `game/effects/mod.rs` condition evaluator (the only `Not` evaluation site).
- U-E: test-only plus one parser match arm.
- Frontend, WASM, server, protocol: untouched.

## Rust Idioms

- `Effect::publishes_created_referent(&self) -> bool`: exhaustive, **wildcard-free** `match self` — one grouped `true` arm listing members, each sub-group with its CR/resolver comment, one grouped `false` arm listing every other variant. Doc carries the "do not add an `_ =>` arm" sentence, as the precedent does.
- `Effect::is_token_creator(&self) -> bool`: `matches!(self, Effect::Token { .. } | Effect::CopyTokenOf { .. } | Effect::Populate)` — a fixed, documented subset; its subset invariant is unit-tested (P1). (Exhaustiveness lives on the superset; any new token creator must first answer the superset question.)
- AI pairing: `chain[..index].iter().rposition(|e| e.publishes_created_referent())?` then `matches!(chain[p], Effect::Token { .. }).then_some(p)`.
- U-C: two new match arms on `AbilityCondition`; the guard keeps its `_ => false` (it is a guard over a 100+-variant enum whose default answer is "has a subject"; the existing function already uses that shape).
- No `bool` parameters, no new types.

## Nom Compliance (parser files change)

- `lower.rs`: two functions become one-line delegators. No dispatch, no text matching.
- `sequence.rs`: the tokens-goaded arm's effect-pattern guard `Effect::CopyTokenOf { .. } | Effect::Token { .. } | Effect::Populate` becomes a predicate call; the Oracle-text detection stays `try_parse_tokens_goaded_continuation` (nom `alt`/`tag`, unchanged). No `contains`/`starts_with` is added.
- `oracle_trigger.rs`: one `TriggerCondition` match arm; no text.
- Oracle text newly accepted with deferred semantics: **none**. Coverage stays honest by construction (no parse change; expected parse diff zero).

## Extension vs Creation

Extends: the parser's publisher authority (moved, not re-created), the `impl Effect` classification idiom, the existing no-subject guard (two arms), the existing AI pairing function. Creates: nothing new beyond the two methods D1.1 mandates.

## Variant Discoverability

No enum variant is proposed; `/add-engine-variant` is not triggered. `rg -n "fn (creates_tokens?|is_token_creat\w*|publishes_\w*referent\w*)\b" crates/` finds only the two parser functions, so the methods duplicate nothing. (`data/engine-inventory.json` is not needed for a method addition.)

## Identity / Provenance Contract

| Concept | Authority & storage | Binding time | Live vs snapshot | Consumer | Invalidation | Multi-authority hostile fixture |
|---|---|---|---|---|---|---|
| "it" / "that token" after a publisher (CR 608.2c) | `GameState::last_created_token_ids`, **assigned** by the nearest publisher's resolver | at each publisher's resolution | snapshot of ids; each later publisher overwrites | `TargetFilter::LastCreated` (`targeting.rs`, `filter.rs`); statically, `created_token_placement_source` | next publisher's assignment; `engine.rs` frame bookkeeping clear | A2 `[Token, Conjure, P]` (two publishers; nearest wins), A5 `[Conjure, Token, P]` (order reversed), A7 `[Token, Incubate, P]` (a token-**creating** effect that does **not** write the slot — pairing must still reach the Token), R1 (runtime: the conjured object, not the Soldier, gets the counter) |
| The subject of a negated subject-dependent condition (CR 608.2c) | `ResolvedAbility` referents (`cost_paid_object`, `effect_context_object`, `targets`, trigger event) read through `resolution_only_scope_referent_present` | at resolution (CR 608.2h) | live | `evaluate_condition` `Not` arm via the guard | n/a | C7: `cost_paid_object` absent but `effect_context_object` present (the presence ladder's fallback must make the subject present, so the guard must not fire) |

---

## Architecture decisions carried out (phase-plan detail)

### D1.1 — membership, measured resolver by resolver

Criterion (charter): the effect's resolver assigns the referent `TargetFilter::LastCreated` reads. Measured non-test writers and their `Effect` arm:

| Effect | Resolver write site (symbol) | Today in parser set? | Joins publisher | Joins token-creator |
|---|---|---|---|---|
| `Token` | `token::resolve` / `apply_create_token_after_replacement_with_created_ids` / `record_last_created_token` | yes (both) | yes | yes |
| `CopyTokenOf` | `token_copy::resolve` (clears on empty source) / `drive_copy_token_batches` completion | yes (both) | yes | yes |
| `Populate` | `populate::resolve` → `token_copy::resolve` | yes (both) | yes | yes |
| `Manifest`, `ManifestDread`, `Cloak` | `morph::begin_face_down_referent_production` (clear) + `zone_pipeline` `publish_face_down_entry_referent` | publisher | yes | no (not tokens) |
| `Conjure` | `conjure::resolve` (`last_created_token_ids = created_ids`) | publisher | yes | no (not tokens) |
| `CopyTokenBlockingAttacker` | `copy_token_blocking::resolve` republishes `all_created` | **no** | **yes** | no — see below |
| `EachPlayerCopyChosen` | inner `CopyTokenOf` per player (`each_player_copy_chosen` → `resolve_ability_chain` depth 1) | **no** | **yes** | no |
| `Investigate` | `investigate::resolve` → `token::resolve` (Clue, CR 701.16a / 111.10f) | **no** | **yes** | no |
| `Amass` | `amass::resolve` → `apply_create_token_after_replacement` (only when no Army, CR 701.47a) | **no** | **yes** | no |
| `EmpowerJace` | `empower_jace::resolve` → `apply_create_token_after_replacement` (only when no Jace) | **no** | **yes** | no |
| `Myriad` | `myriad::resolve` → `token_copy::resolve` per opponent (CR 702.116a) | **no** | **yes** | no |
| `Encore` | `encore::resolve` → `token_copy::resolve` per opponent (CR 702.141a) | **no** | **yes** | no |
| `CreateTokenCopyFromPool` | `drive_copy_token_batches` / `token::no_token` | **no** | **yes** | no |

Measured non-members worth naming: `Incubate` creates an Incubator token without writing the slot (`incubate.rs` calls neither writer) → not a publisher; that is consistent with runtime (a `LastCreated` read after Incubate still reads the earlier referent), and it is fixture A7. `Meld`, `ChangeZone` (incl. to the battlefield) do not write the slot → not publishers. Command that regenerates the writer set: the `rg` in §Step 3.5.

The publisher doc states the criterion as "writes the slot on at least one resolution path", and says plainly that for `Amass` and `EmpowerJace` the write happens only when they create the Army/Jace — so a later `LastCreated` read may name their object, which is exactly why a placement after them must not pair with an earlier `Token`.

**Token-creator stays `{Token, CopyTokenOf, Populate}`** (charter: "the set the parser calls token-creating today"; a candidate joins "only if" it also publishes a token it creates — a necessary condition). Every consumer of this set is a parser nearest-producer scan (`resolve_populated_token_anaphors`, `publishes_tracked_set_from_resolution` via `nearest_publisher_is_self_move`, `classify_bare_card_aggregate_publisher`, the token-cleanup gate, the goaded arm) or phase 2's union. Admitting a keyword action that creates a token as one step would move those scans: e.g. Disorder in the Court ("Exile X target creatures, then investigate X times. Return the exiled cards …") would see Investigate as the nearest tracked-set publisher. `CopyTokenBlockingAttacker` meets both conditions, but its only printed card (Mirror Match) lowers its "those tokens" follow-up bespoke to `LastCreated`, so no card needs it; it is recorded as a follow-up candidate, not admitted. Result: **zero** token-creator parse change by construction.

### D1.2 — AI pairing

`created_token_placement_source`: nearest preceding publisher via the engine predicate; pair only if it is `Effect::Token`. Doc records why `CopyTokenOf`/`Populate` stay unpaired: the pairing exists to give the imperative "create … then put counters on it" the same reading as the declarative `Token { enter_with_counters }` (the parity suite's contract); `CopyTokenOf` and `Populate` carry no `enter_with_counters` twin, so pairing them would invent a reading no consumer defines. No other AI logic is added.

### D1.3 — guard

Rename `subject_dependent_type_condition_has_no_subject` → `subject_dependent_condition_has_no_subject` (2 call sites, same file). Add arms `QuantityCheck { lhs, rhs, .. }` and `QuantityQuality { quantity, .. }` calling `quantity_expr_missing_resolution_only_referent`. `And`/`Or` not recursed; the doc records the known limitation verbatim from D1.3. The PR-added `evaluate_condition` `QuantityQuality` arm comment cites CR 608.2c (not 701.57c); the pre-PR `QuantityCheck` arm's CR 701.57c (Discover) rationale is left. The PR-added test comment in `quantity_quality_ability_condition_evaluates_at_resolution` ("CR 701.57c absent-referent guard") becomes CR 608.2c.

Considered and not done: folding the guard's two arms and `condition_awaits_resolution_only_referent`'s identical leaf arms into one leaf helper. The two walkers deliberately recurse differently (the deferral walk recurses `And`/`Or`; the guard must not), the shared logic already lives in `quantity_expr_missing_resolution_only_referent`, and `condition_awaits_resolution_only_referent` is outside phase 1's subsystem scope rule.

### D1.4 — C4

Explicit `TriggerCondition::EventTime { .. } => None` before `_ => None` in `positive_count_gate_qty`, comment: "CR 603.4 + CR 603.8: an `EventTime` condition is the trigger event itself (an attack-while gate, an event count qualifier, a state trigger's condition), read when the event occurs and never rechecked; CR 603.4's recheck applies only to an intervening "if". It is not a positive resolution-recheck count gate, so it names no antecedent for the body's count anaphor — binding one would read the count at resolution, not the count the event measured. Deliberately no descent." Also add `EventTime` to the doc list ("`Not`, `Or` and `EventTime` never bind").

### D1.5 — C2

Rename `game/triggers.rs::quantity_quality_rechecked_at_resolution` → `bind_triggered_resolution_scope_rechecks_quantity_quality`; doc: it exercises `stack::bind_triggered_resolution_scope` with a constructed scope; production coverage of the stacked trigger's recheck is `tests/integration/zimone_all_questioning.rs::resolution_recheck_and_live_count`. No logic change.

### D1.6 — Wild Hypothesis guard

E1/E1b below, in `created_token_counter_sequence.rs`.

---

## Step-by-step plan

### U-A

1. **`crates/engine/src/types/ability.rs`, `impl Effect`** (next to `moves_card_to_or_from_library`):
   - `pub fn publishes_created_referent(&self) -> bool` — doc moves and generalizes the rationale now on `lower.rs::publishes_chain_created_referent` (Conductive Machete / Weight Room #7531, Agent of Raffine): "CR 608.2c: true when this effect's resolver writes the referent `TargetFilter::LastCreated` reads (`GameState::last_created_token_ids`) on at least one resolution path. Each such write is an assignment, so the referent a later instruction reads is the *nearest* publisher's product. Consumers: the parser's anaphor passes (through `publishes_chain_created_referent`), the declined-gate walk, phase-ai's created-token pairing." Wildcard-free `match`; `true` arm grouped with comments: token creation (CR 111.1: `Token`, `CopyTokenOf`, `Populate`); face-down entry (CR 701.40a, 701.58a, 701.62a: `Manifest`, `ManifestDread`, `Cloak`); `Conjure` (digital-only); copies (CR 707.2: `CopyTokenBlockingAttacker`, `EachPlayerCopyChosen`, `CreateTokenCopyFromPool`); keyword actions/abilities that create a token as one step (CR 701.16a `Investigate`, CR 701.47a `Amass`, CR 702.116a `Myriad`, CR 702.141a `Encore`, `EmpowerJace` digital-only) with the conditional-write sentence for Amass/EmpowerJace. `false` arm: every other variant (copy the variant list from `moves_card_to_or_from_library` and drop the members).
   - `pub fn is_token_creator(&self) -> bool` — `matches!` over `Token | CopyTokenOf | Populate`; doc: "CR 111.1: the token-creation effects whose own parameters define the token(s) — a spec, a copy source, or the populated token. A subset of `publishes_created_referent`. Keyword actions that create a token as one step of a larger action publish the referent but are deliberately not members: this set keys the parser's nearest-token-creator and tracked-set scans and the player-scope created-token union, and widening it moves those scans."
   - Inline tests (new `#[cfg(test)] mod created_referent_classification_tests` at file end): P1–P3.
2. **`crates/engine/src/parser/oracle_effect/lower.rs`**: `is_token_creating_effect(effect)` → `effect.is_token_creator()`; `publishes_chain_created_referent(effect)` → `effect.publishes_created_referent()`. Visibility and names unchanged. Each keeps a two-line doc naming the method as the authority (the long rationale has moved). Callers untouched.
3. **`crates/engine/src/parser/oracle_effect/sequence.rs`**, tokens-goaded arm only: replace the effect-pattern list with the predicate, same arm position, same detector. The crate is edition 2021 (no let chains), so use one `if let` guard, e.g. `effect if let Some(continuation) = effect.is_token_creator().then(|| try_parse_tokens_goaded_continuation(&lower)).flatten() => Some(continuation)`, or any equivalent that compiles, keeps the arm's position, and evaluates the detector only for token creators. Keep the CR 701.15a + 701.15b comment.
4. **`crates/engine/src/game/effects/mod.rs`**: delete `use crate::parser::oracle_effect::publishes_chain_created_referent;`; in `declined_gate_surviving_instructions` call `node.effect.publishes_created_referent()`; update the adjacent comment to name `Effect::publishes_created_referent`.
5. **`crates/engine/src/types/zones.rs`** (`ChainReferentIntent` doc, comment-only): "the parser's admission authority (`oracle_effect::lower::publishes_chain_created_referent`)" → "the publisher classification (`Effect::publishes_created_referent`, which the parser delegates to)".
6. **`crates/phase-ai/src/ability_chain.rs`**: module doc (the created-token-placement paragraph) and `created_token_placement_source` doc/body per D1.2. Rename the existing test `placement_pairs_with_the_nearest_preceding_token` → `placement_pairs_with_the_nearest_publisher_when_it_is_a_token` and fix its doc ("whatever sits between them" is no longer true). Add A-rows.
7. **AI gate**: `cargo ai-gate` per CLAUDE.md (V-AI). Expected: no delta (0 corpus classification changes, lead). If baselines move, refresh only with the paired-seed report and return the delta to the orchestrator.

### U-C

8. **`crates/engine/src/game/effects/mod.rs`**: rename + two arms + doc per D1.3; fix the PR-added CR citation in the `QuantityQuality` arm and in the PR-added test comment. Add C-rows as inline tests next to `quantity_quality_ability_condition_evaluates_at_resolution`.

### U-E

9. **`crates/engine/src/game/triggers.rs`**: C2 rename + doc (D1.5).
10. **`crates/engine/src/parser/oracle_trigger.rs`**: C4 arm + doc line (D1.4).
11. **`crates/engine/tests/integration/created_token_counter_sequence.rs`**: E1, E1b, R1 (a `const WILD_HYPOTHESIS` with the verbatim Oracle; the CLASS row already uses the same text).

### Verification commands (executor; Tilt is absent here, so direct cargo with an isolated target dir, `CARGO_INCREMENTAL=0`)

- `cargo fmt --all`
- `cargo clippy -p phase-engine -p phase-ai --lib --tests -- -D warnings`
- `cargo test -p phase-engine --features test-support --lib -- created_referent_classification_tests subject_dependent quantity_quality bind_triggered_resolution_scope_rechecks_quantity_quality positive_count_gate` (the C-rows' final names) and `cargo test -p phase-engine --features test-support --lib parser::oracle_trigger` (the existing `positive_count_gate_qty_binds_only_positive_gates` stays green)
- `cargo test -p phase-engine --features test-support --test integration -- created_token_counter_sequence issue_2871 zimone_all_questioning oversimplify perpetual_last_created war_games goaded`
- `cargo test -p phase-ai --lib` (ability_chain tests + `created_token_placement_parity` + every consumer suite)
- `./scripts/check-parser-combinators.sh 8949699a13a541a7e00ada2bb8e16c00c530b82a`
- Parse diff: `./scripts/gen-card-data.sh` at `PHASE_BASE_SHA` (into the run dir, e.g. `zimone-r2/base-data-p1/`) and at the candidate; compare per card (and/or `cargo coverage` both sides + `cargo run --profile tool --features cli --bin coverage-parse-diff -- <base> <head>`, the format the PR's sticky comment uses). **Expected: zero cards.**
- `cargo ai-gate` (V-AI).
- Disk: ~4.6 GB free on `/` at planning time; the orchestrator should confirm capacity before the first cold build.

---

## Verification Matrix

Legend: **fails@base** = derived from the base code to fail when the phase's source change is reverted (unprobed until the executor runs it); **control** = green at base and head.

### U-A — engine predicates (`types/ability.rs` inline; new API, so pure unit rows)

| ID | Assertion | Reach-guard / sibling | Status |
|---|---|---|---|
| P1 | For fixtures `Token`, `CopyTokenOf`, `Populate`: `is_token_creator()` and `publishes_created_referent()` both true (subset invariant) | three distinct variants constructed | new-API unit |
| P2 | `publishes_created_referent()` true, `is_token_creator()` false for `Conjure`, `Manifest`, `Investigate`, `CopyTokenBlockingAttacker`, `Amass` | — | new-API unit; the discriminating counterparts are A3/A4 |
| P3 | Both false for `Incubate`, `Mill`, `PutCounter{LastCreated}`, `ChangeZone{→Battlefield}`, `Meld` | P1/P2 prove the method answers true at all | new-API unit (hostile: token-creating non-publisher, battlefield movers) |

### U-A — AI pairing (`phase-ai/src/ability_chain.rs` inline; drives production consumers)

`P` = `last_created_placement(Plus1Plus1, 1)`; abilities built with `effect_chain(Spell, ContinuationStep, …)`.

| ID | Chain | Assertions | Reach-guard | Status |
|---|---|---|---|---|
| A1 (F4 ordinary) | `[Token, P]` | `source_at(..,1) == Some(0)`; `ability_creates_token_with_plus_one_counters` true; `AbilityWalk::unconditional(..).without_created_token_placements().len() == 1` | positive leg of A2 | control |
| A2 (F4 intervening Conjure) | `[Token, Conjure, P]` | `source_at(..,2) == None`; `ability_creates_token_with_plus_one_counters` **false**; walk keeps all 3 | A1 shows the same consumers return the paired answer | **fails@base** (base `rposition(Token)` → `Some(0)`, consumer true, walk len 2) |
| A3 (class) | `[Token, X, P]` for **every** X in the publisher `true` arm except `Token` | per X: `assert!(X.publishes_created_referent())` (fixture reach-guard), then `source_at(..,2) == None` | per-X reach-guard; A1 | **fails@base** for every X |
| A4 (copy publishers stay unpaired) | `[CopyTokenOf, P]`, `[Populate, P]`, `[Token, CopyTokenOf, P]` | all `None` | A5 positive | first two control; third **fails@base** |
| A5 (order) | `[Conjure, Token, P]`, `[CopyTokenOf, Token, P]` | `Some(1)` | — | control |
| A6 (walk-level, maintainer's `x_reference`/`plus_one_counters` consumers) | the A2 ability through `AbilityWalk::of(ability, collect_definition_effects(ability))` | `without_created_token_placements().len() == 3` | A1's walk drops 1 | **fails@base** |
| A7 (hostile: token-creating non-publisher) | `[Token, Incubate, P]`, `[Token, Mill, P]` (existing) | `Some(0)` | `!Incubate.publishes_created_referent()` asserted | control (proves the rule is "publisher", not "creates a token") |
| A8 (existing) | `else_branch_placement_is_not_paired_with_the_root_token`, `placements_in_another_mode_are_not_paired`, `non_token_publishers_and_other_targets_do_not_pair`, `filtering_view_drops_only_created_token_placements` | unchanged | — | control |
| A9 (suite) | `policies/tests/created_token_placement_parity.rs` and every consumer suite (`cast_facts`, `redundancy_avoidance`, `plus_one_counters`, `x_cast_gate`) | green | — | control |

Fixture construction for A3's heavier variants (`EachPlayerCopyChosen`, `CreateTokenCopyFromPool`, `EmpowerJace`, `Amass`, `Manifest`, `Cloak`, `CopyTokenBlockingAttacker`) may use the engine types' constructors or `serde_json::from_value` of the card-data shape; the per-fixture `publishes_created_referent()` assertion is what keeps the row non-vacuous. The fixture list is test data, not a second classification authority (the assertion is that each listed effect *is* classified as a publisher).

### U-A — runtime reference reading (`created_token_counter_sequence.rs`)

| ID | Scenario | Assert | Reach-guard | Status |
|---|---|---|---|---|
| R1 | Resolve `[Token 1/1 Soldier, Conjure{Named, Battlefield}, PutCounter{P1P1, 1, LastCreated}]` through `resolve_ability_chain` | the conjured object has one +1/+1 counter; the Soldier has none | both objects exist on the battlefield; the control chain `[Token, PutCounter{LastCreated}]` puts the counter on the Soldier | control (establishes D1.2's premise: the nearest publisher's product is the referent) |

If a faceless `Conjure{Named}` object cannot hold a counter, use a registry-backed conjure name (as `oracle_of_the_alpha_conjure_power_nine.rs` does) and record which.

### U-A — parser/game delegation

| ID | Claim | Evidence | Status |
|---|---|---|---|
| D1 | Parse output unchanged for every card | card-data per-card diff / `coverage-parse-diff`: **zero cards** (derivation below) | measurement |
| D2 | Declined-gate walk unchanged on printed cards | lead (`census_gate.py`): no gated chain has a newly admitted publisher followed by another node; covered by existing `effects/mod.rs` declined-gate tests staying green | control |
| D3 | Tokens-goaded continuation unchanged | existing goad suites (`goaded_creature_under_pacifism_visible.rs`, War Games/Rendmaw/Life of the Party parses) green; parse diff zero | control |

### U-C — guard (`game/effects/mod.rs` inline; through `resolve_ability_chain`, the production entry that evaluates `ability.condition`)

Fixture: a P0-controlled source; root `GainLife { amount: 3, player: Controller }`; `condition` set per row; subject supplied by `CostPaidObjectSnapshot::capture` on an object whose mana value is set per row (or by an object target for C6).

| ID | Condition | Subject | Assert | Reach-guard | Status |
|---|---|---|---|---|---|
| C1 (F3 negated, absent) | `Not(QQ(ObjectManaValue{CostPaidObject}, Even))` | none | no life gained | C2 in the same test gains | **fails@base** (`Not` of the arm's `false` → gain) |
| C2 (negated, present) | same | MV 3 | gains 3 | `resolve_quantity_for_ability_condition(..)==3` | control |
| C3 (negated, present, quality holds) | same | MV 2 | no gain | — | control |
| C4 (positive, present / absent) | `QQ(.., Even)` | MV 2 → gain; none → no gain | — | — | control (existing arm guard) |
| C5 (QuantityCheck negated, absent) | `Not(QuantityCheck{ObjectManaValue{CostPaidObject} GE 1})` | none | no gain | C5b | **fails@base** |
| C5b (value 0 ≠ absent) | same | MV 0 | gains (0 ≥ 1 false → Not true) | — | control — fails if a guard keys on value 0 instead of presence |
| C6 (other scope) | `Not(QuantityCheck{Power{Target} GE 3})` | no object target → no gain; target with power 2 → gain | — | positive leg | absent leg **fails@base** |
| C7 (multi-authority) | `Not(QQ(.., Even))` | `cost_paid_object` none, `effect_context_object` MV 3 | gains | — | control — the presence ladder's fallback keeps the guard quiet |
| C8 (revealed arm) | `issue_2871_currency_converter.rs` (5 tests) | — | green | — | control |

Evaluate through `resolve_ability_chain(.., 0)` and read P0's life; the direct `evaluate_condition` form is acceptable for one row only if the executor shows the chain form cannot stage that subject, and says so.

### U-E

| ID | Scenario | Assert | Reach-guard | Status |
|---|---|---|---|---|
| E1 (Wild Hypothesis, D1.6) | Real cast, `with_mana_pool(G×3)`, X=2, library top `Card A/B/C`; `cast(spell).x(2).commit()`, then `resolve_top()` | at `WaitingFor::SurveilChoice` (2 cards): one Fractal on the battlefield with 2 +1/+1 counters; then `act(SelectCards { cards: vec![] })` → both to graveyard; final: Fractal 2/2 with 2 counters, P0 graveyard = 3 (2 surveilled + the sorcery), library = 1 | `matches!(waiting_for, SurveilChoice { cards, .. } if cards.len() == 2)` | guard; derived green at base (`Token → PutCounter{X, LastCreated} → Surveil(2, SequentialSibling)`, no `player_scope`); if red at base → pre-existing-report route (charter D1.6) |
| E1b (X=0 sibling) | same, X=0 | the surveil prompt still arrives and resolves; after resolution the 0/0 Fractal is gone (CR 704.5f at the CR 704.3 check, then CR 111.7) and graveyard = 3 cards | E1 | guard: Surveil is independent of the placement and of the token's survival |
| E2 (C2) | renamed test | assertions unchanged, passes | — | non-discriminating by construction (rename) |
| E3 (C4) | `positive_count_gate_qty` | explicit arm present; existing `positive_count_gate_qty_binds_only_positive_gates` green; parse diff zero | — | non-discriminating by construction (`EventTime` already fell to `_ => None`); a lock-in assertion would belong in `oracle_trigger_tests.rs`, outside the scope rule — not added |

### V-AI

`cargo ai-gate`: expected no delta. If any, return it with the paired-seed report; do not refresh baselines silently.

### DEFERRED rows

- `DEFERRED(phase 2)`: the driver's union publication keyed on `Effect::is_token_creator` (War Games, Rendmaw, Life of the Party; sync and paused).
- `DEFERRED(phase 3)`: Oversimplify two-action order, per-seat binding, both Fractals at the first replacement prompt; the rewritten splitter unit test.
- `DEFERRED(phase 4)`: coverage condition-quantity honesty.

## Expected parse diff (derived card by card)

Mechanism: the parse can change only where a parser site consults a predicate whose membership changed. `is_token_creator` membership is unchanged → zero from `resolve_populated_token_anaphors`, the token-cleanup gate, `publishes_tracked_set_from_resolution`, `classify_bare_card_aggregate_publisher`, the goaded arm. `publishes_created_referent` gains eight variants; its parser sites (`chain_prior_referent_is_created_token`, `bare_they_player_declaration`, `relink_gated_token_referent_consumers`, `clone_would_transplant_gated_referent`) all ask about a publisher that **precedes** a later consumer clause. So only cards where a newly admitted publisher precedes another clause in the same chain can move. Lead (`census_members.py`): Investigate 143 cards / 13 with a following clause; Amass 71 / 10; EmpowerJace 37 / 1; CopyTokenBlockingAttacker 1 / 1; Encore 26 / 0; Myriad 23 / 0; EachPlayerCopyChosen 2 / 0; CreateTokenCopyFromPool 0. Every card with a following clause:

| Card | Clause after the member (Oracle) | Site that could see the member | Derived reading |
|---|---|---|---|
| Assault on Osgiliath | "then Goblins and Orcs you control gain double strike and haste" | none (typed group, no anaphor) | unchanged |
| Azog, Moria's Ruin | "If you controlled that creature, draw a card." | condition parse (`HasObjectTarget ∧ TargetMatchesFilter`, target-based) | unchanged — the one row resting on the condition parser not consulting the anaphor walk; the parse diff confirms |
| Foray of Orcs | "When you do, ~ deals X damage to target creature …, where X is the amassed Army's power" | `relink_gated…`: publisher = Amass, which carries no gate → `continue`; typed target | unchanged |
| Goblin Plate Mail | "then attach this Equipment to the amassed Army" | none (`AmassedArmy` referent) | unchanged |
| Grishnákh, Brash Instigator | "gain control of target … Untap that creature. It gains haste" | anaphor walk re-anchors on the typed target before reaching Amass | unchanged |
| Lazotep Plating | "You and permanents you control gain hexproof" | none | unchanged |
| Sauron, Lord of the Rings | "mill five cards, then return a creature card from your graveyard to the battlefield" | none | unchanged |
| Summons of Saruman | "Mill X cards. You may cast … from among them" | bare-card aggregate (Mill nearest; token-creator set unchanged) | unchanged |
| Surrounded by Orcs | "then target player mills X cards, where X is the amassed Army's power" | none | unchanged |
| Tidings of War | "If this spell was cast from a graveyard, amass Goblins 3 instead." | none | unchanged |
| Protégé's Awakening | "Draw a card." | none | unchanged |
| Mirror Match | "Exile those tokens at end of combat." | bespoke IR, already `LastCreated`; `relink`: the publisher is ungated → `continue` | unchanged |
| Ate-o'-clock | "Regenerate target creature." | typed target | unchanged |
| Confront the Unknown | "then target creature gets +1/+1 … for each Clue you control" | typed target | unchanged |
| Disorder in the Court | "Return the exiled cards to the battlefield tapped …" | tracked-set scan (token-creator set, unchanged) | unchanged |
| Elemental, My Dear | "then create an 0/0 … token. Put +1/+1 counters on that token …" | the nearest publisher is the later `Token` | unchanged |
| Eliminate the Impossible | "Creatures your opponents control get -2/-0 … If any of them are suspected, they're no longer suspected." | `bare_they` needs a player declaration — none | unchanged |
| Innocuous Researcher | "Then each player draws a card." | none | unchanged |
| No Witnesses | "Then destroy all creatures." | none | unchanged |
| Ongoing Investigation | "You gain 2 life." | none | unchanged |
| Panther Pounce | "Target creature gets +1/+0 and gains flying … Untap it." | anaphor walk re-anchors on the typed target | unchanged |
| Secrets of the Key | "If this spell was cast from a graveyard, investigate twice instead." | none | unchanged |
| Serene Sleuth | "Then each creature you control is no longer goaded." | none | unchanged |
| Tail the Suspect | "You may play an additional land this turn." | none | unchanged |
| Wernog, Rider's Chaplain | "Each opponent who doesn't loses 1 life. You investigate X times …" | none (`player_actions_this_way` quantity) | unchanged |

C4 (`EventTime`) and the goaded-arm delegation change no output by construction. **Expected run-span contribution of phase 1: zero cards.** A non-empty diff returns to the phase plan step (the charter's derived-set rule).

## Reference Readings

| Row | Reference | Derived from Oracle + CR | Measured | Verdict |
|---|---|---|---|---|
| E1 | Wild Hypothesis today | CR 608.2c: create the 0/0 Fractal, put X=2 counters on it, then surveil 2 as a separate instruction (CR 701.25a); both looked-at cards may go to the graveyard; the sorcery goes to the graveyard after resolving → graveyard 3, Fractal 2/2 | plan-r1 head probe at `dfc1a50` (Fractal had 2 counters at the `SurveilChoice` prompt, graveyard 3 after); not re-run at `8949699` (`dfc1a50..8949699` touches none of these paths) | agrees; E1 re-measures it at base |
| R1 | Runtime `LastCreated` after `[Token, Conjure, PutCounter]` | CR 608.2c: "put a counter on it" after "conjure X" refers to the conjured card | `conjure::resolve` assigns the slot; `perpetual_last_created_empty_no_source_fallback.rs::nonzero_conjure_still_binds_the_last_created_perpetual_grant` | agrees |
| A1/A5/A7 | Existing pairing (`Some(..)` rows) | the pairing's own contract (parity with `Token { enter_with_counters }`) | existing tests | agrees |
| D1/D3 | "parse unchanged" | derivation table above | executor's parse diff | pending measurement |
| C8 | Revealed-type guard | issue #2871 rows | existing integration tests | agrees |

No defective reference was found.

## Maintainer-simulation matrix

| Maintainer point (cited site) | What they will check | Plan answer | Rows |
|---|---|---|---|
| F4: `ability_chain.rs:114-123` searches back for any `Token` | `[Token, Conjure, PutCounter{LastCreated}]` does not pair | nearest publisher via the engine predicate; pair only if `Token` | A2, A6 |
| F4: "engine-owned publisher classification" (`conjure.rs:275-279`, `x_reference.rs:334-339`, `ability_chain.rs:178-183`) | one engine authority; no list in phase-ai or parser | `Effect::publishes_created_referent` (wildcard-free) in `types/ability.rs`; parser and game layer delegate; phase-ai consumes | P1–P3, step 2–4 |
| F4 controls: ordinary and intervening-Conjure | both present | A1 / A2, plus every other publisher (A3) | A1, A2, A3 |
| F4 (implicit): the publisher list is complete | no missed writer | measured writer table; 6 runtime writers the parser list missed are admitted; exhaustive match | D1.1 table, A3 |
| F3: `Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even))` with no subject (`effects/mod.rs:20609-20616, 21072-21076, 20114-20126`) | not vacuously true; via `quantity_expr_missing_resolution_only_referent` | guard arms for `QuantityQuality` and `QuantityCheck` | C1, C5, C6 |
| F3 controls: positive, negated, absent, present | all four | C1 (negated absent), C2/C3 (negated present), C4 (positive present/absent) | C1–C4 |
| F3 (implicit): CR citation | CR 608.2c on the PR-added arm | step 8 | — |
| C2 (CodeRabbit; no maintainer stance) | name says what it tests | rename + production pointer | E2 |
| C4: "Blind recursion is not justified" | no descent | explicit `EventTime => None` with CR 603.4/603.8 reason | E3 |
| C3: "predates this change. Nonblocking." | — | outside the run (charter) | — |
| C5: "Nonblocking limitation" | — | outside the run (charter) | — |
| Parse evidence: Wild Hypothesis "structural reparenting" | reparented Surveil still resolves | runtime guard | E1, E1b |
| Fresh parse evidence for the head | no unintended parse change | zero-card parse diff | D1 |
| AI receipts pending | ai-gate | V-AI | V-AI |

## Per-unit acceptance rows

- **U-A accepted when:** A2, A3 (every listed X), the third A4 row and A6 fail at `PHASE_BASE_SHA` (or, where the base cannot compile the new test, the base reading is shown by an equivalent base-run assertion) and pass at the candidate; A1, A5, A7–A9, P1–P3, R1 pass; the parse diff is zero cards; `rg -n "Effect::Token \{ \.\. \}" crates/phase-ai/src/ability_chain.rs` shows only D1.2's `Token` check; no publisher or token-creator list remains in `lower.rs` or the goaded arm; `effects/mod.rs` no longer imports the parser predicate; `types/zones.rs` names the engine method; clippy and `cargo test -p phase-ai --lib` green; `cargo ai-gate` shows no unexplained delta.
- **U-C accepted when:** C1, C5 and C6's absent leg fail at base and pass at the candidate; C2–C4, C5b, C7, C8 pass; the guard's doc records the `And`/`Or` limitation; the PR-added arm and test comment cite CR 608.2c.
- **U-E accepted when:** E1 and E1b pass (if red at base: pre-existing-report route, recorded); the C2 test carries its new name and production pointer; the C4 arm and doc line are present; the existing `positive_count_gate_qty_binds_only_positive_gates` is green.

## Unprobed (executor closes before relying on them)

1. Every **fails@base** claim (derived from the base code). Measure by running the new tests against the base implementation where they compile, or by asserting the base reading directly.
2. Parse diff = zero cards (D1).
3. AI gate = no delta (V-AI).
4. The compile form of the goaded-arm guard (step 3).
5. E1's prompt stop: that `commit()` then `resolve_top()` halts at `SurveilChoice`, and that `SelectCards { cards: vec![] }` sends both cards to the graveyard (per the handler: the payload is the keep-on-top set).
6. R1's conjured object accepting a counter (fallback named under R1).
7. The disk budget for the cold builds and `cargo ai-gate` (`server-release`).

## Notes for later phases and the integration review (not findings)

- Phase 2 keys its union on `Effect::is_token_creator()`. `EachPlayerCopyChosen` is not a member and never enters the per-seat loop (it is excepted from the player-scope fan-out in `resolve_ability_chain`).
- Pre-existing, outside phase 1's scope: the doc on `oracle_effect/mod.rs::chain_prior_referent_is_created_token` still lists only `Token`/`CopyTokenOf`/`Populate` as the referent producers (stale since Conjure/Manifest joined; `mod.rs` is not in phase 1's scope rule). Report route: fix in a phase that edits that file, or file.
- `Amass`/`EmpowerJace` write the slot only when they create; their own referent is modelled separately (`AmassedArmy`). No printed card reads `LastCreated` after them (derivation table). No action.
- Considered, not done: admitting `CopyTokenBlockingAttacker` to the token-creator set (it meets both D1.1 conditions; no printed card needs it).
