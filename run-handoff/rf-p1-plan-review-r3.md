# Phase-1 plan review r3: etb-review-fixes (rf-p1-plan-r3.md), the executor stop-and-return round

Reviewer: fresh Step 2 `review-engine-plan`, **phase-plan mode**. Inputs: charter `phase-charter`, phase index 1, allowlist (all C1 content `DEFERRED(phase 2)`), addenda `addenda/phase-1`, attempt history `phase-fit`. The phase-fit context is declared, so check 12 (Sizing) is blocking.

Base: `addbc884ee251794a64f2e1da0487cd329f8ce90`. The main checkout was clean before and after this review (`git status --short` empty). `git worktree list` shows only the main checkout and the unrelated `/home/user/wt/complete`. I ran no build.

Evidence used:
- code read at base: `sequence.rs` 8270–8335 and 16374–16400, `swallow_check.rs` 4225–4275, 4441–4560, 4655–4675 and 8630–8700, `conditions.rs` 1595–1660 and 1849–1920, `oracle_effect/mod.rs` 43270–43295, `oracle_effect/tests.rs` 87687–87810 and 59197;
- the executor patch `p1-exec-r1.patch`, for the conditions.rs and tests.rs hunks and the `MULTI_TYPE_RELEASE_JUNO` constant;
- the planner's r3 probe source `p1r3/zz_p1r3_probe.rs` and its logs `base2.log`, `change.log`, `counterfactual.log` and `lib-change.log`, each exit 0, ending in `PROBE_DONE` or `test result: ok. 33 passed`;
- MTGJSON `AtomicCards.json`, with my own wider scan;
- `docs/MagicCompRules.txt`.

**Verdict: no `behavior` finding.** `behavior` = 0, `text` = 1 (F1), `machinery` = 0.

## The r2 → r3 delta

I read every sentence the "Changes from r2" table and the r3-marked passages add.
- **Scope admission vs the charter.** `sequence.rs` is a charter read-only anchor. A test-module edit adds non-comment lines, so it is not a standing class. Routing the edit through the plan step is correct (chartered.md §Scope rule; charter phase 1 "Editing a read-only anchor returns to the phase's plan step").
- **Decision 6 stands.** Decision 6 says "The converter stays the single authority that both the fold and the sequence-level swallow recognizer consume." The admission carries that out. It does not change it.
- **No other decision is touched.**
  - Decision 9's out-of-bounds files (`effects/mod.rs`, `types/ability.rs`, the Zimone path, the non-modal `enters_with_gate` routing gap) are untouched.
  - No phase boundary, order, seam or deferral moves.
  - T2 stays below 13 (see Sizing).
  - So this is **addendum-class drift, not a re-charter**, and the addendum line already in `addenda/phase-1` (decision 6) records it accurately.

## Checks on the delta's claims

### Production reach of the recognizer
Checked by `rg is_moved_object_entry_this_way_counters_clause`:
- The only production call is `swallow_check.rs:4265`, inside `conditional_enter_counters_if_is_only_if_marker`.
- That function's only caller is `detect_condition_if` (`:4559`).
- Every other hit is a test: `sequence.rs:16387/16396`, and `swallow_check.rs:8648–8682`, all in `condition_if_still_flags_unrepresented_entry_this_way_voices`.

The converter `card_type_condition_as_entry_this_way` has two consumers: `anaphoric_entry_counter_gate` (`conditions.rs:1658`), and through it the fold at `mod.rs:43291` and the recognizer at `sequence.rs:8319`. The plan's trace is accurate.

### What "multi-type" means here
`parse_its_a_card_type_gate_body` (`conditions.rs:1892–1919`) builds a `card_types` list of more than one type only from a fully consumed **disjunction** (`parse_core_type_disjunction`). A conjunctive stack such as "artifact creature" falls through to the single-type path. So mapping more than one type to `TypeFilter::AnyOf` agrees with the parse, which is also what the executor's doc says.

