# Charter review r1: etb-review-fixes (rf-charter-r1.md)

Reviewer: fresh `review-engine-plan`, **charter mode**. Phase-fit context declared, so Sizing consistency blocks.
Inputs: `rf-charter-r1.md`, the original task (`pr10036-review.md`), the draft `rf-plan-r1.md` and its review `rf-plan-review-r1.md` (findings F1–F11 are constraints), and the attempt history (plan loop round 1 was a design round with b=7, t=4; the gate fired mid-Step-2; this is charter round 1).
Base: `addbc884`. I measured `git diff --stat e56851e addbc88 -- crates/`, which is empty, so engine sources are identical to the draft's base.
Evidence: code reads at HEAD; `card-data.json` (p2); `AtomicCards.json`; `docs/MagicCompRules.txt`. No cargo build was run, as the caller instructed.

**Verdict: design round.** 1 decision finding (`behavior`, MED), 9 corrections (`text`, LOW), 0 `machinery`.

---

## Decision findings

### D1 `behavior` MED: phase-2 decision 2 freezes "gets" as unknown onset, which the CR settles as after-entry

**Coordinate.** Phase 2, decision 2, third bullet (line 207):

> `Continuous` splits by the verb its dispatch matched: gains gives `AfterEntry`, has gives `SimultaneousWithEntry`, and gets or any other verb gives `None`.

The Goal says the same thing more narrowly: "A 'becomes/gains' rider applies after entry".

**Rules.**
- CR 611.2e is an exception. Only a continuous effect that says the permanent "is [characteristic]", "has [characteristic]" or "doesn't have" one applies at the same time as the entry. Its last sentence spells out the after-entry reading for "becomes" and "gains", but nothing in it makes any other verb simultaneous.
- Outside that exception, CR 608.2c applies: instructions are followed in order. The effect a later instruction creates does not exist yet when the entry event happens.
- CR 603.10 checks triggers against objects as they exist right after the event.
- So a "gets +N/+N" rider created by the instruction after the entry applies **after** entry, the same as "gains".
- PR finding C1 asks for exactly this: "defer only for a rider that applies at entry". Because `None` keeps today's deferral, a "gets" node keeps the blanket behavior C1 reports.

**Printed evidence (measured).**
- Macabre Mockery reads: "Put target creature card from an opponent's graveyard onto the battlefield under your control. It gets +2/+0 and gains haste until end of turn. Sacrifice it at the beginning of the next end step." (MTGJSON.)
- Its card-data chain is `ChangeZone > GenericEffect{AddPower 2, AddToughness 0, AddKeyword Haste} > CreateDelayedTrigger`. That is a direct entry → D1 chain.
- Under the frozen mapping, the predicate head is "gets", so the node is `None` and still deferred. The entry observation is taken after the +2/+0.
- Example: return a 2-power creature with Elemental Bond on the battlefield. The engine still draws. The correct result is no draw, because the creature enters at 2 power.
- This is the same wrong-reading class as Kardur's III, Hunting Wilds and Rampaging Growth. The charter fixes those three and leaves this one wrong by decision.
- Census: in `rf/c1detail.json` over the 334 enter → D1 cards, Oracle verbs after a pronoun subject include "gets" on 4 cards and "loses" on 7. Macabre Mockery is the gets case that is a real chain.

**Why this is a decision finding.**
- Decision 2 is frozen architecture. "gets → None" forecloses the rules-correct onset, and R3 would carry it as a permanent residual.
- F1's supplied mapping put gets/other in `None` as a conservative placeholder, without a CR derivation. Revising that mapping leaves F1's architectural resolution intact: the onset still comes from typed dispatch, with no fragment classifier.

**Required revision (decision text).** Replace the line 207 bullet with:

> - `Continuous` takes its onset from the verb its dispatch matched, derived from CR 611.2e's exception scope plus CR 608.2c / CR 603.10. "has/have" and "doesn't have" give `SimultaneousWithEntry`. "gains" and "gets" give `AfterEntry`. The 611.2e simultaneity is limited to its named forms, so any later-instruction effect applies after the entry event. A verb the phase plan cannot derive from the CR keeps `None`, with the derivation gap recorded (for example "loses": CR 611.2e names "doesn't have", not "loses"). A node whose stamped parts are all `AfterEntry` ("gets … and gains …") is `AfterEntry`. Decision 3's mixed rule covers only Simultaneous-with-AfterEntry mixes.

