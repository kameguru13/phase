# Phase 1 plan review r2 (run zimone-r2)

Mode: `review-engine-plan`, **phase-plan mode**. Phase index 1 of 4. `PHASE_BASE_SHA` = `8949699a13a541a7e00ada2bb8e16c00c530b82a`, and `git rev-parse HEAD` agrees. Artifact: `p1-plan-r2.md`, reviewed in full and not only as a delta.

Inputs read:
- the frozen `phase-charter` (phase 1 entry, scope rule, D1.1–D1.6, deferral list, seam notes);
- `addenda/phase-1` (empty);
- `phase-fit` (entries 1–14);
- `p1-plan-review-r1.md`;
- CLAUDE.md, `review-engine-plan/SKILL.md`, `engine-implementer/SKILL.md` (§Run limits, §Phase-fit gate, Step 2) and `chartered.md`.

**Phase-fit context declared: the Sizing consistency check (check 12) is blocking.**

**Method: read-only.**
- No repo edits, no cargo build, no stash, checkout or restore.
- Probes were `grep`/`sed` reads of the source at `8949699`, plus independent static censuses over `data/card-data.json`. Each census has a positive control.
- I wrote the censuses myself in `/tmp/claude-0/-home-user-phase-personal/3ffacad5-7db0-5e02-a1a7-192aa01f873f/scratchpad/rev2/`:
  - `carriers.py`: which `Effect` variants carry a nested `AbilityDefinition` or `Effect`.
  - `followers2.py`: the nodes that follow `DraftFromSpellbook` and `Endure`.
  - `wrap2.py`: inline-payload wrappers that publish, and their followers.
  - `ai2.py`: AI pairing, old rule against new rule.
  - `grant_census.py`: perpetual-grant parses that use the Tome body.
  - `dump.py`: card dumps.
- Each was run with `python3 -I`.
- I did not run the planner's `p1r2/` scripts. One invocation was refused by the sandbox, so I re-derived each figure independently instead.

**Result: 0 `behavior`, 1 `text`.** This round has no `behavior` finding, so it closes the loop once the orchestrator applies the `text` finding.

---

## Prior-round findings

| r1 finding | Status at r2 | Evidence |
|---|---|---|
| F-1 `behavior` (publisher membership) | **Resolved** | See below. |
| F-2 `behavior` (one wildcard-free classification) | **Resolved** | See below. |
| F-3 `text` | **Applied** | §Per-unit acceptance rows carries the `awk` predicate verbatim. Control at base: `awk … \| wc -l` prints `1`, while `grep -c "Effect::Token { .. }"` prints `2` (the second is the test-module hit). |
| F-4 `text` | **Applied** | Step-by-step U-A step 3 carries the fall-through replacement verbatim. The existing arm (`sequence.rs:9300-9304`) already uses an `if let` match guard (toolchain `nightly-2026-04-19`, crate edition 2021), so the suggested form compiles. |
| F-5 `text` | **Applied** | Lines 95, 583 and 670 use the durable form "every D1.1 row marked **no** in 'Today in parser set?'". A sweep for `six `/`eight `/`15 publisher`/`6 runtime`/`N variants` finds no remaining snapshot count. |

**F-1, resolved.**
- `DraftFromSpellbook` and `Endure` are admitted.
- The eight wrappers are decided by option (a), with a recorded rationale.
- Each wrapper's non-membership boundary is stated.
- The 14 `DraftFromSpellbook` followers and the one wrapper follower are derived card by card.
- P2/A3 are extended, the censuses re-run, and the snapshot counts reformulated.

**F-2, resolved.**
- The single authority is the private `fn created_referent_publication(&self) -> Option<CreatedReferentPublisher>`, wildcard-free with no `_ =>` arm.
- `is_token_creator` is `== Some(TokenCreator)`, which is node-only.
- `publishes_created_referent` is the `OwnResolutionOnly` walk over `.is_some()`.
- A new variant must therefore choose `TokenCreator`, `OtherProducer` or `None` at the one site both predicates read. A new payload carrier must make the walker's own wildcard-free descend-or-leaf decision.

---

## Findings

### T-1 `text`: the Hinterland Chef row cites corroboration its references cannot give

**Coordinate:** §Expected parse diff, table row "Hinterland Chef", column "Site that could see the member".

**Old:** "Every printed "It perpetually becomes" card is `unbound_subject` regardless of its antecedent (Kavaron Consumed, Pep, Turbocharged Escape, Veko, Woodland Investigation)."

**Why it is wrong.** I listed every chain node whose description starts "It/That card perpetually becomes". There are 9: Glimmer Hoarder, Hinterland Chef, Kavaron Consumed, Pep, Support Skyforge, Thought Partition, Turbocharged Escape, Veko and Woodland Investigation.

