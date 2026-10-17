# Phase charter: PR #10036 review fixes (entry projection population, observation-scoped keyword kinds, typed CR 611.2e onset)

- **Run:** `etb-review-fixes`, branch `engine/etb-entry-snapshot`. Run-level `BASE_SHA` for implementation: `addbc884ee251794a64f2e1da0487cd329f8ce90`. The draft plan was written at `e56851e`; `addbc88` only adds an upstream client-file merge, so engine sources are the same at both. Budget: standard.
- **Run root:** `.git/engine-implementer-runs/etb-review-fixes/`. This charter is recorded at `<run-root>/phase-charter`.
- **Mode:** `engine-planner` charter mode, charter loop round 1.
- **Input shape:** a draft plan (`rf-plan-r1.md`, not yet clean) plus the plan round-1 review findings (`rf-plan-review-r1.md`: 7 `behavior`, F1 F2 F3 F4 F5 F9 F10; 4 `text`, F6 F7 F8 F11). The phase-fit gate fired mid-Step-2, so this charter is **derived** from the draft. It does not partition reviewed material. Every review finding is a constraint either on a decision below or on the phase plan that owns it (see the finding-ownership table). Each phase plan passes its own plan-review loop.
- **Firing context:** T1 = 6 units (B1, B2/C2, C3, C1, C4, L1/C5; C6 is comment-only). T2 = 28 counted scope paths per F6, or 19 if inline test-module literals are classed as fixtures. Both are at or above their thresholds, so the conjunction fires.
- **Scout precedent pack:** none was produced for this run (phase-fit record, entry 1).

## Surviving steps (charter mode)

### Step 0: premise

Every card was re-checked verbatim against `/home/user/wt/data-root/mtgjson/AtomicCards.json` for this charter.

- **Heliod, Sun-Crowned:**
  - "As long as your devotion to white is less than five, Heliod isn't a creature."
  - Its third ability: "Whenever you gain life, put a +1/+1 counter on target creature or enchantment you control." This is the F4 confound.
  - Rulings 2020-01-24 (a) and (b) are as the PR review quotes them.
- **Recommission:** "… If a creature enters this way, it enters with an additional +1/+1 counter on it."
- **What Must Be Done:** the mode is named **"Release Juno"**, not "Let the Past Return" as the draft's Step 0 says. That is a correction the phase-1 plan applies. The mode reads: "Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature."
- **Rusted Relic:** "Metalcraft — This artifact is a 5/5 Golem artifact creature as long as you control three or more artifacts." This is F7's wording.
- **Starfield of Nyx:** "As long as you control five or more enchantments, each other non-Aura enchantment you control is a creature in addition to its other types and has base power and base toughness each equal to its mana value."
- **Soul Warden:** "Whenever another creature enters, you gain 1 life."
- **Elemental Bond:** "Whenever a creature you control with power 3 or greater enters, draw a card."
- **Kardur's Vicious Return, chapter III:** "Return target creature card from your graveyard to the battlefield. Put a +1/+1 counter on it. It gains haste until your next turn."
- **Hunting Wilds:** "Search your library for up to two Forest cards, put them onto the battlefield tapped, then shuffle. / If this spell was kicked, untap all Forests put onto the battlefield this way. They become 3/3 green creatures with haste that are still lands." This is F7's wording.
- **Rampaging Growth:** "Search your library for a basic land card, put it onto the battlefield, then shuffle. Until end of turn, that land becomes a 4/3 Insect creature with reach and haste. It's still a land." This is F2.
- **Rise from the Grave:** "… That creature is a black Zombie in addition to its other colors and types."
- **Path of the Schemer:** "… It's an artifact in addition to its other types." This is F7.
- **Call a Surprise Witness:** "… Put a flying counter on it. It's a Spirit in addition to its other types."
- **Dragon Tempest:** "Whenever a creature you control with flying enters, it gains haste until end of turn."
- **Emergent Sequence:** "… That land becomes a 0/0 green and blue Fractal creature that's still a land. …" It is a ruling witness only, not a fixture (F2).

The premise holds. The review's quotes and rulings match MTGJSON.

### Step 1: skills, which inform unit boundaries