Also replace the Goal's second bullet:
- Old: "A "becomes/gains" rider applies after entry, so it no longer suppresses settlement."
- New: "A rider the CR places after entry (every dispatched verb outside CR 611.2e's is/has/doesn't-have forms: becomes, gains, gets) no longer suppresses settlement."

The phase plan then carries a Macabre Mockery row through its own matrix. The row is not written into the charter (see C1 below).

---

## Corrections (orchestrator applies; each `text`, LOW)

### C1: acceptance rows and phase-established claims written into the charter (Architecture-only rule)

Predicate: a charter sentence that specifies a test row, evidence class or delta report that a phase must produce. Command: read every decision, the finding-ownership table and the seam notes. Every matching site and its disposition:

| Line | Old text (start) | Disposition |
|---|---|---|
| 148 | "The decision rejects "live-path semantics unchanged" … Each changed class needs a derivation, and at least one record-skeleton path needs a sibling row or a measured zero-reach statement." | Replace with: "The decision rejects "live-path semantics unchanged" as an unestablished blanket claim (constraint: rf-plan-review-r1 F5)." |
| 158 | "The semantic claim is runtime (CR 614.1c), so the phase's discriminating evidence is a required runtime row, not parse shape alone (F9)." | Replace with: "The semantic claim is runtime (CR 614.1c) (constraint: rf-plan-review-r1 F9)." |
| 213–216 | "The phase plan establishes, for each D1 kind (…), which producers stamp it, and covers three cases with rows: …" plus its three sub-bullets | Replace with: "The phase plan establishes, for each D1 kind (`GenericEffect`, `Animate`, `ReturnAsAura`), which producers stamp it (constraint: rf-plan-review-r1 F1's required rows)." |
| 224–232 | Decision 6, "Required evidence classes (constraints on the phase plan's matrix, F2): …", through "Which base values are red is a claim the phase plan establishes, not this charter." | Delete decision 6. Add to the phase-2 entry: "Constraints on the phase plan's matrix: rf-plan-review-r1 F1, F2, F10, and this review's D1." |
| 258 | F2 cell: "decision 6: a direct-chain row (Rampaging Growth with Soul Warden); the phase plan corrects the printed-reach sentence" | "phase-2 matrix constraint (F2's required revision, verbatim from rf-plan-review-r1)" |
| 259 | F3 cell: "decision 2: scratch-carrier controller semantics; a phase-1 unit case with owner ≠ entering controller, covering `DevotionGE` and an `ObjectCount` comparison" | "decision 2: scratch-carrier controller semantics; the phase-1 matrix carries F3's required unit case" |
| 260 | F4 cell: "the phase plan isolates the ruling-(b) observer row … stays the positive guard" | "phase-1 matrix constraint (F4's required revision, verbatim from rf-plan-review-r1)" |
| 265 | F9 cell: "decision 6: a required runtime C4 row (…)" | "decision 6; phase-1 matrix constraint (F9's required revision)" |
| 288–290 | "Coverage honesty." with its two sub-bullets ("It reports its coverage and semantic-audit delta." / "Its plan shows a zero supported/unsupported delta, or reports and explains one.") | Keep the first clause of each ("Phase 1 changes one lowering (C4): already-accepted text, newly folded." / "Phase 2 adds metadata to already-parsed clauses."). Delete the reporting sentences, because coverage-status impact is a check-9 matrix column in each phase plan. Keep "No phase accepts text with deferred semantics." |

The deferral list's row names (line ~173, "every C1 runtime and parse row (…)") are a deferral attribution, which is allowed. No change there.

### C2: B2 decision 4 contradicts its own invariant for an off-battlefield snapshot (line 141)

- Line 141 says: "An off-battlefield object keeps today's off-zone effective path."
- Two lines later the decision says: "for every caller, the kind-level answer comes from the same object as that caller's `WithKeyword` answer."
- Today's off-zone path is id-keyed (`off_zone_has_keyword_kind(state, object_id, kind)`, `off_zone_characteristics.rs:362`). `WithKeyword` reads the held object (`obj.has_keyword`, `filter.rs:8891`).
- The damage-record source skeleton is a held object with a **non-battlefield** zone: `filter.rs` ~5416, `GameObject::new(…, record.source_zone)`, where the zone is `Stack` for a spell. For that skeleton the two sentences give different objects.
- Measured reach: 43 card-data cards carry `HasKeywordKind` / `WithoutKeywordKind`, and none sits on a trigger `valid_card` / `valid_source` path. The census reached all 43 (positive control). So no printed wrong reading exists; the problem is only the text.
- Replacement for line 141: "A battlefield-zoned object answers from its own keywords. An off-battlefield object that is the live object of its id keeps today's off-zone effective path. A non-live off-battlefield snapshot (for example the damage-record source skeleton stamped with its damage-time zone) takes the disposition the phase plan derives under the invariant below."

