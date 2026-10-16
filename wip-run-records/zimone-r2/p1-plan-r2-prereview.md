# Phase 1 plan r2: run zimone-r2 (engine-owned created-referent authority, F3 guard, minor follow-ups)

Mode: `engine-planner` phase-plan mode. Phase index 1 of 4. `PHASE_BASE_SHA` = run `BASE_SHA` = `8949699a13a541a7e00ada2bb8e16c00c530b82a` (branch `card/zimone-all-questioning`, PR phase-rs/phase#10004). Units: U-A (F4), U-C (F3), U-E (C2, C4, Wild Hypothesis guard). The plan carries out charter decisions D1.1–D1.6 as written. This revision resolves review r1's `behavior` findings F-1 (publisher membership) and F-2 (one wildcard-free classification) and carries its `text` findings F-3..F-5. It is a complete plan, not a delta.

**Evidence discipline.** I produced this plan read-only. No cargo build ran: the box has about 9.3 GB free on `/`, Tilt is not running, and the instructions forbid building in the main checkout. I read every code claim at `8949699`. Corpus claims come from static Python censuses over `data/card-data.json`, which was generated at 17:08Z from the PR head before `BASE` merged main; `dfc1a50..8949699` touches only loyalty and static-parser files, none of the sites below. Corpus claims are labelled **lead (static census)**, and each one names the command that regenerates it. Each census prints a positive control.

Census scripts (run with `python3 -I`), in `/tmp/claude-0/-home-user-phase-personal/3ffacad5-7db0-5e02-a1a7-192aa01f873f/scratchpad/p1r2/`:
- `sweep.py`: the resolver sweep.
- `payload_variants.py`: the type-level payload carriers.
- `wrappers.py`: wrapper nodes and the nodes that follow them.
- `followers.py`: the nodes that follow a given member.
- `census_ai2.py`: AI pairing.
- `census_gate2.py`: the declined-gate walk.
- `relink_census.py`: `LastCreated` readers.
- `dump.py`: parse plus Oracle text for named cards.

The r1 scripts in `…/scratchpad/p1/` are superseded where they hard-code the member set.

**CR numbers cited in this plan**, each grepped in `docs/MagicCompRules.txt` (line number in parentheses):

| Range | Rules |
|---|---|
| 100–499 | 101.4 (235), 111.1 (645), 111.2 (647), 111.7 (661), 111.10f (679), 115.10a (890), 122.1 (1180), 122.6 (1210), 400.7 (1955) |
| 600–699 | 603.3 (2588), 603.4 (2598), 603.7a (2618), 603.8 (2637), 603.12 (2662), 605.1a (2689), 608.2c (2799), 608.2d (2801), 608.2e (2804), 608.2f (2806), 608.2h (2812), 609.3 (2853), 616.1 (3179) |
| 700–700.x | 700.1 (3206), 700.3 (3229) |
| 701.x | 701.15a (3400), 701.15b (3402), 701.16a (3410), 701.25a (3516), 701.36a (3610), 701.38a (3624), 701.40a (3638), 701.47a (3697), 701.55a (3765), 701.57c (3783), 701.58a (3787), 701.62a (3827), 701.63a (3833) |
| 702.x | 702.116a (4859), 702.141a (5053) |
| 704–707 | 704.3 (5506), 704.5f (5523), 705.2 (5587), 706.2 (5599), 707.2 (5633) |

Conjure, Draft-from-spellbook and Empower Jace are digital-only and have no CR entry, so their arms carry no CR number.

---

## Step 0 — Premise verification (Oracle text, `data/mtgjson/AtomicCards.json`)

| Card | Verbatim clause the phase depends on | Used by |
|---|---|---|
| Wild Hypothesis {X}{G} | "Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it.\nSurveil 2. (…)" | E1, E1b |
| Oversimplify | "Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it …" | seam only: D1.2 must still pair it (phases 2–3) |
| Mirror Match | "For each creature attacking you or a planeswalker you control, create a token that's a copy of that creature and that's blocking that creature. Exile those tokens at end of combat." | D1.1 derivation |
| Panther Pounce | "Target player investigates. Target creature gets +1/+0 and gains flying until end of turn. Untap it." | parse-diff derivation |
| Disorder in the Court | "Exile X target creatures, then investigate X times. Return the exiled cards to the battlefield tapped …" | token-creator non-widening reason |
| Elemental, My Dear | "Investigate X times, then create an 0/0 Blue Elemental creature token. Put +1/+1 counters on that token equal to the number of Clues you control." | parse-diff derivation, A-rows |
| Zimone, All-Questioning | "… create Primo, the Indivisible, … then put that many +1/+1 counters on it." | C2 doc pointer (`resolution_recheck_and_live_count`) |
| Blood Age Muster | "Whenever one or more cards leave your graveyard, conjure a random card from Blood Age Muster's spellbook onto the battlefield. Its base power and toughness perpetually become 2/2. This ability triggers only once each turn." | DraftFromSpellbook parse derivation |
| Brood Astronomer | "When this creature enters, you may sacrifice a land. If you do, draft a card from the Planets Spellbook and put it onto the battlefield tapped." | same |
| Saint Elenda | "When Saint Elenda enters, conjure a card of your choice from Saint Elenda's spellbook into your hand. You may cast that card this turn without paying its mana cost." | same |
| Tome of the Infinite | "{U}, {T}: Conjure a random card from Tome of the Infinite's spellbook into your hand. It perpetually gains "You may spend mana as though it were mana of any color to cast this spell."" | same |
| Hinterland Chef | "When Hinterland Chef enters the battlefield, draft a card from Hinterland Chef's spellbook. It perpetually becomes a Food artifact in addition to its other types and perpetually gains "{2}, {T}, Sacrifice this creature: You gain 3 life."" | same |
| Ominous Traveler | "When Ominous Traveler enters the battlefield, draft a card from Ominous Traveler's spellbook. That card perpetually gains "You may spend mana as though it were mana of any color to cast this spell" and "When you cast this spell, return a creature named Ominous Traveler you control to its owner's hand."" | same |
| Support Skyforge | "Whenever this Vehicle attacks, draft a card from Support Skyforge's spellbook. That card perpetually becomes an artifact creature." | same |
| Adaptive Armorer / Masterpiece Vault | "… draft a card from [~]'s spellbook and put it onto the battlefield. When an Equipment enters this way, attach it to [up to one] target creature you control." | same |
| Dazzling Flameweaver | "… conjure a random card from Dazzling Flameweaver's spellbook into exile. You may play that card until the end of your next turn." | same |
| Kayla's Kindling | "At the beginning of your upkeep, draft a card from Kayla's Kindling's spellbook and exile it. Until end of turn, you may cast that card." | same |
| Oyaminartok, Polar Werebear | "{U}, Sacrifice a Food: Draft a card from Oyaminartok's spellbook. When you do, add {U}{U}{U}. …" | same |
| Skywriter Djinn | "… conjure a random card from Skywriter Djinn's spellbook into your hand. Then if you have fewer cards in hand than the number of basic land types among lands you control, repeat this process." | same |
| Sliver Weftwinder | "… "When this creature enters, conjure a random card from the Slivers Spellbook into the top five cards of your library at random, then draw a card."" | same |
| Done for the Day | "At the beginning of your end step, if you control an Employee, a Performer, or a Robot, you may get {TK} or create a Treasure token. If you control all three, you may put a sticker on a nonland permanent you own." | wrapper parse derivation |
| Bind to Secrecy (mode 2) | "Conjure a duplicate of target creature card in an opponent's graveyard into your hand. It perpetually gains "You may spend mana as though it were mana of any color to cast this spell."" | reference for Tome of the Infinite |
| Futurist Spellthief | "When Futurist Spellthief enters the battlefield, conjure a duplicate of target spell into your hand. That card perpetually gains "You may spend mana as though it were mana of any color to cast this spell."" | reference for Ominous Traveler |

**F3 and F4 have no printed member.** The maintainer's own examples are constructed: `Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even))` and `[Token, Conjure, PutCounter{LastCreated}]`. Lead (static census):
- `p1/census_not.py`: the only `AbilityCondition::Not(QuantityCheck|QuantityQuality)` in the corpus is Ravenous Necrotitan, and its operand reads no resolution-only scope.
- `p1r2/census_ai2.py`: no corpus chain has a publisher between a `Token` and a `PutCounter{LastCreated}`.

Both fixes are building-block fixes. Their tests are constructed by necessity and drive production functions.

## Step 1 — Applicable skills and checklist skeleton

| Skill | Applies to | Checklist disposition in this phase |
|---|---|---|
| `/casting-stack-conditions` (§Sub-Ability Chain Resolution, §Condition Systems, §Common Pitfalls) | U-C: `evaluate_condition`'s `Not` arm, reached from `resolve_ability_chain`'s top-level condition gate | The condition is evaluated at resolution (CR 608.2h), unchanged. No new `WaitingFor`, no `PendingCast`/`StackEntry` field, no serde change, no intervening-if change. |
| `/oracle-parser` §1 rules, §9a Phase 6, Self-Review | U-A delegation in `lower.rs` / `sequence.rs`; U-E's C4 arm in `oracle_trigger.rs` | §9a Phases 1–5 (new pattern, subject, chain composition, routing) are N/A: no Oracle text is newly matched and no combinator is added. Phase 6: a delegation needs no parser unit test. The verification is the parse diff, derived card by card below, plus the combinator gate. |
| `/card-test` | E1/E1b (Wild Hypothesis real cast); R1–R4 runtime chain rows | E1/E1b use `GameScenario` + `GameRunner::cast(..).x(..)`, verbatim Oracle text, and assertions via runner accessors. The prompt stop is driven manually, because the default driver auto-answers `SurveilChoice`. R-rows drive `engine::game::effects::resolve_ability_chain` on a scenario board, as R1 did in r1, and answer pauses with `GameRunner::act`. |
| `/add-engine-variant` | — | Not triggered: no variant is added to any existing engine enum. A new *private* enum type (`CreatedReferentPublisher`, see Rust Idioms) is the return type of a private method. It is not a variant on an engine enum and has no serialized or exported surface. |
| `/add-ai-feature-policy` | — | Not triggered: no `DeckFeatures` axis or policy is added, and `created_token_placement_source` only consumes the engine predicate. CLAUDE.md's `cargo ai-gate` rule applies (V-AI). |

Deferrals:
- `DEFERRED(phase 2)`: the player-scope driver's consumption of the token-creator predicate (D2.1).
- `DEFERRED(phase 2/3)`: every Oversimplify, War Games and Rendmaw behaviour row.
- `DEFERRED(phase 4)`: coverage.

## Step 2 — Analogous traces

1. **The parser's publisher authority, end to end** (the thing D1.1 moves).
   - Parser predicates: `parser/oracle_effect/lower.rs::is_token_creating_effect` and `publishes_chain_created_referent`.
   - Parser consumers:
     - `lower.rs`: `resolve_populated_token_anaphors` (nearest token creator), `relink_gated_token_referent_consumers`, `clone_would_transplant_gated_referent`, and the token-cleanup gate.
     - `oracle_effect/mod.rs`: `chain_prior_referent_is_created_token` (which feeds `ParseContext::token_created_in_chain`), `bare_they_player_declaration`, `publishes_tracked_set_from_resolution` and `classify_bare_card_aggregate_publisher`.
     - The `sequence.rs` tokens-goaded arm.
   - Readers of `token_created_in_chain` (they decide what a flag flip can change): `counter.rs::counter_anaphor_created_token_binding` / `chain_created_demonstrative_binding`, `imperative.rs::parse_attach_recipient`, `mod.rs::parse_perpetual_self_subject` ("it/they perpetually gain"), the for-each counter dispatch, `subject.rs`'s bare-"it" `AffectedObject` exclusion, and the conditional-sacrifice rebind.
   - Game consumer: `game/effects/mod.rs::declined_gate_surviving_instructions`, which imports the parser function.
   - AI consumer: `phase-ai/src/ability_chain.rs::created_token_placement_source`, which re-derives `Token` only.
   - Runtime referent: `GameState::last_created_token_ids`, read by `TargetFilter::LastCreated` (`game/targeting.rs`, `game/filter.rs`).
   - The parser list already misses every D1.1 row marked **no** in 'Today in parser set?'; see D1.1 membership.
2. **The engine-owned AST classification idiom.** Two parts:
   - `types/ability.rs::Effect::moves_card_to_or_from_library` is a node-only, static-AST predicate on `impl Effect` with a **wildcard-free** match. Its doc reads: "This method answers for THIS NODE ONLY; nested payloads are the walker's business (`ability_visit::visit_ability_def_scoped` under `ResolutionScope::OwnResolutionOnly`)".
   - Its composition with the walker is `game/mana_abilities.rs::chain_moves_card_to_or_from_library`. That is `visit_ability_def_scoped(def, OwnResolutionOnly, |e| e.moves_card_to_or_from_library())`.
   - The walker, `types/ability_visit.rs::visit_effect_scoped`, is the engine's single own-resolution boundary authority, and it is wildcard-free over `Effect`. Under `OwnResolutionOnly` it descends the inline payloads (`ChooseOneOf.branches`, `RollDie.results`, `FlipCoin`/`FlipCoins` win/lose, `FlipCoinUntilLose.win_effect`, `SeparateIntoPiles` piles, `Vote.per_choice_effect`/`outcome_template`, `RevealFromHand.on_decline`). It stops at the CR 603.3/603.7a/603.12/611.2/614.1 boundary carriers: `CreateDelayedTrigger`, `CreateBoon`, `GenericEffect`, `AddTargetReplacement`, `Counter`'s rider, `Token`'s statics, and the draw/planeswalk replacements.
   - U-A's classification copies this two-part shape exactly: a node-only wildcard-free match plus the walker.
3. **The existing absent-referent guard.**
   - `game/effects/mod.rs::subject_dependent_type_condition_has_no_subject` (issue #2871, `RevealedHasCardType`, recursing through `Not` only) is called by `evaluate_condition`'s `Not` arm.
   - The leaf test it needs already exists as `game/quantity.rs::quantity_expr_missing_resolution_only_referent(state, expr, ability)`. It is used by the `QuantityCheck`/`QuantityQuality` arms of `evaluate_condition` and by `condition_awaits_resolution_only_referent`.
   - Integration evidence for the revealed arm: `tests/integration/issue_2871_currency_converter.rs`.
4. **Wild Hypothesis runtime path.**
   - The `created_token_counter_sequence.rs` runtime rows (`moved_placement_keeps_its_x_count`, Fractal Summoning X=3) run through `game/scenario.rs`'s `SpellCast::commit` / `GameRunner::resolve_top` / `act`.
   - `WaitingFor::SurveilChoice` is handled in `engine_resolution_choices.rs`. Its `SelectCards { cards }` payload is the ordered keep-on-top set; every other looked-at card goes to the graveyard.
5. **Resume of a paused publisher** (the evidence for DraftFromSpellbook and Endure).
   - `effects/mod.rs`'s auto-stash list (the `matches!` that includes `WaitingFor::SpellbookDraft` and `WaitingFor::ChooseOneOfBranch`) parks the printed tail.
   - On `SubmitSpellbookDraft`, `spellbook::complete_draft` (which calls `conjure::resolve` and assigns the slot) runs before `finish_with_continuation` drains the tail.
   - On `ChooseBranch`, `choose_one_of::resolve_branch` resolves the chosen branch through `resolve_ability_chain` before the tail drains.

## Step 3 — Files read

- `types/ability.rs`: `impl Effect`, `moves_card_to_or_from_library`, the `Effect` enum (the payload fields of every wrapper), the `TriggerCondition::EventTime` doc, and the `AbilityCondition` helpers.
- `types/ability_visit.rs`: module doc, `ResolutionScope`, `scope_prunes_nested_ability`, `visit_ability_def_scoped`, and the `visit_effect_scoped` arms.
- `game/mana_abilities.rs` (`chain_moves_card_to_or_from_library`).
- `parser/oracle_effect/lower.rs`: `resolve_populated_token_anaphors`, the predicates, `relink_gated_token_referent_consumers`, `clone_would_transplant_gated_referent`, and the token-cleanup gate.
- `parser/oracle_effect/sequence.rs`: the tokens-goaded arm and its neighbours.
- `parser/oracle_effect/mod.rs`: `chain_prior_referent_is_created_token`, `chain_prior_player_declaration`, `bare_they_player_declaration`, `try_parse_perpetual_base_pt`, `parse_perpetual_self_subject`, the `ParseContext` seeding, the sacrifice rebind, and the `UNBOUND_SUBJECT_GAP` emission sites.
- `parser/oracle_effect/subject.rs` (bare-"it" binding), `counter.rs` (created-referent binding helpers), `imperative.rs` (`parse_attach_recipient`), and `zone_change_reflexive.rs` (`unsupported_zone_change_reflexive`).
- In `game/effects/`: `mod.rs`, specifically:
  - `resolve_effect` dispatch;
  - the auto-stash list;
  - `declined_gate_surviving_instructions`;
  - `subject_dependent_type_condition_has_no_subject`;
  - `evaluate_condition`'s `Not`/`QuantityCheck`/`QuantityQuality` arms;
  - the inline test `quantity_quality_ability_condition_evaluates_at_resolution`.
- Resolvers: `spellbook.rs` (whole), `spellbook_tests.rs` (fixtures), `conjure.rs` (slot assignment), `endure.rs` (whole), `choose_one_of.rs` (`resolve`, `prompt_next`, `resolve_branch`), `exile_from_top_until.rs` (tail driving), and `counters.rs` (`apply_pending_counter_post_action`, the writing arms). Also `choose_and_sacrifice_rest.rs` (its counter post-action) and every non-test writer of `last_created_token_ids` (`p1r2/sweep.py` seeds).
- `game/engine_resolution_choices.rs`: the `SpellbookDraft`, `ChooseOneOfBranch` and manifest-dread arms.
- `game/engine.rs::clear_frame_bookkeeping`, which is a loop-detection frame helper and not a resolution-time clear.
- `game/morph.rs` (face-down publish, 825–890) and `types/zones.rs` (`ChainReferentIntent` doc).
- `game/quantity.rs` (2160–2300).
- `phase-ai/src/ability_chain.rs` (whole), `phase-ai/src/policies/x_reference.rs` (263–360) and `phase-ai/src/features/plus_one_counters.rs` (215–245).
- `parser/oracle_trigger.rs` (`positive_count_gate_qty`) and `game/triggers.rs` (`quantity_quality_rechecked_at_resolution`).
- Integration tests:
  - `tests/integration/created_token_counter_sequence.rs` (whole);
  - `tests/integration/zimone_all_questioning.rs` (224–245);
  - `tests/integration/perpetual_last_created_empty_no_source_fallback.rs`.
- `game/scenario.rs`: the `GameRunner` API (`state_mut`, `act`, `spellbook_pick`).

## Step 3.5 — Probes

All probes were static, because no build was possible within the constraints. Each was probed by reading or by census, with a positive control.

- **Membership regenerator** (replaces r1's `rg`; this is the durable command). The script is `p1r2/sweep.py` and runs as `python3 -I sweep.py crates/engine/src`.
  - **Method.** For each `resolve_effect` dispatch arm, it runs a bounded breadth-first search over non-test functions in `game/effects/*.rs` (except `effects/mod.rs`) and `game/morph.rs`. The search never passes through the chain drivers. It reports three things:
    - **WRITE**: the arm reaches an assignment, push or clear of `last_created_token_ids`, or `record_last_created_(copy_batch_)token`, or `.publishing_chain_referent()`.
    - **SYNTH**: the arm synthesizes `ResolvedAbility::new(Effect::X…)` or `AbilityDefinition::new(…, Effect::X…)` and resolves it.
    - **PAYLOAD**: the arm reaches `resolve_ability_chain` / `resolve_effect`.
  - **Positive control.** `Token`, `CopyTokenOf`, `Populate`, `Conjure`, `Manifest`, `ManifestDread` and `Cloak` all report WRITE.
  - **Known limits.** Payloads resumed after a pause (`ChooseOneOf`, `Vote`, `SeparateIntoPiles`) are not reached statically. Those are closed by the type-level rule below.
  - **False positive, refuted by reading.** `ChooseAndSacrificeRest` reports WRITE through `counters::apply_pending_counter_post_action`. Its only post-action is `PendingCounterPostAction::ContinuePlayerScopeSacrifice`, which delegates to `continue_player_scope_sacrifice` and writes nothing. The writing post-action arms (`FinalizeTokenEntry`, `ContinueTokenCreation`, `ContinueCopyTokenCreation`, and kin) are built only by the token pipeline.
  - The regenerating rule for the inline-payload half is the walker's `OwnResolutionOnly` descend set. `p1r2/payload_variants.py types/ability.rs` lists the `Effect` variants that carry an `AbilityDefinition`: `Vote`, `SeparateIntoPiles`, `RevealFromHand`, `CreateDelayedTrigger`, `FlipCoin`, `FlipCoins`, `FlipCoinUntilLose`, `ChooseOneOf`, and `RollDie` (via `DieResultBranch`).
- **Wrapper census**, with `p1r2/wrappers.py card-data.json AtomicCards.json`.
  - **Positive control.** It prints the wrapper nodes with a nested own-resolution publisher, per variant.
  - **Reading.** It lists every one of those nodes that has a following node in its chain. Lead: only Done for the Day.
- **DraftFromSpellbook and Endure followers**, with `p1r2/followers.py card-data.json DraftFromSpellbook AtomicCards.json`, and the same for `Endure`.
  - **Positive control.** It prints the follower count per member.
  - **Reading.** Lead: Endure has no node with a follower.
- **Readers.** `p1r2/relink_census.py` lists every `LastCreated` reader whose nearest publisher is newly admitted. Its positive control prints the count of readers whose nearest publisher is in the base set. Lead: the only newly admitted case is Mirror Match, which r1 already derived.
- **AI pairing under the r2 classification** (`p1r2/census_ai2.py`). It covers every `PutCounter{LastCreated}` placement and its base-paired subset, the latter as positive control. Lead: **0** classification changes.
- **Declined-gate walk** (`p1r2/census_gate2.py <data> r2`). Its positive control is `… control`, which counts the `Token`-followed gated chains. Lead: **0** hits for the newly admitted members.

**Not probed** (each closes in the executor's first build; see §Unprobed):
- the compile form of the `sequence.rs` guard;
- that `types::ability` can call `ability_visit::visit_effect_scoped` (it is `pub(crate)`; `game/mana_abilities.rs` already does);
- the Wild Hypothesis prompt-stop sequence;
- the R-row pause and resume sequences;
- whether a faceless conjured object accepts a counter;
- every "fails at base" claim, which is derived from code, not run.

---

## Pattern Coverage

| Unit | Class | Members |
|---|---|---|
| U-A | The class has two parts. **First**, every chain in which a `PutCounter{target: LastCreated}` follows one or more effects whose own resolution assigns the `LastCreated` referent. The runtime referent is the **nearest** publisher's product, because each publisher *assigns* the slot. The AI reads the placement as part of a token's creation only when that publisher is `Effect::Token`. **Second**, every engine or parser site that asks: "does this effect publish the referent that a later `LastCreated` read names?", or "is it one of the token-creation effects?" | Every row of the D1.1 table: node-level members, plus the inline-payload wrappers whenever their own-resolution payload publishes. Lead (`census_ai2.py`): every base-paired corpus placement keeps its reading. The class rows cover every non-`Token` member interleaved. Charter class attribution: F4 (D1.1/D1.2), plus phase 2's union consumer. |
| U-C | A negated subject-dependent condition whose single resolution-only subject is absent: `Not(X)` with X ∈ {`RevealedHasCardType`, `QuantityCheck`, `QuantityQuality`}, over any `ObjectScope` that `quantity_expr_missing_resolution_only_referent` classifies. | No printed member today (lead). Building-block fix (charter F3). |
| U-E | C2 test honesty; C4's explicit no-descent arm; and a regression guard for the chain shape "create, put counters on it, then an independent instruction" (Wild Hypothesis), which phase 3's splitter change must not disturb. | Wild Hypothesis, plus every single-player Token → PutCounter{LastCreated} → independent tail. |

None of the units covers a single card. U-A covers every D1.1 row and every chain shape. U-C covers 3 condition shapes over every resolution-only scope.

## Sizing

**Units: 3** (U-A, U-C, U-E). A reviewer who splits U-E into C2 / C4 / Wild Hypothesis gets 5. T1 is true either way.

| Unit | Behaviour | Registration surfaces | Discriminating tests |
|---|---|---|---|
| U-A | One engine-owned, wildcard-free classification (`Effect::created_referent_publication`) from which both public predicates derive. The AI pairs only with the nearest publisher, and only when it is `Token`. The parser and the game layer delegate. | 2 `pub fn` + 1 private `fn` on `impl Effect`, and 1 private enum type (`CreatedReferentPublisher`). No variant on an existing enum, no serialized surface, no `WaitingFor`/`GameAction`. | A2, A3, the third A4 row, A6, A10a, A10c (all fail at base) |
| U-C | `Not` over an absent resolution-only subject is false, for `QuantityCheck` and `QuantityQuality`. The revealed arm is unchanged. | none (private fn) | C1, C5, C6 (fail at base) |
| U-E | C2 rename and doc; C4's explicit `EventTime => None`; the Wild Hypothesis guard | none | None by construction: C2 and C4 are behaviour-identical, and E1/E1b are guards that are green at base. |

**Dependency edges.** None inside the phase: U-A, U-C and U-E are independent. Outgoing: U-A → phase 2 (`Effect::is_token_creator` is the driver's union key, D2.1).

**Expected changed paths.** These use phase-fit counting. Integration tests count. No exclusion applies: there are no fixtures, no committed regenerated data and no protocol surface.

| # | Path | Unit | Kind |
|---|---|---|---|
| 1 | `crates/engine/src/types/ability.rs` | U-A | definite (classification, predicates, inline P-tests) |
| 2 | `crates/engine/src/parser/oracle_effect/lower.rs` | U-A | definite (delegators, doc move) |
| 3 | `crates/engine/src/parser/oracle_effect/sequence.rs` | U-A | definite (goaded-arm delegation only) |
| 4 | `crates/phase-ai/src/ability_chain.rs` | U-A | definite (including inline tests) |
| 5 | `crates/engine/src/game/effects/mod.rs` | U-A (declined-gate call, import removal), U-C | definite |
| 6 | `crates/engine/src/game/triggers.rs` | U-E (C2) | definite |
| 7 | `crates/engine/src/parser/oracle_trigger.rs` | U-E (C4) | definite |
| 8 | `crates/engine/tests/integration/created_token_counter_sequence.rs` | U-E (E1/E1b), U-A (R1–R4) | definite |
| 9 | `crates/engine/src/types/zones.rs` | U-A | comment-only (charter conditional; this plan edits it) |
| — | `crates/phase-ai/src/policies/x_reference.rs` | — | **not touched**: every walk-level row lives in `ability_chain.rs` |
| — | `crates/engine/src/types/ability_visit.rs` | — | **not touched**: called, not edited |

**Count.** 9; the worst case is 10, if a reviewer moves a walk row into `x_reference.rs`. T2 is false, so NOT(T1∧T2) holds and the phase stays single.

**Scope.** Every path is inside the charter's phase-1 scope rule. F-1's resolution needs no test file outside it: the runtime evidence (R2–R4) lives in `created_token_counter_sequence.rs`, which is an anchor path. The phase therefore needs no addendum and is no re-charter candidate.

There is no generated-artifact grouping: `ChainReferentIntent` has no TS binding (`rg -l ChainReferentIntent` finds Rust files only), and the new private enum is not exported.

## Building Blocks

- `types::ability_visit::{visit_effect_scoped, ResolutionScope::OwnResolutionOnly}`: the single own-resolution boundary authority, used as is. It decides which nested payloads belong to *this* resolution (CR 608.2c) and which are registered for a later one (CR 603.3, 603.7a, 603.12). The classification does not re-implement that list.
- `Effect::moves_card_to_or_from_library` together with `game/mana_abilities.rs::chain_moves_card_to_or_from_library`: the precedent for "node-only wildcard-free match + walker".
- `game::quantity::quantity_expr_missing_resolution_only_referent(state, expr, ability)`: the single absent-referent authority (the maintainer's named fix shape), used as is.
- phase-ai: `ability_chain::{collect_chain_effects, AbilityWalk, created_token_placement_source}`, and `features::plus_one_counters::ability_creates_token_with_plus_one_counters` (a production consumer the tests use).
- `phase-ai::policies::tests::created_token_placement_parity::{token_effect, last_created_placement, effect_chain}`: existing fixture builders.
- Engine runtime: `engine::game::effects::resolve_ability_chain`.
- Scenario API: `GameScenario` / `GameRunner` (`commit`, `resolve_top`, `act`, `state_mut`, `graveyard_count`, `counters`).
- Spellbook fixture shape: `spellbook_tests.rs` (`GameObject::spellbook` set on the source; the single-entry list makes the random draft deterministic).
- No new helper is introduced beyond the private classification method that F-2 mandates.

## Logic Placement

- **The classification and both predicates:** `types/ability.rs`, `impl Effect`. They are static-AST classifications consumed by the layers of three crates (parser, game, phase-ai), and `moves_card_to_or_from_library` sets the precedent. The parser keeps its names as delegators (D1.1), so no parser caller moves.
- **The own-resolution walk:** reused from `types/ability_visit.rs`, which is the same crate and the same layer.
- **`declined_gate_surviving_instructions`** (game layer) switches from the parser import to `node.effect.publishes_created_referent()`. The game layer should not depend on a parser function, and the charter permits this line-level switch.
- **AI:** `phase-ai/src/ability_chain.rs` consumes the engine predicate. It adds no classification of its own beyond D1.2's `Token` check.
- **U-C:** the condition evaluator in `game/effects/mod.rs`, which is the only `Not` evaluation site.
- **U-E:** test-only, plus one parser match arm.
- **Untouched:** frontend, WASM, server and protocol.

## Rust Idioms

- **One wildcard-free authority (F-2).** The authority is a private node-only classification:

  ```rust
  /// CR 608.2c: which kind of created-referent publisher THIS node's own resolver is.
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  enum CreatedReferentPublisher {
      /// CR 111.1 + CR 701.36a + CR 707.2: a token-creation effect whose own
      /// parameters define the token(s) — a spec, a copy source, the populated token.
      TokenCreator,
      /// Every other resolver that assigns the referent: face-down entry, conjure /
      /// spellbook draft, a keyword action or copy batch that creates as one step.
      OtherProducer,
  }

  fn created_referent_publication(&self) -> Option<CreatedReferentPublisher>
  ```

  - It is one exhaustive `match self` with **no `_ =>` arm**. The arms are: a `Some(TokenCreator)` arm, a `Some(OtherProducer)` arm, and one `None` arm listing every other variant, including the inline-payload wrappers.
  - The doc carries the precedent's sentence: "a new `Effect` variant is a compile error here … Do not add an `_ =>` arm".
  - The arm lists are the D1.1 table. The `None` arm is "every variant not in the two `Some` arms". The compiler enforces that set; it is not hand-copied from another list.
- **The two public predicates, derived from that one match:**
  - `pub fn is_token_creator(&self) -> bool { self.created_referent_publication() == Some(CreatedReferentPublisher::TokenCreator) }`. This is node-only. A wrapper is never a token creator, because its own node creates nothing (D1.1: the published object must be a token *it* creates).
  - `pub fn publishes_created_referent(&self) -> bool`. This is `visit_effect_scoped(self, ResolutionScope::OwnResolutionOnly, &mut |e| if e.created_referent_publication().is_some() { ControlFlow::Break(()) } else { ControlFlow::Continue(()) }).is_break()`.
    - The walker visits `self` first, so every node-level member answers immediately.
    - A wrapper answers through its own-resolution payload, nested wrappers included.
    - A boundary carrier (`CreateDelayedTrigger { effect: Token }`) answers `false`.
  - A new token-creating variant therefore cannot diverge: its author must choose `TokenCreator`, `OtherProducer` or `None` at the one site that both predicates read. A new payload carrier must also make the walker's descend-or-leaf decision (that match is wildcard-free too).
- **AI pairing:** `let p = chain[..index].iter().rposition(|e| e.publishes_created_referent())?;` then `matches!(chain[p], Effect::Token { .. }).then_some(p)`.
- **U-C:** two new match arms on `AbilityCondition`. The guard keeps its `_ => false`: it guards an enum of more than 100 variants whose default answer is "has a subject", and the existing function already has that shape. It is not a classification authority.
- No `bool` parameter and no public type is added.

## Nom Compliance (parser files change)

- `lower.rs`: two functions become one-line delegators. There is no dispatch and no text matching.
- `sequence.rs`: the tokens-goaded arm's effect-pattern guard `Effect::CopyTokenOf { .. } | Effect::Token { .. } | Effect::Populate` becomes a predicate call. The Oracle-text detection stays `try_parse_tokens_goaded_continuation` (nom `alt`/`tag`, unchanged). No `contains` or `starts_with` is added.
- `oracle_trigger.rs`: one `TriggerCondition` match arm, with no text.
- **Oracle text newly accepted with deferred semantics: none.** Coverage stays honest by construction. The expected parse diff, derived below, is zero cards.

## Extension vs Creation

- **Extends:**
  - the parser's publisher authority (moved, not re-created);
  - the `impl Effect` node classification idiom and its walker composition;
  - the existing no-subject guard (two arms);
  - the existing AI pairing function.
- **Creates:**
  - the two methods D1.1 mandates;
  - the private node classification and its private return enum that F-2 mandates. These are the single wildcard-free match both methods derive from. A `bool` pair cannot express three outcomes (token creator / other publisher / not a publisher) from one site.

## Analogous Trace

Two features were traced:
- **`moves_card_to_or_from_library`:** `types/ability.rs` (node-only wildcard-free match) → `types/ability_visit.rs` (`visit_ability_def_scoped`, `OwnResolutionOnly`) → `game/mana_abilities.rs::chain_moves_card_to_or_from_library` (the composition) → `is_mana_ability` (consumer).
- **The current parser publisher authority:** `parser/oracle_effect/lower.rs` → `parser/oracle_effect/mod.rs` → `parser/oracle_effect/{counter,subject,imperative}.rs` (the flag readers) → `game/effects/mod.rs` (declined gate) → `phase-ai/src/ability_chain.rs` → runtime `GameState::last_created_token_ids` (`game/effects/{token,token_copy,conjure,spellbook,endure,choose_one_of}.rs`, `game/morph.rs`).

## Variant Discoverability

No variant on an existing engine enum is proposed, so `/add-engine-variant` is not triggered. `rg -n "fn (creates_tokens?|is_token_creat\w*|publishes_\w*referent\w*|created_referent\w*)\b" crates/` finds only the two parser functions, so the new methods duplicate nothing. `rg -n "enum CreatedReferent" crates/` finds nothing, so the private enum duplicates no type. `data/engine-inventory.json` is not needed for a method addition.

## Identity / Provenance Contract

**Concept 1: "it" / "that token" after a publisher (CR 608.2c).**
- **Authority and storage.** `GameState::last_created_token_ids`, **assigned** by the nearest publisher's resolver. For a wrapper, it is assigned by the chosen payload's publisher (CR 608.2c, 608.2d: the payload is followed inside this resolution).
- **Binding time.** At each publisher's resolution. For a paused publisher (spellbook choice, endure, choose-one-of), at the resume, before the stashed tail drains.
- **Live vs snapshot.** A snapshot of ids. Each later publisher overwrites it. A wrapper whose chosen payload publishes nothing leaves the earlier value in place, and so does an Amass that finds an Army.
- **Consumers.** `TargetFilter::LastCreated` (`targeting.rs`, `filter.rs`). Statically, `created_token_placement_source`.
- **Invalidation.** The next publisher's assignment.
- **Multi-authority hostile fixtures:**
  - A2 `[Token, Conjure, P]`: two publishers; the nearest wins.
  - A5 `[Conjure, Token, P]`: the order reversed.
  - A7 `[Token, Incubate, P]`: an effect that creates a token but does **not** write the slot.
  - A10a/A10b: a wrapper with and without a publishing payload.
  - A10c: a wrapper nested inside a wrapper.
  - A10d: a boundary carrier.
  - R1–R4: runtime.

**Concept 2: the subject of a negated subject-dependent condition (CR 608.2c).**
- **Authority and storage.** `ResolvedAbility` referents (`cost_paid_object`, `effect_context_object`, `targets`, the trigger event), read through `resolution_only_scope_referent_present`.
- **Binding time.** At resolution (CR 608.2h).
- **Live vs snapshot.** Live.
- **Consumer.** `evaluate_condition`'s `Not` arm, via the guard.
- **Invalidation.** n/a.
- **Multi-authority hostile fixture.** C7: `cost_paid_object` absent but `effect_context_object` present.

---

## Architecture decisions carried out (phase-plan detail)

### D1.1 — membership, measured resolver by resolver

**Criterion** (charter): the effect's resolver assigns the referent that `TargetFilter::LastCreated` reads. In this plan, "the effect's resolver" means everything the effect's own resolution runs. That is the node's resolver, plus any `AbilityDefinition` payload that the resolver resolves inline in the same resolution (CR 608.2c, 608.2d). It excludes a payload registered to resolve separately (CR 603.3, 603.7a, 603.12). That boundary is the one `ResolutionScope::OwnResolutionOnly` already encodes, so the inline-payload wrappers are decided by option **(a)**: from their own typed payload, through that walker.

**Why (a) and not (b):**
- What the resolver writes: each wrapper resolves its chosen payload inline with `resolve_ability_chain`, and a publisher in the payload assigns the slot before the wrapper's continuation runs.
  - `choose_one_of.rs::resolve_branch` appends the continuation after the branch and resolves both in one chain.
  - `roll_die`, `flip_coin` and `reveal_from_hand` resolve the payload inside their own resolver.
  - The `SeparateIntoPiles` and `Vote` payloads resolve when their choice completes, before the stashed tail drains.

  Under (b), `[Token, RollDie{…Token…}, PutCounter{LastCreated}]` would pair with the outer `Token` while the runtime referent is the die result's token, which is the F4 defect shape.
- (b) would also narrow the charter's criterion, which (a) keeps as written.
- Corpus cost of (a): the wrapper census finds one wrapper with a following node (Done for the Day), and its follower reads no created referent (parse-diff table).

| Effect | Resolver evidence (symbol) | CR | Today in parser set? | Node classification | Publishes | Token creator |
|---|---|---|---|---|---|---|
| `Token` | `token::resolve` / `apply_create_token_after_replacement_with_created_ids` / `record_last_created_token` | 111.1 | yes (both) | `TokenCreator` | yes | yes |
| `CopyTokenOf` | `token_copy::resolve` (clears on an empty source) / `drive_copy_token_batches` completion / `engine_replacement::handle_copy_target_choice` | 707.2, 111.1 | yes (both) | `TokenCreator` | yes | yes |
| `Populate` | `populate::resolve` → `token_copy::resolve` | 701.36a | yes (both) | `TokenCreator` | yes | yes |
| `Manifest`, `ManifestDread`, `Cloak` | `morph::begin_face_down_referent_production` (clear) + `.publishing_chain_referent()` → `publish_face_down_entry_referent` (also the manifest-dread choice arm in `engine_resolution_choices.rs`) | 701.40a, 701.62a, 701.58a | publisher | `OtherProducer` | yes | no (not tokens) |
| `Conjure` | `conjure::resolve` (`last_created_token_ids = created_ids`) | — | publisher | `OtherProducer` | yes | no (not tokens) |
| `DraftFromSpellbook` | Random path: `spellbook::resolve` → `complete_draft` → `conjure::resolve`. Choice path: `WaitingFor::SpellbookDraft` (auto-stashed tail) → `SubmitSpellbookDraft` arm → `complete_draft`, before `finish_with_continuation`. Grouped with `Conjure` (as `moves_card_to_or_from_library` groups them). | — | **no** | `OtherProducer` | **yes** | no (conjured cards are not tokens) |
| `Endure` | `endure::resolve` builds an `Effect::Token` Spirit branch → `choose_one_of::prompt_next` (auto-stashed tail) → `ChooseBranch` → `resolve_branch` → `resolve_ability_chain` → `token::resolve`. It writes only on the token branch, the conditional-write shape of `Amass`. | 701.63a | **no** | `OtherProducer` | **yes** | no (one step of a larger action) |
| `CopyTokenBlockingAttacker` | `copy_token_blocking::resolve` republishes `all_created` | 707.2 | **no** | `OtherProducer` | **yes** | no (see below) |
| `EachPlayerCopyChosen` | an inner `CopyTokenOf` per player (`each_player_copy_chosen` → `resolve_ability_chain`) | 707.2 | **no** | `OtherProducer` | **yes** | no |
| `Investigate` | `investigate::resolve` → `token::resolve` (Clue) | 701.16a, 111.10f | **no** | `OtherProducer` | **yes** | no |
| `Amass` | `amass::resolve` → `apply_create_token_after_replacement`, only when there is no Army | 701.47a | **no** | `OtherProducer` | **yes** | no |
| `EmpowerJace` | `empower_jace::resolve` → `apply_create_token_after_replacement`, only when there is no Jace | — | **no** | `OtherProducer` | **yes** | no |
| `Myriad` | `myriad::resolve` → `token_copy::resolve` per opponent | 702.116a | **no** | `OtherProducer` | **yes** | no |
| `Encore` | `encore::resolve` → `token_copy::resolve` per opponent | 702.141a | **no** | `OtherProducer` | **yes** | no |
| `CreateTokenCopyFromPool` | `drive_copy_token_batches` / `token::no_token` | 707.2 | **no** | `OtherProducer` | **yes** | no |
| `ChooseOneOf` | `choose_one_of::resolve_branch` → `resolve_ability_chain(branch + continuation)` | 608.2d, 701.55a | **no** | `None` (a wrapper) | when a branch's own-resolution chain publishes | no |
| `RollDie` | `roll_die::execute_roll` → a result branch, via `resolve_ability_chain` | 706.2 | **no** | `None` (a wrapper) | when a result's chain publishes | no |
| `FlipCoin`, `FlipCoins`, `FlipCoinUntilLose` | `flip_coin::run_flip_branch` → `resolve_ability_chain` | 705.2 | **no** | `None` (a wrapper) | when a win/lose chain publishes | no |
| `SeparateIntoPiles` | the chosen/unchosen pile effect, resolved by `separate_piles` at the pile choice | 700.3 | **no** | `None` (a wrapper) | when a pile effect's chain publishes | no |
| `Vote` | `per_choice_effect` / `outcome_template`, resolved when the vote completes | 701.38a | **no** | `None` (a wrapper) | when a vote outcome's chain publishes | no |
| `RevealFromHand` | `on_decline` via `reveal_from_hand::run_on_decline_now` → `resolve_ability_chain` | 608.2c | **no** | `None` (a wrapper) | when `on_decline`'s chain publishes | no |

**Measured non-members**, named with the reason:
- `Incubate` and `GiftDelivery` create tokens but call no writer (`incubate.rs`, `gift_delivery.rs`; Incubate's pause post-action writes nothing). Incubate is fixture A7.
- `Meld` and `ChangeZone` (including moves to the battlefield): no write. `ChainReferentIntent::Publishes` is set only by the manifest family.
- `ChooseAndSacrificeRest`: the sweep's false positive, refuted in §Step 3.5.
- `CreateDelayedTrigger`, `CreateBoon` (CR 603.7a, 603.3), `EpicCopy`, and `CastFromZone` / `Discover` / `Cascade` (a cast spell resolves separately): each payload resolves in a later resolution, and the walker's boundary already stops there (fixture A10d).
- `ExileFromTopUntil`: it drives its own chain *tail*, which the chain scan classifies node by node; it carries no nested payload.
- `Forage`, `Learn`, `ExploreAll`, `ApplySticker`, `PutSticker`, `Bolster`: they synthesize only non-publishers (`Sacrifice`, `ChangeZone`, `SearchOutsideGame`, `Explore`, `NoOp`, `PutCounter`).

**Commands that regenerate this table:** `p1r2/sweep.py` (node half) and `p1r2/payload_variants.py` (payload half). §Step 3.5 gives each with its positive control.

**The publisher doc** states the criterion as "this effect's own resolution writes the slot on at least one resolution path; an inline payload counts, a payload registered for a later resolution does not". It adds a known-limitation sentence: a conditional-write publisher may leave an earlier referent in place on a path that does not write — `Amass` with an Army, `EmpowerJace` with a Jace, `Endure`'s counter branch, a wrapper whose chosen payload creates nothing. That is why a placement after one of them must not pair with an earlier `Token`. The parser's ungated-publisher seeding (`chain_prior_referent_is_created_token`) assumes the referent exists, and no printed card reads a created-referent anaphor after a conditional writer (`relink_census.py`, `followers.py`, `wrappers.py`).

**The token-creator set stays `{Token, CopyTokenOf, Populate}`.** The charter defines it as "the set the parser calls token-creating today", and a candidate joins "only if" it also publishes a token it creates, which is a necessary condition.
- Every consumer of this set is either a parser nearest-producer scan or phase 2's union. The parser scans are `resolve_populated_token_anaphors`, `publishes_tracked_set_from_resolution` via `nearest_publisher_is_self_move`, `classify_bare_card_aggregate_publisher`, the token-cleanup gate and the goaded arm.
- Admitting a keyword action that creates a token as one step would move those scans. For example, Disorder in the Court would then see Investigate as the nearest tracked-set publisher.
- `CopyTokenBlockingAttacker` meets both conditions. Its only printed card, Mirror Match, lowers its "those tokens" follow-up bespoke to `LastCreated`, so no card needs it. It is recorded as a follow-up candidate, not admitted.
- Result: **zero** token-creator parse change, by construction.

### D1.2 — AI pairing

`created_token_placement_source` finds the nearest preceding publisher through the engine predicate, and pairs only if it is `Effect::Token`. A wrapper whose own-resolution payload publishes is a publisher, so it blocks the pairing; a wrapper whose payload does not publish is transparent.

The doc records why `CopyTokenOf` and `Populate` stay unpaired. The pairing exists to give the imperative "create … then put counters on it" the same reading as the declarative `Token { enter_with_counters }`, which is the parity suite's contract. `CopyTokenOf` and `Populate` carry no `enter_with_counters` twin, so pairing them would invent a reading no consumer defines. No other AI logic is added.

### D1.3 — guard

- Rename `subject_dependent_type_condition_has_no_subject` → `subject_dependent_condition_has_no_subject`. It has 2 call sites, both in the same file: the recursive call and `evaluate_condition`'s `Not` arm.
- Add the arms `QuantityCheck { lhs, rhs, .. }` (absent if either side is) and `QuantityQuality { quantity, .. }`, both calling `quantity_expr_missing_resolution_only_referent`.
- `And`/`Or` are not recursed. The doc records D1.3's known limitation verbatim: a negated compound with a subject-dependent operand remains vacuously true, and no printed card reaches it.
- Fix the PR-added citations:
  - the PR-added `evaluate_condition` `QuantityQuality` arm comment cites CR 608.2c, not CR 701.57c;
  - the pre-PR `QuantityCheck` arm keeps its CR 701.57c (Discover) rationale;
  - the PR-added test comment in `quantity_quality_ability_condition_evaluates_at_resolution` ("CR 701.57c absent-referent guard") becomes CR 608.2c.

Considered and not done: folding the guard's two arms and `condition_awaits_resolution_only_referent`'s identical leaf arms into one leaf helper.
- The two walkers deliberately recurse differently: the deferral walk recurses `And`/`Or`, and the guard must not.
- The shared logic already lives in `quantity_expr_missing_resolution_only_referent`.
- `condition_awaits_resolution_only_referent` is outside phase 1's subsystem scope rule.

### D1.4 — C4

Add an explicit `TriggerCondition::EventTime { .. } => None` before `_ => None` in `positive_count_gate_qty`, with this comment:

> CR 603.4 + CR 603.8: an `EventTime` condition is the trigger event itself (an attack-while gate, an event count qualifier, a state trigger's condition), read when the event occurs and never rechecked; CR 603.4's recheck applies only to an intervening "if". It is not a positive resolution-recheck count gate, so it names no antecedent for the body's count anaphor — binding one would read the count at resolution, not the count the event measured. Deliberately no descent.

Also add `EventTime` to the doc list: "`Not`, `Or` and `EventTime` never bind".

### D1.5 — C2

Rename `game/triggers.rs::quantity_quality_rechecked_at_resolution` → `bind_triggered_resolution_scope_rechecks_quantity_quality`. Its doc says that it exercises `stack::bind_triggered_resolution_scope` with a constructed scope, and that the production coverage of the stacked trigger's recheck is `tests/integration/zimone_all_questioning.rs::resolution_recheck_and_live_count`. No logic changes.

### D1.6 — Wild Hypothesis guard

E1 and E1b, below, in `created_token_counter_sequence.rs`.

---

## Step-by-step plan

### U-A

1. **`crates/engine/src/types/ability.rs`, `impl Effect`** (next to `moves_card_to_or_from_library`):
   - **Private `enum CreatedReferentPublisher { TokenCreator, OtherProducer }`**, defined as in Rust Idioms, placed beside the methods.
   - **`fn created_referent_publication(&self) -> Option<CreatedReferentPublisher>`.**
     - It is a wildcard-free `match self`. Its doc says it answers for **this node only**, that nested payloads are `publishes_created_referent`'s walk, and the "do not add an `_ =>` arm" sentence.
     - `Some(TokenCreator)` arm (CR 111.1, 701.36a, 707.2): `Token`, `CopyTokenOf`, `Populate`.
     - `Some(OtherProducer)` arm, in sub-groups, each with its comment:
       - face-down entry (CR 701.40a, 701.58a, 701.62a): `Manifest`, `ManifestDread`, `Cloak`;
       - digital-only: `Conjure | DraftFromSpellbook` (no CR);
       - copies (CR 707.2): `CopyTokenBlockingAttacker`, `EachPlayerCopyChosen`, `CreateTokenCopyFromPool`;
       - keyword actions and abilities that create a token as one step: CR 701.16a `Investigate`, CR 701.47a `Amass`, CR 701.63a `Endure`, CR 702.116a `Myriad`, CR 702.141a `Encore`, and `EmpowerJace` (digital-only). This sub-group carries the conditional-write sentence for `Amass`, `EmpowerJace` and `Endure`.
     - `None` arm: every other variant.
       - It opens with the inline-payload wrappers, under a comment: "CR 608.2c + CR 608.2d: their own node writes nothing; the payload they resolve inline is classified by the own-resolution walk in `publishes_created_referent`". The wrappers are `ChooseOneOf`, `RollDie`, `FlipCoin`, `FlipCoins`, `FlipCoinUntilLose`, `SeparateIntoPiles`, `Vote` and `RevealFromHand`.
       - It continues with the boundary carriers, under a comment citing CR 603.3 / 603.7a, then the remaining variants.
       - The set is "every variant not in the two `Some` arms". Take it from the enum, with the compiler as the check.
   - **`pub fn publishes_created_referent(&self) -> bool`.** Its body is as in Rust Idioms, `visit_effect_scoped` under `ResolutionScope::OwnResolutionOnly`. Its doc moves and generalizes the rationale now on `lower.rs::publishes_chain_created_referent` (Conductive Machete / Weight Room #7531, Agent of Raffine). The doc reads: "CR 608.2c: true when this effect's own resolution writes the referent `TargetFilter::LastCreated` reads (`GameState::last_created_token_ids`) on at least one resolution path: this node's resolver, or a publisher in an `AbilityDefinition` payload it resolves inline (CR 608.2d). A payload registered to resolve later (CR 603.3, CR 603.7a, CR 603.12) does not count; `ResolutionScope::OwnResolutionOnly` is that boundary. Each write is an assignment, so the referent a later instruction reads is the nearest publisher's product." It then adds the known-limitation sentence from D1.1, and the consumers: "the parser's anaphor passes (through `publishes_chain_created_referent`), the declined-gate walk, phase-ai's created-token pairing".
   - **`pub fn is_token_creator(&self) -> bool`.** Its body is as in Rust Idioms. Its doc reads: "CR 111.1: the token-creation effects whose own parameters define the token(s) — a spec, a copy source, or the populated token. Node-only, and a subset of `publishes_created_referent` by construction (both read `created_referent_publication`). Keyword actions that create a token as one step, and wrappers whose payload creates one, publish the referent but are deliberately not members: this set keys the parser's nearest-token-creator and tracked-set scans and the player-scope created-token union, and widening it moves those scans."
   - **Inline tests:** a new `#[cfg(test)] mod created_referent_classification_tests` at the end of the file, holding P1–P5.
2. **`crates/engine/src/parser/oracle_effect/lower.rs`.** Replace the bodies: `is_token_creating_effect(effect)` → `effect.is_token_creator()`, and `publishes_chain_created_referent(effect)` → `effect.publishes_created_referent()`. Visibility and names stay unchanged. Each keeps a two-line doc naming the method as the authority, since the long rationale has moved. Callers are untouched. Because the delegator is the D1.1 authority, the parser sees the expanded publisher set; the parse diff it can cause is derived below and is expected to be zero.
3. **`crates/engine/src/parser/oracle_effect/sequence.rs`, tokens-goaded arm only.** Replace the effect-pattern list with the predicate, keeping the same arm position and the same detector. The crate is edition 2021 (no let chains), so use one `if let` guard, e.g. `effect if let Some(continuation) = effect.is_token_creator().then(|| try_parse_tokens_goaded_continuation(&lower)).flatten() => Some(continuation)`, or any equivalent that compiles, keeps the arm's position, evaluates the detector only for token creators, and does **not** match when the detector returns `None`, so the effect falls through to the later arms. Those include `Effect::Token { .. } => parse_token_source_power_toughness_followup(&lower)` (`sequence.rs:9341`) and the arm commented 'Must be checked before the Effect::Token arm' (`sequence.rs:9314`). A guard of the form `effect if effect.is_token_creator() => try_parse_tokens_goaded_continuation(&lower)` is **not** equivalent: it would swallow those arms for every `Token`. Keep the CR 701.15a + 701.15b comment.
4. **`crates/engine/src/game/effects/mod.rs`.** Delete `use crate::parser::oracle_effect::publishes_chain_created_referent;`. In `declined_gate_surviving_instructions`, call `node.effect.publishes_created_referent()`, and update the adjacent comment to name `Effect::publishes_created_referent`.
5. **`crates/engine/src/types/zones.rs`** (the `ChainReferentIntent` doc, comment-only). Change "the parser's admission authority (`oracle_effect::lower::publishes_chain_created_referent`)" to "the publisher classification (`Effect::publishes_created_referent`, which the parser delegates to)".
6. **`crates/phase-ai/src/ability_chain.rs`.**
   - Update the module doc (its created-token-placement paragraph) and `created_token_placement_source`'s doc and body per D1.2.
   - Rename the existing test `placement_pairs_with_the_nearest_preceding_token` → `placement_pairs_with_the_nearest_publisher_when_it_is_a_token`, and fix its doc: "whatever sits between them" is no longer true.
   - Add the A-rows.
7. **AI gate.** Run `cargo ai-gate` per CLAUDE.md (V-AI). Expected: no delta, since the corpus shows 0 classification changes (lead). If the baselines move, refresh them only with the paired-seed report, and return the delta to the orchestrator.

### U-C

8. **`crates/engine/src/game/effects/mod.rs`.** Rename, add the two arms and the doc per D1.3. Fix the PR-added CR citation in the `QuantityQuality` arm and in the PR-added test comment. Add the C-rows as inline tests next to `quantity_quality_ability_condition_evaluates_at_resolution`.

### U-E

9. **`crates/engine/src/game/triggers.rs`:** the C2 rename and doc (D1.5).
10. **`crates/engine/src/parser/oracle_trigger.rs`:** the C4 arm and doc line (D1.4).
11. **`crates/engine/tests/integration/created_token_counter_sequence.rs`:** E1, E1b and R1–R4. E1 uses a `const WILD_HYPOTHESIS` holding the verbatim Oracle text; the CLASS row already uses the same text.

### Verification commands

These are for the executor. Tilt is down here: if `tilt get uiresource clippy` exits 0, use `./scripts/tilt-wait.sh`. Otherwise use direct cargo with an isolated `CARGO_TARGET_DIR` and `CARGO_INCREMENTAL=0`.

- `cargo fmt --all`
- `cargo clippy -p phase-engine -p phase-ai --lib --tests -- -D warnings`
- `cargo test -p phase-engine --features test-support --lib -- created_referent_classification_tests subject_dependent quantity_quality bind_triggered_resolution_scope_rechecks_quantity_quality positive_count_gate` (with the C-rows' final names). Also `cargo test -p phase-engine --features test-support --lib parser::oracle_trigger`: the existing `positive_count_gate_qty_binds_only_positive_gates` stays green.
- `cargo test -p phase-engine --features test-support --lib -- spellbook choose_one_of endure`: the resolver suites behind the new members stay green.
- `cargo test -p phase-engine --features test-support --test integration -- created_token_counter_sequence issue_2871 zimone_all_questioning oversimplify perpetual_last_created war_games goaded`
- `cargo test -p phase-ai --lib`: the `ability_chain` tests, `created_token_placement_parity` and every consumer suite.
- `./scripts/check-parser-combinators.sh 8949699a13a541a7e00ada2bb8e16c00c530b82a`
- **Parse diff.** Run `./scripts/gen-card-data.sh` at `PHASE_BASE_SHA` (into the run dir, e.g. `zimone-r2/base-data-p1/`) and at the candidate, then compare per card. Alternatively, run `cargo coverage` on both sides plus `cargo run --profile tool --features cli --bin coverage-parse-diff -- <base> <head>`, the format the PR's sticky comment uses. **Expected: zero cards** (derivation below). A non-empty diff returns to the phase plan step.
- `cargo ai-gate` (V-AI).
- **Disk.** About 9.3 GB was free on `/` at planning time. The orchestrator should confirm capacity before the first cold build.

---

## Verification Matrix

Legend:
- **fails@base**: derived from the base code to fail when the phase's source change is reverted. Unprobed until the executor runs it.
- **control**: green at base and at head.

### U-A — engine classification (`types/ability.rs` inline; new API, so these are pure unit rows)

| ID | Assertion | Reach-guard / sibling | Status |
|---|---|---|---|
| P1 (subset invariant) | For fixtures `Token`, `CopyTokenOf`, `Populate`: `is_token_creator()` and `publishes_created_referent()` are both true | three distinct variants constructed | new-API unit |
| P2 (other producers) | For one fixture of **every** `OtherProducer` row of the D1.1 table (including `DraftFromSpellbook` and `Endure`): `publishes_created_referent()` is true and `is_token_creator()` is false | — | new-API unit; the discriminating counterparts are A3 |
| P3 (non-members) | Both false for `Incubate`, `GiftDelivery`, `Mill`, `PutCounter{LastCreated}`, `ChangeZone{→Battlefield}`, `Meld`, `ChooseAndSacrificeRest` | P1/P2 prove the method answers true at all | new-API unit (hostile: an effect that creates a token without writing the slot, battlefield movers, the sweep's false positive) |
| P4 (wrappers, option (a)) | For each wrapper variant (`ChooseOneOf`, `RollDie`, `FlipCoin`, `FlipCoins`, `FlipCoinUntilLose`, `SeparateIntoPiles`, `Vote`, `RevealFromHand`): with a payload whose chain contains a `Token` (also as a non-head sub-node), `publishes_created_referent()` is true and `is_token_creator()` is false. With a payload of `GainLife`/`Draw` only, both are false. | the positive and negative payloads of the same variant | new-API unit |
| P5 (boundary) | `CreateDelayedTrigger { effect: Token }` → false. A `ChooseOneOf` whose only `Token` sits in a branch def carrying a `WhenYouDo` marker → false (CR 603.12, `scope_prunes_nested_ability`). | P4's positive `ChooseOneOf` | new-API unit (hostile: a later-resolution payload) |

Wrapper and heavy-variant fixtures may use the engine types' constructors, or `serde_json::from_value` of the card-data shape (for example Done for the Day's `ChooseOneOf` node). The fixture lists are test data, not a second classification authority: each row asserts that the listed effect *is* classified the way the D1.1 table says.

### U-A — AI pairing (`phase-ai/src/ability_chain.rs` inline; drives production consumers)

`P` = `last_created_placement(Plus1Plus1, 1)`. Abilities are built with `effect_chain(Spell, ContinuationStep, …)`.

| ID | Chain | Assertions | Reach-guard | Status |
|---|---|---|---|---|
| A1 (F4 ordinary) | `[Token, P]` | `source_at(..,1) == Some(0)`; `ability_creates_token_with_plus_one_counters` is true; `AbilityWalk::unconditional(..).without_created_token_placements().len() == 1` | positive leg of A2 | control |
| A2 (F4, intervening Conjure) | `[Token, Conjure, P]` | `source_at(..,2) == None`; `ability_creates_token_with_plus_one_counters` is **false**; the walk keeps all 3 | A1 shows the same consumers return the paired answer | **fails@base** (base `rposition(Token)` gives `Some(0)`, the consumer true, walk length 2) |
| A3 (class) | `[Token, X, P]` for **every** X in the D1.1 table whose node classification is `OtherProducer` | per X: `assert!(X.publishes_created_referent())` (fixture reach-guard), then `source_at(..,2) == None` | per-X reach-guard; A1 | **fails@base** for every X |
| A4 (copy publishers stay unpaired) | `[CopyTokenOf, P]`, `[Populate, P]`, `[Token, CopyTokenOf, P]` | all `None` | A5 positive | first two control; the third **fails@base** |
| A5 (order) | `[Conjure, Token, P]`, `[CopyTokenOf, Token, P]`, `[DraftFromSpellbook, Token, P]` | `Some(1)` | — | control |
| A6 (walk-level, the maintainer's `x_reference`/`plus_one_counters` consumers) | the A2 ability through `AbilityWalk::of(ability, collect_definition_effects(ability))` | `without_created_token_placements().len() == 3` | A1's walk drops 1 | **fails@base** |
| A7 (hostile: an effect that creates a token without publishing) | `[Token, Incubate, P]`; `[Token, Mill, P]` (existing) | `Some(0)` | `!Incubate.publishes_created_referent()` asserted | control (proves the rule is "publisher", not "creates a token") |
| A8 (existing) | `else_branch_placement_is_not_paired_with_the_root_token`, `placements_in_another_mode_are_not_paired`, `non_token_publishers_and_other_targets_do_not_pair`, `filtering_view_drops_only_created_token_placements` | unchanged | — | control |
| A9 (suite) | `policies/tests/created_token_placement_parity.rs` and every consumer suite (`cast_facts`, `redundancy_avoidance`, `plus_one_counters`, `x_cast_gate`) | green | — | control |
| A10a (a wrapper publishes) | `[Token, ChooseOneOf{branches:[Token, GainLife]}, P]`; `[Token, RollDie{results:[…Token…]}, P]` | `None` | `assert!(wrapper.publishes_created_referent())` | **fails@base** (base gives `Some(0)`) |
| A10b (a wrapper is transparent) | `[Token, ChooseOneOf{branches:[GainLife, Draw]}, P]` | `Some(0)` | `!wrapper.publishes_created_referent()` asserted | control (hostile sibling of A10a) |
| A10c (nested wrapper) | `[Token, ChooseOneOf{branches:[RollDie{results:[…Token…]}]}, P]` | `None` | — | **fails@base** |
| A10d (boundary carrier) | `[Token, CreateDelayedTrigger{effect: Token}, P]` | `Some(0)` | `!carrier.publishes_created_referent()` asserted | control (a later-resolution payload never blocks the pairing) |

### U-A — runtime reference readings (`created_token_counter_sequence.rs`; `resolve_ability_chain` on a `GameScenario` board)

| ID | Scenario | Assertion | Reach-guard | Status |
|---|---|---|---|---|
| R1 | Resolve `[Token 1/1 Soldier, Conjure{Named, Battlefield}, PutCounter{P1P1, 1, LastCreated}]` | the conjured object has one +1/+1 counter; the Soldier has none | both objects exist on the battlefield. Control chain: `[Token, PutCounter{LastCreated}]` puts the counter on the Soldier. | control (establishes D1.2's premise: the nearest publisher's product is the referent) |
| R2 (`DraftFromSpellbook`, random path) | The source's `spellbook` = one name (deterministic, as in `spellbook_tests.rs::random_spellbook_draft_conjures_without_pausing`). Resolve `[Token Soldier, DraftFromSpellbook{random: true, destination: Battlefield}, PutCounter{P1P1,1,LastCreated}]`. | the drafted object has the counter; the Soldier has none | the drafted object exists on the battlefield | control (runtime evidence for the D1.1 row) |
| R2b (`DraftFromSpellbook`, choice path) | same, with `random: false`; at `WaitingFor::SpellbookDraft`, `runner.act(GameAction::SubmitSpellbookDraft { card })` | before the submit: no counter anywhere (the tail is stashed). After it: the drafted object has the counter, the Soldier none. | `matches!(waiting_for, SpellbookDraft { .. })` | control |
| R3 (`Endure`, token branch) | Resolve `[Token Soldier, Endure{amount: 2, subject: SelfRef}, PutCounter{P1P1,1,LastCreated}]` with the source on the battlefield; at `WaitingFor::ChooseOneOfBranch`, act `ChooseBranch` with the token branch's index (assert that `branch_descriptions` names the Spirit token) | the 2/2 Spirit has the counter; the Soldier has none | the Spirit exists | control |
| R4 (wrapper, option (a)) | Resolve `[Token Soldier, ChooseOneOf{chooser: Controller, branches:[Token 1/1 Spirit, GainLife 1]}, PutCounter{P1P1,1,LastCreated}]`; choose the Spirit branch | the Spirit has the counter; the Soldier has none | the Spirit exists | control (runtime evidence that the wrapper's resolver assigns the slot before its continuation) |

If a faceless conjured object cannot hold a counter, use a registry-backed conjure name, as `oracle_of_the_alpha_conjure_power_nine.rs` does, and record which. If `GameRunner::act` cannot answer a pause that `resolve_ability_chain` raised directly on `state_mut()`, stage R2b, R3 and R4 through an activated ability on the source (as the spellbook tests do), and record which.

### U-A — parser and game delegation

| ID | Claim | Evidence | Status |
|---|---|---|---|
| D1 | Parse output is unchanged for every card | the card-data per-card diff / `coverage-parse-diff` shows **zero cards** (derivation below) | measurement |
| D2 | The declined-gate walk is unchanged on printed cards | lead (`census_gate2.py`: 0 gated chains with a newly admitted publisher followed by another node; positive control `control`); the existing `effects/mod.rs` declined-gate tests stay green | control |
| D3 | The tokens-goaded continuation is unchanged | the existing goad suites (`goaded_creature_under_pacifism_visible.rs`, the War Games / Rendmaw / Life of the Party parses) are green, and the parse diff is zero | control |

### U-C — guard (`game/effects/mod.rs` inline, through `resolve_ability_chain`, the production entry that evaluates `ability.condition`)

Fixture: a P0-controlled source; root `GainLife { amount: 3, player: Controller }`; `condition` set per row. The subject is supplied by `CostPaidObjectSnapshot::capture` on an object whose mana value is set per row, or by an object target for C6.

| ID | Condition | Subject | Assertion | Reach-guard | Status |
|---|---|---|---|---|---|
| C1 (F3 negated, absent) | `Not(QQ(ObjectManaValue{CostPaidObject}, Even))` | none | no life gained | C2 in the same test gains | **fails@base** (`Not` of the arm's `false` → gain) |
| C2 (negated, present) | same | MV 3 | gains 3 | `resolve_quantity_for_ability_condition(..)==3` | control |
| C3 (negated, present, quality holds) | same | MV 2 | no gain | — | control |
| C4 (positive, present / absent) | `QQ(.., Even)` | MV 2 → gain; none → no gain | — | — | control (the existing arm's guard) |
| C5 (QuantityCheck negated, absent) | `Not(QuantityCheck{ObjectManaValue{CostPaidObject} GE 1})` | none | no gain | C5b | **fails@base** |
| C5b (value 0 ≠ absent) | same | MV 0 | gains (0 ≥ 1 is false, so `Not` is true) | — | control; fails if a guard keys on value 0 instead of presence |
| C6 (other scope) | `Not(QuantityCheck{Power{Target} GE 3})` | no object target → no gain; a target with power 2 → gain | — | positive leg | the absent leg **fails@base** |
| C7 (multi-authority) | `Not(QQ(.., Even))` | `cost_paid_object` none, `effect_context_object` MV 3 | gains | — | control; the presence ladder's fallback keeps the guard quiet |
| C8 (revealed arm) | `issue_2871_currency_converter.rs` (every test in the file) | — | green | — | control |

Evaluate through `resolve_ability_chain(.., 0)` and read P0's life. The direct `evaluate_condition` form is acceptable for one row only if the executor shows that the chain form cannot stage that subject, and says so.

### U-E

| ID | Scenario | Assertion | Reach-guard | Status |
|---|---|---|---|---|
| E1 (Wild Hypothesis, D1.6) | Real cast with `with_mana_pool(G×3)`, X=2, library top `Card A/B/C`; `cast(spell).x(2).commit()`, then `resolve_top()` | At `WaitingFor::SurveilChoice` (2 cards): one Fractal on the battlefield with 2 +1/+1 counters. Then `act(SelectCards { cards: vec![] })` sends both to the graveyard. Final state: Fractal 2/2 with 2 counters, P0 graveyard = 3 (2 surveilled + the sorcery), library = 1. | `matches!(waiting_for, SurveilChoice { cards, .. } if cards.len() == 2)` | guard; derived green at base (`Token → PutCounter{X, LastCreated} → Surveil(2, SequentialSibling)`, no `player_scope`). If red at base → the pre-existing-report route (charter D1.6). |
| E1b (X=0 sibling) | same, with X=0 | The surveil prompt still arrives and resolves. After resolution the 0/0 Fractal is gone (CR 704.5f at the CR 704.3 check, then CR 111.7), and the graveyard holds 3 cards. | E1 | guard: Surveil is independent of the placement and of the token's survival |
| E2 (C2) | the renamed test | assertions unchanged; passes | — | non-discriminating by construction (a rename) |
| E3 (C4) | `positive_count_gate_qty` | the explicit arm is present; the existing `positive_count_gate_qty_binds_only_positive_gates` is green; the parse diff is zero | — | non-discriminating by construction (`EventTime` already fell to `_ => None`). A lock-in assertion would belong in `oracle_trigger_tests.rs`, which is outside the scope rule, so none is added. |

### V-AI

`cargo ai-gate`: expected no delta. If there is one, return it with the paired-seed report; do not refresh baselines silently.

### DEFERRED rows

- `DEFERRED(phase 2)`: the driver's union publication keyed on `Effect::is_token_creator` (War Games, Rendmaw, Life of the Party; synchronous and paused).
- `DEFERRED(phase 3)`: Oversimplify's two-action order, the per-seat binding, both Fractals at the first replacement prompt, and the rewritten splitter unit test.
- `DEFERRED(phase 4)`: coverage condition-quantity honesty.

## Expected parse diff (derived card by card)

**Mechanism.** The parse can change only where a parser site consults a predicate whose answer changed.
- `is_token_creator` membership is unchanged, so `resolve_populated_token_anaphors`, the token-cleanup gate, `publishes_tracked_set_from_resolution`, `classify_bare_card_aggregate_publisher` and the goaded arm contribute zero.
- `publishes_created_referent` gains every D1.1 row marked **no** in 'Today in parser set?'. Its parser sites all ask about a publisher that **precedes** a later consumer clause: `chain_prior_referent_is_created_token` (and through it every `token_created_in_chain` reader, §Step 2), `bare_they_player_declaration`, `relink_gated_token_referent_consumers` and `clone_would_transplant_gated_referent`.
- So only cards where a newly admitted publisher precedes another clause in the same chain can move.

Lead (static census):
- `p1/census_members.py` for the r1 members.
- `p1r2/followers.py` for `DraftFromSpellbook` and `Endure`. Endure has no node with a follower.
- `p1r2/wrappers.py` for the wrappers. One wrapper node has a follower.

`p1r2/relink_census.py` confirms that the only `LastCreated` reader whose nearest publisher is newly admitted is Mirror Match.

Every card with a following clause:

| Card | Clause after the member (Oracle) | Site that could see the member | Derived reading |
|---|---|---|---|
| Assault on Osgiliath | "then Goblins and Orcs you control gain double strike and haste" | none (typed group, no anaphor) | unchanged |
| Azog, Moria's Ruin | "If you controlled that creature, draw a card." | condition parse (`HasObjectTarget ∧ TargetMatchesFilter`, target-based) | unchanged. This is the one row resting on the condition parser not consulting the anaphor walk; the parse diff confirms it. |
| Foray of Orcs | "When you do, ~ deals X damage to target creature …, where X is the amassed Army's power" | `relink_gated…`: the publisher is Amass, which carries no gate → `continue`; typed target | unchanged |
| Goblin Plate Mail | "then attach this Equipment to the amassed Army" | none (`AmassedArmy` referent) | unchanged |
| Grishnákh, Brash Instigator | "gain control of target … Untap that creature. It gains haste" | the anaphor walk re-anchors on the typed target before reaching Amass | unchanged |
| Lazotep Plating | "You and permanents you control gain hexproof" | none | unchanged |
| Sauron, Lord of the Rings | "mill five cards, then return a creature card from your graveyard to the battlefield" | none | unchanged |
| Summons of Saruman | "Mill X cards. You may cast … from among them" | bare-card aggregate (Mill is nearest; the token-creator set is unchanged) | unchanged |
| Surrounded by Orcs | "then target player mills X cards, where X is the amassed Army's power" | none | unchanged |
| Tidings of War | "If this spell was cast from a graveyard, amass Goblins 3 instead." | none | unchanged |
| Protégé's Awakening | "Draw a card." | none | unchanged |
| Mirror Match | "Exile those tokens at end of combat." | bespoke IR, already `LastCreated`; `relink`: the publisher is ungated → `continue` | unchanged |
| Ate-o'-clock | "Regenerate target creature." | typed target | unchanged |
| Confront the Unknown | "then target creature gets +1/+1 … for each Clue you control" | typed target | unchanged |
| Disorder in the Court | "Return the exiled cards to the battlefield tapped …" | tracked-set scan (token-creator set, unchanged) | unchanged |
| Elemental, My Dear | "then create an 0/0 … token. Put +1/+1 counters on that token …" | the nearest publisher is the later `Token` | unchanged |
| Eliminate the Impossible | "Creatures your opponents control get -2/-0 … If any of them are suspected, they're no longer suspected." | `bare_they` needs a player declaration, and there is none | unchanged |
| Innocuous Researcher | "Then each player draws a card." | none | unchanged |
| No Witnesses | "Then destroy all creatures." | none | unchanged |
| Ongoing Investigation | "You gain 2 life." | none | unchanged |
| Panther Pounce | "Target creature gets +1/+0 and gains flying … Untap it." | the anaphor walk re-anchors on the typed target | unchanged |
| Secrets of the Key | "If this spell was cast from a graveyard, investigate twice instead." | none | unchanged |
| Serene Sleuth | "Then each creature you control is no longer goaded." | none | unchanged |
| Tail the Suspect | "You may play an additional land this turn." | none | unchanged |
| Wernog, Rider's Chaplain | "Each opponent who doesn't loses 1 life. You investigate X times …" | none (`player_actions_this_way` quantity) | unchanged |
| Adaptive Armorer | "When an Equipment enters this way, attach it to target creature you control." | `zone_change_reflexive::unsupported_zone_change_reflexive` returns `zone_change_reflexive_event_source` from the producer test (`publishes_event(DraftFromSpellbook, Enter)` is false). This is decided before any referent read and does not consult the publisher predicate. | unchanged (`Unimplemented`, same gap) |
| Masterpiece Vault | same clause | same | unchanged |
| Blood Age Muster | "Its base power and toughness perpetually become 2/2." | `try_parse_perpetual_base_pt`'s "its base power and toughness perpetually" arm takes no `ParseContext` and returns `ParentTarget` unconditionally. No later pass rewrites `ParentTarget` from the publisher predicate: its only rewrite consumers key on the token-creator set. | unchanged (`ApplyPerpetual{ParentTarget}`) |
| Brood Astronomer | "draft a card … and put it onto the battlefield tapped" | The put is a within-clause sub-node of the draft. `chain_prior_referent_is_created_token` reads only *earlier* clauses (here the Sacrifice), so the draft is never its prior clause. For `relink`, the follower reads `ParentTarget`, not `LastCreated`. | unchanged (`ChangeZone{ParentTarget}`) |
| Dazzling Flameweaver | "You may play that card until the end of your next turn." | `TrackedSet` permission; no `token_created_in_chain` reader | unchanged |
| Kayla's Kindling | "Until end of turn, you may cast that card." | same | unchanged |
| Hinterland Chef | "It perpetually becomes a Food artifact … and perpetually gains …" | `parse_perpetual_self_subject`'s "it perpetually " arm requires "gain(s) " and fails before reading the flag. `try_parse_perpetual_base_pt`'s "it perpetually" arm requires "base power and toughness". `subject.rs`'s bare-"it" flag check also needs a declared typed object target (none). Every printed "It perpetually becomes" card is `unbound_subject` regardless of its antecedent (Kavaron Consumed, Pep, Turbocharged Escape, Veko, Woodland Investigation). | unchanged (`unbound_subject`) |
| Ominous Traveler | "That card perpetually gains "…" and "…"" | The "that " arm returns `ParentTarget` with no flag read, and the grant body fails. Reference: Futurist Spellthief, whose ungated `Conjure` already publishes, is `unbound_subject` for the same sentence. | unchanged (`unbound_subject`) |
| Oyaminartok, Polar Werebear | "When you do, add {U}{U}{U}. …" | `Mana`, with no anaphor. `relink`: the publisher is ungated → `continue`. | unchanged |
| Saint Elenda | "You may cast that card this turn without paying its mana cost." | the cast parser's anaphor does not read `token_created_in_chain` (reader list, §Step 2) | unchanged (`CastFromZone{ParentTarget}`) |
| Skywriter Djinn | "Then if you have fewer cards …, repeat this process." | `unrecognized_clause_head`, no anaphor | unchanged |
| Sliver Weftwinder | "then draw a card" | `Draw{Controller}` | unchanged |
| Support Skyforge | "That card perpetually becomes an artifact creature." | The "that " perpetual arm requires "gain(s)". "that card" is not a chain-created demonstrative (`anaphor_is_chain_created_demonstrative`). Reference: Glimmer Hoarder (no publisher) is `unbound_subject` for the same shape. | unchanged (`unbound_subject`) |
| Tome of the Infinite | "It perpetually gains "You may spend mana as though it were mana of any color to cast this spell."" | **The flag does flip here.** The ungated `DraftFromSpellbook` makes `token_created_in_chain` true, so `parse_perpetual_self_subject` now binds `LastCreated` instead of declining. The grant body is the same one that fails in Bind to Secrecy's mode 2, where an ungated `Conjure` already sets the flag and the clause is `unbound_subject`. The fallback after a failed body is the same path either way. | unchanged (`unbound_subject`). This is the one row resting on a reference rather than on a site that ignores the flag; the parse diff confirms it. |
| Done for the Day | "If you control all three, you may put a sticker on a nonland permanent you own." | `PutSticker{Typed}`: no anaphor, no "they", and not a `LastCreated` reader for `relink` | unchanged |

C4 (`EventTime`) and the goaded-arm delegation change no output, by construction. **Expected run-span contribution of phase 1: zero cards.** A non-empty diff returns to the phase plan step, under the charter's derived-set rule.

**Pre-existing runtime readings, not changed by this phase.** Three follower readings that the table keeps are probably inert at runtime, because `ParentTarget` with no declared target binds nothing:
- Blood Age Muster's `ApplyPerpetual{ParentTarget}`;
- Brood Astronomer's `ChangeZone{ParentTarget}`;
- Saint Elenda's `CastFromZone{ParentTarget}`.

This is unprobed. Fixing them changes parser files and arms outside phase 1's scope rule (`oracle_effect/mod.rs`, the compound-action and cast parsers). They take the pre-existing-report route (fix, file or drop) and are not absorbed. They are not reference readings any row of this plan relies on.

## Reference Readings

| Row | Reference | Derived from Oracle + CR | Measured | Verdict |
|---|---|---|---|---|
| E1 | Wild Hypothesis today | CR 608.2c: create the 0/0 Fractal, put X=2 counters on it, then surveil 2 as a separate instruction (CR 701.25a). Both looked-at cards may go to the graveyard, and the sorcery goes to the graveyard after resolving → graveyard 3, Fractal 2/2. | plan-r1 head probe at `dfc1a50` (the Fractal had 2 counters at the `SurveilChoice` prompt, graveyard 3 after). Not re-run at `8949699` (`dfc1a50..8949699` touches none of these paths). | agrees; E1 re-measures it at base |
| R1 | Runtime `LastCreated` after `[Token, Conjure, PutCounter]` | CR 608.2c: "put a counter on it" after "conjure X" refers to the conjured card | `conjure::resolve` assigns the slot; `perpetual_last_created_empty_no_source_fallback.rs::nonzero_conjure_still_binds_the_last_created_perpetual_grant` | agrees |
| R2/R2b | `LastCreated` after a spellbook draft | Constructed (no printed card places a counter after a draft). The reading follows from CR 608.2c and the D1.1 criterion: the drafted card is the object the draft just produced. It is labelled constructed, not derived from a card. | `spellbook::complete_draft` → `conjure::resolve` (static) | to measure (R2/R2b) |
| R3 | `LastCreated` after endure's token branch | Constructed. CR 701.63a: the token branch creates an N/N Spirit; CR 608.2c makes it the nearest created object. | `endure::resolve` → `resolve_branch` (static) | to measure (R3) |
| R4 | `LastCreated` after a choose-one-of whose chosen branch creates | Constructed. CR 608.2c + CR 608.2d: the chosen branch is followed inside this resolution. | `choose_one_of::resolve_branch` (static) | to measure (R4) |
| A1/A5/A7/A10b/A10d | Existing pairing (the `Some(..)` rows) | the pairing's own contract (parity with `Token { enter_with_counters }`) | existing tests; the new rows are derived from D1.2 | agrees |
| D1/D3 | "parse unchanged" | the derivation table above | the executor's parse diff | pending measurement |
| Tome of the Infinite (parse) | Bind to Secrecy mode 2's `unbound_subject` | the grant body is identical, and both clauses have the created-referent flag set | static (card-data) | agrees; the parse diff confirms it |
| C8 | Revealed-type guard | issue #2871 rows | existing integration tests | agrees |

R2–R4 are constructed chains, because no printed card has the shape. Each states its reading from the CR and the D1.1 criterion, labelled as such. No defective reference was found.

## Maintainer-simulation matrix

| Maintainer point (cited site) | What they will check | Plan answer | Rows |
|---|---|---|---|
| F4: `ability_chain.rs:114-123` searches back for any `Token` | `[Token, Conjure, PutCounter{LastCreated}]` does not pair | the nearest publisher via the engine predicate; pair only if it is `Token` | A2, A6 |
| F4: "engine-owned publisher classification" (`conjure.rs:275-279`, `x_reference.rs:334-339`, `ability_chain.rs:178-183`) | one engine authority; no list in phase-ai or the parser | `Effect::created_referent_publication` (wildcard-free) in `types/ability.rs`, with both predicates derived from it; the parser and game layer delegate; phase-ai consumes | P1–P5, steps 1–4 |
| F4 controls: ordinary and intervening Conjure | both present | A1 / A2, plus every other publisher (A3) and every wrapper shape (A10) | A1, A2, A3, A10 |
| F4 (implicit): the publisher list is complete | no missed writer | a measured writer table with a regenerating sweep; every D1.1 row marked **no** in 'Today in parser set?' is admitted; exhaustive match | D1.1 table, A3, P2, P4 |
| F3: `Not(QuantityQuality(ObjectManaValue(CostPaidObject), Even))` with no subject (`effects/mod.rs:20609-20616, 21072-21076, 20114-20126`) | not vacuously true; via `quantity_expr_missing_resolution_only_referent` | guard arms for `QuantityQuality` and `QuantityCheck` | C1, C5, C6 |
| F3 controls: positive, negated, absent, present | all four | C1 (negated absent), C2/C3 (negated present), C4 (positive present/absent) | C1–C4 |
| F3 (implicit): CR citation | CR 608.2c on the PR-added arm | step 8 | — |
| C2 (CodeRabbit; no maintainer stance) | the name says what it tests | rename + production pointer | E2 |
| C4: "Blind recursion is not justified" | no descent | explicit `EventTime => None` with the CR 603.4/603.8 reason | E3 |
| C3: "predates this change. Nonblocking." | — | outside the run (charter) | — |
| C5: "Nonblocking limitation" | — | outside the run (charter) | — |
| Parse evidence: Wild Hypothesis "structural reparenting" | the reparented Surveil still resolves | runtime guard | E1, E1b |
| Fresh parse evidence for the head | no unintended parse change | zero-card parse diff, derived card by card | D1 |
| AI receipts pending | ai-gate | V-AI | V-AI |

## Per-unit acceptance rows

**U-A is accepted when all of the following hold.**
- A2, A3 (every listed X), the third A4 row, A6, A10a and A10c fail at `PHASE_BASE_SHA` and pass at the candidate. Where the base cannot compile the new test, an equivalent base-run assertion shows the base reading instead.
- A1, A5, A7–A9, A10b, A10d, P1–P5 and R1–R4 pass.
- The parse diff is zero cards.
- The non-test part of `crates/phase-ai/src/ability_chain.rs` (lines before its `#[cfg(test)]`) contains exactly one `Effect::Token { .. }`, in `created_token_placement_source` (`awk '/^#\[cfg\(test\)\]/{exit} /Effect::Token \{ \.\. \}/' crates/phase-ai/src/ability_chain.rs | wc -l` prints `1`).
- No publisher or token-creator list remains in `lower.rs` or the goaded arm.
- `created_referent_publication` has no `_ =>` arm.
- `effects/mod.rs` no longer imports the parser predicate.
- `types/zones.rs` names the engine method.
- Clippy and `cargo test -p phase-ai --lib` are green.
- `cargo ai-gate` shows no unexplained delta.

**U-C is accepted when:**
- C1, C5 and C6's absent leg fail at base and pass at the candidate;
- C2–C4, C5b, C7 and C8 pass;
- the guard's doc records the `And`/`Or` limitation;
- the PR-added arm and test comment cite CR 608.2c.

**U-E is accepted when:**
- E1 and E1b pass (if red at base: the pre-existing-report route, recorded);
- the C2 test carries its new name and production pointer;
- the C4 arm and doc line are present;
- the existing `positive_count_gate_qty_binds_only_positive_gates` is green.

## Unprobed (the executor closes these before relying on them)

1. Every **fails@base** claim, which is derived from the base code. Measure it by running the new tests against the base implementation where they compile, or by asserting the base reading directly.
2. Parse diff = zero cards (D1), in particular the Tome of the Infinite row. Command: §Verification commands.
3. AI gate = no delta (V-AI).
4. The compile form of the goaded-arm guard (step 3).
5. That `types/ability.rs` can call `crate::types::ability_visit::visit_effect_scoped` and `ResolutionScope` (both `pub(crate)`/`pub` in the same crate, as `game/mana_abilities.rs` already does). Closed by the first `cargo clippy`.
6. E1's prompt stop: that `commit()` then `resolve_top()` halts at `SurveilChoice`, and that `SelectCards { cards: vec![] }` sends both cards to the graveyard (per the handler, the payload is the keep-on-top set).
7. The pause and resume sequences of R2b, R3 and R4 (the fallback is named under the R-rows), and R1/R2's conjured object accepting a counter (the fallback is named under the R-rows).
8. The disk budget for the cold builds and `cargo ai-gate` (`server-release`).

## Notes for later phases and the integration review (not findings)

- Phase 2 keys its union on `Effect::is_token_creator()`. `EachPlayerCopyChosen` is not a member, and it never enters the per-seat loop: it is excepted from the player-scope fan-out in `resolve_ability_chain`.
- Pre-existing and outside phase 1's scope, so they take the report route (fix in a phase that edits the file, or file):
  - The doc on `oracle_effect/mod.rs::chain_prior_referent_is_created_token` still lists only `Token`/`CopyTokenOf`/`Populate` as referent producers.
  - `game/morph.rs`'s comment beside `manifest_card`'s request says the manifest family is "exactly the three effects the parser admits as chain-referent producers (`oracle_effect::lower::publishes_chain_created_referent`)". That is already false at base, and it stays false after this phase.
- The three probably-inert `ParentTarget` follower readings (Blood Age Muster, Brood Astronomer, Saint Elenda) are recorded under §Expected parse diff as pre-existing-report candidates.
- `Amass`, `EmpowerJace`, `Endure` and the wrappers write the slot only on some paths. `Amass`'s own referent is also modelled separately (`AmassedArmy`). No printed card reads `LastCreated`, or a created-referent anaphor, after them (derivation table and censuses). No action beyond the doc sentence.
- Considered and not done: admitting `CopyTokenBlockingAttacker` to the token-creator set. It meets both D1.1 conditions, but no printed card needs it.
- `types/ability_visit.rs`'s module doc names `is_mana_ability` as the first `OwnResolutionOnly` consumer. `publishes_created_referent` becomes another. The doc's statement stays true, so the file is not edited.
