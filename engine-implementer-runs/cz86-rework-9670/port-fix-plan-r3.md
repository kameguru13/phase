# Port fix plan r2: maintainer review of the port (F1 + F2), one batch

Mode: ordinary engine-planner plan for one `/address-review` batch (one push, one reply).
Revision r2 of `port-fix-plan.md`, closing `port-fix-plan-review-r1.md` (§0.1); everything else is
r1's.
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

### 0.1 Plan review r1 findings and how r2 closes them

| Plan-review finding | r2 closure |
|---|---|
| 1 HIGH behavior: a paused wither/infect leg's post-counter remainder, drained by the funnel while its noncombat batch is parked, settles its lifelink win/loss at once instead of with the batch's event | Traced in source (§4 "Outcome scope of a drained remainder"): on the merged tree the drain runs with no batch running (`GameState::damage_event_losses == None`), so `elimination::event_outcomes`' `None` arm records the win or loss into the nearest parked `Effect::ResumeDamageBatch` carrier's `ParkedDamageBatch.outcomes`, which the carrier's resume reinstalls as `EventLosses::Recording` and `settle_event_losses` settles with the batch's event. The upstream lines the finding cites are main's pre-merge `win_by_effect`, which the merge replaces with this branch's. **No engine edit**: the remainder keeps one owner (the funnel drain) and its outcomes keep one scope (the parked batch). The reviewer's witness is added as **V5-L** (Sickle Ripper + Basilisk Collar, Nefarious Lich, Laboratory Maniac, empty library; with and without the counter-order pause; reload at the prompt), with mutation **E6(a2)** proving the `None` arm is what carries it. A red V5-L on the merged head is a premise failure: stop and return (E7). |
| 2 MED machinery: W1/W2 do not prove all claimed ownership | V5 (W1) variant A asserts the Ripper's `DamageDealt` before the Colossus's and exactly one CR 120.1 record per leg; new **V5-T** (Prizefight: "Create a Treasure token." after the fight) asserts the later instruction runs once, after the batch completes, with mutation **E6(a3)**; V6 (W2) gives the Ripper a printed lifelink grant (the Lifelink aura) and asserts one gain of 2 and one damage record after the answer. Barroom Brawl was checked and not used (§0 Step 0). |
| 3 MED text: sizing | Replacement text applied verbatim (§4 Sizing); recounted after the r2 changes: the new rows land in already-counted test files, so the count is unchanged. |
| 4 LOW text: CI reconciliation | Replacement text applied verbatim (§8 Claimed parse impact). |

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
- **Basilisk Collar** (Equipment): "Equipped creature has deathtouch and lifelink. (Any amount of
  damage it deals to a creature is enough to destroy it. Damage dealt by this creature also causes
  you to gain that much life.)\nEquip {2} ({2}: Attach to target creature you control. Equip only
  as a sorcery.)"
- **Nefarious Lich**: "If damage would be dealt to you, exile that many cards from your graveyard
  instead. If you can't, you lose the game.\nIf you would gain life, draw that many cards
  instead.\nWhen this enchantment leaves the battlefield, you lose the game." (matches the test
  file's `NEFARIOUS_LICH`)
- **Laboratory Maniac** (2/2): "If you would draw a card while your library has no cards in it, you
  win the game instead." (matches the test file's `LABORATORY_MANIAC`)
- **Prizefight**: "Target creature you control fights target creature you don't control. (Each
  deals damage equal to its power to the other.)\nCreate a Treasure token. (It's an artifact with
  "{T}, Sacrifice this token: Add one mana of any color.")" Export parse: `Fight` with a
  `SequentialSibling` `Token` (Treasure, count 1) sub-ability, no `Unimplemented`.
- **Barroom Brawl** (checked, not used): "Target creature you control fights target creature the
  opponent to your left controls. Then that player may copy this spell and may choose new targets
  for the copy." Export parse: the fight's second target is `controller: null` (the "opponent to
  your left" restriction is dropped) and the `CopySpell` sub-ability is `optional: true` with no
  "that player" decider, so the copy would be offered to the wrong player. Its later instruction's
  reach is not established; Prizefight's unconditional, fully parsed later instruction is the
  witness instead.
