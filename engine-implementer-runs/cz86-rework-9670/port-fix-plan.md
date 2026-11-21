# Port fix plan: maintainer review of the port (F1 + F2), one batch

Mode: ordinary engine-planner plan for one `/address-review` batch (one push, one reply).
REVIEWED_HEAD = BASE_SHA = `f995714a3c` (pushed). Main to merge: `upstream/main` = `4ba131744b`
(#10016) at planning time. Review text: `pr-review-port-r1.md` (verbatim, read in full).
Nothing below was compiled or run by the planner (read-only constraint, no builds): every runtime
claim is **unprobed** unless it names a source line, and the executor's E-rows (§3.5) measure them.

## 0. Findings and dispositions

| Finding | Disposition | Closing target (the maintainer's `Closes when:`) |
|---|---|---|
| F1 HIGH, a parked combat batch rereads its prevention guards on resume | **fix** | A public-action regression with P0's Undergrowth Champion (one +1/+1 counter) attacking, blocked by P1's Angrath's Marauders and Grizzly Bears, reaches both ordering prompts, prevents both blockers' damage, removes one counter and leaves Champion alive, including a save/reload between the answers. |
| F2 MED, no parse comparison for the ported head | **fix** | The merged head is pushed, CI posts a parse sticky naming it, and its changed-card set is reconciled with the description's claims. Integration condition: a wither/infect leg that pauses for a counter-replacement choice inside a parked or resumed batch has one owner for its tail, with witnesses from each side green on the merged head. |

No 🟡 item is acted on. No earlier finding is carried (the review says so).

## Step 0 — premise (Oracle text, verified)

Source: `/mnt/data/dev/phase/client/public/card-data.json` (`oracle_text`), cross-checked against
`data/mtgjson/AtomicCards.json` 5.3.0+20261021 where noted.

- **Undergrowth Champion** (2/2): "If damage would be dealt to this creature while it has a +1/+1
  counter on it, prevent that damage and remove a +1/+1 counter from this creature.\nLandfall —
  Whenever a land you control enters, put a +1/+1 counter on this creature." (both sources)
- **Angrath's Marauders** (4/4): "If a source you control would deal damage to a permanent or
  player, it deals double that damage to that permanent or player instead." (both sources)
- **Grizzly Bears** (2/2): vanilla (no Oracle text in either source).
- **Hardened Scales**: "If one or more +1/+1 counters would be put on a creature you control, that
  many plus one +1/+1 counters are put on it instead."
- **Doubling Season**: "If an effect would create one or more tokens under your control, it
  creates twice that many of those tokens instead.\nIf an effect would put one or more counters on
  a permanent you control, it puts twice that many of those counters on that permanent instead."
- **Temper**: "Prevent the next X damage that would be dealt to target creature this turn. For
  each 1 damage prevented this way, put a +1/+1 counter on that creature."
- **Fencing Ace** (1/1): "Double strike (This creature deals both first-strike and regular combat
  damage.)"
- **Sickle Ripper** (2/1, keywords [Wither]): "Wither (This deals damage to creatures in the form
  of -1/-1 counters.)"
- **Winding Constrictor**: "If one or more counters would be put on an artifact or creature you
  control, that many plus one of each of those kinds of counters are put on that permanent
  instead.\nIf you would get one or more counters, you get that many plus one of each of those
  kinds of counters instead." Parsed: `AddCounter`, `quantity_modification: Plus 1`.
- **Vizier of Remedies**: "If one or more -1/-1 counters would be put on a creature you control,
  that many -1/-1 counters minus one are put on it instead." Parsed: `AddCounter`,
  `quantity_modification: Minus 1`, `counter_match: M1M1`.
- **Darksteel Colossus** (11/11): "Trample (…)\nIndestructible (…)\nIf Darksteel Colossus would be
  put into a graveyard from anywhere, reveal Darksteel Colossus and shuffle it into its owner's
  library instead."
- **Pit Fight**: "Target creature you control fights another target creature. (Each deals damage
  equal to its power to the other.)"