The node before each cited reference is `ChangeZone`, `ExileTop`, `TargetOnly`, `ChangeZone` or `Shuffle`. None is a created-referent publisher. So none of them has `token_created_in_chain` set, and they cannot show "regardless of its antecedent" for the flag-set case. That is the only case this row's flip could change.

The row's verdict is still correct. It rests on the two code reasons the same cell gives, both verified at `8949699`:
- the "it perpetually " arm fails on `alt(gains/gain).ok()?` before `counter_anaphor_created_token_binding` is reached (`oracle_effect/mod.rs:12828-12834`);
- `subject.rs:3669-3673`'s flag check sits behind `chain_declared_object_target == Some(Typed(_))`, and Hinterland Chef's draft declares no target.

No other perpetual arm matching "perpetually become(s)" reads `ParseContext`. The base-P/T arms require "base power and toughness", and `try_parse_perpetual_base_pt` takes no context.

**Replacement:** "The printed "It/That card perpetually becomes" cards (Kavaron Consumed, Pep, Turbocharged Escape, Veko, Woodland Investigation, Glimmer Hoarder) are all `unbound_subject`, but none has a created-referent antecedent. They corroborate the subject path only, not the flag-set case. That case rests on the two code reasons above and on the measured parse diff."

---

## Sizing (check 12, blocking in this context): consistent

- **Units.** There are 3 (U-A, U-C, U-E), or 5 with U-E split, as the plan and the charter say.
  - The body's independently tested behaviours are: the U-A classification (P1–P5) and its AI pairing consumer (A-rows), the U-C guard (C-rows), and the Wild Hypothesis guard (E1/E1b).
  - A reviewer who splits U-A into "classification" and "pairing" gets 4 or 6. T1 holds under every count.
- **Paths.** The 9 counted paths match the step list one for one:
  1. `types/ability.rs`
  2. `oracle_effect/lower.rs`
  3. `oracle_effect/sequence.rs`
  4. `phase-ai/src/ability_chain.rs`
  5. `game/effects/mod.rs`
  6. `game/triggers.rs`
  7. `parser/oracle_trigger.rs`
  8. `tests/integration/created_token_counter_sequence.rs`
  9. `types/zones.rs` (comment-only)

  The worst case is 10, with `x_reference.rs`. T2 is false, so NOT(T1∧T2) holds.
- **No path was added.** R2–R4 land in `created_token_counter_sequence.rs`, an anchor path that is already a `mod` in `tests/integration/main.rs:254`. The P-rows are a new inline `mod` in `types/ability.rs`, inside the "impl Effect classification predicates" anchor.
- **Registration surfaces.** The row is "2 `pub fn` + 1 private `fn` + 1 private enum". It matches Rust Idioms, Extension vs Creation and step 1. No variant is added to an existing enum, and nothing is serialized or exported.

## Verified (no finding)

### Publisher classification against resolver code

- **Writer census, independent.** Non-test writers of `last_created_token_ids`, or `record_last_created_(copy_batch_)token` / `.publishing_chain_referent()`, are confined to:
  - `token.rs`, `token_copy.rs`, `conjure.rs:279`, `copy_token_blocking.rs:87`;
  - `morph.rs:837/868-885`, `engine_resolution_choices.rs:2312` (manifest dread);
  - the per-frame `engine.rs:7010` clear.

  Their non-test callers are exactly the plan's rows: `amass.rs:237`, `investigate.rs:39`, `copy_token_blocking.rs:73`, `encore.rs:85`, `populate.rs:88`, `create_token_copy_from_pool.rs:165`, `myriad.rs:57`, `empower_jace.rs:67`, `spellbook.rs:136`, `engine_resolution_choices.rs:3384`, and the `replacement.rs:4913/4970` appended batches inside the `CreateToken` pipeline.
- **Synthesizer sweep.** I grepped `ResolvedAbility::new` / `AbilityDefinition::new` building any member effect. Hits:
  - `each_player_copy_chosen:251`, `endure:54`, `investigate:19`, `populate:72`, `spellbook:120`;
  - the Populate choice arm (`engine_resolution_choices:3723`);
  - debug-only `engine_debug.rs`.

  This is the plan's set.
- **`DraftFromSpellbook` is a member.** The random path goes `spellbook::resolve` → `complete_draft` → `conjure::resolve`, which assigns the slot.
  - The choice path sets `WaitingFor::SpellbookDraft`. That waiting state is in the auto-stash list (`effects/mod.rs:4707`).
  - On submit, `SubmitSpellbookDraft` (`engine_resolution_choices.rs:7876-7898`) runs `complete_draft` before `finish_with_continuation`.
  - Grouping it with `Conjure` matches `moves_card_to_or_from_library`.
