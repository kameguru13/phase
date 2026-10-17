# Charter review r2: etb-review-fixes (rf-charter-r2.md)

Reviewer: fresh `review-engine-plan`, **charter mode**. Phase-fit context was declared, so Sizing consistency blocks.

Inputs:
- the charter `rf-charter-r2.md`;
- the original task, `pr10036-review.md`;
- `rf-plan-r1.md` with its review `rf-plan-review-r1.md`;
- `rf-charter-r1.md` with its review `rf-charter-review-r1.md`;
- the attempt history: plan loop round 1 was a design round (b=7, t=4); the gate fired (T1=6, T2=28); charter round 1 was a design round (b=1, t=9). This is charter round 2. Budget: standard.

Base: `addbc884`. I re-measured `git diff --stat e56851e addbc884 -- crates/`. It is empty, so the line-3 premise holds.

Evidence used:
- `git show`/`git grep` at base;
- `AtomicCards.json`, for text and rulings;
- `docs/MagicCompRules.txt`;
- the existing census `rf/c1detail.json`, which I cross-joined with MTGJSON text.

No cargo build was run, as the caller instructed.

**Verdict: clean round.** 0 decision findings (`behavior` 0). 6 corrections, all `text`: 2 MED and 4 LOW. 0 `machinery`. Every r1 finding is closed. The orchestrator applies the corrections below, and the charter freezes with no further round.

---

## r1 closure check

I diffed `rf-charter-r1.md` against `rf-charter-r2.md` and checked every changed line.

| r1 item | Status |
|---|---|
| D1 (gets → `AfterEntry`) | Closed. Both texts are verbatim: decision 2's third bullet and the Goal's second bullet. Macabre Mockery has been added to Step 0, and I confirmed its text in MTGJSON. The ownership row has been added, and R3 has been extended. The supplied text left one inconsistency of its own (see T1). |
| C1 (acceptance rows, claims) | Closed for all 9 listed sites. The predicate matches two more sites that the r1 list missed (see T2). |
| C2 (off-battlefield snapshot) | Closed, verbatim. |
| C3 (C6 attribution) | Closed. |
| C4 (four code-state lines) | Closed. I re-checked the slice-pattern premise of decision 6 at base: `conditions.rs:1624` is `let [core] = card_types.as_slice() else { return None }`. |
| C5 (class-attribution names) | Closed in substance. The comma list reintroduces the Winter Soldier ambiguity (see T3). |
| C6 (CR 109.5, add CR 205.1b) | Closed. Both are verified: 109.5 at line 610 and 205.1b at line 1404. |
| C7 (read-only anchor) | Closed, verbatim. |
| C8 (phase-1 re-charter route) | Closed, verbatim. One order clause is incomplete (see T6). |
| C9 (snapshot directories) | Closed. All three directories exist at base. |
| Non-blocking notes (multi-entrant, mixed leaves, `ability_holder` doc) | All three are incorporated: decision 1's "Pre-entry means before the event", R5's last bullet, and decision 2's doc sentence. |
| Pre-existing (Oviya) | Recorded as R8. |

New sentences beyond the supplied text:
- the Mode line;
- R8's two sentences;
- decision 1's multi-entrant bullet;
- the D1 ownership row.

I checked each one. Decision 1's multi-entrant reading is rules-correct. Under CR 614.12, the other members of a simultaneous event are not yet on the battlefield. Their statics therefore do not "already exist", and they add nothing to a CR 700.5 population. Nothing else in these sentences needs a change.

---

## Corrections (the orchestrator applies these; each is `text`)

### T1 `text` MED: decision 2's general sentence and the Goal settle "loses" as after-entry, but the decision and R3 keep it `None`

