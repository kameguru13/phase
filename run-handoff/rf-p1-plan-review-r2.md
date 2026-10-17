# Phase-1 plan review r2: etb-review-fixes (rf-p1-plan-r2.md), plan-loop design round 3 of 5

Reviewer: fresh Step 2 `review-engine-plan`, **phase-plan mode** (charter `phase-charter`, phase index 1, allowlist: all C1 content `DEFERRED(phase 2)`). Phase-fit context declared, so check 12 (Sizing) is blocking.
Base: `addbc884ee251794a64f2e1da0487cd329f8ce90`. The checkout was clean before and after this review, and no worktree was left behind (`git worktree list` shows only the main checkout and the unrelated `/home/user/wt/complete`). Evidence: code read at base; the planner's probe sources and logs in `scratchpad/p1r2/` (`probe-run.log` exit 0 with 4/4 ok and every `PROBE` marker printed; `probe-run2.log` 1/1 ok); MTGJSON `AtomicCards.json`; card-data `/home/user/wt/data-out/p2/card-data.json` (I re-ran `p1r/gates.py`: 6 gate consumers, each a single-object `ChangeZone`); `docs/MagicCompRules.txt`. I ran no build. Every point below is settled by a quoted code read, or by the planner's probe output, which I checked against its source.

**Verdict: no `behavior` finding.** `behavior` = 0, `text` = 2 (F1, F2), `machinery` = 0. Under the run limits this round closes once the orchestrator applies F1 and F2.

## Prior findings

**rf-p1-plan-review-r1 F1–F6: all closed.**
- **F1 (copy payload).** U1 pins `CopyValues { values }.static_definitions` with the recipient's scratch controller. I confirmed that every production `CopyValues` producer binds a single recipient:
  - `become_copy.rs:225–252` binds `SpecificObject { id: recipient_id }` plus `affected_recipient`;
  - `merge.rs:353` binds `SpecificObject`;
  - `create_emblem.rs:660` and `derived_views.rs` construct `CopyValues` only inside test modules;
  - `layers.rs:11068` asserts that `CopyValues` originates from a transient, so a static can never carry one.

  V5b was probed (P13: the copier has `base_static_defs=0`, `static_defs=1` and one `CopyValues` transient; Sanctum got 2 counters at copy + 3). R5(c) is rewritten.
- **F2 (carriers in every zone, both fields).**
  - `reset_recipient_to_base` runs only over battlefield ids (`layers.rs:4890`).
  - `static_source_effects_in_pass` reads `source.static_definitions` (`layers.rs:7178–7200`).
  - No production path inside the pass rebuilds `base_static_definitions` from card data. Every `layers.rs` writer from line 12535 onward is inside `mod tests` (which starts at 12179).

  So pinning both fields is correct. V5c was probed (P14), the V6a (f)/(g) cases are present, and the "exactly the population" sentence is gone.
- **F3 (owner-zone clone).** `HeldObject::Live` on the clone path goes to `off_zone_has_keyword_kind(state, obj.id, ..)`, which is keyed by id and is the same as at base. V9b is discriminating against an address test, per the `probe-run2.log` f3b line: `own_keywords_kind=false`, `r1-design … = false`, `owner_zone_door=true` on both paths. **On the disputed premise, the planner is right:** `probe-run.log`'s f3 line, with Lier's `InZone Graveyard` grant, shows `own_keywords_kind=true` for the graveyard Divination. So "the layer pass never materializes graveyard keywords" does not hold for a zone-scoped grant. `layer_pass_materializes_keywords` (`layers.rs:10398`) restricts only the identity-filter scan (`collect_scan_zones`, ~10448–10466). Choosing Snapcaster's `SpecificObject` grant as the fixture is the right repair.
- **F4 (L1 red-at-base).** V20 uses `#[should_panic]` with `max_steps = 0` after `commit()`. P16 (`unasserted_0=false asserted_0=true asserted_40=false settled_board_asserted_0=false`) shows the instrument fires and that V20b is not vacuous.
- **F5 and F6** are applied verbatim.

**rf-plan-review-r1 binding constraints:**
- F3 (scratch-carrier controller; V6b/V6c/V6d) is satisfied.
- F4 (Elemental Bond observer; Soul Warden confound measured in P1d) is satisfied.
- F5 (14-door disposition table plus V10) is satisfied. `rg "filter_inner_for_object(" game/filter.rs` gives exactly 14 non-recursive call sites, at 4022, 4310, 4649, 4669, 5063, 5100, 5146, 5215, 5240, 5316, 5364, 5416, 5661 and 5834. I mapped each one to its enclosing door, and the table's rows cover all of them.
- F9 (V19 required and discriminating) is satisfied.
- F7 (Rusted Relic verbatim, "Release Juno") and F8 (bullet verbatim) are present.

