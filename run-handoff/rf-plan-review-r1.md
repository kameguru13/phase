# Plan review r1: etb-review-fixes (rf-plan-r1.md)

Reviewer: fresh Step 2 `review-engine-plan`, ordinary mode, phase-fit context declared, so check 12 (Sizing) blocks.
Base: e56851e (repo HEAD at addbc88; addbc88 is a client-only merge, so engine files match e56851e).
Evidence: code read at HEAD, card-data `/home/user/wt/data-out/p2/card-data.json`, MTGJSON `AtomicCards.json`, and `docs/MagicCompRules.txt`. I compiled no probe. Every claim below is a quoted code or data read that settles its question without running anything.

**Verdict: design round.** 7 `behavior`, 4 `text`, 0 `machinery`.

---

## Blocking findings

### F1 `behavior` HIGH: the C1 onset classifier rebuilds verb detection from fragment text and misses the "X is …" form (C1 step 3, V14, V15)

**Evidence.**
- Step 3 says: if the fragment is not `it's`/`they're`, take `find_predicate_start(text)` and then `alt(is / are / isn't / … / becomes / gains)` on the remainder.
- `find_predicate_start` (`parser/oracle_effect/subject.rs:8389`) stops only on `token_starts_predicate`, which is `is_restriction_predicate_verb || PREDICATE_VERBS.contains(normalize_verb_token(token))`.
  - `normalize_verb_token("is")` returns `"be"` (`parser/oracle_effect/mod.rs:48478`).
  - `"be"` is not in `PREDICATE_VERBS` (`subject.rs:8199-8287`, grep count 0), and neither is `"are"`. Only `isn't`/`aren't` pass, as restriction verbs.
- So the `tag("is ")` and `tag("are ")` arms can never be reached.
- Rise from the Grave reads "That creature is a black Zombie in addition to its other colors and types." That fragment has no predicate token, so it classifies `None`. V14's assertion that the r7 D1 node carries `Some(SimultaneousWithEntry)` cannot pass. (Runtime stays green only because `None` keeps the deferral.)
- The scan also does not skip quoted text. In an is-form rider whose granted ability is quoted ("… is a … with "Whenever …, you gain 1 life"…"), the first predicate token is the quoted `gain`, so the rider classifies `AfterEntry`: a regression versus base.
- The parser already detects the verb with types. `try_parse_subject_become_clause` produces `PredicateAst::Become` (`subject.rs:264-287`). `try_parse_subject_continuous_clause` produces `PredicateAst::Continuous`, and the additive "is a … in addition" form and the copula forms have their own dispatch arms (`subject.rs:190-300`). Check 6 applies: the parser is the detector, so the plan should not duplicate it with a second text scan.

**Required revision.** Derive the onset where the predicate is dispatched, not from `clause_ir.source.fragment()`:
- `Become` gives `AfterEntry`.
- The additive-type "is … in addition", `it's`/`they're` and `has` forms give `SimultaneousWithEntry`.
- `Continuous` splits by its verb: gains gives `AfterEntry`, has gives `SimultaneousWithEntry`, and gets/other verbs give `None`.

Carry it on the parsed clause (`ParsedEffectClause`/`ClauseIr`) into the def. If a text classifier is kept instead, it must be a nom word-boundary scan that recognizes copula heads (`is`/`are`) and skips quoted text.

Add classifier and parse rows for:
- "That creature is a black Zombie…" (Rise from the Grave) giving Simultaneous;
- a quoted-verb hostile ("It's a … with "… you gain …"" / "… is a … with "… gain …"") that must not give AfterEntry;
- "Until end of turn, that land becomes …" (leading duration) giving AfterEntry.

Re-count Sizing (F6) after the redesign, since it may add parser IR paths.

### F2 `behavior` MED: C1 has no row for the largest changed class (an entry followed directly by becomes/gains), and the printed reach is understated