**Coordinates.**
- Phase 2, Goal, second bullet (line 185): "A rider the CR places after entry (every dispatched verb outside CR 611.2e's is/has/doesn't-have forms: becomes, gains, gets) no longer suppresses settlement."
- Phase 2, decision 2, third bullet (line 210): "The 611.2e simultaneity is limited to its named forms, so any later-instruction effect applies after the entry event."

**Contradiction.**
- "Loses" is a dispatched verb outside the is/has/doesn't-have forms. By the Goal, and by the line-210 rationale, it therefore no longer suppresses settlement.
- The same bullet then says it keeps `None` ("for example 'loses' …"), and R3 lists it as conservative deferral.
- A phase planner told to "derive from the CR" can derive `AfterEntry` for "loses" from the first sentence.

**The rules side favours the decision, not the general sentence.**
- CR 611.2e (line 2921) names "becomes" and "gains" as after-entry. It says nothing about "loses".
- The Bronzehide Lion ruling (2020-01-24, MTGJSON) says the following about a copy that returns as "an Aura enchantment … and it loses all other abilities": "If it has any enters-the-battlefield replacement effects, those won't apply." So a "loses" rider in that construction applies **with** the entry. A blanket "any later-instruction effect applies after the entry event" is overbroad.
- "Gets" (D1) stands. It is the P/T sibling of "gains", and no ruling I found reads it as simultaneous. The Macabre Mockery rulings and the census cards are silent on this.

**Printed reach of the inconsistency.** The census (`c1detail.json`, 334 cards, joined with Oracle text) has "loses" riders on these enter → D1 chains:
- Retched Wretch: "… return it to the battlefield under its owner's control and it loses all abilities.";
- Vraska, the Silencer;
- Bronzehide Lion;
- Takklemaggot;
- Animate Dead and Dance of the Dead.

**Replacement texts.**
- Line 185, old: "A rider the CR places after entry (every dispatched verb outside CR 611.2e's is/has/doesn't-have forms: becomes, gains, gets) no longer suppresses settlement."
  New: "A rider the CR places after entry (becomes and gains by CR 611.2e; gets, its P/T sibling, by CR 608.2c) no longer suppresses settlement. A verb the CR does not settle keeps today's deferral (R3)."
- Line 210, old: "The 611.2e simultaneity is limited to its named forms, so any later-instruction effect applies after the entry event."
  New: "CR 611.2e names "becomes" and "gains" as after-entry, and CR 608.2c's instruction order puts "gets" with "gains"."
- Line 210, old: "(for example "loses": CR 611.2e names "doesn't have", not "loses")"
  New: "(for example "loses": CR 611.2e names "doesn't have", not "loses", and the Bronzehide Lion ruling (2020-01-24) applies an "it's … and it loses all other abilities" rider with the entry)"

### T2 `text` LOW: two sites matching r1-C1's predicate that the r1 list missed

**Predicate** (r1-C1): a charter sentence that specifies a test row, evidence class or delta report a phase must produce.

**Command:** `grep -n -i "report\|\brows\?\b\|measure\|assert\|\bcount\b\|establish" rf-charter-r2.md`. I read all 27 matching lines. Their dispositions:
- "Whether … is a claim the phase plan establishes" (lines 141, 149, 150, 154, 166, 200, 216, 316), and line 129's "or establishes": allowed. This is chartered.md's form: the plan names the claim and the charter does not state it.
- Line 176: allowed. It is a deferral attribution.
- Lines 40, 96, 162–163, 266, 270, 273–274, 295, 299–300 and 305: architecture or seam descriptions. Line 305 is the C4 form r1 supplied. Allowed.
- **Line 188** matches the predicate. It also carries a snapshot count.
  - Old: "The draft's corpus scan counts several hundred such chains, including the direct-chain becomes/gains class (F2). The phase plan re-measures the class with a named command."
  - New: "It includes the direct-chain becomes/gains/gets class (F2)."
  - "Gets" is added for consistency with D1. The size of the class is the phase plan's measurement.
