# Phase 1 plan — Merge onto main's authorities (cz86-rework-9670)

Mode: `engine-planner` phase-plan mode for charter phase **P1**. Inputs: `task.md`, `phase-charter`
(frozen), `phase-inputs/phase-1`, `phase-inputs/run-level`, `addenda/phase-1` (the maintainer's
13:51Z conditions on Q2 and Q4, binding), `maintainer-answer-2026-11-17T1351.md`, `plan-r1.md`
(§13–§15 reused and corrected below), `plan-review-r1.md`.

- `PHASE_BASE_SHA` = run `BASE_SHA` = `2fc5a9188e` (branch `engine/unpreventable-damage-riders`).
- Main to merge: **`91639fdf02`**, the SHA pinned in `task.md` and the charter. Planning-time note:
  `upstream/main` has since moved to `da2af2bca5`, two commits past the pin (Gingerbrute token
  presets #10019 and an AI test #11018). Neither touches a P1 file. Merge the **pinned SHA**
  (`git merge --no-ff 91639fdf02`), not the moving ref. Taking the newer main is run-level PR
  synchronization, not P1.
- Merge base `c57fd36003`. Conflicted tree (default style) `e8dcf6fa52b2…`. The same merge in diff3
  style (`git -c merge.conflictStyle=diff3 merge-tree --write-tree 2fc5a9188e 91639fdf02`) is tree
  `8a7e1065f10c…`, which is what this plan's hunk notes were read from.
- Probe status: the planning worktree is read-only and no build was allowed, so **no assertion in this
  plan was probed by running code**. Every runtime claim is either read from source (named by
  function) or listed in §16 (Unprobed) with the test that settles it.
- Q1 is resolved as outcome A and affects only P3. **Q2 and Q4 are answered** (accepted, each with a
  condition, now `addenda/phase-1`). This plan carries both conditions (§2.1, §2.4, §10 rows M-Q2,
  M-Q4a–d). The charter's seam-1 phrase "none at 0, CR 120.8" is **superseded by the Q4 addendum**: a
  prevention rider's amount is the amount actually prevented, **including 0** (CR 615.5). Had the
  maintainer rejected Q2 or Q4, P1's seam-1 or seam-4 decision would have broken and a re-charter
  would come before this plan's dispatch. He accepted both, so nothing changes.

---

## 0. Premise (Step 0, carried)

The charter's Step 0 record stands (MTGJSON `5.3.0+20261021`). Every card this plan's tests name was
re-read from `/mnt/data/dev/phase/data/mtgjson/AtomicCards.json` at that version, and its Oracle text
matches the constant in the test file it already sits in. The cards new to this plan:

- Fateful Showdown: "Fateful Showdown deals damage to any target equal to the number of cards in your
  hand. Discard all the cards in your hand, then draw that many cards."
- The Last Agni Kai: "Target creature you control fights target creature an opponent controls. If the
  creature the opponent controls is dealt excess damage this way, add that much {R}. / Until end of
  turn, you don't lose unspent red mana as steps and phases end."
- Skyshroud Ambush (fallback only): "Target creature you control fights target creature you don't
  control. When the creature you control wins the fight, draw a card."
- Torbran, Thane of Red Fell: "If a red source you control would deal damage to an opponent or a
  permanent an opponent controls, it deals that much damage plus 2 instead."
- Furnace of Rath: "If a source would deal damage to a permanent or player, it deals double that
  damage to that permanent or player instead."
- Test of Faith: "Prevent the next 3 damage that would be dealt to target creature this turn. For each
  1 damage prevented this way, put a +1/+1 counter on that creature."
- Hardened Scales: "If one or more +1/+1 counters would be put on a creature you control, that many
  plus one +1/+1 counters are put on it instead."
- Doubling Season (second line): "If an effect would put one or more counters on a permanent you
  control, it puts twice that many of those counters on that permanent instead."

Every other card in this plan (Rakdos Charm, Allies at Last, Laboratory Maniac, Nefarious Lich,
Pariah, Mending Hands, Awe Strike, Ironscale Hydra, Phytohydra, Force Bubble, Polukranos, Unchained,
Battletide Alchemist, Gratuitous Violence, Swans of Bryn Argoll, Abundance, Dralnu, Lich Lord,
Outfitted Jouster, Shockmaw Dragon, Delaying Shield, Farideh's Fireball, Pit Fight, Nomads en-Kor,
Undergrowth Champion, Panther Habit, Exquisite Blood, Flame Rift) is quoted verbatim in the test
constants this plan cites. Each was re-checked against MTGJSON.

CR numbers used in this plan, each grep-verified in `/mnt/data/dev/phase/docs/MagicCompRules.txt`:
101.4, 104.1, 104.2b, 104.3e, 104.3f, 104.4a, 120.4b, 120.4c, 120.8, 122.1c, 510.2, 510.4, 603.12,
608.2c, 608.2d, 608.2f, 614.1a, 614.5, 614.6, 614.9, 615.5, 615.7, 615.12, 615.13, 616.1, 616.1e,
616.1f, 616.1g, 616.2, 700.1, 701.14a, 701.21a, 702.15b, 702.16e, 704.3, 706.4, 800.4a, 800.4e,
810.8a.

## 1. Pattern Coverage (Step 0.5: charter's attribution, cited)

The charter's class record is not re-run here. Classes A (12 damage substitutions, hash
`3f5d7d09…`), B (39 prevention-with-rider cards, hash `a49c1dc6…`) and C (48 CR-defined combat choice
cards). P1's attribution: **B** (per-leg riders and the noncombat batch), **C** (per-leg asks), and the
substitution rows of **A** that P1 keeps green. The rulings P1 drives are S7, S8, S9, S11, P1, P2, P5,
P7 and P8, quoted verbatim in `class/notes.txt` and plan-r1 §1, with the Verification Matrix row for
each in §10.

One measurement taken for this plan (Step 3.5, read-only): no class-B card's prevention is itself
optional. The command was:

```
jq -r '.cards[]' class/prevention_rider_class.json | while read c; do jq -r --arg c "$c" '.data[$c][0].text' AtomicCards.json | grep -qi "you may prevent\|may have .* prevent" && echo "$c"; done
```

It returns nothing. This decides the seam-1 amount at the optional-accept install site (§2.1).

Pattern-coverage stop (phase-plan mode): assessed against the charter's class attribution (A ∪ B ∪
C). P1 is an infrastructure merge and covers cards through the class, not through a single card.

## 2. The P1 decisions, as carried out

### 2.1 Seam 1: one installer, one amount authority, one reader (charter decision 3, Q4 addendum)

- **Field.** `PostReplacementDrain::event_amount: Option<u32>`: one declaration with main's
  `#[serde(default)]`. The doc is rewritten to the widened meaning (exact text in Step A6).
- **Installer.** Main's `ReplacedEventContext { source, target, amount }` is the only path that fills
  a drain's `event_amount`, through the branch's `mandatory_post_drain` (mandatory arms) and through
  `continue_replacement_impl`'s optional-branch drain.
- **Amount authority (new private fn, `replacement.rs`):**
  `fn replaced_event_amount(state: &GameState, rid: ReplacementId, before: Option<u32>, outcome: ReplacementOutcome<'_>) -> Option<u32>`.
  - `before` is the replaced event's own quantity, captured before the applier consumes the event.
    It comes from the new private helper `fn event_quantity(event: &ProposedEvent) -> Option<u32>`,
    which returns `Some(amount)` for `Damage`, `LifeGain` and `LifeLoss` and `None` otherwise.
  - `outcome` is a new private enum: `ReplacementOutcome<'a> { Replaced, Modified(&'a ProposedEvent), AcceptedBeforeApplying }`.
  - The definition is looked up with the existing `stored_replacement_definition(state, rid)`, the
    same lookup `apply_single_replacement` uses, which includes pending shields under
    `ObjectId(0)`. The body dispatches on `def.event`:
    - `DamageDone | DealtDamage`, substitution class (new helper `damage_replacement_substitutes`,
      below): `Some(before)` for `Replaced` and `AcceptedBeforeApplying`. `Modified` gives `None`,
      because a substitution that returns `Modified` did not apply.
    - `DamageDone | DealtDamage`, prevention shape (`damage_shape_prevents(def) == Some(true)`,
      below). **CR 615.5 + Q4 addendum:** `Replaced` gives `Some(before)`, and
      `Modified(after)` gives `Some(before.saturating_sub(event_quantity(after).unwrap_or(before)))`,
      **including `Some(0)`**. `AcceptedBeforeApplying` gives `None`: nothing has been prevented
      yet, and no class-B prevention is optional (§1).
    - `GainLife`: `Some(before)` when `outcome` is `Replaced` and
      `gain_life_execute_substitutes_event_type(state, rid)`.
    - `LoseLife`: `Some(before)` when `outcome` is `Replaced` and
      `lose_life_execute_substitutes_event_type(state, rid)`.
    - Any other event gives `None`.
  - Doc: `/// CR 614.6 + CR 615.5: the amount of the event replacement `rid` replaced or prevented —
    a substitution's "that many", or the damage a prevention actually prevented ("the amount of
    damage that was prevented", zero included) — latched once, as the replacement applies, onto the
    drain its action runs from.`
- **Two-representation helper** (charter decision 3: "one helper whose body names both current
  representations, and P3 collapses that helper"):
  `fn damage_replacement_substitutes(state: &GameState, rid: ReplacementId) -> bool { matches!(shield_kind_for_rid(state, rid), Some(ShieldKind::Substitution)) || damage_execute_substitutes_event_type(state, rid) }`.
  Its doc names P3 as its collapse point. Its **only** caller is `replaced_event_amount`.
- **Prevention-shape authority.** The shape half of `is_damage_prevention_replacement` is extracted,
  exactly equivalent, into `fn damage_shape_prevents(repl: &ReplacementDefinition) -> Option<bool>`:
  - `damage_modification = Some(PreventionMinus { .. })` → `Some(true)`;
  - any other `Some(_)` → `Some(false)`;
  - `None` with `ShieldKind::Prevention { amount }` → `Some(!matches!(prevention_shield_route_for_def(repl, amount), PreventionShieldRoute::Redirect(_)))`;
  - `ShieldKind::PreventionOneShot` → `Some(true)`;
  - `ShieldKind::Substitution` → `Some(false)`;
  - `ShieldKind::None | Regeneration | DamageReplacementOneShot | Redirection { .. }` → `None`.

  The match is exhaustive, with no wildcard. `is_damage_prevention_replacement` then becomes: event
  gate, definition lookup, `if let Some(verdict) = damage_shape_prevents(repl) { return verdict; }`,
  and the legacy description test unchanged. The CR 615.12 gate and the amount authority therefore
  classify a prevention from one function. No new consumer of `ShieldKind::Substitution` is added:
  the arm moves.
- **New private predicate** `lose_life_execute_substitutes_event_type(state, rid)`. It mirrors
  `gain_life_execute_substitutes_event_type` (the execute's effect is neither `LoseLife` nor
  `Unimplemented`). `lose_life_applier`'s inline `!matches!(&*execute.effect, Effect::LoseLife {..} | Effect::Unimplemented {..})`
  is replaced by a call to it, so the applier and the authority read one predicate.
- **Appliers keep setting `last_effect_count` directly.** The six `stamp_replacement_amount(state, x)`
  calls become `state.last_effect_count = Some(u32_to_i32_saturating(x));` in place, under their
  existing guards. This includes the branch's substitution arm at the `ShieldKind::Substitution`
  check in `damage_done_applier`.
- **Reader.** The branch's dispatch-bound scope:
  - `engine_replacement::dispatch_pending_post_replacement_effect` binds `drain.event_amount` through
    `quantity::with_replacement_action_scope`;
  - `PendingContinuation::new` captures `quantity::current_replacement_action()`, and
    `effects::drain_pending_continuation` re-enters it through `resume_replacement_action`;
  - `quantity::event_context_amount` reads only `replacement_action_amount`.
- **Deleted:**
  - `REPLACEMENT_AMOUNT_STAMP` and `stamp_replacement_amount`;
  - the `apply_single_replacement` save/restore wrapper: `apply_single_replacement_inner` is renamed
    `apply_single_replacement`, and its callers already use that name;
  - main's `substituted_damage_amount` local;
  - `GameState::post_replacement_event_amount()`;
  - main's `.or_else(|| state.post_replacement_event_amount()…)` cascade tier.
- **Q4 condition, scope lifetime.** The scope restores the outer scope after nested work, on unwind
  too: `with_nested_amount_scope` holds a `ResumedReplacementAction` drop guard. It never leaks into
  the enclosing spell's tail. The spell's tail is a separate `PendingContinuation` built by the
  spell's chain driver outside the rider's dispatch, so it captures the enclosing scope, and the
  branch's merge guards in `append_to_pending_continuation` /
  `prepend_to_pending_continuation_with_producer` refuse to merge across owners. Tests M-Q4c and
  M-Q4d (§10).

### 2.2 Mechanical integration (charter decision 2)

- **`LazyBoxedVec`:** one type. `#[derive(Debug, Clone, PartialEq, Eq)]`, main's methods (`iter`,
  `iter_mut`, `push`, `remove`, `swap_remove`, `clear`, `take`) plus the branch's `is_empty`,
  `as_slice`, `From<Vec<T>>`, `Serialize` and `Deserialize`.
