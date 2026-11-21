# Plan r1: reconcile #10639 with main's #9670, one authority per seam

Run: `cz86-rework-9670`. BASE (PR head) `2fc5a9188e`, main `upstream/main` = `91639fdf02`, merge base
`c57fd36003`, conflicted merge tree `e8dcf6fa52b25b801a15b947ce9b9b366a6d523e` (11 conflicted files).
Delivery shape (task constraint): one merge commit of `upstream/main` into
`engine/unpreventable-damage-riders`, then follow-up commits, pushed once after verification and an
independent implementation review. No model identifiers anywhere. No `git stash`. `.claude/skills/**`
untouched.

Probe status: the run forbids builds in this worktree (another build holds the target dir), so **no
assertion in this plan was probed by running code**. Every claim about runtime behaviour below is
either read from source (cited by function) or labelled `UNPROBED` in §17 with the test that settles
it at implementation.

---

## 0. Premise check (Step 0)

Oracle text was read from MTGJSON `AtomicCards.json` (version `5.3.0+20261021`, at
`/mnt/data/dev/phase/data/mtgjson/AtomicCards.json`), not from the task brief. The brief's card claims
match the printed text:

- Soul-Scar Mage: "If a source you control would deal noncombat damage to a creature an opponent
  controls, put that many -1/-1 counters on that creature instead."
- Szadek, Lord of Secrets: "If Szadek would deal combat damage to a player, instead put that many
  +1/+1 counters on Szadek and that player mills that many cards."
- Undead Alchemist: "If a Zombie you control would deal combat damage to a player, instead that player
  mills that many cards."
- Phytohydra: "If damage would be dealt to this creature, put that many +1/+1 counters on it instead."
- Force Bubble: "If damage would be dealt to you, put that many depletion counters on this enchantment
  instead. / When there are four or more depletion counters on this enchantment, sacrifice it. / At the
  beginning of each end step, remove all depletion counters from this enchantment."
- Nefarious Lich: "If damage would be dealt to you, exile that many cards from your graveyard instead.
  If you can't, you lose the game. / If you would gain life, draw that many cards instead. / When this
  enchantment leaves the battlefield, you lose the game."
- Dralnu, Lich Lord: "If damage would be dealt to Dralnu, sacrifice that many permanents instead. / …"
- Ironscale Hydra: "If a creature would deal combat damage to this creature, prevent that damage and put
  a +1/+1 counter on this creature."
- Battletide Alchemist: "If a source would deal damage to a player, you may prevent X of that damage,
  where X is the number of Clerics you control."
- Channel Harm: "Prevent all damage that would be dealt to you and permanents you control this turn by
  sources you don't control. If damage is prevented this way, you may have Channel Harm deal that much
  damage to target creature."
- Gatta and Luzzu: "Flash / When Gatta and Luzzu enters, choose target creature you control. If damage
  would be dealt to that creature this turn, prevent that damage and put that many +1/+1 counters on it."
- Polukranos, Unchained: "… If damage would be dealt to Polukranos while it has a +1/+1 counter on it,
  prevent that damage and remove that many +1/+1 counters from it. …"
- Torbran, Thane of Red Fell: "If a red source you control would deal damage to an opponent or a
  permanent an opponent controls, it deals that much damage plus 2 instead."
- Furnace of Rath: "If a source would deal damage to a permanent or player, it deals double that damage
  to that permanent or player instead."
- Undergrowth Champion: "If damage would be dealt to this creature while it has a +1/+1 counter on it,
  prevent that damage and remove a +1/+1 counter from this creature. / Landfall — …"
- The Last Agni Kai: "Target creature you control fights target creature an opponent controls. If the
  creature the opponent controls is dealt excess damage this way, add that much {R}. / Until end of
  turn, you don't lose unspent red mana as steps and phases end."
- Outfitted Jouster: "When Outfitted Jouster enters, conjure a card named Steelclaw Lance and a card
  named Brawler's Plate onto the battlefield and attach them to Outfitted Jouster. / If damage would be
  dealt to Outfitted Jouster while it's equipped, prevent that damage. When damage is prevented this
  way, sacrifice an Equipment attached to Outfitted Jouster."

Other texts quoted below (Swans of Bryn Argoll, Abundance, New Way Forward, Hardened Scales, Doubling
Season, Test of Faith, Vigor, Lightning Helix, Pit Fight) are the verbatim constants already in the
branch and main test files; they match MTGJSON.

CR numbers below were each grep-verified in `/mnt/data/dev/phase/docs/MagicCompRules.txt`: 101.4,
101.4d, 104.1, 104.2b, 104.3e, 104.3f, 104.4a, 120.4b, 120.4c, 120.8, 510.2, 510.4, 603.12, 604.2,
608.2c, 608.2d, 608.2f, 611.3a, 614.1a, 614.5, 614.6, 614.9, 615.1a, 615.5, 615.7, 615.8, 615.12,
615.13, 616.1, 616.1e, 616.1f, 616.1g, 616.2, 700.1, 701.14a, 702.15b, 702.16e, 706.4, 800.4a,
800.4e, 810.8a.

---

## 1. Pattern Coverage (Step 0.5 record)

`jq --version` = `jq-1.8.1`; MTGJSON `5.3.0+20261021`. Query script (the skill's query verbatim, with
one change: each face's text has reminder text removed, `gsub("\\([^)]*\\)"; "")`, before masking, so
shield-counter / Spaceship reminder text does not select a card) saved as
`/mnt/data/dev/phase/.git/engine-implementer-runs/cz86-rework-9670/class/query.sh`.

### Class A — damage substitutions (seams 1 and 2)

Command (run in the `class/` directory):

```bash
bash query.sh 'If (damage would be dealt to [^.,]*|[^.]* would (deal|be dealt) [^.,]*damage[^.,]*), (instead (put|remove|exile|sacrifice|that player mills|create)|(put|remove|exile|sacrifice|that player exiles|that player mills)[^.]* instead)' '' '[]' '[]' > substitution_class.json
```

`add` = `[]`, `drop` = `[]`, `masked_only` = `[]`, `rejected_add`/`rejected_drop` empty, no keyword.
Count **12**: Crumbling Sanctuary, Delaying Shield, Dralnu, Lich Lord, Force Bubble, Lichenthrope,
Nefarious Lich, Oozeavite, Phytohydra, Soul-Scar Mage, Szadek, Lord of Secrets, Undead Alchemist,
Underdark Beholder. 38 ruling instances, 38 distinct rulings. File
`class/substitution_class.json`, `jq -cS . | sha256sum` =
`3f5d7d097714407289320c69a66af51663beebf175f6eeb177f48921597b256b`.

The 7 cards #10639 converts (Phytohydra, Lichenthrope, Dralnu, Force Bubble, Delaying Shield,
Nefarious Lich, Crumbling Sanctuary) and the 3 #9670 converts (Soul-Scar Mage, Szadek, Undead
Alchemist) are 10 of the 12. Oozeavite and Underdark Beholder are class members neither PR claims; the
fold (seam 2) makes the predicate structural, so their parse must be measured (§17 P6).

A broader sweep (any "would deal/be dealt … damage … instead", minus modifiers/redirects/prevention)
found no further substitution; the remaining hits are amount modifiers (Divine Presence, Equal
Treatment, Forethought Amulet, Mijo) which are `damage_modification`, not substitutions.

### Class B — prevention effects with additional effects (seams 3 and 4)

```bash
bash query.sh '(If [^.]*(would deal|would be dealt)[^.]*damage[^.]*, (you may )?prevent[^.]*\.[^.]*(this way|that much|that many)|prevented this way)' '' '[]' '[]' > prevention_rider_class.json
```

Count **39** (Acolyte's Reward, Awe Strike, Bone Mask, Brace for Impact, Candles' Glow, Channel Harm,
Chant of Vitu-Ghazi, Cho-Arrim Alchemist, Comeuppance, Deflecting Palm, Divine Deflection, Hallow,
Honorable Passage, Hostility, Immortal Coil, Inkshield, Intervention Pact, Judgment of Alexander, New
Way Forward, Outfitted Jouster, Phyrexian Hydra, Phyrexian Vindicator, Purity, Refraction Trap, Rem
Karolus, Stalwart Slayer, Reverse Damage, Ria Ivor, Bane of Bladehold, Sacred Boon, Samite
Ministration, Scars of the Veteran, Shadowbane, Sokrates, Athenian Teacher, Stormwild Capridor, Stuffy
Doll Avatar, Swans of Bryn Argoll, Temper, Test of Faith, Vengeful Archon, Vigor). 110 distinct
rulings. Hash `a49c1dc69e5fbbb2fcb63e1ec55e135623b8a17109bac4a63e5a24b24e5de6b5`.

### Class C — CR-defined: combat replacement/prevention choices (seam 3)

The rule (CR 510.2 + CR 616.1) applies without card text: combat damage is one simultaneous event and
its affected player chooses among replacement/prevention effects. Selection: A ∪ B (51 cards) minus
the three whose effect is noncombat-only by its printed text (Purity, Soul-Scar Mage, Stormwild
Capridor) = **48** cards whose replacement/rider can meet combat damage. Damage modifiers (Furnace of
Rath, Torbran, Gratuitous Violence) and redirects (Pariah, Ancient Adamantoise) are the other CR 616.1
candidates in combat; their behaviour is main's #9670 and is pinned by main's suites.

### Bearing rulings (all read; notes in `class/notes.txt`)

Quoted verbatim, with the Verification Matrix row each drives:

- Soul-Scar Mage: "Soul-Scar Mage’s effect isn’t a prevention effect. Damage that can’t be prevented
  will be replaced with -1/-1 counters." Phytohydra: "Phytohydra's ability doesn't prevent damage.
  Damage that can't be prevented will still be replaced by +1/+1 counters being put on Phytohydra."
  Delaying Shield: "Note that the damage is replaced and is not “prevented”. This means it can replace
  damage that “can’t be prevented”." Underdark Beholder: "…isn’t a damage prevention effect. It can
  replace damage that can’t be prevented." → **V2.2**.
- Force Bubble: "The effect does not prevent damage, it replaces it so the damage is neither dealt or
  prevented." Lichenthrope: "This card’s effect replaces damage with placement of -1/-1 counters, so
  side-effects of damage to it will not trigger." → **V2.3**.
- Force Bubble: "If you control more than one, each time you take damage you can decide which one
  replaces the damage, and that one replaces all of the damage. … Note that damage from multiple
  sources that happens at one time (such as combat damage) gives one choice, not one per source." →
  **V3.9** and the kept `damage_substitution_choice_incomplete` fence (Q1).
- Soul-Scar Mage: "If multiple prevention and/or replacement effects are trying to apply to the same
  damage, the controller of the creature that would be dealt damage chooses the order in which to apply
  them. …" Phytohydra: "If another effect would prevent damage from being dealt to Phytohydra or replace
  it with something else, Phytohydra's controller chooses which effect to apply first." Szadek: "If
  another effect would prevent Szadek's combat damage from being dealt to the defending player or
  replace it with something else, that player chooses which effect applies first. For example, the
  player can choose to first have Mending Hands prevent 4 of the damage and then apply Szadek's ability
  so that it gets only one counter." → **V2.4** (materiality) and **V3.1–V3.4** (combat asks).
- Soul-Scar Mage: "If an effect causes a creature you control to fight a creature an opponent controls,
  the damage is dealt simultaneously. For instance, if your 2/2 creature fights an opponent’s 4/4
  creature, your creature will be dealt 4 damage and your opponent’s creature will have two -1/-1
  counters put on it." → **V4.6** (fight batch with a held substitution action).
- Dralnu, Lich Lord: "Dralnu may be one of the permanents you sacrifice while applying its replacement
  effect." and "If Dralnu would be dealt more damage than the number of permanents you control, instead
  you sacrifice all permanents you control." → **V3.6**.
- Undead Alchemist: "If you control multiple Undead Alchemists, the multiple replacement abilities will
  have no added effect. … replaced only once …" → CR 614.5, unchanged (main's suite).