**Evidence.**
- The plan tests only "enter → modification → D1" (V12, V13) and says the printed defect "reaches Hunting Wilds and Kardur's Vicious Return".
- But D1 also defers settlement through a D1 node that **directly** follows the entry. `settle_entries_before_instruction` with `next` = the D1 node defers, so the observation is taken after the becomes/gains effect applies (`game/effects/mod.rs:4725-4745`, `5022-5047`).
- So every direct "enter → becomes/gains" chain observes the after-entry characteristics at base. The change flips that whole class (about 165 cards per P1), and no row covers it.
- Printed wrong reading at base: **Rampaging Growth** ("Search your library for a basic land card, put it onto the battlefield, then shuffle. Until end of turn, that land becomes a 4/3 Insect creature with reach and haste. It's still a land.")
  - Its card-data chain is `ChangeZone > Shuffle > GenericEffect`, with no Unimplemented.
  - With Soul Warden on the battlefield, base observes a creature and gains 1 life. Under CR 611.2e ("becomes … applies after the permanent is on the battlefield") the land enters as a noncreature, so the gain is 0.
  - Emergent Sequence's 2021-04-16 ruling states the same principle: "The land won't be a creature as it enters the battlefield … Abilities that trigger whenever a creature enters the battlefield won't trigger." Its card-data parse holds an Unimplemented node, so it is not a good fixture.

**Required revision.** Add a discriminating `/card-test` row: Rampaging Growth (verbatim Oracle) with Soul Warden. Assert life +0 (base +1). Reach guards: the land is on the battlefield and is a 4/3 Insect creature after resolution. Correct the Pattern Coverage / P2 printed-reach sentence to name the direct-chain class.

### F3 `behavior` MED: the B1 pin's controller argument is undefined and the plan's mechanism does not reach `DevotionGE`

**Evidence.**
- Step 1 evaluates `evaluate_condition_with_context(pre_entry, leaf, holder_controller, source_id, ConditionContext { ability_holder: (source_id == entrant).then_some(..) })`. `holder_controller` is never defined.
- `DevotionGE` reads the `controller` argument: `count_devotion(state, controller, colors)` (`game/layers.rs:2118`).
- `QuantityComparison` resolves its quantities with `controller` (`layers.rs:2146-2163`).
- Only `IsPresent` (and the whose-turn leaves) read `ability_holder` (`layers.rs:2121-2131`).
- The live pass gets its controller from `active_effect_condition_controller(state, effect)` (`layers.rs:10589`), which differs between printed statics and transients.
- If `holder_controller` is taken from the pre-entry entrant (its owner, while the card is in the graveyard), then an entrant returned "under your control" counts the owner's devotion and artifacts. That loses the CR 110.2a override the plan says it keeps.

**Required revision.**
- Define the controller for each carrier as the one the scratch layer pass would use: `active_effect_condition_controller` semantics on the scratch. For the entrant's own statics that is the post-override scratch controller; for a transient continuous effect it is the effect's controller.
- Set `ability_holder` only to make `IsPresent` / `FilterContext` read that same player.
- Add a V6 unit case with an entrant owned by player 1 that enters under player 0's control, covering `DevotionGE` and an ObjectCount `QuantityComparison`. The pinned truth must use player 0's pre-entry battlefield.

### F4 `behavior` MED: in V3, Heliod's own life-gain trigger can put a counter on Heliod

**Evidence.** Heliod, Sun-Crowned (MTGJSON) also reads "Whenever you gain life, put a +1/+1 counter on target creature or enchantment you control." In V3, Soul Warden's +1 life triggers that ability, and Heliod (an enchantment you control) is a legal target. The row's "0 counters **and** life +1" then depends on which target the harness picks. If it picks Heliod, the fixed engine shows 1 counter and the base engine shows 2. The row then no longer isolates the entry counter it claims to test.

**Required revision.** Either:
- assert Heliod's counter count after the Recommission resolution and before Heliod's life-gain trigger resolves, or
- choose that trigger's target explicitly as a permanent other than Heliod, or
- use a creature-entry observer that gains no life.

Keep life +1 (or the observer firing) as the ruling (b) positive guard.

### F5 `behavior` MED: the B2 change reaches matcher callers the plan does not list, and "live-path semantics are unchanged" is not established

**Evidence.**
- The plan switches `matches_filter_prop`'s `HasKeywordKind` / `WithoutKeywordKind` arms to read the `obj` they hold. It states that `obj` is "the view on the observation path, and the live object elsewhere".
- `grep -n "filter_inner_for_object(" crates/engine/src/game/filter.rs` lists more callers that pass a non-live object, and those answers change too:
  - the attack-declaration record skeleton (`filter.rs` ~5336: `Zone::Battlefield`, `keywords = record.lki.keywords`);
  - the counter-added record skeleton (~5296);
  - the damage-record source (~5395, `record.source_zone`);
  - the entering-token object `build_battlefield_entry_token_object` (~5239), which is absent from `state.objects`, so at base it reads `false`;
  - the face-down entering projection (~5140).