- **`event_amount`:** one field; one `event_amount: None` per struct literal.
- **Size ceiling:** main's 16,384 assert. The size doc keeps the branch's boxed-`damage_batch_ledger`
  paragraph, because that field survives.
- **Loop ring:** #10810's removal of the loop-ring life-move call is kept.
- **`LossReason`:**
  - `lose_by_effect`'s immediate path uses `LossReason::EffectStatedLoss`.
  - `win_by_effect`'s immediate path uses `LossReason::OpponentWon`.
  - `settle_event_outcomes` builds `BTreeMap<PlayerId, BTreeSet<LossReason>>`: `EffectStatedLoss` for
    each effect loser and each CR 810.8a shared-life teammate added for it. **For each winner who is
    not an effect loser, insert `OpponentWon` for every living opponent, including opponents already
    carrying `EffectStatedLoss`; union both reasons in the same set.** It then eliminates through
    `eliminate_players(.., DepartureTriggers::ScannedByCaller, ..)` plus the look-back collection,
    as `eliminate_players_simultaneously` does.

  The shared tail is extracted (Step A5) so the two callers can't drift. This applies CR 104.2b,
  CR 104.3e, CR 104.3f, CR 104.4a and CR 810.8a.
- **Census baseline:** regenerated (Step A9).

### 2.3 Seam 3, legs (charter decision 4)

Main's resumable `CombatDamageReplacementPass` is the only combat replacement authority. The branch's
per-batch auto-settle of order parks (`settle_combat_order_parks`) is deleted, together with the
branch's `replace_combat_damage_batch` body and its `(survivors, tally)` signature. P1 adds or
changes four things.

1. **The ledger replaces main's one-shot slot.** On every entry, `replace_combat_damage_batch`
   installs
   `DamageBatchLedger { applications: take(pass.one_shot_applications), settled_guards: read_damage_guards(state), paused_rider: None, held_riders: None, split_portions: vec![] }`.
   It moves `applications` back into the pass on exit. In P1, guards are read once per pass entry.
   Carrying them across a park is DEFERRED(phase 2). `drain_combat_damage_continuation` suspends the
   ledger (take/restore) around its drain, where main suspended `combat_one_shot_applications`
   (CR 700.1).
2. **CR 104.1, established: main's pass does not carry it.** No `game_end` read appears in main's
   `replace_combat_damage_batch`, `settle_combat_damage_replacement` or
   `drain_combat_damage_continuation`. P1 adds three checks:
   - at the loop top, after filing the step, a game that has ended settles every leg that is not
     `Settled` as `Settled(None)` and breaks `Complete`;
   - `drain_combat_damage_continuation` returns `Settled(survivor)` when the drain ended the game,
     before it looks at the pause;
   - `run_combat_damage_replacements`' `Complete` arm returns
     `CombatDamageBatch::Complete(record.batch_events)` before Phase C when `state.game_end.is_some()`.

   The branch's guards in `finish_combat_damage_sub_step`, `drain_combat_lifelink` and
   `fire_combat_prevention_riders` are already in the auto-merged tree and stay.
3. **CR 800.4a + CR 800.4e, established: main's pass carries it only for parked legs**
   (`reconcile_parked_combat_leg`). P1 adds it for unstarted legs: after reconciliation and before
   ranking, every `Unstarted(event)` with `combat_damage_recipient_has_left(state, event)` becomes
   `Settled(None)`, so it is never proposed.
4. **Kept under the existing `damage_replacement_choice_unasked_in_combat` fence.** The aggregate
   Phase D riders (`fire_combat_prevention_riders` → `drain_combat_damage_rider`) and the substitution
   arm's drain of an earlier rider inside a leg (`drain_damage_rider`'s combat routing →
   `drain_combat_damage_rider`) keep the branch's in-batch settlement: `settle_combat_rider_pauses`,
   `CombatRiderPause`, `answer_combat_rider_park` and `abandon_combat_rider_pause` stay. Their
   deletion is DEFERRED(phase 2).

### 2.4 Seam 4: `DamageTail` for every damage instruction (charter decision 6, Q2 addendum)

- `deal_damage::multi_event_damage_resolver` lists main's five instructions plus:
  - `Effect::Fight { .. }` → `fight::resolve_fight_damage` (CR 701.14a);
  - `Effect::ResumeDamageBatch { .. }` → `resolve_resume_damage_batch`;
  - `Effect::ApplyPostReplacementDamage { .. }` →
    `|s, a, e| resolve_post_replacement(s, a, e).map(|()| DamageTail::Caller)`.
- `resolve_legacy_effect` lists all eight in its `unreachable!("multi-event damage dispatch precedes
  the legacy adapter")` arm.
- `resolve_chain_body` places a tail only for `TailOwnership::Caller`. Its child-boundary condition is
  main's `|| deal_damage::is_multi_event_damage_instruction(&ability.effect)`. Deleted: the branch's
  `damage_rider_resident_before` capture, its child-boundary alternative, and its
  `damage_resolver_owns_tail` early return.
- **Q2 condition: `ResumeDamageBatch` is a carrier only.** It has no tail rule of its own.
  `damage_resolver_owns_tail`, `parked_batch_owns_tail` and `report_parked_tail` are deleted. Fresh
  execution, resume and re-pause all go through `run_damage_batch`, and so does the answered-portion
  hand-back: `return_answered_portion_to_batch` writes into the parked node, which
  `resolve_resume_damage_batch` feeds back to `run_damage_batch`. Ownership is reported as
  `DamageTail`:
  - `park_damage_batch` returns `Caller` after its child-boundary insert, and `Stashed` after
    parking the tail beneath the batch.
  - Every other early return of the runner returns `Caller`, and so does completion.
- **Per-recipient loops** (`resolve_with_splice`, `resolve_all_portions`,
  `resolve_each_player_portions`) keep the branch's rider placement
  (`park_recipients_behind_paused_rider`):
  - a rider pause → `Caller`, and the chain inserts the tail at the child boundary beneath the parked
    recipients;
  - `NeedsChoice` → the stash plus its tail → `Stashed`.

  Main's `pause_behind_continuation_prompt` arms are not taken.
- **One tail builder:** main's name `stashed_tail`, the branch's body (`cast_tail_with_parent_targets`,
  including the Searing Blaze "instead" base-clause binding). One node builder: main's
  `build_remaining_damage_node(ability, source, target, amount)` signature without
  `carry_instruction_context`. The branch's `keep_chain_identity`, applied by
  `remaining_damage_chain`, already sets context, ability index and settled guards.
- **Deleted from main:** `carry_instruction_context`, `build_post_replacement_damage_node`,
  `stash_remaining_post_replacement_damage`, `pause_behind_continuation_prompt`,
  `stash_each_source_combined_continuation`, `stash_remaining_each_source_damage`,
  `stash_remaining_each_deals_chain`, `apply_batch_survivors`,
  `apply_batch_survivors_before_replacement_pause`, and main's bodies of
  `resolve_each_target_power_damage`, `resolve_each_deals_power_damage` and
  `resolve_each_source_damage`.

### 2.5 Seam 2 untouched (charter decision 7)

`ShieldKind::Substitution` and its arms, `hold_damage_rider`'s timing, the CR 120.8 candidate gate and
the coverage fence key all stay as the branch has them. Main's `is_damage_substitution` /
`damage_execute_substitutes_event_type` / Branch 3 / `CandidateMateriality::Unconditional` all stay
as main has them. The only new reader is `damage_replacement_substitutes` (§2.1).

---

## 3. Sizing

| Unit | Behaviour | Registration surfaces | Discriminating tests |
|---|---|---|---|
| U1 Mechanical integration | merge lands; `LossReason` through effect-stated outcomes; `LazyBoxedVec` union; duplicate fields; imports; size budget; census | elimination.rs, win_lose.rs, engine.rs, game_state.rs, game_state_size.rs, baseline | M-L1, M-L2, M-L3 (§10); `game_state_size` const asserts; census gate |
| U2 Seam 1 amount | `replaced_event_amount` at every stash site; dispatch-bound reader only | replacement.rs, quantity.rs, game_state.rs, engine_replacement.rs (comment) | M-Q4a–d, V1.1–V1.4 |
| U3 Seam 3 legs | main's pass + ledger + CR 104.1 / CR 800.4a legs; per-leg asks; conversions | replacement.rs, combat_damage.rs, game_state.rs (doc), 3 test files | §11 rows (C-xx), V3.x |
| U4 Seam 4 tail | `DamageTail` for all eight instructions; carrier-only `ResumeDamageBatch` | deal_damage.rs, fight.rs, effects/mod.rs | M-Q2 rows R1–R12, M-F1 |

**Dependency edges.** U1 → U2, U3, U4 (a green tree). U2, U3 and U4 are independent of each other.
All four land in the merge commit, because each is a conflict-hunk resolution (charter §Feasibility
1–4). The test conversions (U3) and the new tests (U2, U4) follow in commit B.

**Expected scope-path count** (counting rule: test fixtures, regenerated data and protocol-version
surfaces excluded; P1-authored files only, not upstream's own changes):

1. `crates/engine/src/game/combat_damage.rs`
2. `crates/engine/src/game/replacement.rs`
3. `crates/engine/src/game/effects/deal_damage.rs`
4. `crates/engine/src/game/effects/fight.rs`
5. `crates/engine/src/game/effects/mod.rs`
6. `crates/engine/src/game/effects/win_lose.rs`
7. `crates/engine/src/game/elimination.rs`
8. `crates/engine/src/game/engine.rs`
9. `crates/engine/src/game/quantity.rs`
10. `crates/engine/src/types/game_state.rs`
11. `crates/engine/src/types/game_state_size.rs`
12. `crates/engine/tests/integration/damage_substitution_replacements.rs`
13. `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs`
14. `crates/engine/tests/integration/ironscale_hydra_combat_prevention.rs`

**Count: 14** (T1: 4 units; T2: 14 ≥ 13). The conjunction fires. The answer is the charter's recorded
P1 feasibility exit, not a second-level split.

- **Excluded as regenerated data:** `scripts/post-replacement-continuation-baseline.txt`.
- **Standing compiler-forced and comment-only class:** `crates/engine/src/types/resolution.rs`,
  `crates/engine/src/game/targeting.rs`, `crates/engine/tests/integration/swans_prevention_followup.rs`,
  `crates/engine/tests/integration/spell_resolution_final_part_after_paused_instruction.rs`, and
  `crates/engine/src/game/engine_replacement.rs` (one comment).
- **No protocol record changes** in P1 (§13).

## 4. Building Blocks

- **From main:**
  - `replacement::{CombatDamageReplacementPass, replace_combat_damage_batch, settle_combat_damage_replacement, drain_combat_damage_continuation, reconcile_parked_combat_leg, repropose_combat_damage_leg, first_chooser_of_unstarted_combat_leg, combat_damage_recipient_has_left, lift_departed_combat_leg_choice, settled_survivors, ReplacedEventContext, is_damage_substitution, damage_execute_substitutes_event_type}`;
  - `combat_damage::{order_batch_by_affected_seat, run_combat_damage_replacements, resume_combat_damage_replacements, parked_combat_damage_awaits_replacement_choice}`;
  - `deal_damage::{DamageTail, is_multi_event_damage_instruction, multi_event_damage_resolver, resolve_instruction, stashed_tail (name)}`;
  - `effects::{TailOwnership, retire_empty_replacement_frames}`;
  - `elimination::{eliminate_players, DepartureTriggers::ScannedByCaller, LookedBackDeparture::collect}`.
- **From the branch:**
  - `deal_damage::{run_damage_batch, run_damage_batch_phases, park_damage_batch, run_object_damage_batch, resolve_resume_damage_batch, park_recipients_behind_paused_rider, park_portions_behind_paused_rider, remaining_damage_head, remaining_damage_chain, keep_chain_identity, settle_joined_damage_run, hold_riders_for_awaited_portion, return_answered_portion_to_batch, defer_choice_behind_earlier_players}`;
  - `replacement::{with_damage_batch, read_damage_guards, settled_damage_guards, damage_batch_has_paused_rider, drain_damage_rider, drain_combat_damage_rider, hold_damage_rider, damage_rider_placement, mandatory_post_drain, stash_post_replacement_continuation(state, drain, behind), damage_batch_holds_riders, stored_replacement_definition}`;
  - `quantity::{with_replacement_action_scope, current_replacement_action, resume_replacement_action, replacement_action_amount}`;
  - `elimination::{lose_by_effect, win_by_effect, settle_event_outcomes, event_outcomes}`.
- **New private helpers, each with its closest authority:**
  - `replaced_event_amount` extends main's inline `substituted_damage_amount` (same site, same
    inputs) to the prevention and life-substitution kinds, instead of the thread-local stamp.
  - `event_quantity` is a scalar capture. Main's comment forbids cloning damage events there.
  - `damage_replacement_substitutes` is the charter-mandated two-representation helper.
  - `damage_shape_prevents` is extracted from `is_damage_prevention_replacement`, not a second
    classifier.
  - `lose_life_execute_substitutes_event_type` is the sibling the doc of
    `gain_life_execute_substitutes_event_type` names ("siblings (life-loss substitution, …) can
    extend through the same primitive").
  - `elimination::eliminate_with_reasons`: Step A5.

## 5. Logic Placement

| Behaviour | Owner | Delegating callers |
|---|---|---|
| amount a replacement's action owns | `replacement::replaced_event_amount` → `ReplacedEventContext.amount` → `PostReplacementDrain::event_amount` | `apply_single_replacement` (Modified arm, Prevented arm), `continue_replacement_impl` (optional accept) |
| "is this a prevention" (shape) | `replacement::damage_shape_prevents` | `is_damage_prevention_replacement` (CR 615.12 gate), `replaced_event_amount` |
| reading "that many" | `quantity::replacement_action_amount` (scope bound in `dispatch_pending_post_replacement_effect`) | `quantity::event_context_amount` only |
| combat replacement pass, ledger, CR 104.1 / CR 800.4a legs | `replacement::replace_combat_damage_batch` | `combat_damage::run_combat_damage_replacements` (fresh and resumed) |
| per-leg continuation drain (asked) | `replacement::drain_combat_damage_continuation` | `settle_combat_damage_replacement`, pass resume |
| in-leg earlier rider and Phase D drains (auto-settled, fenced) | `replacement::drain_combat_damage_rider` | `drain_damage_rider` (combat routing), `combat_damage::fire_combat_prevention_riders` |
| tail ownership | `deal_damage::multi_event_damage_resolver` → `DamageTail`; mapped in `effects::resolve_effect_node` | `resolve_chain_body` (reads `TailOwnership` only) |
| effect-stated outcomes | `elimination::settle_event_outcomes` / `lose_by_effect` / `win_by_effect` → `eliminate_with_reasons` | `win_lose::resolve_lose` / `resolve_win`, `deal_damage::settle_event_losses` |

**Consumers whose cardinality or binding changes:**
- Fight, `ResumeDamageBatch` and `ApplyPostReplacementDamage` move from the legacy adapter to the
  multi-event arm (same `LegacyEffectBoundary`).
- The per-leg rider in combat is now asked instead of settled.
- `event_amount` is `Some(0)` for a prevention that prevented nothing (Q4 addendum).
- An accepted optional damage branch's drain now carries `event_source` / `event_target` (main's
  reading).

