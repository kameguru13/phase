Addressed https://github.com/phase-rs/phase/pull/10639#pullrequestreview-5952427921 at c2869d4263.

- **F1** is fixed in b78d783d18. The combat pass reads the printed guards once, when the combat damage step's batch opens, and carries that reading through the park (`CombatDamageReplacementPass.settled_guards`, `PendingCombatDamageReplacement.settled_guards`, game protocol record `6ca14e37`). An older save without the field resumes reading the guards live.
  - Evidence: `undergrowth_champion_keeps_its_reading_across_both_orders_of_a_double_block` reaches both ordering prompts through the public `ChooseReplacement`. It runs in both answer orders, in memory and with a save/reload between the answers. It asserts Champion stays alive with no damage marked, one counter removed, and both blockers' damage prevented. At f995714a3c it fails: Champion dies to the doubled damage after the second answer.
  - `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park`: a false reading stays false even after a rider makes the live guard true. It also fails at f995714a3c.
  - Siblings: noncombat batches already carry the reading (`with_damage_batch`, `PendingReplacement.settled_damage_guards`). A later combat damage step reads afresh (`a_later_combat_damage_step_reads_the_guard_afresh`).
- **F2**: main is merged through f99eb6336f in 0cd9cff869, with witnesses in b78d783d18 and c2869d4263. The only conflict was the `deal_damage.rs` import block, resolved as the union of both sides; no body edit was needed. A paused wither/infect leg has one owner per piece:
  - the counter stash owns the leg's event and record;
  - the parked batch owns the rest of the batch;
  - `DamageTail` owns the spell's later instructions.
  A win or loss that the leg's lifelink causes while the batch is parked is recorded on the parked batch and settles after its remaining damage. Witnesses, each checked by a mutation that turns it red:
  - `a_wither_fight_leg_paused_on_its_counters_completes_once`: both sides, both orders, reload; the paused leg's damage precedes the later leg's, with one record per leg.
  - `a_wither_fight_legs_lifelink_win_waits_for_the_rest_of_the_fight` and `…_without_a_pause`: Sickle Ripper with Basilisk Collar, plus Nefarious Lich and Laboratory Maniac.
  - `a_fights_later_instruction_runs_once_after_a_paused_wither_leg`: Prizefight's Treasure, created once, after the fight.
  - `a_wither_leg_pauses_on_its_counters_inside_a_resumed_combat_batch`: a combat wither leg pausing after the F1 park. It gets one lifelink gain and one damage record, both after the third answer.
  - #10016's suites are green on the merged head (`darksteel_angel_cant_have_counters`, `wither_counter_pause_resume`, `soul_scar_mage_damage_to_counters`, and others), as are this PR's batch suites. The engine suite, clippy, server, AI and wasm checks, Gates A/B/D and the protocol check all pass locally.
  - Parse comparison: the [sticky](https://github.com/phase-rs/phase/pull/10639#issuecomment-6621063155) names c2869d4263. The description's "Claimed parse impact" is reconciled against the full `parse-diff.json` artifact from that [run](https://github.com/phase-rs/phase/actions/runs/44300785386): 16 cards across 13 signatures, the same set the description lists, with `ShieldKind::Substitution` gone.