### P19 is believable
- The probe source calls the public `parse_oracle_text` and reads `parse_warnings` `SwallowedClause` detectors.
- Its positive control (verbatim What Must Be Done: `slot=true`, `swallowed=[]`) fires in all three states.
- The counterfactual run shows that the instrument can report `Condition_If` on the same input.
- The (i)/(ii)/(iii) texts V19b specifies are byte-equal to the probe's inputs:
  - `MULTI_TYPE_RELEASE_JUNO` is identical in the probe and in the executor's V19 constant;
  - (ii) equals `MODAL_HEAD + TAIL_MULTI_PLUS_PRONOUN`;
  - the verbatim text matches MTGJSON exactly.
- The mechanism behind the base values agrees with the code:
  - at base, `has_slot("condition")` (`swallow_check.rs:4665`) is fed by the `PutCounter` sub's `RevealedHasCardType`, which suppresses `Condition_If`;
  - `conditional_enter_counters_if_is_only_if_marker` returns false without the `conditional_enter_with_counters` slot.

### The audit change is correct, not a side effect
The rider folds into the typed slot. V19 proves the runtime, and the base `Replacement` swallow was the C4 defect. V19b (ii) keeps the pronoun rider visible after the change. That matches the existing contract test `represented_typed_rider_does_not_hide_an_unrepresented_pronoun_rider`.

### Printed reach is 0
I widened P20, because its `pat1` misses three-item lists ("an X, Y, or Z"). I scanned every printed sentence containing "enters with … counter(s)" with an "if" and an "or" (`scratchpad/rv-r3/scan.py`): 14 hits. None of them is an anaphoric "if it's an X or Y" type gate:
- Spark Double's riders are single-type;
- the rest are numeric or game-state conditions.

So P20's conclusion holds.

### V18b
- **Red at base.** The two multi-type strings sit in the `represented` loop, and the converter returns `None` for two types at base.
- **The "put" negatives are rejected in both states.** `anaphoric_entry_counter_gate` requires the body to parse as `parse_moved_object_enters_with_counters`, and "put two …" fails that, whatever the type list.
- **The count is right.** The represented list is 4 + 2 = 6 strings, which agrees with P21's log.

### V19b
- **Red at base:**
  - (i): `["Replacement"]`, and the reach-guard slot is absent;
  - (ii): `["Replacement"]`, no `Condition_If`.
- **Helpers exist.** `battlefield_move_entry_gate` and `typed_entry_gate` are at `tests.rs:87687/87700`, and `parse_oracle_text` is imported at `tests.rs:14`.
- **Placement.** The precedent named is real (`dont_blink_lifts_to_floating_zone_redirect_replacement`, `tests.rs:59197`).

### CR
The r3 additions cite CR 614.1c, CR 614.12 and CR 608.2c. All three resolve in `docs/MagicCompRules.txt` (lines 3066, 3104 and 2799) to the rules meant.

### Stale docs
- `rg "several types|multi-type|artifact or creature\."` over `conditions.rs`, `sequence.rs`, `swallow_check.rs`, `lower.rs` and `mod.rs` finds only the `card_type_condition_as_entry_this_way` doc (U5 edits it), the sequence test doc (V18b edits it), and unrelated hits (`sequence.rs:2071`, `:10677`; `mod.rs:7662`).
- The production docs on `is_moved_object_entry_this_way_counters_clause` and `is_anaphoric_entry_counters_clause`, and the fold comment at `mod.rs:43287`, stay true.
- So no read-only anchor needs a comment edit.

## Blocking findings

### F1 `text` LOW: the V18b reach guard's four preserved `represented` strings have no reference reading (check 9a)

**Evidence.** V18b's reach-guard column relies on "the six-string `represented` list". Four of those six strings are accepted at base and after, so they are preservation assertions: the What Must Be Done trailing voice, the leading single-type voice, the Heroic Return typed voice, and the Oviya active voice. Reference Readings derives V18b's negatives ("V18b negatives (r3)") but not these four. This is the same class as rf-p1-plan-review-r2 F2, which covered reach guards.

My derivations agree with the expected value (accepted) for all four. One of them rests on a pre-existing engine reading rather than on the CR: Oviya's "put two +1/+1 counters on it" is a CR 608.2c later instruction that the engine folds into the entry slot. The run already carries that as a follow-up (phase-fit entry 6, "Oviya fold (already follow-up #6)"). The bullet must say so rather than present it as a rules derivation.