- **Lifelink** (Aura): "Enchant creature\nEnchanted creature has lifelink. (Damage dealt by the
  creature also causes its controller to gain that much life.)"

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
- Added in r2 (grep-verified): CR 104.2b "An effect may state that a player wins the game."; CR
  104.3e "An effect may state that a player loses the game."; CR 104.3f "If a player would both win
  and lose the game simultaneously, that player loses the game."; CR 104.4a "If all the players
  remaining in a game lose simultaneously, the game is a draw."; CR 614.6 "If an event is replaced,
  it never happens. A modified event occurs instead, …"; CR 702.15b "Damage dealt by a source with
  lifelink causes that source's controller, or its owner if it has no controller, to gain that much
  life (in addition to any other results that damage causes)."; CR 701.14a "… Each of those
  creatures deals damage equal to its power to the other creature."; CR 120.3 (damage results), CR
  120.4 ("Damage is processed in a four-part sequence."); CR 111.10a (Treasure token); CR 702.2b
  (deathtouch). The PR's existing annotation for one simultaneous noncombat damage event whose
  effect-stated outcomes settle together is `CR 104.4a + CR 614.6 + CR 120.4`
  (`run_damage_batch_phases`), the noncombat analog of CR 510.2; r2 cites the same.

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
    `DamageDealt` per leg, exactly once across the pause; its lifelink sentence drives V5-L (a
    wither source with lifelink: its gain is a result of the leg's damage, CR 702.15b) and V6's
    lifelink grant.
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
- **Plan-review finding 1 (outcome scope of the drained remainder).** No new census: the class is
  the F2 census's sources that also have or are granted lifelink (W-R1's lifelink sentence; printed
  grants such as Basilisk Collar, Loxodon Warhammer, the Lifelink aura) crossed with the PR's
  existing effect-stated outcome producers in a noncombat batch (Laboratory Maniac's win, Nefarious
  Lich's loss, the class the PR's `a_lifelink_win_settles_with_the_batchs_damage` and
  `a_player_who_wins_and_loses_in_one_batch_loses` already cover). The mechanism is the PR's one
  outcome scope (`elimination::event_outcomes`), so the decision is ownership, not a card list.

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