- In each case where the skeleton's zone is Battlefield, the answer moves from the live object to the snapshot. The direction matches `WithKeyword` and CR 603.10 / CR 608.2i, but the plan claims no change, so these paths have no derivation and no sibling row.

**Required revision.**
- Replace the "Live-path semantics are unchanged" bullet with a disposition for each `filter_inner_for_object` caller: live, entry observation, or one of the record/projection skeletons. Each disposition names the CR behind its snapshot reading.
- Add one sibling assertion on a record-skeleton path (for example an attack-declaration record whose live object has since lost the kind), or a measured statement that no printed card puts a kind-level filter on those paths. In card-data, kind filters are mostly off-battlefield kinds: Suspend, Flashback, Foretell, Augment, …

### F9 `behavior` LOW: the C4 runtime row (V19) is optional, so only parse shape backs the semantic claim

**Evidence.** C4's claim is runtime: the counters become part of the entry (CR 614.1c), not a later instruction. V18 is a parse-shape row, and V19 is labelled optional. Check 9 says parser shape tests do not satisfy runtime claims.

**Required revision.** Make V19 required and discriminating.
- Card: a synthetic modal bullet "Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's an artifact or creature."
- Case 1: a 2/2 artifact creature returned with Elemental Bond on the battlefield draws 1, because it enters 4/4. Base draws 0, because the later PutCounter is observed after settlement.
- Case 2: a legendary non-artifact, noncreature enchantment gets 0 counters.

### F10 `behavior` LOW: the classifier silently moves `ApplyPerpetual` riders to `AfterEntry`

**Evidence.**
- `opt(tag("perpetually "))` followed by become/gain classifies Blood Age Muster ("Its base power and toughness perpetually become 2/2") and the Karlach, Tiefling … cards and Niambi ("It perpetually gains …") as `AfterEntry`. Tend to the Kiln, if its node is stamped, is reclassified too.
- At base all of these are deferred.
- CR 611.2e does not address perpetual effects, which are digital-only Alchemy with no CR entry. No row and no derivation covers the change.

**Required revision.** State the disposition. Either keep `ApplyPerpetual` nodes `None` (base behavior, the conservative choice), or derive the AfterEntry reading under the codebase's "Digital-only Alchemy (no CR entry)" convention and add a parse row and a runtime row.

### F6 `text` (Sizing, blocking under phase-fit): the T2 count excludes standing-class paths that the counting rule does not exclude; the gate fires

