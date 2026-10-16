# Phase 1 plan review r1 (run zimone-r2)

Mode: `review-engine-plan`, **phase-plan mode**. Phase index 1 of 4. `PHASE_BASE_SHA` = `8949699a13a541a7e00ada2bb8e16c00c530b82a` (checked out; `git rev-parse HEAD` agrees). Artifact: `p1-plan-r1.md`. Inputs read: the frozen `phase-charter` (phase 1 entry, scope rule, D1.1–D1.6, deferral list, seam notes), `addenda/phase-1` (empty), `phase-fit`, `review-input.md`, CLAUDE.md, `review-engine-plan/SKILL.md`, `engine-implementer/SKILL.md` (§Run limits, §Phase-fit gate, Step 2) and `chartered.md`.

**Phase-fit context declared: the Sizing consistency check (check 12) is blocking.**

Method: read-only. I made no repo edits, ran no cargo build, and used no stash, checkout or restore. My probes were `rg`/`sed` reads at `8949699` and static Python censuses over `data/card-data.json`, run with `python3 -I` from `…/scratchpad/rev/` and over the planner's own `…/scratchpad/p1/census_*.py`. Each census had a positive control.

**Result: 2 `behavior`, 3 `text`. This is a design round.**

---

## Blocking findings

### F-1 `behavior`: the D1.1 publisher membership is incomplete, and the measurement that produced it cannot find the missing members

**The plan's claims.** The plan says it has a "measured writer table". It also says (Maintainer-simulation row, "F4 (implicit): the publisher list is complete") "no missed writer". It gives "15 publisher variants" as the class, and it names `the rg in §Step 3.5` as the "Command that regenerates the writer set".

**Why the method misses members.** It traced only the direct callers of the writer functions (`token::resolve`, `token_copy::resolve`, `drive_copy_token_batches`, `apply_create_token_after_replacement*`, `conjure`, face-down publish). It did not trace resolvers that **synthesize** a publisher effect and resolve it. The plan caught one such resolver, `EachPlayerCopyChosen` (inner `CopyTokenOf`), so the method was applied inconsistently. It also did not decide the variants whose resolver runs an **AST payload** inline. D1.1's criterion ("the effect's resolver assigns the referent `TargetFilter::LastCreated` reads") is met by these variants, which the plan's wildcard-free `false` arm would silently answer `false`:

