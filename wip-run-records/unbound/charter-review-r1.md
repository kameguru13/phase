## Charter review: Unbound Flourishing, phase-charter r1 (charter mode)

**Verdict: clean round.** I found no decision findings, only corrections (C1–C8 below). The orchestrator applies them; none goes back to the planner.

I ran no cargo builds, tests or probes. Code was read at 6295aa7 with `git show`. The Zimone phase-2 WIP was ignored.

### What holds
- **The charter reached the right decisions without the full review text.** Its planner saw `plan-review-r1.md` as empty (charter:5–13). The file now has 90 lines, and I checked every F1–F6 and every note against the charter. Nothing in the full text contradicts a frozen decision.
  - **F1** is resolved by phase 2, decision 4 (charter:150–153).
  - **F2** is resolved by phase 2, decision 5 (charter:154–159). The bump numbers come from the phase base, so they come out at 145/127 after Zimone. The Zimone charter (zimone/phase-charter:86, 162) confirms its phase 2 adds no bump.
  - **F3** is resolved by phase 1, decision 4 (charter:80–83).
  - **F4** is resolved by phase 1, decision 5 (charter:84–86). It is stricter than the review's "decline, or condition present in each half", and that is acceptable.
  - **F5** is carried as a constraint (charter:97), with one scope path missing (C2).
  - **F6** is carried, but without a landing phase (C3).
  - **Notes:** note 1 (journal) is decided in phase 3, decision 5 and phase 2, decision 6. Note 3 (mana abilities) is declared out of scope at run-level deferral 1. Note 5 (U4 over-prompting) is narrowed in phase 4, decision 1. Note 4 is phase-plan detail. Note 2 is covered under the residual note below.
- **Seams.** All three are green-tree safe:
  - 1→2: a stifled-ability copy resolves to `CopySourceLookup::Gone`, so no copy is made (copy_spell.rs:900–917).
  - 1→3: line 1 stays `Unimplemented`.
  - 3→4: Krasis auto-orders in the interim, and the charter records this divergence (charter:226).
  - Ordering is infrastructure before consumer.
  - Every deferral names its landing phase, or is an explicit run-level out-of-scope item.
- **Recursive gate.** No single phase trips T1∧T2:
  - phase 1 is about 6 paths;
  - phase 2 is about 8 even with C2 added;
  - phases 3 and 4 are one unit each.
- **Step 0 is present.** Unbound's Oracle text matches oracle-verified.md. I re-read the four class cards from `data/mtgjson/AtomicCards.json`, and they match the quotes at charter:20–23.
- **CR numbers.** All 20 cited at charter:24 exist in docs/MagicCompRules.txt, including 605.5a, whose "triggers from activating a mana ability but couldn't produce mana" clause fits charter:266.
- **Every addenda file is named.**
- **Code-state assertions that decisions rest on, measured by reading:**
  - The in-place, unjournaled stack-mutation class exists: resolved_commands.rs:1226–1233 says "none of it is journaled today". This supports phase 3, decision 5.
  - Mana abilities do not trigger today. `pending_activations` is pushed only in `record_activated_ability_placed`, at stack placement (casting.rs:27952), and `pending_activation_cost_has_x` reads it (casting_costs.rs:17127). This supports run-level deferral 1.
  - The departed-spell lifecycle sites exist: stack.rs:645/669–676 and turns.rs:1303.
  - The protocol precedents exist in lobby-broker protocol.rs: v139 at line 100, v142 at line 80, v144 at line 63 (4 new types), and v129 at line 141.
  - The gap_diagnosis pin exists at gap_diagnosis.rs:1173.

### Corrections (for the orchestrator to apply)

**C1. Phase 2, decision 4 (charter:153): the example redactions are wrong. Replace them with the measured ones.**
- Old: "for example, private cleanup authority, hidden choices, and face-down source identity. It also adds a projection test in the existing departed-spell redaction test pattern."
- New: "at minimum the two F1 names: `activation_record = None` (`redact_activation_records`, visibility.rs:1241–1296) and creation-lookback hidden-zone-change redaction (`for_each_creation_lookback_event_mut`, visibility.rs:1701+). Both apply through `filter_state_for_scope`, which serves the seated and unseated viewer. Also any casting-permission cleanup redaction that `project_paid_cast_cleanup_authority` (visibility.rs:50–90) applies to departed spells."
- Dropping the projection-test sentence follows the architecture-only rule: the test is F1's acceptance row and belongs in the phase plan.