### C3: the C6 acceptance is attributed to the wrong reviewer (line 162)

- `pr10036-review.md` records C6 as a CodeRabbit nitpick, with no PR-reviewer response.
- The acceptance came from the plan reviewer (rf-plan-review-r1, answer 3).
- Old: "The PR reviewer accepted this disposition."
- New: "The plan review (rf-plan-review-r1, answer 3) judged this disposition proportionate for a nitpick."

### C4: code-state and snapshot assertions

Each item below is either measured or not load-bearing. Rewrite each in its durable form.

| Line | Old | Status | New |
|---|---|---|---|
| 156 | "The existing fold and the sequence-level swallow recognizer consume the same converter, so they follow it." | Measured true. `oracle_effect/mod.rs:43291` and `sequence.rs:8319` both call `anaphoric_entry_counter_gate`, which ends in `card_type_condition_as_entry_this_way`. The `lower.rs` fold destructures `ZoneChangedThisWay { filter, .. }` and does not constrain the filter's shape. Phrase it as a decision. | "The converter stays the single authority that both the fold and the sequence-level swallow recognizer consume." |
| 201 | "copied at both def → resolved conversion sites" | Cardinality snapshot | "copied at every def → resolved conversion site that copies `sub_link`" |
| 197 | "No protocol-version bump is expected for a `serde(default)` optional field." | Phase-established claim | "Whether the protocol policy requires a version bump is a claim the phase plan establishes." |
| 311 | R2: "… line parses to `Unimplemented "enters_with_gate"`, in both single-type and multi-type forms." | Code-state statement on a triaged issue | "… (measured in rf-plan-r1 P4; tracked by the filed follow-up issue)." |

### C5: class-attribution card names (line 92) are not real gate consumers as written

- MTGJSON's "Oviya Pashiri, Sage Lifecrafter" has no entry gate. Its abilities only create tokens.
- The card-data gate consumer is **Oviya, Automech Artisan**. Its rider, "If you put an artifact onto the battlefield this way, put two +1/+1 counters on it", is folded into `conditional_enter_with_counters`. That fold is itself a pre-existing misreading (see Pre-existing).
- "Winter Soldier" is ambiguous. The gate consumer is **Winter Soldier, Reborn Avenger** ("If a Hero enters this way, …").
- Replacement for the parenthetical: "(Recommission, What Must Be Done, Heroic Return, Winter Soldier, Reborn Avenger, Summoner's Grimoire)".

### C6: CR list (line 66, and one addition)

- **CR 109.5.** Old: ""you" on a static ability means the current controller of its source; for a resolved effect it is the effect's controller."
  - New: ""you" on an object means its controller. For a static ability, that is the current controller of the object it's on. A resolution-created effect's "you" is the controller of the spell or ability that created it."
  - Reason: CR 109.5 has no "for a resolved effect" clause. The transient reading follows from its first sentence.
- **Add CR 205.1b** under "CR rules the decisions rest on": "an effect that says an object is "still a [type]" retains prior types. It is part of the type-changing effect, not a separate characteristic statement." Phase-2 decision 3 (the retention tail) rests on this rule. I verified it at line 1404.

### C7: the read-only-anchor sentence contradicts chartered.md's scope rule (line 117)

- chartered.md §Scope rule: an out-of-list file that is not in a standing class "returns to the phase's plan step, so a planner and its reviewer admit it before any executor edits it".
- Old: "Editing a read-only anchor is drift and gets an addendum. Standing inclusion classes are implied."
- New: "Editing a read-only anchor returns to the phase's plan step for admission (chartered.md §Scope rule). The admitted scope is then sized as drift: an addendum when every decision stands, a re-charter when one breaks. Standing inclusion classes are implied."

### C8: the phase-1 re-charter route names one trigger and a split that can fire again (line 303)