## Blocking findings

### F1 `text` LOW: V8's moved sub-case states a base value its fixture, as specified, does not produce (Verification Matrix, row V8)

**Evidence.**
- At base, the moved entrant (incarnation + 1, in the graveyard) is read by id: `object_has_effective_keyword_kind` → `off_zone_has_keyword_kind` → `effective_off_zone_keyword_contributions` (`off_zone_characteristics.rs:306–346`). That path starts from **`obj.base_keywords`** plus off-zone effects. It never reads `obj.keywords`.
- The `entry_observation_tests::fixture` creates the entrant with `create_object`, which leaves `base_characteristics_initialized` false, and then runs `mark_layers_full` / `flush_layers`. That first reset runs `sync_missing_base_characteristics` (`game_object.rs:2899`: `if self.base_keywords.is_empty() && !self.keywords.is_empty() { self.base_keywords = self.keywords.clone() }`) and then reseeds `keywords` from `base_keywords`.
- So a Flying set up so that it survives the fixture's flush, and `add_counters`'s later flush, sits in `base_keywords`. With only `keywords` cleared, the base answer for the moved sub-case is **true**, not the stated false.
- The sub-assertion would then pass at base and after the fix, so it would not discriminate. The removed case, false → true, is unaffected, so the row as a whole still discriminates.

**Coordinate:** Verification Matrix, row V8, column "Revert-failing assertion".
**Old:** "**false → true** [read]; plus the moved case (incarnation + 1, graveyard, keywords cleared): false → true [read]"
**New:** "**false → true** [read]; plus the moved case (incarnation + 1, graveyard, both `keywords` and `base_keywords` cleared; the base off-zone path, `off_zone_characteristics::effective_off_zone_keyword_contributions`, reads `base_keywords` plus off-zone effects, and the fixture's flush back-fills `base_keywords` from `keywords` through `sync_missing_base_characteristics`): false → true [read]"

### F2 `text`: Reference Readings omits several rows whose expected value equals the base value (check 9a)

Check 9a requires a derivation for every row whose expected value equals base. The matrix carries the following such rows that the Reference Readings list does not derive:
- **V6b's sibling** (P0 devotion 5 / P0 3 artifacts → creature);
- **the post-entry "a creature after entry" reach guards** of V1, V4, V5, V5b, V5c and V6d;
- **V10's positive guard** (`WithKeyword(Flying)` = true on the record);
- **the first-strike negatives** of V8 and V10.

My own derivations match the plan's values for every one of them. That makes this a missing bullet only, the same class as rf-p1-plan-review-r1 F5 (`text`).

**Coordinate:** Reference Readings. Insert a new bullet immediately before the bullet that begins "- **V19 cases 2 and 3.**".
**New:**
"- **V6b sibling; post-entry creature reach guards (V1, V4, V5, V5b, V5c, V6d); V10 positive guard; V8/V10 first-strike negatives.** V6b sibling: derived from Heliod's and Rusted Relic's text, CR 614.12, CR 700.5, CR 109.5 and CR 110.2a. The entrant enters under P0, so "you" is P0. With P0's pre-entry devotion to white at 5 (Heliod), or three P0 artifacts before entry (Rusted Relic), the condition already holds before entry, so the entrant is a creature as it would exist on the battlefield. Base also gives creature, because its scratch counts the entrant [read]; reference equals derivation. Post-entry reach guards: derived from Heliod ruling (b) and each card's text. After entry the live population counts the entrant: Heliod's {W} makes P0's devotion 5, Rusted Relic makes the third artifact, and Sanctum makes the fifth enchantment. So the condition holds and the entrant is a creature. Base gives the same [probed P1–P4, P13, P14]. V10 positive: the attack-declaration snapshot recorded Flying (CR 508.1a + CR 608.2i), so `WithKeyword(Flying)` on the record is true at base and after [probed P11]. First-strike negatives: no fixture object ever has first strike, so `HasKeywordKind(FirstStrike)` is false at base and after [read]."

## Sizing (check 12, blocking): consistent

- **T1 = 5:** U6 (L1/C5), U2 (B2/C2), U3 (C3), U1 (B1), U5 (C4). C6 is comment-only. V5b, V5c and V6a (f)–(h) test B1's carrier set, and V9b tests B2's door declaration. They are cases of an existing unit, not new behaviors, so the count stands. Counting them separately would not change T1 ≥ 2 in any case.
- **T2 = 8, or 5 if the test files are classed as fixtures.**
  - Product files (5): `game/layers.rs`, `game/filter.rs`, `game/keywords.rs`, `parser/oracle_effect/conditions.rs`, and `game/effects/change_zone.rs` (comment-only standing class).
  - Test files (3): `tests/integration/conditional_entry_gate_projection.rs`, `tests/integration/etb_trigger_entry_time_characteristics.rs`, `parser/oracle_effect/tests.rs`.
  - No compiler-forced site lies outside these files. `filter_inner_for_object` (5853), `matches_filter_prop` (8771) and `entry_filter_matches_staged` (5084) are private, and every caller is in `filter.rs`. `rg` finds only doc-comment mentions elsewhere. `object_has_effective_keyword_kind` keeps its signature, and `ConditionContext` gets a doc change only.
  - There is no `tests/integration/main.rs` edit.