**Evidence.**
- The T2 rule (`engine-implementer/SKILL.md` §Phase-fit gate) excludes only test fixtures, regenerated pipeline data and protocol-version surfaces.
- `chartered.md` §Scope rule: "Every extension re-counts T2 over the extended list" and "T2 re-adjudication at scope-freeze counts the materialized list". The materialized list admits the compiler-forced, shared-registration and comment-only classes.
- The new field on `AbilityDefinition` and `ResolvedAbility` forces these sites (`grep -rln "sub_link:" crates/engine/src`, each site checked against its file's `mod tests` line):
  - 7 product files: `game/quantity.rs`, `game/stack.rs`, `game/ability_rw.rs`, `game/text_substitution.rs`, `game/ability_scan.rs`, `game/resolution_prompt.rs`, `ai_support/shortcut_efficacy.rs`. The last two are missing from the plan's Step 2 list.
  - 9 files whose only forced site is an inline test-module literal: `game/effects/{skip_next_turn,player_counter,additional_phase,reverse_turn_order,extra_turn,double,vote,skip_next_step,grant_extra_loyalty_activations}.rs`.
- Not forced: `game/triggers.rs` (a fn parameter), and `parser/oracle_effect/mod.rs` and `parser/oracle_ir/effect_chain.rs` (fields of `AbilityShellIr`/`ClauseIr`).
- `game/effects/change_zone.rs` is comment-only and is also admitted.
- Count: 11 + 7 + 9 + 1 = **28**, or 19 if inline test-module edits are classed as fixtures. Either way T2 ≥ 13, and T1 = 6, so **the T1 ∧ T2 gate fires**.

**Replacement text.** Replace the paragraph that begins "Expected counted scope paths, excluding test files, test fixtures, regenerated card data and standing compiler-forced sites. There are **11**:" (its list items stay) and the sentence "Six units and about 10–11 counted paths is within T1 (multi-unit) and near T2 (paths)." with:

> Expected counted scope paths, excluding only the T2 exclusions (test files and fixtures, regenerated card data). Standing-class paths count, because T2 is re-counted over the materialized `SCOPE_PATHS`. Authored: the 11 paths listed. Compiler-forced by the new `AbilityDefinition`/`ResolvedAbility` field: every file `grep -rln "sub_link:" crates/engine/src` lists other than the authored paths, `types/ability.rs`, `game/triggers.rs` (fn parameter) and `parser/oracle_effect/mod.rs` / `parser/oracle_ir/effect_chain.rs` (IR shell fields). At BASE_SHA that is 7 product files (game/quantity.rs, game/stack.rs, game/ability_rw.rs, game/text_substitution.rs, game/ability_scan.rs, game/resolution_prompt.rs, ai_support/shortcut_efficacy.rs) and 9 files whose forced site is an inline test-module literal (game/effects/{skip_next_turn, player_counter, additional_phase, reverse_turn_order, extra_turn, double, vote, skip_next_step, grant_extra_loyalty_activations}.rs). Comment-only: game/effects/change_zone.rs. Total 28 (19 if inline test-module edits are classed as fixtures). T1 = 6 and T2 ≥ 13: the phase-fit gate fires. The U4-alone seam below is a single unit, so it does not trip T1 on its own.

Re-derive this count with the same commands after F1's redesign.

### F7 `text`: three Step 0 Oracle quotes are not verbatim (MTGJSON)

- Hunting Wilds row. Old: "Search your library for up to two Forest cards and put them onto the battlefield tapped. Then shuffle." New: "Search your library for up to two Forest cards, put them onto the battlefield tapped, then shuffle."
- Rusted Relic row. Old: "Metalcraft — Rusted Relic is a 5/5 Golem artifact creature as long as you control three or more artifacts." New: "Metalcraft — This artifact is a 5/5 Golem artifact creature as long as you control three or more artifacts."
- Rise from the Grave / Path of the Schemer row, and the V14 Reference Readings bullet. Old: "It's a Phyrexian in addition…" / "Path of the Schemer ("It's a Phyrexian")". New: "It's an artifact in addition to its other types." / "Path of the Schemer ("It's an artifact in addition to its other types")".

### F8 `text`: Reference Readings omits the rows whose expected value equals the base value

The positive controls V2, V4 (3 other artifacts) and V5 (5 enchantments) assert values the base engine also produces. They are not in the Reference Readings list. My own derivations agree with the plan's values:
- V2: 5 pre-entry white devotion means Heliod is a creature as it would enter, so 1 counter (ruling a).
- V4: 3 artifacts pre-entry means Rusted Relic is a 5/5 artifact creature, so 2 counters.
- V5: Starfield plus 4 others is 5 enchantments, so Sanctum is a creature, so 2 counters.

**Replacement text.** Add this bullet after the V12/V13 bullet:

> - **V2, V4 (3 others), V5 (5 enchantments) positive controls.** Derived from CR 614.12 and Heliod ruling (a), not from parity. With the condition already met pre-entry, the entrant is a creature as it would exist on the battlefield, so the gate applies: 1 counter for V2 and 2 counters for V4/V5. Base gives the same values, by its own route.

### F11 `text`: the R1 residual names the wrong observer class

R1 old: "A Bond-style observer does not care about either change, so there is no wrong printed reading for P/T-gated observers. Flag it for a type-gated observer."

R1 new: "A P/T-gated or type-gated observer reads correctly here: the Spirit type applies with the entry, and a flying counter changes no P/T. A **keyword-gated** observer reads wrongly. Dragon Tempest ("Whenever a creature you control with flying enters, it gains haste until end of turn") sees the flying counter put after entry. That is pre-existing at base, which defers the same way."

---

## Answers to the orchestrator's specific questions

1. **B1 design.**
   - Pinning population and zone-membership leaves to their pre-entry truth inside the scratch is rules-correct and in the right layer.
   - Heliod ruling (a) generalizes to any count the entrant would join ("replacement effects are considered before the God is on the battlefield"). CR 614.12's "continuous effects that already exist" excludes an effect whose condition only becomes true once the entrant is there. The Recommission ruling (2022-10-14, March of the Machines) confirms that effects which already exist apply.
   - Pinning beats excluding the entrant from counts. Exclusion would have to be threaded into `count_devotion`, `ObjectCount`, `IsPresent` and the rest, and would still miss the departure from the origin zone, which the zone clause covers.
   - Over-pinning: the population classifier is conservatively `true`, but its over-classified leaves (`IsPresent` in any zone, `Unrecognized`) also have a correct pre-entry reading. The entrant's non-population own-state leaves (`SourceEnteredThisTurn`, `SourceInZone`, tap state) are correctly left unpinned, per CR 614.12's "would apply once it's on the battlefield".
   - Open item: the controller argument (F3).
   - Residuals to document, not blocking:
     - a layer-1 copy effect copies a source's pinned `base_static_definitions` into the copy, carrying the source's controller-relative truth;
     - `Duration::ForAsLongAs` conditions of transient effects are pruned, not pinned.
2. **C1 design.**
   - The CR 611.2e fidelity of the is/has against becomes/gains split is correct. 611.2e's "nontoken" applies; for tokens, becomes/gains also apply after entry, so the AfterEntry stamp is still right.
   - A new field is warranted. No existing onset or timing enum was found (`grep "enum .*Timing|enum .*Onset|WithEntry"` in `types/`; inventory not generated), and `AbilityDefinition` already carries parse-stamped per-node metadata (`target_reads`, `declares_return_result`, `sub_link`).
   - Serde: the `AbilityDefinitionDe` mirror and the `audit_projection` key list are covered.
   - Wire: a new `#[serde(default)]` optional field needs no `PROTOCOL_VERSION` bump under `lobby-broker/src/protocol.rs`'s policy.
   - Use an inherent method on `Effect` for the D1-kind membership, not a parser import from `game::effects`.
   - The classifier is the defect (F1). The matrix is missing the direct-chain row (F2) and the perpetual disposition (F10).
3. **C3 / C6 dispositions are justified.**
   - C3 is defensive hardening at a public-in-crate seam. It follows the `battlefield_entry_observation` incarnation convention (`types/game_state.rs:1924-1928`). The lib unit test is the only reachable seam (P7, read).
   - Declining C6 (two projections per gated entrant, zero printed cards with both gates, the calls in different functions) with a doc comment is proportionate for a nitpick.
4. **Sizing.** Six units is consistent with the body (B1, B2, C3, C1, C4, L1/C5; C6 is comment-only). The 11-path count is not consistent with the T2 rule (F6). The verdict then follows: **the gate fires**. The plan's U1–U3, U5, U6 against U4 seam is a valid split, because U4 alone is one unit.
5. **Red at base.**
   - V1 is red: the existing row asserts 1 counter and passes on e56851e.
   - V4, V5, V8, V9, V11, V12 and V13 are red by code reading. V18 is red: the existing test asserts that no fold happens.
   - V3 is confounded (F4). V19 is not required (F9). The direct-chain class has no row (F2).

## Non-blocking notes

- V4: also assert that the Relic is a 5/5 artifact creature on the live battlefield after entry (the ruling-(b)-analog reach guard), alongside "on the battlefield".
- `clause_ir.source.fragment()` returns `Option<&str>` (`unwrap_or_default()` at the precedent site). This matters only if F1 keeps a text path.
- Insta snapshots under `parser/oracle_effect/snapshots` and `parser/oracle_ir/snapshots` change if any fixture contains a stamped D1 node. Expect snapshot updates.

## Pre-existing (untagged, non-blocking)

- **Call a Surprise Witness with Dragon Tempest** (or Momo, Friendly Flier, or Waterkin Shaman):
  - Text: "Put a flying counter on it. It's a Spirit in addition to its other types."
  - Base defers settlement past the counter, so a "creature with flying enters" observer fires.
  - Under CR 611.2e and CR 603.6a the counter is put after entry, so the creature entered without flying and the observer should not fire.
  - The plan keeps this (R1); no row the change asserts depends on it.