**Coordinate:** Reference Readings. Insert a new bullet immediately after the bullet that begins "- **V18b negatives (r3).**".

**New:**
"- **V18b preserved positives (r3).** The four existing `represented` strings are accepted at base and after (the old test is green at base; P21 after), and the derivation agrees. (1) "It enters with two additional +1/+1 counters on it if it's a creature." (What Must Be Done's verbatim Release Juno rider) and (2) "If it's an artifact, it enters with an additional +1/+1 counter on it." are "enters with" riders on the moved permanent, so they are entry replacements (CR 614.1c + CR 614.12). The fold represents them in `conditional_enter_with_counters` (`what_must_be_done_rider_lowers_to_entry_gate`, `anaphoric_leading_voice_lowers_to_entry_gate`), so the recognizer must accept them. (3) "If a Hero enters this way, it enters with two additional +1/+1 counters on it." (Heroic Return's voice) is the same entry replacement in the typed present-tense voice, represented by the same slot, so it is accepted. (4) "If you put an artifact onto the battlefield this way, put two +1/+1 counters on it." (Oviya's voice) is accepted because the engine folds it into the same slot, which is the audit contract the recognizer mirrors. Rules-wise that clause is a later instruction (CR 608.2c), not an entry replacement. This is a pre-existing engine reading that the run carries as a follow-up (Oviya fold, follow-up #6) and that phase 1 does not change. The row asserts acceptance as the audit's agreement with the lowering, not as a CR 614.1c derivation."

## Sizing (check 12, blocking): consistent

**T1 = 5** (U6 L1/C5, U2 B2/C2, U3 C3, U1 B1, U5 C4), my own count.
- V18b and V19b are test rows of U5. The recognizer has no code change: it reads U5's converter, and both rows are discriminated by that one edit.
- C6 is comment-only.
- Even counting the audit effect as a sixth behavior would leave T1 ≥ 2, so the conjunction is unaffected.

**T2 = 9 counted paths**, my own count over the extended list. Every path the plan body edits is in the list, and no other path is edited:
- Product files (5): `game/layers.rs` (including the `ConditionContext::ability_holder` doc), `game/filter.rs`, `game/keywords.rs`, `parser/oracle_effect/conditions.rs`, and `game/effects/change_zone.rs` (comment-only).
- Test-bearing files (4): `parser/oracle_effect/sequence.rs` (test module only), `parser/oracle_effect/tests.rs`, `tests/integration/conditional_entry_gate_projection.rs`, `tests/integration/etb_trigger_entry_time_characteristics.rs`.
- Classing the test files as fixtures gives 6 or 5, as the plan says.
- Not counted, correctly: `swallow_check.rs` and `lower.rs` (read only), `tests/integration/main.rs` (no new `mod` line), and card data (regenerated, excluded).

T2 < 13 under every classing. **The T1 ∧ T2 conjunction does not fire.** The Sizing table's U5 row names `sequence.rs`, and its four rows (V18, V18b, V19, V19b) match the matrix. The plan's Sizing is consistent with its body.

## Non-blocking notes

- **Card name in V19b.** V19b parses under the name "What Must Be Done", while P19 parsed under "Probe Card". None of the three texts contains the card name, so `~` normalization cannot differ. Using "Probe Card" would keep the test byte-identical to the measured probe.
- **Duplicate test filter.** Step 5's lib filter `multi_type_entry_rider` already matches V19b's name `multi_type_entry_rider_is_represented_not_swallowed`, so "plus the V19b name" is redundant (harmless).
- **Stale trace line.** The kept r2 Step 2 trace line for U5 still says "(also consumed by `sequence.rs::is_anaphoric_entry_counters_clause`)". The new r3 bullet directly below corrects the entry point, so a reader is not misled.
- **Round count.** The header's "r2 was plan-loop design round 3 of 5" restates the r2 review's own header. Per phase-fit entries 10, 12 and 15, the design-round count is 3 only with this executor return included. The orchestrator's ledger governs.

## Pre-existing (untagged, non-blocking)

None reported. I did not hunt beyond the change's own claims.
