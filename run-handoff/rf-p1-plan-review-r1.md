# Phase-1 plan review r1: etb-review-fixes (rf-p1-plan-r1.md), design round 2 of 5 of the plan loop

Reviewer: fresh Step 2 `review-engine-plan`, **phase-plan mode** (charter `phase-charter`, phase index 1, allowlist: all C1 content `DEFERRED(phase 2)`). Phase-fit context declared, so check 12 (Sizing) is blocking.
Base: `addbc884ee251794a64f2e1da0487cd329f8ce90` (checkout clean, untouched). Evidence: code read at base; MTGJSON `AtomicCards.json`; card-data `/home/user/wt/data-out/p2/card-data.json`; `docs/MagicCompRules.txt`. No build was run: every finding below is settled by a quoted code read.

**Verdict: design round.** `behavior` = 4 (F1 MED; F2, F3, F4 LOW), `text` = 2 (F5, F6), `machinery` = 0.

Prior-round constraints: F3 (pin controller: V6b/V6c/V6d, scratch-carrier semantics), F4 (Elemental Bond observer, Soul Warden confound measured), F5 (per-caller disposition table, sibling row V10), F9 (V19 required and discriminating), F7 (Rusted Relic verbatim, "Release Juno") and F8 (bullet present verbatim) are all satisfied. Oracle text in Step 0 matches MTGJSON for every card I checked (Heliod and both 2020-01-24 rulings, Recommission, What Must Be Done, Rusted Relic, Starfield of Nyx, Sanctum of Tranquil Light, Elemental Bond, Memnite, Sol Ring). Every CR number in Step 0 resolves to the stated line in `docs/MagicCompRules.txt`.

---

## Blocking findings

### F1 `behavior` MED: copy-carried statics escape the pin; R5(c) states the mechanism backwards (U1, Residuals R5(c))

**Evidence.**
- A layer-1 copy is a transient `ContinuousModification::CopyValues { values: Box<CopiableValues>, .. }` (`types/ability.rs`, `enum ContinuousModification`). `CopiableValues` carries `pub static_definitions: Arc<Vec<StaticDefinition>>` (`types/ability.rs`, `struct CopiableValues`), latched when the copy was made.
- In the pass, `printed_cards::apply_copiable_values` installs them: `obj.static_definitions = Arc::clone(&values.static_definitions).into();`, and `layers.rs` refreshes the generator index after layer 1 so the copied statics generate effects in the same pass ("a static granted there must generate effects for layers 2-7 of THIS pass").
- U1 pins only `base_static_definitions` of scratch objects and `tce.condition` of transients, before the pass. A copy's latched `values.static_definitions` is neither, so a copied population-gated static is evaluated unpinned against the scratch board, which includes the entrant. That is exactly B1's defect, in the "another object's group static" class that charter decision 1 and the phase's class attribution (V5) cover.
- R5(c) says the opposite: "a layer-1 copy effect in the scratch copies a source's pinned definitions … no gate consumer's entrant is projected as a copy [unprobed]". The copy does not read the source's (pinned) definitions; it carries its own unpinned snapshot. And the reach does not need the entrant to be a copy: the copier is the *source* of the group static that types the entrant.
- Printed reach (both cards parse with no `Unimplemented` in card-data): **Copy Enchantment** ("You may have this enchantment enter as a copy of any enchantment on the battlefield.") copying **Starfield of Nyx**; P0 controls the copy plus 3 other enchantments (pre-entry 4), and resolves Release Juno on **Sanctum of Tranquil Light**. Derived (CR 614.12 + Heliod ruling (a)): the copied static counts 4, so Sanctum is not a creature as it would enter, so 0 counters. Under the plan the copier's static counts 5 with Sanctum in the scratch, so Sanctum gets 2 counters: the same wrong reading V5 fixes for the original Starfield.

**Required revision.**
- Pin the conditions of every static definition carried in a scratch transient's `CopyValues` payload (copy-on-write, same leaf rule, `pinned_static_condition`), with the controller the scratch pass would use for that static: the copy's affected object's scratch controller (the copier is the static's source, CR 109.5; the effect's recipient is bound on the transient). Alternatively pin after layer 1, if the planner can do so without moving the authority out of `layers.rs`. Charter decision 1 already names "every scratch object's static definitions" as the carrier set, so no charter change is needed.
- Add a discriminating `/card-test` row in `conditional_entry_gate_projection.rs`: Starfield of Nyx (P1) copied by Copy Enchantment (P0, cast and resolved, choosing Starfield) + 3 other P0 enchantments with mana value ≥ 1, Release Juno on Sanctum: counters 2 → 0. Positive control with 4 others: 2 → 2. Reach: the copy is on the battlefield as Starfield of Nyx; Sanctum is on the battlefield and a creature after entry (live count 5).
- Replace R5(c) with the true mechanism and its disposition.

### F2 `behavior` LOW: U1 writes the carrier field the full pass does not reseed for non-battlefield sources, and omits functioning off-battlefield statics (U1 "Controller" bullets, "Rewrite" bullet)