## 6. Rust Idioms

- `DamageTail` (main's two-variant enum) replaces frame-tag inference.
- `ReplacementOutcome` is a typed enum, not `Option<Option<&ProposedEvent>>`.
- `damage_shape_prevents` returns `Option<bool>` (verdict or no verdict) with an exhaustive
  `ShieldKind` match and no wildcard.
- `replaced_event_amount` is a pure function; the thread-local is gone.
- `settle_event_outcomes` builds `BTreeMap<PlayerId, BTreeSet<LossReason>>` with `entry().or_default()`.
- `LazyBoxedVec` is one generic type with derive bounds per impl.
- Every new or changed match stays exhaustive.

## 7. Nom Compliance

No file under `crates/phase-parser/src/parser/` changes in P1. N/A. No Oracle text is newly accepted.

## 8. Extension vs Creation

- **Extends:**
  - main's combat pass: the ledger, two CR 104.1 checks and the unstarted-leg CR 800.4a check;
  - main's `DamageTail` table: three entries;
  - main's `ReplacedEventContext`: more amount kinds;
  - `is_damage_prevention_replacement`: its shape half is extracted.
- **Creates:** five private functions (§4), one private enum (`ReplacementOutcome`), and one private
  elimination helper. No public type, no `WaitingFor`, no `GameAction`, no protocol-visible change.

## 9. Analogous Trace (Step 2)

- **Main's tail authority:** `effects::resolve_effect_node` (multi-event arm) →
  `deal_damage::resolve_instruction` → `multi_event_damage_resolver` → `resolve_damage` →
  `stash_remaining_damage_chain` → `remaining_damage_head` → `stashed_tail` → `DamageTail::Stashed` →
  `TailOwnership::Transferred` → `resolve_chain_body`'s `match tail_ownership` early return. On
  `Caller` with child work, the same function's `insert_ability_continuation_parent_at_child_boundary`
  arm runs.
- **Branch's batch carrier:** `run_damage_batch` → `replacement::replace_event` /
  `drain_damage_rider` / `hold_damage_rider` → `park_damage_batch` (`Effect::ResumeDamageBatch`,
  `SpellContext.parked_damage_batch`) → answer → `engine_replacement::handle_replacement_choice_inner`
  (`hold_riders_for_awaited_portion`, `return_answered_portion_to_batch`) →
  `effects::drain_pending_continuation` → `resolve_resume_damage_batch` → `run_damage_batch`.
- **Main's combat park:** `combat_damage::apply_combat_damage` → `order_batch_by_affected_seat` →
  `run_combat_damage_replacements` → `replace_combat_damage_batch` (Paused →
  `state.pending_combat_lifelink` with `replacement`) → answer →
  `engine_replacement::handle_replacement_choice` (top routing
  `parked_combat_damage_awaits_replacement_choice`) → `resume_pending_combat_lifelink(.., Some(index))`
  → `resume_combat_damage_replacements` → pass resume → `apply_combat_damage_survivors` →
  `drain_combat_lifelink` → `fire_combat_prevention_riders`.
- **Seam 1 reader:** `engine_replacement::dispatch_pending_post_replacement_effect` (reads
  `drain.event_amount`) → `quantity::with_replacement_action_scope` → `PendingContinuation::new`
  (`current_replacement_action()`) → `effects::drain_pending_continuation`
  (`resume_replacement_action`) → `quantity::event_context_amount`.

Files followed: `effects/mod.rs`, `effects/deal_damage.rs`, `effects/fight.rs`, `replacement.rs`,
`engine_replacement.rs`, `combat_damage.rs`, `quantity.rs`, `elimination.rs`, `types/game_state.rs`.

## 10. Verification Matrix (with Identity / Provenance)

Production entry points follow `/card-test`:
- spells through `CastSpell` and the file's own `cast` / `settle` drivers;
- combat through `DeclareAttackers` / `DeclareBlockers` / `PassPriority`, using the
  `combat_events` / `fight` / `combat_at` drivers;
- answers through `GameAction::ChooseReplacement` / `SelectCards`.

Completion boundary, unless a row asserts a game end: `WaitingFor::Priority`, empty stack and empty
`resolution_stack`, `pending_replacement.is_none()`, `pending_combat_lifelink.is_none()`. In combat,
the phase is `EndCombat` / `PostCombatMain` / the CR 510.3 window.

**Row M-Q4a.** A prevention rider reads the amount prevented inside a resolving trigger (CR 615.5,
CR 603.12).
- Seam: `replaced_event_amount` (prevention arm), with the scope as reader.
- Test: `damage_substitution_replacements::a_replacements_action_reads_its_own_amount_inside_a_resolving_trigger`
  (unchanged).
- Revert-failing assertion: Test of Faith's Bear and Phytohydra have `(1, 1)` counters, not 4.
- Reach-guard: P1 life 16 (Shockmaw Dragon dealt 4).
- Hostile cases: Dralnu sacrifices 1 of 2 lands; the Scales + Season order gives `(4, 4)`.
- Provenance: amount bound at apply time, a snapshot. Storage: drain → scope. Consumer:
  `replacement_action_amount`.

**Row M-Q4b.** A substitution's amount outranks the d20 (CR 706.4, CR 614.6).
- Seam: same as M-Q4a.
- Tests: `farideh_fireball_die_branch_selection::{delaying_shield_counts_the_rows_damage_not_the_d20_face, force_bubble_counts_the_rows_damage_not_the_d20_face, delaying_shield_counts_the_rows_damage_across_an_order_choice, nefarious_lich_exiles_the_rows_damage_across_its_prompt_not_the_d20_face, a_replacement_actions_step_after_its_prompt_reads_the_rows_damage}`
  (unchanged).
- Revert-failing assertion: Delaying Shield has 2 delay counters for every face (5 under Doubling
  Season + Pir).
- Reach-guard: the face recorded (`emitted_face`).
- Hostile case: the order-choice row.
- Provenance: as M-Q4a.

**Row M-Q4c (new).** The scope restores the outer scope after nested work and on unwind (Q4
condition).
- Seam: `quantity::with_nested_amount_scope`, `resume_replacement_action`.
- Test: new quantity unit test
  `replacement_action_scope_restores_the_outer_scope_after_nested_work`.
- Revert-failing assertions:
  - inside `Some(2)`, a nested `Some(5)` reads 5, and the read after it returns 2;
  - a `resume_replacement_action(Some(7))` guard reads 7 until dropped, then 2;
  - after a `catch_unwind` of a panicking `with_replacement_action_scope(Some(9), ..)`, the
    `Enclosing` reading (the match count 3) is back.
- Reach-guard: the outer read is 2, the inner 5.
- Hostile case: a reflexive scope inside the action reads the die result (17), as in the existing row.
- Provenance: none (a unit seam).

**Row M-Q4d (new).** The amount never leaks into the enclosing spell's tail (Q4 condition, CR 608.2c,
CR 615.5).
- Seam: `resolve_chain_body` child-boundary insert, `PendingContinuation::new` capture, merge guards.
- Test: new `damage_substitution_replacements::a_riders_amount_does_not_reach_the_spells_tail`,
  spelled out in Step B3.