- The T1 ∧ T2 conjunction does not fire. I agree with the plan.

## Checks passed (summary)

- **Class (1):** assessed against the charter's class attribution. U1 covers population and origin-zone leaves on every carrier the scratch can apply. U2 covers every kind-level keyword read at every door. U5 is the any-of building block.
- **Reuse and trace (2–3):** the traces are accurate. The fold path (`lower.rs:2758`) pushes whatever filter `ZoneChangedThisWay` carries. `TypeFilter::AnyOf` exists, and the projection's reads classifier handles it (`filter.rs:630`). The swallow recognizer (`sequence.rs:8317`) consumes the same converter.
- **Code facts checked:**
  - **U1:**
    - `StaticCondition::None` evaluates true (`layers.rs:2621`).
    - `static_condition_uses_object_population(None)` is false, so a pinned leaf is never re-pinned.
    - `evaluate_condition_inner`'s `IsPresent` arm reads `ability_holder` through `from_source_with_controller`.
    - `Definitions::get_mut` is copy-on-write.
    - `transient_continuous_effects` is an `im::Vector`.
  - **U3:** `record_and_emit_entry_from_no_zone` sets `entered_incarnation` (`zones.rs:1910`), so V11's incarnation filter bites.
- **Abstraction (4), idioms (5) and nom (6):** U5 changes a slice match only. `HeldObject` follows the door-declared-mode precedent, and it is not one-to-one with `ControllerLookup`, since `LiveOrLki` serves both live doors and skeletons.
- **CR (7):** all 27 cited numbers resolve in `docs/MagicCompRules.txt` to the rules the plan means: 614.12 / 614.12a / 614.1c / 700.5 / 109.5 / 110.2a / 613.1 / 613.1a / 603.10 / 603.10a / 603.6a / 608.2h / 608.2i / 608.2c / 611.2c / 611.3c / 400.3 / 400.7 / 702.26b / 702.34a / 707.2 / 708.2a / 111.3 / 113.6 / 114.4 / 508.1a / 122.6.
- **Oracle text:** the new r2 fixtures match MTGJSON verbatim: Copy Enchantment; Snapcaster Mage; Divination; Lier, Disciple of the Drowned (quoted with an ellipsis); Sanctum of Tranquil Light; Starfield of Nyx (relevant clause); Rusted Relic; Elemental Bond; Memnite.
- **Verification matrix (9), identity (10) and scope (11):** every unit has a row that is red at base. Hostile owner ≠ controller, copier ≠ original controller, departed and re-incarnated fixtures are present. The C1 rows are `DEFERRED(phase 2)` with a named landing phase.
- **Unprobed lib rows (U-a: V6a, V6b, V7–V9, V11).** These are acceptable under the skill. None is a premise the design rests on; each is an expected test value that the executor's red-at-base run measures.
  - V6a has no base reading, because the function is new.
  - I re-derived V7 from the fixture and the `WithKeyword` shape assertion, and V11 from `fixture(false)`, `entered_incarnation` and the record arm. Both agree with the plan.
  - V6b uses the mechanism P4 measured at runtime.
  - V9's live arm is the unchanged id-keyed path.
  - The one wrong derivation is V8's moved sub-case (F1).

## Non-blocking notes

- **Charter decision 7 wording.** The charter says `drive_to_empty` "tracks whether it broke on an empty stack at Priority, and asserts that after the loop". The plan instead asserts the state postcondition after the loop. That is strictly stronger: a run whose final permitted step settles passes, and the goal of the decision is unchanged. The orchestrator may want one addenda line recording the mechanism.
- **V6a (d).** Build the "graveyard-count" leaf from `GraveyardSize` / `ZoneCardCount`, not from an `ObjectCount` over a graveyard filter. `quantity_ref_uses_object_count` (`quantity.rs:2659`) classifies every `ObjectCount` as population, so that leaf would pin even with `origin = Some(Hand)`, and the "untouched" half of the case would fail.
- **Pinned truth reads `pre_entry` unflushed.** In practice the state is clean at a gate: resolution flushes before each sub-ability (`effects/mod.rs:18351`) and at priority. A one-line doc note on `pin_entry_population_conditions` stating that assumption would keep it visible.

## Pre-existing (untagged, non-blocking)

None reported. I did not hunt beyond the change's own claims.