- **`Endure` is a member, on its token branch only.**
  - `endure.rs:54-68` builds a synthesized `Effect::Token` branch.
  - `prompt_next` is called with `continuation: None`, so the tail is auto-stashed through `ChooseOneOfBranch` (`mod.rs:4689`).
  - On the token branch, `resolve_branch` runs `resolve_ability_chain` (`choose_one_of.rs:270`).
  - The counter branch writes nothing, the conditional-write shape the plan documents.
- **Refuted non-members:**
  - `ChooseAndSacrificeRest`: its only post-action is `ContinuePlayerScopeSacrifice` (`choose_and_sacrifice_rest.rs:707`, `counters.rs:645`), which writes nothing.
  - `Incubate`: `create_object` directly (`incubate.rs:38`). It bypasses the `CreateToken` pipeline and makes no slot write, including on the paused path.
  - `GiftDelivery`: `create_gift_token`, then `push_committed_token_entry_events` (`token.rs:2313-2329`), which records the zone change and emits `TokenCreated` only.
  - `ExileFromTopUntil`: it resolves its own `sub` tail (`exile_from_top_until.rs:293/346/354`). That tail is the ability's `sub_ability` chain, which the AI and parser scans classify node by node. It carries no nested payload.
- **Payload carriers, independent.** `carriers.py` (positive control: 33 carrier variants found) adds only boundary or later-resolution carriers beyond the walker's descend set: `CreateDrawReplacement`/`CreatePlaneswalkReplacement`, `EpicCopy`, `CastFromZone`, `ExileResolvingSpellInsteadOfGraveyard`, `GrantCastingPermission` and `PayCost`.
  - `PayCost`'s cost is a leaf in the walker. That is safe: `AbilityCost::supports_effect_cost_payment` (`types/ability.rs:15264`) admits only `PutCounter{SelfRef}` and fixed `Mana`, so no resolution-time cost can publish.

### Walker as the building block (CR boundaries)

- `visit_effect_scoped(_, OwnResolutionOnly, _)` visits the node first, so every node-level member answers immediately.
- It descends exactly the inline payloads: `Vote` (`per_choice_effect` and `outcome_template`), `SeparateIntoPiles`, `RevealFromHand.on_decline`, `FlipCoin`/`FlipCoins`/`FlipCoinUntilLose`, `RollDie.results` and `ChooseOneOf.branches`.
- Each wrapper resolver runs its payload in the same resolution:
  - `roll_die.rs:598`, `flip_coin.rs:129`, `reveal_from_hand.rs:164`;
  - `separate_piles.rs:395`, `vote.rs` `resolve_tally` (depth 1);
  - `choose_one_of.rs:270`, after appending the continuation.
- The walk stops at the CR 603.7a/603.3/611.2/614.1/114.1/111.1 boundary carriers (`ability_visit.rs:625-813`), and prunes `WhenYouDo` nested defs (CR 603.12, `scope_prunes_nested_ability`).
- Nested defs' costs pass through `visit_cost_scoped`, which yields no publisher, as above.
- The `debug_assert` tripwires are unreachable from an `Effect` root under this scope.
- Visibility: the walker is `pub(crate)`, and `mana_abilities.rs` already imports it from the same crate.

### Parse diff zero

- **Followers, independent.** `followers2.py` finds:
  - `DraftFromSpellbook`: 45 nodes, of which 14 have a follower. They are exactly the plan's 14 cards.
  - `Endure`: 12 nodes, 0 with a follower.
  - `wrap2.py` (control: ChooseOneOf 25, RollDie 10, Vote 6, FlipCoin 3, FlipCoins 2, FlipCoinUntilLose 1) finds one publishing wrapper with a follower: Done for the Day → `PutSticker`.