**Third trace (plan-review finding 1): an effect-stated outcome produced by an answer while a
noncombat batch is parked.** The analog is this PR's lifelink gain parked on a life-gain ordering
choice (`damage_substitution_replacements.rs` `a_lifelink_win_settles_with_the_batchs_damage`,
`ordered == true`: "the win then comes inside that answer, while the batch is parked, and still
waits for the event"). Path on this branch (`f995714a3c`):
- `deal_damage::run_damage_batch` saves the enclosing `state.damage_event_losses`, sets
  `EventLosses::Immediate`, runs `run_damage_batch_phases`, and restores the enclosing value on
  return (`None` for a batch entered from a resolution).
- `run_damage_batch_phases` opens Phase C with
  `state.damage_event_losses = Some(EventLosses::Recording(outcomes.unwrap_or_default()))`; a
  delivery returning `DamageResult::NeedsChoice` calls `take_event_outcomes` (moves the recording
  out, leaves `Immediate`) and parks it in `ParkedDamageBatch { held, outcomes, .. }` via
  `park_damage_batch(.., &replaced[index + 1..], ..)`, which stores the record on the carrier
  node's `context.parked_damage_batch` (always, since `outcomes` is `Some`) and parks the carrier
  with `park_ability_continuation` (or `insert_ability_continuation_parent_at_child_boundary`),
  i.e. as a `ResolutionFrame::AbilityContinuation`.
- `elimination::win_by_effect` / `lose_by_effect` → `event_outcomes(state)`: `Recording` → record;
  `Immediate` → at once; **`None` → the first `ResumeDamageBatch` carrier found by
  `state.resolution_stack.ability_continuations_from_top_mut()` (`types/resolution.rs`, every
  `AbilityContinuation` frame, top first), recording into its `parked_damage_batch.outcomes`**.
- `resolve_resume_damage_batch` clones `ability.context.parked_damage_batch` (the record the
  answer wrote into) and re-enters `run_damage_batch`, whose Phase C reinstalls those outcomes as
  `Recording`; `settle_event_losses` → `elimination::settle_event_outcomes` settles them after the
  last delivery (or before the first `AfterEvent` rider).
On the merged tree (`git merge-tree --write-tree f995714a3c upstream/main` → `cab4eb484b`):
`elimination.rs` keeps this branch's `event_outcomes` unchanged (the merge adds only
`retire_parked_resolution_work`'s `pending_damage_post_counter = None` and test imports); main's
`apply_damage_after_replacement` stashes `PendingDamagePostCounter` and returns
`DamageResult::NeedsChoice` into the branch's Phase C loop; the answer reaches
`engine_replacement::handle_replacement_choice` → `handle_replacement_choice_inner`, whose
`ProposedEvent::AddCounter` arm applies the counters (`apply_counter_addition`) and falls through
to the funnel, which calls `drain_pending_damage_post_counter` before it drains
`active_ability_continuation` (the carrier). No code on that path sets `damage_event_losses`
(`grep -rn damage_event_losses crates/engine/src` on the branch and
`git grep -n damage_event_losses cab4eb484b -- crates/engine/src` on the merged tree: the only
writers are `deal_damage.rs`'s `run_damage_batch`, Phase C opening and `take_event_outcomes`; the
other hits are the reader in `elimination.rs` and the field's declaration and initializers), so the drain's
`gain_lifelink_for_damage_leg` → Nefarious Lich's draw → Laboratory Maniac's win reaches
`event_outcomes` with `None` and records into the carrier.

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
§D4/§Commit A. Added in r2: `game/elimination.rs` (`lose_by_effect`, `win_by_effect`,
`settle_event_outcomes`, `event_outcomes`) on the branch and the merged tree;
`types/resolution.rs` `ability_continuations_from_top_mut`; `types/game_state.rs`
`damage_event_losses` / `EventLosses`; `effects/deal_damage.rs` `run_damage_batch`,
`take_event_outcomes`, `settle_event_losses`, `resolve_resume_damage_batch`; main's
`deal_damage.rs` stash site in `apply_damage_after_replacement`, `wither_infect_post_counter_core`,
`drain_pending_damage_post_counter`; the merged `engine_replacement.rs` (`handle_replacement_choice`,
`handle_replacement_choice_inner`'s `AddCounter` arm, the funnel's post-loss and post-counter drains);
`damage_substitution_replacements.rs` `a_lifelink_win_settles_with_the_batchs_damage`,
`a_player_who_wins_and_loses_in_one_batch_loses`, `player_losses`, `completions`;
`unpreventable_damage_prevention_effects.rs` `NEFARIOUS_LICH`, `LABORATORY_MANIAC`,
`equipment_of`, `attach_to` use, `assert_nothing_after_the_game_ended`,
`nothing_happens_in_the_batch_after_a_lifelink_gain_ends_the_game`.

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
  auto-orders, or the leg is not wither), report the fixture, not a result. The same markers hold
  at V5-L's prompt (variant A board, replaced len 1) and at V5-T's prompt (Constrictor + Vizier:
  the Additive/Subtractive pair `replacement_ordering_is_material` reports as non-commuting, as in
  W2); V5-L's no-pause variant (Constrictor only) reaches no prompt.
- **E5 (merged tree).** After the import union the workspace compiles with no other edit; the CR733
  matrix test passes with main's fixture; the post-replacement continuation census `--check`
  passes; `GameState` stays under 16,384. Any further compiler-forced edit is a standing-class
  path; a semantic conflict returns to planning.
- **E6 (mutation, F2 tail owners).** Temporarily (never committed), one at a time:
  (a1) skip the funnel's `drain_pending_damage_post_counter` call → W1 must go red (paused leg's
  `DamageDealt` missing);
  (a2) make `elimination::event_outcomes`' `None` arm return `None` → V5-L's pause variant must go
  red because the Colossus's return-leg `DamageDealt` is **absent** before `GameOver` (immediate
  elimination can retire the carrier before that event is emitted), while its no-pause variant,
  written as a separate `#[test]` so the paused case's first failing assertion cannot stop it, stays green (the
  live `Recording` scope carries it), which proves the parked carrier's arm is what records the
  drained remainder's win;
  (a3) keep `park_damage_batch`'s `DamageTail::Stashed` return, but invert its park order so the
  stashed Treasure tail is placed **above** the carrier instead of beneath it → V5-T must go red on
  the Treasure's `TokenCreated` preceding `EffectResolved { kind: Fight }`. (Returning
  `DamageTail::Caller` is not a valid mutation: Fight is a multi-event damage instruction, so the
  caller would park the tail at the captured child boundary beneath the carrier, another valid owner,
  and V5-T could stay green; deal_damage.rs:1341-1356, effects/mod.rs:29632-29658.)
  (b) make `complete_combat_phase_c_remnant` skip the counter stash → W2 must go red. (c) For F1,
  restore the fresh `read_damage_guards(state)` at entry → T-F1 and N-3 go red. Restore and
  re-run green.
