<!-- claude-projects-attribution: {"github_login":"kameguru13"} -->
_Requested by **Jakob**_

## Summary

This is the first part of CR 615.12 work: prevention effects with additional effects ("riders"). It fixes three problems those effects have today. Applying riders under damage that can't be prevented (CR 615.12 itself), and applying a recipient-phrased effect once per simultaneous batch, are a follow-up (see Scope Expansion).

**1. "…instead" damage replacements replace damage (CR 614.1a, CR 615.1a).** Phytohydra, Lichenthrope, Dralnu, Force Bubble, Delaying Shield, Nefarious Lich and Crumbling Sanctuary were modelled as prevention plus a rider. Six of them replaced nothing: Phytohydra took the damage with no counters, and Nefarious Lich lost the game on any damage. They are now `DamageDone` replacements with a new `ShieldKind::Substitution`.
- Damage that can't be prevented is still replaced (Phytohydra's ruling), and nothing counts as prevented (CR 615.13).
- Riders that pick from a population now choose (CR 608.2d): Dralnu sacrifices permanents of your choice, Nefarious Lich exiles cards you choose from your graveyard, and Outfitted Jouster sacrifices an Equipment attached to it.
- A combat order prompt now takes one legal order instead of dropping the damage (Polukranos, Unchained with Furnace of Rath). Offering that choice to the player stays in the existing combat-prompt follow-up.
- A later instruction survives these routes exactly once. Lightning Helix's "and you gain 3 life" runs after a prompting rider and once after an order prompt. It keeps the chain's identity behind remaining recipients (Nibelheim Aflame), and a prevented answer doesn't drop it.