- **Parser call sites of the publisher predicate.** There are exactly four: `mod.rs:26816` (`chain_prior_referent_is_created_token`), `mod.rs:27048` (`bare_they_player_declaration`), `lower.rs:3979` (relink) and `lower.rs:4444` (clone-transplant).
- **`token_created_in_chain` readers.** They are `counter.rs:116/156` (and their callers `counter.rs:305/331/1367`, `imperative.rs:7302`, `mod.rs:12868/13148/19109`), `imperative.rs:7277`, `mod.rs:12912` and `subject.rs:3673`. Nothing outside `oracle_effect/` reads it, and no cast parser does, which confirms the Saint Elenda row.
- **Spot checks:**
  - **Tome of the Infinite.** The flag flips, so the "it perpetually " arm now binds `LastCreated`. It then fails, because the grant body "You may spend mana as though it were mana of any color to cast this spell." never classifies.
    - Corpus: 32 nodes carry that body, and 0 parse to `ApplyPerpetual`.
    - Several of them reach the body through the `"that "` arm, which returns `ParentTarget` with **no** context read: Futurist Spellthief, Ominous Traveler, Grave Choice. That is flag-independent evidence the body fails, and stronger than Bind to Secrecy alone. Optional: cite Futurist Spellthief beside Bind to Secrecy on the Tome row.
    - After the arm returns `None`, the remaining path is flag-independent for Tome: `subject.rs`'s flag arm needs a declared typed target, and Tome declares none. So `unbound_subject` stands.
  - **Blood Age Muster.** `try_parse_perpetual_base_pt` takes no `ParseContext` and returns `ParentTarget` (`mod.rs:12114-12175`). Every `ParentTarget`→`LastCreated` rewrite is reached only through `resolve_populated_token_anaphors` or the token-cleanup gate, both keyed on the unchanged token-creator set (`lower.rs:4668/4691/5139-5156`).
  - **Adaptive Armorer and Masterpiece Vault.** `unsupported_zone_change_reflexive` returns `zone_change_reflexive_event_source` because `publishes_event(DraftFromSpellbook, EnterBattlefield)` is false (`zone_change_reflexive.rs:30-46, 206-208`), before any body or referent read.
  - **Support Skyforge.** The "that " arm requires "gain(s)", and `anaphor_is_chain_created_demonstrative` does not include "that card" (`counter.rs:122-137`).
  - **Oyaminartok.** The relink finds the ungated draft and `continue`s.
- **AI census, independent.** `ai2.py` finds 114 `LastCreated` placements, of which 51 are base-paired (the control), and **0** changes under the r2 rule. This reproduces the plan.

### One classification drives both predicates

The predicates are specified as derived from `created_referent_publication`. The acceptance row "`created_referent_publication` has no `_ =>` arm" makes this checkable. The AI pairing is `rposition(publishes_created_referent)`, then a `Token` check, which yields `Some` only for `Token` (D1.2).

### R2/R2b/R3/R4

Each is a two-sided runtime discriminator of the D1.1 premise:
- the produced object has the counter, the Soldier has none, and a reach-guard checks the produced object exists;
- an empty spellbook, or a draft of an unknown name, fails the reach-guard instead of passing vacuously.

They reach production. Each runs `engine::game::effects::resolve_ability_chain` (`pub`, `effects/mod.rs:14954`, already used by integration tests). R2b, R3 and R4 then answer the pause with `GameRunner::act`. The actions are `SubmitSpellbookDraft { card }` and `ChooseBranch { index }`, and both shapes exist (`types/actions.rs:464, 484`).

They are correctly labelled `control`. The phase changes no runtime code they exercise; they measure the reference reading D1.1 rests on. The revert-discriminating rows for the change are A2, A3, the third A4 row, A6, A10a and A10c, as the Sizing table says. The fallbacks for the pause and faceless-object cases are named.

### CR citations

Every number in the plan's table exists at the line it gives in `docs/MagicCompRules.txt`. I re-grepped all 44: 101.4 … 707.2, including 603.3/2588, 603.12/2662, 608.2d/2801, 700.3/3229, 701.36a/3610, 701.38a/3624, 701.55a/3765, 701.63a/3833, 705.2/5587 and 706.2/5599. Each is applied to the rule it describes. The digital-only arms (Conjure, DraftFromSpellbook, EmpowerJace) correctly carry no CR.

### Scope

Every step stays inside the charter's phase-1 anchors:
- In `effects/mod.rs`, the changes are only the import removal and line-level switch in the declined-gate walk, plus the guard and its inline tests.
- In `sequence.rs`, only the goaded arm changes.
- The `morph.rs`/`mod.rs` stale docs are correctly routed as pre-existing.
- Deferrals are phase-attributed: `DEFERRED(phase 2)`, `DEFERRED(phase 3)` and `DEFERRED(phase 4)`.

## Residual assumptions (unprobed, because no build was allowed; the executor closes them)

1. The measured parse diff is zero cards, including Tome of the Infinite, Hinterland Chef and Done for the Day. The plan routes a non-empty diff back to the plan step.
2. The pause and resume sequences of R2b, R3 and R4 under `resolve_ability_chain` on `state_mut()`, and the conjured object accepting a counter. Fallbacks are named.
3. **The declined-gate walk widens for the class "if-you-do gate → newly admitted publisher → rider".**
   - The walk algorithm is unchanged, and it reads the authority the P-rows test.
   - `census_gate2`-style scans (and the plan's) find no printed member.
   - No test pins the widened verdict. That is acceptable for a consumer of an unchanged algorithm, but the executor should keep the existing declined-gate inline tests green.
4. The plan's own §Unprobed items 1–8 stand.