The review's witness premise holds: both blockers are sources P1 controls, so Marauders applies to
each blocker's portion, and each portion also meets Champion's shield; P0 (Champion's controller)
orders them (CR 616.1). The card-data export is stale for Champion (`condition: null`; the PR's
parser adds the guard), which does not affect the Oracle text.

CR text grep-verified in `docs/MagicCompRules.txt`: 510.2, 510.4, 608.2c, 608.2f, 604.2, 611.3a,
614.6, 614.16, 615.1a, 615.5, 616.1, 616.1e, 616.1f, 616.1g, 700.1, 701.14a, 702.12b, 702.80a,
702.90c, 101.4, 120.1, 120.3d, 120.3e, 120.3f, 120.10. Key texts:
- CR 510.2: "Second, all combat damage that's been assigned is dealt simultaneously. This
  turn-based action doesn't use the stack. No player has the chance to cast spells or activate
  abilities between the time combat damage is assigned and the time it's dealt."
- CR 608.2f (multiple objects processed simultaneously), CR 700.1 ("One 'happening' may be treated
  as a single event by one ability and as multiple events by another.").
- CR 120.3d: "Damage dealt to a creature by a source with wither and/or infect causes that source's
  controller to put that many -1/-1 counters on that creature."
- CR 614.16: "These replacement effects apply if the effect of a resolving spell or ability creates
  a token or puts a counter on a permanent, and they also apply if another replacement or
  prevention effect does so, even if the original event being modified wasn't itself an effect."
- CR 616.1f: "Once the chosen effect has been applied, this process is repeated (taking into
  account only replacement or prevention effects that would now be applicable) until there are no
  more left to apply."

## Step 0.5 — class census and rulings

Files: `<run root>/class-port-fix/` (query script `../class/query.sh`, the skill's query with
reminder text stripped, as the charter recorded). MTGJSON `5.3.0+20261021`, `jq-1.8.1`.
Bearing rulings, numbered, are in `class-port-fix/notes.txt`.

- **F1 class (printed "while" guard on a self prevention shield).**
  `bash ../class/query.sh 'If (combat )?damage would be dealt to [^.]*? while [^.]*?, prevent' '' '[]' '[]'`
  → `guarded_prevention_class.json`, count **8**: Bloatfly Swarm, Jared Carthalion, True Heir,
  Magma Pummeler, Oathsworn Knight, Outfitted Jouster, Polukranos, Unchained, Ugin's Conjurant,
  Undergrowth Champion. `masked_only`, `rejected_*` empty; a broadened regex
  (`(would be dealt damage|damage would be dealt to|would deal damage to)[^.]*? while …, prevent`)
  selects the same 8. 39 distinct rulings, all read; sha256 (`jq -cS . | shasum -a 256`)
  `98a1b51465ce5a91db8fbd70019523ed12966ca7a3229c9c036b1a4d8ac5f5a6`. Bearing (>30, so only these
  quoted):
  - G1 Undergrowth Champion: "If Undergrowth Champion is dealt damage from multiple sources at the
    same time (for example, if it's blocked by multiple creatures), its prevention effect will
    apply once to all that damage. You'll remove only one +1/+1 counter." → rows V1, V2.
  - G2 Oathsworn Knight: "If multiple sources would deal damage to Oathsworn Knight simultaneously
    (for example, multiple creatures blocking it), its last ability applies once. All of that damage
    is prevented, and only one counter is removed." → V1 (reading once). The "only one counter"
    half with two or more counters is the PR's disclosed known gap ("one counter per simultaneous
    batch for recipient-phrased effects (Undergrowth Champion, the Phantom cycle, Oathsworn
    Knight)"); the maintainer's witness uses one counter, so this plan does not claim it.
  - G4 Jared Carthalion: "If you're the monarch and damage would be dealt to Jared Carthalion at
    the same time combat damage is dealt to you, Jared Carthalion's last ability will prevent
    damage and put counters on it before the triggered ability that makes another player the
    monarch resolves." → preservation row V4 (existing test).
  - G6 the CR 615.12 rulings ("damage that can't be prevented"): out of scope, strict-failure tags
    stay (PR follow-up).
  Every member reads through the one guard ledger (`settled_damage_guard_reading`), so the carry
  covers all 8; Jared's and Jouster's guards (monarch, equipped) are combat-flippable only by a
  rider, Champion/Knight/Polukranos/Conjurant/Pummeler/Bloatfly by their own counter-removing rider.
- **F2 class (wither/infect damage to a creature, CR 120.3d).**
  `bash ../class/query.sh '\b(wither|infect)\b' 'Wither' '[]' '[]'` → `wither_infect_class.json`,
  count **104** (same with kw `Infect`; `keyword_missing` empty for both). 128 distinct rulings, all
  read in two batches; sha256 `38f03c2c78e710e76d2af2cf10af3c574cda6b79c8ac9242472b5efa0e047594`.
  Bearing:
  - W-R1 (21 cards) "Damage from a source with infect is damage in all respects. If the source with
    infect also has lifelink, damage dealt by that source also causes its controller to gain that
    much life. … Abilities that trigger on damage being dealt will trigger …" → W1/W2 assert one
    `DamageDealt` per leg, exactly once across the pause.
  - W-R2 Raven's Run "Since damage from a creature with wither is real damage, it follows all the
    other rules for damage. It can be prevented or redirected. When it's dealt, it will cause
    'Whenever [this creature] deals damage' and 'Whenever [this creature] is dealt damage'
    abilities to trigger." → same rows.
  - W-R4 "If the source of the damage is still on the battlefield at the time that damage is dealt,
    its characteristics are checked to see if it has wither. If the source has left the battlefield
    by then, its last existence on the battlefield is checked to see if it had wither." → W1
    variant B (the Ripper has been dealt lethal damage before its own leg is delivered).
  The F2 seam is every wither/infect source whose creature leg meets two order-material counter
  modifiers (CR 616.1) inside a damage batch: printed sources include Sickle Ripper, Necroskitter,
  Puncture Blast, Village Pillagers, Blight Sickle and Grafted Exoskeleton (granted), modifiers
  Winding Constrictor, Vizier of Remedies, Doubling Season (effect placements only, CR 614.16).

## Step 1 — skills applied

- `/add-replacement-effect` (prevention guard reading; parked replacement pass).
- `/add-interactive-effect` (a CR 616.1 prompt round trip that parks and resumes the batch, with a
  serialized parked record).
- `/card-test` (public-action entries; completion boundary `assert_batch_settled` /
  `assert_settled`).
- `/address-review` (one batch, one push, one reply).
- `/add-engine-variant`: not triggered — no enum variant is added; one struct field on an existing
  parked record and one on the pass.
- Protocol: `project-reference` "Protocol version changes" (one `bump game` record).

## Step 2 — analogous trace (hard gate)

**Traced feature: the pass's one-shot application ledger carried across a park**
(`CombatDamageReplacementPass::one_shot_applications`), the exact shape F1 needs for the readings:
- `crates/engine/src/game/replacement.rs` `CombatDamageReplacementPass` (field
  `one_shot_applications`) → `replace_combat_damage_batch` installs it as
  `DamageBatchLedger::applications` on entry (`std::mem::take(&mut pass.one_shot_applications)`)
  and moves it back on exit (`state.damage_batch_ledger.take().map(|l| l.applications)`) → consumer
  `consumed_replacement_applies_in_combat_batch` reads the ledger.
- `crates/engine/src/game/combat_damage.rs` `run_combat_damage_replacements` Paused arm parks
  `one_shot_applications: pass.one_shot_applications` in `PendingCombatDamageReplacement`;
  `resume_combat_damage_replacements` rebuilds the pass from it.
- `crates/engine/src/types/game_state.rs` `PendingCombatDamageReplacement::one_shot_applications`
  (`#[serde(default, skip_serializing_if = "Vec::is_empty")]`, "Absent from saves written before it
  existed").

The readings' own path today: `read_damage_guards` (replacement.rs) builds
`Vec<SettledDamageGuard>`; `replace_combat_damage_batch` installs a fresh read per entry (the F1
defect, `let settled_guards = read_damage_guards(state);`); consumer `settled_damage_guard_reading`
reads `ledger.settled_guards`; `pending_candidates_are_current` → `stored_candidate_applies`
revalidates a surfaced leg's candidates under that ledger on resume, which is where Champion is
dropped. Noncombat already carries the reading: `with_damage_batch` uses
`ability.context.settled_damage_guards` when non-empty, `PendingReplacement.settled_damage_guards`
travels with a parked portion, and `hold_riders_for_awaited_portion` reinstalls it.

**Second trace (F2): the post-loss stash** (`pending_damage_post_loss`), which #10016's
`pending_damage_post_counter` copies contract for contract:
`deal_damage::apply_damage_after_replacement` (stash + `DamageResult::NeedsChoice`) →
non-combat: `engine_replacement::handle_replacement_choice_inner` drains it
(`drain_pending_damage_post_loss`) "after the batch drain retires with no re-park and before outer
continuations"; combat: `combat_damage::run_phase_c_legs` → `PhaseCRunOutcome::PausedOnLoss` →
`PendingCombatLifelink::phase_c_remnant` → `complete_combat_phase_c_remnant` consumes the
combat-flagged stash; cleared by `elimination::retire_parked_resolution_work` and
`turns::enter_phase`. In this PR's two-phase batch: `deal_damage::run_damage_batch_phases` Phase C
`DamageResult::NeedsChoice` → `park_damage_batch(.., &replaced[index + 1..], ..)` →
`Effect::ResumeDamageBatch` carrier → `resolve_resume_damage_batch` → `DamageTail`.
#10016's additions on the same path: `git show 4ba131744b -- crates/engine/src/game/effects/deal_damage.rs
crates/engine/src/game/combat_damage.rs crates/engine/src/game/engine_replacement.rs
crates/engine/src/game/turns.rs crates/engine/src/game/elimination.rs` (`PendingDamagePostCounter`,
`wither_infect_post_counter_core`, `drain_pending_damage_post_counter`, `PausedOnCounter`).

## Step 3 — files read

`replacement.rs` (`read_damage_guards`, `settled_damage_guard_reading`, `settled_damage_guards`,
`with_damage_batch`, `with_settled_damage_guards`, `replace_combat_damage_batch`,
`reconcile_parked_combat_leg`, `lift_departed_combat_leg_choice`,
`first_chooser_of_unstarted_combat_leg`, `CombatDamageReplacementPass`, `settled_survivors`,
`settle_combat_damage_replacement`, `drain_combat_damage_continuation`, `stash_damage_substitute`,
`pending_candidates_are_current`, `replacement_ordering_is_material`, the Prevention::All tally
branch, `batched_combat_all_shield`, `shield_has_per_event_execute_followup`, the
`record_proposed_events` unit test); `combat_damage.rs` (`CombatDamageBatch`, Phase C runner,
`complete_combat_phase_c_remnant`, `apply_combat_damage`, `run_combat_damage_replacements`,
`resume_combat_damage_replacements`, `resume_pending_combat_lifelink`,
`apply_combat_damage_survivors`, `drain_combat_lifelink`); `types/game_state.rs`
(`PendingCombatDamageReplacement`, `DamageBatchLedger`); `types/ability.rs` (`SettledDamageGuard`,
`DamageGuardReading`); `effects/deal_damage.rs` (`DamageTail`, `run_damage_batch(_phases)`,
`park_damage_batch`, `hold_riders_for_awaited_portion`, `return_answered_portion_to_batch`,
`resolve_resume_damage_batch`, `resolve_with_splice`, `resolve_post_replacement`,
`resolve_all_portions`, `in_apnap_order`); `effects/prevent_damage.rs` unit test at the
`CombatDamageReplacementPass::new` call; `types/game_state_size.rs`; the merged-tree
`engine_replacement.rs` funnel (post-loss and post-counter drains before
`active_ability_continuation`); `tests/integration/unpreventable_damage_prevention_effects.rs`
(helpers `undergrowth_champion`, `fight`, `fight_answering`, `Combat`, `preferring`, `offered`,
`assert_batch_settled`, `cast`, `answer_one`, `creature_with_keywords`, `scales_and_season`,
existing guard rows); `damage_substitution_replacements.rs` (`temper`, `cast_temper_at_one`,
private); `cr733_resolved_commands_p0.rs` (field-level census gate);
`backup/cz86-p2-full` commit `8b16c4a374` (D4 guard carry and N-3) and `phase-2-plan-r3.md`
§D4/§Commit A.

Merge facts (`git merge-tree --write-tree --name-only f995714a3c upstream/main` → tree
`cab4eb484b…`): one conflicted file, `crates/engine/src/game/effects/deal_damage.rs`, two hunks,
both in the `use` block (`crate::types::game_state::{…}` and `crate::types::proposed_event` /
`crate::types::resolution::ChildStackDepth`). Every other file auto-merges (ability_scan,
casting_costs, combat_damage, effects/mod, elimination, engine_payment_choices,
engine_replacement, replacement, resolution_prompt, triggers, types/game_state, phase-ast ability,
phase-parser oracle_replacement). The branch adds no call to a function #10016 changed the
signature of (`add_counter_with_replacement`, `ProposedEvent::AddCounter`, `CounterPlacement`,
player-counter adders): `git diff 206116c5cd f995714a3c -- crates/ | grep '^+.*add_counter_with_replacement\|AddCounter {\|CounterPlacement {'`
returns nothing. Whether the merged tree compiles is unprobed (E5).

## Step 3.5 — probes

The planner ran no build or test (read-only). Unprobed assertions, each with the design decision it
could change, become executor E-rows:

- **E1 (F1 red at base).** T-F1 fails at `f995714a3c` because the second answer's revalidation
  drops Champion (doubled Bears damage kills it). If it passes at base, the defect is not reached:
  stop and return. Changes: whether the witness is the closing witness.
- **E2 (N-3 red at base).** The backup recorded red at its base (damage 1 instead of 3). Re-measure
  at `f995714a3c`.
- **E3 (Temper's counters and Doubling Season after #10016).** N-3's first prompt offers
  {Scales A, Scales B, Doubling Season}: Temper's rider resolves its put-counter as an effect
  (`effects/counters.rs` stamps `CounterCause::ResolvingEffect` on resolution) and CR 614.16 admits
  a prevention effect's counters. If the merged engine offers only the two Scales, that is a
  #10016 defect: stop and return (do not fix).
- **E4 (W1/W2 reach).** At W1's prompt `state.pending_damage_post_counter` is `Some` with
  `is_combat == false` and a `ResumeDamageBatch` carrier is on the resolution stack (variant A: one
  replaced portion; variant B: none). At W2's third prompt the combat record's `phase_c_remnant`
  is `Some` and the counter stash `is_combat == true`. If either pause is not reached (the engine
  auto-orders, or the leg is not wither), report the fixture, not a result.
- **E5 (merged tree).** After the import union the workspace compiles with no other edit; the CR733
  matrix test passes with main's fixture; the post-replacement continuation census `--check`
  passes; `GameState` stays under 16,384. Any further compiler-forced edit is a standing-class
  path; a semantic conflict returns to planning.
- **E6 (mutation, F2 tail owners).** Temporarily (never committed): (a) skip the funnel's
  `drain_pending_damage_post_counter` call → W1 must go red (paused leg's `DamageDealt` missing);
  (b) make `complete_combat_phase_c_remnant` skip the counter stash → W2 must go red. (c) For F1,
  restore the fresh `read_damage_guards(state)` at entry → T-F1 and N-3 go red. Restore and
  re-run green.

## 4. Architecture

### Pattern Coverage

- F1: the 8 printed guarded-prevention cards of §0.5 (verbatim texts above for Champion; Jared:
  "If damage would be dealt to Jared Carthalion while you're the monarch, prevent that damage and
  put that many +1/+1 counters on it."; Oathsworn Knight: "If damage would be dealt to this creature
  while it has a +1/+1 counter on it, prevent that damage and remove a +1/+1 counter from it.";
  Polukranos: "If damage would be dealt to Polukranos while it has a +1/+1 counter on it, prevent
  that damage and remove that many +1/+1 counters from it."). Every one reads its guard through the
  one combat-batch ledger, and every combat park re-enters through
  `resume_combat_damage_replacements` (ordering answer, substitute/rider continuation,
  departed-chooser lift), so carrying the reading in the pass covers the class and every park
  kind.
- F2: the 104 wither/infect cards × the counter modifiers that make a placement order-material
  (Winding Constrictor `Plus`, Vizier of Remedies `Minus`, Doubling Season `Times` for effect
  placements; `replacement_ordering_is_material` treats Additive vs Subtractive and Additive vs
  Multiplicative as non-commuting). No card-specific code: the decision is ownership, and the
  witnesses use two printed sources (Sickle Ripper in a fight and in combat).

### Sizing

Units (one coherent behavior each):
- **U1 — the batch's guard reading survives a combat park (F1).** Registration surfaces:
  `CombatDamageReplacementPass::settled_guards` + `new(state, events)`; `replace_combat_damage_batch`
  install/restore; `PendingCombatDamageReplacement::settled_guards` (serialized);
  `combat_damage::{apply_combat_damage, run_combat_damage_replacements,
  resume_combat_damage_replacements}`; two compiler-forced unit-test calls; one protocol record.
  Discriminating test: **T-F1** (red at base).
- **U2 — main's #10016 merged with one owner per piece of a paused wither/infect leg (F2).**
  Registration surfaces: the `deal_damage.rs` import union (the only conflict); the
  `game_state_size.rs` re-measurement note. Discriminating tests: **W1** (noncombat two-phase
  batch) and **W2** (combat, inside a resumed batch), each with an E6 mutation.
Dependency edges: U2's merge is applied first (U1 is written on the merged tree); W2 depends on U1
(its first two prompts are T-F1's board).

Scope paths (phase-fit counting rule; protocol record and regenerated census excluded; files the
merge brings in unchanged from main are main's, not this change's):
1. `crates/engine/src/game/replacement.rs`
2. `crates/engine/src/game/combat_damage.rs`
3. `crates/engine/src/types/game_state.rs`
4. `crates/engine/src/game/effects/prevent_damage.rs` (compiler-forced unit test)
5. `crates/engine/src/game/effects/deal_damage.rs` (conflict resolution, imports only)
6. `crates/engine/src/types/game_state_size.rs` (measurement note)
7. `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs`
8. `crates/engine/tests/integration/damage_substitution_replacements.rs` (`temper`,
   `cast_temper_at_one` become `pub(super)`)
Excluded: `protocol/changes/<new uuid>.json`; `scripts/post-replacement-continuation-baseline.txt`
only if E5's `--check` reports drift (regenerated with `--write`).
**Count: 8 paths, 2 units. T1 and T2 do not fire.**

### Building Blocks

- `replacement::read_damage_guards` — the single reader of printed guards; called once per batch,
  at `CombatDamageReplacementPass::new`, exactly as `with_damage_batch` calls it once per
  noncombat batch.
- `DamageBatchLedger::settled_guards` + `settled_damage_guard_reading` — the single consumer; no
  new reader.
- `PendingCombatDamageReplacement` — the existing parked record (it already carries
  `one_shot_applications` the same way).
- `resume_pending_combat_lifelink` → `resume_combat_damage_replacements` — the single resume
  authority for every combat park (ordering answer, continuation, departure lift via
  `lift_departed_combat_leg_choice`, the priority-pass re-entry in `resolve_combat_damage`).
- F2: `pending_damage_post_counter` + `drain_pending_damage_post_counter` (non-combat),
  `PhaseCRunOutcome::PausedOnCounter` + `complete_combat_phase_c_remnant` (combat),
  `park_damage_batch` / `Effect::ResumeDamageBatch` / `DamageTail` (the PR's batch). No new helper.
- Tests: `fight_answering` (generalized to `fight_driving`), `preferring`, `offered`,
  `assert_batch_settled`, `undergrowth_champion`, `creature_with_keywords`, `cast`, `answer_one`,
  `plus_counters`, `damage_dealt_to`.

### Logic Placement

- **F1 owner:** `replacement::CombatDamageReplacementPass` owns the reading for its batch's whole
  life: created by `CombatDamageReplacementPass::new(state, events)` (reads once), installed into
  the ledger by `replace_combat_damage_batch` on every entry, moved back on every exit, parked by
  `combat_damage::run_combat_damage_replacements`' Paused arm into
  `PendingCombatDamageReplacement::settled_guards`, restored by
  `resume_combat_damage_replacements`. Delegating consumers unchanged:
  `settled_damage_guard_reading` (candidate discovery and the revalidation in
  `pending_candidates_are_current` / `stored_candidate_applies`),
  `first_chooser_of_unstarted_combat_leg` (its probe clone sees the installed ledger),
  `reconcile_parked_combat_leg` / `repropose_combat_damage_leg`. A rider's own damage still reads
  live: `drain_combat_damage_continuation` suspends the ledger around it (CR 700.1). A later batch
  (the regular step after first strike, CR 510.4) creates a new pass and reads afresh.
- **F2 owners (the decision, §4 "Tail ownership").** Three pieces, three owners, no new mechanism:
  1. *The paused leg's remainder* (deathtouch flag, `DamageDealt` + CR 120.1 record, excess,
     lifelink): non-combat → `state.pending_damage_post_counter`, drained only by
     `engine_replacement::handle_replacement_choice_inner`'s funnel
     (`drain_pending_damage_post_counter`), which runs before the funnel drains outer
     continuations; combat → the combat-flagged stash, consumed only by
     `complete_combat_phase_c_remnant` (the funnel skips combat stashes).
  2. *The batch's unapplied portions, held riders, recorded outcomes and `EffectResolved`*:
     non-combat two-phase batch → the `Effect::ResumeDamageBatch` carrier parked by
     `park_damage_batch(.., &replaced[index + 1..], ..)` (the paused leg is never in it); combat →
     `PendingCombatLifelink::phase_c_remnant` (`remaining[index + 1..]`), then the lifelink tail and
     Phase D in `drain_combat_lifelink`.
  3. *The instruction's later instructions*: `DamageTail` from `park_damage_batch` (`Stashed`:
     parked beneath the carrier; `Caller` when the pause raised frames). Single-recipient and
     DamageAll paths (`resolve_with_splice`, `resolve_all_portions`) keep their
     `stash_remaining_damage_chain` + `DamageTail::Stashed` (exercised by #10016's Puncture Blast and
     Village Pillagers rows on the merged head).
  Order on the answer (sync order, #10016's contract "each damage target completes before later
  targets resume"): remainder (1) → carrier (2) → tail (3). This is the same composition the PR's
  batch already has with `pending_damage_post_loss`; the merge adds a second stash of the same
  shape and nothing else.

### Rust Idioms

- `Vec<SettledDamageGuard>` moved with `std::mem::take` and `if let Some(ledger) =
  state.damage_batch_ledger.take()` on exit (no clone, no `unwrap_or_default` that would silently
  drop a reading); the struct keeps `#[derive(Debug, Default)]`.
- `#[serde(default, skip_serializing_if = "Vec::is_empty")]` on the parked field, matching
  `one_shot_applications`.
- No bool, no new enum; `DamageGuardReading` stays the typed reading.

### Nom Compliance

No file under `crates/phase-parser/src/parser/` changes in this batch (the merge brings main's
`oracle_replacement.rs` unchanged). Not applicable.

### Extension vs Creation

Extension only: one field on the pass, one on the parked record, mirroring
`one_shot_applications`. F2 creates nothing: it composes #10016's stash with the PR's carrier.

### Analogous Trace

Traced `CombatDamageReplacementPass::one_shot_applications` through `replacement.rs`
(`CombatDamageReplacementPass`, `replace_combat_damage_batch`,
`consumed_replacement_applies_in_combat_batch`) → `combat_damage.rs`
(`run_combat_damage_replacements`, `resume_combat_damage_replacements`) → `types/game_state.rs`
(`PendingCombatDamageReplacement`) → tests (`unpreventable_damage_prevention_effects.rs`
park/reload rows). F2: traced `pending_damage_post_loss` through `effects/deal_damage.rs`
(`apply_damage_after_replacement`, `drain_pending_damage_post_loss`) → `engine_replacement.rs`
(`handle_replacement_choice_inner` funnel) → `combat_damage.rs` (`run_phase_c_legs`,
`complete_combat_phase_c_remnant`) → `elimination.rs` / `turns.rs` (abandonment), and #10016's
`pending_damage_post_counter` along the same path.

### Variant Discoverability

No enum variant is added. `data/engine-inventory.json` not consulted for that reason; the two new
fields are struct fields on types that already exist.

### Identity / Provenance Contract (F1)

| Item | Value |
|---|---|
| Rules concept | A printed "while" guard is read once for one simultaneous batch (CR 510.2, CR 608.2f + CR 700.1; ruling G1) |
| Authority type | `SettledDamageGuard { source: ObjectIncarnationRef, condition: ReplacementCondition, reading: DamageGuardReading }` |
| Binding time | `CombatDamageReplacementPass::new`, after Phase A's gate and before any portion is proposed |
| Live vs latched | Latched for the batch; a rider's own damage reads live (ledger suspended); a new batch reads afresh |
| Storage | `pass.settled_guards` → `DamageBatchLedger::settled_guards` while the pass runs → `PendingCombatDamageReplacement::settled_guards` while parked (serialized) |
| Consumer | `settled_damage_guard_reading` (discovery, revalidation, chooser probe) |
| Invalidation | Dropped with the pass when Phase B completes (`apply_combat_damage_survivors` consumes the pass); never written to `GameState` outside the ledger/parked record |
| Hostile fixture | N-3 (a false reading stays false when a rider adds counters before the park), S-1 (a later batch reads afresh), V4 (Jared monarchy, existing) |

## 5. Verification Matrix

Completion boundary for combat rows: `assert_batch_settled` (no `pending_replacement`, empty
resolution stack, `Priority`, empty stack, `Phase::EndCombat`). Noncombat rows: `assert_settled`
(this file's helper) plus an empty stack.

| Row | Claim (CR) | Seam | Production entry → test | Revert-failing assertion | Positive reach guard | Siblings / hostile | Coverage |
|---|---|---|---|---|---|---|---|
| **V1 T-F1** `undergrowth_champion_keeps_its_reading_across_both_orders_of_a_double_block` | A combat batch parked on an order prompt keeps the guard reading it opened with, in memory and after a save/reload between the answers (CR 510.2, CR 608.2f + CR 700.1, G1) | `CombatDamageReplacementPass::new` / `replace_combat_damage_batch` / parked record | `fight_driving`: P0 attacks with Champion (1 counter); P1 blocks with Angrath's Marauders and Grizzly Bears; answers via the public `ChooseReplacement`; in the reload variants the state is serialized and deserialized at prompt 2 (assert `pending_combat_lifelink.replacement.is_some()` first) | Champion `zone == Battlefield`, `damage_marked == 0`, `damage_dealt_to(champion)` empty (base: Champion dies to doubled damage after the second answer) | 2 prompts, choosers `[P0, P0]`, each `offered == {Champion, Marauders}`; `plus_counters == 0` (the one counter was removed); the blockers took Champion's damage (`DamageDealt` from Champion present) | Answer orders: Champion first at both prompts; Marauders first at both (the doubled event still meets Champion's shield, CR 616.1f). × {in memory, reload at prompt 2} = 4 runs | none (no fence change) |
| **V2 N-3** `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park` (from backup, adapted) | A false reading stays false across a park even after a rider makes the live guard true (CR 608.2f + CR 700.1, CR 611.3a) | same | `fight_driving`: counterless Champion (P0) blocked by two Grizzly Bears; P0 cast Temper at X = 1 on it; P0 controls two Hardened Scales and Doubling Season; Temper's counter meets three effects → two prompts; reload variant at prompt 2 | Champion `damage_marked == 3` (base 1); no prompt offers Champion | `prevented(events) == 1`; counters 6 (Scales, Scales, Season) / 5 (Scales, Season, Scales); first prompt offers {Scales A, Scales B, Season} (E3) | × {in memory, reload} × {two orders} | none |
| **V3 S-1** `a_later_combat_damage_step_reads_the_guard_afresh` | Each combat damage step is its own batch, so the reading is not carried past it (CR 510.4, CR 510.2) | `apply_combat_damage` creates a pass per sub-step | `fight`: P0's Champion (1 counter) attacks, P1's Fencing Ace blocks | `damage_dealt_to(champion) == [1]`, `damage_marked == 1` (preservation: green at base and after; red only if the reading leaks into the regular step) | first-strike portion prevented: `plus_counters == 0`; Ace in graveyard | — | none |
| **V4** existing `undergrowth_champion_double_blocked_reads_its_guard_once`, `jared_carthalion_reads_the_monarchy_before_combat_damage_moves_it`, every noncombat guard row (`a_paused_batch_keeps_its_guard_reading_in_every_answer_order`, `the_batch_reading_survives_serialization_at_the_pause`, `a_paused_fight_keeps_its_guard_reading`, …) | unchanged | — | as today | green | — | sibling sites with no change: noncombat batches already carry (`with_damage_batch`, `PendingReplacement.settled_damage_guards`) | — |
| **V5 W1** `a_wither_fight_leg_paused_on_its_counters_completes_once` | A wither leg of a two-phase damage batch that pauses for its -1/-1 counter order completes exactly once, before the batch's later portion, and the batch completes once (CR 120.3d + CR 702.80a, CR 616.1, CR 701.14a, CR 608.2c) | merged `run_damage_batch_phases` Phase C + `drain_pending_damage_post_counter` + `ResumeDamageBatch` | `cast` Pit Fight (P0). Variant A: P0's Darksteel Colossus (keywords Trample, Indestructible) fights P1's Sickle Ripper (keyword Wither), P0 controls Winding Constrictor + Doubling Season → P0 orders. Variant B: P0's Sickle Ripper fights P1's Colossus, P1 controls both → P1 orders. Answer with `answer_one`; reload variant at the prompt | `-1/-1` on Colossus: Constrictor first 6, Season first 5; exactly one `DamageDealt` Ripper→Colossus amount 2 (emitted after the answer) and one Colossus→Ripper amount 11; exactly one `EffectResolved { kind: Fight }`, after both | E4 markers at the prompt (stash `Some`, `!is_combat`; carrier present, replaced len 1 in A / 0 in B); Ripper in graveyard; Pit Fight in graveyard | Parity sibling: Constrictor only → no prompt, 3 counters, same event shape (derived 2 + 1). E6(a) mutation red | none |
| **V6 W2** `a_wither_leg_pauses_on_its_counters_inside_a_resumed_combat_batch` | A combat wither leg that pauses on its counter order after the batch was parked and resumed in its replacement pass completes once, and the batch completes once (CR 510.2, CR 120.3d, CR 616.1) | merged Phase C `PausedOnCounter` + `complete_combat_phase_c_remnant`, after U1's resume | `fight_driving`: T-F1's board plus P0's Sickle Ripper attacking, blocked by P1's Winding Constrictor; P1 also controls Vizier of Remedies. Prompts 1–2 answered Champion; prompt 3 (P1) both orders; reload variant at prompt 2 and at prompt 3 | Constrictor has exactly 2 -1/-1 counters (Constrictor first: 3 − 1; Vizier first: 1 + 1) and is on the battlefield; exactly one `DamageDealt` Ripper→Constrictor amount 2, emitted after prompt 3's answer; Champion as in V1 | choosers `[P0, P0, P1]`; prompt 3 `offered == {Constrictor, Vizier}`; E4 combat markers at prompt 3; Ripper in graveyard (Constrictor's 2, doubled by Marauders to 4, against toughness 1) | E6(b) mutation red | none |
| **V7** both sides' suites on the merged head | unchanged behavior | merge | #10016: `darksteel_angel_cant_have_counters`, `wither_counter_pause_resume`, `soul_scar_mage_damage_to_counters` (incl. #10016's Doubling Season row), `grub_storied_matriarch_769`, `issue_1337_tekuthal_proliferate_twice`, `serpent_society_ward_poison_cost`, `std_counters_grammar_axes`, `mauhur_swarming_of_moria`, `one_ring_burden_upkeep_lethal`, `cr733_resolved_commands_p0`; #9670: the eight suites in `task.md`; this PR: `damage_substitution_replacements`, `unpreventable_damage_prevention_effects`, `ironscale_hydra_combat_prevention`, `farideh_fireball_die_branch_selection`, `combat_lifelink_replacement_ordering` | green | — | A PR row using Doubling Season on a rider's or substitute's counters (Phytohydra, Gatta and Luzzu, Ironscale Hydra, Temper) that flips under #10016's CounterCause gate is derived from CR 614.16 (those placements are a replacement/prevention effect's, so eligible): a flip is a defect, stop and return, never re-baseline | — |

Hostile-fixture notes: V1's first production branch is `pending_candidates_are_current` on resume
(`Some(_) if … !pending_candidates_are_current` → re-propose); V2's is
`settled_damage_guard_reading` returning `Fails` during discovery of the second Bears portion;
V5's are `park_damage_batch` with non-empty vs empty `replaced` and the funnel's
`!stashed.is_combat` gate; V6's is `take_if(|s| s.is_combat)` in
`complete_combat_phase_c_remnant`. Empty/decline path: not applicable (no optional replacement);
multi-authority: V1 (two shields on one event, one guarded), V6 (two counter modifiers, the
chooser is the counters' recipient's controller). No Oracle text is newly accepted; coverage
unchanged.

## 6. Reference Readings

| Row | Derived value (card + CR) | Reference measured by | Status |
|---|---|---|---|
| V3 (fresh read per step) | Fencing Ace's first-strike 1 is prevented (guard holds, one counter removed); the regular-step batch opens with no counter, so its 1 is dealt (CR 510.4, CR 510.2, G1) → `[1]` | E-run at `f995714a3c` and the candidate | derived; measure both |
| V4 (preservation) | as in each existing test's own derivation | unchanged suites | the plan asserts no new value |
| V5 parity sibling | Constrictor alone: 2 + 1 = 3 counters, no prompt (CR 614.1, CR 616.1 needs two) | E-run on the merged head | derived; a mismatch is a defect → stop and return |
| V5 orders | Constrictor first (2+1)×2 = 6; Season first 2×2+1 = 5 (CR 616.1f; CR 614.16 admits the fight's effect placement) | merged head | derived |
| V6 orders | Constrictor then Vizier 3−1 = 2; Vizier then Constrictor 1+1 = 2 | merged head | derived |
| V2 counts | X = 1 → one counter; Scales, Scales, Season ((1+1)+1)×2 = 6; Scales, Season, Scales (1+1)×2+1 = 5; Champion dealt 1 + 2 = 3 | merged head | derived (backup measured the same at its base) |

No row copies another route's reading without its own derivation.

## 7. Step-by-step plan

Commits on top of `f995714a3c` (no rebase): **M** (merge), **C1** (F1), **C2** (F2 witnesses). If
`upstream/main` has moved past `4ba131744b` when the merge starts, merge the current tip instead,
re-run `git merge-tree --write-tree --name-only f995714a3c upstream/main`, and return to planning
if any conflict other than the `deal_damage.rs` imports appears.

### M — merge main (orchestrator starts `git merge --no-commit --no-ff upstream/main`; executor resolves)

- M1 `crates/engine/src/game/effects/deal_damage.rs`: union the two import hunks:
  `crate::types::game_state::{DamageBatchLedger, DamageRecord, DistributionScope, EventLosses,
  GameState, HeldBatchRider, PendingContinuation, PendingDamagePostCounter, ResidentDrainPolicy,
  RiderTiming, TriggerSourceRead, WaitingFor}`; `crate::types::proposed_event::{CounterCause,
  ProposedEvent}`; keep `crate::types::resolution::ChildStackDepth`. No body edit.
- M2 `crates/engine/src/types/game_state_size.rs`: re-measure with the module's
  `-Zprint-type-sizes` recipe and append the branch's usual note ("`GameState` re-measured on
  <target> (<toolchain>) at N B when the two-phase damage batch merged main through `4ba13174`:
  main's boxed `pending_damage_post_counter` (#10016) unioned with the branch's fields. That is
  under the unchanged 16,384 ceiling."). The ceiling does not change.
- M3 Run E5 (compile, CR733 test, census `--check`; regenerate the census baseline with `--write`
  only for rows the merge itself adds, and list them).
- Commit message: `Merge upstream main (#10016) into engine/unpreventable-damage-riders`, body
  naming the one conflict, its resolution, and the size note.

### C1 — F1: the batch's guard reading survives a combat park

1. `crates/engine/src/game/replacement.rs`
   - `CombatDamageReplacementPass`: add
     ```rust
     /// CR 608.2f + CR 700.1: the printed "while" guards, read once when the pass is created and
     /// installed as `DamageBatchLedger::settled_guards` on every entry, so a batch that parks
     /// resumes under the reading it opened with. Undergrowth Champion ruling: "its prevention
     /// effect will apply once to all that damage."
     pub(crate) settled_guards: Vec<SettledDamageGuard>,
     ```
   - `CombatDamageReplacementPass::new(state: &GameState, events: impl IntoIterator<Item =
     ProposedEvent>) -> Self`: set `settled_guards: read_damage_guards(state)`; doc "A pass over
     `events`, every leg not yet proposed, under the guard reading of `state` as the batch opens
     (CR 608.2f)."
   - `replace_combat_damage_batch`: replace `let settled_guards = read_damage_guards(state);` and
     the ledger's `settled_guards,` with `settled_guards: std::mem::take(&mut pass.settled_guards),`;
     rewrite the entry comment ("the one-shot applications and the printed guards' reading travel
     with the pass exactly like the tally, so a parked batch resumes with the applications its
     earlier portions recorded and under the reading it opened with"); on exit replace the
     `pass.one_shot_applications = state.damage_batch_ledger.take().map(..).unwrap_or_default();`
     with `if let Some(ledger) = state.damage_batch_ledger.take() { pass.one_shot_applications =
     ledger.applications; pass.settled_guards = ledger.settled_guards; }`; rewrite the doc's last
     paragraph ("installed on every entry with the pass's one-shot applications and the guard
     reading the pass took when it was created (CR 608.2f), and both move back into the pass on
     every exit").
   - Unit test at the `record_proposed_events` site: `CombatDamageReplacementPass::new(&state,
     [damage.clone()])` (compiler-forced).
2. `crates/engine/src/types/game_state.rs` `PendingCombatDamageReplacement`: add
   ```rust
   /// CR 608.2f + CR 700.1: what the printed "while" guards read when the batch opened. Every
   /// portion of the batch meets that one reading, including the portions proposed after the
   /// pause (Undergrowth Champion ruling: "its prevention effect will apply once to all that
   /// damage"). Absent from saves written before it existed.
   #[serde(default, skip_serializing_if = "Vec::is_empty")]
   pub settled_guards: Vec<crate::types::ability::SettledDamageGuard>,
   ```
3. `crates/engine/src/game/combat_damage.rs`: `apply_combat_damage` builds the pass with
   `replacement::CombatDamageReplacementPass::new(state, proposed_events)` under the comment
   `// CR 608.2f + CR 700.1: the pass reads the printed guards once, as the batch opens.`;
   `run_combat_damage_replacements`' Paused arm parks `settled_guards: pass.settled_guards`;
   `resume_combat_damage_replacements` rebuilds the pass with `settled_guards:
   parked.settled_guards`. Extend the Paused arm's comment by one clause naming the reading
   (CR 608.2f).
4. `crates/engine/src/game/effects/prevent_damage.rs` unit test: `CombatDamageReplacementPass::new(&state,
   [proposed])` (compiler-forced).
5. Protocol: `node scripts/protocol-versions.mjs bump game --reason "A combat damage batch parked on
   a replacement choice carries the printed damage guards' reading it opened with
   (PendingCombatDamageReplacement.settled_guards). An older reader drops it and rereads the guards
   when the batch resumes; a newer reader of an older save resumes with no settled reading, so the
   remaining portions read the guards live."`, then `generate`, `check`, and
   `node scripts/check-protocol-version.mjs --base upstream/main`. Keep every existing record.
6. Tests in `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs`, in the
   guard section after `undergrowth_champion_double_blocked_reads_its_guard_once`:
   - Helpers: add `pub(super) fn fight_driving(runner, attackers, blocks, answer: impl
     FnMut(&mut GameRunner) -> GameAction) -> Combat` holding today's `fight_answering` body;
     `fight_answering` becomes `fight_driving(runner, attackers, blocks, |runner|
     answer(runner.state()))`. Add `pub(super) choosers: Vec<PlayerId>` to `Combat`, pushed from
     `WaitingFor::ReplacementChoice { player, .. }` beside `prompts`. Add `const
     ANGRATHS_MARAUDERS` (verbatim) and `const FENCING_ACE: &str = "Double strike"` (bare keyword
     line, the harness's keyword-hint idiom; doc comment quotes the full printed text).
   - V1 T-F1, V2 N-3 (port the backup test; make `temper` / `cast_temper_at_one` in
     `damage_substitution_replacements.rs` `pub(super)`; its doc as the backup's, with the
     fails-at-base value re-measured in E2), V3 S-1. Each doc comment quotes the Oracle text and
     cites CR 510.2 / CR 608.2f + CR 700.1 / CR 510.4 as in §5.
   - Update the module doc's guard paragraph by one sentence: the reading "survives an order
     prompt inside combat damage, and a save and load at it".
7. E1/E2 at `f995714a3c` (scratch worktree with an isolated target dir, pruned after), E6(c).
8. Commit: `fix(engine): a parked combat damage batch keeps the guard reading it opened with`.

### C2 — F2 witnesses: a wither leg paused on its counters has one owner for each piece

1. Same test file, new section after the guard section, headed by a comment: "A wither or infect
   leg whose -1/-1 counters meet two counter modifiers pauses inside the batch (CR 120.3d, CR
   616.1). Its remainder (the damage event and record) is the counter stash's, the batch's rest is
   the parked batch's, and the instruction's later instructions are the tail's." Add consts
   `SICKLE_RIPPER` (bare `"Wither"` line plus the doc-quoted printed text, as #10016's suite does),
   `WINDING_CONSTRICTOR`, `VIZIER_OF_REMEDIES`, `DARKSTEEL_COLOSSUS` (verbatim; keyword hints via
   `creature_with_keywords`), and `fn minus_counters(runner, id) -> u32`.
2. V5 W1 (two variants × {in memory, reload at the prompt} × {two orders} + parity sibling) and
   V6 W2 (both prompt-3 orders × {in memory, reload at prompts 2 and 3}).
3. E4, E6(a), E6(b).
4. Commit: `test(engine): a wither leg paused on its counters inside a damage batch completes once`.

### Verification (risk-scaled: shared state machine and a main merge → full Rust evidence)

`cargo fmt --all`; then, via Tilt when it watches this worktree (else directly, isolated target
dir): workspace `clippy --all-targets -D warnings`; `phase-engine` all targets at the default test
stack; `phase-parser`; `phase-server`; `phase-ai --lib`; wasm32 check;
`scripts/check-engine-authorities.sh`; the Kathril stack test at 16 MiB
(`kathril_reaches_matching_counter_and_tail_past_false_earlier_gates`); protocol `generate` /
`check` / `check-protocol-version --base upstream/main`; Gate A (AI-CONTRIBUTOR §6) with base =
the merged main commit. Optional before push (network): the CI parse-diff recipe locally (base
coverage from `https://data.phase-rs.dev/parse-baselines/coverage-data-$(./scripts/engine-source-hash.sh <main sha>).json`,
head `data/coverage-data.json` from `cargo coverage`, `cargo run --profile tool --features cli
--bin coverage-parse-diff -- base.json data/coverage-data.json --markdown parse-diff.md --json
parse-diff.json`); the CI sticky is authoritative either way.

Final review (address-review §4): fresh-agent `$review-engine-impl` in PR Review Mode over
`git diff <merged main> HEAD`, given the review text, this disposition table, `git diff
f995714a3c HEAD`, the drafted title/body, every earlier maintainer review (22 requested changes,
superseded by review 5911461092), and asked to state per finding whether it is cleared.

## 8. PR text (drafted before the commit review; applied right after the push)

- **Title:** unchanged.
- **New top section** "Review of the port (round 20)", replacing nothing above it:
  - "A parked combat damage batch keeps the guard reading it opened with. The pass reads the
    printed guards once, when the batch opens, and carries the reading through every park and a
    save and load (`PendingCombatDamageReplacement.settled_guards`, protocol record `<uuid>`). With
    Undergrowth Champion (one counter) blocked by Angrath's Marauders and Grizzly Bears, both
    portions are prevented, one counter is removed and Champion survives, in either answer order
    and after a reload between the answers (CR 510.2, CR 608.2f; the ruling: 'its prevention
    effect will apply once to all that damage')."
  - "Merged main through `<sha>` (#10016). The one conflict was the `deal_damage.rs` imports. A
    wither or infect leg that pauses for its -1/-1 counter order inside a damage batch has one
    owner per piece: its damage event and record belong to the counter stash
    (`pending_damage_post_counter`, drained before the batch resumes; in combat, the Phase C
    remnant), the batch's remaining portions and completion to the parked batch
    (`ResumeDamageBatch`; in combat, `phase_c_remnant`), and the spell's later instructions to
    `DamageTail`. Witnesses: Sickle Ripper fighting Darksteel Colossus under Winding Constrictor
    and Doubling Season (both orders, either side, with a reload), and Sickle Ripper blocked by
    Winding Constrictor under Vizier of Remedies after the Champion prompts above."
- **Claimed parse impact:** rewritten from the CI sticky for the pushed head (it must name that
  head). Replace the `4a00656a8` archive sentence and the "pending" sentences with the sticky's
  card count and signature count and a link. Reconcile card by card: every card in the sticky
  appears under one of the bullets with its signature change; every claimed card the sticky does
  not list is removed from the claim. Expected (to confirm, not to assert): the seven substitution
  cards (`shield_kind` Prevention{All} → None, no `ShieldKind::Substitution` anywhere), the two
  population riders, the four counter guards, Jared and Jouster's guards, and the two fail-closed
  cards; this batch changes no parser code and #10016's parser changes are main's, so a card
  outside that set is investigated before the body is applied (a parse change from the
  `oracle_replacement.rs` auto-merge would be a real finding). If the sticky still shows
  `ShieldKind::Substitution`, the baseline is wrong for the head: report it, do not edit the claim
  to match.
- **Files changed:** add `crates/engine/src/game/effects/prevent_damage.rs` (unit test) and the
  new protocol record; `game_state_size.rs` and `combat_damage.rs` are already listed.
- **CR references:** add CR 120.3d and CR 510.4 (cited by the new tests); 608.2f, 700.1, 510.2,
  616.1 already listed.
- **Verification:** a new head block for the pushed head (the §7 list with actual counts), "Both
  sides' suites pass" extended with #10016's list, the new witnesses with their fails-at-base
  results (T-F1 and N-3 at `f995714a3c`; W1/W2 mutation-checked), `Gate A PASS head=… base=…`,
  `Final review-impl PASS head=…` (or UNRESOLVED per §4). Keep the LLM section as it is.
- **Validation Failures / known gaps:** unchanged unless the human decides otherwise on the
  residual in the report (a wither leg's drained lifelink runs outside the batch's recorded
  outcomes, as the post-loss stash already does).

## 9. Reply (address-review §6), after the push

```markdown
Addressed <review URL> at <pushed head>.

- F1 — fixed in <C1>. The combat pass reads the printed guards once, when the batch opens, and carries the reading through the park (`CombatDamageReplacementPass.settled_guards`, `PendingCombatDamageReplacement.settled_guards`). Evidence: `undergrowth_champion_keeps_its_reading_across_both_orders_of_a_double_block` (fails at f995714a3c: Champion dies to the doubled damage after the second answer), in both answer orders and with a save/reload between the answers; `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park` (a false reading stays false, fails at f995714a3c). Siblings: noncombat batches already carry the reading (`with_damage_batch`, `PendingReplacement.settled_damage_guards`); a later combat damage step reads afresh (`a_later_combat_damage_step_reads_the_guard_afresh`).
- F2 — fixed in <M> (main through <sha>) and <C2>. One owner per piece of a paused wither/infect leg: the counter stash owns its event and record, the parked batch its rest, `DamageTail` the spell's later instructions. Witnesses: `a_wither_fight_leg_paused_on_its_counters_completes_once`, `a_wither_leg_pauses_on_its_counters_inside_a_resumed_combat_batch` (each mutation-checked), and #10016's suites green on the merged head. Parse sticky for <pushed head>: <link>; the description's changed-card set is reconciled with it.
```

## 10. For the orchestrator / a human

- Residual (no fix planned; decide whether to disclose): in a parked two-phase noncombat batch, a
  paused wither/infect leg's lifelink gain is drained by the funnel outside the batch's CR 104.4a
  recording scope, so a win or loss that gain causes settles at once rather than with the event's
  other outcomes. The post-loss stash already behaves this way; reaching it needs a lifelink
  wither source (Sickle Ripper with Basilisk Collar) whose gain ends the game during a counter
  pause. Unprobed.
- #10016's combat `PausedOnCounter` path has no witness on main (its combat rows no longer pause
  once Doubling Season is ineligible in combat); W2 is the first. If W2 shows a defect in that
  path, it is main's code: report it rather than fix it here (the maintainer offered help).
- G1's "remove only one counter" with two or more counters stays the PR's disclosed known gap.
- The local parse diff needs network for the base coverage; skip it if not allowed and rely on
  the sticky.