- **Line 306 (R3)** matches the predicate.
  - Old: "… keep the conservative deferral. Phase 2 reports the count."
  - New: "… keep the conservative deferral."

### T3 `text` LOW: the class-attribution list is ambiguous again (line 94)

- MTGJSON has three Winter Soldier cards: "Winter Soldier, Bucky Barnes", "Winter Soldier, Icy Assassin" and "Winter Soldier, Reborn Avenger". Only Reborn Avenger is a gate consumer ("If a Hero enters this way, it enters with an additional +1/+1 counter on it.").
- In a comma-separated list, "Winter Soldier, Reborn Avenger" reads as two cards.
- Old: "(Recommission, What Must Be Done, Heroic Return, Winter Soldier, Reborn Avenger, Summoner's Grimoire)"
- New: "(Recommission; What Must Be Done; Heroic Return; Winter Soldier, Reborn Avenger; Summoner's Grimoire)"

### T4 `text` MED: the all-`AfterEntry` rule reads "stamped parts" ambiguously when some parts are unstamped (decision 2, line 210)

**Old:** "A node whose stamped parts are all `AfterEntry` ("gets … and gains …") is `AfterEntry`. Decision 3's mixed rule covers only Simultaneous-with-AfterEntry mixes."

**Gap.**
- A node that mixes an `AfterEntry` part with a `None` part is covered neither by this sentence nor by decision 3.
- "Stamped parts" can be read to exclude the unstamped part, which yields `AfterEntry`. That is the non-conservative outcome R3 rules out.

**Printed witness.** Takklemaggot, if the player doesn't choose a creature: "return this card to the battlefield under your control as a non-Aura enchantment. It loses "enchant creature" and gains …". Its card-data chain is `ChangeZone > GenericEffect`.
- Losing "enchant creature" has to apply with the entry, as it does for Bronzehide Lion (see T1).
- So stamping the node `AfterEntry` would be wrong.

**New:** "A node is `AfterEntry` only when every part's onset is `AfterEntry` ("gets … and gains …"). A node with any part whose onset is `None` keeps `None`. Decision 3's mixed rule covers Simultaneous-with-AfterEntry mixes."

This tightens the wording toward the conservative reading the decision already intends. No decision changes.

### T5 `text` LOW: for a token entrant, the is/has onset rests on CR 111.3, not CR 611.2e

**Coordinates.**
- Phase 2, Goal, first bullet (line 184): "… only for a continuous-effect rider that applies **with** the entry: a CR 611.2e "is/has/doesn't have" rider."
- The CR list (line 70).

**Rules.**
- CR 611.2e covers only a resolving spell or ability that "puts a **nontoken** permanent onto the battlefield" (line 2921).
- The decision's class includes token chains. The census has 20 `Token > D1` cards whose rider is "It has / They have …", for example:
  - Dragonbroods' Relic: "Create a 4/4 Dragon creature token named Reliquary Dragon that's all colors. It has flying, lifelink, and "When this token enters, …"";
  - the Eldrazi Scion makers.
- Those characteristics are part of the token's definition under CR 111.3 (line 649): "The spell or ability that creates a token may define the values of any number of characteristics for the token." Reliquary Dragon's own enters trigger only works if that is so.
- So the `SimultaneousWithEntry` outcome is right for tokens, but the authority cited for it is wrong. A CR annotation copied from the charter would misattribute it.

**Replacements.**
- Line 184, old: "a CR 611.2e "is/has/doesn't have" rider."
  New: "a CR 611.2e "is/has/doesn't have" rider. For a token entrant, the same reading is the CR 111.3 characteristics its creating effect defines."
- Add to the CR list after CR 205.1b: "**CR 111.3:** the spell or ability that creates a token may define any of its characteristics. Those values are the token's "text". This is the basis for the is/has reading on token entrants, which CR 611.2e (nontoken only) does not cover."

### T6 `text` LOW: the phase-1 re-charter route drops the one ordering edge the charter states (line 297)