**Evidence.**
- U1 writes `Arc::make_mut(&mut obj.base_static_definitions)[i].condition` and relies on the full pass's seed `obj.static_definitions = Arc::clone(&obj.base_static_definitions)`. That seed (`seed_live_characteristics_from_base`) runs from `reset_recipient_to_base`, which `derive_layer_characteristics` calls only for `state.battlefield_phased_in_ids()` (plus a keywords-only hand reset). A command-zone object's `static_definitions` is never reseeded, while `static_source_effects_in_pass` reads `source.static_definitions`. `Arc::make_mut` on a shared `Arc` clones, so the pinned `base_static_definitions` diverges from the `static_definitions` the gather reads: for every command-zone carrier the pin the plan claims is a no-op.
- The plan justifies its carrier list with "exactly the population `for_each_static_effect_source` draws from". That is false: after the battlefield and command loops, `for_each_static_effect_source` also visits every other object whose statics carry an opt-in `active_zones` or a CDA (the Incarnation cycle: "As long as this card is in your graveyard and you control a [land type], …", an `IsPresent` population leaf). Those carriers are not pinned either.
- Charter decision 1 names the carriers as "every scratch object's static definitions".

**Required revision.** Take the carriers from every object in `scratch.objects` (pinning a static that does not function is inert), and write the pinned tree to the field the gather reads as well as the base: `static_definitions` and `base_static_definitions`, so both the reseeded battlefield path and the unreseeded command or graveyard path carry it. Add a V6a case: a command-zone (emblem) carrier with a population condition is pinned in the field `static_source_effects_in_pass` reads. Remove the "exactly the population" sentence.

### F3 `behavior` LOW: U2 turns a correct off-zone answer into a wrong one on the owner-zone clone path (U2 disposition row "`matches_target_filter_in_owner_zone`, clone path"; Residuals "U2 residual")

**Evidence.**
- `matches_target_filter_in_owner_zone` clones the live object only to set `controller = owner` (`let mut owner_scoped = obj.clone(); owner_scoped.controller = owner_scoped.owner;`). The clone is the live card of that id, incarnation and zone. It is not a snapshot.
- At base the kind arms read `object_has_effective_keyword_kind(state, object_id, ..)`: the live card's off-zone effective keywords, including off-zone grants (for example "Each instant and sorcery card in your graveyard has flashback"). After U2 the clone fails `std::ptr::eq`, so it answers from `clone.keywords`. The layer pass never materializes graveyard or library keywords (`layers.rs` `layer_pass_materializes_keywords`: Battlefield | Hand | Stack only), so the grant is dropped.
- The plan's row cites CR 109.5, which concerns "you", not keywords. No CR drops an off-zone grant from a card because its controller field was rescoped. Charter decision 4 keeps "an off-battlefield object that is the live object of its id" on the off-zone path, and this clone is that object. `matches_target_filter_for_zone`'s doc keeps this path as defence-in-depth for hand-built and serialized states where controller and owner have diverged, so "production-unreachable" does not license a wrong answer there.

**Required revision.** The owner-zone clone's kind read must keep today's off-zone effective answer. For example, have the clone path evaluate kind props against the live object it cloned, or move the owner substitution to the already-threaded `ControllerLookup::OwnerZone` instead of the clone. The planner picks the mechanism; `std::ptr::eq` alone cannot tell this clone from a snapshot. Correct the row's "Reading after U2" and CR column, delete the U2 residual, and add a unit row: a graveyard card with `controller != owner` and an off-zone-granted kind answers true through `matches_target_filter_in_owner_zone` both before and after.

### F4 `behavior` LOW: L1/C5 has no test that is red at base (V20)

**Evidence.** The task's acceptance is "every phase-1 finding fixed with a discriminating test red at base". V20's revert-failing assertion is "structural (no non-settling caller exists to drive red)": with U6 reverted, every current caller still passes, so no assertion discriminates the fix.

**Required revision.** Give the harness a discriminator. For example, `drive_to_empty` delegates to `drive_to_empty_within(runner, pick, branch, max_steps)` (40 for existing callers, behavior unchanged), and add a `#[should_panic(expected = "did not reach Priority on an empty stack")]` test that calls it with `max_steps = 0` on a runner whose stack is non-empty (or not at Priority). With the settled assertion reverted, the call returns silently and the test fails. Update the V20 row with this test and its base → fixed outcome (no panic → panic).

### F5 `text`: V11's record-arm parity is missing from Reference Readings (check 9a)

V11's hostile column asserts "record arm answer equals `matches_target_filter_on_zone_change_record_licensed`", a parity row the Reference Readings list does not derive. My derivation agrees with the plan's stated value and with the existing `entry_observation_absent_matches_todays_arms` record-arm assertions on the same fixture, so the repair is the missing bullet only.