- The conjunction is re-adjudicated whenever a revised plan returns, at scope-freeze, and at any extension. A standing-class extension is only one of these.
- The proposed 1a (B1 + L1/C5) has 2 units. If B1's own scope growth is what fired the conjunction, 1a trips T1 ∧ T2 again. That is a second-level firing, which stops the run.
- L1/C5 must precede only the rows in `etb_trigger_entry_time_characteristics.rs`. B1's rows live in `conditional_entry_gate_projection.rs`.
- Old: "If a standing-class extension brings phase 1's T2 to ≥ 13, the conjunction fires. The proposed split is 1a = B1 (with L1/C5 first) and 1b = B2/C2, C3, C4. The units are independent, so either order is green."
- New: "If any re-adjudication of phase 1 (its plan's Sizing, scope-freeze, or a scope extension) brings T2 to ≥ 13, the conjunction fires. The proposed split makes the unit whose scope growth fired it a single-unit phase 1a, so T1 cannot fire there. The remaining units form 1b, with L1/C5 first. The units are independent, so either order is green."

### C9: a phase-2 scope entry that is not a literal path (line 195)

- Old: "plus insta snapshots the stamp changes."
- New: "plus the insta snapshot directories `crates/engine/src/parser/oracle_effect/snapshots/`, `crates/engine/src/parser/oracle_ir/snapshots/` and `crates/engine/src/parser/snapshots/` (test fixtures, excluded from T2)."
- All three directories exist at base.

---

## Checklist judgments (charter mode)

- **Seams and order.** Phase 1 (B1, B2/C2, C3, C4, L1/C5 and the C6 comment) comes before phase 2 (C1).
  - I re-derived the only ordering edge, the hardened harness under phase-2 rows, and it holds.
  - No logic edge exists. Phase 2's rows (Rampaging Growth, Kardur's III, Hunting Wilds, is-form) use no entry gate, and phase 1's rows use no D1 node.
  - The C4 runtime row (F9) does not touch D1: after the fold no sub node remains.
  - Both seams leave the tree green: phase 1 changes no types, and phase 2 adds a `serde(default)` field.
- **Out-of-bounds lists versus compiler-forced sites (measured).**
  - Phase 2 lists phase-1 files as out of bounds. A standing class never overrides that, so I checked them.
  - `filter.rs`, `layers.rs`, `keywords.rs`, `conditions.rs` and `change_zone.rs` contain no `sub_link:` literal, no `ParsedEffectClause {` literal and no `let AbilityDefinition {` / `let ResolvedAbility {` destructure.
  - So no compiler-forced phase-2 site falls inside a phase-1 out-of-bounds file.
- **Recursive T1 ∧ T2.**
  - Phase 1: 5 units, about 8 paths. It does not fire.
  - Phase 2: 1 unit, about 25 paths. It does not fire. With the `ParsedEffectClause` literal fan-out (`text_change.rs`, `imperative.rs`, `lower.rs`, `test_support.rs`), the "about 25" may read 26–30. That does not change the verdict, because T1 = 1. Sizing is consistent with the bodies.
  - Phase 2's 2a/2b route is a valid T3 seam: infrastructure first, then the consumer.
- **B1, phase-1 decisions 1–3.**
  - Pinning population and origin-zone leaves to their pre-entry truth agrees with CR 614.12 and Heliod ruling (a).
  - **Controller (F3), measured:**
    - `DevotionGE` and `QuantityComparison` use the `controller` argument; `ObjectCount` builds its "you" from `controller` (`quantity.rs` `resolve_ref`).
    - `IsPresent` names its player through `ConditionContext.ability_holder`, and otherwise reads the source's controller **in the state passed**, which pre-entry is the owner (`layers.rs` ~2121–2127).
    - So decision 2's pair (the scratch-carrier controller plus `ability_holder` set to that player) is both necessary and sufficient for those leaves.
    - `condition_controller` (`layers.rs:1573`) separates transients (effect controller) from printed statics (source controller), as the decision states.
  - A constant-truth representation exists: the `StaticCondition::None` arm evaluates `true` (`layers.rs` ~2621), so decision 3's "no new variant" stands.
  - `BattlefieldEntrantProjection::new` has exactly one production caller, `change_zone.rs:1652` (`entry_gate_projection`). The observation is not built from it, so the pin cannot leak into the CR 603.10 view.