- **E7 (premise of plan-review finding 1's closure).** V5-L's pause variant passes on the merged
  head with no engine edit. If it is red, the §2 third trace is wrong at some step: stop and return
  to planning with, from a temporary marker in `drain_pending_damage_post_counter` (never
  committed), the value of `state.damage_event_losses` at entry and whether
  `ability_continuations_from_top_mut()` finds a `ResumeDamageBatch` carrier with
  `parked_damage_batch.outcomes.is_some()`; print the marker on V5-T too as the positive control
  that the instrument fires. Do not improvise an outcome scope.

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
- Plan-review finding 1: every lifelink wither/infect source (printed or granted: Basilisk Collar,
  Loxodon Warhammer "Equipped creature gets +3/+0 and has trample and lifelink.", the Lifelink
  aura) whose drained remainder's gain becomes an effect-stated win or loss (Nefarious Lich's "If
  you would gain life, draw that many cards instead." into Laboratory Maniac's "you win the game
  instead") while its noncombat batch is parked. Every such outcome reaches
  `elimination::win_by_effect` / `lose_by_effect`, so the one outcome scope covers the class.

### Sizing

U1 carries combat prevention guard readings across a park (T-F1). U2 verifies noncombat Phase C continuation through the post-counter funnel and `ResumeDamageBatch` (W1 and E6a). U3 verifies combat Phase C continuation through the `PendingCombatLifelink` remnant (W2 and E6b). U2 and U3 require the main merge, and W2’s opening fixture also requires U1. The current scope is eight counted paths across three units. T1 fires, T2 does not, and the conjunction does not fire. Recount after incorporating any changes to the continuation scope or witnesses.

Scope paths (phase-fit counting rule; protocol record and regenerated census excluded; files the
merge brings in unchanged from main are main's, not this change's):
1. `crates/engine/src/game/replacement.rs`
2. `crates/engine/src/game/combat_damage.rs`
3. `crates/engine/src/types/game_state.rs`
4. `crates/engine/src/game/effects/prevent_damage.rs` (compiler-forced unit test)
5. `crates/engine/src/game/effects/deal_damage.rs` (conflict resolution, imports only)
6. `crates/engine/src/types/game_state_size.rs` (measurement note)
7. `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs` (also V5-L, V5-T
   and V6's lifelink grant)
8. `crates/engine/tests/integration/damage_substitution_replacements.rs` (`temper`,
   `cast_temper_at_one`, `player_losses` become `pub(super)`)
Excluded: `protocol/changes/<new uuid>.json`; `scripts/post-replacement-continuation-baseline.txt`
only if E5's `--check` reports drift (regenerated with `--write`).

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
- Outcome scope (plan-review finding 1): `elimination::event_outcomes` (the one reader of
  `GameState::damage_event_losses` and of a parked carrier's `ParkedDamageBatch.outcomes`),
  `deal_damage::take_event_outcomes` / `settle_event_losses`, `elimination::settle_event_outcomes`.
  No new scope, wrapper or field.
- Tests: `fight_answering` (generalized to `fight_driving`), `preferring`, `offered`,
  `assert_batch_settled`, `undergrowth_champion`, `creature_with_keywords`, `cast`, `answer_one`,
  `plus_counters`, `damage_dealt_to`, `equipment_of` + `attach_to`, `NEFARIOUS_LICH`,
  `LABORATORY_MANIAC`, `assert_nothing_after_the_game_ended`, `pariah`'s Aura-builder idiom (for
  the Lifelink aura), and `damage_substitution_replacements::player_losses`.

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
     `complete_combat_phase_c_remnant` (the funnel skips combat stashes). *Its effect-stated
     outcomes* (non-combat): recorded into the parked batch's `ParkedDamageBatch.outcomes` by
     `elimination::event_outcomes`' no-running-batch arm and settled with the batch's event by
     the carrier's `settle_event_losses` (below, "Outcome scope of a drained remainder").
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

#### Outcome scope of a drained remainder (plan-review finding 1)

Rules: a fight's two damage legs are one simultaneous noncombat event (CR 701.14a: "Each of those
creatures deals damage equal to its power to the other creature"; CR 608.2f; the PR's
`CR 104.4a + CR 614.6 + CR 120.4` batch, the noncombat analog of CR 510.2), and the lifelink gain is
one of that damage's results (CR 702.15b, CR 120.3). A win (CR 104.2b) or loss (CR 104.3e) an
effect states during that event therefore settles with the event's other outcomes, where CR
104.3f can still apply, after the event's remaining damage is dealt.

Decision: **no engine change; one owner for the remainder, one scope for its outcomes.** The
evidence (§2 third trace, source lines named there):
1. While the batch is parked, nothing is running it, so `state.damage_event_losses` is `None`:
   `run_damage_batch` restored the enclosing value on its return after the park, and no code on
   the answer path writes the field.
2. The answer's funnel runs `drain_pending_damage_post_counter` before it resumes the carrier, and
   the drain's lifelink gain → Nefarious Lich's draw → Laboratory Maniac's win reaches
   `win_by_effect` → `event_outcomes`, whose `None` arm records into the nearest
   `ResumeDamageBatch` carrier's `parked_damage_batch.outcomes` (present: the Phase C park parks
   `outcomes: Some(..)`).