**C2. Phase 2 scope is missing a path that is not compiler-forced (F5).**
- `crates/engine/src/analysis/resource.rs` calls `retain_trigger_referenced_departed_spells` at lines 5710–5711.
- Phase 2, decision 2 requires a sibling retention pass "beside" the existing one. Adding a call is not compiler-forced.
- Fix: add `crates/engine/src/analysis/resource.rs` to the anchors at charter:122–128. (The other caller, game_state.rs:30734, is already in scope.)

**C3. F6 needs a landing phase, and the stale input-gap text should go.**
- charter:5–13: replace the "Input gap" paragraph with "Full plan-review r1 text: `.git/engine-implementer-runs/unbound/plan-review-r1.md` (90 lines); charter review r1 checked F1–F6 against it."
- charter:98: replace the F6 bullet with "F6: the two missing ruling rows land here: ruling 8 (an {X} only in an additional cost does not trigger) and ruling 7's copy count (two Unbounds copy twice)."
- charter:162 and 221: change "F6" to "F6 (n/a; landed in phase 1)".
- charter:250: change "F6" to "F6 (n/a; landed in phase 1)".

**C4. Phase 2 scope (charter:136) is not a literal path.**
- Old: "plus whatever adapter tests the Zimone bump moved in lockstep (the phase plan reads `git show 6295aa7 --stat` for the set)"
- New: "`client/src/adapter/__tests__/p2p-adapter-multiplayer.test.ts`, `client/src/services/__tests__/openPhaseSocket.test.ts`". These are the remaining protocol files in `git show 6295aa7 --stat`.

**C5. Phase 4 scope (charter:243) is not a literal path.**
- Old: "The ordering parity-sweep pin lists, if the sweep requires an entry (comment and evidence lists; never a floor bump)"
- New: "`crates/engine/src/game/triggers_ordering_parity_tests.rs` (`SAME_EVENT_EVENT_OBJECT_GENUINE` at :367 and kin; entries only, never a floor bump)"

**C6. Phase 1, decision 7 (charter:89): the citation does not support the sentence, and the second half is a "phase establishes" claim.**
- Old: "(ruling 2; CR 603.2: they are not activations). The phase plan establishes this from the emitters of the activation event (draft V16/P5). It is not assumed."
- New: "(ruling 2; CR 113.3b / CR 602.1 define activated abilities by their '[Cost]: [Effect]' form; a triggered ability, CR 113.3c / CR 603.1, is never activated)."
- CR 603.2 only says that triggers trigger. The proof belongs to the phase plan.

**C7. Make the single-bump ship coupling explicit (phase 2, decision 5, charter:157–159).**
- The bump is in phase 2, but phase 3 adds a serialized `Effect` tag under the same number. That is correct only because all phases ship in one PR (charter:15).
- Append to decision 5: "Phases 2 and 3 ship in the same merge. `/ship-commits` must not land phase 2 without phase 3. If the run is split at ship time, phase 3 takes its own bump."

**C8. Phase 1, decision 5 (charter:84–86) and the constraints (charter:96): scope the honest-red rule, and drop the acceptance-row phrasing.**
- Append to decision 5: "The honest-red rule covers drops the splitter would itself introduce. Vazi's existing silent drop of its Treasure condition (`condition: null` in today's card data; plan-review r1 'Pre-existing') goes to triage. Phase 1 does not depend on it."
- charter:79, delete: "The phase plan must establish that a rechecked activation-cost leaf would fail at resolution. That is the decisive hostile case behind this decision, and the charter does not assert it."
  - The reason is already on record: `pending_activation_cost_has_x` reads `pending_activations`, which is cleared on PassPriority (plan-review r1:10).
- charter:96: replace "Each decline path gets a hostile row reached through `parse_trigger_lines`, paired with the Unbound positive" with "decisions 4 and 5; the rows are the phase plan's."

### Residual note (not blocking)
Phase 3, decision 2 (charter:204–206) keeps an activated-ability arm in the X-rewrite authority, and no in-run card reaches it (review note 2). The arm has a real future consumer: Glava's "or activate an ability with {X} in its activation cost … become 5". So keeping it is defensible. The phase-3 plan should still give it, and the `state.announced_source_x` channel (triggers.rs:17160), a direct test, or make the arm a no-op.

### Pre-existing
None new. Vazi's dropped condition is already listed in plan-review r1.