**Old:** "The remaining units form 1b, with L1/C5 first. The units are independent, so either order is green."

**Gap.**
- Decision 7 and the seam notes rest phase ordering on one edge: every row in `etb_trigger_entry_time_characteristics.rs` runs under the hardened `drive_to_empty`.
- If 1a is a unit that adds rows to that file (B2 or C3, for example) and it runs first, its rows run under the unhardened harness.
- "Either order is green" is true, but it does not respect that edge.

**New:** "The remaining units form 1b, with L1/C5 first. The sub-phase that carries L1/C5 goes first whenever the other sub-phase adds rows to `crates/engine/tests/integration/etb_trigger_entry_time_characteristics.rs`. Otherwise either order is green, since the units are independent."

---

## Checklist judgments (charter mode)

### Seams and order

- Phase 1 (B1, B2/C2, C3, C4, L1/C5 and the C6 comment) comes before phase 2 (C1).
- No logic edge exists between the phases. Phase 1 touches neither the onset nor D1, and phase 2 touches no projection, keyword or condition authority.
- The one verification edge holds. `drive_to_empty` is at `etb_trigger_entry_time_characteristics.rs:372`, and its callers at lines 449, 583, 850 and 880 are in that file.
- Both seams leave the tree green:
  - phase 1 changes no types;
  - phase 2 adds a `serde(default)`, skip-if-`None` field, and its insta snapshot directories are in its own scope.

### Anchors exist at base

| Anchor | Location at base |
|---|---|
| `matches_target_filter_on_entry_observation` | `filter.rs:3987`. C3's fallback arm is here, and its only production caller is `trigger_matchers.rs:1264`. |
| `BattlefieldEntrantProjection` | `filter.rs:4822` |
| `filter_inner_for_object` | `filter.rs:5853` |
| `condition_controller` / `active_effect_condition_controller` | `layers.rs:1573` / `layers.rs:1545` |
| Population classifier `static_condition_uses_object_population` | `layers.rs:1666` |
| Zone classifier `static_condition_reads_zone_membership` | `layers.rs:3701` |
| `has_keyword_kind` / `object_has_effective_keyword_kind` | `keywords.rs:75` / `keywords.rs:80` |
| `card_type_condition_as_entry_this_way` | `conditions.rs:1624` |
| `entry_gate_projection` | `change_zone.rs:1644` |
| `TypeFilter::AnyOf` | `ability.rs:6287` |
| `try_parse_subject_continuous_clause` / `try_parse_subject_become_clause` | `subject.rs:619` / `subject.rs:874` |
| `PredicateAst` | `oracle_ir/ast.rs:258` |
| `ParsedEffectClause` | `oracle_ir/ast.rs:26` |
| `ClauseIr` | `oracle_ir/effect_chain.rs:882` |
| `assemble_effect_chain` | `assembly.rs:2089` |
| D1 walkers | `effects/mod.rs:4809` and `effects/mod.rs:4823` |
| `replay_deferred_entry_events` | `engine_replacement.rs:2793`, a read-only anchor, which is consistent with decision 5 placing C3 in `filter.rs` |

- The def → resolved `sub_link` copy sites are `ability_utils.rs:228` and `separate_piles.rs:478`. Both are in phase-2 scope.
- Every other non-test `sub_link` site is one of these:
  - a `sub_link: _` destructure, which is compiler-forced and a standing class;
  - an assignment of a fixed link value;
  - a read.

### Recursive T1 ∧ T2 and Sizing consistency

- **Phase 1.** It has 5 units (decisions 1–3, 4, 5, 6 and 7, with decision 8 comment-only) and about 8 paths. It does not fire, and its Sizing is consistent with the body.
- **Phase 2.** It has 1 unit (the C1 narrowing: field, stamp and consumer) and about 25 paths. It does not fire, whatever T2 turns out to be.
- The 2a/2b route is a valid T3 seam: infrastructure first, then the consumer, with 2a's runtime rows written `DEFERRED(2b)`.
- The phase-1 route, after C8, makes the firing unit a single-unit 1a. T6 adds the order clause.
- The table's units sum to 6, which matches the T1 that fired.