3. The carrier then resumes; `resolve_resume_damage_batch` passes that record to
   `run_damage_batch_phases`, whose Phase C reinstalls it as `EventLosses::Recording`, delivers
   the remaining leg (the Colossus's 11), and settles the recorded win in `settle_event_losses`.
The design alternatives the finding names are therefore already the code: the drain does run
under the parked batch's outcome scope, through `event_outcomes`; wrapping the drain in a second
`Recording` scope would add a second writer of `damage_event_losses` and a second owner of the
same outcomes, which the finding's own constraint forbids. The finding's upstream citations
(`deal_damage.rs:1622-1642`, `elimination.rs:176-190` on `upstream/main`) are main's pre-merge
code, whose `win_by_effect` the merge replaces with this branch's (`git diff f995714a3c
cab4eb484b -- crates/engine/src/game/elimination.rs` changes only `retire_parked_resolution_work`
and test imports). The closure is the witness V5-L with mutation E6(a2), and E7 if it is red.
The same arm already carries the post-loss stash's drained lifelink (`drain_pending_damage_post_loss`
runs in the same funnel state), which r1's §10 wrongly listed as a residual; that note is withdrawn.

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

### Identity / Provenance Contract (plan-review finding 1: outcomes of a drained remainder)

| Item | Value |
|---|---|
| Rules concept | An effect-stated win or loss during one simultaneous noncombat damage event settles with that event's other outcomes (CR 104.2b, CR 104.3e, CR 104.3f; the PR's `CR 104.4a + CR 614.6 + CR 120.4`), and a lifelink gain is a result of that event's damage (CR 702.15b, CR 120.3) |
| Authority type | `EventOutcomes { losers, winners }` inside `ParkedDamageBatch.outcomes` on the `Effect::ResumeDamageBatch` carrier's `context.parked_damage_batch` |
| Binding time | Phase C opens the recording (`EventLosses::Recording`); the park moves it onto the carrier (`take_event_outcomes`) |
| Live vs latched | Accumulates: the drained remainder's win or loss is added to the parked record while the batch is parked (`event_outcomes`' `None` arm); it never settles before the carrier resumes |
| Storage | Running: `GameState::damage_event_losses` (`#[serde(skip)]`, `None` outside a running batch); parked: the carrier's `ParkedDamageBatch.outcomes` (serialized with the carrier, so a reload at the prompt keeps it) |
| Consumer | `elimination::win_by_effect` / `lose_by_effect` → `event_outcomes` (record); `settle_event_losses` → `settle_event_outcomes` (settle) |
| Invalidation | Settled once by the resumed batch; a game end discards the carrier (`retire_parked_resolution_work`, which also clears `pending_damage_post_counter` on the merged tree) |
| Hostile fixture | V5-L: the win arrives in the answer while the batch is parked and the Colossus's leg is still undelivered (variant A order); its no-pause sibling records through the live scope; E6(a2) separates the two arms |

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
| **V5 W1** `a_wither_fight_leg_paused_on_its_counters_completes_once` | A wither leg of a two-phase damage batch that pauses for its -1/-1 counter order completes exactly once, before the batch's later portion, and the batch completes once (CR 120.3d + CR 702.80a, CR 616.1, CR 701.14a, CR 608.2c) | merged `run_damage_batch_phases` Phase C + `drain_pending_damage_post_counter` + `ResumeDamageBatch` | `cast` Pit Fight (P0). Variant A: P0's Darksteel Colossus (keywords Trample, Indestructible) fights P1's Sickle Ripper (keyword Wither), P0 controls Winding Constrictor + Doubling Season → P0 orders. Variant B: P0's Sickle Ripper fights P1's Colossus, P1 controls both → P1 orders. Answer with `answer_one`; reload variant at the prompt | `-1/-1` on Colossus: Constrictor first 6, Season first 5; exactly one `DamageDealt` Ripper→Colossus amount 2 (emitted after the answer) and one Colossus→Ripper amount 11; in variant A the Ripper's `DamageDealt` precedes the Colossus's (the paused leg completes before the batch's later portion); exactly one CR 120.1 record per leg in `state.damage_dealt_this_turn` (Ripper→Colossus amount 2, Colossus→Ripper amount 11, no other record from either source); exactly one `EffectResolved { kind: Fight }`, after both | E4 markers at the prompt (stash `Some`, `!is_combat`; carrier present, replaced len 1 in A / 0 in B); Ripper in graveyard; Pit Fight in graveyard | Parity sibling: Constrictor only → no prompt, 3 counters, same event shape (derived 2 + 1). E6(a1) mutation red | none |
| **V5-L W1 lifelink win** `a_wither_fight_legs_lifelink_win_waits_for_the_rest_of_the_fight` | A win stated by an effect during a paused wither leg's drained remainder is recorded into the parked batch's event and settles after the batch's remaining damage, with and without the counter-order pause (CR 104.2b, CR 702.15b, CR 120.3, the PR's CR 104.4a + CR 614.6 + CR 120.4; W-R1) | `elimination::event_outcomes` (`None` arm → parked carrier) reached from `drain_pending_damage_post_counter`; Phase C reinstall + `settle_event_losses` | `cast` Pit Fight (P0): variant A's board (P0's Darksteel Colossus fights P1's Sickle Ripper) with P1's Basilisk Collar attached to the Ripper (`equipment_of` + `attach_to`), P1's Nefarious Lich and Laboratory Maniac, P1's library empty (asserted). Pause variant: P0 controls Winding Constrictor + Doubling Season, both orders, plus a reload at the prompt; no-pause variant: Constrictor only | `GameOver { winner: Some(P1) }` exactly once, and the Colossus→Ripper `DamageDealt` amount 11 precedes it; `player_losses == [(P0, {OpponentWon})]`; `assert_nothing_after_the_game_ended`; no `EffectResolved { kind: Fight }` (CR 104.1 ends the batch at its settlement) | Before `GameOver`: the Ripper→Colossus `DamageDealt` amount 2 (after the answer in the pause variant), `ReplacementApplied` from the Lich (`GainLife`) and the Maniac (`Draw`); pause variant prompt reached (E4); exactly one `CounterAdded` event for the Colossus with counter type `-1/-1` and count 6 / 5 / 3 (no pause), emitted before `GameOver` (do not read the Colossus's counters at the end: P0's loss removes its objects from the game, CR 800.4a, and its counters cease, CR 122.2) | E6(a2): pause variant red, no-pause variant green. E7 if red on the merged head | none |
| **V5-T W1 later instruction** `a_fights_later_instruction_runs_once_after_a_paused_wither_leg` | The instruction after the fight runs once, after the paused leg's remainder and the batch's completion (CR 608.2c, CR 111.10a) | `park_damage_batch` → `DamageTail::Stashed` (tail parked beneath the carrier) | `cast` Prizefight (P0): P0's Darksteel Colossus fights P1's Sickle Ripper; P0 controls Winding Constrictor + Vizier of Remedies (no Doubling Season, which would double the Treasure) → P0 orders; both orders; reload at the prompt | exactly one `TokenCreated` named Treasure for P0, after `EffectResolved { kind: Fight }`, which follows both `DamageDealt` events; P0 controls exactly one Treasure; Colossus has 2 `-1/-1` counters in either order | E4 markers at the prompt; Prizefight in graveyard; `assert_settled` | E6(a3) mutation red | none |
| **V6 W2** `a_wither_leg_pauses_on_its_counters_inside_a_resumed_combat_batch` | A combat wither leg that pauses on its counter order after the batch was parked and resumed in its replacement pass completes once, and the batch completes once (CR 510.2, CR 120.3d, CR 616.1) | merged Phase C `PausedOnCounter` + `complete_combat_phase_c_remnant`, after U1's resume | `fight_driving`: T-F1's board plus P0's Sickle Ripper attacking, enchanted by P0's Lifelink aura (printed grant: "Enchanted creature has lifelink."; Aura builder as `pariah`, attached with `attach_to` after the build), blocked by P1's Winding Constrictor; P1 also controls Vizier of Remedies. Prompts 1–2 answered Champion; prompt 3 (P1) both orders; reload variant at prompt 2 and at prompt 3 | Constrictor has exactly 2 -1/-1 counters (Constrictor first: 3 − 1; Vizier first: 1 + 1) and is on the battlefield; exactly one `DamageDealt` Ripper→Constrictor amount 2, emitted after prompt 3's answer; exactly one CR 120.1 record Ripper→Constrictor amount 2 in `state.damage_dealt_this_turn` after the answer (none before it); exactly one P0 `LifeChanged` with `amount == 2` in the whole run, after prompt 3's answer, and P0's life 22 at the end (CR 702.15b: the gain is the damage dealt, 2, not the counters placed); Champion as in V1 | choosers `[P0, P0, P1]`; prompt 3 `offered == {Constrictor, Vizier}`; E4 combat markers at prompt 3; Ripper in graveyard (Constrictor's 2, doubled by Marauders to 4, against toughness 1) | E6(b) mutation red | none |
| **V7** both sides' suites on the merged head | unchanged behavior | merge | #10016: `darksteel_angel_cant_have_counters`, `wither_counter_pause_resume`, `soul_scar_mage_damage_to_counters` (incl. #10016's Doubling Season row), `grub_storied_matriarch_769`, `issue_1337_tekuthal_proliferate_twice`, `serpent_society_ward_poison_cost`, `std_counters_grammar_axes`, `mauhur_swarming_of_moria`, `one_ring_burden_upkeep_lethal`, `cr733_resolved_commands_p0`; #9670: the eight suites in `task.md`; this PR: `damage_substitution_replacements`, `unpreventable_damage_prevention_effects`, `ironscale_hydra_combat_prevention`, `farideh_fireball_die_branch_selection`, `combat_lifelink_replacement_ordering` | green | — | A PR row using Doubling Season on a rider's or substitute's counters (Phytohydra, Gatta and Luzzu, Ironscale Hydra, Temper) that flips under #10016's CounterCause gate is derived from CR 614.16 (those placements are a replacement/prevention effect's, so eligible): a flip is a defect, stop and return, never re-baseline | — |