- Revert-failing assertion: Fateful Showdown with 5 cards in hand at P1's 3/3 under Test of Faith. P0
  draws exactly **5** cards (a leak reads Test of Faith's 3). It holds both without and with P1's
  Hardened Scales + Doubling Season, where the rider pauses.
- Reach-guard: Test of Faith's counters were placed (`CounterAdded` on the 3/3), and the 3/3 has 2
  damage marked.
- Hostile case: the paused-rider variant, where the tail waits behind the rider's prompt.
- Provenance: the rider's continuation carries `replacement_action` Some(3); the tail's carries
  `None`.

**Row M-Q4e (new).** A prevention that prevented 0 latches `Some(0)` (Q4 addendum, CR 615.5).
- Seam: `replaced_event_amount`.
- Test: new replacement unit test
  `replaced_event_amount_latches_each_owning_kind_and_zero_prevented`.
- Revert-failing assertions:
  - Prevention `Next(3)` on 5 damage: `Modified(2)` → `Some(3)`.
  - `All` on 5: `Replaced` → `Some(5)`.
  - `PreventionMinus` that left 3 of 3: `Some(0)`.
  - Phytohydra-shaped substitution on 4: `Replaced` → `Some(4)`; `AcceptedBeforeApplying` → `Some(4)`.
  - Prevention `AcceptedBeforeApplying` → `None`.
  - Furnace-of-Rath-shaped `Double` → `None`.
  - Pariah-shaped redirect `Prevention` shield → `None`.
  - Lich-shaped gain-life substitution `Replaced` → `Some(n)`; loss-life twin → `Some(n)`.
  - A `LoseLife` execute → `None`.
- Reach-guard: each definition is found by `stored_replacement_definition`.
- Hostile cases: `Unimplemented` execute; `ObjectId(0)` pending shield.
- Provenance: none.

**Row M-L1.** Effect-stated losses carry `LossReason` (CR 104.3e, CR 104.2b, CR 104.3f, CR 104.4a,
CR 810.8a).
- Seam: `elimination::settle_event_outcomes`, `eliminate_with_reasons`.
- Tests (updated):
  - `a_player_who_wins_and_loses_in_one_batch_loses`: also `PlayerLost { P0, {EffectStatedLoss} }`;
  - `a_two_headed_giant_team_that_wins_and_loses_in_one_batch_loses`: P0 and P1 each
    `{EffectStatedLoss}`;
  - `a_lifelink_win_settles_with_the_batchs_damage`: P1 `{OpponentWon}` in both loops.
- Revert-failing assertion: exact reason sets.
- Reach-guard: each test's existing reach-guards.
- Hostile case: two-Lich draw (`two_lich_losses_in_one_batch_are_a_draw`: both `{EffectStatedLoss}`).
- Provenance: reasons attached at settle time.

**Row M-L2 (new).** Union of reasons (charter decision 2 sentence).
- Seam: `settle_event_outcomes`.
- Test: new elimination unit test
  `settle_event_outcomes_unions_an_effect_loss_with_an_opponents_win`.
- Revert-failing assertion: two players, outcomes `losers [P1]`, `winners [P0]` →
  `PlayerLost { P1, {EffectStatedLoss, OpponentWon} }`, P0 not eliminated, `GameOver { Some(P0) }`.
- Reach-guard: P1 eliminated.
- Hostile case: P0 in both sets → P0 `{EffectStatedLoss}` only, no `OpponentWon` for P1.
- Provenance: none.

**Row M-L3.** The immediate path's reason (CR 104.3e).
- Seam: `lose_by_effect` immediate path.
- Test: `unpreventable::nothing_happens_after_a_noncombat_portion_ends_the_game` (updated: also
  `PlayerLost { P0, {EffectStatedLoss} }`).
- Revert-failing assertion: the exact reason set.
- Reach-guard: P0 eliminated.
- Provenance: none.

Rows M-Q2/R1–R12, the claim-to-test map, are in §11.1. Rows C-xx, the combat conversions, are in
§11.2. Main's #9670 suites run unchanged; their derivations are in §12.

**Row M-F1 (new).** A fight's tail is owned once (CR 701.14a, CR 608.2c, CR 616.1).
- Seam: the table entry `Fight` → `park_damage_batch`'s `Stashed`.
- Test: new `damage_substitution_replacements::a_paused_fights_tail_runs_once` (Step B4).
- Revert-failing assertions:
  - after P1's answer, P0's pool holds exactly **12** {R} for Torbran first, or **10** for Furnace
    first;
  - 0 {R} before the answer.
- Reach-guard: the prompt is P1's, with candidates {Torbran, Furnace}.
- Hostile cases: the other order. Leaving `Fight` out of the table makes the driver place the tail as
  well, and the pool doubles.
- Provenance: the batch parks its tail beneath itself → `Stashed`.

**Coverage status impact.** None in P1. The `damage_replacement_choice_unasked_in_combat` fence keeps
its predicates: honest for Phase D and in-leg riders, conservative for per-leg ones. P2 removes it.
`damage_substitution_choice_incomplete` keeps its tag key until P3. No parser change, so no parse
flips.

## 11. Tests

### 11.1 Claim-to-test map for the retained batch (phase-inputs/phase-1, Q2 addendum)

Every row is an existing branch test, unchanged in P1 unless noted. Each "derived reading" is worked
from the card and the CR. The test's own value was not copied.

**R1.** `damage_substitution_replacements::a_lifelink_win_settles_with_the_batchs_damage` (ordered
loop).
- Seam: the carrier's `ParkedDamageBatch.outcomes` across a delivery park. The win is recorded into
  the parked batch through `elimination::event_outcomes`. This is the effect-stated behaviour that
  justifies the carrier.
- Derived reading: Allies at Last, "Up to two target creatures you control each deal damage equal to
  their power to target creature an opponent controls."
  - Vampire of the Dire Moon (1, lifelink) and Grizzly Bears (2) hit the Wall as one event
    (CR 120.4b, CR 120.4c).
  - The gain (CR 702.15b) meets Rhox Faithmender, Leyline of Hope and Nefarious Lich ("If you would
    gain life, draw that many cards instead"). P0 orders and picks the Lich (CR 616.1).
  - The draw from an empty library becomes Laboratory Maniac's win ("you win the game instead").
  - The win is part of the batch's event and settles when it does (CR 104.4a, CR 104.1), so the
    Bears' 2 is dealt before the game ends, once.
- Discriminating assertion: `events[..over]` holds `DamageDealt { second, 2 }`; one `GameOver`; no
  `DamageDealt` after it.
- Reach-guard: prompts include `("order", P0)`; `ReplacementApplied` from the Lich (GainLife) and the
  Maniac (Draw).

**R2.** `…::two_lich_losses_in_one_batch_are_a_draw` (case P1 active, P0 with 3 creatures and 2
graveyard cards).
- Seam: outcomes recorded across an exile park, the held substitution action's own choice.
- Derived reading: Rakdos Charm, "Each creature deals 1 damage to its controller."
  - P1's Lich, with an empty graveyard, can't exile → P1's loss is recorded.
  - P0's first exile asks which card (CR 608.2d). At the third portion P0 can't exile → P0 loses.
  - Both lose simultaneously → a draw (CR 104.4a).
- Discriminating assertion: `game_overs == [None]`; both eliminated.
- Reach-guard: prompt `("exile", P0)`.

**R3.** `…::a_two_headed_giant_team_that_wins_and_loses_in_one_batch_loses`.
- Seam: `settle_event_outcomes`, with the CR 810.8a fold before the CR 104.3f weighing.
- Derived reading: P1's Lich loss is the team's (CR 810.8a). P0 would win and lose, so P0 loses
  (CR 104.3f). The P2/P3 team wins.
- Discriminating assertion: P0 and P1 eliminated, P2 and P3 not.
- Reach-guard: `ReplacementApplied` (p0_lich GainLife, maniac Draw, p1_lich DamageDone);
  `WinTheGame` and `LoseTheGame` both resolved.

**R4.** `…::a_paused_batch_finishes_its_replacements_before_dealing_damage`.
- Seam: held riders behind the simultaneous event (CR 614.6, CR 615.5), deferred APNAP (CR 101.4).
- Derived reading:
  - Every portion's replacement comes before any damage (CR 120.4b).
  - P2's Force Bubble / Delaying Shield order is an original choice, asked first.
  - Delivery then asks P0's gain order (Rhox and Leyline), and the Lich's held exile (its
    substitution's action, CR 614.6) follows.
  - The instruction completes once, after the last damage (CR 608.2c).
- Discriminating assertions: `prompts == [("order",p2),("gain",P0),("exile",P1)]`;
  `dealt_before_last_replacement_answer == 0`; the one completion comes after the last `DamageDealt`.
- Reach-guard: `dealt(&events) == 1` (Rhox's 1).

**R5.** `…::deferred_damage_queue_survives_game_state_serde_round_trip`.
- Seam: deferred choices on the serialized carrier.
- Derived reading: CR 101.4 order survives a save. The restored game's events equal the uninterrupted
  game's.
- Discriminating assertion: `events == uninterrupted_events`.
- Reach-guard: `assert_damage_chooser(&runner, P1)` at the pause.

**R6.** `…::a_batch_keeps_a_shared_shield_for_the_players_choice`.
- Seam: the CR 615.7 wait (`shield_is_still_choosable`).
- Derived reading: Mending Hands, "Prevent the next 4 damage that would be dealt to any target this
  turn", on the Wall; Awe Strike on the Hill Giant.
  - The Giant's 3 meets both shields, so P1 chooses. The Baloth's 4 meets only Mending Hands and
    waits.
  - Mending Hands for the Giant: 1 left, so the Baloth deals 3 to the Wall and P0 gains 0.
  - Awe Strike for the Giant: P0 gains 3, and Mending Hands prevents the Baloth's 4, so the Wall takes
    0.
- Discriminating assertions: Wall 3 / P0 20, or Wall 0 / P0 23.
- Reach-guard: the chooser is P1 (`assert_eq!(player, P1)`).

**R7.** `unpreventable::a_capped_redirects_part_meets_its_new_recipients_protection`.
- Seam: capped-redirect split parts (CR 614.9, CR 614.5) through `run_object_damage_batch`, now
  reached through the `Fight` table entry.
- Derived reading: Pit Fight between a red Hill Giant and Nomads en-Kor, "The next 1 damage that would
  be dealt to this creature this turn is dealt to target creature you control instead", aimed at the
  pro-red Silver Knight.
  - The 1 redirected part is prevented by protection (CR 702.16e), and the Nomads take 2.
  - Under Stomp, nothing is prevented (CR 615.12), and the Knight takes 1.
- Discriminating assertions: Nomads `[2]`; Knight `[]` + prevented, or `[1]` + not prevented.
- Reach-guard: the Knight's prevented event, or its damage.

**R8.** `…::a_redirected_answer_still_waits_for_the_batch`.
- Seam: the answered-portion hand-back (`return_answered_portion_to_batch`) into the same runner.
- Derived reading: P1 picks Pariah over Furnace for each Bear's 1, so the damage goes to the Wall.
  Nothing is dealt before P1's last answer (CR 120.4b), and Rhox's gain is asked in delivery
  (CR 120.4c).
- Discriminating assertion: every P1 answer is taken with 0 dealt; P1 life 20; one completion after
  the last damage.
- Reach-guard: `!p1_orders.is_empty()`.

**R9.** `…::a_substitution_loss_comes_with_the_batchs_damage`.
- Seam: the CR 104.1 stop inside the runner (`run_damage_batch` post-rider terminal guard).
- Derived reading: the Lich's "If you can't, you lose the game" is the substitution's action, in the
  same event (CR 614.6). P0's surviving portion is dealt with it, then the game ends (CR 104.1). There
  is no completion.
- Discriminating assertions: `DamageDealt` from Rhox before `GameOver { Some(P0) }`; nothing dealt
  after; no completion.
- Reach-guard: `ReplacementApplied` from the Lich.

**R10.** `…::a_batch_continues_for_survivors_after_one_of_three_players_loses`.
- Seam: the CR 800.4a stop (`recipient_present`).
- Derived reading: P1 leaves at its loss. P1's Grey Ogre's portion has no recipient, and P2 takes 1.
- Discriminating assertions: no `DamageDealt` from the Ogre or to P1; P2 life 19; one completion.
- Reach-guard: `prompts == [("order", P0, 1)]`.

**R11.** `soul_scar_mage_damage_to_counters::{multi_source_tail_waits_for_every_counter_prompt, multi_source_life_gain_order_runs_the_tail_once}`
(main's suites, unchanged).
- Seam: `DamageTail` from the batch runner (`Stashed` / `Caller` through `park_damage_batch`).
- Derived reading: §12 (Allies at Last under Soul-Scar Mage: (2·3−1)+(2·2−1) = 8 counters, two
  prompts, the tail once).
- Discriminating assertion: one draw / tail once.
- Reach-guard: two prompts.

**R12.** `unpreventable::a_fights_capped_redirects_ask_the_recipients_shield_choice` (unchanged).
- Seam: `Fight` → `run_object_damage_batch` → `park_damage_batch` → `DamageTail`.
- Derived reading: each redirected 1 meets Test of Faith and Mending Hands, so P0 chooses (CR 616.1).
  The fight completes once, after the choices (CR 608.2c).
- Discriminating assertion: one `EffectResolved { Fight }` after the choices; nothing dealt to the
  Wall or the Nomads.
- Reach-guard: the prompts surfaced.

### 11.2 Combat test conversions (charter decision 5)

These are per-leg prompts that main's pass now asks: an order at the leg, or the leg's own
continuation (CR 616.1, CR 616.1g, CR 608.2d). Every row drives each answer the player has and asserts
the exact value per answer. None is weakened to a set.

**Test-helper changes, made first in commit B:**
- `unpreventable_damage_prevention_effects.rs`:
  - `scales_and_season` returns `(ObjectId, ObjectId)` (Scales, Season). Its existing callers ignore
    the tuple.
  - New `pub(super) fn fight_answering(runner, attackers, blocks, answer: impl FnMut(&GameState) -> GameAction) -> Combat`.
    It is `fight`'s loop, with `ReplacementChoice` and `EffectZoneChoice` delegated to `answer`, and
    it still pushes the `ReplacementChoice` candidates to `prompts`. `fight` becomes
    `fight_answering` with its current `prefer` rule.
  - `combat_at` gains a `ReplacementChoice` arm taking an `answer` closure, the same pattern.
- `damage_substitution_replacements.rs`: conversions use the file's `combat_events` (it stops at the
  first non-Priority wait) followed by `choose_source` / `choose`. Those call `answer`, which
  `settle`s.

Classification of every branch combat test:

| Test | Card(s) / Oracle | CR | Branch expected | Class | P1 expectation (derived) |
|---|---|---|---|---|---|
| C-01 `damage_substitution_replacements::combat_order_park_keeps_force_bubbles_damage` | Force Bubble (P1) + Furnace of Rath (P0); unblocked 1/1; then 3/3 after Stomp | 616.1, 510.2, 615.12 (S1–S4) | `[1,2].contains`; graveyard or 3 | per-leg | P1 asked. 1/1: Bubble first → 1 counter, Furnace first → 2. Stomp 3/3: Bubble first → 3 counters (stays), Furnace first → 6 → "four or more … sacrifice it" → Graveyard. Life 20 in every row |
| C-02 `…::combat_order_park_keeps_phytohydras_damage` | Phytohydra (P1) blocks Ogre 3/3; Furnace | 616.1 (S9) | `[3,6]` | per-leg | P1 asked: Phytohydra first → 3, Furnace first → 6; damage 0 |
| C-03 `…::combat_order_park_runs_a_prevention_rider_before_the_batch_drain` | Polukranos (6 counters) blocks Bear 2/2; Furnace | 616.1, 615.5 | `[4,2]` | per-leg | P1 asked: Furnace first → 4 prevented → 2 left; Polukranos first → 2 prevented → 4 left; damage 0; one `DamagePrevented` |
| C-04 `…::combat_order_park_keeps_a_shield_counter_replacement` | Dapper Shieldmate (2 shield) blocks 2/2; Furnace; ± Stomp | 616.1, 122.1c | 1 shield; damage 0 / 4 | per-leg if asked | If P1 is asked, answer both ways: each gives 1 shield and damage 0 (4 under Stomp). Whether it is asked is established at implementation (§16 U3); assert the measured presence |
| C-05 `…::combat_order_park_keeps_a_damage_modifiers_damage` | Torbran + Furnace (P0); red 2/2 unblocked | 616.1 (Torbran ruling) | `[6,8]` | per-leg | P1 asked: Torbran first → (2+2)×2 = 8 lost; Furnace first → 2×2+2 = 6 |
| C-06 `…::combat_order_parks_repeat_until_the_event_settles` | Torbran, Furnace, Gratuitous Violence; then Force Bubble for GV | 616.1f | `[10,12,16]`; graveyard or 2 | per-leg | P1 asked until settled. Torbran first → 16; one doubler then Torbran → 12; both doublers then Torbran → 10 (each asserted). Bubble board: Bubble first → 2 counters and life 20; Torbran or Furnace first → ≥ 4 counters → Graveyard, life 20 |
| C-07 `…::combat_order_answer_drains_in_its_own_portion` | two Phytohydras each block a 1/1; Furnace. Then Phytohydra + Polukranos | 616.1, 615.5 | `>= 1` each; `[4,2]` | per-leg | Two P1 prompts. The prompt offering `first` is answered `first` → 1, the one offering `second` is answered Furnace → 2. Board 2, run 1: Phytohydra-leg Phytohydra → 1, Polukranos-leg Furnace → 2 left. Run 2: Furnace → 2, Polukranos → 4 left. `!has_post_replacement_drain()` |
| C-08 `…::two_force_bubbles_take_all_of_a_combat_batch_on_one` | two Force Bubbles (P1); three 1/1s unblocked | 616.1, ruling S6 | `[0,3]` | per-leg (fenced gap) | Three P1 prompts, one per source: the gap `damage_substitution_choice_incomplete` names. Answering the same Bubble each time → `[0,3]`, life 20. The strict-failure comment cites S6 and the fence |
| C-09 `…::a_parked_may_gets_no_automatic_yes_in_combat` → rename `a_parked_may_is_asked_in_combat` | Battletide Alchemist (P1; X = 1); Ogre 3/3 unblocked | 616.1, 615.5 | life 20 (skipped) | per-leg | P1 asked (the "may"). Accept → 1 prevented, life 18; decline → life 17. The noncombat reach half is kept |
| C-10 `…::a_resident_rider_from_another_recipient_is_not_lost` (combat half) | Test of Faith on Phytohydra (P1), blocking Ogre 4/4 | 616.1, 615.5 | 4 counters | per-leg | P1 asked (Test of Faith vs Phytohydra, both order-material). Test of Faith first → 3 counters (rider) + 1 (substitution of the last 1) = 4. Phytohydra first → 4 counters and the shield unused. Damage 0 both ways; no resident drain |
| C-11 `unpreventable::phytohydras_counter_order_is_settled_inside_the_batch` → `…_is_asked_inside_the_batch` | Phytohydra (P1) blocks a 2/2; P1's Scales + Season | 616.1g | `[5,6]` | per-leg | P1 asked: Scales first → (2+1)×2 = 6; Season first → 2×2+1 = 5. Without Scales: 2, no prompt. The Bolt row is kept |
| C-12 `…::swans_of_bryn_argolls_draws_are_kept_under_abundance` → `…_draws_ask_abundance` | Swans (P1) blocks Bears; P0's Abundance | 616.1, 121.6 (Abundance rulings) | 2 cards, declined | per-leg | Two P0 prompts (one per draw). Decline both → 2 `CardDrawn`, hand +2. Accept both and choose nonland → 0 `CardDrawn`, hand +2 (the revealed filler spells). Without Abundance: 2 drawn, no prompt |
| C-13 `…::double_strike_keeps_the_swans_draw_in_each_damage_step` | Boros Swiftblade blocked by Swans; Abundance | 510.4, 616.1 | 2 cards, declined | per-leg | One P0 prompt per damage step. Decline → a `CardDrawn` in each step (2 total); accept + nonland → 0 `CardDrawn`, hand +2 |
| C-14 `…::a_may_park_and_a_riders_own_prompt_settle_in_one_batch` (mixed) | Swans + Abundance (per-leg); Channel Harm (Phase D) | 616.1, 608.2d | Abundance declined, Channel Harm cleared | per-leg + Phase D | Abundance's two prompts are asked and answered as in C-12, with exact per-answer draws. Channel Harm's "may" stays settled in the batch: the Giant has 2 damage, and its strict-failure note is kept and points at P2. Rename `…_asks_abundance_and_settles_channel_harm` |
| C-15 `…::nefarious_lichs_exile_choice_is_cleared_inside_the_batch` → `…_is_asked_inside_the_batch` | Nefarious Lich, 4 graveyard cards, 3/3 unblocked | 608.2d, 614.6 | nothing exiled | per-leg | P1 chooses 3 of the 4 (EffectZoneChoice count 3). Those 3 are exiled; P1 not eliminated; life 20; graveyard 1 |
| C-16 `…::dralnus_sacrifice_choice_is_cleared_inside_the_batch` → `…_is_asked_inside_the_batch` | Dralnu (P1) blocks a 2/2; P1 has 3 Bears and a Courser | 608.2d, 701.21a, ruling S11 | nothing sacrificed | per-leg | P1 chooses 2 permanents (Dralnu eligible). Choosing two Bears sacrifices exactly those two; Dralnu damage 0; P0's Bears dies to Dralnu's 3 |
| C-17 `…::outfitted_jousters_sacrifice_in_combat` | Jouster, 1 or 2 Equipment, 1 or 2 blockers | 608.2d, 701.21a, 615.5 | 2 equipped: none sacrificed | per-leg | 1 equipped: unchanged (the Lance is sacrificed, no prompt). 2 equipped, 1 blocker: P0 picks one; that one is sacrificed and the other stays. 2 equipped, 2 blockers: each leg's prevention runs its own "When damage is prevented this way" (per-source prevention, as Ironscale's ruling counts per creature). Leg 1 asks, the pick is sacrificed; leg 2 sacrifices the remaining one with no prompt. Prevented aggregate 1 in every row (§16 U4) |
| C-18 `ironscale_hydra_combat_prevention::ironscale_counter_order_is_settled_inside_the_batch` → `…_is_asked_inside_the_batch` | Ironscale Hydra (P0) blocked by Courser; P0's Scales + Season | 616.1g (Ironscale ruling) | `[3,4]` | per-leg | P0 asked: Scales first → 4, Season first → 3. Courser dies; Bolt kills Bears |
| C-19 `…::two_ironscale_hydras_each_settle_their_own_counter_order` → `…_each_ask_their_own_counter_order` | two Hydras | 616.1, 101.4 | `[3,4]` each | per-leg | Two P0 prompts. All Scales → both 4; rerun all Season → both 3 |
| C-20 `…::a_damage_order_then_a_rider_order_in_one_portion` | + Furnace | 616.1, 616.1g | `[3,4]` | per-leg | Two P0 prompts (the damage order, then the rider order). Rider Scales → 4, Season → 3. Bolt reach `[6]` kept |
| C-21 `…::a_rider_order_of_three_or_four_effects_is_settled_in_one_legal_order` → `…_is_asked_in_one_legal_order` | 2S+1D, 1S+2D, 2S+2D | 616.1f | legal sets | per-leg | Policy "a Hardened Scales while one is offered" → 6 / 8 / 12; policy "a Doubling Season while one is offered" → 4 / 5 / 6. Each asserted exactly |
| C-22 `…::a_settled_rider_order_lets_the_next_turn_come` → `an_answered_rider_order_lets_the_next_turn_come` | | 704.3 (P7, P8) | `[3,4]` | per-leg | Answer Scales → 4; the next turn is reached; Bolt kills Bears |
| C-23 `…::double_strike_settles_a_rider_order_in_each_damage_step` → `…_asks_…` | Boros Swiftblade blocked by P1's Hydra; P1's Scales + Season | 510.4 | `[6,7,8]` | per-leg | One P1 prompt per step: all Scales → 8, all Season → 6. Without Scales: 2 |
| C-24 `…::a_first_strike_lifelink_answer_then_a_regular_step_rider_order` | Fiendslayer Paladin + Rhox/Leyline (P0); P1's Hydra + Scales/Season | 702.15b, 616.1 | 1 prompt | per-leg | Prompts: Rhox (P0) then the rider order (P1); 2 with Scales, 1 without. Rider Scales → 4, Season → 3; life +5 |
| C-25 `unpreventable::a_player_who_loses_inside_the_batch_leaves_and_the_batch_goes_on` | 3 players; P0's Hydra + Scales/Season; P1's Lich | 800.4a, 800.4e, 616.1, 101.4 | `[3,4]` | per-leg + APNAP | The Courser→Hydra leg affects P0 (active) and is asked first. Scales → 4, Season → 3. The rest is unchanged: P1 eliminated; P2 18; P0 22; nothing dealt to P1 |
| C-26 `unpreventable::nothing_happens_in_the_batch_after_the_game_ends` | Lich ends the game at the Hill Giant's leg | 104.1, 101.4 | later portions never dealt | APNAP restage | Main orders legs by the affected seat, and P0's legs come first. **Restage boards 1 and 2** so every rider-bearing leg affects P1 and follows the Lich leg in assignment order: board 1 uses P1's Outfitted Jouster (two Equipment) blocking P0's Grizzly Bears; board 2 uses P1's Ironscale Hydra under P1's Scales + Season blocking P0's Centaur Courser, Giant declared first. Then the original assertions hold: no prompt, prevented 0, Equipment on the battlefield / Hydra counters 0, the blocked creature damage 0. Boards 3 (Hydra declared first, no Scales) and 4 (Swans + Sheoldred) are unchanged |

**Unchanged** (no per-leg prompt, or the prompt is a Phase D / in-leg rider that P1 keeps):
- `gatta_and_luzzus_counter_order_is_settled_after_the_batch`, `a_lifelink_answer_then_gatta_and_luzzus_rider`,
  `channel_harms_may_is_settled_after_the_batch`, `new_way_forwards_draws_are_kept_under_abundance`:
  Phase D, DEFERRED(phase 2).
- `nothing_happens_in_the_batch_after_a_lifelink_gain_ends_the_game`,
  `a_prevention_rider_that_ends_the_game_ends_the_batchs_riders`, `nefarious_lich_ends_the_game_inside_the_batch`.
- `jared_carthalion_reads_the_monarchy_before_combat_damage_moves_it`,
  `undergrowth_champion_double_blocked_reads_its_guard_once`, `a_false_guard_stays_false_when_a_rider_adds_a_counter`,
  `redirected_portions_meet_the_batch_reading`: no park, so one guard read per pass entry equals one
  per batch.
- The six non-rider Ironscale rows; `phytohydra_replaces_damage_preventable_or_not`;
  `lifelink_has_nothing_to_gain_from_replaced_damage`; `a_substitution_covers_only_its_recipient`;
  `a_substitution_inside_the_batch_does_not_strip_a_board_census_guard`; every noncombat row.

Each converted test's own strict-failure comment ("combat can't ask … Command Zone #140") is replaced
by a doc line naming the asked choice and its CR. The **module-doc** paragraphs of the three files are
DEFERRED(phase 2).

**Any further failure at verification** is classified before it is touched:
- (a) a combat prompt the branch settled: rewrite as above;
- (b) an APNAP reorder: CR 101.4, restage and keep the intent;
- (c) a CR 616.1 materiality prompt between a substitution and another replacement: answer both ways;
- (d) a defect: fix the code.

The class goes into commit B's message. No assertion is relaxed.

### 11.3 New tests: M-Q4c, M-Q4d, M-Q4e, M-L2, M-F1 (§10) and Steps B2–B6

## 12. Reference Readings

- **Main's #9670 suites "pass unchanged in intent"** (`soul_scar_mage_damage_to_counters`,
  `szadek_combat_damage_to_counters`, `undead_alchemist_combat_damage_to_mill`,
  `adamantoise_vigor_combat_order`, `combat_hit_bookkeeping_follows_delivery`, `awe_strike_prevention`,
  `ultimate_magic_meteor_per_opponent_destroy`, `swans_prevention_followup`). Derivations for the rows
  P1's resolution can move:
  - **Lightning Helix** at P1's Vigor-protected Hill Ogre under Soul-Scar Mage: one event, two
    applicable effects, so P1 chooses (CR 616.1). The instruction then gains 3 once (CR 608.2c).
    Matches `replacement_order_choice_runs_the_tail_once`.
  - **Vampires' Vengeance** under Soul-Scar Mage, Doubling Season and Vizier of Remedies: Doubling
    Season first gives 2×2−1 = 3 counters each, and the Blood token is created once, after the damage
    instruction (CR 608.2c). The branch path parks each remaining recipient beneath the rider and the
    tail at the child boundary, which gives the same order.
  - **Allies at Last** under Soul-Scar Mage: (2·3−1) + (2·2−1) = 8. In the branch's batch, main's
    Soul-Scar substitution is held with `AfterEvent` timing. Its `WithEvent` timing is
    DEFERRED(phase 3), S10. The counts and the prompt count (2) are timing-independent here.
  - **Szadek + Mending Hands** (ruling S8): Mending Hands first → 1 counter.
  - **Meteor**: the tail keeps its casting context through `stashed_tail`, with the branch's body
    calling `apply_parent_chain_context`.

  None of these disagrees with main's measured reading, so there is no defective-reference route.
  This is unprobed: §16 U1 settles it at verification.
- **Branch suites "pass"**: §11.1 rows are derived there. The §11.2 combat values are derived from the
  card and CR 616.1, never copied from the branch's "set the player could reach" assertions.
- **Not derivable from the card:** the order of two combat legs at one seat. That is assignment order,
  main's stable sort in `order_batch_by_affected_seat`, an engine convention. C-07, C-19 and C-26 are
  written so their values do not depend on it, except C-26's restage, which states it.

## 13. Serialized surfaces

- `PostReplacementDrain::event_amount`: both sides added it, with the same type. Main's
  `#[serde(default)]` replaces the branch's `skip_serializing_if`. A `None` now serializes as `null`
  instead of being omitted, and both shapes deserialize. No protocol bump.
- `PendingCombatDamageReplacement` is unchanged in P1. Its `settled_guards` is DEFERRED(phase 2), with
  the registry step.
- Both sides' existing protocol records auto-merge (`protocol/changes/`): the branch's `c85ad0f6`
  and `ccf4c183`, main's `94232026`. Step A10 runs `node scripts/protocol-versions.mjs check`.

## 14. Step-by-step plan

### Commit A: the merge commit (every hunk resolved; compile repairs; census regenerated)

Run `git merge --no-ff 91639fdf02` on `engine/unpreventable-damage-riders` in the executor's worktree.
The commit message names main's merged SHA and lists every compile repair. It contains no model
identifiers.

**General rule for `deal_damage.rs` and `replacement.rs`.** Git auto-merged main lines into function
bodies **outside** the markers. Resolve the functions named below at **function level**: replace the
whole function, from its `fn` line to its closing brace, with the body named here. Leave every other
auto-merged region as it is. Main's only non-#9670 change in `deal_damage.rs` (e60daed343, Steel
Hellkite) sits in `collect_matching_players`, which keeps its merged text.

**A1. `crates/engine/src/game/effects/deal_damage.rs`**
1. **Imports:** the branch's `use crate::types::game_state::{DamageBatchLedger, DamageRecord, DistributionScope, EventLosses, GameState, HeldBatchRider, PendingContinuation, ResidentDrainPolicy, RiderTiming, TriggerSourceRead, WaitingFor}`.
   Drop any name the compiler reports unused.
2. **`apply_damage_to_target`, Prevented arm:** the branch's `crate::game::replacement::drain_damage_rider(state, events);`.
   Keep the surrounding `if !is_combat && state.has_post_replacement_drain()`. Delete main's
   CR 616.1g comment block and its `apply_pending_post_replacement_effect` call.
3. **`build_remaining_damage_node`:** main's signature `(ability, damage_source_id, target, amount)`
   using `ability.controller`. **Delete** its `carry_instruction_context(&mut node, ability)` call and
   delete `carry_instruction_context`. The context comes from `keep_chain_identity`, which
   `remaining_damage_chain`, the only caller, applies.
4. **Region after it (hunk with main's 129 lines):**
   - **keep** main's `DamageTail`, `is_multi_event_damage_instruction`, `DamageInstructionResolver`,
     `multi_event_damage_resolver` and `resolve_instruction`;
   - **delete** main's `build_post_replacement_damage_node`, `stash_remaining_post_replacement_damage`
     and `pause_behind_continuation_prompt`;
   - extend `multi_event_damage_resolver` with:
     ```rust
     Effect::Fight { .. } => Some(super::fight::resolve_fight_damage),
     Effect::ResumeDamageBatch { .. } => Some(resolve_resume_damage_batch),
     Effect::ApplyPostReplacementDamage { .. } => Some(|state, ability, events| {
         resolve_post_replacement(state, ability, events).map(|()| DamageTail::Caller)
     }),
     ```
   - Rewrite its doc and `is_multi_event_damage_instruction`'s doc to say: "every instruction that
     deals damage through the replacement pipeline, including a fight's two directions (CR 701.14a),
     a parked two-phase batch (`ResumeDamageBatch`, a carrier with no tail rule of its own) and an
     already-replaced portion."
5. **`remaining_damage_head`:** the branch's body, with `remaining_damage_chain(state, ability, ..)`
   and the tail built by `stashed_tail(state, sub, ability)`.
6. **Tail builder:** one fn **`stashed_tail(state: &mut GameState, sub: &ResolvedAbility, ability: &ResolvedAbility) -> ResolvedAbility`**
   with the branch's `cast_tail_with_parent_targets` body and doc, including the kicker paragraph and
   the Searing Blaze "instead" base-clause binding. Rename every `cast_tail_with_parent_targets` call
   to `stashed_tail`, including the one in `park_damage_batch`.
7. **`keep_chain_identity` … `settle_joined_damage_run` region:** the branch's `keep_chain_identity`,
   `named_damage_dealer`, `next_damage_clause`, `joins_next_damage_clause` and
   `settle_joined_damage_run`. **Delete** main's `stash_each_source_combined_continuation` and
   `stash_remaining_each_source_damage`, and the branch's `report_parked_tail`.
8. **Ownership functions:** delete the branch's `damage_resolver_owns_tail` and
   `parked_batch_owns_tail` with their docs (Q2 addendum).
9. **`run_damage_batch` / `run_damage_batch_phases`:** the branch's bodies, with the return type
   `Result<DamageTail, EffectError>`.
   - The game-ended early return → `Ok(DamageTail::Caller)`.
   - Each `park_damage_batch(...)?; return Ok(());` → `return park_damage_batch(...);`.
   - The two terminal `return Ok(())` after `game_end` / `settle_event_losses` → `return Ok(DamageTail::Caller);`.
   - Completion → `Ok(DamageTail::Caller)`.
   - `run_damage_batch`'s wrapper returns `result` unchanged (type follows).
   - Doc addition: "CR 608.2c: reports who places the instruction's tail (`DamageTail`): the park
     decides it, and completion leaves it to the chain."
10. **`park_damage_batch`** → `Result<DamageTail, EffectError>`.
    - After the child-boundary insert: `return Ok(DamageTail::Caller);`. The chain inserts the tail
      at the same boundary, beneath the batch.
    - After parking the tail (if any) and then the batch: `Ok(DamageTail::Stashed)`.
    - Doc: replace the branch's "where the chain parks the instruction's later instructions beneath
      it" with "and reports `Caller`, so the chain parks the instruction's later instructions beneath
      it; with none raised … parks them beneath it here and reports `Stashed`".
11. **`run_object_damage_batch`** → `Result<DamageTail, EffectError>` (returns `run_damage_batch`).
12. **`resolve_resume_damage_batch`** → `Result<DamageTail, EffectError>`. The body is unchanged, so it
    returns the `with_damage_batch(.. run_damage_batch ..)` value.
13. **DealDamage entry points (hunks at `resolve` / `resume_resolution_division` / the fn header):**
    ```rust
    pub fn resolve(..) -> Result<(), EffectError> {
        resolve_damage(state, ability, RemainingDamageSplice::AfterContinuation, events).map(|_| ())
    }
    // resume_resolution_division: `resolve_damage(state, &stamped, RemainingDamageSplice::BeforeContinuation, events).map(|_| ())`
    /// CR 608.2f + CR 700.1: one damage instruction as one simultaneous batch, reporting who places its tail.
    fn resolve_damage(state, ability, splice, events) -> Result<DamageTail, EffectError> {
        replacement::with_damage_batch(state, ability, |state| resolve_with_splice(state, ability, splice, events))
    }
    ```
    `resolve_with_splice` keeps the branch's name, doc and body, returning `Result<DamageTail, EffectError>`:
    - no damage source → `Caller`;
    - `resolve_each_target_power_damage(..)` → `return` its value;
    - the division pause → `.map(|()| DamageTail::Caller)`;
    - both rider-pause arms (`DamageResult::Applied(_) if damage_batch_has_paused_rider(state) && park_recipients_behind_paused_rider(..)?`)
      → `return Ok(DamageTail::Caller);`;
    - both `NeedsChoice` arms → stash, **delete** `report_parked_tail`, `return Ok(DamageTail::Stashed);`;
    - completion → `Ok(DamageTail::Caller)`.

    Main's `multi_event_damage_resolver` entry for `DealDamage` already calls `resolve_damage(..)`.
14. **`resolve_each_target_power_damage`:** the branch's body (`run_damage_batch(..)`) with
    `-> Result<DamageTail, EffectError>`. **Delete** main's Phase B/C body, `apply_batch_survivors`
    and `apply_batch_survivors_before_replacement_pause`.
15. **DamageAll:** take main's hunk side
    (`pub fn resolve_all(..) { resolve_all_damage(state, ability, events).map(|_| ()) }` and the
    `fn resolve_all_damage(..) -> Result<DamageTail, EffectError>` header). Give `resolve_all_damage`
    the body `replacement::with_damage_batch(state, ability, |state| resolve_all_portions(state, ability, events))`.
    Then add `fn resolve_all_portions(..) -> Result<DamageTail, EffectError>`: **the branch's
    function body verbatim**, with returns mapped as follows:
    - rider pause → `Caller`;
    - `NeedsChoice` (stash + `mark_pending_continuation_parent(state, EffectKind::DamageAll)`) → `Stashed`;
    - completion and early returns → `Caller`.

    The loop hunk takes the branch arms. Main's `pause_behind_continuation_prompt` arm and its `_ =>`
    are dropped.
16. **DamageEachPlayer:** the same pattern. `pub fn resolve_each_player` →
    `resolve_each_player_damage(..).map(|_| ())`; `fn resolve_each_player_damage` →
    `with_damage_batch(.. resolve_each_player_portions ..)`; `fn resolve_each_player_portions` gets
    **the branch's body verbatim**.
    - The loop's `if dmg > 0 { … }` block is the branch's. **Delete** main's auto-merged lines after
      the marker, from `let result = apply_damage_to_target(…)` through `return Ok(DamageTail::Stashed);`.
      They duplicate the branch's block.
    - Returns: rider pause with remaining recipients parked → `Caller`; `NeedsChoice` (stash + the kept
      `mark_pending_continuation_parent(state, EffectKind::DamageEachPlayer)`) → `Stashed`;
      completion → `Caller`.
17. **EachDealsDamageEqualToPower:** `pub fn resolve_each_deals_equal_to_power` →
    `resolve_each_deals_power_damage(..).map(|_| ())`; `fn resolve_each_deals_power_damage` →
    `with_damage_batch(.. resolve_each_deals_equal_to_power_portions ..)`; the `_portions` fn gets the
    branch's body (`run_damage_batch(..)`) → `Result<DamageTail, _>`. Delete main's body.
18. **EachSourceDealsDamage:** the same, with `resolve_each_source_damage` and
    `resolve_each_source_deals_damage_portions`. The branch's `run_damage_batch` body. Delete main's
    114-line body.
19. **After `object_power_with_lki`:** delete main's `stash_remaining_each_deals_chain`.
20. **Expected result:** no `pause_behind_continuation_prompt`, `carry_instruction_context`,
    `cast_tail_with_parent_targets`, `report_parked_tail`, `damage_resolver_owns_tail` or
    `parked_batch_owns_tail` token remains in the file.

    Check: `grep -nE 'pause_behind_continuation_prompt|carry_instruction_context|cast_tail_with_parent_targets|report_parked_tail|damage_resolver_owns_tail|parked_batch_owns_tail' crates/engine/src/game/effects/deal_damage.rs` is empty.

**A2. `crates/engine/src/game/effects/fight.rs`** (no conflict; compile-forced by A1.4)
```rust
pub fn resolve(..) -> Result<(), EffectError> { resolve_fight_damage(state, ability, events).map(|_| ()) }
/// CR 701.14a + CR 608.2f + CR 608.2c: a fight's two directions as one damage batch, reporting who places its tail.
pub(crate) fn resolve_fight_damage(state, ability, events) -> Result<DamageTail, EffectError> {
    crate::game::replacement::with_damage_batch(state, ability, |state| resolve_fight(state, ability, events))
}
```
`resolve_fight` → `Result<DamageTail, EffectError>`. Its two early `EffectResolved` returns →
`Ok(DamageTail::Caller)`. Its last line returns `run_object_damage_batch(..)`. Import
`crate::game::effects::deal_damage::DamageTail`.

**A3. `crates/engine/src/game/effects/mod.rs`**
1. `resolve_legacy_effect` (first hunk): take main's side, which drops the branch's
   `ResumeDamageBatch`, `EachDealsDamageEqualToPower` and `EachSourceDealsDamage` arms. **Also delete**
   the auto-merged `Effect::ApplyPostReplacementDamage { .. } => …` and
   `Effect::Fight { .. } => fight::resolve(..)` arms, and add `| Effect::Fight { .. } | Effect::ResumeDamageBatch { .. } | Effect::ApplyPostReplacementDamage { .. }`
   to the `unreachable!("multi-event damage dispatch precedes the legacy adapter")` arm.
2. `resolve_chain_body` (second hunk): main's side, `|| deal_damage::is_multi_event_damage_instruction(&ability.effect))`.
   Keep main's comment ("a damage instruction reaches here only when it stashed nothing
   (`DamageTail::Caller`)") and extend it with "or its rider paused and its remaining recipients, or
   its parked batch, wait inside the child stack". **Delete** the
   `let damage_rider_resident_before = …;` capture and the
   `if deal_damage::damage_resolver_owns_tail(…) { return Ok(progress); }` block (clean regions).
3. `resolve_effect_node`'s multi-event arm comment: append "including a fight (CR 701.14a) and a
   parked two-phase batch, whose park reports the same contract".

**A4. `crates/engine/src/game/effects/win_lose.rs`**
- `resolve_lose`: the branch's `lose_by_effect(state, &[pid], events);`.
- `resolve_win`: the branch's two lines (`// CR 104.2b + CR 104.3e: …` + `win_by_effect(state, winner, events);`).
- Compile repair: the import becomes `use crate::types::events::GameEvent;`, since `LossReason` is
  unused there.

**A5. `crates/engine/src/game/elimination.rs`**
- Imports (hunk): `use crate::types::ability::{Effect, EventOutcomes};` and
  `use crate::types::events::{GameEvent, LossReason};`.
- Clean-region compile repairs:
  1. Extract `fn eliminate_with_reasons(state: &mut GameState, losses: &BTreeMap<PlayerId, BTreeSet<LossReason>>, events: &mut Vec<GameEvent>)`
     from `eliminate_players_simultaneously`'s body (`eliminate_players(state, losses, DepartureTriggers::ScannedByCaller, events)`
     + the `looked_back.collect(state, events, departure_end)` block, with its CR 603.2 comment).
     `eliminate_players_simultaneously` builds its one-reason map and calls it.
  2. `lose_by_effect`, `None =>` arm: `eliminate_players_simultaneously(state, losers, LossReason::EffectStatedLoss, events)`.
  3. `win_by_effect`, `None =>` arm: `eliminate_players_simultaneously(state, &opponents, LossReason::OpponentWon, events)`.
  4. `settle_event_outcomes`:
     - Keep the CR 810.8a loop that extends `losers`.
     - Then build
       `let mut losses: BTreeMap<PlayerId, BTreeSet<LossReason>> = losers.iter().map(|&p| (p, BTreeSet::from([LossReason::EffectStatedLoss]))).collect();`.
     - For each winner not in `losers`, for each `living_opponents(state, *winner)`:
       `losses.entry(opponent).or_default().insert(LossReason::OpponentWon);`.
     - Then `if !losses.is_empty() { eliminate_with_reasons(state, &losses, events); }`.
     - Doc: "CR 104.3e + CR 104.2b + CR 104.3f + CR 104.4a + CR 810.8a: … For each winner who is not
       an effect loser, insert `OpponentWon` for every living opponent, including opponents already
       carrying `EffectStatedLoss`; union both reasons in the same set."
     - Verify `BTreeMap` / `BTreeSet` imports (both are used by main's code already).

**A6. `crates/engine/src/types/game_state.rs`**
1. **`LazyBoxedVec`** (both hunks): one definition.
   - `#[allow(clippy::box_collection)]` and `#[derive(Debug, Clone, PartialEq, Eq)]`.
   - Doc: "A rarely-populated `Vec` stored behind one pointer and allocated only while it holds an
     element: `None` when empty, and collapsed back to `None` whenever it empties, so a type carrying
     it widens by 8 B instead of 24 and an empty clone allocates nothing. Serializes exactly as the
     `Vec` it holds. Used for action-scoped working memory on `GameState` (empty at every action
     boundary) and for `ResolvedAbility` context lists (deep resolution chains keep many on the
     stack)."
   - Methods: `is_empty`, `as_slice`, `iter`, `iter_mut`, `push`, `remove`, `swap_remove`, `clear`,
     `take`; impls `Default`, `From<Vec<T>>`, `Serialize`, `Deserialize`.
2. **`PostReplacementDrain`** (clean region, compile repair): delete the second `event_amount` field.
   Keep one field at main's position, with `#[serde(default)]` and this doc:
   "/// CR 614.6 + CR 615.5: the amount of the event this drain's replacement replaced or prevented,
   latched when it applied (`replacement::replaced_event_amount`): a damage or life substitution's
   replaced amount ("that many"), or the damage a prevention actually prevented, zero included ("for
   each 1 damage prevented this way"). Dispatch binds it as the action's own amount
   (`quantity::with_replacement_action_scope`), ahead of the die result and the enclosing trigger's
   amounts; work the action leaves behind a prompt carries it
   (`PendingContinuation::replacement_action`). `None` when the replacement owns no amount."

   In `PostReplacementDrain::new`, keep one `event_amount: None`.
3. Delete `GameState::post_replacement_event_amount()`.
4. Doc-only:
   - `DamageBatchLedger::paused_rider`: "Combat settles every rider pause…" → "In combat a settled
     leg's own continuation parks the pass (`replacement::drain_combat_damage_continuation`), and
     only the in-leg earlier-rider and aggregate drains settle their pauses
     (`drain_combat_damage_rider`), so it is never set there."
   - The two `combat_one_shot_applications` mentions (the `combat_prevention_tally` doc and the
     CR 510.2 doc near `PendingCombatLifelink`) → `DamageBatchLedger::applications`.

**A7. `crates/engine/src/types/game_state_size.rs`:** keep both histories. The branch's paragraph
(15,920 B → boxed ledger → 15,808 B) comes first, then main's four entries ending at the 16,384
ceiling. The assert stays `16_384`. The merged-head measurement entry is commit C.

**A8. `crates/engine/src/game/engine.rs`:** main's import list, minus `PostReplacementDrainStack` (the
merged file only uses it fully qualified). `LoopDetectionMode` is not listed (main removed it).
`UntapStepProgress` stays.

**A9. `scripts/post-replacement-continuation-baseline.txt`:** after A1–A12 compile, run
`python3 scripts/post_replacement_continuation_census.py --write`. Expected `replacement.rs` rows:
`apply_single_replacement` (renamed from `_inner`) resolved 1 / stash 2 / template 5;
`continue_replacement_impl` template 1; `drain_combat_damage_continuation` consumer 1;
`drain_combat_damage_rider` consumer 1; `drain_damage_rider` consumer 1. There are no `deal_damage.rs`
consumer rows and no `replace_combat_damage_batch` row. The commit message lists each row change.

**A10. `crates/engine/src/game/quantity.rs`:** the hunk takes the branch's side
(`.or_else(replacement_action_amount)` with its comment) and drops main's tier.

**A11. `crates/engine/src/game/combat_damage.rs`**
1. `apply_combat_damage` hunk: main's side (`order_batch_by_affected_seat(state, &mut entries, &mut proposed_events);`).
   Drop the branch's tuple call and its CR 104.1 guard.
2. `drain_combat_lifelink` hunk:
   ```rust
   // CR 104.1: a gain that ended the game ends the batch; no rider runs after it.
   if state.game_end.is_none() {
       fire_combat_prevention_riders(state, &record.prevention_tally, &mut record.batch_events);
   }
   ```
   There is no loop-ring call (#10810).
3. `run_combat_damage_replacements` (clean region): its `Complete` arm becomes
   ```rust
   replacement::CombatDamageReplacementOutcome::Complete => {
       // CR 104.1: a game that ended inside the pass ends the batch: nothing is dealt, gained or prevented after it.
       if state.game_end.is_some() {
           return CombatDamageBatch::Complete(record.batch_events);
       }
       apply_combat_damage_survivors(state, record, entries, pass)
   }
   ```

**A12. `crates/engine/src/game/replacement.rs`**
1. **Imports hunk:** the union, `HeldBatchRider`, `RiderTiming`, `PendingCombatLegChoice` and
   `PendingCombatLegProgress` included.
2. **`mandatory_post_drain` hunk:** the branch's function
   (`fn mandatory_post_drain(continuation, source, controller, applied, replaced: ReplacedEventContext) -> PostReplacementDrain`).
   - Destructure `replaced` as main does (`source: event_source, target: event_target, amount: event_amount`).
   - Set `event_amount` from it.
   - Delete the `REPLACEMENT_AMOUNT_STAMP` line.
   - Follow it with the branch's `damage_rider_placement`.
   - The branch's `stash_post_replacement_continuation(state, drain, behind)` stays. Main's
     six-argument version is not taken.
   - `ReplacedEventContext`'s doc: "… and the amount its action owns (`replaced_event_amount`)",
     replacing "for a damage substitution only".
3. **Delete** the `REPLACEMENT_AMOUNT_STAMP` thread-local and `stamp_replacement_amount`. Its six call
   sites become `state.last_effect_count = Some(u32_to_i32_saturating(<amount>));`. Keep each site's
   guard and comment, and replace "stamp" with "record".
4. **After `damage_done_applier` hunk:** the union, the branch's six functions followed by main's
   `damage_execute_substitutes_event_type` and `is_damage_substitution` unchanged. Then add the new
   private items from §2.1: `event_quantity`, `ReplacementOutcome`, `damage_replacement_substitutes`,
   `damage_shape_prevents` and `replaced_event_amount`, each with CR-annotated docs. Add
   `lose_life_execute_substitutes_event_type` next to `gain_life_execute_substitutes_event_type`, and
   switch `lose_life_applier`'s inline test to it.
5. **`is_damage_prevention_replacement`:** keep the event gate and the definition lookup, then
   `if let Some(verdict) = damage_shape_prevents(repl) { return verdict; }`, then the legacy
   description test unchanged. The `Substitution` arm and its CR 614.1a / CR 615.1a comment move into
   `damage_shape_prevents`.
6. **Collapse `apply_single_replacement`:** delete the wrapper and rename
   `apply_single_replacement_inner` → `apply_single_replacement`.
   - Replace main's `substituted_damage_amount` block with
     `let replaced_quantity = event_quantity(&proposed);`, placed before the applier consumes
     `proposed`.
   - In the `Modified` arm's `ReplacedEventContext`:
     `amount: replaced_event_amount(state, rid, replaced_quantity, ReplacementOutcome::Modified(&new_event))`.
   - In the `Prevented | PreventedBehindRider` arm:
     `amount: replaced_event_amount(state, rid, replaced_quantity, ReplacementOutcome::Replaced)`.
   - Both arms keep the branch's `hold_damage_rider` / `damage_rider_placement` routing, which is
     already in the merged text.
7. **`replace_combat_damage_batch`:**
   - **Doc hunk:** main's text, plus "The batch's ledger (`GameState::damage_batch_ledger`) is
     installed on every entry with the pass's one-shot applications and a fresh guard reading
     (CR 608.2f), and the applications move back into the pass on every exit."
   - **Body-start hunk:** main's side, with the one-shot slot replaced:
     ```rust
     let settled_guards = read_damage_guards(state);
     let restore_ledger = state.damage_batch_ledger.replace(Box::new(DamageBatchLedger {
         applications: std::mem::take(&mut pass.one_shot_applications),
         settled_guards,
         paused_rider: None,
         held_riders: None,
         split_portions: Vec::new(),
     }));
     ```
     Keep main's AppliedReplacementKey comment.
   - **Loop hunks:** main's side. Insert immediately after the `if let Some((leg, advanced)) = step.take() { … }` filing block:
     ```rust
     // CR 104.1: a game that ended inside the pass ends it: no later leg is proposed or answered.
     if state.game_end.is_some() {
         for leg in &mut pass.resolved {
             if !matches!(leg, PendingCombatLegProgress::Settled(_)) {
                 *leg = PendingCombatLegProgress::Settled(None);
             }
         }
         break CombatDamageReplacementOutcome::Complete;
     }
     ```
     Insert immediately after the reconcile `if let … continue;`:
     ```rust
     // CR 800.4a + CR 800.4e: an unstarted leg whose recipient has left the game is not dealt, so it is never proposed.
     for leg in &mut pass.resolved {
         if matches!(leg, PendingCombatLegProgress::Unstarted(event) if combat_damage_recipient_has_left(state, event)) {
             *leg = PendingCombatLegProgress::Settled(None);
         }
     }
     ```
   - **Exit hunk:** main's side, with the one-shot restore replaced:
     ```rust
     pass.one_shot_applications = state.damage_batch_ledger.take().map(|ledger| ledger.applications).unwrap_or_default();
     state.damage_batch_ledger = restore_ledger;
     ```
   - **Delete** `settle_combat_order_parks`.
   - Keep `drain_combat_damage_rider`, `settle_combat_rider_pauses`, `CombatRiderPause`,
     `answer_combat_rider_park` and `abandon_combat_rider_pause`, which are DEFERRED(phase 2).
     `drain_combat_damage_rider`'s doc becomes "… at a combat drain that P1 still settles in the
     batch: the substitution arm's drain of an earlier rider inside a leg, and Phase D's aggregate
     drain …".
   - `CombatDamageReplacementPass::one_shot_applications` doc: "(installed as
     `DamageBatchLedger::applications` while the pass runs)".
   - **`drain_combat_damage_continuation`:** main's function, with the suspension using the ledger
     (`let ledger = state.damage_batch_ledger.take(); … state.damage_batch_ledger = ledger;`, CR 700.1
     comment kept). After the restore, add
     `if state.game_end.is_some() { return CombatDamageReplacementStep::Settled(survivor); } // CR 104.1`,
     placed before the `if let Some(waiting_for) = paused` check.
8. **`continue_replacement_impl`** (last hunk): keep main's `ReplacedEventContext` computation, with
   its amount line replaced by
   `amount: replaced_event_amount(state, rid, event_quantity(&proposed), ReplacementOutcome::AcceptedBeforeApplying),`.
   Then:
   ```rust
   let drain = PostReplacementDrain {
       status: DrainStatus::Ready(PostReplacementContinuation::Template(def)),
       source: Some(rid.source),
       applied: proposed.applied_set().clone(),
       event_source: replaced.source,
       event_target: replaced.target,
       event_amount: replaced.amount,
       controller: branch_controller,
   };
   // CR 616.1: a damage batch settling its original choices holds the branch's action as it holds a mandatory one (`hold_damage_rider`).
   let drain = match &proposed {
       ProposedEvent::Damage { target, .. } => { let target = target.clone(); hold_damage_rider(state, rid, Some(&target), drain) }
       _ => Some(drain),
   };
   if let Some(drain) = drain { state.install_post_replacement_drain(drain, ResidentDrainPolicy::Replace); }
   ```

**A13. Compile repairs outside the markers** (every one listed in the commit message):
- `crates/engine/src/types/resolution.rs` (`LegacyReplacementTailsWire`): keep main's
  `event_amount: None` with its comment and delete the duplicate.
- `crates/engine/src/game/targeting.rs` (two test literals), `crates/engine/src/game/replacement.rs`
  (a test literal near the old `22806` line), `crates/engine/tests/integration/swans_prevention_followup.rs`
  and `crates/engine/tests/integration/spell_resolution_final_part_after_paused_instruction.rs`: one
  `event_amount: None` each.
- `crates/engine/src/game/engine_replacement.rs`: the comment near
  `dispatch_pending_post_replacement_effect` that names `replacement::stamp_replacement_amount` →
  `replacement::replaced_event_amount` (comment only).
- Any `PlayerLost { player_id }` pattern without `..`, or other arity site the compiler names: fix in
  place, behaviour-preserving, and list it. The planning grep found none in the branch test files.

Before committing, run `cargo fmt --all`. Through Tilt, check that `clippy` (workspace, all targets)
is green on the merge commit. Tests may be red only for the rows §11.2 converts in commit B. Record
which failed.

### Commit B: conversions and new tests

- **B1.** Make the test-helper changes in §11.2, then convert rows C-01 … C-26 exactly as tabled.
- **B2.** In `crates/engine/src/game/quantity.rs`, add the tests module unit test
  `replacement_action_scope_restores_the_outer_scope_after_nested_work` (row M-Q4c).
- **B3.** In `damage_substitution_replacements.rs`, add `a_riders_amount_does_not_reach_the_spells_tail`
  (row M-Q4d).
  - Setup: P0's hand holds Test of Faith, Fateful Showdown and five other cards, and P0's library
    holds at least 8 cards. P1 has a 3/3. P0 casts Test of Faith at P1's 3/3, then casts Fateful
    Showdown at it.
  - Arithmetic: at resolution the hand is 5, so Fateful Showdown deals 5. Test of Faith prevents 3,
    the 3/3 is dealt 2 and gets the counters, P0 discards 5 and draws 5.
  - Loop variant `[without, with]`: P1 controls Hardened Scales + Doubling Season, which pause the
    rider. Every `ReplacementChoice` is answered with Doubling Season.
  - Assertions:
    - exactly 5 `CardDrawn` for P0;
    - 5 Showdown-discarded cards in P0's graveyard plus Test of Faith and Showdown;
    - the 3/3 `damage_marked == 2`;
    - the reach-guard: `CounterAdded` on the 3/3;
    - in the paused variant, every `CardDrawn` index is after the last `CounterAdded`.
  - Precondition (§16 U5): "draw that many" lowers to `QuantityRef::EventContextAmount`. If it lowers
    to another reference, the test is still added, its doc says it is not discriminating for the
    cascade, and the orchestrator is told.
- **B4.** In `damage_substitution_replacements.rs`, add `a_paused_fights_tail_runs_once` (row M-F1).
  - Board: P0 controls a red 5/5 and Torbran, Thane of Red Fell. P1 controls a 2/2. Furnace of Rath is
    on the battlefield (P0's).
  - P0 casts The Last Agni Kai at the 5/5 and the 2/2. The 5/5's 5 to the 2/2 meets Torbran and
    Furnace, an order P1 chooses (CR 616.1). The 2/2's 2 to the 5/5 meets only Furnace.
  - Assertions: before the answer, no {R} in P0's pool. Torbran first → (5+2)×2 = 14 dealt, 2/2's
    lethal 2, excess 12 → 12 {R}. Furnace first → 5×2+2 = 12, excess 10 → 10 {R}.
  - Reach-guard: the prompt's candidates are {Torbran, Furnace}.
  - Fallback if the excess clause doesn't resolve at the merged head (the reach-guard sees 0 {R} after
    the answer): Skyshroud Ambush on the same board. The 5/5 wins the fight, so P0 draws exactly 1
    card after the answer and 0 before.
- **B5.** In the replacement.rs tests module, add the unit test
  `replaced_event_amount_latches_each_owning_kind_and_zero_prevented` (row M-Q4e). Definitions are
  built with the existing `ReplacementDefinition` builders, installed on test objects, and pending
  shields go on `state.pending_damage_replacements`.
- **B6.** In the elimination.rs tests module, add `settle_event_outcomes_unions_an_effect_loss_with_an_opponents_win`
  (row M-L2). Update the four existing tests of rows M-L1 and M-L3 with their `PlayerLost` reason
  asserts.

### Commit C: the size budget entry

Measure `size_of::<GameState>()` at the commit-B head (64-bit, the command in
`game_state_size.rs`'s module docs). Append one entry: platform, toolchain, measured bytes, the fields
the merge added on each side, and "under the unchanged 16,384 ceiling". If the measurement exceeds
16,384, stop and report. Do not widen.

### Verification (P1 green tree; Tilt first, else direct, never both)

1. `cargo fmt --all`.
2. Tilt resources: `clippy`, `test-engine` (all shards, default stack, including
   `mutable_pupa_perpetual_keyword_mirror::kathril_reaches_matching_counter_and_tail_past_false_earlier_gates`
   and the `game_state_size` const asserts), `test-ai` (`phase-ai --lib`), `wasm`, `server`.
3. `cargo test -p phase-parser --lib`. The parser is unchanged, so this is a smoke run.
4. Direct: `scripts/check-engine-authorities.sh` (the continuation census included) and
   `node scripts/protocol-versions.mjs check`.

Run-level rows (`phase-inputs/run-level`, not re-planned here): GameState ≤ 16,384 B (measured in
commit C, re-measured in P2 if a carrier grows), and Kathril within 16 MiB (P1 changes frame return
types).

## 15. Skills and checklist steps (Step 1)

- **`/add-replacement-effect`.**
  - Phase 1 (types): N/A, no new event or condition.
  - Phase 2 (registry): N/A, no new handler.
  - Phase 3 (parser): N/A (P3 owns the parser-side fold).
  - Phase 4 (engine integration): `apply_post_replacement_effect` / dispatch are unchanged except the
    amount source, §2.1.
  - Phase 5 tests:
    - pipeline: M-Q4e;
    - engine flow: §11.2 and M-Q4a/b;
    - provenance: §10 rows;
    - candidate-count: C-01–C-10 (2+ candidates), and the unchanged single-candidate rows (C-17 with
      one Equipment);
    - multi-replacement: C-07, C-10;
    - optional accept and decline: C-09;
    - serialized state: §13 (no new field), R5.
- **`/add-interactive-effect`.**
  - Phases 1–7: N/A. No new `WaitingFor` / `GameAction`. AI and frontend already handle
    `ReplacementChoice` / `EffectZoneChoice`, which main's pass already raises in combat.
  - Phase 8: round-trip tests C-01 … C-26 through `ChooseReplacement` / `SelectCards`; decline C-09,
    C-12; resume-state C-22.
  - DEFERRED(phase 2): the Phase D and in-leg asks.
- **`/casting-stack-conditions`:** the sub-ability chain (`resolve_chain_body` tail ownership, §2.4),
  WaitingFor resume through `resume_pending_combat_lifelink` (unchanged).
- **`/card-test`:** every runtime row above uses its production entry and completion boundary, and
  every negative assertion names a reach-guard.
- **DEFERRED(phase 2):** guards across a park; the in-leg earlier-rider park; Phase D asks; deletion of
  the auto-settle machinery and the combat fence; the module-doc paragraphs. Rows: the two-aggregate-
  shields row, the in-leg earlier-rider row (plan-r1 V3.11), and the cross-park guard row (plan-r1
  V3.10).
- **DEFERRED(phase 3):** the substitution predicate fold, the parse and predicate rows, the
  substitution-vs-redirect order row, and the Soul-Scar Mage fight-ruling row (S10, `WithEvent` for
  main's cards).

## 16. Unprobed assertions (each settled at implementation by the named test)

- **U1.** Main's #9670 suites stay green with the branch's batch and the branch's rider placement in
  place of main's per-node stash (§12). Settled by the suites themselves. A failure is classified per
  §11.2 (c/d) before any edit.
- **U2.** No branch test reads a drain-time `EffectResolved { DealDamage }`, which `report_parked_tail`'s
  deletion removes. Grep at planning: `resolved(&events, …)` is used only for Draw, DamageEachPlayer
  and DamageAll, whose tags stay. Settled by test-engine.
- **U3.** Whether Dapper Shieldmate's shield-counter replacement is order-material against Furnace in
  main's pass (C-04). The row asserts the measured presence, and both answers give equal values.
- **U4.** C-17's two-blocker row: the Jouster's per-event rider runs once per prevented leg in main's
  pass. If the engine runs it once per batch, stop and report the reading. It is a card-reading
  question (no ruling exists) and is not to be resolved by editing the assertion.
- **U5.** Fateful Showdown's "draw that many" lowers to `EventContextAmount` (B3); The Last Agni Kai's
  excess clause resolves (B4). Each has its reach-guard and its fallback.
- **U6.** The PreventionMinus-plus-`runtime_execute` shape has no producer (`runtime_execute` is set
  only by `create_draw_replacement`, `create_planeswalk_replacement` and `prevent_damage`, none of
  which builds `PreventionMinus`). So the amount authority's unconditional `Some(prevented)` differs
  from the branch's tally-gated stamp only on a shape no card produces. A LifeLoss `Prevented` arm
  reached through a static `Prevent` quantity modification with a non-`LoseLife` execute has no
  printed producer either.
- **U7.** `GameState` stays ≤ 16,384 B, and Kathril fits 16 MiB with `DamageTail` returns on the
  fight and batch frames (commit C; run-level).
- **U8.** The leg order of `nothing_happens_in_the_batch_after_the_game_ends`'s restaged boards
  follows assignment order within P1's seat (C-26).

## 17. For the orchestrator / human

- No human decision is open for P1. The `upstream/main` drift past the pinned SHA is noted in the
  header and is a run-level synchronization question.
- U4 (Outfitted Jouster, two blockers) is the one place where the derived reading rests on an analogy
  (Ironscale Hydra's ruling) rather than on a ruling of the card itself. If implementation measures a
  different engine reading, it comes back as a card-reading question, not a test edit.