- **B2 (decision 4).** The object-scoped authority with a per-caller disposition answers F5. C2 above fixes the one internal contradiction.
- **C3 / C4 / L1 / C6.** These are rules-correct. CR 400.7 for C3. CR 614.1c with `TypeFilter::AnyOf` (`ability.rs:6287`) for C4.
- **C1 (phase-2 decisions 1–5), apart from D1.**
  - Onset from typed dispatch: `PredicateAst::{Become, Continuous}` are produced in `subject.rs` and lowered to `ParsedEffectClause` at `oracle_effect/mod.rs` ~27250.
  - Become → AfterEntry. A retention tail stays AfterEntry (CR 205.1b). Perpetual is never stamped (F10).
  - D1 is "kind ∧ onset ≠ AfterEntry", as an exhaustive match. Both walkers (`resolved_chain_…`, `definition_chain_…`, `effects/mod.rs:4809/4823`) read node fields, and the field lives on both structs.
  - Unstamped producers stay conservative: for example the Assimilate lowering, whose `imperative.rs:14999` CR 611.2e deviation comment applies.
- **New enum and field (`/add-engine-variant`).**
  - Stage 1: `rg "enum .*(Onset|Timing)|SimultaneousWithEntry|AfterEntry|WithEntry" crates/engine/src/types` finds only the unrelated `ExileCastTiming`, `TargetChoiceTiming` and `CombatDamageTiming`. `data/engine-inventory.json` is not generated, and the charter assigns that gate to the phase plan.
  - Stage 2: no sibling cluster. Stage 3: one CR section (611.2e).
  - Node-level placement mirrors `sub_link`.
  - No TS binding exists: `AbilityDefinition` has no `ts`/`tsify` derive, and the client has no `sub_link` reference. So no generated artifact joins T2.
  - The new type is justified.
- **Deferral allowlist.**
  - Phase 1: `DEFERRED(phase 2)` for all C1 content, with a named landing phase. Phase 2: none.
  - R1–R7 are run-level residuals. R3 shrinks under D1.
  - Nothing the original task needs is unattributed. Every item is attributed: B1's five-devotion control and observer row, B2/C2, C3, C4, C5/L1, C6, and C1's production regression.
- **Premise.** Step 0 is present.
  - I re-verified every quoted card against MTGJSON: Heliod, Recommission, What Must Be Done (mode "Release Juno"), Rusted Relic, Starfield of Nyx, Soul Warden, Elemental Bond, Kardur's III, Hunting Wilds, Rampaging Growth, Rise from the Grave, Path of the Schemer, Call a Surprise Witness, Dragon Tempest and Emergent Sequence. All match.
  - The only name errors are in the class attribution (C5).
- **CR numbers.** I grep-verified every cited rule in `docs/MagicCompRules.txt`: 614.12, 614.12a, 614.1c, 700.5, 109.5, 110.2a, 613.1, 611.2e, 611.3c, 603.10, 603.10a, 603.6a, 608.2c, 608.2i, 400.7, 122.6, plus the added 205.1b. The CR 109.5 paraphrase needs C6's correction.
- **No file inventory.** The scope rules use literal anchor paths and one directory, `oracle_ir/`, with no globs. C9 fixes the one non-literal entry. Step 3 defers to the scope rules.
- **Addenda files.** Each phase names its own.

## Non-blocking notes (for the phase-1 plan)

- **Multi-entrant "pre-entry".** For a simultaneous multi-object entry, CR 614.12's "continuous effects that already exist" excludes every member of the event, not only the entrant under projection. The phase plan should define the pinned state as the state before the event, or state that no gate consumer is multi-entrant. Every gate consumer in C5's list returns a single target.
- **Mixed leaves.** A population leaf that also reads the source's own state (a `QuantityComparison` mixing an `ObjectCount` with the source's power) would be pinned wholesale, against the source's pre-entry (graveyard) state. Add this to R5 alongside the recipient-context limit, or establish that no gate consumer reaches it.
- **`ability_holder` doc.** `ConditionContext.ability_holder`'s doc says "`None` everywhere else" (`layers.rs:1394`). The pin is a new legitimate use (the holder differs from the source's pre-entry controller), so the doc comment needs to name it.

## Pre-existing (untagged, non-blocking)

- **Oviya, Automech Artisan.** Its rider, "If you put an artifact onto the battlefield this way, put two +1/+1 counters on it.", is a later instruction (CR 608.2c), not an "enters with" replacement (CR 614.1c). Card-data folds it into the ChangeZone's `conditional_enter_with_counters` (`[Typed Artifact, P1P1, 2]`). So an ETB observer sees the counters: a 1/1 artifact creature with Elemental Bond draws, where the correct result is no draw. No row the change asserts depends on it.