**2. Printed "while" guards gate the prevention (CR 604.2, CR 611.3a).** Polukranos, Oathsworn Knight, Undergrowth Champion, Ugin's Conjurant ("while it has a +1/+1 counter on it"), Jared Carthalion, True Heir ("while you're the monarch") and Outfitted Jouster ("while it's equipped") prevented damage even when the condition was false.
- Counter guards go through `OnlyIfQuantity`. Other states use a new `ReplacementCondition::OnlyIfStatic`.
- The guard is read once for all simultaneous damage (CR 510.2, CR 608.2f), as Undergrowth Champion's ruling says: "its prevention effect will apply once to all that damage." The reading survives an order prompt, travels with the remaining portions, and covers a sentence's joined clauses (Chandra's Outrage, Searing Blaze's else clause).
- Outfitted Jouster's conjured Equipment now attach to it.
- Riders listed after "prevent that damage," (Bloatfly Swarm, Sekki, Seasons' Guide) are now reported unsupported instead of passing as supported.

**3. Every rider pause inside a combat damage batch is settled (CR 510.2).** A rider that paused mid-batch left its prompt or resolution frame behind.
- An Ironscale Hydra counter-order prompt (with Hardened Scales and Doubling Season) left `pending_replacement` set. State-based actions stopped: the 3/3 blocker survived 5 damage, a later Lightning Bolt left a 2/2 alive, and the game stuck at Cleanup.
- Channel Harm's "you may" in combat panicked on an orphaned `OptionalEffect` frame.
- `drain_combat_damage_rider` now settles every pause at both combat drains (the per-portion drain and Phase D):
  - an order prompt takes one legal order (CR 616.1f);
  - a "may" prompt is declined, so Swans of Bryn Argoll's draws under Abundance are kept;
  - a prompt raised by the rider's own effect is cleared (Channel Harm, Nefarious Lich, Dralnu, Outfitted Jouster with two Equipment).
- Each of these carries a strict-failure note pointing at the combat-prompt follow-up. This also fixes the same leak on damage that can be prevented.

### Review round 1 (matthewevans)

- M1: later rider work waits behind a damage rider that paused (Test of Faith + Dralnu, Lich Lord under Hardened Scales and Doubling Season); the paused rider's frame keeps dispatching its queued work in order; a rider whose draw was replaced card by card retires when its frame resumes. This also fixes a Pyroclasm crash (`repeat-for drain may consume only the active repeat-for frame`).
- M2: the Perforator Crocodile test proves the same input reaches Conjure.
- CodeRabbit: three inline noncombat drains go through `drain_damage_rider`; the Substitution doc says why CR 615.12 doesn't reach it.
- One `ResolutionPosition` replaces two copies of the same measure; Gate D now also counts `dispatch_pending_post_replacement_effect` as a consumer.
- `PostReplacementDrain.event_amount` is a new optional field (serde default, skipped when empty), covered by the PR's existing `game` protocol records.

### Merge with main (#9341) and CodeRabbit round 2

- Merged main 402ceaec6: #9341's resolution-time damage division keeps its resume and splicing, and its portions run inside the damage batch (`resume_resolution_division` opens its own); witness `a_resolution_time_division_keeps_its_later_riders_behind_a_paused_one`.
- A replacement's action reads its own amount inside a resolving trigger (Shockmaw Dragon + Phytohydra / Dralnu, Lich Lord / Test of Faith).
- A prevention line whose "while" guard can't be lifted fails closed (`prevention_while_guard`); no printed card changes.

### Review round 2 (matthewevans)

- A replacement's action owns its amount ahead of a die result: Farideh's Fireball's 2 damage to each player now puts 2 delay counters on Delaying Shield (was the d20 face), 2 depletion counters on Force Bubble, and Nefarious Lich asks for 2. The amount is stamped through one authority, bound when the action runs, carried across a prompt on the pending continuation, and restored on unwind.

### Main merge (6d99397fb) and fight damage

- Merged main 6d99397fb (no conflicts).
- Fight damage is one damage batch (CR 701.14a + CR 608.2f), so a printed "while" guard is read once for both directions, including when the first direction pauses for a CR 616.1 choice (two Nomads en-Kor redirecting fight damage to an Undergrowth Champion with one counter).
- A board-census guard (Sanwell, Avenger Ace's "As long as an artifact creature you control is attacking") is read once per damage batch, so a substitution earlier in the batch (Dralnu, Lich Lord sacrificing the attacker) can't strip it; a forwarding conjure's Attach keeps the ability's source as host.
- A die-roll table finishes once its replacement child retires: Farideh's Fireball with Delaying Shield, Doubling Season and Pir now reaches full rest with Fireball in the graveyard after the order choice (CR 706.3b, CR 608.2n).

### Review round 4 (matthewevans)

- A game that ends inside a damage batch ends it (CR 104.1): no later combat portion, trigger, lifelink gain or prevention rider happens, and outside combat no later recipient is dealt damage. With three or more players, later combat portions assigned to the player who left aren't dealt (CR 800.4a, CR 800.4e).
- A fight's reverse direction waits behind a paused rider instead of overwriting its prompt; Searing Blaze's paused second clause binds as the unpaused path does; the static-condition visitor is complete.
- A damage shield whose rider asks a choice combat damage can't ask yet reports the gap `damage_replacement_choice_unasked_in_combat`: Battletide Alchemist, Channel Harm and Dralnu, Lich Lord report unsupported. Force Bubble's and Outfitted Jouster's confirmed-misparse rows are deleted.

### Main merge (430dbf12b)

- Merged main 430dbf12b (465a98138). Two conflicts, both mechanical: the `coverage.rs` import list (main's `LibraryShuffleMode` rename) and two adjacent additions in `effects/mod.rs` (the resumed action's amount scope ends, then main's die-result publication is taken). The combat-choice classifier also lists main's new `ChangeZone::library_shuffle` field, which asks no choice. Engine clippy and all engine tests are green, and Codex found nothing.

### Review round 6 (matthewevans)

- A noncombat rider that leaves its prompt standing holds the instruction's remaining recipients behind its answer (CR 608.2c, CR 616.1). Each remaining portion waits in its own frame, beneath the paused rider and ahead of the spell's later instructions, so no later recipient's prompt replaces it. This covers Flame Rift with two Nefarious Liches, Pyroclasm at Dralnu and a Phytohydra under Mending Hands, and Rakdos Charm with a lifelink survivor.
- Aggregate combat riders stop at game end, checked before each shield's rider (CR 104.1).
- A damage substitution reports `damage_substitution_choice_incomplete`: protection prevents ahead of its CR 616.1 choice, and simultaneous multi-source damage asks one choice per source (Force Bubble's ruling gives one). Force Bubble, Phytohydra, Lichenthrope, Crumbling Sanctuary and Delaying Shield become unsupported.
- Outfitted Jouster's confirmed-misparse row is restored for its sacrifice clause, which still runs as a rider rather than a trigger (CR 603.3).

### Review round 7 (matthewevans)

- The each-source, each-target-power and each-deals-equal-to-power damage batches run on one runner. Every portion's replacements are applied before any is delivered (CR 120.4b, CR 120.4c), through every pause. A paused batch parks the rest of itself as one internal `Effect::ResumeDamageBatch`; an answered order choice hands its event back to that batch; and the batch emits the instruction's single completion, with the original kind and source. Rakdos Charm, Allies at Last, Coordinated Clobbering and Pariah redirects are covered.
- Damage to each player (and the player half of damage to all) collects players in APNAP order (CR 616.1, CR 101.4).
- The combat stop row uses Deflecting Palm's printed inline rider.

### Review round 8 (matthewevans)

- Merged main (#10801's mana-ability import kept).
- The damage batch stops at game end at every boundary (entry and resume, after a replacement, a rider or a delivery, and before its completion; CR 104.1), and skips a recipient that left the game (CR 800.4a).
- A batch's portions are ordered by the APNAP rank of the player each affects, so simultaneous replacement choices are asked in APNAP order (CR 616.1, CR 101.4).

### Review round 9 (matthewevans)

- Every fight pause (first direction, second direction, self-fight) parks the rest of the fight as one batch that completes once (CR 701.14a, CR 608.2c).
- An order choice a redirect hands to a player later in APNAP order than another outstanding choice of the same batch waits in the batch, keeping its applied replacements (CR 614.5); the batch asks every outstanding choice in APNAP order, restarting for an earlier player (CR 616.1, CR 101.4, CR 101.4d).

### Review round 10 (matthewevans)

- A fight's two directions run as one two-phase batch: both replaced before either is dealt, choices in APNAP order, one completion (CR 701.14a, CR 608.2f, CR 616.1). With Gratuitous Violence, Dralnu, Lich Lord now deals the cards' 6.
- A choice the batch's own replacement pass hands to a later player (a sole Pariah redirect) waits behind earlier players' outstanding choices too (CR 101.4).

### Review round 11 (matthewevans)

- A damage batch carries every portion through its automatic replacements (a sole Pariah's redirect) before it asks any choice, then asks the outstanding choices in APNAP order (CR 616.1, CR 101.4).
- Dralnu's sacrifice in a fight is recorded as a strict failure on both seats (held-rider follow-up; Dralnu reports unsupported).

### Review round 12 (matthewevans)

- Held riders: while a two-phase damage batch settles its original replacement and prevention choices, a replacement's action or a prevention's rider is held, including one from an answered order choice the batch awaits (CR 616.1, CR 615.5). The held work belongs to the batch: its ledger while it runs, its parked continuation (`SpellContext.parked_damage_batch`) while it waits.
- After delivery the batch runs substitution actions (CR 614.6), then prevention after-effects (CR 615.5), each in APNAP order of the affected player (CR 101.4). Both Dralnu fight rows give the cards' 6, and Pit Fight asks P0's shield order before P1's counter order on either seat.
- Rows that assumed a substitution's action ran during replacement now follow that order: Nefarious Lich's exile and loss come with the batch's damage, and Undergrowth Champion's counter removal waits for it.

### Review round 13 (matthewevans)

- A capped redirect (Nomads en-Kor's "next 1 damage") inside a two-phase batch hands its redirected part back to the batch as a portion that keeps its applied replacements (CR 614.9, CR 614.5): it is replaced with the batch's other portions, its choices are asked in APNAP order, and it is dealt with them (CR 120.4b, CR 616.1). Split parts survive a pause and an answered choice, and an answer's next choice waits behind an unprocessed split part (CR 101.4d).

### Review round 14 (matthewevans)

- A two-phase batch's damage and substitution actions are one simultaneous event: effect-stated wins and losses in it (`elimination::lose_by_effect` / `win_by_effect`) are recorded with the batch, even inside an answer while it is parked, and settle together before its prevention after-effects. A player who would both win and lose loses (CR 104.3f), and players who all lose draw (CR 104.4a). Two Nefarious Lich losses draw; Allies at Last deals the Bears' damage before the Laboratory Maniac win.
- A capped redirect's part meets its new recipient's protection (CR 702.16e).

### Review round 15 (matthewevans)

- Protection's prevention (CR 702.16e) at the pre-replacement gate now respects damage that can't be prevented (CR 615.12, Stomp), on every damage path; a capped redirect's point reaches a pro-red Silver Knight after Stomp.
- A batch's event outcomes add each loser's shared-life teammates before weighing wins (CR 810.8a), so a Two-Headed Giant team that would both win and lose loses (CR 104.3f).

### Review round 16 (matthewevans)

- The two-phase batch defers a portion that would spend a shared limited shield after a competing replacement choice has been collected (CR 615.7, CR 616.1). The Allies at Last test offers Mending Hands and Awe Strike for Hill Giant when it is selected first. Review round 17 closes the other source order: a portion that would take a limited shield on its own also waits while a portion still to be replaced will choose among that shield and another effect, so selecting Rumbling Baloth first offers the same choice. The choice is which source's damage the shield meets; the shield is then spent on that damage in full (narrowed in review round 18, below).
- Fenced: a capped redirect's part meets protection at the pre-replacement gate, ahead of a competing prevention shield on its new recipient, so the recipient's controller isn't offered the CR 616.1 order. Capped redirect shields report `capped_redirect_recipient_choice_incomplete` (14 cards: Carom, Daughter of Autumn, Harm's Way, Lancers en-Kor, Martyrdom, Nomads en-Kor, Outrider en-Kor, Razia, Boros Archangel, Shaman en-Kor, Spirit en-Kor, Vassal's Duty, Warrior en-Kor, Zealous Inquisitor, Zhalfirin Crusader).

### Review round 18 (matthewevans)

- Fenced and narrowed: CR 615.7 lets the recipient's controller split a limited shield among several sources that deal damage to it at the same time (Mending Hands: 2 on Hill Giant and 2 on Rumbling Baloth). The batch offers only which source's damage the shield meets first, then spends it on that damage in full, as on main. Damage instructions whose several sources can hit one recipient report `shared_shield_split_unoffered` (24 cards: Allies at Last, Band Together, Common Goal, Friendly Rivalry, Graceful Takedown; Coordinated Clobbering, Tandem Takedown, Terrific Team-Up; Aura Barbs, Bartz and Boko, Case of the Gateway Express, Grim Contest, Judgment of Alexander, Kamahl's Will, Missy, Moonlight Hunt, Nissa's Judgment, Princess Snowfall, Rakdos Charm, Sarkhan the Mad, Sarkhan the Masterless, Signature Slam, The Bears of Littjara, The Great Aerie). A fight is not fenced: its two directions hit different recipients unless redirects send both to one (two Jade Monolith activations), and that case keeps the gap.

## Files changed

- `crates/phase-ast/src/types/ability.rs`, `crates/phase-ast/src/types/ability_visit.rs`
- `crates/phase-parser/src/parser/oracle_replacement.rs`, `oracle_cost.rs`, `oracle_effect/{imperative,lower,mod}.rs`, `oracle_static/mod.rs`, `oracle_tests.rs`
- `crates/engine/src/game/{replacement,engine_replacement,combat_damage,elimination,filter,mana_abilities,sba,zone_pipeline,ability_scan}.rs`, `effects/{mod,deal_damage}.rs`
- `crates/engine/src/types/{ability,game_state,resolution}.rs`
- `crates/engine/tests/integration/damage_substitution_replacements.rs` (new), `unpreventable_damage_prevention_effects.rs` (new), `ironscale_hydra_combat_prevention.rs`, `cost_zone_pipeline.rs`, `spelunking_shockland_order.rs`, `main.rs`
- `protocol/changes/c85ad0f6-….json` (`ShieldKind::Substitution`), `protocol/changes/ccf4c183-….json` (`ReplacementCondition::OnlyIfStatic`)
- `scripts/post-replacement-continuation-baseline.txt`

## Track

Developer

## LLM

Model: Claude Opus 5.5
Tier: Frontier
Thinking: high

## Implementation method (required)

Method: /engine-implementer (chartered run, 3 phases)

## CR references

CR 614.1a, CR 615.1a, CR 615.5, CR 615.13, CR 608.2c, CR 608.2d, CR 608.2f, CR 604.2, CR 611.3a, CR 510.2, CR 616.1, CR 616.1f, CR 603.12, CR 120.8, CR 609.7b, CR 700.1, CR 702.15b, CR 104.1.

## Verification

- [x] Required checks ran clean, or the exact CI-owned alternative is stated below.
- [x] Gate A output below is for the current committed head.
- [ ] Final review-impl below is clean for the current committed head.
- [x] Both anchors cite existing analogous code at the same seam.

- Current head `5c7ba6c37522f84674f19fe31acc38361e9fe866`: formatter and Git diff checks pass. The maintainer commit changes one test comment and leaves test logic intact. [Exact-head hosted CI is pending](https://github.com/phase-rs/phase/actions/runs/43639706795).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean at 7911d131f (rebased on main). The two later commits change one parser function and comments; `cargo clippy -p phase-parser -p phase-engine --all-targets -- -D warnings` is clean at 8152ff952.
- `cargo test -p phase-engine --tests --no-fail-fast` at 7911d131f: 28,677 passed, 0 failed (lib 13,535; integration shards 2,814 / 3,010 / 3,087 / 3,038 / 3,103).
- `cargo test -p phase-parser --lib` at 8152ff952: 12,157 passed, 0 failed.
- `scripts/check-engine-authorities.sh`: Gate B, draw-replacement producers and Gate D PASS at the head.
- `protocol-versions.mjs generate` + `check`: clean.
- `.githooks/pre-push`: ran on push.
- Integration with main at 77b702374 (contains main a87272f29; conflicts resolved keeping both sides, listed in the PR reply): `GameState` boxed its damage batch ledger to stay under the stack-size ceiling (15,808 B), and `LazyBoxedVec` keeps `ResolvedAbility` at main's size so the Kathril chain test fits the 16 MiB test stack (15.98 MiB, as on main). Workspace clippy clean; `cargo test -p phase-engine` green at the default test stack; `phase-parser`, `phase-server` and `phase-ai --lib` green; wasm32 check and engine-authorities gates PASS.
- Integration with main at 6535a6fb5 (contains main 880296e82; every conflict resolved keeping both sides, listed in the PR reply): workspace clippy clean; `cargo test -p phase-engine` green at the default test stack; `phase-parser`, `phase-server` and `phase-ai --lib` green; wasm32 check and engine-authorities gates PASS.
- Review round 18 at 5bc3e4eb4 (contains main 49e410adc): workspace clippy clean; `cargo test -p phase-engine --tests` green; coverage tests green; wasm32 check passes. The shared-recipient fence test (Allies at Last, Coordinated Clobbering, Rakdos Charm fenced; Pit Fight not) is mutation-checked.
- Review round 17 at d112df5a2 (contains main 2e8320047): workspace clippy clean; `cargo test -p phase-engine --tests` green; server and AI suites green on the fix; wasm32 check passes. The shared-shield witness runs both selection orders and fails without the look-ahead.
- Review round 16 at 4a00656a8 (contains main c66ede515): workspace clippy clean; `cargo test -p phase-engine --tests` green; server, parser and AI suites and the engine-authorities gates green on the batch; wasm32 check passes. Shared-shield, 2HG and capped-redirect fence tests mutation-checked.
- Review round 15 at f7ee65a99: workspace clippy clean; engine, server and AI suites green; wasm32 check and engine-authorities gates PASS. Stomp and 2HG witnesses mutation-checked.
- Merged main f31e0d673 at 6250a48b2 (no conflicts): main's new event-target scan classifies `ResumeDamageBatch`, and its loop-normalization field census names this PR's ledger fields. Native and wasm32 builds, clippy, and engine, server, parser and AI suites green.
- Merged main 733a37b47 at 7e412ef06: four conflicts with #10742 resolved keeping both continuation fields and guards, with a witness for each side; the joined-damage-run clone is boxed off the chain walker's frame (Kathril's chain back within the 16 MiB test stack). Workspace clippy clean; engine, server, parser and AI suites green; engine-authorities gates PASS.
- Merged main fc726f971 at e5ec57c98: two conflicts resolved keeping both sides; the combat-choice coverage check classifies main's new owner-shuffle lowering (Weeping Angel). Workspace clippy clean; engine (default test stack), server, parser and AI suites green; engine-authorities gates PASS.
- Review round 14 at 34a282892: workspace clippy clean; `cargo test -p phase-engine --tests` green at the default test stack; `phase-server`, `phase-parser --lib` and `phase-ai --lib` green; engine-authorities gates PASS. Witnesses mutation-checked (immediate settlement, parked owner, settle order, split gate, CR 104.3f each turn rows red).
- Review round 13 at 286dc5336: workspace clippy clean; `cargo test -p phase-engine --tests` green at the default test stack; `phase-server`, `phase-parser --lib` and `phase-ai --lib` green; engine-authorities gates PASS. Capped-redirect witnesses mutation-checked (split capture, split deferral, answer hand-off each turn rows red).
- Review round 12 at 3accfb100: workspace clippy clean; `cargo test -p phase-engine --tests` green at the default test stack; `phase-server`, `phase-parser --lib` and `phase-ai --lib` green; engine-authorities gates PASS. Held-rider witnesses mutation-checked (answer path, stash sites, APNAP order each turn rows red).
- Every seam has a test that fails when the seam is reverted (phase 3: per-portion settle 14 rows, Phase D settle 5, order arm 11, may arm 4, unwind 4; parser `while` fix: its unit test).

## Gate A

Gate A PASS head=77b7023747f85a226f9e6f052180de81e367e230 base=a87272f29b060c3805ed2a74fb5bd35d7e8aefb4

## Anchored on

- `crates/engine/src/game/engine_replacement.rs:232` on the base: `handle_replacement_choice`, the existing replacement-answer authority shared with the combat rider path.
- `crates/phase-parser/src/parser/oracle_replacement.rs:12313` on the base: `parse_while_antecedent`, the life-gain/draw "while" antecedent parser that `while_antecedent_clause` now shares with damage prevention.

## Final review-impl

Final review-impl PASS head=0f04754b5 (Codex, `review-engine-impl` PR Review Mode, two rounds: one finding, the fence missing the "they each deal damage" form, fixed and cleared). The later commits are main merges. Codex reviewed the first (b5993c8e3, the continuation-path conflicts) with no findings; the later two resolve field-name, test-module and import conflicts only.

Historical final review-impl PASS head=76add36c2 (Codex, `review-engine-impl` PR Review Mode, two rounds: F1 cleared; one body-wording finding, corrected). The later commit is a main merge with no conflicts.

Historical final review-impl PASS head=d7694d67b (Codex, `review-engine-impl` PR Review Mode, two rounds: one finding, the F3 fence's proving test, fixed and cleared). The later commits are a main merge with no conflicts and a type alias in that test.

Historical final review-impl PASS head=d1abce96a8513c1c0f24f944538b9edf4073acc9

Each phase had its own implementation review (phase 1: three rounds; phase 2: one; phase 3: one). The run-level integration review needed two rounds. Round 1 found five items; round 2 found one comment-only correction, applied at the reviewed historical head. The maintainer’s current-head review has open findings.

## Claimed parse impact

The [parse archive for the code-identical parent `4a00656a8`](https://github.com/phase-rs/phase/actions/runs/43636452720/artifacts/14130596516) reports 16 cards across 13 signatures. The [hosted coverage job](https://github.com/phase-rs/phase/actions/runs/43636452720/job/132574204240) reports 29,342 → 29,330 supported faces, with 12 coverage-honesty drops and zero classified engine regressions. The review round 17 head changes the damage batch's scheduler and its test, and no parser code, so the parse signatures are expected to match `4a00656a8`; its exact hosted comparison is pending. Review round 18 changes no parse signature; its fence adds coverage-honesty drops for the shared-recipient damage instructions listed above that were supported (up to 24), and the hosted coverage job at the pushed head gives the exact count.

- **Substitution (7):** Phytohydra, Lichenthrope, Dralnu, Nefarious Lich, Force Bubble, Delaying Shield and Crumbling Sanctuary: `shield_kind` Prevention{All} → Substitution. The first six also change `event` from DealtDamage to DamageDone, and Nefarious Lich, Force Bubble and Delaying Shield gain `damage_target_filter: Player{Controller}`.
- **Population riders (2):** Outfitted Jouster and Three Dog, Galaxy News DJ: "sacrifice an Equipment/Aura attached to ~" gains `AttachedTo{Source}`.
- **Counter guards (4):** Polukranos, Unchained; Oathsworn Knight; Undergrowth Champion; Ugin's Conjurant gain `OnlyIfQuantity{CountersOn(P1P1, Source) ≥ 1}`.
- **Other guards (2):** Jared Carthalion, True Heir gains `OnlyIfStatic{IsMonarch}`. Outfitted Jouster gains `OnlyIfStatic{SourceIsEquipped}`, and its conjure trigger attaches the conjured Equipment to itself.
- **Fail closed (2):** Sekki, Seasons' Guide and Bloatfly Swarm had a "prevent all" shield whose rider was silently dropped. The shield is now replaced by an Unimplemented ability. Both cards appear in the current signature diff. Neither appears in the current hosted support-flip list.
- The phase 3 commits and the last two commits change no card.

## Scope Expansion

The task asked for CR 615.12 in full. Planning found three prerequisite defects, and this PR ships them: the seven "…instead" cards, the "while" guards, and the combat rider pauses. The run's last phase, applying recipient-phrased effects once per simultaneous batch and then admitting riders under damage that can't be prevented, went six plan-review rounds. Each round closed its findings and found a new edge, all on multiplayer, concession and multi-prompt boards. Jakob chose to ship phases 1-3 and move the rest to a follow-up with the evidence. CR 615.12's strict-failure tags stay in place and point to it.

## Validation Failures

None.

Known gaps, each filed as a follow-up:
- A limited shield can't be split among several sources that deal damage to its recipient at the same time (CR 615.7); it meets one source's damage in full first. Multi-source damage instructions that can hit one recipient report unsupported (`shared_shield_split_unoffered`). A fight whose two directions are redirected to one recipient, and combat damage, keep the same gap without the fence, as on main.
- Combat damage can't pause for a rider's choice yet: Dralnu's sacrifice, Channel Harm's "may" and Outfitted Jouster's Equipment choice are answered by the engine; those faces report unsupported. CR 616.1 order choices between other cards' replacements in combat (Hardened Scales with Doubling Season) take one engine-chosen order.
- A prevention rider that runs in its own portion still runs when a later portion of the same combat batch ends the game (Swans of Bryn Argoll's draw before a Nefarious Lich loss).
- A multi-card draw keeps drawing after Laboratory Maniac wins (draw sequence).
- A damage substitution's CR 616.1 choice is incomplete: protection prevents the damage before the substitution is offered (CR 702.16e), and simultaneous damage from several sources asks one choice per source where Force Bubble's ruling gives one. These faces report unsupported (`damage_substitution_choice_incomplete`).
- New Way Forward's "When damage is prevented this way" is a trigger (CR 615.13) that the parser runs as an inline rider; the face already reports a confirmed misparse.
- Delivery choices in a damage batch (a lifelink gain's replacements) follow delivery order rather than the choosing player's APNAP rank. Same on main for every damage instruction.
- In a two-phase damage batch, a substitution's action runs just after the batch's damage rather than as one simultaneous event with it (CR 614.6, CR 101.4).
- Combat damage settles effect-stated wins and losses, and a capped redirect's part, as main does: combat does not run the two-phase batch.
- A capped redirect's part reaching a recipient with protection from its source is prevented by protection before any other prevention shield there is offered (CR 616.1); those faces report unsupported (`capped_redirect_recipient_choice_incomplete`). Protection is a pre-replacement gate on every damage path, not one of the CR 616.1 candidates.
- One player's several held riders run in creation order; the player doesn't choose it (CR 101.4c).
- DamageAll (Pyroclasm) resolves recipients in its own loop, not a two-phase batch, so its riders still run per recipient, before a later recipient's original choice.
- A conjure with an unmodelled tail after its zone parses as a supported Conjure (Kari Zev, Crew of Two's "tapped and attacking"; Perforator Crocodile's "attached to").
- Across different creatures in a DamageAll instruction, a substitution (Phytohydra, Dralnu) can wait behind another creature's paused rider, depending on creation order; CR 614.6 + CR 615.5 put it first. Two-phase batches now hold both and run substitution actions first. It was creation-dependent before this PR too.
- Perforator Crocodile's Stab Wounds aren't attached to the creatures ("attached to that creature" is dropped).
- Abundance on a multi-card draw reveals in reverse order and reads the wrong chosen kind.
- Riders under damage that can't be prevented (CR 615.12), and one counter per simultaneous batch for recipient-phrased effects (Undergrowth Champion, the Phantom cycle, Oathsworn Knight).
- Offering combat order and rider choices to the player instead of taking one legal order (the existing combat-prompt follow-up).
- Awe Strike on Thrashing Wumpus covers only the first damage portion (CR 615.8).
- "Put that many +1/+1 counters" parses as one counter per damage (Gatta and Luzzu, Brace for Impact, Temper, Test of Faith); two tests assert today's reading under strict-failure comments.
- Searing Blaze can't be cast without a planeswalker, and after a pause its else clause damages the planeswalker.

## CI Failures

None.

🤖 Generated with [Claude Code](https://claude.com/claude-code)


<!-- This is an auto-generated comment: release notes by coderabbit.ai -->
<!-- coderabbit-ai-generated:v1 -->

## Summary by CodeRabbit

* **Bug Fixes**
  * Improved damage replacement handling, including prevention and substitution effects, ordering, and resolution after player choices.
  * Corrected replacement conditions that depend on the affected object and preserved damage amounts when resolving replacement effects.
  * Fixed parsing and resolution of attachment instructions, including conjured cards attaching to a specified object and sacrifice costs involving attached objects.
  * Improved Dredge eligibility checks and handling of unrepresentable life, counter, and energy changes.
  * Fixed replacement and combat effects that resume after a prompt from resolving out of order or more than once.
* **Tests**
  * Expanded coverage for combat damage, replacement ordering, and resolution that resumes after prompts.
<!-- end of auto-generated comment: release notes by coderabbit.ai -->





