- **Phase 1:** `/add-replacement-effect` and `/add-static-ability` for the projection's condition evaluation (B1); `/add-trigger` for the observation matcher (B2, C3); `/oracle-parser` for the C4 condition conversion; `/card-test` for the runtime rows.
- **Phase 2:** `/add-engine-variant` (mandatory, for the new onset enum and field), `/oracle-parser` (the onset is produced by predicate dispatch), `/add-trigger` for the deferral predicate, and `/card-test`.
- No phase touches the frontend, the AI, `WaitingFor` or `GameAction`.

### Step 2: analogous traces, which inform the seams

The draft's traces stand:
- `SubAbilityLink` for per-node parse-stamped metadata (phase 2);
- the CR 614.12 `BattlefieldEntrantProjection` and the `layers.rs` population and zone classifiers (phase 1);
- `WithKeyword` against `HasKeywordKind` in `matches_filter_prop` (phase 1);
- the `battlefield_entry_observation` incarnation convention (phase 1);
- the `card_type_condition_as_entry_this_way` fold path (phase 1).

F1 adds one trace that phase 2 must follow: subject-predicate dispatch (`try_parse_subject_become_clause` → `PredicateAst::Become`; `try_parse_subject_continuous_clause` → `PredicateAst::Continuous`; the additive and copula arms), through `ParsedEffectClause` / `ClauseIr`, to the def built at assembly.

### Step 3: files

Scoped to each phase's scope rule below. Every anchor path named there was confirmed to exist at the run base.

## CR rules the decisions rest on

Each was grep-verified in `docs/MagicCompRules.txt` for this charter.