- Acolyte's Reward: "If Acolyte's Reward prevents damage, it deals its damage immediately afterward as
  part of that same prevention effect. This happens before state-based actions are performed, and
  before any player can cast spells or activate abilities. If the source of the original damage was a
  spell or ability, this happens before that spell or ability resumes its resolution." (Vengeful
  Archon, Refraction Trap, Divine Deflection carry the same.) → **V4.1–V4.4** (a rider and the batch's
  held work finish before the instruction's tail).
- Channel Harm: "The new damage is not combat damage, even if the prevented damage was." and the
  two-Channel-Harm loop "either player can break by choosing to not have Channel Harm try to deal
  damage" → **V3.7** (the "may" is a real choice in combat; its damage is an independent event,
  CR 700.1).
- Inkshield: "If multiple damage prevention effects would apply to combat damage that would be dealt to
  you, you get to choose the order the prevention effects are applied in." (Deflecting Palm, Phyrexian
  Hydra, New Way Forward, Ria Ivor carry equivalents.) → **V3.1–V3.4**.
- Vengeful Archon: "If you would be dealt combat damage by multiple attacking creatures, you choose
  which of that damage to prevent." (Acolyte's Reward, Divine Deflection, Refraction Trap.) → CR 615.7
  shared-shield gap, kept as on main for combat (no new row; `shared_shield_split_unoffered` fence
  unchanged).
- Test of Faith: "The +1/+1 counters are put onto the creature at the same time the damage is
  prevented." Temper: "The +1/+1 counters do show up at the same time as any unprevented damage …"
  Stormwild Capridor: "…it gets +1/+1 counters for the prevented damage before checking to see if it
  dies from the damage that couldn’t be prevented." → **V3.5** (riders settle before SBA; batch
  completes once).

---

## 2. The four seam decisions

### Seam 1 — replaced-event amount: one meaning, one installer, one reader