Hostile-fixture notes: V1's first production branch is `pending_candidates_are_current` on resume
(`Some(_) if … !pending_candidates_are_current` → re-propose); V2's is
`settled_damage_guard_reading` returning `Fails` during discovery of the second Bears portion;
V5's are `park_damage_batch` with non-empty vs empty `replaced` and the funnel's
`!stashed.is_combat` gate; V6's is `take_if(|s| s.is_combat)` in
`complete_combat_phase_c_remnant`; V5-L's is `event_outcomes`' `match state.damage_event_losses`
(`None` arm in the pause variant, `Recording` arm in the no-pause variant); V5-T's is
`park_damage_batch`'s `ability.sub_ability` arm (the tail parked beneath the carrier, returning
`DamageTail::Stashed`). Empty/decline path: not applicable (no optional replacement);
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
| V6 lifelink | The Ripper deals 2 (not doubled: Angrath's Marauders doubles sources P1 controls); lifelink gains the damage dealt, 2, once, as a result of that damage, which is dealt when the counter event P1 orders occurs (CR 702.15b, CR 120.3, CR 616.1) → one gain of 2 after prompt 3's answer, P0 at 22 | merged head | derived |
| V5-L | Ripper's 2 to the Colossus is dealt; its lifelink gain of 2 becomes Nefarious Lich's draw of 2, and the first draw with an empty library becomes Laboratory Maniac's win for P1 (CR 614.6, CR 104.2b); the Colossus's 11 to the Ripper is part of the same fight event (CR 701.14a), so it is dealt before the win settles with the event; P0 loses because P1 won (CR 104.2b) → one `GameOver { winner: Some(P1) }` after both damage events, `[(P0, {OpponentWon})]`. Counters: Constrictor first 6, Season first 5, Constrictor alone 3 (as V5) | merged head | derived; the existing `a_lifelink_win_settles_with_the_batchs_damage` reads the same rule on a different board |
| V5-T | The fight completes, then "Create a Treasure token." runs once (CR 608.2c, CR 111.10a): one Treasure for P0 (no token doubler on the board); counters 2 in either order (as V6) | merged head | derived |
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
   `creature_with_keywords`), `BASILISK_COLLAR`, `LIFELINK_AURA` and `PRIZEFIGHT` (verbatim, §0
   Step 0; reuse the file's `NEFARIOUS_LICH` and `LABORATORY_MANIAC`), `fn minus_counters(runner,
   id) -> u32`, and `fn damage_records(runner, source, target) -> Vec<u32>` (the amounts of
   `state().damage_dealt_this_turn` entries with that `source_id` and `target ==
   TargetRef::Object(target)`). In `damage_substitution_replacements.rs` make `player_losses`
   `pub(super)`.
2. V5 W1 (two variants × {in memory, reload at the prompt} × {two orders} + parity sibling, with
   variant A's event order and the per-leg `damage_records` assertions), V5-L (pause variant: two
   orders × {in memory, reload at the prompt}; no-pause variant: in memory), V5-T (two orders ×
   {in memory, reload at the prompt}) and V6 W2 (both prompt-3 orders × {in memory, reload at
   prompts 2 and 3}, with the Lifelink aura's gain and record assertions). Each doc comment quotes
   the cards' Oracle text and cites the §5 row's CR numbers; V5-L's doc states the derivation in
   §6 (the Colossus's 11 is part of the same fight event, so it is dealt before the win settles).
3. E4, E6(a1), E6(a2), E6(a3), E6(b), E7.
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
parse-diff.json`); the CI run's sticky and its `parse-diff.json` artifact are authoritative either
way (§8).

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
    `DamageTail`. A win or loss the leg's lifelink causes while the batch is parked is recorded
    with the batch's event and settles after the batch's remaining damage (CR 104.2b, CR
    702.15b). Witnesses: Sickle Ripper fighting Darksteel Colossus under Winding Constrictor
    and Doubling Season (both orders, either side, with a reload); the same fight with the Ripper
    wearing Basilisk Collar under Nefarious Lich and Laboratory Maniac with an empty library (the
    Colossus's damage is dealt before the win settles, with and without the pause); Prizefight's
    Treasure created once, after the fight; and Sickle Ripper with the Lifelink aura blocked by
    Winding Constrictor under Vizier of Remedies after the Champion prompts above (one gain of 2,
    one damage record)."
- **Claimed parse impact:** Wait for a completed parse-diff sticky naming the pushed head. Reconcile the description against the full `parse-diff.json` artifact from that same run, using the sticky as the published reference. A baseline-pending sticky leaves F2 open.
  Replace the `4a00656a8` archive sentence and the "pending" sentences with the sticky's
  card count and signature count and a link. Reconcile card by card: every card in the artifact
  appears under one of the bullets with its signature change; every claimed card the artifact does
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
- **CR references:** add CR 120.3d, CR 510.4, CR 104.2b, CR 702.15b and CR 111.10a (cited by the
  new tests), each grep-verified; 608.2f, 700.1, 510.2, 616.1 already listed.
- **Verification:** a new head block for the pushed head (the §7 list with actual counts), "Both
  sides' suites pass" extended with #10016's list, the new witnesses with their fails-at-base
  results (T-F1 and N-3 at `f995714a3c`; W1 variants and W2 mutation-checked, E6(a1)–(a3), (b)), `Gate A PASS head=… base=…`,
  `Final review-impl PASS head=…` (or UNRESOLVED per §4). Keep the LLM section as it is.
- **Validation Failures / known gaps:** unchanged (r1's candidate residual about a drained
  lifelink outside the batch's recorded outcomes is withdrawn: §4 "Outcome scope of a drained
  remainder", witnessed by V5-L).

## 9. Reply (address-review §6), after the push

```markdown
Addressed <review URL> at <pushed head>.

- F1 — fixed in <C1>. The combat pass reads the printed guards once, when the batch opens, and carries the reading through the park (`CombatDamageReplacementPass.settled_guards`, `PendingCombatDamageReplacement.settled_guards`). Evidence: `undergrowth_champion_keeps_its_reading_across_both_orders_of_a_double_block` (fails at f995714a3c: Champion dies to the doubled damage after the second answer), in both answer orders and with a save/reload between the answers; `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park` (a false reading stays false, fails at f995714a3c). Siblings: noncombat batches already carry the reading (`with_damage_batch`, `PendingReplacement.settled_damage_guards`); a later combat damage step reads afresh (`a_later_combat_damage_step_reads_the_guard_afresh`).
- F2 — fixed in <M> (main through <sha>) and <C2>. One owner per piece of a paused wither/infect leg: the counter stash owns its event and record, the parked batch its rest, `DamageTail` the spell's later instructions. A win or loss the leg's lifelink causes while the batch is parked is recorded with the batch's event and settles after its remaining damage. Witnesses: `a_wither_fight_leg_paused_on_its_counters_completes_once`, `a_wither_fight_legs_lifelink_win_waits_for_the_rest_of_the_fight`, `a_fights_later_instruction_runs_once_after_a_paused_wither_leg`, `a_wither_leg_pauses_on_its_counters_inside_a_resumed_combat_batch` (each mutation-checked), and #10016's suites green on the merged head. Parse sticky for <pushed head>: <link>; the description's changed-card set is reconciled with that run's `parse-diff.json` artifact.
```

## 10. For the orchestrator / a human

- r1's residual (a drained remainder's lifelink win settling outside the parked batch's outcomes)
  is withdrawn: the source trace (§2 third trace, §4 "Outcome scope of a drained remainder") shows
  `elimination::event_outcomes` records it into the parked carrier on the merged tree, with no
  engine edit. This is source-traced, not probed; V5-L with E6(a2) measures it, and a red V5-L
  returns to planning (E7) rather than being fixed ad hoc.
- #10016's combat `PausedOnCounter` path has no witness on main (its combat rows no longer pause
  once Doubling Season is ineligible in combat); W2 is the first. If W2 shows a defect in that
  path, it is main's code: report it rather than fix it here (the maintainer offered help).
- G1's "remove only one counter" with two or more counters stays the PR's disclosed known gap.
- The local parse diff needs network for the base coverage; skip it if not allowed and rely on
  the CI run's sticky and `parse-diff.json` artifact (§8).


## r3 changes (Codex plan review round 2, magi-2dc86710 r2)
- V5-L: the counter assertion reads the single `CounterAdded` placement event before `GameOver`, not the Colossus's end-state counters (elimination exiles it and clears them; elimination.rs:1467-1485, :2219-2227; zones.rs:702-708).
- E6(a2): the expected failure is the absence of the Colossus's return damage; the no-pause control is its own test.
- E6(a3): replaced with an inverted park order (tail above the carrier); `Caller` hands the tail to another valid owner.
- Round-1 findings 1, 3 and 4 closed by the reviewer; finding 2 closed by the E6(a3) correction.
