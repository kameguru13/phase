Reviewed head: `f995714a3c3b3fd22676b919a945c64db15e44ef`. Re-review of the port, delta from `2fc5a9188e` (the head the [07:23Z hold](https://github.com/phase-rs/phase/pull/10639#issuecomment-6941579419) and the earlier approval named).

**Changes requested: all three conditions from the [13:51Z read](https://github.com/phase-rs/phase/pull/10639) are met in source and the #10433 resolution keeps both sides, but the parked combat pass rereads its prevention guards on resume, and this head has no parse comparison because it conflicts with `main` again.** The earlier approval at `2fc5a9188e` does not carry to this head.

## 🔴 Blocker

**F1 · HIGH — A parked combat batch rereads its prevention guards on resume, so an answered guard un-prevents the rest of the batch**

Introduced by the port. Every entry to the combat replacement pass installs `settled_guards: read_damage_guards(state)` fresh (`crates/engine/src/game/replacement.rs:15659`, whose comment says "the printed guards are read fresh for this entry"). Parking keeps only the leg progress, the tally and the one-shot applications (`crates/engine/src/game/combat_damage.rs:2181`), and the resume rebuilds `CombatDamageReplacementPass` without the original readings (`combat_damage.rs:2210`). Meanwhile a settled portion's continuation runs inline, "immediately afterward" (`replacement.rs:16156`), so the guard's own rider has already changed the board when the next portion is answered.

**Witness:** P0's Undergrowth Champion has one +1/+1 counter from landfall ("Landfall — Whenever a land you control enters, put a +1/+1 counter on this creature." / "If damage would be dealt to this creature while it has a +1/+1 counter on it, prevent that damage and remove a +1/+1 counter from this creature."). It attacks and P1 blocks with Angrath's Marauders ("If a source you control would deal damage to a permanent or player, it deals double that damage to that permanent or player instead.") and Grizzly Bears. Both portions offer Champion's prevention against Marauders' doubling. Choosing Champion for the first portion removes the counter; on the next answer the guard is reread as false, candidate revalidation drops Champion (`replacement.rs:15690`, `replacement.rs:8940`), and the doubled damage kills it. Champion's ruling: "If Undergrowth Champion is dealt damage from multiple sources at the same time (for example, if it's blocked by multiple creatures), its prevention effect will apply once to all that damage. You'll remove only one +1/+1 counter." CR 510.2: "Second, all combat damage that's been assigned is dealt simultaneously."

**Fix:** carry the batch's `SettledDamageGuard` readings in `CombatDamageReplacementPass` and `PendingCombatDamageReplacement`, and reinstall them on resume instead of calling `read_damage_guards` again. **Closes when:** a public-action regression reaches both ordering prompts, prevents both blockers' damage, removes one counter and leaves Champion alive, including a save/reload between the answers.

**F2 · MED — No parse comparison exists for the ported head**

The [parse-diff sticky](https://github.com/phase-rs/phase/pull/10639#issuecomment-6621063155) still names `6cba858ff5`, and it still reports `ShieldKind::Substitution`, which this head removes (record `9ebdf88e`). No CI run exists for `f995714a3c`: GitHub reports the PR CONFLICTING with `main`, so the pull-request workflow cannot build its merge commit.

**Integration (our doing, not yours):** you merged `main` through `206116c5cd`; #10016 (`4ba131744b`, Darksteel Angel / CR 614.16 counter doublers) landed after that and rewrote the same damage path. `git merge-tree` against `a0d78cc70f` conflicts only in the `crates/engine/src/game/effects/deal_damage.rs` imports, but the auto-merged body is not mechanical: #10016 adds a wither/infect post-counter pause (`PendingDamagePostCounter`, `wither_infect_post_counter_core`, `drain_pending_damage_post_counter`) inside damage application, beside this PR's batch runner, held riders and `DamageTail`. CR 120.3d: "Damage dealt to a creature by a source with wither and/or infect causes that source's controller to put that many -1/-1 counters on that creature." A wither or infect leg that pauses for a counter-replacement choice inside a parked or resumed batch needs one owner for the tail, with a witness from each side green on the merged head (#10016's Darksteel Angel / doubler suites and this PR's batch suites). You're welcome to do the integration with the F1 fix; maintainers can help with it.

**Closes when:** the merged head is pushed, CI posts a sticky naming it, and its changed-card set is reconciled with the description's claims.

## 🟡 Non-blocking

- 22 earlier maintainer reviews requested changes; [review 5911461092](https://github.com/phase-rs/phase/pull/10639#pullrequestreview-5911461092) superseded their findings. Neither finding above is carried.
- Reach note: the compound census wrappers are not yet shown on a printed guarded-prevention card; Sanwell's direct census is handled at `crates/engine/src/game/ability_scan.rs:7765`.
- Earlier CodeRabbit items stand as dispositioned: the combat rider-choice deferral stays fenced (`crates/engine/src/game/coverage.rs:10405`), as the description says.
- Nothing was executed for this review; the suite results in your 06:11Z comment are taken as reported.

## ✅ Clean

- Condition 1: the coverage fence calls the runtime substitution predicate (`coverage.rs:11489`) and names protection only (`coverage.rs:10416`).
- Condition 2: Fight, resumed batches and already-replaced damage share the damage dispatch and `DamageTail` (`crates/engine/src/game/effects/deal_damage.rs:1356`, `crates/engine/src/game/effects/mod.rs:15294`); `damage_resolver_owns_tail` and `parked_batch_owns_tail` are gone.
- Condition 3: `replaced_event_amount` records the amount actually prevented, zero included (`replacement.rs:4025`), and an optional answer restores that scope only around its own resolution (`crates/engine/src/game/engine_payment_choices.rs:211`). Channel Harm's amount after reload and Fateful Showdown's tail are asserted positively (`unpreventable_damage_prevention_effects.rs:3387`, `:3679`). CR 615.5: the additional effect "may refer to the amount of damage that was prevented."
- The #10433 resolution keeps both paths: pre-entry dice choices keep their pre-entry view and only the live-board arm holds damage riders (`crates/engine/src/game/engine_replacement.rs:744`); entrant parking and settlement wrap the scoped dispatch (`engine_replacement.rs:6728`).

**Recommendation: request changes for F1 and F2; the conditions on the three accepted design choices are satisfied.**

To address this review in one push, follow https://github.com/phase-rs/phase/blob/main/.claude/skills/address-review/SKILL.md