- **CR 614.12:** check the permanent as it would exist on the battlefield, counting its own statics that would apply once it is there, and continuous effects that already exist.
- **CR 614.1c:** "enters with" is a replacement effect.
- **CR 614.12a:** an entry-replacement choice is made before entry. Context only.
- **CR 700.5:** devotion counts mana symbols among permanents that player controls.
- **CR 109.5:** "you" on a static ability means the current controller of its source; for a resolved effect it is the effect's controller.
- **CR 110.2a:** an object put onto the battlefield enters under the instructed player's control.
- **CR 613.1:** the layer system.
- **CR 611.2e:** a nontoken permanent put onto the battlefield by a resolving spell or ability. An effect stating the permanent "is" or "has" a characteristic (or doesn't have one) applies simultaneously with the entry. "Becomes [characteristic]" and "gains [an ability]" apply after the permanent is on the battlefield.
- **CR 611.3c:** continuous effects modify characteristics as the permanent enters.
- **CR 603.10 / CR 603.10a:** trigger events are checked against objects as they exist immediately after the event, with the look-back exceptions.
- **CR 603.6a:** enters abilities check newcomers.
- **CR 608.2c:** instructions are followed in order.
- **CR 608.2i:** look-back information about earlier game states.
- **CR 400.7:** a zone change creates a new object.
- **CR 122.6:** counters put on an object include counters it enters with. Context for C4.

---

## Phase 1: entry projection, observation reads and harness fixes (B1, B2/C2, C3, C4, L1/C5, C6)

**Goal.** Bring every existing CR 614.12 and CR 603.10 reading the review flagged into line with the cards, with no change to the IR or the serialized ability shape:

- **B1:** the CR 614.12 entrant projection evaluates its population-reading conditions against the **pre-entry** battlefield (Heliod ruling (a)). Post-entry observers keep reading the realized battlefield (ruling (b)).
- **B2/C2:** kind-level keyword predicates answer from the object the filter is handed, as `WithKeyword` already does (CR 603.10).
- **C3:** the no-observation fallback of the entry-observation matcher honors the entered incarnation (CR 400.7).
- **C4:** a multi-type anaphoric "enters with … if it's an X or Y" rider folds into the entry gate as an any-of type gate (CR 614.1c), instead of surviving as a later counter instruction.
- **L1/C5:** `drive_to_empty` proves it settled.
- **C6:** documented as a deliberate non-change.

**Class attribution:**
- **B1:** every population- or zone-membership-reading `StaticCondition` leaf, under any static (the entrant's own, or another object's group static) or transient continuous effect, evaluated inside the projection. Printed reach: the Theros Gods, Rusted Relic, Starfield of Nyx and kin, crossed with the type and subtype entry-gate consumers (Recommission, What Must Be Done, Oviya Pashiri, Heroic Return, Winter Soldier, Summoner's Grimoire).
- **B2:** every kind-level keyword filter evaluated on a non-live object view.
- **C3:** every enters observer with no observation row.
- **C4:** the building block for any-of anaphoric entry gates.

**Scope rule.** The phase owns:
- **The CR 614.12 projection and the entry-observation matcher:** `crates/engine/src/game/filter.rs`.
- **Condition evaluation and the population and zone classifiers:** `crates/engine/src/game/layers.rs`.
- **Keyword authority:** `crates/engine/src/game/keywords.rs`.
- **The entry-gate condition converter:** `crates/engine/src/parser/oracle_effect/conditions.rs`.
- **The C6 doc comment on the entry-gate projection call site:** `crates/engine/src/game/effects/change_zone.rs`. Comment-only.
- **Tests:**
  - `crates/engine/tests/integration/conditional_entry_gate_projection.rs`
  - `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs`
  - `crates/engine/src/parser/oracle_effect/tests.rs`

Read-only anchors:
- `crates/engine/src/game/devotion.rs`
- `crates/engine/src/game/off_zone_characteristics.rs`
- `crates/engine/src/types/game_state.rs`
- `crates/engine/src/game/trigger_matchers.rs`
- `crates/engine/src/game/engine_replacement.rs`
- `crates/engine/src/parser/oracle_effect/sequence.rs`
- `crates/engine/src/parser/oracle_effect/lower.rs`

Editing a read-only anchor is drift and gets an addendum. Standing inclusion classes are implied.

**Architecture decisions.**

1. **B1: pin population-reading condition leaves to their pre-entry truth inside the scratch state.**
   - The projection keeps carrying the entrant's own statics and every existing continuous effect (CR 614.12). Only the **population** those conditions count is fixed to the pre-entry state.
   - One function in `layers.rs`, the condition-evaluation authority, rewrites the scratch's condition trees before the scratch layer pass. The projection in `filter.rs` only calls it.
   - **Carriers:** every scratch object's static definitions and every scratch transient continuous effect's condition. Each tree is rewritten recursively through `And` / `Or` / `Not`.
   - **Which leaves pin:** a leaf pins when it is population-reading by the existing population classifier, or reads membership of the entrant's origin zone by the existing zone classifier, **and** it does not use recipient context.
   - **Live state is never touched.** Live layers, triggers and the entry observation keep the realized, post-entry battlefield (ruling (b)).
   - Rejected alternatives:
     - Excluding the entrant from each counting primitive: that would thread an exclusion into devotion, `ObjectCount`, `IsPresent` and the rest, and still miss the origin-zone departure.
     - Building the scratch without the entrant: the entrant's own statics must apply (CR 614.12).
2. **B1: the pin's controller semantics (F3).**
   - Each leaf's pinned truth is evaluated against the **pre-entry** state, with the "you" player the **scratch** layer pass would use for that carrier. That is `condition_controller` / `active_effect_condition_controller` semantics evaluated on the scratch:
     - for a printed static, the source's current controller in the scratch (CR 109.5). For the entrant's own statics that is the post-override controller it enters under (CR 110.2a);
     - for a transient effect, the effect's controller.
   - `ConditionContext.ability_holder` carries that same player, only so `IsPresent` and filter-context reads name it. It never names the pre-entry owner.
   - Consequence: an entrant owned by one player and returned under another's control counts the **new** controller's pre-entry devotion and artifacts.
3. **B1: the pinned value's representation.**
   - No new `StaticCondition` variant. The pin is scratch-local and is never serialized or observed outside the projection.
   - It is expressed through one documented constructor that maps a truth value onto an existing constant-truth `StaticCondition` representation. Which representation that is, the phase plan establishes from `cargo engine-inventory`.
   - Any change to a static definition is copy-on-write, made only when a leaf actually pins.
4. **B2/C2: one object-scoped effective keyword-kind authority in `keywords.rs`; `matches_filter_prop`'s `HasKeywordKind` / `WithoutKeywordKind` arms call it with the object they hold, for both polarities.**
   - A battlefield-zoned object answers from its own keywords. An off-battlefield object keeps today's off-zone effective path.
   - The existing id-keyed lookup becomes lookup-plus-delegation, so there is one authority.
   - The invariant: for every caller, the kind-level answer comes from the same object as that caller's `WithKeyword` answer.
   - **Per-caller disposition (F5).** The phase plan enumerates every `filter_inner_for_object` caller and classes each one as live, entry observation, or record/projection skeleton (attack-declaration record, counter-added record, damage-record source, entering token, face-down entering projection, and any others it finds). For each class it:
     - derives the reading from the CR: CR 603.10 for event-time snapshots, CR 603.10a for look-back, CR 608.2i for look-back effect information;
     - establishes what the caller answers today.
   - A departed entrant's observation skeleton answers as the battlefield reading it records. The plan establishes the skeleton's zone stamp.
   - The decision rejects "live-path semantics unchanged" as an unestablished blanket claim. Each changed class needs a derivation, and at least one record-skeleton path needs a sibling row or a measured zero-reach statement.
5. **C3: the no-observation fallback's live arm requires the live object to be the entered incarnation**, using the same `entered_incarnation.is_none_or(…)` convention as the observation lookup. Otherwise it falls to the record snapshot (CR 400.7).
   - This is defensive hardening at a crate-public authority.
   - Whether any production route reaches the mismatch is a claim the phase plan establishes. The charter does not assert it.
6. **C4: the single entry-gate condition converter maps a multi-type affirmative card-type gate onto `TypeFilter::AnyOf` of the per-type filters.** Its outputs:
   - the single type stays the plain filter;
   - an empty list gives `None`;
   - additional or subtype filters stay `None` (the strict path).
   - The existing fold and the sequence-level swallow recognizer consume the same converter, so they follow it.
   - No new variant. No new text matching: it is a slice-pattern extension of the converter.
   - The semantic claim is runtime (CR 614.1c), so the phase's discriminating evidence is a required runtime row, not parse shape alone (F9).
7. **L1/C5: `drive_to_empty` tracks whether it broke on an empty stack at Priority, and asserts that after the loop**, reporting the prompts it saw.
   - It lands first in the phase, so every later row in its file runs under the hardened harness, including phase 2's rows.
8. **C6: no code change.** One doc comment on the entry-gate projection site records two things: the at-most-two-projections-per-gated-entrant bound, and the zero-printed-reach decision.
   - The PR reviewer accepted this disposition.
   - Whether any printed card carries both gates on one entrant is a claim the phase plan establishes.
9. **Out of bounds:**
   - every phase-2 decision (the onset type, the parser stamp, the D1 predicate);
   - `game/effects/mod.rs` and `types/ability.rs`;
   - the Zimone token path;
   - the non-modal "enters_with_gate" routing gap (a run-level residual).

**Addenda file:** `.git/engine-implementer-runs/etb-review-fixes/addenda/phase-1`

**Deferral list.**
- `DEFERRED(phase 2)`: all C1 content. That is the typed onset, its parse stamp, the D1 narrowing, and every C1 runtime and parse row (Kardur's III, Hunting Wilds, Rampaging Growth, the is-form regression rows and the classifier rows).
- No phase-1 discriminating test depends on phase 2. Every phase-1 unit is independently discriminated at the phase base.

---

## Phase 2: a typed CR 611.2e onset narrows entry-observation deferral (C1)

**Goal.**
- An entry observation is deferred past a later instruction only for a continuous-effect rider that applies **with** the entry: a CR 611.2e "is/has/doesn't have" rider.
- A "becomes/gains" rider applies after entry, so it no longer suppresses settlement. That holds whether the rider follows the entry directly (Rampaging Growth with Soul Warden, F2) or after an intervening modification (Kardur's III, Hunting Wilds kicked).
- Unknown onset keeps today's conservative deferral.

**Class attribution:** every battlefield-entering chain whose remaining chain holds a D1-kind node. The draft's corpus scan counts several hundred such chains, including the direct-chain becomes/gains class (F2). The phase plan re-measures the class with a named command.

**Scope rule.** The phase owns:
- **The onset type and the per-node field, with their hand-written serde mirrors and the D1-kind membership method on `Effect`:** `crates/engine/src/types/ability.rs`.
- **Predicate dispatch, where the onset is produced:** `crates/engine/src/parser/oracle_effect/subject.rs`.
- **The parsed-clause and IR carriers:** `crates/engine/src/parser/oracle_ir/` (expanded to its changed files at Sizing) and `crates/engine/src/parser/oracle_effect/mod.rs`.
- **The def construction that stamps the field:** `crates/engine/src/parser/oracle_effect/assembly.rs`.
- **Def → resolved conversion:** `crates/engine/src/game/ability_utils.rs` and `crates/engine/src/game/effects/separate_piles.rs`.
- **The D1 predicate and its walkers:** `crates/engine/src/game/effects/mod.rs`.
- **The audit key list:** `crates/engine/src/parser/audit_projection.rs`.
- **Tests:** `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs` and `crates/engine/src/parser/oracle_effect/tests.rs`, plus insta snapshots the stamp changes.

Compiler-forced literal and destructure sites of the new field are a standing class (F6 lists them). Regenerated card data is excluded from T2. No protocol-version bump is expected for a `serde(default)` optional field.

**Architecture decisions.**

1. **A typed onset on the ability node.** It is a two-value enum (`SimultaneousWithEntry`, `AfterEntry`) on one CR 611.2e axis, carried as `Option<…>` on `AbilityDefinition` and `ResolvedAbility`. `None` means "not a D1-kind node, or onset unknown". The field mirrors the `sub_link` per-node metadata pattern: serde default, skipped when `None`, copied at both def → resolved conversion sites, and listed in the audit key list.
   - Rejected alternative: a field on each D1 `Effect` variant. That touches every literal construction of four effect kinds and puts node-level timing on the effect payload.
   - The `/add-engine-variant` gate runs before the enum is added.
2. **The onset is a product of predicate dispatch (F1).** The parser combinator that recognizes the predicate head yields the onset alongside the predicate:
   - `Become` gives `AfterEntry`;
   - the additive-type "is … in addition", the copula "is/are", the "it's/they're" head and the "has/have" head give `SimultaneousWithEntry`;
   - `Continuous` splits by the verb its dispatch matched: gains gives `AfterEntry`, has gives `SimultaneousWithEntry`, and gets or any other verb gives `None`.

   Carrying the onset:
   - It travels on the parsed clause (`ParsedEffectClause` / `ClauseIr`) into the def built at assembly.
   - No re-scan of `clause_ir.source.fragment()` and no second text classifier. Quoted granted-ability text therefore cannot influence the onset, because dispatch has already consumed the head.
   - A D1-kind def whose producer is not a stamped predicate dispatch keeps `None`.
   - The phase plan establishes, for each D1 kind (`GenericEffect`, `Animate`, `ReturnAsAura`), which producers stamp it, and covers three cases with rows:
     - a leading-duration subject ("Until end of turn, that land becomes …");
     - the "That creature is …" copula (Rise from the Grave);
     - a quoted-verb hostile.
3. **A retention tail is not an is-form onset.** "It's still a land" and "that are still lands" state no new characteristic. A Become clause carrying one stays `AfterEntry`.
   - Any other node whose dispatch yields both onsets keeps `None` (conservative).
4. **`ApplyPerpetual` is not stamped (F10).** Perpetual effects are digital-only, with no CR entry, so this run has no derivation for changing their reading. Their nodes keep `None` and today's deferral.
5. **D1 consumes the typed field and does no string inspection.**
   - The node predicate is "D1-kind **and** onset is not `AfterEntry`", written as an exhaustive `match` on the onset.
   - The D1-kind membership becomes one inherent method on `Effect`, the single authority both the parser stamp and the runtime predicate use. The parser never imports from `game::effects`.
   - Every D1 walker passes the node's field: the frame walker, the library-search resume walker, and any other carrier the phase plan enumerates.
6. **Required evidence classes (constraints on the phase plan's matrix, F2):**
   - a direct "enter → becomes/gains" chain, Rampaging Growth with Soul Warden, verbatim;
   - an "enter → modification → gains" chain, Kardur's III with Elemental Bond;
   - the kicked Hunting Wilds chain;
   - is-form preservation rows, Rise from the Grave and Path of the Schemer, with their stamped `SimultaneousWithEntry`;
   - parse and serde rows for the field;
   - the measured count of entering-chain D1 nodes left `None` after regeneration.

   Which base values are red is a claim the phase plan establishes, not this charter.
7. **Out of bounds:**
   - every phase-1 authority: `filter.rs`, `layers.rs`, `keywords.rs`, `conditions.rs` and the projection;
   - applying an is-form rider inside the entry projection (the Call a Surprise Witness residual);
   - perpetual onset;
   - the Zimone token path;
   - new `Effect` variants.

**Addenda file:** `.git/engine-implementer-runs/etb-review-fixes/addenda/phase-2`

**Deferral list.** None within this run. Phase 2 is the last phase. It omits only the run-level residuals below.

---

## Finding ownership (every review finding is a constraint)

| Finding | Owner | Disposition at decision level |
|---|---|---|
| PR B1 | phase 1 | decisions 1–3 |
| PR B2 / C2 | phase 1 | decision 4 |
| PR C3 | phase 1 | decision 5 |
| PR C4 | phase 1 | decision 6 |
| PR L1 / C5 | phase 1 | decision 7 |
| PR C6 | phase 1 | decision 8 (comment only) |
| PR C1 | phase 2 | decisions 1–6 |
| F1 (HIGH) | phase 2 | decision 2: the onset comes from typed predicate dispatch; no fragment classifier |
| F2 | phase 2 | decision 6: a direct-chain row (Rampaging Growth with Soul Warden); the phase plan corrects the printed-reach sentence |
| F3 | phase 1 | decision 2: scratch-carrier controller semantics; a phase-1 unit case with owner ≠ entering controller, covering `DevotionGE` and an `ObjectCount` comparison |
| F4 | phase 1 | the phase plan isolates the ruling-(b) observer row from Heliod's own life-gain trigger. Options: assert before that trigger resolves, target a permanent other than Heliod explicitly, or use a creature-entry observer that gains no life. Life gain or the observer firing stays the positive guard |
| F5 | phase 1 | decision 4: per-caller disposition |
| F6 | charter | resolved by this split; each phase's Sizing re-counts under the same rule |
| F7 | phases 1 and 2 | verbatim quotes: Rusted Relic in phase 1; Hunting Wilds, Rise from the Grave and Path of the Schemer in phase 2. Phase 1 also corrects the What Must Be Done mode name ("Release Juno") |
| F8 | phase 1 | the reviewer's Reference Readings bullet for the V2/V4/V5 positive controls, verbatim |
| F9 | phase 1 | decision 6: a required runtime C4 row (an any-of gate that admits an artifact creature, and a non-artifact noncreature historic permanent that gets 0) |
| F10 | phase 2 | decision 4 |
| F11 | run-level | the residual wording below, verbatim from the review |

## Seam notes

- **Seam choice.** The draft's seam is kept: phase 1 is U6, U2, U3, U1, U5 plus the C6 comment; phase 2 is U4.
  - **Dependencies.** No logic dependency edge exists between the units: no unit's discriminating test needs another unit's code. The one ordering edge is verification. Phase 2's runtime rows live in the file whose harness L1/C5 hardens, so phase 1 lands first.
  - **Why U4 alone.** U4 is the only unit with a serialized-type change and its compiler-forced fan-out. Isolating it keeps phase 2 at one unit, so T1 cannot fire there however large its T2. It also keeps phase 1's T2 small.
  - **Alternatives rejected:**
    - Moving U5 into phase 2: that makes phase 2 two units with T2 ≥ 13, which fires.
    - Reversing the order: phase 2's rows would run under the unhardened harness, and the human reviewer's requested changes (B1, B2) would land last.
    - A third phase that splits B1 off: the gate does not require it, and it adds a loop.
- **Shared files.**
  - Both phases add rows to `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs`: the harness fix in phase 1, the C1 rows in phase 2.
  - Both phases edit `crates/engine/src/parser/oracle_effect/tests.rs`: the C4 row in phase 1, the onset parse rows in phase 2.
  - Sequential execution makes the overlap safe.
  - Every production file belongs to exactly one phase. In particular, `game/filter.rs`, `game/layers.rs` and `game/keywords.rs` belong to phase 1, and `game/effects/mod.rs` and `types/ability.rs` belong to phase 2. The runtime `game/effects/mod.rs` and the parser's `parser/oracle_effect/mod.rs` are different files, both in phase 2.
- **Independence of the two CR questions.**
  - Phase 1's pin changes only what the CR 614.12 gate projection counts.
  - Phase 2 changes only when the CR 603.10 observation is taken.
  - Neither phase routes its authority through the other's.
  - The integration review checks that phase 2's settlement change leaves phase 1's gate readings (B1 and C4) unchanged, and that phase 1 added no dependency on the onset field.
- **Coverage honesty.**
  - Phase 1 changes one lowering (C4): already-accepted text, newly folded. It reports its coverage and semantic-audit delta.
  - Phase 2 adds metadata to already-parsed clauses. Its plan shows a zero supported/unsupported delta, or reports and explains one.
  - No phase accepts text with deferred semantics.
- **Card-data regeneration** follows the parse-shape changes in phases 1 and 2, and is excluded from T2.

## Recursive phase-fit check (T1 ∧ T2 per phase)

| Phase | Units (T1: ≥2?) | Expected counted scope paths (T2: ≥13?) | Conjunction |
|---|---|---|---|
| 1 | 5 (B1, B2/C2, C3, C4, L1/C5) → **yes** | About 8: 4 authored product files, 1 comment-only file, and 3 test files (counted even though the T2 rule may exclude them) → **no** | does not fire |
| 2 | 1 (C1 onset) → **no** | About 25: F6's 28 minus phase 1's product files, plus the parser IR carriers F1 adds → **yes** | does not fire |

Each phase plan's Sizing measures its own count. The orchestrator re-adjudicates at scope-freeze.

- **Phase 1 re-charter route.** If a standing-class extension brings phase 1's T2 to ≥ 13, the conjunction fires. The proposed split is 1a = B1 (with L1/C5 first) and 1b = B2/C2, C3, C4. The units are independent, so either order is green.
- **Phase 2 re-charter route.** Phase 2's conjunction fires only if its Sizing shows ≥ 2 units. In that case the T3 seam is:
  - **2a:** the onset type, field and parse stamp. Infrastructure: D1 unchanged, runtime green, parse and serde rows only. Its runtime rows are `DEFERRED(2b)`.
  - **2b:** the D1 consumer and its runtime rows.

## Run-level residuals (not deferred to any phase)

- **R1, already triaged as a separate follow-up issue: Call a Surprise Witness with a flying observer.** F11 wording: "A P/T-gated or type-gated observer reads correctly here: the Spirit type applies with the entry, and a flying counter changes no P/T. A **keyword-gated** observer reads wrongly. Dragon Tempest ("Whenever a creature you control with flying enters, it gains haste until end of turn") sees the flying counter put after entry. That is pre-existing at base, which defers the same way."
- **R2, already triaged as a separate follow-up issue:** the non-modal "It enters with … counters on it if it's a <type>." line parses to `Unimplemented "enters_with_gate"`, in both single-type and multi-type forms.
- **R3:** D1 nodes whose onset stays `None` (unstamped producers, mixed-onset nodes) keep the conservative deferral. Phase 2 reports the count.
- **R4:** `ApplyPerpetual` riders keep today's deferral (phase-2 decision 4).
- **R5, B1 limits:**
  - population leaves that also use recipient context are not pinned;
  - dynamic P/T quantities inside continuous modifications are not conditions, so they are not pinned;
  - a layer-1 copy effect inside the scratch copies a source's pinned definitions;
  - `ForAsLongAs` durations of transient effects are pruned, not pinned;
  - granted statics carrying a population-gated type write.

  Whether any gate consumer reaches these is a claim phase 1 establishes.
- **R6:** C6's two-projections bound for an entrant carrying both gates.
- **R7:** the Zimone "create a token, then put counters on it" path is untouched.

## Attempt history carried into every phase

- **Prior run:** `etb-entry-snapshot` was accepted. It produced the CR 614.12 projection, the entry observation and the D1 predicate this run corrects.
- **This run's plan loop:** round 1 (`rf-plan-r1.md`) was a design round, with `behavior` = 7 (F1 HIGH; F2, F3, F4, F5 MED; F9, F10 LOW) and `text` = 4 (F6, F7, F8, F11). The gate was re-adjudicated (T1 = 6, T2 = 28) and fired mid-Step-2, which led to this charter.
- **Charter loop:** round 1 is this file.
- **Phase plan loops** resume the plan loop's count, so each next round is design round 2 of max 5. The round-1 behavior findings attribute as follows:
  - phase 1: F3, F4, F5, F9 (4);
  - phase 2: F1, F2, F10 (3).
- **Phase-plan inputs.** Each phase plan projects its decisions from `rf-plan-r1.md`, with this charter's decisions and the finding-ownership constraints applied. Phase-1 decisions 2 and 4 (controller semantics and per-caller disposition) and phase-2 decisions 2–4 (dispatch-produced onset, retention tail, perpetual) have no clean draft source. Those phase plans design them fresh against this charter, and their reviews audit them fresh.
