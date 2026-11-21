# Charter review r1: removal-restrictions

Mode: `review-engine-plan` charter mode, plus `engine-implementer/chartered.md`.
Artifact: `phase-charter`. Inputs read: `task.md`, `plan-r1.md`, `class.json`, `class-drop.json`, `phase-fit.md`, and `addenda/`, which holds three empty files.
Worktree: `X:/Users/jakob/Documents/dev/phase-wt/resolved-restrictions` at HEAD `7e2a538799e672af23caca367ba938bc24194d66`. The review was read-only: no builds and no edits. Evidence comes from grep and from jq over `data/card-data.json` and `data/mtgjson/AtomicCards.json`.

**Verdict: design round.** There are 2 decision findings and 4 corrections.

---

## What was verified (no finding)

- **Step 0.5 record reproduces.** I re-ran the engine-planner template with the recorded regex and `class-drop.json` into a scratch file.
  - Versions: MTGJSON `5.3.0+20261016`, `jq-1.8.2`.
  - Results: count 427; `masked_only`, `rejected_add` and `rejected_drop` are all empty; 907 distinct rulings and 1045 ruling instances.
  - `jq -cS . | sha256sum` gives `bc328027…f208` for both the scratch output and the saved `class.json`.
  - All 18 `drop` cards are printed static abilities (I read each card's Oracle text).
- **Core premise holds on card-data.** A resolution rule effect and a quoted grant arrive with identical payloads:
  - Panic and Incite Hysteria both lower to `[{"type":"AddStaticMode","mode":"CantBlock"}]`.
  - Promise of Loyalty and Whipgrass Entangler both lower to `GrantStaticAbility{…SelfRef…}`.
  - The wrapper is not a discriminator either. Break Through the Line ("gain haste until end of turn and can't be blocked") is a `Continuous` wrapper over a bare `AddStaticMode{CantBeBlocked}`, the same wrapper shape Incite Hysteria uses.
  - A census of every resolving `GenericEffect` `AddStaticMode` (804 rows) finds exactly 18 cards whose description carries a quoted grant. That matches the draft's 18-card list.
  - None of the 18 uses a mode in the TCE gather skip list. Every other row worded "gain"/"has" is either a keyword plus a rule clause ("gain shroud … and can't be blocked") or the "as though it didn't have defender" rule effect.
  - Conclusion: the parse diff is strictly required, and §0 correction 2 is right on the merits.
- **The provenance rule is CR-correct for every shape the charter names:**
  - an unquoted resolving restriction is an effect that modifies the rules of the game (CR 611.2, CR 611.2c);
  - a quoted grant is an added ability (CR 113.10);
  - layer 6 removes abilities only (CR 613.1f), in timestamp order (CR 613.7);
  - a static-ability-sourced graft is a CR 611.3 effect, and D3 leaves that path at `Characteristic` (fail-closed).
  - Dropping the draft's V5 "U1-b" row was correct. Psychic Paper's "it can't be blocked" is not under "has", so that row would have pinned a wrong reading.
- **Code anchors exist as the charter describes them:**
  - the `RemoveAllAbilities` arm clears `static_definitions` beside the resolution-replacement retain (`layers.rs` ~13312–13326);
  - TCE gather sets `transient_id: Some(tce.id)`;
  - perpetual installs onto base characteristics, not through a TCE graft;
  - copy snapshots read `base_static_definitions`, so grafts are not copied;
  - `object_has_no_abilities` lives in `filter.rs`;
  - `ReplacementOrigin` carries no serde attributes and appears in no TS binding, so the D1 rename is wire-identical, as claimed.
- **CR numbers.** Every CR number in the charter is grep-verified in `docs/MagicCompRules.txt`:
  - 109.3 (l.592), 109.5 (l.610), 113.3d (l.767), 113.10 (l.813), 604.1 (l.2669);
  - 611.2 (l.2908), 611.2c (l.2915), 611.3 (l.2926), 613.1f (l.2976), 613.7 (l.3013), 613.11 (l.3056).
  - The quotations are accurate. "Second-to-last sentence" of 611.2c is correct.
- **Seams:**
  - P1 → P2: P1 targets parity, and its parser shape row fails at base.
  - P2 → P3: vehicles stay `Characteristic`, which is the base behavior, and the Promise row is `DEFERRED(phase 3)`.
  - Ordering U3 → U1 → U2 respects both dependency edges.
  - Each phase names its addenda file.
  - The scope rules use literal paths and directories, with no globs.
  - Deferrals out of the run (Psychic Paper #282, frozen affected-set, quoted CR 109.5 audit) each carry a reason.

---

## Decision findings

### F1 (decision). Phase 1's goal "runtime parity: no card's behavior changes" rests on an unmeasured premise that the evidence contradicts

**Where:** §3 Phase 1 Goal; D4 (perpetual clause); D9(b).

**Evidence.**
- At base, `parse_quoted_rule_static_modifications` (`grammar.rs` ~1773) collapses a quoted definition to `AddStaticMode { mode }` when `affected == Some(SelfRef) && condition.is_none() && modifications.is_empty()`.
  - Every other field of the parsed `StaticDefinition` is discarded: `attack_defended`, `active_zones`, `affected_zone`, `per_player_condition`, and the rest.
  - After phase 1, `GrantStaticAbility` keeps the whole definition. So on the layer path, phase 1 changes runtime behavior wherever the collapse threw a field away.
- Measured on card-data with real cards:
  - **Elrond of the White Council**: "…they gain "This creature can't attack its owner."" It lowers today to a bare `AddStaticMode{CantAttack}`, so the stolen creature can't attack anyone. Alexios, Deimos of Kosmos prints the same clause ("…and can't attack its owner.") and parses to `CantAttack`, `affected: SelfRef`, `attack_defended: "Owner"`, no condition, no modifications. That is exactly the shape the collapse accepts and strips.
    - After phase 1, the stolen creature may attack players other than its owner. That is the printed reading (CR 508.1c restriction scope), and it is a behavior change.
  - **Case of the Uneaten Feast** ("Creature cards in your graveyard gain "You may cast this card from your graveyard" until end of turn.") and **Kethis, the Hidden Hand** ("…gains "You may play this card from your graveyard."") lower to a bare `AddStaticMode{GraveyardCastPermission}`.
    - The printed analog Alien Symbiosis ("You may cast this card from your graveyard by …") parses to a `SelfRef` static with `active_zones: ["Graveyard"]`.
    - So the quoted inner definitions probably carry graveyard zones that the collapse drops (CR 113.6 zone of function). Phase 1 would change their functioning.
- Not measured: a parser probe of Elrond's and Case's quoted inner definitions. That needs a build, which the no-build constraint excludes. The inference is from the printed analogs, which use the same grammar (`shared.rs` `AttackTargetFilter::Owner` tag).
- **Perpetual path.** D4 says the `TryFrom` maps a "*trivial*" `GrantStaticAbility`, but the charter does not define trivial.
  - The draft (§7 step 2) defines it as "every other field default". That is stricter than the base collapse predicate.
  - Under that definition, a perpetually granted quoted static whose inner definition carries a non-default field is rejected and becomes `Effect::Unimplemented`. That drops coverage, against §4's "no card changes supported coverage status".
  - Likely candidates: Mine Security and Sanguine Soothsayer ("It perpetually gains "You may pay {0} rather than pay this spell's mana cost.""), which are cost-modifying statics that function from a non-battlefield zone.
- Charter mode requires a design finding when a frozen goal rests on a code-state assertion.
  - If P5 finds non-parity, the charter has no route for it. It is not among the §6 re-charter triggers. D4's anchors clause covers installer anchors only, not discarded definition fields.
  - So the first phase plan would hit a broken goal.

**Required revision.** The planner restates the goal and D4 so that phase 1 is "representation change; layer-path readings follow the full quoted definition; perpetual path exact parity". Proposed text:

- Phase 1 Goal. Old:
  > - **Goal.** Lower every quoted static grant to `GrantStaticAbility`, never `AddStaticMode`, with runtime parity: no card's behavior changes in this phase. After this phase, a resolving-effect `AddStaticMode` always means a rule effect, which is the input D3 relies on.

  New:
  > - **Goal.** Lower every quoted static grant to `GrantStaticAbility`, never `AddStaticMode`. After this phase, a resolving-effect `AddStaticMode` always means a rule effect, which is the input D3 relies on. Runtime parity holds for every quoted card whose definition the base collapse reproduced in full. Where the collapse discarded a field of the quoted definition (the phase plan measures which cards; the candidates are Elrond of the White Council's `attack_defended` and the graveyard-permission quotes of Case of the Uneaten Feast and Kethis, the Hidden Hand), the card's reading is derived from its verbatim Oracle text and the CR and asserted by a runtime row. Each such card is one line in `addenda/phase-1`. The perpetual path keeps exact parity (D4). No card changes supported coverage status.
- D4 perpetual bullet. Old:
  > - The perpetual `TryFrom<ContinuousModification> for PerpetualGrantModification` maps a *trivial* `GrantStaticAbility` to the perpetual `AddStaticMode` and keeps rejecting every other `GrantStaticAbility`, so it stays fail-closed.

  New:
  > - The perpetual `TryFrom<ContinuousModification> for PerpetualGrantModification` maps a `GrantStaticAbility` that meets the base collapse predicate exactly (`affected == SelfRef`, no `condition`, empty `modifications`; no other field is consulted) to the perpetual `AddStaticMode { mode }`, and keeps rejecting every other `GrantStaticAbility`. The perpetual path is therefore unchanged, and it stays fail-closed.
- D9(b): replace "it is a representation change with a parse diff and runtime parity" with "it is a representation change with a parse diff, runtime parity except for the cards the Goal names". Keep the rest. See C3 for the remainder of D9.

### F2 (decision). D6's replacement half is a pre-existing defect outside the task, and it makes phase 2 a two-unit phase that the charter predicts is ≥13 paths

**Where:** D6 (replacement half), §6 recursive check ("Each phase is one unit").

**Evidence.**
- At base, `object_has_no_abilities` (`filter.rs:12174`) tests `obj.replacement_definitions.is_empty()`.
  - It does this regardless of this change. D2's retain touches only statics, because resolution replacements are already retained at base (`layers.rs` ~13323).
  - So the replacement half reproduces at base, independent of the change.
- No acceptance criterion depends on it:
  - AC1 and AC2 concern restrictions under ability removal.
  - AC3's sibling census covers "other consumers reading restriction statics".
  - Under "Review the change", it belongs in a Pre-existing list. The draft planner came to the same view and deferred it (plan-r1 §9 Q4).
- It is independently tested (its own Maze of Ith row), has its own base defect, and has its own drop-by-addendum clause. Those are the marks of a separate unit under the unit anchor and check 12.
- The statics half differs. It is coupled to D2: without it, D2 flips the Grizzly Bears witness (Panic, then Dress Down, under Muraganda Petroglyphs) from 4/4 to 2/2. So it stays inside U1.
- §6 itself expects phase 2's compiler-forced `StaticDefinition` literal sites to push T2 to ≥13.
  - The phase-2 plan reviewer (check 12 is blocking in phase-plan mode) can count the replacement half as a second unit, giving T1 ∧ T2.
  - That is a second-level firing, which chartered.md resolves as a stop for reassessment.
  - So a scope addition the task did not ask for puts the run's convergence at risk.

**Required revision (recommended).**
- Move the replacement half to §7 as an out-of-run follow-up. One-line reason: "pre-existing (reproduces at base independent of this change: `object_has_no_abilities` counts resolution-installed replacements); no acceptance row depends on it; witness Maze of Ith's resolved prevention shield on an attacking Grizzly Bears under Muraganda Petroglyphs."
- Reduce D6 to the statics half. Keep its existing *Reason (statics)*, and delete *Reason (replacements)* and the "establishes the replacement-half defect" sentence.
- Alternative, if the planner keeps it: count it as a unit in §6 and show that phase 2 does not trip T1 ∧ T2, or split it into its own phase.

---

## Corrections (orchestrator applies; old → new)

### C1. Venser ruling attributed to the wrong ability (§1, ruling bullets)

The bearing ruling is about Venser's −1 ("Creatures can't be blocked this turn."). The emblem is "Whenever you cast a spell, exile target permanent." The ruling reads: "Venser's second ability doesn't lock in what it applies to … including … creatures that have lost all abilities."

- Old: `  - Venser, the Sojourner: an emblem rule effect covers creatures that lost all abilities. Phase 2.`
- New: `  - Venser, the Sojourner: its −1 ability ("Creatures can't be blocked this turn.") is a rule effect that applies even to creatures that have lost all abilities. Phase 2.`

### C2. AC4 acceptance is now recorded (§0 correction 2, §8.1)

`phase-fit.md` line 5 records Jakob #222: "'Keep going' with the bigger fix (accepts the bounded parse diff ~67 cards, 3 steps, 2-3 PRs). AC4 superseded by his answer."

On the merits, the charter handles AC4 well:
- The diff is strictly required, as verified above.
- It is bounded to two named shapes.
- It is measured run-level under Step 5.
- AC3's "no parser change unless strictly required" is satisfied, not overridden.

Apply the following if #222 stands as recorded. If the separate ask to the user changes the answer, re-apply with that answer.

- §0, line 44. Old:
  > 2. **The "engine-only, no parse diff" premise (task AC3 "no parser change unless strictly required", AC4 "Parse diff: none") is superseded by measured evidence.** The draft's card-data measurement …

  New:
  > 2. **The "engine-only, no parse diff" premise (task AC4 "Parse diff: none") is superseded by measured evidence, and Jakob accepted the bounded diff (Command Zone #222). AC3's "no parser change unless strictly required" stands: the parser changes below are the strictly required ones.** The draft's card-data measurement …
- §8 item 1. Old:
  > 1. **Parse diff.** The task (AC4) expected none. This charter ships a bounded parse diff (§4). Per the phase-fit record, Jakob was told the scope grew, but his acceptance of a parse diff in place of "none" is not recorded. Get it before phase 1's executor dispatch, or record that the measured evidence supersedes the premise under the task's own "unless strictly required" clause.

  New:
  > 1. **Parse diff.** The task (AC4) expected none. Jakob accepted the bounded parse diff (Command Zone #222, phase-fit record); §4 states its expected shape. A measured diff outside that shape is drift on D4/D5, sized as an addendum or a re-charter.

### C3. D9 misreads #213 and contradicts #222 (D9, §8.2)

- `task.md` says Jakob chose (#213) "to ship this as its own engine PR, ahead of the Immortal Obligation PR". That means separate from #9608, not necessarily a single PR.
- #222 then accepted "3 steps, 2-3 PRs".
- Shipping is not a frozen charter decision: chartered.md's list is phases, order, seams, goals, architecture decisions, deferrals and addenda. D9 itself says a split is "not a re-charter". So this is a wording correction, not a design round.
- On the merits:
  - Reason (c) holds. The stacked PR needs phase 3's builder, which needs phase 2's field and retain, which needs phase 1 (D8).
  - Reason (b) is weakened by F1: phase 1 now carries real reading changes.

- D9. Old (the whole bullet and its sub-bullets, lines 115–121):
  > - **D9. Shipping: all three phases accumulate into one PR** … That would be a shipping change only, not a re-charter.

  New:
  > - **D9. Shipping: one commit (at least) per phase on branch `engine/resolved-restrictions-survive-ability-removal`; the PR grouping is Jakob's call, made at run-level acceptance.** He accepted "3 steps, 2-3 PRs" (Command Zone #222) after choosing (#213) that this ships as its own engine PR ahead of #9608. Every phase boundary is a green-tree seam and a valid PR boundary. Candidate groupings: {P1+P2, P3}: P1+P2 delivers AC1/AC2 for the `AddStaticMode` class, and P3 lands the vehicle builder the stacked Immortal Obligation PR needs. Or {P1, P2, P3}. The stacked PR (#9608) rebases onto the last of them. Build-backed verification and the parse-diff measurement stay run-level (base to final) under Step 5. Changing the grouping is never a re-charter.
- §8 item 2. Old:
  > 2. **Single PR (D9)** goes against the per-phase PR option. Confirm with Jakob if the size matters to him. Phase 1 is the fallback split point and needs no re-charter.

  New:
  > 2. **PR grouping (D9)** is confirmed with Jakob at run-level acceptance against his "2-3 PRs" (#222); it needs no re-charter.

### C4. Claims a phase must establish, and acceptance-row detail, belong to the phase plans (architecture-only rule)

These blocks list the measurements and acceptance rows each phase must produce. Charter mode classes them as phase-plan content, so the repair is to delete them. The load-bearing ones already appear in the decision text: D2, D3, D4 anchors, D6, D7, §0 correction 2's "Phase 1 must re-establish this at its base", and §1's "Phase 1 regenerates … phase 3 regenerates". The §6 re-charter triggers stay.

- §3 Phase 1: delete the block from `- **Phase 1 establishes, at \`PHASE_BASE_SHA\`:**` through `  - that no card changes supported coverage status.` (lines 144–148). With F1 applied, the Goal carries the parity and coverage obligations.
- §3 Phase 2 Goal: delete the three "Examples:" sub-bullets (lines 155–157) and the trailing ` Examples:` on line 154.
- §3 Phase 2: delete the block from `- **Phase 2 establishes, at \`PHASE_BASE_SHA\`:**` through `  - the protocol assessment.` (lines 175–183). The protocol decision stays in Constraints (line 173).
- §3 Phase 3: replace the block from `- **Phase 3 establishes:**` through `  - for any vehicle whose inner definition carries \`modifications\` (draft: Streetwise Negotiator), its behavior under ability removal, derived from the card.` (lines 202–204) with the single line `- **Census.** The phase plan regenerates the vehicle list (§1) and derives, from the card, the reading under ability removal of any vehicle whose inner definition carries \`modifications\`.` That keeps the D5 scope rule's census input without stating outcomes.
- Line 8: after C4, the sentence "Every statement below about code behavior is labelled either "draft measured" or "phase plan establishes"" still holds for D2, D3, D6 and D7, so it can stay.

---

## Pre-existing (untagged, non-blocking)

- Psychic Paper's static-line `AddStaticMode{CantBeBlocked}` graft is cleared by a later "lose all abilities" effect (Command Zone #282). The charter already defers it with a reason.
- If F2 is applied: `object_has_no_abilities` counts resolution-installed replacements as abilities (Maze of Ith's shield under Muraganda Petroglyphs).

## Residual notes for the phase-plan reviewers (not charter findings)

- **D3 hazard, phase 2.** `effects/become_copy.rs` (`CopyExceptionOperation::classify`) treats a copy-exception `AddStaticMode` as `Layered`, which means it is applied through the copy effect's TCE.
  - D3 would stamp such a graft `Resolution`, although a copy exception's ability is a copiable ability (CR 707.9b).
  - Today, no printed card reaches it. A card-data census of copy effects carrying `AddStaticMode` finds only Firion, Wild Rose Warrior, which uses `CopyTokenOf`, and `token_copy.rs` routes that to base statics.
  - After phase 1, quoted exceptions become `GrantStaticAbility`, which is folded.
  - Phase 2's D3 establishment should still name copy-effect TCEs explicitly (become-copy and merge's `CopyValues` TCE).
- **Phase 3 hostile.** Whipgrass Entangler is the quoted half of the premise pair that D3's inner-declared clause must still remove. It is the natural runtime hostile for phase 3, next to Promise of Loyalty.