**Coordinate:** Reference Readings, the bullet beginning "- **V11 positive sibling / `entry_observation_absent_matches_todays_arms`.**".
**Old:** "- **V11 positive sibling / `entry_observation_absent_matches_todays_arms`.** Derived: the same incarnation is the same object (CR 400.7), so the live answer is the entrant's. Base equals derivation [read]."
**New:** "- **V11 positive sibling / `entry_observation_absent_matches_todays_arms`.** Derived: the same incarnation is the same object (CR 400.7), so the live answer is the entrant's. Base equals derivation [read]. **V11 record-arm parity.** Derived: once the live object is a later incarnation, the entered object no longer exists (CR 400.7), so the entry record answers: the 1/1 Bear as it entered, before the two counters. `power_at_least(3)` is false. That is the value `matches_target_filter_on_zone_change_record_licensed` returns for the record (reference: the record-arm assertions of `entry_observation_absent_matches_todays_arms` on the same fixture), so reference equals derivation [read]."

### F6 `text`: R5(d) leaves unestablished a reach claim the charter assigns to phase 1

Charter R5: "Whether any gate consumer reaches these is a claim phase 1 establishes." The plan writes "not measured [unprobed]" for (d). (R5(c) is replaced under F1.)

**Coordinate:** Residuals, R5 item (d).
**Old:** "  - (d) `ForAsLongAs` durations of transients are pruned, not pinned: not measured [unprobed];"
**New:** "  - (d) `ForAsLongAs` durations of transients are pruned, not pinned. No rules-correct gate reach: a resolution-created effect that modifies characteristics fixes its affected set when it begins (CR 611.2c, verified in `docs/MagicCompRules.txt`), so it never applies to an entrant that was not on the battlefield then, and an effect that only modifies the rules writes no type or subtype a gate reads. An engine transient that applied to the entrant anyway would be a CR 611.2c defect present at base, outside this phase [derived];"

---

## Sizing (check 12, blocking): consistent

- **T1:** I count 5 units: U6 (L1/C5), U2 (B2/C2), U3 (C3), U1 (B1), U5 (C4). C6 is comment-only. The body names no other independently tested behavior: V6a–V6d, V3 and the controls are all B1, and V7–V10 are all B2. This matches the plan: **T1 = 5, fires.**
- **T2:** product paths `game/layers.rs`, `game/filter.rs`, `game/keywords.rs`, `parser/oracle_effect/conditions.rs`, `game/effects/change_zone.rs` (comment-only standing class) = 5; test paths `tests/integration/conditional_entry_gate_projection.rs`, `tests/integration/etb_trigger_entry_time_characteristics.rs`, `parser/oracle_effect/tests.rs` = 3. **T2 = 8, or 5 with test files classed as fixtures. This matches the plan.** There are no compiler-forced sites (no signature, type or variant change) and no `tests/integration/main.rs` edit. The F1–F4 fixes stay inside `layers.rs`, `filter.rs`, `keywords.rs` and the integration test files, so they do not move T2. The T1 ∧ T2 conjunction does not fire.

## Checks passed (summary)

- **Class (check 1):** assessed against the charter's class attribution. U1 covers the population/origin-zone leaf class (with the F1/F2 carrier gaps), U2 the non-live kind-read class, and U5 the any-of building block.
- **Reuse/trace (checks 2–3):** the traces are accurate. `filter_inner_for_object` has exactly the 14 callers the disposition table lists (`rg "filter_inner_for_object(" game/filter.rs` → 4022, 4310, 4649, 4669, 5063, 5100, 5146, 5215, 5240, 5316, 5364, 5416, 5661, 5834, plus recursive calls). The multi-type `RevealedHasCardType` evaluator matches with `any` (`game/effects/mod.rs`, the `RevealedHasCardType` arm), and the parser emits multi-type lists only for genuine disjunctions (`parse_core_type_disjunction` under `all_consuming`; a conjunctive "artifact creature" falls through). So `TypeFilter::AnyOf` is the consistent lowering.
- **CR (check 7):** every cited number was verified.
- **Identity (check 10) and scope (check 11):** owner and controller are covered by V6b/V6c/V6d. The departed and re-incarnated entrant is covered by V8/V11.
- **Verification matrix (check 9):** every row has a revert-failing assertion and a reach guard, except V20 (F4). V19 is now required and discriminating (F9).

## Non-blocking notes

- U1 transients: with `ability_holder(tce.controller)` the pin's `IsPresent` names the effect's controller. The live scratch pass evaluates a transient's `IsPresent` through `FilterContext::from_source(state, source_id)`, which is the source object's current controller. Charter decision 2 mandates the former, which is the CR 109.5 reading. The two differ only when a transient's source object changed controllers. Worth one sentence in the `ability_holder` doc so the deviation from "the player the scratch pass would use" is intentional on the page.
- U2: using `std::ptr::eq` as the live-ness test is precedented, but it classifies by allocation, not by meaning (see F3). If F3 is fixed by routing the owner-zone path differently, record in `effective_keyword_kind_of`'s doc that a caller holding a rescoped view of the live object must pass the live object.

## Pre-existing (untagged, non-blocking)

None reported. I did not hunt beyond the change's own claims.