**Decision.**
- *Field and meaning:* `PostReplacementDrain::event_amount` (both sides added it; the merged tree has
  it twice, §15). One meaning: **the amount of the event this drain's replacement replaced or
  prevented, latched when the replacement applied** (CR 614.6 + CR 615.5). Main's narrower meaning
  ("set only by a damage substitution") cannot carry a prevention rider's "for each 1 damage prevented
  this way" inside a resolving trigger: the branch witness
  `damage_substitution_replacements::a_replacements_action_reads_its_own_amount_inside_a_resolving_trigger`
  (Shockmaw Dragon + Test of Faith reads 1, not the trigger's 4) needs it, and that claim is one the
  PR keeps. The serde attribute is main's (`#[serde(default)]`).
- *Installer:* main's `ReplacedEventContext { source, target, amount }`. `amount` is computed by one
  new private function `replacement::replaced_event_amount(state, rid, before: &ProposedEvent,
  outcome: ReplacedOutcome) -> Option<u32>` (where `ReplacedOutcome` is the existing applier verdict:
  `Prevented` or `Modified(&ProposedEvent)`), called at every stash site (the `Modified` arm, the
  `Prevented | PreventedBehindRider` arm of `apply_single_replacement_inner`, and the optional-branch
  install in `continue_replacement_impl`). It returns `Some` exactly where the branch's
  `stamp_replacement_amount` runs today:
  - damage substitution (`is_damage_substitution`, seam 2) → the damage amount the applier received;
  - damage prevention (`ShieldKind::Prevention{..}`, `ShieldKind::PreventionOneShot`, or a
    `PreventionMinus` damage modification) → `before − after` (all of `before` on `Prevented`), and
    `None` when that is 0 (CR 120.8, CR 609.7b);
  - life-gain substitution (`gain_life_execute_substitutes_event_type`) and life-loss substitution
    (the LoseLife non-`LoseLife` execute test in the life-loss applier) → the life amount;
  - every other replacement → `None`.
  The appliers keep setting `state.last_effect_count` exactly as main and the base do (that half of
  `stamp_replacement_amount` stays, inlined back to the direct assignment).
- *Reader:* the branch's dispatch-bound scope. `engine_replacement::dispatch_pending_post_replacement_effect`
  reads `drain.event_amount` once and binds it (`quantity::with_replacement_action_scope`), work left
  behind a child prompt carries it (`PendingContinuation::replacement_action`, re-entered by
  `effects::drain_pending_continuation`), and the `EventContextAmount` cascade reads only
  `quantity::replacement_action_amount()`. Main's second reader is deleted. Reason: main's reader looks
  up the *resident* drain at read time (`active_post_replacement_or_paired_parent`), which has no
  carrier for work resumed away from its paired frame and does not suppress the enclosing-trigger
  tiers for an amount-less action; the branch's carrier was added at the maintainer's review round 2
  ("bound when the action runs, carried across a prompt on the pending continuation, and restored on
  unwind") and its witnesses stand.

**Deleted:** `replacement::REPLACEMENT_AMOUNT_STAMP` (thread-local), the thread-local half of
`replacement::stamp_replacement_amount` (the fn itself is removed; its six call sites go back to
`state.last_effect_count = Some(u32_to_i32_saturating(amount))`), the `apply_single_replacement`
save/restore wrapper around `apply_single_replacement_inner` (exists only for the stamp; the two
collapse into one fn named `apply_single_replacement`), `GameState::post_replacement_event_amount()`
(main; only consumer is the cascade tier), main's cascade tier in `quantity::event_context_amount`
(`.or_else(|| state.post_replacement_event_amount().map(u32_to_i32_saturating))`), main's
`substituted_damage_amount` local (replaced by `replaced_event_amount`). Kept: `NestedAmountScope`,
`with_replacement_action_scope`, `replacement_action_amount`, `current_replacement_action`,
`resume_replacement_action`, `ResumedReplacementAction`, `ReplacementActionScope`,
`PendingContinuation::replacement_action` and its merge guards in `append_to_pending_continuation` /
`prepend_to_pending_continuation_with_producer`.

### Seam 2 — damage substitution (CR 614.1a): one predicate, `ShieldKind::Substitution` folded in

**Decision (the maintainer's "ShieldKind::Substitution folded into a single substitution
predicate").** Main's definition-level predicate `replacement::is_damage_substitution(repl)` (a
`DamageDone` replacement with `ShieldKind::None`, no `damage_modification`, and an `execute` that is
neither `Effect::DealDamage` nor `Effect::Unimplemented`) and its rid-level wrapper
`damage_execute_substitutes_event_type(state, rid)` are the single authority. `ShieldKind::Substitution`
is deleted; the branch's 7 cards parse with `ShieldKind::None` (default) and are classified by the
predicate exactly as main's 3 cards are. Every branch consumer of the tag calls the predicate.

Runtime semantics carried onto the predicate (all from the branch, none conflicts with main):
- applier: one substitution arm in `damage_done_applier` at main's Branch 3 position, with the
  branch's body — drain an earlier resident rider first unless a batch is holding riders
  (`damage_batch_holds_riders`), return `ApplyResult::Prevented` or `ApplyResult::PreventedBehindRider`
  — and no amount stamp (seam 1 computes it at the stash);
- CR 615.12 / CR 615.1a: `is_damage_prevention_replacement` returns `false` for a substitution
  (explicit `is_damage_substitution` check before the legacy description test);
- CR 120.8: the candidate-discovery gate drops a substitution for a 0-damage event (branch line
  "Urza's Armor applied first leaves a ping at 0"), keyed on the predicate;
- CR 616.1 materiality: main's `CandidateMateriality::Unconditional` for a substitution (the branch's
  `Writes { Damage, NonCommuting }` arm loses its `ShieldKind::Substitution` alternative);
- CR 615.7 shared capacity: a substitution spends none (`ShieldKind::None` falls into the existing
  `false` arm; the `Substitution` alternative is removed from the exhaustive match);
- held-rider timing: `hold_damage_rider` uses `damage_execute_substitutes_event_type(state, rid)` for
  `RiderTiming::WithEvent`, else `AfterEvent`;
- coverage: the `damage_substitution_choice_incomplete` fence keys on the engine predicate (made
  `pub(crate)`); coverage.rs's local `is_damage_substitution` is deleted.

**Deleted:** `ShieldKind::Substitution` (phase-ast), `ReplacementDefinition::substitution_shield`
(phase-ast builder), its 2 parser call sites and 1 engine test call site, every `ShieldKind::Substitution`
match arm (replacement.rs ×5, coverage.rs ×1), coverage.rs `is_damage_substitution`, and
`protocol/changes/c85ad0f6-12d8-45cc-ae35-edf384ac7f40.json` (this PR's unreleased record for the
variant; the change it records no longer exists).

### Seam 3 — combat replacement choices: main's asked order, and every rider pause asked

**Decision.** Main's resumable `CombatDamageReplacementPass` (legs `Unstarted / Settled /
AwaitingChoice / AwaitingContinuation`, APNAP by actual chooser, reconciliation, parking in
`PendingCombatLifelink.replacement`) is the only combat replacement authority. The branch's
auto-settling machinery is deleted. Carried onto main's pass from the branch:
1. **One ledger.** The branch's `GameState::damage_batch_ledger: Option<Box<DamageBatchLedger>>` is the
   single "what this batch settled" authority (main's base field `combat_one_shot_applications` was
   already removed by the branch; the merged tree still references it in main's pass code, §15).
   `CombatDamageReplacementPass` keeps `one_shot_applications` and gains `settled_guards:
   Vec<SettledDamageGuard>`, read once by `read_damage_guards` when the pass is created
   (`CombatDamageReplacementPass::new(state, events)`), so a printed "while" guard reads once for the
   whole batch across a park (CR 608.2f + CR 700.1; Undergrowth Champion ruling, "its prevention effect
   will apply once to all that damage"). `PendingCombatDamageReplacement` gains `settled_guards`
   (`#[serde(default, skip_serializing_if = "Vec::is_empty")]`). `replace_combat_damage_batch`
   installs `DamageBatchLedger { applications: take(pass.one_shot_applications), settled_guards:
   take(pass.settled_guards), paused_rider: None, held_riders: None, split_portions: vec![] }` on entry
   and moves both back into the pass on every exit.
2. **CR 104.1.** At the top of the pass loop, a game that has ended settles every `Unstarted` and
   `AwaitingChoice` leg as `Settled(None)` (clearing `pending_replacement` for a surfaced one) and
   completes; `run_combat_damage_replacements`'s `Complete` arm returns
   `CombatDamageBatch::Complete(record.batch_events)` before Phase C when `state.game_end.is_some()`
   (the branch's Phase B guard, moved). A continuation that ended the game
   (`drain_combat_damage_continuation`) settles its leg instead of parking on `GameOver`.
3. **CR 800.4a + CR 800.4e.** An `Unstarted` leg whose recipient has left
   (`combat_damage_recipient_has_left`, main's predicate, now also used for unstarted legs) settles
   `None` without being proposed.
4. **Earlier rider paused inside the pipeline.** The substitution arm's earlier-rider drain can pause
   inside a combat leg (Test of Faith's counters under Hardened Scales + Doubling Season, then
   Phytohydra's substitution queued behind). `settle_combat_damage_replacement` checks the ledger's
   `paused_rider` before draining: if set, the leg becomes `ContinuationPaused { survivor, waiting_for:
   state.waiting_for.clone() }`; the work queued behind runs when the prompt's round trip settles
   (`effects::dispatch_resident_post_replacement_work`), and the leg resumes through main's
   `AwaitingContinuation` arm.
5. **One combat rider drain.** `replacement::drain_combat_damage_rider(state, events) ->
   Option<WaitingFor>` is re-purposed: suspend the ledger, `apply_pending_post_replacement_effect`,
   restore the ledger, return the pause — no settling. Main's `drain_combat_damage_continuation` calls
   it for the per-leg drain, and Phase D calls it for each aggregate rider.
6. **Phase D parks too.** `combat_damage::fire_combat_prevention_riders` takes `&mut
   Vec<(AppliedReplacementKey, i32)>` and returns `Option<WaitingFor>`; it pops one shield at a time,
   keeps the CR 104.1 stop, and when a rider pauses it returns the prompt with the unprocessed shields
   still in the vector. `drain_combat_lifelink` then parks the record (`remaining` and
   `damage_to_players` already drained, `replacement: None`) in `state.pending_combat_lifelink` and
   returns `CombatDamageBatch::Paused`, exactly like its lifelink-gain pause; the existing
   priority-resume (`resume_pending_combat_lifelink` → `resume_combat_damage_replacements` →
   `drain_combat_lifelink`) finishes the remaining shields once. CR 510.2 forbids a priority window,
   not the rider's choice (CR 615.5, CR 608.2d).
7. Main's `order_batch_by_affected_seat` (CR 616.1 + CR 101.4) and APNAP lifelink order stay as on
   main; the branch's combat paths had no APNAP ordering.

**Deleted:** `replacement::settle_combat_order_parks`, `settle_combat_rider_pauses`,
`CombatRiderPause`, `answer_combat_rider_park`, `abandon_combat_rider_pause`, the branch body of
`replace_combat_damage_batch` and its `(survivors, tally)` signature, the `drain_damage_rider` combat
routing (`if state.combat_prevention_tally.is_some() { drain_combat_damage_rider(..) }`), the branch's
Phase B tuple call in `apply_combat_damage`, the coverage fence
`StructuralFeature::DamageReplacementChoiceUnaskedInCombat` with `damage_replacement_reaches_combat`,
`damage_replacement_asks_a_choice_in_combat`, `prevention_rider_asks_a_choice_in_combat`,
`damage_rider_asks_a_choice`, `damage_rider_effect_asks_a_choice` (the exhaustive `Effect` match),
`RiderTargets`, the `extract_damage_replacement_choice_feature` combat half, the two coverage unit tests
`a_damage_riders_instruction_asks_for_a_picked_player_an_up_to_count_or_a_population` and
`a_damage_replacement_choice_combat_cant_ask_is_a_coverage_gap`, the module-doc strict-failure
paragraphs in the three branch test files, and main's dead `combat_one_shot_applications` references.

### Seam 4 — multi-recipient noncombat pause and tail ownership: main's `DamageTail`

**Decision.** Main's `DamageTail::{Caller, Stashed}` returned through
`deal_damage::resolve_instruction` / `multi_event_damage_resolver`, mapped by
`effects::resolve_effect_step` to `TailOwnership::{Caller, Transferred}`, is the single tail authority;
`resolve_chain_body` places the tail only for `Caller` (at the child boundary when the instruction left
child work, main's condition `is_multi_event_damage_instruction`). The branch's tail inference is
deleted. Carried onto it:
- **The table is every damage instruction.** `multi_event_damage_resolver` lists `DealDamage`,
  `DamageAll`, `DamageEachPlayer`, `EachDealsDamageEqualToPower`, `EachSourceDealsDamage` (main),
  plus `Fight` (CR 701.14a; branch runs it as one two-phase batch), `ResumeDamageBatch` (the parked
  batch), and `ApplyPostReplacementDamage` (an already-replaced portion; no production constructor
  since the branch's phase 3, but a restored game can hold one). `resolve_legacy_effect` lists all
  eight in its `unreachable!` arm.
- **Two-phase batches keep `run_damage_batch`.** The branch's `run_damage_batch` (replacement of every
  portion before any delivery, CR 120.4b/120.4c; held riders, CR 614.6 + CR 615.5; deferred APNAP
  choices, CR 616.1 + CR 101.4/101.4d; capped-redirect split parts, CR 614.9 + CR 614.5; shared-shield
  wait, CR 615.7; effect-stated outcomes settled together, CR 104.3f + CR 104.4a + CR 810.8a; CR 104.1
  and CR 800.4a stops) stays the runner for `EachSourceDealsDamage`, `EachDealsDamageEqualToPower`,
  `DealDamage { damage_source: EachTarget }` and `Fight`. Main's per-node stash representation of a
  paused multi-source batch (`ApplyPostReplacementDamage` + raw `DealDamage` chain) cannot carry held
  riders, deferred choices, split parts, outcomes or the answered-portion hand-back, and was already
  replaced by the branch before #9670 (#9670 only edited those helpers), so it is not restored.
  `Effect::ResumeDamageBatch` stays as the parked batch's node (Q2).
- **Ownership mapping.** `run_damage_batch` and `park_damage_batch` return `Result<DamageTail,
  EffectError>`: `park_damage_batch` returns `Caller` when it inserted the batch at the child boundary
  (a rider's frames were raised; the chain's child-boundary arm places the tail beneath it) and
  `Stashed` when it parked the tail beneath the batch itself (`ability.sub_ability.is_some()`), else
  `Caller`. Completion returns `Caller`.
- **Per-recipient loops keep the branch's rider placement.** In `resolve_damage` (distributed and
  targeted), `resolve_all_damage`, `resolve_each_player_damage`: a rider that left its prompt standing
  (`replacement::damage_batch_has_paused_rider`) parks each remaining recipient in its own frame beneath
  the rider (`park_recipients_behind_paused_rider`) and returns `Caller`; a `NeedsChoice` stashes the
  remaining recipients plus the tail (`stash_remaining_damage_chain` / `stash_remaining_distribution`)
  and returns `Stashed`. Main's `pause_behind_continuation_prompt` arms (same trigger, but appending the
  remaining chain into the active frame) are replaced by these.
- **One tail builder.** The branch's `cast_tail_with_parent_targets` body (parent targets, chain context
  hand-off, and the Searing Blaze "instead" base-clause binding) under main's name `stashed_tail`; the
  branch's `keep_chain_identity` on every remaining node (context, ability index, settled guards,
  parked-batch reset); main's `carry_instruction_context` deleted.
- **Batch opening.** Each table entry opens its damage batch: the public wrappers keep the branch's
  `replacement::with_damage_batch` and return `Result<(), _>` via `.map(|_| ())`; the table calls the
  `DamageTail`-returning bodies through the same wrapper.

**Deleted:** `deal_damage::damage_resolver_owns_tail`, `parked_batch_owns_tail`, `report_parked_tail`
(its `mark_pending_continuation_parent` call is kept inline where the branch had it — that tag is the
paused instruction's `EffectResolved` re-emission, not ownership), the `resolve_chain_body` call to
`damage_resolver_owns_tail`, the `damage_rider_resident_before` capture and the branch's child-boundary
alternative `(!damage_rider_resident_before && state.post_replacement_event_source().is_some())`;
main's `pause_behind_continuation_prompt`, `build_post_replacement_damage_node`,
`stash_remaining_post_replacement_damage`, `stash_each_source_combined_continuation`,
`stash_remaining_each_source_damage`, `stash_remaining_each_deals_chain`, `apply_batch_survivors`,
`apply_batch_survivors_before_replacement_pause`, `carry_instruction_context`, and main's bodies of
`resolve_each_target_power_damage`, `resolve_each_deals_power_damage`, `resolve_each_source_damage`.

---

## 3. Sizing

Units (one coherent behaviour each, one skill-checklist pass):

| Unit | Behaviour | Registration surfaces | Discriminating test |
|---|---|---|---|
| U1 Mechanical integration | merge resolves; `LossReason` through `lose_by_effect` / `win_by_effect` / `settle_event_outcomes`; `LazyBoxedVec` union; size note; duplicate fields; imports; loop-ring removal | elimination.rs, win_lose.rs, game_state.rs, game_state_size.rs, engine.rs, resolution.rs, targeting.rs, two test files | V1.7 (2HG win+lose → EffectStatedLoss reasons); GameState size test; Kathril |
| U2 Seam 1 amount | one installer (`ReplacedEventContext` + `replaced_event_amount`), one reader (scope) | replacement.rs, quantity.rs, game_state.rs | V1.5 unit test over every kind; V1.1 Shockmaw |
| U3 Seam 2 predicate | `ShieldKind::Substitution` folded into `is_damage_substitution` | phase-ast ability.rs, oracle_replacement.rs, oracle_tests.rs, replacement.rs, coverage.rs, deal_damage.rs (test), protocol record | V2.1 parse shape + predicate over the 10 cards; V2.4 Pariah+Force Bubble asks |
| U4 Seam 3a combat legs | main's pass + ledger/guards + CR 104.1/800.4a legs + earlier-rider pause; auto-settle deleted | replacement.rs, combat_damage.rs, game_state.rs | V3.1–V3.6, V3.10, V3.11 |
| U5 Seam 3b Phase D asks | aggregate rider pauses park the record; combat fence deleted | combat_damage.rs, replacement.rs, coverage.rs | V3.7, V3.8 |
| U6 Seam 4 tail | `DamageTail` for every damage instruction incl. two-phase batches, Fight, ResumeDamageBatch | deal_damage.rs, fight.rs, effects/mod.rs | V4.1–V4.6 |

Dependency edges: U1 → every unit (green tree). U4 → U5 (shared `drain_combat_damage_rider`). U2, U3,
U6 are independent of each other and of U4.

Expected scope paths (counting rule: test fixtures, regenerated data and protocol-version surfaces
excluded; tests count):

Product: (1) crates/engine/src/game/combat_damage.rs, (2) crates/engine/src/game/effects/deal_damage.rs,
(3) crates/engine/src/game/effects/mod.rs, (4) crates/engine/src/game/effects/win_lose.rs,
(5) crates/engine/src/game/effects/fight.rs, (6) crates/engine/src/game/elimination.rs,
(7) crates/engine/src/game/engine.rs, (8) crates/engine/src/game/quantity.rs,
(9) crates/engine/src/game/replacement.rs, (10) crates/engine/src/game/coverage.rs,
(11) crates/engine/src/types/game_state.rs, (12) crates/engine/src/types/game_state_size.rs,
(13) crates/engine/src/types/resolution.rs, (14) crates/engine/src/game/targeting.rs,
(15) crates/phase-ast/src/types/ability.rs, (16) crates/phase-parser/src/parser/oracle_replacement.rs,
(17) crates/phase-parser/src/parser/oracle_tests.rs, (18) scripts/post-replacement-continuation-baseline.txt
(regenerated from source by the checked-in census; counted once).
Tests: (19) crates/engine/tests/integration/damage_substitution_replacements.rs,
(20) crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs,
(21) crates/engine/tests/integration/ironscale_hydra_combat_prevention.rs,
(22) crates/engine/tests/integration/swans_prevention_followup.rs,
(23) crates/engine/tests/integration/spell_resolution_final_part_after_paused_instruction.rs.
Excluded: protocol/changes/c85ad0f6-….json (deleted), protocol/changes/ccf4c183-….json (reason edit),
generated protocol mirrors.
Possible compiler-forced additions (standing class): crates/engine/src/game/engine_replacement.rs,
crates/engine/src/game/effects/prevent_damage.rs, crates/engine/src/game/stack.rs,
crates/engine/src/game/visibility.rs, crates/engine/src/game/engine_tests.rs (struct literals and
signatures).

**Expected scope-path count: 23** (T1: 6 units; T2: 23 ≥ 13).

Charter note (for the phase-fit gate, not a phasing of this plan): the merge commit must resolve every
conflict hunk, and the hunks *are* seams 1, 3a and 4, so those cannot be deferred. A green-tree split
that exists: P1 = U1 + U2 + U4 + U6 + the merge (with the branch's `drain_combat_damage_rider`
auto-settle and its fence kept for Phase D only, and `ShieldKind::Substitution` coexisting with main's
predicate); P2 = U3 (fold; not conflict-forced, both predicates compile side by side); P3 = U5 (Phase D
asks; fence deletion). P1 alone still trips T2.

---

## 4. Building Blocks

- Main: `replacement::CombatDamageReplacementPass`, `replace_combat_damage_batch`,
  `settle_combat_damage_replacement`, `drain_combat_damage_continuation`, `reconcile_parked_combat_leg`,
  `repropose_combat_damage_leg`, `first_chooser_of_unstarted_combat_leg`,
  `combat_damage_recipient_has_left`, `lift_departed_combat_leg_choice`, `settled_survivors`,
  `pending_candidates_are_current`; `combat_damage::order_batch_by_affected_seat`,
  `run_combat_damage_replacements`, `resume_combat_damage_replacements`,
  `resume_pending_combat_lifelink`, `drain_combat_lifelink`, `fire_combat_prevention_riders`,
  `parked_combat_damage_awaits_replacement_choice`; `replacement::is_damage_substitution`,
  `damage_execute_substitutes_event_type`, `ReplacedEventContext`; `deal_damage::DamageTail`,
  `is_multi_event_damage_instruction`, `multi_event_damage_resolver`, `resolve_instruction`,
  `stashed_tail` (name); `effects::TailOwnership`, `retire_empty_replacement_frames`;
  `elimination::eliminate_players(.., DepartureTriggers::ScannedByCaller, ..)`,
  `eliminate_players_simultaneously(.., LossReason, ..)`, `LookedBackDeparture::collect`.
- Branch: `deal_damage::run_damage_batch`, `park_damage_batch`, `resolve_resume_damage_batch`,
  `park_recipients_behind_paused_rider`, `park_portions_behind_paused_rider`, `keep_chain_identity`,
  `remaining_damage_head`, `remaining_damage_chain`, `hold_riders_for_awaited_portion`,
  `return_answered_portion_to_batch`, `defer_choice_behind_earlier_players`,
  `settle_joined_damage_run`, `run_object_damage_batch`; `replacement::with_damage_batch`,
  `read_damage_guards`, `settled_damage_guards`, `damage_batch_has_paused_rider`, `drain_damage_rider`,
  `hold_damage_rider`, `damage_rider_placement`, `mandatory_post_drain`,
  `stash_post_replacement_continuation(state, drain, behind)`, `damage_batch_holds_riders`;
  `quantity::with_replacement_action_scope` and friends; `elimination::lose_by_effect`,
  `win_by_effect`, `settle_event_outcomes`, `event_outcomes`; `effects::dispatch_resident_post_replacement_work`.
- New helper `replacement::replaced_event_amount`: closest authority is main's inline
  `substituted_damage_amount` match at the stash site; it is extended (same site, same inputs) to the
  prevention and life-substitution kinds rather than keeping per-applier thread-local stamping.

## 5. Logic Placement

| Behaviour | Owner | Callers that delegate |
|---|---|---|
| amount a replacement's action owns | `replacement::replaced_event_amount` → `ReplacedEventContext.amount` → `PostReplacementDrain::event_amount` | `apply_single_replacement` (both arms), `continue_replacement_impl` (optional Execute branch) |
| reading "that many" | `quantity::replacement_action_amount` (scope bound in `dispatch_pending_post_replacement_effect`) | `quantity::event_context_amount` only |
| "is this a damage substitution" | `replacement::is_damage_substitution` / `damage_execute_substitutes_event_type` | `damage_done_applier`, `candidate_materiality`, `is_damage_prevention_replacement`, the CR 120.8 candidate gate, `hold_damage_rider`, `replaced_event_amount`, `coverage::extract_damage_replacement_choice_feature` |
| combat replacement pass, guards, CR 104.1/800.4a legs | `replacement::replace_combat_damage_batch` | `combat_damage::run_combat_damage_replacements` (fresh and resumed) |
| combat rider drain (leg and Phase D) | `replacement::drain_combat_damage_rider` → `Option<WaitingFor>` | `drain_combat_damage_continuation`, `combat_damage::fire_combat_prevention_riders` |
| Phase D park/resume | `combat_damage::drain_combat_lifelink` | `apply_combat_damage_survivors`, `resume_combat_damage_replacements` |
| tail ownership | `deal_damage::multi_event_damage_resolver` → `DamageTail`; `effects::resolve_effect_step` | `resolve_chain_body` (only `TailOwnership`); `fight::resolve` body via the table |
| effect-stated outcomes | `elimination::lose_by_effect` / `win_by_effect` / `settle_event_outcomes` | `effects::win_lose::resolve_lose` / `resolve_win`, `deal_damage::settle_event_losses` |

Changed cardinality: `fire_combat_prevention_riders` now may stop mid-tally (its caller parks);
`drain_damage_rider` no longer branches on combat (its in-combat caller, the substitution arm's
earlier-rider drain, now reports a pause the leg parks on); `Fight` now resolves through the table
(its tail is owned by `DamageTail`, not by `parked_batch_owns_tail`).

## 6. Rust Idioms

- `DamageTail` (main's two-variant enum) instead of the branch's frame-tag inference.
- `Option<WaitingFor>` return for the combat rider drain and Phase D (no out-parameter, no bool).
- Deleting `ShieldKind::Substitution` removes a variant whose meaning is fully derivable from the typed
  `execute`/`damage_modification`/`shield_kind` fields already on `ReplacementDefinition`; every match
  stays exhaustive (no wildcard added).
- `replaced_event_amount` is a pure fn over typed inputs returning `Option<u32>`; no thread-local.
- `LazyBoxedVec<T>`: one type, `#[derive(Debug, Clone, PartialEq, Eq)]`, the branch's `From<Vec<T>>`,
  `Serialize`, `Deserialize` impls and main's `iter`, `iter_mut`, `push`, `remove`, `swap_remove`,
  `clear`, `take` methods, one doc comment covering both uses.
- `settle_event_outcomes` builds `BTreeMap<PlayerId, BTreeSet<LossReason>>` (main's type) instead of a
  `Vec<PlayerId>`.

## 7. Nom Compliance

`crates/phase-parser/src/parser/oracle_replacement.rs` changes only by deleting the two
`.substitution_shield()` builder calls in `parse_damage_to_self_instead_followup_with_provenance` and
`parse_damage_to_player_instead_followup` (the definitions keep `ReplacementDefinition::new(
ReplacementEvent::DamageDone)`, whose default `shield_kind` is `ShieldKind::None`), and updating the
tests that assert `ShieldKind::Substitution`. No detection, dispatch or classification step is added or
changed; the existing nom spines (`nom_on_lower` + `tag`/`alt`/`take_until`) are untouched. No Oracle
text is newly accepted.

## 8. Extension vs Creation

Extends: main's combat pass (two fields, three guards, one park point), main's `DamageTail` table
(three entries), main's `ReplacedEventContext` (amount kinds), main's predicate (seven more cards).
Creates: one private fn (`replaced_event_amount`). No new type, enum, variant, `WaitingFor` or
`GameAction`. Deletes one variant and roughly a dozen functions per side.

## 9. Analogous Trace

Traced main's #9670 lifelink-gain park end to end, the model the Phase D park copies:
`combat_damage::apply_combat_damage` → `run_combat_damage_replacements` →
`apply_combat_damage_survivors` → `drain_combat_lifelink` (`Err(ReplacementDeferred::ReplacementChoice)`
arm: `state.pending_combat_lifelink = Some(Box::new(record))`, `CombatDamageBatch::Paused`) →
`combat_damage::{first-strike/regular arms}` surface `waiting_for` → answer →
`engine_replacement::handle_replacement_choice` (combat routing first, then generic) →
`resume_pending_combat_lifelink` (also reached on `Priority` from `combat_damage` re-entry and
`turns::auto_advance_once`) → `resume_combat_damage_replacements` (`replacement: None` →
`drain_combat_lifelink`) → `finish_combat_damage_sub_step`. Types:
`types/game_state.rs::{PendingCombatLifelink, PendingCombatDamageReplacement,
PendingCombatLegProgress, PendingCombatLegChoice}`.

Traced the branch's noncombat batch for seam 4: `effects::resolve_effect_step` (legacy adapter) →
`deal_damage::resolve_each_source_deals_damage` → `with_damage_batch` → `run_damage_batch` →
`replacement::replace_event` / `drain_damage_rider` / `hold_damage_rider` → `park_damage_batch`
(`Effect::ResumeDamageBatch`, `SpellContext.parked_damage_batch`) → answer →
`engine_replacement::handle_replacement_choice_inner` (`hold_riders_for_awaited_portion`,
`return_answered_portion_to_batch`, `defer_choice_behind_earlier_players`) →
`effects::drain_pending_continuation` → `resolve_resume_damage_batch` → `run_damage_batch`; tail via
`effects::resolve_chain_body` (`damage_resolver_owns_tail`, `parked_batch_owns_tail`). And main's
`DamageTail`: `effects::resolve_effect_step` multi-event arm → `deal_damage::resolve_instruction` →
`resolve_damage` (`stash_remaining_damage_chain` → `remaining_damage_head` → `stashed_tail`) →
`TailOwnership::Transferred` → `resolve_chain_body` early return.

## 10. Variant Discoverability

No enum variant is added. One is deleted (`ShieldKind::Substitution`); its sibling set
(`None, Regeneration, Prevention, Redirection, DamageReplacementOneShot, PreventionOneShot`) is
otherwise unchanged. Run `cargo engine-inventory` after U3 to regenerate the gitignored
`data/engine-inventory.json`; no `/add-engine-variant` gate applies to a deletion.

---

## 11. Verification Matrix with Identity / Provenance

Production entry points follow `/card-test`: spells through `GameRunner::cast` / `act(CastSpell)`,
combat through `DeclareAttackers`/`DeclareBlockers`/`PassPriority` actions, answers through
`GameAction::ChooseReplacement` / `SelectCards` / the optional-effect answer. Completion boundary for
every row: `WaitingFor::Priority` with an empty stack and empty `resolution_stack`,
`pending_replacement.is_none()`, `pending_combat_lifelink.is_none()` (combat: phase `EndCombat` or the
CR 510.3 window), unless the row asserts a game end.

| Row | Claim (CR) | Seam / owner | Test (file::name; new or updated) | Revert-failing assertion | Paired positive reach-guard | Hostile / sibling cases | Provenance |
|---|---|---|---|---|---|---|---|
| V1.1 | A replacement's action reads its own amount inside a resolving trigger (CR 614.6, CR 615.5, CR 603.12) | `replaced_event_amount`, scope reader | damage_substitution_replacements::a_replacements_action_reads_its_own_amount_inside_a_resolving_trigger (unchanged) | Test of Faith Bear and Phytohydra each 1 counter (not 4) | P1 life 16 (Shockmaw dealt 4) | Dralnu sacrifices 1 of 2 lands; Scales+Season order 4/4 | authority: drain `event_amount` bound at apply; snapshot; storage drain → scope; consumer `replacement_action_amount` |
| V1.2 | Amount outranks a d20 (CR 706.4) | same | farideh_fireball_die_branch_selection rows (unchanged) | Delaying Shield 2 delay counters, Force Bubble 2, Nefarious Lich asks 2 | die face ≠ 2 recorded | Doubling Season+Pir order | same |
| V1.3 | "that many" fixed across links (CR 614.1a) | same | szadek_combat_damage_to_counters::counter_modifier_does_not_change_mill_amount, soul_scar_mage_damage_to_counters::trigger_damage_uses_replaced_amount_not_trigger_event_amount (unchanged) | mill = replaced amount | counters placed | `mill_amount_holds_when_no_counter_can_be_put` | same |
| V1.4 | Amount survives a child prompt (CR 608.2c) | `PendingContinuation::replacement_action` | szadek::counter_order_inside_the_substitute_parks_the_batch_both_orders; effects mod test impossible_parked_draw_preserves_replacement_action_continuation_amount (unchanged) | mill after the order prompt = 5/10 | prompt surfaced | both orders | binding at dispatch; re-entered on drain |
| V1.5 | One installer covers every amount-owning kind (CR 614.6, CR 615.5, CR 120.8) | `replaced_event_amount` | NEW replacement.rs unit `replaced_event_amount_covers_each_amount_owning_kind` | prevention partial (5 → 2) gives `Some(3)`; substitution `Some(5)`; Furnace/redirect `None`; 0 prevented `None`; LifeGain/LifeLoss substitution `Some(n)` | each case's def is found (`replacement_definition_for_id` non-None) | PreventionMinus, PreventionOneShot, Prevention::Next exceeded | — |
| V1.6 | Optional accepted substitution carries amount, decline none | `continue_replacement_impl` | soul_scar::optional_substitution_accept_puts_counters_and_decline_deals_damage, szadek::optional_substitution_accept_performs_counters_and_mill_decline_deals_damage (unchanged) | accept: counters = damage; decline: damage dealt | prompt offered | both answers | declined branch: `ReplacedEventContext` all `None` |
| V1.7 | Effect-stated losses carry `LossReason` (CR 104.3e, CR 104.2b, CR 104.3f, CR 104.4a, CR 810.8a) | `elimination::settle_event_outcomes` | damage_substitution_replacements::a_two_headed_giant_team_that_wins_and_loses_in_one_batch_loses, a_player_who_wins_and_loses_in_one_batch_loses (updated: also assert `GameEvent::PlayerLost` reasons) | losers' events carry `EffectStatedLoss`; winner's opponents `OpponentWon`; a player in both sets carries both reasons | the batch dealt its damage | Two Nefarious Lich losses draw (CR 104.4a); immediate path (Flame Rift + Lich) `EffectStatedLoss` | reasons attached at settle time |
| V2.1 | The 10 substitutions parse to one shape and satisfy one predicate (CR 614.1a) | parser + `is_damage_substitution` | oracle_replacement.rs tests (5 asserts) and oracle_tests.rs:26804 updated to `ShieldKind::None` + `DamageDone` + execute kind; NEW engine unit `printed_damage_substitutions_satisfy_the_predicate` parsing the 10 Oracle texts | predicate true for all 10 | the parse produced a `DamageDone` replacement with `execute` | Battletide Alchemist (prevention, false), Furnace of Rath (`damage_modification`, false), Eye for an Eye-class `DealDamage` execute (false), an `Unimplemented` execute (false) | — |
| V2.2 | Substitution ignores "can't be prevented" (CR 615.12, rulings S1–S4) | `is_damage_prevention_replacement` | damage_substitution_replacements::phytohydra_replaces_damage_preventable_or_not, soul_scar::damage_cant_be_prevented_does_not_stop_the_substitution (unchanged) | counters under Stomp | Stomp resolved (prevention disabled) | Delaying Shield under Stomp | — |
| V2.3 | Nothing counts as prevented (CR 615.13) | applier | damage_substitution_replacements::lifelink_has_nothing_to_gain_from_replaced_damage (unchanged) | no `DamagePrevented`, life 20 | unreplaced reach: 23 | — | — |
| V2.4 | Substitution is order-material against every damage replacement (CR 616.1, rulings S7–S9) | `candidate_materiality` (Unconditional) | NEW damage_substitution_replacements::a_substitution_and_a_redirect_ask_their_order: Force Bubble + Pariah on P1, Lightning Bolt at P1 | `ReplacementChoice` for P1; Bubble first → 3 depletion counters, life 20, creature 0 damage; Pariah first → creature 3 damage, 0 counters | prompt candidates contain both sources | Phytohydra + Vigor (existing `a_substitution_is_ordered_against_a_prevention_shield`) | — |
| V3.1 | Combat order between a substitution and a modifier is asked (CR 616.1, CR 510.2) | main's pass | damage_substitution_replacements::combat_order_park_keeps_phytohydras_damage, combat_order_park_keeps_force_bubbles_damage (UPDATED, §13) | answer per order gives the exact value | prompt candidates | Stomp row | leg chooser = recipient's controller |
| V3.2 | Combat order between modifiers / prevention+modifier asked | main's pass | combat_order_park_keeps_a_damage_modifiers_damage, combat_order_parks_repeat_until_the_event_settles, combat_order_park_runs_a_prevention_rider_before_the_batch_drain, combat_order_park_keeps_a_shield_counter_replacement (UPDATED) | exact per-order values | prompt raised | CR 616.1f re-ask | — |
| V3.3 | Each leg's order asked, its rider drains in its own leg | main's pass + `drain_combat_damage_continuation` | combat_order_answer_drains_in_its_own_portion (UPDATED) | each Phytohydra gets its answered amount; Polukranos 4 or 2 | two prompts | `!has_post_replacement_drain()` at end | — |
| V3.4 | Rider's own order asked in combat (CR 616.1g) | `drain_combat_damage_rider` | ironscale_hydra_combat_prevention rider section and unpreventable::phytohydras_counter_order_is_settled_inside_the_batch (UPDATED, renamed `…_is_asked_inside_the_batch`) | Scales first 4, Season first 3 (Ironscale); 6/5 (Phytohydra blocking a 2/2) | one prompt with both enchantments | two Hydras: two prompts; Furnace + rider: damage order then rider order; double strike: one per step | — |
| V3.5 | Batch completes once after an asked rider; SBA run after (CR 704.3, rulings P7–P8) | main resume | ironscale::a_settled_rider_order_lets_the_next_turn_come → `an_answered_rider_order_lets_the_next_turn_come` (UPDATED) | Courser dies; Bolt kills Bears; next turn reached | answer accepted | — | — |
| V3.6 | Rider's own choice asked in combat (CR 608.2d, ruling S11) | per-leg park | unpreventable::dralnus_sacrifice_choice_is_cleared_inside_the_batch → `…_is_asked_inside_the_batch`; nefarious_lichs_exile_choice_…; outfitted_jousters_sacrifice_in_combat (UPDATED) | chosen permanents sacrificed / chosen cards exiled, P1 not losing | the choice prompt names the population | one attached Equipment: no prompt (positive row kept) | chooser = rider's controller |
| V3.7 | Aggregate (Phase D) rider choices asked (CR 615.5, CR 608.2d, rulings P3–P4) | `fire_combat_prevention_riders` park | unpreventable::channel_harms_may_is_settled_after_the_batch → `channel_harms_may_is_asked_after_the_batch`; gatta_and_luzzus_counter_order_…; a_lifelink_answer_then_gatta_and_luzzus_rider; new_way_forwards_draws_are_kept_under_abundance; a_may_park_and_a_riders_own_prompt_settle_in_one_batch (UPDATED) | accept: Hill Giant dealt 3; decline: Giant unhurt; G&L 5/6 by order | Phase D reached (DamagePrevented aggregate emitted) | two aggregate shields: second rider runs once after the first's answer (NEW row `a_paused_aggregate_rider_keeps_the_next_shields_rider_behind_it`: Channel Harm + Awe Strike, Awe Strike's gain happens once, after the answer) | cursor = record.prevention_tally remainder, parked in `pending_combat_lifelink` |
| V3.8 | A "may" on the rider's event asked (CR 616.1) | per-leg park | unpreventable::swans_of_bryn_argolls_draws_are_kept_under_abundance, double_strike_keeps_the_swans_draw_in_each_damage_step (UPDATED) | accept → revealed card to hand; decline → drawn | prompt for P0 per draw | double strike: per step | — |
| V3.9 | A combat "may" replacement asked (CR 616.1) | main's pass | damage_substitution_replacements::a_parked_may_gets_no_automatic_yes_in_combat → `a_parked_may_is_asked_in_combat` (UPDATED) | accept: P1 life 18; decline: 17 | noncombat reach row kept | — | — |
| V3.10 | Guards read once per combat batch across a park (CR 608.2f, CR 700.1) | pass `settled_guards` | NEW unpreventable::a_parked_combat_batch_keeps_its_guard_reading: P0's Undergrowth Champion with one +1/+1 counter attacks and is double-blocked by two P1 2/2s under Furnace of Rath; each leg's damage to the Champion asks P0 (its controller) the Champion-vs-Furnace order; P0 answers Champion first both times | both legs prevented (guard read Holds once) though the first leg's rider removed the only counter; the game is serialized at the second prompt and restored before answering | first prompt surfaced with both sources | false reading: Champion with no counter, no prompt, both legs dealt (4 + 4 = 8) | guard keyed by `ObjectIncarnationRef` + condition; bound at pass creation; stored in pass ↔ `PendingCombatDamageReplacement.settled_guards` |
| V3.11 | Earlier rider paused inside a combat leg, substitution behind it (CR 615.5, CR 608.2c) | `settle_combat_damage_replacement` | NEW damage_substitution_replacements::an_earlier_riders_paused_counters_finish_before_the_substitution_in_combat: Phytohydra (P1) under Test of Faith, Hardened Scales, Doubling Season blocks a 5/5; P1 orders Test of Faith first | after both prompts: counters = Test of Faith's 3 events + Phytohydra's 2 per order answers (asserted per answer, as the noncombat row does, with its #174 note); batch completes once | Test of Faith prompt surfaced first | Phytohydra-first order: one counter event of 5 | leg survivor `None`; queued drain behind paused frame |
| V3.12 | CR 104.1 ends the combat pass | pass loop guard | unpreventable::nothing_happens_in_the_batch_after_the_game_ends, nefarious_lich_ends_the_game_inside_the_batch, a_prevention_rider_that_ends_the_game_ends_the_batchs_riders (UPDATED only if APNAP order moves the ending leg, §13) | no event after `GameOver`; nothing parked | Lich portion dealt | lifelink-gain end row | — |
| V3.13 | Departed recipient not dealt (CR 800.4a, CR 800.4e) | unstarted-leg check | unpreventable::a_player_who_loses_inside_the_batch_leaves_and_the_batch_goes_on (UPDATED for asked order) | P1 dealt nothing after leaving; Exquisite Blood sees P2 only | P2 dealt 2 | — | — |
| V4.1 | Tail once behind remaining recipients (CR 608.2c, ruling P1) | `DamageTail` | soul_scar::sweep_tail_runs_once_after_the_counter_prompts, targeted_damage_tail_waits_for_every_counter_prompt, divided_damage_tail_waits_for_every_counter_prompt, single_recipient_counter_prompt_runs_the_tail_after_it, replacement_order_choice_runs_the_tail_once (unchanged) | one Blood / both can't-block / one draw / +3 once | prompts answered count | both orders | `Stashed` (stash carries tail) vs `Caller` (child-boundary insert) |
| V4.2 | Multi-source batch tail once (CR 120.4b, CR 608.2c) | `run_damage_batch` → `DamageTail` | soul_scar::multi_source_tail_waits_for_every_counter_prompt, multi_source_life_gain_order_runs_the_tail_once, targeted_sources_each_ask_their_counter_order, filtered_sources_each_ask_their_counter_order (unchanged) | draw once; 5+3 counters | two prompts | targeted and filtered | parked batch below/above tail per `park_damage_batch` |
| V4.3 | Held riders, deferred APNAP choices, split parts survive (branch claims) | `run_damage_batch` | damage_substitution_replacements batch rows (Allies at Last, Pit Fight, Rakdos Charm, Nomads en-Kor, Laboratory Maniac) (unchanged) | as on branch | — | — | `SpellContext.parked_damage_batch` |
| V4.4 | Meteor's tail keeps casting context (CR 601.2a, CR 608.2c) | `stashed_tail` / chain hand-off | ultimate_magic_meteor_per_opponent_destroy (unchanged) | destroy clause runs when cast from exile | counter prompt surfaced | cast from hand: nothing destroyed | `apply_parent_chain_context` |
| V4.5 | Fight's tail owned once (CR 701.14a, CR 608.2c) | table entry `Fight` | NEW damage_substitution_replacements::a_paused_fights_tail_runs_once: The Last Agni Kai, P0's 5/5 (under Test of Faith, P0 controls Furnace of Rath) fights P1's 2/2; P0 orders Furnace vs Test of Faith | after the answer P0's pool holds exactly 8 {R} (2/2 dealt 10, excess 8) | prompt surfaced before any mana | other order: same 8 {R}; reverting `Fight` to `Caller`-only → 16 {R} or mana before the answer | `park_damage_batch` parks tail beneath → `Stashed` |
| V4.6 | Soul-Scar Mage fight ruling (S10) | fight batch, held `WithEvent` action | NEW damage_substitution_replacements::soul_scar_mage_fight_replaces_one_direction: P0 controls Soul-Scar Mage; P0's 2/2 and P1's 4/4 fight (Pit Fight) | P0's 2/2 dealt 4 (dies), P1's 4/4 has two -1/-1 counters and 0 damage | Pit Fight resolved | without Mage: 4/4 dealt 2 | substitution held `RiderTiming::WithEvent` |

Coverage status impact: `damage_replacement_choice_unasked_in_combat` removed → Battletide Alchemist and
Channel Harm return to supported unless another tag holds them; Dralnu stays unsupported
(`damage_substitution_choice_incomplete`). `damage_substitution_choice_incomplete` now also reaches
Soul-Scar Mage, Szadek and Undead Alchemist (Q1). Coverage must be measured at the merged head
(`cargo coverage` / hosted job) and every flip listed in the PR.

---

## 12. Reference Readings

Rows whose expected value comes from another reading, each derived from the card first:

- **Main's #9670 suites "pass unchanged in intent"** (V1.3, V1.6, V4.1, V4.2, V4.4 and szadek /
  adamantoise / combat_hit_bookkeeping / awe_strike). Derivations, clause by clause, for the rows the
  merge can move:
  - Lightning Helix ("deals 3 damage to any target and you gain 3 life") at P1's Vigor-protected Hill
    Ogre under Soul-Scar Mage: one damage event, two applicable effects (Soul-Scar's substitution,
    Vigor's prevention) → P1 chooses (CR 616.1); either way the instruction then continues "you gain 3
    life" exactly once (CR 608.2c) → +3. Matches main's `replacement_order_choice_runs_the_tail_once`.
  - Vampires' Vengeance ("deals 2 damage to each non-Vampire creature. Create a Blood token.") under
    Soul-Scar Mage vs P1's three creatures with Doubling Season and Vizier of Remedies: each creature's
    damage becomes 2 -1/-1 counters, each counter event meets two replacements → three orders;
    Doubling Season first: 2×2−1 = 3 each; the Blood token is the next instruction, created once after
    the damage instruction completes (CR 608.2c) → 1 Blood, 3 counters each. Matches main.
  - Allies at Last with Birds of power 3 and 2 at the Great Wall under Soul-Scar Mage: each source's
    damage is substituted; two counter events, Doubling Season first: (2·3−1) + (2·2−1) = 8. Under the
    branch's batch the two substitutions are held until after delivery (CR 614.6: the substitute is the
    modified event; they still occur within the instruction before its tail), so the counts and the
    prompt count (2) are the same as main's. Reference measured by main's tests; derivation agrees.
  - Szadek + Mending Hands ruling: Mending Hands first → Szadek gets one counter (5 − 4). Matches main's
    `defender_orders_prevention_and_substitution_both_ways`.
  None of these derivations disagrees with main's measured reading, so no defective-reference route.
- **Branch suites "pass"** (V1.1, V1.2, V2.2, V2.3, V4.3): the branch's derivations stand; none is moved
  except the combat rows in §13, whose new values are derived there from the Oracle text.
- **Combat rows in §13** are derived from each card's text and CR 616.1, never copied from the branch's
  "set the player could reach" assertions.
- **Not derivable by the card:** the stable order of two same-seat combat legs (assignment order). It is
  an engine convention (main's `order_batch_by_affected_seat` stable sort), labelled as such where a
  row depends on it.

---

## 13. Tests that change expectation

Each row: card (verbatim Oracle in §0 or the test constant), CR, the branch's expectation, the corrected
expectation. Every updated row drives both answers where the player has two, and asserts the exact
value per answer; none is weakened to a set.

| Test (file::name) | Card / Oracle | CR | Branch expected | Corrected |
|---|---|---|---|---|
| damage_substitution_replacements::combat_order_park_keeps_force_bubbles_damage | Force Bubble + Furnace of Rath, unblocked 1/1 then 3/3 after Stomp | 616.1, 510.2 | engine picks; counters ∈ {1,2}; Stomp: graveyard or 3 | P1 asked; Bubble first → 1 counter (3/3: 3 counters), Furnace first → 2 (3/3: 6, sacrificed); life 20 |
| …::combat_order_park_keeps_phytohydras_damage | Phytohydra blocking 3/3 under Furnace | 616.1 | counters ∈ {3,6} | P1 asked; Phytohydra first 3, Furnace first 6; damage 0 |
| …::combat_order_park_runs_a_prevention_rider_before_the_batch_drain | Polukranos (6 counters) blocking 2/2 under Furnace | 616.1, 615.5 | left ∈ {4,2} | P1 asked; Polukranos first 4 left, Furnace first 2 left; one DamagePrevented |
| …::combat_order_park_keeps_a_shield_counter_replacement | Dapper Shieldmate (2 shield) under Furnace, ± Stomp | 616.1, CR 122.1c ruling | 1 shield; damage 0/4 | if P1 is asked, both answers give 1 shield and damage 0 (prevention possible) / 4 (Stomp); assert the prompt's presence as measured (§17 P1) |
| …::combat_order_park_keeps_a_damage_modifiers_damage | Torbran + Furnace, red 2/2 unblocked | 616.1 | lost ∈ {6,8} | P1 asked; Torbran first 8, Furnace first 6 |
| …::combat_order_parks_repeat_until_the_event_settles | Torbran, Furnace, Gratuitous Violence; then Force Bubble | 616.1f | lost ∈ {10,12,16} | P1 asked twice; each of the three orders asserted exactly; Bubble row: per answer |
| …::combat_order_answer_drains_in_its_own_portion | two Phytohydras each blocking a 1/1 under Furnace; Phytohydra + Polukranos | 616.1, 615.5 | ≥1 counter each; Polukranos ∈ {4,2} | two prompts in assignment order (same seat); each answered both ways; Polukranos 4 or 2 by answer; no drain left |
| …::two_force_bubbles_take_all_of_a_combat_batch_on_one | two Force Bubbles, three 1/1s unblocked | 616.1, ruling S6 | [0,3] | three prompts (one per source: the fenced gap, strict-failure note kept); answering the same Bubble each time gives [0,3]; life 20 |
| …::a_parked_may_gets_no_automatic_yes_in_combat → a_parked_may_is_asked_in_combat | Battletide Alchemist (X = 1), Ogre 3/3 unblocked | 616.1 | P1 takes 0 (skipped) | P1 asked; accept → life 18 (1 prevented, one DamagePrevented); decline → 17 |
| unpreventable::phytohydras_counter_order_is_settled_inside_the_batch → …_is_asked_inside_the_batch | Phytohydra blocking 2/2 under Scales+Season | 616.1g | engine's first candidate | P1 asked; Scales first 6, Season first 5; Bolt kills the 3/3 after |
| unpreventable::gatta_and_luzzus_counter_order_is_settled_after_the_batch → …_is_asked_after_the_batch | Gatta and Luzzu on Grizzly Bears, Scales+Season | 615.5, 616.1 (Phase D) | first candidate each counter (#174 note) | P0 asked per counter event (keep the #174 strict-failure note: counters parse one per damage); per-answer values 3/4 each |
| unpreventable::a_lifelink_answer_then_gatta_and_luzzus_rider | Child of Night, Rhox Faithmender, Leyline of Hope, Gatta and Luzzu | 616.1, 702.15b | one prompt (lifelink) | lifelink prompt, then rider counter prompts at Phase D; per-answer values |
| unpreventable::channel_harms_may_is_settled_after_the_batch → …_is_asked_after_the_batch | Channel Harm on Bears, Hill Giant blocking | 608.2d, 615.5 | "may" declined | P0 asked; accept → Hill Giant dealt 3 (dies, 3/3); decline → Giant unhurt; unblocked row: P1 takes 2, no prompt |
| unpreventable::a_may_park_and_a_riders_own_prompt_settle_in_one_batch → …_are_asked_in_one_batch | Swans + Abundance, Channel Harm | 616.1, 608.2d | Abundance declined, Channel Harm cleared | Abundance "may" per draw asked at the leg, then Channel Harm's "may" at Phase D; each answer's outcome asserted |
| unpreventable::swans_of_bryn_argolls_draws_are_kept_under_abundance → …_draws_ask_abundance | Swans blocks Bears; P0's Abundance | 616.1 | declined (draws kept) | P0 asked per draw; decline → card drawn; accept → chosen kind revealed to hand |
| unpreventable::double_strike_keeps_the_swans_draw_in_each_damage_step | Boros Swiftblade blocked by Swans; Abundance | 510.4, 616.1 | declined | one Abundance prompt per damage step; decline keeps the draw each step |
| unpreventable::new_way_forwards_draws_are_kept_under_abundance | New Way Forward (P1) on Bears; P1's Abundance | 615.5, 616.1 | declined | P1 asked per draw at Phase D; P0 dealt 2 once |
| unpreventable::nefarious_lichs_exile_choice_is_cleared_inside_the_batch → …_is_asked_inside_the_batch | Nefarious Lich, 4 graveyard cards, 3/3 unblocked | 608.2d | nothing exiled | P1 chooses 3 of 4; chosen exiled, P1 doesn't lose, life 20 |
| unpreventable::dralnus_sacrifice_choice_is_cleared_inside_the_batch → …_is_asked_inside_the_batch | Dralnu blocking 2/2, four other permanents | 608.2d, ruling S11 | nothing sacrificed | P1 chooses 2 permanents (Dralnu eligible); those sacrificed; Dralnu 0 damage |
| unpreventable::outfitted_jousters_sacrifice_in_combat | Jouster with two Equipment, blocked | 608.2d, 701.21a | none sacrificed with two | P0 chooses one of two; that one sacrificed; one Equipment: no prompt, it is sacrificed |
| unpreventable::a_player_who_loses_inside_the_batch_leaves_and_the_batch_goes_on | 3 players; Ironscale + Scales/Season | 800.4a, 800.4e, 616.1 | first candidate | P0 asked the counter order (3 or 4); rest unchanged |
| unpreventable::nothing_happens_in_the_batch_after_the_game_ends | Nefarious Lich ends at P0's Hill Giant leg | 104.1, 101.4 | later portions never dealt | legs ordered by APNAP of the affected player (main): legs damaging P0's blocked attackers (Jouster, Ironscale) rank before the P1 leg. Restage so every rider leg affects P1 (P1's blocking Jouster and Ironscale under P1's Scales/Season) and follows the Lich leg in assignment order; then the original assertions hold (§17 P7) |
| ironscale_hydra_combat_prevention rider section: ironscale_counter_order_is_settled_inside_the_batch, two_ironscale_hydras_each_settle_their_own_counter_order, a_damage_order_then_a_rider_order_in_one_portion, a_rider_order_of_three_or_four_effects_is_settled_in_one_legal_order, a_settled_rider_order_lets_the_next_turn_come, double_strike_settles_a_rider_order_in_each_damage_step, a_first_strike_lifelink_answer_then_a_regular_step_rider_order | Ironscale Hydra + Scales/Season (+ Furnace, Boros Swiftblade, Fiendslayer Paladin) | 616.1, 616.1g, 510.4 | `prompts.is_empty()`, counters ∈ {3,4} | renamed `…_is_asked…`; P0 asked; Scales first 4, Season first 3; Furnace row: damage order then rider order (two prompts); three/four-effect row: each order asserted; double strike: one rider prompt per step; Courser dies, Bolt kills Bears |

Any further failing test at verification is classified before it is touched: a combat prompt the
branch settled (rewrite as above, CR 616.1), an APNAP reorder (CR 101.4; restage, keep intent), a
materiality prompt between a substitution and another replacement (CR 616.1; answer both ways, §17 P1),
or a defect (fix the code). The classification goes into the commit message; no assertion is relaxed.

---

## 14. Step-by-step plan

### Step 1 — the merge commit (conflict resolution, built on main's shapes)

The merge commit contains Steps 1, 2 and 6 (items 1–3) and Step 4 item 2 (the ledger swap for main's
`combat_one_shot_applications` references), because those are forced by the conflict hunks or by the
tree compiling: the `resolve_chain_body` hunk removes the branch's tail inference, so every damage
instruction (including `Fight` and `ResumeDamageBatch`) must report `DamageTail` in the same commit,
and main's pass code does not compile without the ledger swap. The remaining design units (Steps 3,
4 rest, 5, 7, 8) are follow-up commits. Tests may be red between the merge commit and the follow-ups;
only the pushed head must be green.

Run `git merge upstream/main` on the branch in the executor's worktree. Resolve each hunk:

**crates/engine/src/game/combat_damage.rs**
- `apply_combat_damage` (hunk at merged line 2121): take main's `order_batch_by_affected_seat` call and
  pass construction; drop the branch's `replace_combat_damage_batch` tuple call and its CR 104.1 guard
  (moved into `run_combat_damage_replacements`, Step 4).
- `drain_combat_lifelink` (hunk at 2429): take main's side (no `invalidate_loop_ring_on_unobserved_life_move`;
  #10810 removed it), wrapped in the branch's CR 104.1 guard `if state.game_end.is_none() {
  fire_combat_prevention_riders(..) }` (Step 5 changes its signature).

**crates/engine/src/game/effects/deal_damage.rs** (24 hunks)
- imports (24): union, minus symbols deleted in this step (`ResidentDrainPolicy` stays if
  `run_damage_batch` uses it).
- `apply_damage_to_target` (666): branch's `drain_damage_rider(state, events)` (Step 4 adds the
  `waiting_for` assignment inside `drain_damage_rider`).
- after `carry_instruction_context` (1337): keep main's `DamageTail`, `is_multi_event_damage_instruction`,
  `DamageInstructionResolver`, `multi_event_damage_resolver`, `resolve_instruction`; drop
  `build_post_replacement_damage_node`, `stash_remaining_post_replacement_damage`,
  `pause_behind_continuation_prompt`; delete `carry_instruction_context`.
- `remaining_damage_head` (2577): branch's body (`remaining_damage_chain(state, ability, ..)`), calling
  `stashed_tail`.
- `remaining_damage_chain`/`stashed_tail` (2629, 2647): one fn `stashed_tail` with the branch's
  `cast_tail_with_parent_targets` body and doc (incl. the Searing Blaze base-clause binding).
- `keep_chain_identity` region (2684): branch's `keep_chain_identity`, `named_damage_dealer`,
  `next_damage_clause`, `joins_next_damage_clause`, `settle_joined_damage_run`; delete
  `report_parked_tail` (inline its `mark_pending_continuation_parent` at its former call sites); drop
  main's `stash_each_source_combined_continuation`, `stash_remaining_each_source_damage`.
- `resolve` / `resume_resolution_division` (2909, 2945): branch's `with_damage_batch` wrapper around
  `resolve_damage(..)` with `.map(|_| ())`.
- fn header (3067): `fn resolve_damage(..) -> Result<DamageTail, EffectError>` with the branch body.
- `resolve_damage` loops (3153, 3177, 3193, 3221): branch's arms
  (`DamageResult::Applied(_) if damage_batch_has_paused_rider(state) && park_recipients_behind_paused_rider(..)? => return Ok(DamageTail::Caller)`,
  `DamageResult::Applied(_) => {}`, `DamageResult::NeedsChoice => { stash…; return Ok(DamageTail::Stashed) }`);
  every other early return in the fn returns `Caller` except the division pause (`Caller`, as main).
- `resolve_each_target_power_damage` (3730): branch's `run_damage_batch(..)` (returns `DamageTail`);
  drop main's Phase B/C body and `apply_batch_survivors*`.
- `resolve_all` / `resolve_all_damage` (3888): public `resolve_all` = `with_damage_batch(.., |s|
  resolve_all_damage(s, ..)).map(|_| ())`; `resolve_all_damage` (branch's `resolve_all_portions` body)
  returns `Result<DamageTail, _>`; the table entry calls a `with_damage_batch` wrapper returning
  `DamageTail`.
- DamageAll loop (4070): branch's rider-pause arm → `Caller`; `NeedsChoice` → `Stashed`.
- `resolve_each_player` (4408) and loop (4725): same pattern; keep the branch's remaining-amount
  pre-resolution and `mark_pending_continuation_parent(state, EffectKind::DamageEachPlayer)`; rider
  pause → `Caller`, `NeedsChoice` → `Stashed`.
- `resolve_each_deals_equal_to_power` (4851, 4901) and `resolve_each_source_deals_damage` (5000, 5162):
  public wrappers + `DamageTail`-returning bodies running the branch's `run_damage_batch`.
- after `object_power_with_lki` (5302): drop main's `stash_remaining_each_deals_chain`.

**crates/engine/src/game/effects/mod.rs**
- `resolve_legacy_effect` (15298): drop the branch's `ResumeDamageBatch` / `EachDealsDamageEqualToPower`
  / `EachSourceDealsDamage` arms; add `Effect::Fight { .. }`, `Effect::ResumeDamageBatch { .. }`,
  `Effect::ApplyPostReplacementDamage { .. }` to main's `unreachable!("multi-event damage dispatch
  precedes the legacy adapter")` arm and remove their legacy arms.
- `resolve_chain_body` (29596): main's `|| deal_damage::is_multi_event_damage_instruction(&ability.effect))`;
  drop the branch alternative. Also (clean region) delete the `damage_rider_resident_before` capture and
  the `if deal_damage::damage_resolver_owns_tail(..) { return Ok(progress); }` block.

**crates/engine/src/game/effects/win_lose.rs** — `resolve_lose` (82): `lose_by_effect(state, &[pid],
events)`; `resolve_win` (147): `win_by_effect(state, winner, events)`; comment `CR 104.2b + CR 104.3e`
(branch) plus main's CR 104.1 note if kept.

**crates/engine/src/game/elimination.rs** — imports: `use crate::types::ability::{Effect,
EventOutcomes}; use crate::types::events::{GameEvent, LossReason};`. Then (clean region, Step 2)
thread `LossReason`.

**crates/engine/src/game/engine.rs** — import list: main's list; remove any name the merged file no
longer uses (the branch removed `SubtreePosition`, so `PostReplacementDrainStack` is likely unused;
compiler decides).

**crates/engine/src/game/quantity.rs** — `event_context_amount` (8215): branch's
`.or_else(replacement_action_amount)` with its comment; drop main's tier.

**crates/engine/src/game/replacement.rs**
- imports (32): union (`HeldBatchRider`, `RiderTiming`, `PendingCombatLegChoice`,
  `PendingCombatLegProgress`, …).
- `mandatory_post_drain` vs main's `stash_post_replacement_continuation` body (1180): keep the
  branch's split (`mandatory_post_drain` → `hold_damage_rider` → `damage_rider_placement` →
  `stash_post_replacement_continuation(state, drain, behind)`), but `mandatory_post_drain` takes
  `replaced: ReplacedEventContext` (main's struct, kept with its doc) and sets
  `event_amount: replaced.amount`; delete the `REPLACEMENT_AMOUNT_STAMP` line.
- after `damage_done_applier` (3806): union — the branch's `read_damage_guards`,
  `settled_damage_guard_reading`, `settled_damage_guards`, `damage_batch_has_paused_rider`,
  `with_damage_batch`, `with_settled_damage_guards`, and main's `damage_execute_substitutes_event_type`,
  `is_damage_substitution` (made `pub(crate)`).
- `replace_combat_damage_batch` doc (15534): main's doc; append one sentence on the ledger.
- body (15572, 15603): main's body; replace `state.combat_one_shot_applications` handling with the
  ledger install/restore (Step 4).
- after the body (main's `reconcile_parked_combat_leg` … `drain_combat_damage_continuation`): keep.
- `continue_replacement_impl` (16672): main's `ReplacedEventContext`-filled drain, passed through the
  branch's `hold_damage_rider` (damage event) and installed with `ResidentDrainPolicy::Replace`; amount
  from `replaced_event_amount` (Step 3).

**crates/engine/src/types/game_state.rs** — `LazyBoxedVec` (287, 320): one type as in §6.

**crates/engine/src/types/game_state_size.rs** (128): keep both histories: the branch's paragraph
(boxed `damage_batch_ledger`) and main's entries through the 16,384 ceiling, then append a new
paragraph after measuring the merged head (Step 7).

**scripts/post-replacement-continuation-baseline.txt** — regenerate with
`python3 scripts/post_replacement_continuation_census.py --write` after Step 6.

### Step 2 — U1 mechanical (clean-merge breakages, §15)

- `crates/engine/src/types/game_state.rs`: one `PostReplacementDrain::event_amount` field (doc per
  seam 1, `#[serde(default)]`); one `event_amount: None` in `PostReplacementDrain::new`.
- `crates/engine/src/types/resolution.rs` (`LegacyReplacementTailsWire` migration), 
  `crates/engine/src/game/targeting.rs` (two test literals), `crates/engine/src/game/replacement.rs`
  test literal, `crates/engine/tests/integration/swans_prevention_followup.rs`,
  `crates/engine/tests/integration/spell_resolution_final_part_after_paused_instruction.rs`: one
  `event_amount: None` each.
- `crates/engine/src/game/elimination.rs`: `lose_by_effect` immediate path →
  `eliminate_players_simultaneously(state, losers, LossReason::EffectStatedLoss, events)`;
  `win_by_effect` immediate path → `eliminate_players_simultaneously(state, &opponents,
  LossReason::OpponentWon, events)`; `settle_event_outcomes` builds
  `BTreeMap<PlayerId, BTreeSet<LossReason>>`: each effect loser and each CR 810.8a shared-life teammate
  added for it → `EffectStatedLoss`; each opponent of a winner who is not an effect loser →
  `OpponentWon` (inserted into the same set, so a player in both carries both); then
  `eliminate_players(state, &losses, DepartureTriggers::ScannedByCaller, events)` and, as
  `eliminate_players_simultaneously` does, `outcome.looked_back.collect(state, events, departure_end)`.
  CR annotations: 104.3e, 104.2b, 104.3f, 104.4a, 810.8a.
- Compile the merged tree (`cargo check --workspace --all-targets` via Tilt `clippy`, or isolated
  target dir) and fix every remaining compiler-forced site (struct literals, signatures) without
  changing behaviour; list each in the commit message.

### Step 3 — U2 seam 1 (follow-up commit)

`crates/engine/src/game/replacement.rs`:
1. Add `fn replaced_event_amount(state: &GameState, rid: ReplacementId, before: &ProposedEvent, after:
   Option<&ProposedEvent>) -> Option<u32>` (`after == None` means prevented/replaced wholesale) with
   the cases of seam 1; doc `/// CR 614.6 + CR 615.5: the amount the replacement's action owns …`.
2. In `apply_single_replacement_inner` `Modified` arm: `ReplacedEventContext { source: None (as
   today), target: proposed_event_target.clone(), amount: replaced_event_amount(state, rid, &proposed,
   Some(&new_event)) }`; `Prevented | PreventedBehindRider` arm: `amount: replaced_event_amount(state,
   rid, &proposed, None)`; delete `substituted_damage_amount`.
3. `continue_replacement_impl` optional branch: for `ReplacementBranch::Execute` on a damage event,
   `amount: replaced_event_amount(state, rid, &proposed, None)` when the def is a substitution (the
   Execute branch replaces the event), else as computed for the accepted kind; decline → all `None`.
4. Delete `REPLACEMENT_AMOUNT_STAMP`, `stamp_replacement_amount` (six sites → direct
   `state.last_effect_count = Some(u32_to_i32_saturating(..))`), and merge
   `apply_single_replacement`/`apply_single_replacement_inner`.
5. `crates/engine/src/types/game_state.rs`: delete `GameState::post_replacement_event_amount`.
6. Add the V1.5 unit test.

### Step 4 — U4 seam 3a (follow-up commit)

`crates/engine/src/game/replacement.rs`:
1. `CombatDamageReplacementPass`: add `pub(crate) settled_guards: Vec<SettledDamageGuard>`;
   `new(state: &GameState, events)` sets it from `read_damage_guards(state)`.
2. `replace_combat_damage_batch`: on entry `let restore_ledger = state.damage_batch_ledger.replace(
   Box::new(DamageBatchLedger { applications: take(&mut pass.one_shot_applications), settled_guards:
   take(&mut pass.settled_guards), ..Default::default() }))`; on exit move `applications` and
   `settled_guards` back into the pass and restore. Remove every `combat_one_shot_applications`
   reference (`drain_combat_damage_continuation`'s suspension becomes the ledger suspension inside
   `drain_combat_damage_rider`).
3. Loop top: `if state.game_end.is_some()` → settle all non-settled legs `None`, clear a surfaced
   `pending_replacement`, `break Complete` (CR 104.1 annotation).
4. In the `start_leg` path: an `Unstarted` leg whose event satisfies `combat_damage_recipient_has_left`
   settles `None` without proposing (CR 800.4a + CR 800.4e).
5. `settle_combat_damage_replacement`: after mapping the result, if
   `state.damage_batch_ledger.as_ref().and_then(|l| l.paused_rider).is_some()` → clear `paused_rider`
   and return `ContinuationPaused { survivor, waiting_for: Box::new(state.waiting_for.clone()) }`
   (CR 615.5 + CR 608.2c).
6. `drain_combat_damage_rider(state, events) -> Option<WaitingFor>`: take ledger, call
   `engine_replacement::apply_pending_post_replacement_effect(state, None, None, None, events)`,
   restore ledger, return. `drain_combat_damage_continuation` calls it; when `state.game_end.is_some()`
   afterwards it returns `Settled(survivor)` (CR 104.1).
7. `drain_damage_rider`: delete the combat routing; on pause also set `state.waiting_for =
   pause.waiting_for.clone()` (main's explicit assignment).
8. Delete `settle_combat_order_parks`, `settle_combat_rider_pauses`, `CombatRiderPause`,
   `answer_combat_rider_park`, `abandon_combat_rider_pause`.
9. Update `DamageBatchLedger::paused_rider` doc ("Combat settles every rider pause…" → combat parks the
   leg on it).

`crates/engine/src/types/game_state.rs`: `PendingCombatDamageReplacement.settled_guards` with serde
default/skip; doc CR 608.2f + CR 700.1.

`crates/engine/src/game/combat_damage.rs`: `run_combat_damage_replacements` parks
`settled_guards` into `PendingCombatDamageReplacement` and restores it in
`resume_combat_damage_replacements`; `Complete` arm returns `CombatDamageBatch::Complete(record.batch_events)`
when `state.game_end.is_some()`; `apply_combat_damage` builds the pass with `new(state, ..)`.

Tests: convert the per-leg rows of §13; add V3.10, V3.11.

### Step 5 — U5 seam 3b (follow-up commit)

`crates/engine/src/game/combat_damage.rs`:
1. `fire_combat_prevention_riders(state, tally: &mut Vec<(AppliedReplacementKey, i32)>, events) ->
   Option<WaitingFor>`: `while !tally.is_empty()` remove index 0; keep the CR 104.1 `break`; replace the
   branch's `replacement::drain_combat_damage_rider(state, events)` statement with `if let
   Some(waiting_for) = replacement::drain_combat_damage_rider(state, events) { if
   state.game_end.is_none() { return Some(waiting_for); } }`.
2. `drain_combat_lifelink`: after appending `damage_to_players`, `if state.game_end.is_none() { if let
   Some(waiting_for) = fire_combat_prevention_riders(state, &mut record.prevention_tally, &mut
   record.batch_events) { let events = record.batch_events.clone(); state.pending_combat_lifelink =
   Some(Box::new(record)); return CombatDamageBatch::Paused { events, waiting_for: Box::new(waiting_for)
   }; } }`. Doc: CR 510.2 + CR 615.5 + CR 608.2d.
3. `crates/engine/src/game/coverage.rs`: delete `StructuralFeature::DamageReplacementChoiceUnaskedInCombat`
   (enum arm, tag string, `FeatureSupport` arm), `RiderTargets`, `damage_replacement_reaches_combat`,
   `damage_replacement_asks_a_choice_in_combat`, `prevention_rider_asks_a_choice_in_combat`,
   `damage_rider_asks_a_choice`, `damage_rider_effect_asks_a_choice`, their two calls in
   `extract_ability_features_with_token_statics`, the combat half of
   `extract_damage_replacement_choice_feature`, and the two unit tests; drop now-unused imports
   (`SubAbilityLink`, `CombatDamageScope` if unused).
4. Tests: convert the Phase D rows of §13; add the V3.7 two-shield row.

### Step 6 — U6 seam 4 (items 1–3 in the merge commit; item 4 a follow-up commit)

1. `crates/engine/src/game/effects/deal_damage.rs`: `run_damage_batch`, `run_damage_batch_phases`,
   `park_damage_batch`, `run_object_damage_batch`, `resolve_resume_damage_batch` return
   `Result<DamageTail, EffectError>` per seam 4; `park_damage_batch` returns `Caller` after the
   child-boundary insert, `Stashed` after parking the tail beneath; every other early return in the
   runner returns `Caller`. Delete `damage_resolver_owns_tail` and `parked_batch_owns_tail`.
   `multi_event_damage_resolver` gains `Effect::Fight { .. } => Some(fight::resolve_instruction)`,
   `Effect::ResumeDamageBatch { .. } => Some(..)`, `Effect::ApplyPostReplacementDamage { .. } =>
   Some(|s, a, e| resolve_post_replacement(s, a, e).map(|()| DamageTail::Caller))`; update its doc
   ("every instruction that deals damage through the replacement pipeline").
2. `crates/engine/src/game/effects/fight.rs`: `pub(crate) fn resolve_instruction(..) ->
   Result<DamageTail, EffectError>` = `with_damage_batch(.., resolve_fight)`; `resolve_fight` returns
   the batch's `DamageTail` (and `Caller` on its early returns); public `resolve` = `.map(|_| ())`.
3. `crates/engine/src/game/effects/mod.rs`: done in Step 1; confirm `resolve_effect_step`'s
   multi-event arm comment names the full table.
4. Tests: add V4.5 and V4.6.

### Step 7 — U3 seam 2 (follow-up commit)

1. `crates/phase-ast/src/types/ability.rs`: delete `ShieldKind::Substitution` and
   `ReplacementDefinition::substitution_shield`.
2. `crates/phase-parser/src/parser/oracle_replacement.rs`: delete the two `.substitution_shield()`
   calls; update the parser tests to assert `ShieldKind::None`, `ReplacementEvent::DamageDone` and the
   execute's effect; `crates/phase-parser/src/parser/oracle_tests.rs:26804` likewise.
3. `crates/engine/src/game/replacement.rs`: substitution arm in `damage_done_applier` at Branch 3 keyed
   on `damage_execute_substitutes_event_type` with the branch body; `hold_damage_rider` timing via the
   predicate; CR 120.8 gate via `is_damage_substitution(repl_def)`; `is_damage_prevention_replacement`
   returns `false` for a substitution before the legacy description test (CR 615.1a annotation);
   materiality: remove `| ShieldKind::Substitution` (main's `Unconditional` arm handles it);
   `replacement_spends_shared_capacity`: remove the arm.
4. `crates/engine/src/game/coverage.rs`: fence calls `crate::game::replacement::is_damage_substitution`;
   delete the local fn.
5. `crates/engine/src/game/effects/deal_damage.rs` test (~6787): drop `.substitution_shield()`.
6. Delete `protocol/changes/c85ad0f6-12d8-45cc-ae35-edf384ac7f40.json`; edit
   `protocol/changes/ccf4c183-5976-4e18-905c-68b9afee9848.json`'s reason to add "a combat damage batch
   parked in its replacement pass carries its settled guards (PendingCombatDamageReplacement.settled_guards)";
   `node scripts/protocol-versions.mjs generate` then `check`.
7. Tests: V2.1 parser and engine units, V2.4.

### Step 8 — registration and docs

- `python3 scripts/post_replacement_continuation_census.py --write`; expected rows include
  `replacement.rs drain_combat_damage_rider consumer 1`, `replacement.rs drain_damage_rider consumer 1`,
  and none of main's `deal_damage.rs apply_damage_to_target/resolve_each_source_damage/
  resolve_each_target_power_damage` consumers nor `drain_combat_damage_continuation`. Review each row
  change in the commit message.
- `crates/engine/src/types/game_state_size.rs`: measure `size_of::<GameState>()` at the merged head and
  append the entry (must be ≤ 16,384).
- Test-module docs: replace the Command Zone #140 paragraphs in the three branch test files with the
  asked-choice description.
- PR body: rewrite the combat bullets (main's #9670 asks; fence removed), seam outcomes, the deletions,
  coverage flips, and the new known gaps list (Q1 if accepted).

---

## 15. Clean-merge breakages already visible (not in conflict markers)

- `PostReplacementDrain::event_amount` declared twice (`types/game_state.rs` merged lines ~31923 and
  ~31952) and `event_amount: None` twice in `PostReplacementDrain::new` (~32076/32078),
  `types/resolution.rs` (~7279/7285), `game/targeting.rs` (~5533/5535, ~5582/5584),
  `game/replacement.rs` tests (~22702/22704), `tests/integration/swans_prevention_followup.rs` (76/81),
  `tests/integration/spell_resolution_final_part_after_paused_instruction.rs` (3787/3789).
- `replacement.rs` `apply_single_replacement_inner` `Prevented` arm already calls the branch's
  `mandatory_post_drain` with main's `ReplacedEventContext` argument (hybrid auto-merge) — fixed by
  Step 1's `mandatory_post_drain` signature.
- Main's pass code references the deleted `GameState::combat_one_shot_applications` (replacement.rs
  merged ~15593, ~15850, ~15853, ~16141, ~16145) — Step 4.
- `elimination.rs` calls `eliminate_players_simultaneously` without `LossReason` (merged ~174, ~191,
  ~232) — Step 2.
- `LazyBoxedVec` defined by both sides — Step 1.
- Main's tests constructing `PendingCombatLifelink` / calling `replace_combat_damage_batch` and the
  branch's tests calling the old tuple signature — compiler-forced in Step 2/4.

---

## 16. Verification (at the merged head, after Step 8)

Tilt first (`tilt get uiresource clippy`), else direct, never both. `cargo fmt --all`;
workspace `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test -p phase-engine`
(all shards, default test stack) including `mutable_pupa_perpetual_keyword_mirror::kathril_reaches_matching_counter_and_tail_past_false_earlier_gates`
under the 16 MiB default stack and the `game_state_size` assertion (≤ 16,384);
`cargo test -p phase-parser --lib`; `cargo test -p phase-server`; `cargo test -p phase-ai --lib`;
wasm32 check; `scripts/check-engine-authorities.sh` (Gate B, draw-replacement producers, Gate D, the
continuation census); `node scripts/protocol-versions.mjs check`; coverage report at the merged head
with every support flip listed; `cargo engine-inventory`.

---

## 17. Unprobed assertions (settled at implementation by the named test)

- **P1** Main's `Unconditional` materiality for a substitution may add a CR 616.1 prompt in branch
  rows where a substitution meets another replacement (Pariah rows: `a_terminal_substitution_answer_waits_for_the_batch`,
  `an_automatic_redirect_waits_for_an_earlier_players_choice`, `redirected_damage_choices_follow_four_player_apnap_order`,
  `a_batch_keeps_its_order_through_an_answer_that_pauses_again`; Dapper Shieldmate + Furnace). A new
  prompt there is the correct reading (CR 616.1, rulings S7–S9); the row is updated to answer both ways.
- **P2** The settled guards travel correctly through main's leg parking (`PendingCombatLegChoice.pending`
  already carries `PendingReplacement.settled_damage_guards`) — V3.10.
- **P3** Removing auto-settle surfaces exactly the prompts §13 predicts — the §13 rows.
- **P4** `GameState` stays ≤ 16,384 B and the Kathril chain fits 16 MiB after adding
  `DamageTail` returns to the fight/batch frames — §16.
- **P5** The Last Agni Kai's excess-mana clause is supported at the merged head; if its coverage
  reports the clause unsupported, V4.5 uses Fight for the Throne ("… Then it fights target creature an
  opponent controls. When the creature an opponent controls dies this turn, if you control your
  commander, you become the monarch.") and asserts exactly one delayed trigger registered after the
  answer.
- **P6** Oozeavite and Underdark Beholder: whether their parsed `execute` now satisfies the structural
  predicate; if one does and its execute is not faithful, it must report unsupported (coverage), not
  become a supported substitution.
- **P7** Main's APNAP leg order moves which combat leg comes first in
  `nothing_happens_in_the_batch_after_the_game_ends` / `a_player_who_loses_inside_the_batch_leaves_and_the_batch_goes_on`.

---

## 18. Questions that need a human decision

- **Q1 (coverage regression vs honesty).** Folding the predicate makes Soul-Scar Mage, Szadek and
  Undead Alchemist carry `damage_substitution_choice_incomplete` (protection prevents ahead of the
  substitution's CR 616.1 choice, CR 702.16e; one choice per source where Force Bubble's ruling gives
  one). The gap is real for them on main too, unfenced. Recommendation: fence uniformly (three
  coverage-honesty drops, listed in the PR). Alternative: narrow the fence to cards whose recipient can
  be a player with several simultaneous sources — but protection applies to all twelve, so no narrowing
  is rules-clean. The acceptance line "no card that #10639 or main supports regresses" conflicts with
  the recommendation.
- **Q2 (`Effect::ResumeDamageBatch`).** The brief lists it as an example deletion. This plan keeps it as
  the parked two-phase batch's node and moves only tail ownership to `DamageTail`, because main's
  per-node stash cannot carry held riders, deferred APNAP choices, split parts, outcomes or the
  answered-portion hand-back that the PR's kept claims rest on. Deleting it means designing another
  carrier (a resolution frame), which is new machinery beyond this merge.
- **Q3 (`Effect::ApplyPostReplacementDamage`).** After the merge it has no production constructor (as
  on the branch since phase 3). This plan routes it through the `DamageTail` table and leaves it;
  deleting it is a separate protocol change.
- **Q4 (seam 1 meaning).** The plan widens main's "substitution only" meaning of `event_amount` to every
  amount-owning replacement and keeps the branch's dispatch-bound reader, deleting main's
  `post_replacement_event_amount()`. That departs from "main's event_amount meaning" in the maintainer
  note; the reason is the Test of Faith-in-a-trigger claim (V1.1). Confirm this is acceptable to the
  maintainer, or accept dropping that claim.