### Rules-correctness of each decision

- **B1, decisions 1–3.** These are correct under CR 614.12 (line 3104) and the two Heliod rulings.
  - Pinning population and origin-zone leaves to the pre-event state makes Starfield of Nyx's group static, and Rusted Relic's or Heliod's own static, count only the permanents already there. Under "continuous effects that already exist", an effect whose condition is false before the event does not exist.
  - The controller reading follows CR 109.5 and CR 110.2a.
  - The R5 limits are residuals, and the phase plan establishes their reach.
- **B2, decision 4.** Correct under CR 603.10. The invariant is that the kind-level answer comes from the same object as `WithKeyword`.
- **C3, decision 5.** Correct under CR 400.7.
- **C4, decision 6.** Correct under CR 614.1c. It has zero printed reach, as the task's orchestrator note says, and it is a building block. No new variant is involved.
- **L1/C5 and C6.** Process and comment only.
- **C1, decisions 1–5.** The axis stays within one CR section (611.2e), with the token basis corrected by T5. Become → `AfterEntry`. A retention tail is `AfterEntry` (CR 205.1b). Perpetual is never stamped. The predicate is an exhaustive match. "Gets" → `AfterEntry` holds. T1 and T4 fix only wording, toward the conservative reading the decision already takes.

### Deferral allowlist

- Phase 1 writes `DEFERRED(phase 2)` for all C1 content, with a named landing phase. Phase 2 defers nothing.
- R1–R8 are run-level residuals or pre-existing items. None is a requested behaviour left unattributed. I checked every item in the original task: B1 with its four suggested sub-items, B2, L1, and C1 through C6.

### Re-charter routes

- Both routes are present. They obey chartered.md's rule that only one split is allowed inside a phase before executor dispatch. T6 adds the order clause.

### Rule-type checks

- **No file inventory.** Scope rules are literal paths plus two directories: `oracle_ir/` and the snapshot directories. There are no globs. Step 3 defers to the scope rules.
- **No code-state assertions.** Two lines state code state:
  - line 3, which I re-measured;
  - line 57, "Every anchor path … was confirmed to exist", which I re-measured above.
  Both are true. Every other code-dependent claim uses the sanctioned "establishes" form.
- **No acceptance rows**, once T2 is applied.
- **Premise.** Step 0 is present. I re-verified Macabre Mockery, the Winter Soldier names, Oviya, What Must Be Done ("Release Juno"), Heroic Return, Summoner's Grimoire and Recommission in MTGJSON. The other Step 0 cards are unchanged since r1 verified them.
- **CR numbers.** I grep-verified 614.12, 614.12a, 614.1c, 700.5, 109.5, 110.2a, 613.1, 611.2e, 611.3c, 603.10, 603.10a, 603.6a, 608.2c, 608.2i, 400.7, 122.6 and 205.1b, plus 111.3 for T5.
- **Addenda files.** Each phase names its own.

## Non-blocking notes (for the phase plans)

- **Phase 2.** Some parser passes rebuild or relink sub-ability defs after assembly: `lower.rs` (fold/relink sites around lines 3996–4054) and `oracle_saga.rs` (lines 1450–1464, which relinks chapter-instruction grants, relevant to Kardur's III). The onset must survive those passes. Decision 2 already asks the phase plan to establish which producers stamp each D1 kind. If the stamp needs an edit in `oracle_saga.rs` or `lower.rs`, that file is outside phase 2's scope rule and goes through the plan-step admission route.

## Pre-existing (untagged, non-blocking)

None new. Oviya, from r1, is recorded as R8.