| Variant | Resolver path that assigns `last_created_token_ids` (read at `8949699`) | Disposition required |
|---|---|---|
| `DraftFromSpellbook` | `effects/spellbook.rs::complete_draft` (lines 101–136) builds `Effect::Conjure` and calls `super::conjure::resolve`, which assigns the slot at `conjure.rs:279`. Both paths reach it: random (`spellbook.rs:73`) and choice (`engine_resolution_choices.rs:7885`). Non-test code (`spellbook.rs` has no test module; tests live in `spellbook_tests.rs`). | **Member** (digital-only, like `Conjure`; no CR). Note that the precedent `moves_card_to_or_from_library` already groups `Conjure \| DraftFromSpellbook` in one arm (`types/ability.rs` ~24555). Following the plan's instruction "copy the variant list … drop the members" would split them. |
| `Endure` | `effects/endure.rs:51-71` builds an `Effect::Token` Spirit branch, and `choose_one_of::prompt_next` hands it on. On the token branch, `choose_one_of.rs:270` runs `resolve_ability_chain`, which calls `token::resolve` and assigns the slot. | **Member** (CR 701.63a, verified at line 3833: "creates an N/N white Spirit creature token unless …"). It writes only on the token branch, the same conditional-write shape as `Amass`/`EmpowerJace`. It is not a token creator: one step of a larger action, like `Investigate`. |
| `ChooseOneOf`, `RollDie`, `FlipCoin`, `FlipCoins`, `FlipCoinUntilLose`, `SeparateIntoPiles`, `Vote`, `RevealFromHand` (`on_decline`) | Each carries an `AbilityDefinition` payload (`types/ability.rs` `Effect` enum) and resolves it inline in the same resolution: `choose_one_of.rs:270`, `roll_die.rs:598`, `flip_coin.rs:129`, `separate_piles.rs:395/440`, `reveal_from_hand.rs:67/93`. A payload that contains a publisher assigns the slot. | **Decide, with recorded rationale.** Option (a): decide from the variant's own typed payload (recursive, like the precedent's `PayCost` delegation). Option (b): `false`, with the criterion reworded to say it means *this node's own* resolution, excluding nested ability payloads, plus a documented known limitation in the D1.3 style and a census. Under (b), `[Token, RollDie{results:[…Token…]}, PutCounter{LastCreated}]` still pairs with the outer `Token` while the runtime referent is the die result's token, which is the F4 defect shape. My census (`rev/wrap.py`) finds **0** printed chains with such a wrapper before a `LastCreated` placement out of 114 placements. The positive control fires on wrapper nodes with nested token payloads: ChooseOneOf 25, RollDie 10, Vote 6, FlipCoin 3. |
| `CreateDelayedTrigger`, `EpicCopy`, `CastFromZone`/`Discover`/`Cascade` | The payload resolves in a later, separate resolution. | Non-member (state it). |
| `GiftDelivery`, `Incubate` | They create tokens but call neither writer (no `last_created` write in `gift_delivery.rs` / `incubate.rs`; Incubate's pause post-action `counters.rs:700-720` does not write either). | Non-member. `Incubate` is already fixture A7. |

**Consequence 1: the zero-parse-diff derivation rests on the incomplete list.** `census_members.py … DraftFromSpellbook` reports **45 cards, 14 with a following node**. Several of those follow-ups read the conjured card through `ParentTarget`:
- Blood Age Muster: "conjure a random card from Blood Age Muster's spellbook onto the battlefield. Its base power and toughness perpetually become 2/2." This lowers to `DraftFromSpellbook` → `ApplyPerpetual { target: ParentTarget }` under a target-less trigger.
- Brood Astronomer: "draft a card from the Planets Spellbook and put it onto the battlefield tapped". This lowers to `ChangeZone { ParentTarget }`.
- Saint Elenda: "You may cast that card this turn …". This lowers to `CastFromZone { ParentTarget }`.

The same follow-up after a plain `Conjure` already lowers to `LastCreated`: 6 chains of `Conjure` → `ApplyPerpetual{LastCreated}` (e.g. Indris, the Hydrostatic Surge) and 1 of `Conjure` → `PutCounter{LastCreated}` (Hangarback Assembler). So admitting `DraftFromSpellbook` very likely moves `chain_prior_referent_is_created_token` and/or `bare_they_player_declaration` on these cards. That is a **non-empty** phase-1 parse diff, and the plan has not derived it. It is also a correctness move: `ParentTarget` with no declared target probably binds nothing at runtime today. That is unprobed, since no build was allowed. Dazzling Flameweaver and Kayla's Kindling read `TrackedSet` and probably do not move. `Endure` has 12 cards and 0 with a following node, so it plausibly contributes no parse diff and no AI change.

**Consequence 2: the AI and gate censuses exclude the missing members.** `census_ai.py` and `census_gate.py` hard-code the 15-name set. I re-ran both and reproduced the plan's figures: 51 paired / 0 changes, and gate 0 hits, with control `Token` = 9 lines. They have not been run with `DraftFromSpellbook` or `Endure` included.

**Required revision.**
1. Re-derive membership with this predicate, which replaces the §Step 3.5 command as the durable regenerator. In non-test `crates/engine/src/game/**`, an `Effect` is a member when either of these holds:
   - its dispatch arm (`effects/mod.rs` `resolve_effect`) reaches one of the following, directly or through a synthesized `ResolvedAbility::new(Effect::<member>…)` / `AbilityDefinition::new(…, Effect::<member>…)` that it resolves in the same resolution:
     - an assignment or push to `last_created_token_ids`;
     - `record_last_created_(copy_batch_)token`;
     - `publish_face_down_entry_referent` (via `.publishing_chain_referent()`);
     - `conjure::resolve`;
   - or it resolves an `AbilityDefinition` payload inline through `resolve_ability_chain`.

   The synthesizer sweep I ran over non-test `effects/*.rs` finds exactly these: `copy_token_blocking`, `each_player_copy_chosen`, `encore`, `endure`, `investigate`, `myriad`, `populate`, `spellbook`. The plan's table covers all but `endure` and `spellbook`.
2. Admit `DraftFromSpellbook` and `Endure` to `publishes_created_referent`, or show from the resolver that either one fails the criterion.
3. Decide each payload wrapper (table above) as (a) or (b), with the reason in the method's doc.
4. Derive the parse-diff rows for every `DraftFromSpellbook` card with a following node, card by card from its Oracle text, as the plan did for the other members. Do the same for the wrappers under option (a).
   - Where a reading flips, name the runtime evidence for the new reading.
   - If that evidence needs a test file outside phase 1's scope rule, say so, so the orchestrator can record the scope extension as an addendum line. Admitting a member the D1.1 criterion already defines breaks no charter decision.
5. Extend P2 and A3 with the new members, and re-run `census_ai.py` / `census_gate.py` with the full member set.
6. Replace the snapshot counts "15 publisher variants", "six measured writers", "6 runtime writers" and "gains eight variants" with the durable form ("every row of the D1.1 table", "every row marked **no** in 'Today in parser set?'"). See F-5.

### F-2 `behavior`: `is_token_creator` is a wildcard classification, and nothing forces an answer for a new token creator

The plan specifies `Effect::is_token_creator` as `matches!(self, Token | CopyTokenOf | Populate)`, which is an implicit `_ => false`. It justifies this as "Exhaustiveness lives on the superset; any new token creator must first answer the superset question." That does not hold. A future token-creating variant gets a compile error only in `publishes_created_referent`. Its author adds it to the `true` arm, and `is_token_creator` silently answers `false`.

That answer is load-bearing: `is_token_creator` keys `resolve_populated_token_anaphors` (`lower.rs:3874`), the token-cleanup gate (`lower.rs:5139`), `publishes_tracked_set_from_resolution` (`oracle_effect/mod.rs:35704`), `classify_bare_card_aggregate_publisher` (`mod.rs:35801`), the goaded arm, and phase 2's union (D2.1). The caller's focus and CLAUDE.md ("exhaustive match without wildcard", "never a raw bool") require the token-creator answer to be forced at the same site.

**Required revision.** Make one wildcard-free `match` the single authority and derive both public predicates from it. For example: a private `fn created_referent_publication(&self) -> Option<CreatedReferentPublisher>` with `enum CreatedReferentPublisher { TokenCreator, Other }` (`None` = not a publisher). Then `publishes_created_referent` is `.is_some()` and `is_token_creator` is `== Some(TokenCreator)`. The two charter-mandated method names are kept (D1.1), so no charter decision moves. Update §Rust Idioms, §Extension vs Creation ("Creates: nothing new beyond the two methods"), §Variant Discoverability, and the U-A registration-surface cell in Sizing. This is a new private type, not a variant on an existing engine enum. Keep P1 as the subset-invariant test.

### F-3 `text`: the U-A acceptance-row `rg` predicate also matches existing test code

**Coordinate:** §Per-unit acceptance rows, U-A.

**Old:** "`rg -n "Effect::Token \{ \.\. \}" crates/phase-ai/src/ability_chain.rs` shows only D1.2's `Token` check"

This is already false at base. The existing test `filtering_view_drops_only_created_token_placements` contains `assert!(matches!(kept[0], Effect::Token { .. }));`, and the new A-rows will add more.

**Replacement:** "the non-test part of `crates/phase-ai/src/ability_chain.rs` (lines before its `#[cfg(test)]`) contains exactly one `Effect::Token { .. }`, in `created_token_placement_source` (`awk '/^#\[cfg\(test\)\]/{exit} /Effect::Token \{ \.\. \}/' crates/phase-ai/src/ability_chain.rs | wc -l` prints `1`)"

### F-4 `text`: the goaded-arm guard must keep falling through when the detector declines

**Coordinate:** Step-by-step plan, U-A step 3.

**Old:** "or any equivalent that compiles, keeps the arm's position, and evaluates the detector only for token creators."

**Replacement:** "or any equivalent that compiles, keeps the arm's position, evaluates the detector only for token creators, and does **not** match when the detector returns `None`, so the effect falls through to the later arms. Those include `Effect::Token { .. } => parse_token_source_power_toughness_followup(&lower)` (`sequence.rs:9341`) and the arm commented 'Must be checked before the Effect::Token arm' (`sequence.rs:9314`). A guard of the form `effect if effect.is_token_creator() => try_parse_tokens_goaded_continuation(&lower)` is **not** equivalent: it would swallow those arms for every `Token`."

(The existing arm already uses an `if let` match guard, so the plan's suggested form compiles under the crate's edition-2021 setting.)

### F-5 `text`: the plan states two different counts for the parser-missed members

**Coordinates:**
- §Step 2 trace 2: "(the parser list already misses six measured writers, see D1.1 membership)"
- §Maintainer-simulation matrix, row "F4 (implicit)": "6 runtime writers the parser list missed are admitted"
- §Expected parse diff: "`publishes_created_referent` gains eight variants"

The D1.1 table marks **eight** rows "no". Replace each of the three figures with "every D1.1 row marked **no** in 'Today in parser set?'". After F-1, the set and its size change again; the durable form survives that.

---

## Sizing (check 12, blocking in this context): consistent

- **Units.** There are 3 (U-A, U-C, U-E), or 5 if U-E is split, as the plan and the charter both say. The body has three independently tested behaviours: the U-A predicates and pairing (A2/A3/A4/A6), the U-C guard (C1/C5/C6), and the Wild Hypothesis guard (E1/E1b). C2 and C4 carry no test by construction. No under-count. T1 holds.
- **Paths.** The 9 counted paths match the step list one for one:
  - `types/ability.rs`, `lower.rs`, `sequence.rs`, `ability_chain.rs`, `game/effects/mod.rs`, `game/triggers.rs`, `oracle_trigger.rs`, `created_token_counter_sequence.rs`;
  - `types/zones.rs`, comment-only and charter-conditional.
  - Worst case is 10, with `x_reference.rs`.

  This matches the charter's recursive check (definite 8 + `x_reference.rs` + `zones.rs` ≤ 10). The import removal in `effects/mod.rs` is in a file already counted. T2 is false.
- **F-1's possible extra test file** (runtime evidence for a flipped `DraftFromSpellbook` reading) would add at most 1 path plus the standing-class `main.rs` mod line. That stays below 13, but the revised plan must recount. A refresh of the `cargo ai-gate` baseline files, if it happens, is correctly routed back to the orchestrator.

## Verified (no finding)

- **Token-creator set unchanged.** `is_token_creating_effect` is `Populate | Token | CopyTokenOf` (`lower.rs:3883-3888`), and the goaded arm lists the same three (`sequence.rs:9300`). Delegation therefore changes no output from the token-creator consumers, as the plan claims.
- **Claimed members confirmed against the resolver code:**
  - `Token`: `token.rs:1230`, `:660`.
  - `CopyTokenOf`: `token_copy.rs:237`, `:555`, batch drain.
  - `Populate`: `populate.rs:47→64→88` into `token_copy::resolve`.
  - `Manifest`, `ManifestDread`, `Cloak`: `morph.rs:837` `.publishing_chain_referent()`, `zone_pipeline.rs:4089-4090`, and `begin_face_down_referent_production` in `manifest.rs:41` / `manifest_dread.rs:19` / `cloak.rs:50`.
  - `Conjure`: `conjure.rs:279`.
  - `CopyTokenBlockingAttacker`: `copy_token_blocking.rs:87`.
  - `EachPlayerCopyChosen`: `each_player_copy_chosen.rs:251/268`.
  - `Investigate`: `investigate.rs:19/39`.
  - `Amass`: `amass.rs:237`, which writes only when the Army is created.
  - `EmpowerJace`: `empower_jace.rs:67`.
  - `Myriad`: `myriad.rs:57`.
  - `Encore`: `encore.rs:85`.
  - `CreateTokenCopyFromPool`: `create_token_copy_from_pool.rs:165` and `token::no_token`.
  - The counter-pause continuations (`counters.rs:786/851/948`) and `engine_replacement.rs:2506-2509` belong to the `Token`/`CopyTokenOf` entry pipeline.
  - The `attach.rs`, `delayed_trigger.rs`, `suspect.rs`, `populate.rs:273` and `zone_pipeline.rs:7165` writes are all in test modules.
- **Claimed non-members confirmed:** `Incubate` (and its pause post-action), `Meld`, `ChangeZone` (`ChainReferentIntent::Publishes` is set only at `morph.rs:837` and `engine_resolution_choices.rs:2312`, both manifest).
- **AI pairing.**
  - `created_token_placement_source` currently does `rposition(Token)` (`ability_chain.rs:114-123`), so A2, A3, the third A4 row and A6 do fail at base, as the plan derives.
  - The existing rows `[Investigate, Token, P] → Some(1)` and `[Token, Mill, P] → Some(0)` stay green under the new rule.
  - The `Token` fixture `token_effect(0,0,vec![])` has no `enter_with_counters`, so A2's `ability_creates_token_with_plus_one_counters` verdict really is decided by the pairing. It is not degenerate.
  - `x_reference.rs::effect_matches_in_walk` consumes the pairing, so not touching that file is correct.
- **F3 guard.**
  - The rename has exactly 2 call sites (`effects/mod.rs:20124` recursive, `:21073` in `evaluate_condition`'s `Not` arm), and nothing else in the repo references the name.
  - The top-level gate that C-rows reach is `resolve_ability_chain`'s `if let Some(ref condition) = ability.condition { if !evaluate_condition(..) }` (`effects/mod.rs:16404-16405`). That is a production entry point, and the sub-only deferral at `:18641-18654` does not intercept a root condition.
  - C1, C5 and C6's absent leg fail at base: the `QuantityCheck`/`QuantityQuality` arms return `false` for an absent referent, and `Not` inverts that.
  - C7 is supported by the `CostPaidObject` presence ladder (`quantity.rs:2181-2185`: `cost_paid_object || effect_context_object || event source`).
  - C5b correctly separates value-0 from absent.
  - Corpus: `census_not.py` re-run finds only Ravenous Necrotitan as an `AbilityCondition::Not(QuantityCheck)`, with no resolution-only scope.
- **C4.** `positive_count_gate_qty` has no `EventTime` arm today (`oracle_trigger.rs:3068-3085`). The plan's comment matches the `TriggerCondition::EventTime` doc (`types/ability.rs:31185-31202`: attack-while gate, count qualifier, CR 603.8 state trigger).
- **C2.** The test `quantity_quality_rechecked_at_resolution` is at `triggers.rs:35201`. Its production counterpart `zimone_all_questioning.rs:224 resolution_recheck_and_live_count` includes the negative recheck (b).
- **E1.** The `SurveilChoice` + `SelectCards` payload is the keep-on-top set (`engine_resolution_choices.rs:4721-4735`), so `cards: vec![]` sends both cards to the graveyard. The scenario API exists: `commit`, `resolve_top`, `act`, `counters`, `graveyard_count`. The derived reading (Fractal 2/2 with 2 counters; graveyard 3; library 1; X=0 Fractal gone only after resolution, CR 704.3/704.5f/111.7) agrees with the Oracle text and the CR.
- **CR citations.** Every number the plan cites exists in `docs/MagicCompRules.txt` at the line it gives: 101.4, 111.1, 111.2, 111.7, 111.10f, 122.6, 603.4, 603.8, 608.2c, 608.2e, 608.2f, 608.2h, 609.3, 616.1, 700.1, 701.16a, 701.25a, 701.40a, 701.47a, 701.57c, 701.58a, 701.62a, 702.116a, 702.141a, 704.3, 704.5f, 707.2. Moving the PR-added `QuantityQuality` arm and its test comment from CR 701.57c (Discover-specific) to CR 608.2c is correct. Optional: cite CR 701.36a for `Populate` in the token-creation sub-group, alongside CR 111.1.
- **Scope rule.** Every step stays inside the charter's phase-1 anchors. The `types/ability.rs` inline test module tests the `impl Effect` predicates. Leaving `x_reference.rs` untouched is allowed. Deferrals are phase-attributed: `DEFERRED(phase 2)` for the driver union, `DEFERRED(phase 3)` for Oversimplify, and `DEFERRED(phase 4)` for coverage.

## Pre-existing (untagged; does not block; not repeated next round)

- `crates/engine/src/game/morph.rs:829-833`: "they are exactly the three effects the parser admits as chain-referent producers (`oracle_effect::lower::publishes_chain_created_referent`)". This is false at base: the parser also admits `Token`, `CopyTokenOf`, `Populate` and `Conjure`. It is comment-only and outside phase 1's scope rule. It joins the plan's existing note on `chain_prior_referent_is_created_token`'s stale doc.

## Residual assumptions (unprobed; the executor closes them)

- That admitting `DraftFromSpellbook` actually moves Blood Age Muster, Brood Astronomer and Saint Elenda's parse. I inferred this from the `Conjure` analogue; no build was allowed.
- The plan's own §Unprobed items 1–7 stand.
