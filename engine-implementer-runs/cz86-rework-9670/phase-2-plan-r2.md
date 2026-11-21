# Phase 2 plan, revision 2 — Every combat pause asked (cz86-rework-9670)

Mode: `engine-planner` **phase-plan mode** for charter phase **P2**. Inputs read: `task.md`,
`phase-charter` (frozen; P2 entry, deferral list, seam notes), `addenda/phase-2`,
`phase-inputs/phase-2`, `phase-inputs/run-level`, `phase-1-accepted-summary.md`,
`maintainer-answer-2026-11-17T1351.md`, `pr-body.md`, `class/`, the reviewed P2 plan r1
(`phase-2-plan.md`, clean after one text fix), and the executor's round-1 stop-and-return.

Attempt history:
- Pre-charter plan r1: 2 behavior + 1 text. Charter r1: 2 corrections (frozen).
- P1 plan r1: 2 behavior; r2: 1 text, then clean. P1 implementation review r1: 0 findings. P1 accepted.
- P2 plan r1: 1 text (applied), then clean.
- P2 executor round 1 implemented r1 in full, checkpointed as `8b16c4a374` on
  `engine/unpreventable-damage-riders`. Two converted rows stay red: the accept arms of C-D3
  (`channel_harms_may_is_asked_after_the_batch`) and C-D5
  (`a_may_park_and_a_riders_own_prompt_are_both_asked`). Its diagnosis: an accepted rider "may"
  deals 0, in combat and noncombat, because `OptionalEffectFrame` carries no replacement-action
  scope. The fix sites lie outside P2's `SCOPE_PATHS` and are not a standing class.
- This revision (design round 1 of the P2 plan loop) keeps everything r1 specified and the
  executor delivered, and adds that fix as a defective-reference repair: **D7 / U3** below.
  Budget: standard.

- `PHASE_BASE_SHA` = P1's accepted candidate **`00396db22d`**. Every "today" claim in §§2–3 was
  read at that commit. The D7 functions (`types/resolution.rs`, `effects/mod.rs`,
  `engine_payment_choices.rs`, `quantity.rs`, `engine_replacement.rs::dispatch_pending_post_replacement_effect`)
  have the same bodies at `8b16c4a374`, which touches none of them except a comment in
  `engine_replacement.rs`.
- **Delivered.** Commits A–C of §17 (U1, U2, D1–D6) are delivered as the single checkpoint
  `8b16c4a374`. The next executor round starts there and adds Commit D (U3). Nothing in A–C changes.
- **Probe status.** The planning worktree is read-only and no build was run, so **no claim in this
  revision was probed by running code.** The executor's round-1 runs (the red accept arms) are
  the only measurements. Each runtime claim is read from source and named by function, or listed
  in §18 (Unprobed) with the test that settles it. §3 lists the claims the executor establishes
  first, before any edit depends on them.
- Q1 is outcome A and touches only P3. Q2/Q4 were accepted in P1. P2 changes neither.

---

## 0. Premise (Step 0)

Oracle text read from MTGJSON `AtomicCards.json` `5.3.0+20261021`, verbatim:

- **Channel Harm**: "Prevent all damage that would be dealt to you and permanents you control this
  turn by sources you don't control. If damage is prevented this way, you may have Channel Harm deal
  that much damage to target creature." Rulings include: "The new damage is not combat damage, even
  if the prevented damage was." and the two-Channel-Harm loop "either player can break by choosing
  to not have Channel Harm try to deal damage" (class notes P3, P4).
- **Awe Strike**: "The next time target creature would deal damage this turn, prevent that damage.
  You gain life equal to the damage prevented this way."
- **Gatta and Luzzu**: "Flash / When Gatta and Luzzu enters, choose target creature you control. If
  damage would be dealt to that creature this turn, prevent that damage and put that many +1/+1
  counters on it."
- **Temper**: "Prevent the next X damage that would be dealt to target creature this turn. For each
  1 damage prevented this way, put a +1/+1 counter on that creature."
- **Phytohydra**: "If damage would be dealt to this creature, put that many +1/+1 counters on it
  instead."
- **Undergrowth Champion**: "If damage would be dealt to this creature while it has a +1/+1 counter
  on it, prevent that damage and remove a +1/+1 counter from this creature. / Landfall — …" Ruling:
  "If Undergrowth Champion is dealt damage from multiple sources at the same time (for example, if
  it's blocked by multiple creatures), its prevention effect will apply once to all that damage.
  You'll remove only one +1/+1 counter."
- **New Way Forward**: "The next time a source of your choice would deal damage to you this turn,
  prevent that damage. When damage is prevented this way, New Way Forward deals that much damage to
  that source's controller and you draw that many cards."
- **Swans of Bryn Argoll**: "Flying / If a source would deal damage to this creature, prevent that
  damage. The source's controller draws cards equal to the damage prevented this way."
- **Abundance**: "If you would draw a card, you may instead choose land or nonland and reveal cards
  from the top of your library until you reveal a card of the chosen kind. Put that card into your
  hand and put all other cards revealed this way on the bottom of your library in any order."
- **Hardened Scales**: "If one or more +1/+1 counters would be put on a creature you control, that
  many plus one +1/+1 counters are put on it instead." Ruling: "If two or more effects attempt to
  modify how many counters would be put on a creature you control, you choose the order to apply
  those effects, no matter who controls the sources of those effects." Also: "Each additional
  Hardened Scales you control will increase the number of +1/+1 counters placed on a creature you
  control by one."
- **Doubling Season**: "If an effect would create one or more tokens under your control, it creates
  twice that many of those tokens instead. / If an effect would put one or more counters on a
  permanent you control, it puts twice that many of those counters on that permanent instead."
- **Rhox Faithmender**: "Lifelink … / If you would gain life, you gain twice that much life
  instead." **Leyline of Hope**: "… If you would gain life, you gain that much life plus 1 instead.
  …"
- **Dralnu, Lich Lord**: "If damage would be dealt to Dralnu, sacrifice that many permanents
  instead. / {T}: …"; **Nefarious Lich**: "If damage would be dealt to you, exile that many cards
  from your graveyard instead. If you can't, you lose the game. / …"
- Vanilla bodies used by the new rows (all real, no rules text): **Grizzly Bears** 2/2, **Hill
  Giant** 3/3, **Centaur Courser** 3/3, **Rumbling Baloth** 4/4, **Eager Cadet** 1/1.
  **Child of Night**: "Lifelink". **Lightning Bolt**: "Lightning Bolt deals 3 damage to any
  target."
- **Shockmaw Dragon** (4/4): "Flying / Whenever this creature deals combat damage to a player, it
  deals 1 damage to each creature that player controls." (the maintainer's example in condition 3).
- Channel Harm's other rulings that bear on D7 (MTGJSON, verbatim): "Channel Harm's only target is
  the creature it may deal damage to. You choose that target as you cast Channel Harm, not at the
  time it prevents damage." and "You can choose a creature you control as the target. If you do,
  the damage Channel Harm deals to that creature won't be prevented."
- Census cards for U3's class (§1), verbatim: **Flitwing, Lyev Detective**: "Flying / If you would
  create one or more tokens, you may create that many Clue tokens instead."; **Jinnie Fay,
  Jetmir's Second**: "If you would create one or more tokens, you may instead create that many 2/2
  green Cat creature tokens with haste or that many 3/1 green Dog creature tokens with
  vigilance."; **Esix, Fractal Bloom**, **Moonlit Meditation**, **Mirrormind Crown**: "… you may
  instead create that many tokens that are copies of …"; **Remnants of Beleren**: "If you would
  empower Jace, you may instead put that many loyalty counters on a Jace you control."

CR numbers used in this plan, each grep-verified in `docs/MagicCompRules.txt`: 101.4, 104.1,
117.2e, 117.4, 120.8, 121.2, 510.1c, 510.2, 510.3, 510.3a, 510.4, 603.2, 603.2c, 603.3, 603.3b,
603.12, 608.2c, 608.2d, 608.2f, 609.7b, 614.1a, 614.5, 614.6, 615.1a, 615.5, 615.7, 615.13, 616.1,
616.1e, 616.1f, 616.1g, 700.1, 701.21a, 702.4b, 702.15b, 704.3, 800.4a, 800.4e. CR 615.5 in full:
"Some prevention effects also include an additional effect, which may refer to the amount of
damage that was prevented. The prevention takes place at the time the original event would have
happened; the rest of the effect takes place immediately afterward." 

## 1. Pattern Coverage (Step 0.5: the charter's attribution, cited, not re-run)

Charter record: `jq-1.8.1`, MTGJSON `5.3.0+20261021`, `class/query.sh`.
- **Class B** (prevention with additional effects): `class/prevention_rider_class.json`, 39 cards,
  110 distinct rulings, hash `a49c1dc6…e5de6b5`. P2 owns its **aggregate (Phase D) riders** and
  **every rider prompt inside a combat batch**: Channel Harm, Awe Strike, Temper, Test of Faith,
  Brace for Impact, Inkshield, New Way Forward, Deflecting Palm, Acolyte's Reward, Swans of Bryn
  Argoll, Outfitted Jouster and the rest of the 39 whose rider can pause.
- **Class C** (CR-defined combat replacement/prevention choices, CR 510.2 + CR 616.1): A ∪ B minus
  the three noncombat-only = 48. P2 owns the aggregate and in-leg rider asks of this class.
- **Guards** (charter decision 4; CR 608.2f + CR 700.1): every printed self-scoped "while" damage
  shield `read_damage_guards` reads (Undergrowth Champion, Polukranos, Oathsworn Knight, Ugin's
  Conjurant, Jared Carthalion, Outfitted Jouster; pr-body §2). The Undergrowth Champion ruling above
  is the bearing ruling.
- Bearing rulings attributed to P2 by the charter (class/notes.txt): **P1, P2** (riders
  "immediately afterward … before that spell or ability resumes"), **P3** (Channel Harm's damage is
  not combat damage), **P4** (Channel Harm's "may" is a real choice), **P7** (Test of Faith /
  Temper counters at the same time as the prevention), **P8** (state-based actions after the
  batch), **S11** (Dralnu's sacrifice is a choice). Each drives a matrix row in §12 (named there).

The pattern stop (Step 4) is assessed against the charter's class (39 / 48 cards), not P2's diff.
No row is built for one card: every behavior below is the combat batch's rider drain, park or
resume, which every class member with a pausing rider reaches.

**U3's class** (D7, the defective-reference repair). The charter's Class B attribution covers
Channel Harm, whose rider is the only one with a "you may" that reads the prevented amount. U3
fixes the building block: every "may" reached inside a replacement's own action that owns an
amount. Two censuses establish the printed class.

- Oracle census, MTGJSON `5.3.0+20261021`, first face of each card:
  `jq '.data | to_entries[] | .value[0] as $f | ($f.text // "") | select(test("(would|prevented this way|instead)[^.]*\\.[^.]*\\byou may\\b[^.]*\\b(that much|that many)\\b|\\byou may\\b[^.]*\\b(that much|that many)\\b[^.]*\\binstead\\b"; "i"))'`.
  The widened second pass anchors on `\b(prevent|instead)\b` plus "you may … that much / that
  many / equal to". Together they return Channel Harm plus the "you may instead … that many" cards
  (Flitwing, Jinnie Fay, Esix, Moonlit Meditation, Mirrormind Crown, Remnants of Beleren) and two
  non-replacement hits (Spectral Arcanist, Wishing Well, whose "may" is a trigger's, not a
  replacement action's).
- Parsed census over `client/public/card-data.json`: replacement `execute` trees, and
  `PreventDamage` sub-abilities, holding an `optional: true` node whose subtree reads
  `EventContextAmount`. **Channel Harm is the only card.**
- The "you may instead" cards parse as `ReplacementMode::Optional` (Moonlit Meditation and Jinnie
  Fay: `{"type":"Optional"}`). Their "may" is the replacement's own optional mode, asked before
  the action runs (CR 614.1a), and the action reads its amount inside the dispatch scope. No
  `OptionalEffectFrame` is involved, so they are outside U3's class. **No printed non-damage
  replacement has a "may" inside its action that reads the replaced amount**, so U3 has no
  non-damage card row. Its building block is tested at unit level instead (UT-1, UT-2, §16.5).
- Other pausing frames inside an amount-owning action read their amount when their instruction
  starts, before their prompt: Dralnu ("sacrifice that many"), Barbarian Class / Pixie Guide / Wyll
  ("roll that many dice plus one"), Kenessos ("scry that many cards plus one"), Krark's Thumb,
  Tekuthal (same parsed census, with coin, die, scry, proliferate and sacrifice effects). They need
  no carrier (§4 D7 residuals).
- Any-player "may" inside a replacement action: the Oracle census
  (`(would|prevented this way|instead)… (any player|any opponent|that player|an opponent) may`)
  returns only activation permissions (Glittering Lion, Glittering Lynx, Mercenaries, Samite
  Sanctuary). No printed card reaches `OpponentMayChoice` inside a replacement action.

## 2. What the P1 base does today (read at `00396db22d`, named by function)

- `replacement::drain_damage_rider` routes combat (`state.combat_prevention_tally.is_some()`) to
  `drain_combat_damage_rider`, which calls `settle_combat_rider_pauses`: an order park takes
  candidate 0, a "may" is declined, a rider's own prompt is abandoned (`CombatRiderPause`,
  `answer_combat_rider_park`, `abandon_combat_rider_pause`). It is reached from the substitution
  arm of the damage applier (the in-leg earlier rider) and from
  `combat_damage::fire_combat_prevention_riders` (Phase D).
- `replacement::drain_combat_damage_continuation` (the leg's own continuation) suspends the ledger
  and calls `apply_pending_post_replacement_effect` directly; a pause becomes
  `CombatDamageReplacementStep::ContinuationPaused` and parks the pass (asked since P1).
- `replace_combat_damage_batch` reads `read_damage_guards(state)` on **every entry**.
- `combat_damage::drain_combat_lifelink` parks only on a lifelink gain's
  `ReplacementDeferred::ReplacementChoice`; Phase D (`fire_combat_prevention_riders`) runs every
  shield in one call and returns nothing.
- The resume authority is `combat_damage::resume_pending_combat_lifelink`. It is reached from
  `engine_replacement::handle_replacement_choice` (a `ChooseReplacement` answer, in the answering
  action), from `combat_damage::resolve_combat_damage`'s re-entry branch (only when the turn
  machine re-enters the step after **all players pass priority**, priority.rs
  `combat_damage_incomplete`), and from `turns.rs`'s departed-active-player path.
- `coverage::StructuralFeature::DamageReplacementChoiceUnaskedInCombat` fences the cards whose rider
  asks a choice in combat.

- (D7) A post-replacement action runs inside
  `quantity::with_replacement_action_scope(owned_amount, ..)`
  (`engine_replacement::dispatch_pending_post_replacement_effect`; the owned amount is the resident
  drain's `event_amount`). The dispatch also stamps `last_effect_count`. Work it parks as a
  `PendingContinuation` captures the scope (`PendingContinuation::new` →
  `quantity::current_replacement_action()`), and `effects::drain_pending_continuation` re-enters it
  (`quantity::resume_replacement_action`).
- (D7) The action's "you may" node is parked by `effects::resolve_chain_body` (the up-front
  optional gate: `ResolutionOptionalPaymentChoice` and `OptionalEffectChoice`) or by
  `effects::offer_optional_in_apnap_order` (`OpponentMayChoice`). In each case a
  `ResolutionFrame::OptionalEffect(OptionalEffectFrame { .. })` literal is installed. The frame
  carries `trigger_event`, `trigger_events`, `trigger_match_count`, `return_result_occurrence` and
  `parked_poll`, but no replacement-action scope.
- (D7) The answer is a new action. `apply()` runs `GameState::clear_player_action_transients`
  (`last_effect_count = None`, `die_result_this_resolution = None`), then
  `engine_payment_choices::handle_optional_effect_choice_inner` restores the frame's trigger fields
  and calls `effects::resolve_optional_effect_decision`, all outside any replacement scope.

## 3. Claims the executor establishes first (before the edit that depends on each)

- **E1 — an answer that is not a `ChooseReplacement` does not resume a parked combat batch in its
  own action.** Read: `apply_non_priority_pass_action`'s post-match code and
  `engine::resume_pending_continuation_if_priority` never reach `resume_pending_combat_lifelink`;
  only `handle_replacement_choice` and the priority-pass re-entry do. So after P1's
  `dralnus_sacrifice_choice_is_asked_inside_the_batch` answers the `EffectZoneChoice`, the action
  returns `WaitingFor::Priority` with `state.pending_combat_lifelink` still `Some`, and the batch
  completes only after both players pass (a priority window between combat damage being assigned
  and dealt, against CR 510.2). **Measure it** at `PHASE_BASE_SHA` by adding the helper invariant of
  Step B1 (§16) to `fight_driving` first and running that one test: it must fail with
  "a parked combat batch outlived its answer". If it passes, E1 is false: drop Step B6 (the
  action-boundary call) and report it; nothing else changes. This decides D5 only.
- **E2 — a post-replacement dispatch that pauses leaves its prompt in `state.waiting_for`.** The
  noncombat batch already relies on this (`run_damage_batch`'s `damage_batch_has_paused_rider` arm
  parks and returns with the rider's prompt standing), and D2 reads `state.waiting_for` for the
  in-leg park. Settled by §16 row N-2's first counter prompt (its `ReplacementChoice` must offer
  exactly Hardened Scales and Doubling Season). Guarded in code by a `debug_assert!`.
- **E3 — two Awe Strikes cast in order fire their Phase D riders in cast order.** Both are
  `pending_damage_replacements` entries (`AppliedReplacementKey::Floating { index }`), and
  `apply_combat_damage_survivors` sorts the tally by `(1, 0, index, 0)` for floating keys. Settled by
  row N-1's prompt-2 life snapshot. The order between two shields is an engine convention (§13).
- **E4 — Channel Harm's "may" surfaces as `WaitingFor::OptionalEffectChoice { player: P0, .. }`
  answered by `GameAction::DecideOptionalEffect { accept }`.** Settled by row C-D3; if the engine
  raises another wait for that "may", the executor answers that wait in the test closure, and the
  derived values do not change.

- **E5 — at `PHASE_BASE_SHA`, accepting Channel Harm's "may" after a spell's damage is
  prevented deals no damage.** Read: the cascade in `quantity::event_context_amount` has tier 0
  `None` outside the scope, no trigger tiers for a spell, a die tier cleared by
  `clear_player_action_transients`, and `last_effect_count` cleared by it too. Settled by running
  N-5 at `PHASE_BASE_SHA` (or at `8b16c4a374`, where D7's functions are unchanged) before Commit
  D's fix: its accept arm must fail. If it passes, report it and stop: the root cause would be
  different from §4 D7.
- **E6 — inside a resolving trigger the base accept reads the trigger's amount.** Read:
  `handle_optional_effect_choice_inner` restores `current_trigger_event` from the frame, and
  outside the scope the enclosing-trigger tier reads it (Shockmaw Dragon's combat damage, 4).
  Settled by N-6 at the base: Shockmaw dealt 4 and destroyed. N-6 fails at the base whatever the
  wrong value is (it asserts exactly 1). Only this comment's predicted base value depends on E6.
- **E7 — at the base the two-Channel-Harm loop asks once.** P0's accepted Channel Harm deals 0, so
  P1's Channel Harm prevents nothing and is never asked. Settled by N-7 at the base.
- **E8 — no existing row accepts a "may" inside an amount-owning replacement action.** Read: the
  §1 censuses, and a grep of the integration tests for `DecideOptionalEffect { accept: true }` near
  a prevention or substitution (only C-D3/C-D5). So D7 changes no other row's expectation.
  Settled by the focused suites and the engine suite at run-level verification.

## 4. The P2 decisions, as carried out

### D1 — One combat rider drain (charter decision 1)

`replacement::drain_combat_damage_rider(state, events) -> Option<WaitingFor>`: suspend the ledger,
run the resident post-replacement work, restore the ledger, return the pause. It settles nothing and
does not restore any earlier wait. It is built on one shared primitive,
`replacement::dispatch_damage_rider_outside_batch(state, events) -> Option<PostReplacementPause>`
(CR 700.1: the rider's event is independent of the batch), which `drain_damage_rider` uses too, so
the take/restore of the ledger lives in one place.

- Callers: `drain_combat_damage_continuation` (the leg's own continuation; it stops calling
  `apply_pending_post_replacement_effect` itself) and `fire_combat_prevention_riders` (Phase D).
- `drain_damage_rider` loses its combat branch and becomes one body for both: dispatch outside the
  batch; on a pause record the frame in `DamageBatchLedger::paused_rider` and return
  `DamageRiderDrain::Paused { owner }`. In combat this is reached only from the substitution arm
  (deal_damage.rs's other callers are noncombat), and it now lets the earlier rider's prompt stand.
- Deleted: `CombatRiderPause` (enum + impl), `settle_combat_rider_pauses`,
  `answer_combat_rider_park`, `abandon_combat_rider_pause`, and the old body of
  `drain_combat_damage_rider`.

### D2 — In-leg earlier rider parks the leg (charter decision 2; CR 615.5 + CR 608.2c)

When the substitution arm's earlier-rider drain pauses inside a leg, the applier already returns
`ApplyResult::PreventedBehindRider(PausedRider { owner })` and the substitution's own drain is
queued behind that frame (`stash_post_replacement_continuation` →
`install_post_replacement_drain_behind_paused`). P2 adds the leg-side half in
`replacement::settle_combat_damage_replacement`, ahead of the continuation drain:

1. `NeedsChoice` → `NeedsChoice` (unchanged).
2. CR 104.1: `state.game_end.is_some()` → clear `paused_rider`, `Settled(survivor)`.
3. CR 615.5 + CR 608.2c: `take_combat_leg_paused_rider(state)` (new private fn: `take()` the
   ledger's `paused_rider`, `is_some()`) → `ContinuationPaused { survivor, waiting_for:
   Box::new(state.waiting_for.clone()) }`, with
   `debug_assert!(!matches!(state.waiting_for, WaitingFor::Priority { .. }))` (E2).
4. Otherwise `drain_combat_damage_continuation(state, survivor, events)`.

The check must precede the drain: with a paused resident and a Ready entry queued behind it, a drain
here would run the substitution's actions ahead of the rider's unfinished work. On the round trip,
the answer finishes the rider, `effects::resume_resolution_frames` →
`dispatch_resident_post_replacement_work` runs the queued substitution actions (which may ask their
own order), and once the action settles at priority the pass resumes through main's
`AwaitingContinuation` arm (`drain_combat_damage_continuation`, which then finds nothing Ready and
settles the leg). Nothing else changes in `replace_combat_damage_batch`.

### D3 — Phase D parks the record like the lifelink-gain pause (charter decision 3)

`combat_damage::fire_combat_prevention_riders(state, prevention_tally: &mut Vec<(AppliedReplacementKey,
i32)>, events) -> Option<WaitingFor>`:

- `while !prevention_tally.is_empty()`: CR 104.1 `break` if `state.game_end.is_some()`; then
  `let (key, total_prevented) = prevention_tally.remove(0);` — the shield leaves the record
  **before** its rider runs, so a resume never fires it again. The body is unchanged through the
  `install_ready_continuation` and the `event_source` seed.
- The same `resident_mut` block that seeds `event_source` also sets
  `drain.event_amount = Some(total_prevented as u32)` (`u32::try_from`, the value is `> 0` here):
  CR 615.5 "the amount of damage that was prevented", latched on the drain so the dispatch-bound
  reader (P1's seam 1) owns it across a pause. `state.last_effect_count = Some(total_prevented)` stays.
- `if let Some(waiting_for) = replacement::drain_combat_damage_rider(state, events)` and
  `state.game_end.is_none()` → `return Some(waiting_for)`. A pause that ended the game falls through
  to the loop top's CR 104.1 break.
- Returns `None` when every shield ran.

`combat_damage::drain_combat_lifelink`: after appending `damage_to_players`, when
`state.game_end.is_none()` and `fire_combat_prevention_riders(state, &mut record.prevention_tally,
&mut record.batch_events)` returns `Some(waiting_for)`: set `state.waiting_for =
waiting_for.clone()`, park `state.pending_combat_lifelink = Some(Box::new(record))` (its
`remaining` and `damage_to_players` are already empty, `replacement` and `phase_c_remnant` are
`None`, `prevention_tally` holds the unfired shields), return `CombatDamageBatch::Paused { events:
batch_events.clone(), waiting_for: Box::new(waiting_for) }` — the shape of its lifelink arm.

Resume needs no new code: `resume_pending_combat_lifelink` → `resume_combat_damage_replacements`
(`replacement: None`) → `drain_combat_lifelink` → no gains, an empty aggregate, Phase D continues
with the remaining shields once. CR 510.2 forbids a priority window, not the rider's choice
(CR 615.5, CR 608.2d).

### D4 — Guards read once across a park (charter decision 4; CR 608.2f + CR 700.1)

- `replacement::CombatDamageReplacementPass` gains `pub(crate) settled_guards:
  Vec<SettledDamageGuard>`. `CombatDamageReplacementPass::new(state: &GameState, events)` sets it
  from `read_damage_guards(state)`: the guards are read once, when the batch's pass is created.
- `replace_combat_damage_batch` installs `settled_guards: std::mem::take(&mut pass.settled_guards)`
  in place of today's fresh `read_damage_guards(state)`, and on exit moves **both** `applications`
  and `settled_guards` back into the pass (`if let Some(ledger) = state.damage_batch_ledger.take()
  { pass.one_shot_applications = ledger.applications; pass.settled_guards = ledger.settled_guards; }`).
  Its doc's "a fresh guard reading" sentence is rewritten.
- `types::game_state::PendingCombatDamageReplacement` gains
  `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub settled_guards:
  Vec<crate::types::ability::SettledDamageGuard>` (doc: CR 608.2f + CR 700.1, the Undergrowth
  Champion ruling; absent from saves written before it existed).
- `combat_damage::run_combat_damage_replacements`' Paused arm parks `settled_guards:
  pass.settled_guards`; `resume_combat_damage_replacements` rebuilds the pass with
  `settled_guards: parked.settled_guards`; `apply_combat_damage` builds the pass with
  `CombatDamageReplacementPass::new(state, proposed_events)`.
- Compiler-forced: the two unit tests calling `CombatDamageReplacementPass::new([..])`
  (`effects/prevent_damage.rs` and the `record_proposed_events` test in `replacement.rs`) pass
  `&state` first.

A rider's own damage (Channel Harm's) still reads guards live: the drain suspends the ledger
(CR 700.1). A later batch creates a new pass and reads afresh.

### D5 — The answer resumes the batch (charter goal: "never auto-settled", CR 510.2; refines decision 3)

Contingent on E1. `combat_damage::resume_parked_combat_damage_after_answer(state, waiting_for:
&WaitingFor, action_event_start: usize, events) -> Option<WaitingFor>`: when `waiting_for` is
`Priority` and `state.pending_combat_lifelink.is_some()`, set `state.waiting_for`, call
`resume_pending_combat_lifelink(state, action_event_start, None, events)` and return its wait
(`None` otherwise). One call site, in `engine::apply_non_priority_pass_action`, after
`resume_parked_counter_cost_root_at_action_boundary` and before the post-action pipeline, with
`action_event_start = events.len()` captured just before the action `match`.

Why: once D1–D3 let a rider's own prompt stand (Channel Harm's "may", Dralnu's sacrifice, a
two-Equipment Outfitted Jouster's choice, Nefarious Lich's exile), the batch must finish in the
action that answers it. Otherwise the answer returns a priority window with damage unassigned
(Phase B, CR 510.2) or with prevention riders unrun and combat-damage triggers not yet on the stack
(Phase D; CR 615.5, CR 510.3, CR 510.3a). This is the same authority `handle_replacement_choice`
already calls for a `ChooseReplacement` answer, with the same event anchor and the same
`claim_combat_lifelink_batch_events_for_ordinary_collection`; a `ChooseReplacement` answer leaves
nothing parked at `Priority`, so the new call is a no-op after it. The priority-pass re-entry in
`resolve_combat_damage` stays for the departure paths (CR 800.4a), and its comment is narrowed to
them.

This touches `crates/engine/src/game/engine.rs`, outside P2's anchor paths. It leaves decision 3
standing (the record parks like the lifelink pause and resumes through
`resume_pending_combat_lifelink`); it adds the call site the answering action needs. **For the
orchestrator: an addendum line, not a re-charter** (§19).

### D6 — Deletions and documents (charter decision 5)

- Coverage (`game/coverage.rs`): `StructuralFeature::DamageReplacementChoiceUnaskedInCombat` (its
  doc, `tag()` arm, `support()` arm and MEASURED comment), `RiderTargets`,
  `damage_replacement_reaches_combat`, `damage_replacement_asks_a_choice_in_combat`,
  `prevention_rider_asks_a_choice_in_combat`, `damage_rider_asks_a_choice`,
  `damage_rider_effect_asks_a_choice`, the `prevention_rider_asks_a_choice_in_combat` block in
  `extract_ability_features_with_token_statics`, the combat half of
  `extract_damage_replacement_choice_feature` (it keeps the substitution half), the unit tests
  `a_damage_riders_instruction_asks_for_a_picked_player_an_up_to_count_or_a_population` and
  `a_damage_replacement_choice_combat_cant_ask_is_a_coverage_gap`, and the deleted test's doc
  paragraph that sits misplaced above `a_shared_recipient_damage_instruction_is_a_coverage_gap`
  (the "CR 608.2d + CR 510.2: a combat-reachable damage replacement … combat never meets it."
  paragraph; that test keeps its own CR 615.7 paragraph). Imports left unused (`CombatDamageScope`,
  top-level `SubAbilityLink`) are removed; check each with a grep before removing.
- `game/resolution_prompt.rs`: `quantity_offers_up_to_choice` and `effect_offers_choice` return to
  private, as on main; the branch widened them only for the deleted classifier. (Its
  `ResumeDamageBatch` arm stays.) Optional cleanup, needing an addendum (§19).
- The strict-failure module-doc paragraphs and section headers citing Command Zone #140 in the three
  test files are rewritten (§16, Step C2).
- Doc truths updated: `DamageBatchLedger::paused_rider` (now set in combat: the leg parks on it),
  `DamageRiderDrain` variants, `drain_damage_rider`, `PendingCombatLifelink` and
  `PendingCombatLifelink::prevention_tally` (a Phase D park keeps the unfired shields),
  `CombatDamageBatch::Paused` (a fourth park point: a Phase D rider's prompt),
  `resolve_combat_damage`'s re-entry comment, and comment-only lines in
  `engine_replacement::handle_replacement_choice` ("the only boundary that knows where THIS action's
  events begin") and `tests/integration/combat_lifelink_replacement_ordering.rs` (the
  `&[(AppliedReplacementKey, i32)]` parameter line).

### D7 — A replacement action's "may" keeps the action's amount across its prompt (defective-reference repair; CR 614.6 + CR 615.5 + CR 608.2c + CR 608.2d)

**Card and derived reading.** Channel Harm: "If damage is prevented this way, you may have Channel
Harm deal that much damage to target creature." CR 615.5: the additional effect "may refer to the
amount of damage that was prevented" and "takes place immediately afterward". The "may" is
Channel Harm's controller's choice, made as part of that additional effect (CR 608.2d; rulings P3,
P4). So "that much" is the amount the shield prevented, whenever the choice is answered:
- 3 when Hill Giant's 3 is prevented (C-D3), and 3 for a Lightning Bolt (N-5);
- 7 for C-D5's aggregate (the Giant's 3 and the Swans' 4);
- 1 for Shockmaw Dragon's trigger, which deals 1 to each creature (N-6).

**Measured reference.** The executor ran round 1 at `8b16c4a374`: the accept arms of C-D3 and
C-D5 deal 0. The cause, read from source (§2's D7 bullets):
1. The dispatch runs the rider inside the scope that owns the drain's amount.
2. The rider's "you may" node parks an `OptionalEffectFrame` and returns
   `ResolutionProgress::Waiting`. The closure returns, so `quantity::NESTED_AMOUNT_SCOPE` is
   `Enclosing` again.
3. `OptionalEffectFrame` has no slot for the scope. `PendingContinuation` has one
   (`replacement_action`), but the optional frame was never given the same carrier.
4. The answer action clears `last_effect_count` and resolves the decision outside the scope.
5. `EventContextAmount` then falls to the enclosing trigger's event amount, restored from the
   frame (E6), or to nothing, which reads as 0 (E5).

So the defect is not specific to combat. It is pre-existing at `PHASE_BASE_SHA`: no P1 row
accepted such a "may" (P1 declined combat ones unasked, and no noncombat Channel Harm row exists).
The delivered C-D3/C-D5 accept arms assert the derived values. They are the dependent rows, and
the engine's 0 is the defective reference. **Defective-reference route**: fixed in this phase.
The rows red at base cover the class: noncombat (N-5), inside a trigger (N-6), nested (N-7), and
every frame kind at unit level (UT-1, UT-2).

**Decision: one carrier, built from the existing scope machinery.** `OptionalEffectFrame` gains
`replacement_action: Option<ReplacementActionScope>`. It uses the type, the capture function
(`quantity::current_replacement_action`) and the re-entry guard (`quantity::resume_replacement_action`)
that `PendingContinuation::replacement_action` already uses. There is no new scope kind, no
resident `GameState` field and no reading from the paused drain.

- **Field** (`types/resolution.rs`, beside `return_result_occurrence`):
  `#[serde(default, skip_serializing_if = "Option::is_none")] pub replacement_action:
  Option<crate::types::game_state::ReplacementActionScope>`. It is `pub` like its siblings, because
  `phase-ai` and the integration tests build frame literals. Doc: "CR 614.6 + CR 615.5 + CR 608.2c +
  CR 608.2d: `Some` when this "may" was reached inside a replacement's own action that owns an
  amount. The answer resumes that action, so its "that much" is still the amount the replacement
  replaced or prevented (Channel Harm: "If damage is prevented this way, you may have Channel Harm
  deal that much damage to target creature."). Captured when the offer is made
  (`quantity::current_replacement_action`), like `PendingContinuation::replacement_action`.
  Re-entered by every resolution of this frame's ability. Serde-defaulted: a save taken at the
  prompt before this field existed resumes outside the action."
- **Capture, once.** A new `OptionalEffectFrame::live_offer(ability: ResolvedAbility, state:
  &GameState) -> Self` in `types/resolution.rs` captures every ambient field the three live park
  sites copy today, plus the scope, with `parked_poll: None`:
  - `state.current_trigger_event`, `current_trigger_events`, `current_trigger_match_count`,
    `active_return_result_occurrence`;
  - `replacement_action: crate::game::quantity::current_replacement_action()`.

  This mirrors `PendingContinuation::new`, which captures its own context. The per-field CR 608.2 /
  CR 603.2c comments at `resolve_chain_body`'s `OptionalEffectChoice` site move onto the
  constructor.
- **Park sites.** All three live producers call `OptionalEffectFrame::live_offer(<the hydrated
  ability they build today>, state)`:
  - `effects::offer_optional_in_apnap_order` (`OpponentMayChoice`);
  - `effects::resolve_chain_body`'s `ResolutionOptionalPaymentChoice` install;
  - `effects::resolve_chain_body`'s `OptionalEffectChoice` install.
- **Re-offer: copy, don't capture.** `handle_opponent_may_choice_inner`'s re-offer becomes
  `OptionalEffectFrame { parked_poll: None, ability: reoffered, ..frame.clone() }`. The next
  responder's offer then belongs to the same action, along with any ambient field the frame later
  gains.
- **Legacy wire.** `LegacyOptionalEffectWire::into_frame` sets `replacement_action: None` (a v1
  save predates the field).
- **Re-entry at every resolution of a frame's ability.** The guard is held only around that
  resolution and dropped before `resume_pending_continuation_if_priority`. So the enclosing spell's
  tail, the paused dispatch's retirement, D5's combat-batch resume and any queued drain run outside
  it, each under its own scope (maintainer condition 3: "check that the amount doesn't leak into
  the enclosing spell's tail").
  - **R1** `engine_payment_choices::handle_optional_effect_choice_inner`: bind `replacement_action`
    in the frame's exhaustive destructure, then `let result = { let _action =
    crate::game::quantity::resume_replacement_action(replacement_action);
    effects::resolve_optional_effect_decision(state, *ability, choice, events, 1) };`. This covers:
    - `DecideOptionalEffect`;
    - `handle_resolution_optional_payment_choice`, pay and decline;
    - the resolution-sacrifice completion (`casting_costs` re-pushes the frame and calls
      `handle_optional_effect_choice`);
    - a departed payer's canonical decline (`elimination` re-pushes the frame and calls it).
  - **R2** `handle_opponent_may_choice_inner`, accept arm: a guard around
    `effects::resolve_ability_chain(state, &ability, events, 1)`, and one around the selection
    branch's `PendingContinuation::new(sub, state)`, so the parked "if a player does" rest captures
    the scope.
  - **R3** `advance_opponent_may_offer`: a guard around `resolve_all_declined_opponent_may(state,
    &frame.ability, events)`.
  - **R4** `arm_parked_opponent_may_poll`, the no-seat-left close: take `replacement_action` from the
    frame with `poll` and `ability`, and put a guard around `resolve_all_declined_opponent_may`. That
    completion is a no-op by the function's own `debug_assert!`; it is entered so every resolution of
    a frame's ability takes the same route.
- **Pass-through, no edit** (the frame moves whole):
  - `replace_active_optional_effect_frame` in `handle_resolution_optional_payment_choice`;
  - `casting_costs::handle_resolution_optional_sacrifice_for_cost` →
    `PendingSacrificeCostCompletion::ResolutionOptionalPayment { frame }` → its completion arm;
  - `casting_costs::take_and_settle_parked_resolution_optional_sacrifice`;
  - `elimination::stage_resolution_optional_sacrifice_decline_for_departing_payer`;
  - the elimination abandon path, which drops the frame unresolved;
  - `GameState::park_opponent_may_poll_beneath_live_prompt`;
  - readers that resolve nothing: `precast_copy_shortcut::pending_optional_is_chain_copy`, `filter.rs`,
    `triggers.rs`, `engine_resolution_choices.rs`, `GameState` validation.
- **Nesting.** The guard restores the previous scope when it drops, on unwind too. A nested action
  entered inside a resumed one (the two-Channel-Harm loop, ruling P4: "a loop that either player can
  break by choosing to not have Channel Harm try to deal damage") runs under its own amount. Its
  "may" frame captures that inner amount, and the outer scope comes back afterwards (maintainer
  condition 3: "Restore the outer scope after nested work").

**Residuals, recorded and not built** (no printed card reaches them; §1 census):
- R2's selection branch hands the accepted ability to `WaitingFor::MultiTargetSelection {
  pending_ability }`, a second carrier. No replacement action contains an any-player "may".
- The other direct-choice frames inside an amount-owning action read "that many" when their
  instruction starts, before their prompt: coin flip, die roll, choose-one, vote, proliferate,
  repeated optional payment.

**Rejected alternatives.**
- A field on `SpellContext`: a second carrier next to `PendingContinuation::replacement_action`,
  and context is cloned into sub-abilities and reflexive triggers, where CR 603.12 suppresses the
  scope.
- Reading the paused `PostReplacement` drain beneath the frame: positional.
  `dispatch_pending_post_replacement_effect`'s own comment records how positional lookups failed,
  and an enclosing spell's "may" parked above an unrelated paused drain would enter its scope.
- A resident `GameState` field: maintainer condition 3 accepted the dispatch-bound lifetime, not
  a resident tier.

## 5. Sizing

| Unit | Behavior | Registration surfaces | Discriminating tests |
|---|---|---|---|
| **U1 Every combat rider pause asked** | D1 one drain; D2 in-leg park; D3 Phase D park; D5 answer resumes; D6 auto-settle and fence deleted | replacement.rs, combat_damage.rs, engine.rs (one call), coverage.rs, resolution_prompt.rs, game_state.rs (docs), three test files, census baseline | N-1 (two aggregate shields), N-2 (in-leg earlier rider), N-4 (departure at a Phase D pause), C-D1…C-D5 conversions, the helper invariant over every combat row |
| **U2 Guards read once across a park** | D4 | replacement.rs, combat_damage.rs, game_state.rs (serialized field), prevent_damage.rs + replacement.rs unit tests (compiler-forced), protocol record | N-3 (Undergrowth Champion across a park, restored from serialization at the second prompt) |
| **U3 A replacement action's "may" keeps its amount across the prompt** (defective-reference repair) | D7 | types/resolution.rs (field, `live_offer`, legacy wire), effects/mod.rs (three park sites + unit tests), engine_payment_choices.rs (R1–R4, re-offer), compiler-forced `replacement_action: None` in every other frame literal (13 files, all test code), a new protocol record | N-5 (noncombat, with a save at the prompt), N-6 (inside Shockmaw Dragon's trigger), N-7 (two-Channel-Harm loop), UT-1, UT-2; it turns C-D3's and C-D5's accept arms green |

Dependency edges:
- U2 and U1: none (r1). U2 then U1 were delivered as `8b16c4a374`.
- **U3 → U1**: U3 is infrastructure; C-D3's and C-D5's accept arms (U1 rows) consume it. U3's own
  rows need neither U1 nor U2. N-5 and N-7 are noncombat. N-6's combat has no rider, and its rider
  is a resolving trigger's noncombat damage.

Order: U2, U1 (delivered), then U3 as Commit D.

Scope paths under the phase-fit counting rule (directories expanded; fixtures, regenerated data
and protocol surfaces excluded; standing-class paths counted, as P1 plan r2's reviewer-supplied
text states: "Include those standing-class paths in T2; their admission does not exclude them
from counting"):

Delivered (r1):
1. `crates/engine/src/game/replacement.rs`
2. `crates/engine/src/game/combat_damage.rs`
3. `crates/engine/src/game/coverage.rs`
4. `crates/engine/src/types/game_state.rs` (also holds compiler-forced U3 test literals)
5. `crates/engine/src/game/engine.rs` (addendum; also holds one compiler-forced U3 test literal)
6. `crates/engine/src/game/resolution_prompt.rs` (addendum)
7. `crates/engine/src/game/effects/prevent_damage.rs` (compiler-forced, D4)
8. `crates/engine/tests/integration/unpreventable_damage_prevention_effects.rs` (also U3's N-5–N-7)
9. `crates/engine/tests/integration/damage_substitution_replacements.rs`
10. `crates/engine/tests/integration/ironscale_hydra_combat_prevention.rs`
11. `crates/engine/src/game/engine_replacement.rs` (comment-only)
12. `crates/engine/tests/integration/combat_lifelink_replacement_ordering.rs` (comment-only)

Added by U3, production:
13. `crates/engine/src/types/resolution.rs`
14. `crates/engine/src/game/effects/mod.rs`
15. `crates/engine/src/game/engine_payment_choices.rs`

Added by U3, compiler-forced test literals (one `replacement_action: None,` per
`OptionalEffectFrame { .. }` literal, all inside `#[cfg(test)]` code or test crates):
16. `crates/engine/src/game/engine_phase_trigger_regression_tests.rs`
17. `crates/engine/src/game/visibility.rs`
18. `crates/engine/src/game/elimination.rs`
19. `crates/engine/src/game/sba.rs`
20. `crates/engine/src/game/casting_costs.rs`
21. `crates/engine/src/game/effects/phase_out.rs`
22. `crates/engine/src/game/triggers.rs`
23. `crates/engine/src/ai_support/payment_continuation.rs`
24. `crates/engine/src/game/effects/scoped_library_search.rs`
25. `crates/engine/tests/integration/cr733_resolved_frame_transition.rs`
26. `crates/engine/tests/integration/optional_effect_remember_legal_actions.rs`
27. `crates/phase-ai/src/policies/sacrifice_value.rs`
28. `crates/phase-ai/src/planner/mod.rs`

Excluded:
- `scripts/post-replacement-continuation-baseline.txt` (regenerated by its census);
- `protocol/changes/a26a16db-3963-4e52-a838-8473d6808198.json` (delivered, U1/U2);
- U3's new `protocol/changes/<uuid>.json` (protocol surface).

Regenerate the literal list with `grep -rln "OptionalEffectFrame {" crates/ --include=*.rs`.
A literal that uses `..` needs no edit; today every literal names all fields.

**Count: 28 counted paths. T1: 3 units. T2: 28 ≥ 13. The T1∧T2 conjunction fires.**
- The firing does not depend on the compiler-forced literals: without paths 16–28 the count is 15.
- It does not depend on the delivered optional cleanup (path 6) either.
- Every correct U3 touches at least the frame's type (`types/resolution.rs`) and the answer
  handler (`engine_payment_choices.rs`), neither in r1's 12. So no U3 design keeps P2 under 13.

That makes it a routing question for the orchestrator (§19). The plan is **split-ready**:
- Commit D (U3) is self-contained: its production files are disjoint from Commits A–C, and its
  rows need nothing from U1 or U2.
- A charter-mode partition can carve U3 into its own phase without re-planning: 1 unit,
  19 counted paths (13–28, plus 4 and 5 for their test literals and 8 for its rows), so the
  conjunction does not fire for it.
- The green-tree order is U3 first, on `00396db22d`, then the delivered U1+U2 replayed on U3's
  candidate. In the other order, C-D3's and C-D5's accept arms would sit red in the U1+U2 phase.

## 6. Building Blocks

- `replacement::{replace_combat_damage_batch, settle_combat_damage_replacement,
  drain_combat_damage_continuation, CombatDamageReplacementPass, read_damage_guards,
  settled_damage_guard_reading, damage_rider_placement, stash_post_replacement_continuation}` — the
  pass and ledger P1 kept; D2 adds one check, D4 one field.
- `engine_replacement::dispatch_pending_post_replacement_effect` / `PostReplacementPause` — the
  single dispatcher both drains call through `dispatch_damage_rider_outside_batch`.
- `combat_damage::{drain_combat_lifelink, resume_pending_combat_lifelink,
  resume_combat_damage_replacements, run_combat_damage_replacements,
  claim_combat_lifelink_batch_events_for_ordinary_collection}` — main's #9670 park and resume; D3
  copies the lifelink arm, D5 reaches the same resume.
- `effects::{resume_resolution_frames, dispatch_resident_post_replacement_work}` — the round trip
  that runs work queued behind a paused rider (D2), unchanged.
- `GameState::install_post_replacement_drain_behind_paused`, `active_post_replacement_drains_mut`,
  `PostReplacementDrainStack::resident_mut` — unchanged.
- Tests: `unpreventable::{fight, fight_answering, preferring, offered, scales_and_season,
  rider_board, assert_batch_settled, bolt_at, cast_by_p1, drive}`, `damage_substitution::
  {combat_events, combat_answering, choose_shield, choose_source, answer_counter_orders, temper,
  cast_temper_at_one, phytohydra}`, main's Szadek pattern `assert_batch_completed_by_the_answer` /
  `round_trip_at_pause` (re-expressed in the helpers, not imported).

- (U3) `quantity::{current_replacement_action, resume_replacement_action,
  ResumedReplacementAction}` and `types::game_state::ReplacementActionScope`: the capture, re-entry
  guard and carrier type `PendingContinuation::replacement_action` uses. D7 adds a second *carrier*
  of the same scope, not a second mechanism. `quantity::with_replacement_action_scope` (unchanged)
  is what the unit tests enter.
- (U3) `effects::{offer_optional_in_apnap_order, resolve_chain_body, resolve_optional_effect_decision,
  resolve_ability_chain}`, `engine_payment_choices::{handle_optional_effect_choice_inner,
  handle_opponent_may_choice_inner, advance_opponent_may_offer, arm_parked_opponent_may_poll,
  resolve_all_declined_opponent_may}`, `GameState::install_direct_choice_frame`: the park and
  answer authorities, unchanged except for the capture and the guards.
- (U3) Tests: `unpreventable::{rider_board, spell, cast, cast_by_p1, drive, bolt_at,
  creature_with_keywords, assert_settled, prevented}`, the module's `serde_json` save/load at a
  pause (`*runner.state_mut() = serde_json::from_str(..)`), and `effects::replacement_action_continuation_tests::{action_life, life}`.

No new helper duplicates an authority: `dispatch_damage_rider_outside_batch` is the extraction of
the ledger take/restore both drains already do; `resume_parked_combat_damage_after_answer` is a
guard around the existing resume authority. `OptionalEffectFrame::live_offer` is the
extraction of the field copies the three park sites already make, plus the scope, as
`PendingContinuation::new` captures its own.

## 7. Logic Placement

| Behavior | Owning function | Delegating callers |
|---|---|---|
| A damage rider runs outside the batch's ledger | `replacement::dispatch_damage_rider_outside_batch` | `drain_damage_rider`, `drain_combat_damage_rider` |
| A combat rider's pause is returned, never settled | `replacement::drain_combat_damage_rider` | `drain_combat_damage_continuation`, `combat_damage::fire_combat_prevention_riders` |
| An in-leg earlier rider's pause parks the leg | `replacement::settle_combat_damage_replacement` | every leg step: `replace_combat_damage_batch` (start, AwaitingChoice answer), `repropose_combat_damage_leg` |
| Phase D parks and resumes once | `combat_damage::fire_combat_prevention_riders` (pop, run, return pause) + `drain_combat_lifelink` (park) | `apply_combat_damage_survivors`, `resume_combat_damage_replacements` |
| Guards read once per batch | `CombatDamageReplacementPass::new` | `apply_combat_damage`; carried by `run_combat_damage_replacements` / `resume_combat_damage_replacements` |
| The answer completes the parked batch | `combat_damage::resume_parked_combat_damage_after_answer` → `resume_pending_combat_lifelink` | `engine::apply_non_priority_pass_action` (one call); `handle_replacement_choice` keeps its own call |
| A "may" reached inside a replacement action is offered with that action's scope | `OptionalEffectFrame::live_offer` (capture through `quantity::current_replacement_action`) | `effects::offer_optional_in_apnap_order`, `effects::resolve_chain_body` (two installs); the re-offer copies the frame (`..frame.clone()`) |
| Its answer resolves inside that action, and only its own work does | `quantity::resume_replacement_action(frame.replacement_action)`, held around the resolution | R1 `handle_optional_effect_choice_inner` (and through it the optional-payment, resolution-sacrifice and departure-decline paths); R2 `handle_opponent_may_choice_inner`; R3 `advance_opponent_may_offer`; R4 `arm_parked_opponent_may_poll` |

Consumers whose cardinality changes with the same signatures: `drain_damage_rider`'s combat callers
(the substitution arm) can now see `Paused`; `resolve_combat_damage`'s re-entry branch now meets a
Phase D record (`replacement: None`, empty `remaining`), which it resumes through the same path;
`elimination`'s departure handling of `pending_combat_lifelink` now meets a Phase D record too (row
N-4). (U3) Every path that forwards an optional frame whole now forwards its scope as well,
with no edit: `casting_costs`' resolution-sacrifice completion, `elimination`'s departed-payer
decline, and `handle_resolution_optional_payment_choice`'s frame replacement. Each reaches R1.

## 8. Rust Idioms

`Option<WaitingFor>` for "the pause it raised", as `drain_combat_damage_continuation` and
`apply_pending_post_replacement_effect` already return; `&mut Vec` popped from the front for the
serialized tally (a `Vec`, not a `VecDeque`, because the record's type is unchanged); no new enum;
exhaustive matches untouched. `u32::try_from(total_prevented)` for the latched amount. No `bool`
parameters. (U3) `Option<ReplacementActionScope>` is the existing typed carrier, not a bool or a
raw `i32`. The RAII guard (`ResumedReplacementAction`) bounds the re-entry lexically, in a block
expression around the one resolving call. The handler's frame destructure stays exhaustive, so a
later field forces a re-audit. The re-offer uses struct-update syntax so it copies every ambient
field.

## 9. Nom Compliance

No file under `crates/phase-parser/src/parser/` changes. No Oracle text is newly accepted.

## 10. Extension vs Creation

Extension throughout: the Phase D park is main's lifelink park applied to a second producer; the
in-leg park is main's `AwaitingContinuation` leg state; the guards ride the pass next to its
one-shot applications; the answer resume reuses `resume_pending_combat_lifelink`. The only new call
site (D5) exists because the answering action is the one boundary that holds the event anchor, as
`handle_replacement_choice`'s own doc explains.

(U3) Extension: the optional frame gets the carrier the continuation frame already has, through
the same capture and re-entry functions. The constructor mirrors `PendingContinuation::new`. No
new pattern.

## 11. Analogous Trace (Step 2)

Traced main's #9670 lifelink-gain park end to end, the model D3 copies (charter Step 2):
`combat_damage::apply_combat_damage` → `run_combat_damage_replacements` →
`apply_combat_damage_survivors` → `drain_combat_lifelink` (`Err(ReplacementDeferred::ReplacementChoice)`
arm: `state.pending_combat_lifelink = Some(Box::new(record))`, `CombatDamageBatch::Paused`) →
`resolve_combat_damage` (first-strike / regular arms surface `waiting_for`) → answer →
`engine_replacement::handle_replacement_choice` (generic inner handler, then the `Priority` tail
`resume_pending_combat_lifelink(.., None, ..)`) → `resume_combat_damage_replacements`
(`replacement: None` → `drain_combat_lifelink`) → `finish_combat_damage_sub_step`. Types:
`types/game_state.rs::{PendingCombatLifelink, PendingCombatDamageReplacement,
PendingCombatLegProgress}`.

Traced the leg continuation park P1 inherited from main (D2's model):
`replace_combat_damage_batch` → `settle_combat_damage_replacement` →
`drain_combat_damage_continuation` → `ContinuationPaused` → `PendingCombatLegProgress::AwaitingContinuation`
→ answer → `effects::resume_resolution_frames` → resume → `take_surfaced_leg` →
`drain_combat_damage_continuation`.

Traced the noncombat earlier-rider queue (D2's other half): `apply_damage_done_replacement`'s
substitution arm (`drain_damage_rider` → `ApplyResult::PreventedBehindRider`) →
`apply_single_replacement` (`damage_rider_placement` → `stash_post_replacement_continuation` →
`install_post_replacement_drain_behind_paused`) → `effects::resume_resolution_frames`
(`PostReplacement` arm, `queued_behind`) → `dispatch_resident_post_replacement_work`; witness
`damage_substitution_replacements::an_earlier_riders_paused_counters_finish_before_the_substitution`.

Traced guard reading: `read_damage_guards` → `DamageBatchLedger::settled_guards` →
`settled_damage_guard_reading` (in `evaluate_replacement_condition`'s path) and the noncombat
carrier `SpellContext.settled_damage_guards` / `with_damage_batch`, whose serialized pause row is
`unpreventable::the_batch_reading_survives_serialization_at_the_pause`.

(U3) Traced the replacement-action scope's existing carrier end to end, the model D7 copies:
`engine_replacement::dispatch_pending_post_replacement_effect` (`captured_amount` from the
resident drain → `with_replacement_action_scope(owned_amount, ..)`) → the action pauses →
`types/game_state.rs::PendingContinuation::new` (`replacement_action:
quantity::current_replacement_action()`) → `effects::park_ability_continuation` /
`prepend_to_pending_continuation_with_producer` (merge guarded on equal `replacement_action`) →
answer → `effects::drain_pending_continuation` (`quantity::resume_replacement_action(replacement_action)`
held until the chain and its detached tail have run) → `quantity::event_context_amount` tier 0
`replacement_action_amount`. Unit witnesses:
`effects::replacement_action_continuation_tests::{latched_payment_and_replacement_action_keep_separate_continuation_heads,
a_replacements_action_continuation_never_merges_with_work_of_another_owner}`,
`quantity` nested-scope tests. Traced the optional frame's park and answer: `effects::resolve_chain_body`
(up-front gate → `install_direct_choice_frame(ResolutionFrame::OptionalEffect(..),
WaitingFor::OptionalEffectChoice)`) → `engine::apply` (`clear_player_action_transients`) →
`engine_payment_choices::handle_optional_effect_choice` → `_inner` (restores trigger fields →
`effects::resolve_optional_effect_decision`) → `resume_pending_continuation_if_priority` →
(combat) `engine::apply_non_priority_pass_action` → `resume_parked_combat_damage_after_answer` (D5).

## 12. Verification Matrix (with Identity / Provenance)

Production entry for every runtime row: `GameRunner` driving a real combat (declare attackers →
blockers → `AssignCombatDamage` → the batch) through `fight_driving`/`fight_answering` or
`combat_events`; completion boundary: the answering action completes the batch
(`pending_combat_lifelink.is_none()` with `combat.regular_damage_done` at that `Priority`), then
`assert_batch_settled` (no park, no frame, EndCombat priority with an empty stack), then a
post-batch Lightning Bolt for state-based actions where the row has one.

| Row | Claim (CR, ruling) | Seam | Test | Revert-failing assertion | Positive reach-guard | Hostile / sibling | Identity / provenance |
|---|---|---|---|---|---|---|---|
| V1 | Every pause a combat rider raises is answered by its player; the answer completes the batch in the same action (CR 510.2, CR 510.3, CR 615.5) | D5 + D1 | Helper invariant in `fight_driving` and `combat_answering` (Step B1), over every combat row of the three files | After an answer that leaves `Priority`, `pending_combat_lifelink.is_none()` (fails at base on `dralnus_sacrifice_choice_is_asked_inside_the_batch`, E1) | the row's own prompt was asked (each row records it) | `ChooseReplacement` answers (already resumed: no-op); answers that leave another prompt (no resume); departure (N-4, the re-entry fallback) | anchor = start of the answering action's events, a local |
| V2 | Phase D asks its riders, one shield at a time; the second shield's rider runs once, after the first one's answer (CR 615.5, CR 616.1; P1, P2) | D3 | **N-1** `two_aggregate_riders_are_asked_one_after_the_other` | 2 prompts; life at prompt 2 is 25 / 26; final life exact per answer pair (§16) | `prevented == 2`, P1 dealt no combat damage | one shield (C-D1); a shield with tally 0 (C-D3 unblocked); serialization at the Phase D park (prompt 2) | tally key `Floating { index }` popped before the rider; order = cast order (E3, engine convention) |
| V3 | An earlier rider's prompt inside a leg parks the leg; the substitution's actions wait behind it and run on the round trip (CR 615.5 + CR 608.2c, CR 616.1f) | D2 | **N-2** `an_earlier_riders_pause_inside_a_combat_leg_is_asked` | 3 prompts in order shield-order / Temper's counter / Phytohydra's counters; counters per answer pair | first prompt offers Temper and Phytohydra; at prompt 2 `pending_combat_lifelink` has `replacement.is_some()` and Phytohydra has 0 counters | Phytohydra first (no earlier rider; P1's path); no Hardened Scales (rider completes, no park) | `PausedRider { owner }` frame; the queued drain keeps its own latched amount (P1 seam 1) |
| V4 | Guards are read once, when the batch opens, and survive a park and serialization (CR 608.2f + CR 700.1; Undergrowth Champion ruling) | D4 | **N-3** `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park` | Champion `damage_marked == 3`; no prompt offers the Champion; counters 6 / 5 (fails at base: re-read at resume gives damage 1) | Temper applied (`prevented == 1`, counters > 0: the live guard did flip) | in-memory vs serialized at prompt 2; existing no-park `undergrowth_champion_double_blocked_reads_its_guard_once` (reading Holds) | `SettledDamageGuard { source: ObjectIncarnationRef, condition, reading }`; bound at `CombatDamageReplacementPass::new`; stored in the pass, the parked `PendingCombatDamageReplacement.settled_guards`, the ledger while running; consumed by `settled_damage_guard_reading`; dropped when the pass completes |
| V5 | A rider's own "may" in Phase D is its controller's (CR 608.2d; P3, P4) | D3 + D5 | **C-D3** `channel_harms_may_is_asked_after_the_batch` | accept → Giant dealt 3 by Channel Harm, `is_combat: false`, dies; decline → Giant 2 damage, alive | blocked: `prevented == 1` | unblocked: nothing prevented, no prompt, P1 at 18 | latched `event_amount = 3` on the Phase D drain, carried across the prompt on `OptionalEffectFrame::replacement_action` (D7) |
| V6 | A rider's event "may" at Phase D (Abundance on New Way Forward's draws) is asked of the drawing player (CR 616.1, CR 121.2) | D3 | **C-D4** `new_way_forwards_draws_ask_abundance` | 2 prompts to P1; `CardDrawn` (P1) 2 decline / 0 accept | P0 life −2 exactly once | no Abundance: 2 draws, no prompt | chosen source P0's Bears (`event_source` seed) |
| V7 | Counter order of a Phase D rider asked (CR 616.1; Hardened Scales ruling) | D3 | **C-D1** `gatta_and_luzzus_counter_order_is_asked_after_the_batch`, **C-D2** `a_lifelink_answer_then_gatta_and_luzzus_rider` | exact counters per answer (§16) | `prevented == 1`; lifelink +5 (C-D2) | no Hardened Scales: 1 counter, no rider prompt | — |
| V8 | Per-leg and Phase D prompts in one batch (CR 616.1, CR 608.2d) | D1 + D3 + D5 | **C-D5** `a_may_park_and_a_riders_own_prompt_are_both_asked` | Abundance asked per draw, then Channel Harm's "may"; accept → Giant dealt 7 noncombat, dies | `prevented == 2` | no Abundance: only Channel Harm's prompt | Channel Harm's aggregate amount 3 + 4, carried across the prompt on `OptionalEffectFrame::replacement_action` (D7) |
| V9 | A rider prompt whose player leaves the game: the batch still finishes (CR 800.4a, CR 104.1 not reached) | D3 + fallback | **N-4** `a_phase_d_rider_prompt_whose_player_leaves_lets_the_batch_finish` | after P1 concedes at its Phase D prompt, pumping ≤ 16 actions drains the record, `regular_damage_done`, no further prompt to P1 | P1 was asked (`ReplacementChoice { player: P1 }` with `replacement.is_none()` record) | P2's life 20, P0's life 20 | — |
| V10 | CR 104.1 between Phase D shields (kept) | D3 | existing `a_prevention_rider_that_ends_the_game_ends_the_batchs_riders` (unchanged) | Awe Strike's gain never happens after Deflecting Palm's rider ends the game | game over, P0 eliminated | — | — |
| V11 | Coverage: the fence is gone and no class member is claimed supported by mistake | D6 | `cargo coverage` at run-level verification (charter decision 5) | flips listed; expected: Battletide Alchemist and Channel Harm leave `damage_replacement_choice_unasked_in_combat`; Dralnu and Nefarious Lich stay unsupported through `damage_substitution_choice_incomplete`; Outfitted Jouster through its confirmed-misparse row (pr-body line 69) | the deleted unit tests' card list | — | — |
| V12 | A replacement action's "may" reads the amount the replacement prevented after its prompt: after a spell's damage, and from a save taken at the prompt (CR 614.6, CR 615.5, CR 608.2c, CR 608.2d; P3, P4) | D7 (live_offer, R1) | **N-5** `channel_harms_may_deals_the_prevented_amount_after_a_spell` | accept → `DamageDealt { source_id: channel_harm, amount: 3, is_combat: false }` to the Giant, Giant in the graveyard (base: nothing dealt, Giant alive, E5) | at the prompt the frame's `replacement_action == Some(ReplacementActionScope { amount: 3 })`; `prevented == 1`; P0 asked once | decline (Giant 0 damage); Bolt aimed at P0 the player; serde round trip at the prompt (revert probe: `#[serde(skip)]` on the field gives nothing dealt); an ordinary "may" outside any action keeps `None` (UT-1) | bound when the frame is installed, from the dispatch's scope (the drain's `event_amount` 3); stored on the frame (serialized); consumed by R1; gone with the frame |
| V13 | Inside a resolving trigger the "may" reads its own prevented amount, not the trigger's event (CR 614.6, CR 615.5, CR 603.2; maintainer condition 3's Shockmaw Dragon) | D7 (R1; the scope suppresses the enclosing-trigger tiers) | **N-6** `channel_harms_may_reads_the_prevented_amount_inside_a_resolving_trigger` | accept → Channel Harm deals 1 to Shockmaw Dragon, which stays on the battlefield with `damage_marked == 1` (base: 4, destroyed, E6) | P1 at 16 at the prompt; `prevented == 1`; frame amount 1; P1 asked once | decline → Shockmaw 0 damage; the P1 row `a_replacements_action_reads_its_own_amount_inside_a_resolving_trigger` stays green | the frame's `trigger_event` (4) and its `replacement_action` (1) both restored: tier 0 wins and suppresses tier 4 |
| V14 | Nested actions: each "may" reads its own action's amount, and the outer scope returns after the inner (ruling P4; CR 615.5, CR 608.2d) | D7 (guard nesting) | **N-7** `two_channel_harms_loop_until_a_player_declines` | (accept, accept, decline): prompts to [P0, P1, P0]; no `DamageDealt` at all (base: [P0] only, E7) | each prompt's frame amount 3 | (accept, decline): [P0, P1]; (decline): [P0]; Courser, Giant and Bears 0 damage in every row | two frames, two actions, each with its own scope |
| V15 | Building block: every "may" park kind captures the scope, every resolve path re-enters it, and nothing leaks (CR 614.6 + CR 608.2c + CR 101.4) | D7 (live_offer, R1, R2, R3) | **UT-1** `a_may_reached_inside_a_replacement_action_resolves_inside_it`, **UT-2** `an_any_player_may_inside_a_replacement_action_resolves_inside_it` | inside `with_replacement_action_scope(Some(2))`: OptionalEffectChoice accept gains 2 (base 0); OpponentMayChoice accept gains 2; the all-declined "if no one does, gain that much" gains 2 | frame field `Some(ReplacementActionScope { amount: 2 })` at each prompt | the same ability outside any scope: field `None` (paired with the in-scope `Some(2)`); `with_replacement_action_scope(None)`: `None`; after every answer `current_replacement_action() == None` | — |

Ruling → row: P1, P2 → V2, V3; P3, P4 → V5, V8, V12, V14; P7 → V3, V4; P8 → V2 (Bolt after the
batch); S11 → V1 (Dralnu row). Channel Harm's "You choose that target as you cast Channel Harm"
ruling → V12–V14 (each row casts Channel Harm at its creature up front). Coverage status impact: V11. No Oracle text is newly accepted.

`DEFERRED(phase 3)`: the substitution predicate fold, `ShieldKind::Substitution` deletion and the
fence `damage_substitution_choice_incomplete`'s re-key (outcome A). No P2 row depends on them.

## 13. Reference Readings

- **Order between two Phase D shields (N-1 snapshot)** — not derivable from the cards: both riders
  are P1's, of two prevention effects on two damage events. It is the engine's tally sort
  (`apply_combat_damage_survivors`, floating index = cast order), labelled so in the test. Totals
  per row do not depend on it.
- **Lifelink before Phase D riders (C-D2 prompt order)** — the record's documented emission order
  (gains, aggregate, Phase D), an engine convention; the test keeps asserting the first prompt
  offers Rhox Faithmender, as today.
- Every other expected value is derived from the card text and the CR in §16 and is not copied from
  the engine. The base's readings these rows replace (candidate 0, a declined "may", a cleared own
  prompt) disagree with the cards; they are the defect P2 removes, not references.
- **New Way Forward (C-D4)** — a derivation disagrees with the engine in a way P2 does not fix: "When
  damage is prevented this way, …" is a triggered ability (CR 603.2, CR 615.13) that goes on the
  stack before the active player gets priority (CR 603.3, CR 510.3a), while the engine runs it as a
  rider inside the batch — the same gap P1 recorded for Outfitted Jouster. Who is asked, how many
  cards and how much damage are the card's; the timing is not. The row keeps a `strict-failure`
  line naming that gap. Fixing it needs the parser and a trigger seam outside the charter (§19 Q-3).
- **Command Zone #174** (counters "for each 1 damage" parsed one counter per event): the converted
  Gatta rows move to a 1-power blocker (Eager Cadet) so one counter is prevented and the card and
  engine agree exactly; Temper rows use X = 1. #174 stays witnessed by the existing noncombat rows
  in `damage_substitution_replacements` (`an_earlier_riders_paused_counters_finish_before_the_substitution`).

- **C-D3 and C-D5 accept arms, and N-5–N-7 (D7)**: a pre-existing defect, taking the
  defective-reference route. The derived value is the amount the shield prevented: 3, 7, 3, 1 and
  3 per prompt, from Channel Harm's text and CR 615.5 (§4 D7). The measured reference is the
  engine's reading after an optional prompt inside a replacement action: 0 at `8b16c4a374`, from
  the executor's round 1. For N-6 the reference at base is predicted, not measured: 4 (E6). The two
  disagree, so the reference is wrong for the card. This plan fixes it (D7), and every row asserts
  the derived value. Recorded for the phase-fit record (§19).

## 14. Serialized surfaces

- `PendingCombatDamageReplacement.settled_guards` (new, `serde(default, skip_serializing_if =
  "Vec::is_empty")`). An older reader drops it and re-reads the guards at its next entry (silent
  capability loss).
- `PendingCombatLifelink` can now be parked in Phase D (empty `remaining`, `replacement: None`,
  partial `prevention_tally`): no new field; an older reader resumes it through the same path.
- (U3) `OptionalEffectFrame.replacement_action` (new, `serde(default, skip_serializing_if =
  "Option::is_none")`), inside `GameState.resolution_stack`, which the full protocol and the P2P
  wire carry. An older reader drops it. A "may" a newer peer parked inside a replacement action
  then resolves outside it, and its "that much" reads 0: silent capability loss. A newer reader of
  an older save gets `None`, which is the base behavior.
- Registry step, delivered for U1/U2: per addendum Q-2 the executor created
  `protocol/changes/a26a16db-3963-4e52-a838-8473d6808198.json` (`full`, `wire`), whose reason
  names `settled_guards` and the Phase D park. The branch's earlier record `ccf4c183…` is untouched.
- **Registry step for U3: a new record**, `node scripts/protocol-versions.mjs bump game --reason "An
  optional-effect prompt reached inside a replacement's own action carries that action's amount
  (OptionalEffectFrame.replacement_action), which an older reader drops, so the accepted action
  reads no amount"`, then `node scripts/protocol-versions.mjs generate`, `node
  scripts/protocol-versions.mjs check`, `node scripts/check-protocol-version.mjs --base 91639fdf02`.
  Why a new record and not an amended `a26a16db…`:
  - Amending would pass the tooling. `checkProtocolHistory` checks append-only history only against
    the base tree, and `a26a16db…` is absent at `91639fdf02` and at `00396db22d`.
  - The documented rule has no carve-out for unreleased records: "Do not edit the baseline,
    alter/delete existing records" (project-reference, Protocol version changes). Once `8b16c4a374`
    holds `a26a16db…`, it is an existing record.
  - The run already applied that rule this way in addendum Q-2.
  - A new record stays right if U3 becomes its own phase (§5, §19).
  - A second bump in one PR moves `full` and `wire` by two instead of one. The project-reference
    note "these versions are not schema fingerprints" makes that harmless.
- `GameState` size: unchanged (`pending_combat_lifelink` is boxed); no `game_state_size.rs` entry.

## 15. Skills and checklist steps (Step 1)

- `/add-interactive-effect`: no new `WaitingFor`/`GameAction` (existing `ReplacementChoice`,
  `OptionalEffectChoice`, `EffectZoneChoice`); Phase 2 resolver/continuation = D1–D3; Phase 3 engine
  handler = D5; Phase 4 AI legal actions: unchanged prompts, existing candidates; Phase 5 routing:
  `acting_player()` of the existing prompts; Phase 6 frontend: none (no new prompt type); Phase 7
  visibility: `pending_combat_lifelink` is already filtered (`visibility.rs`); Phase 8 tests = §16
  (production round trip, continuation, decline, resume-state via V1).
- `/add-replacement-effect`: pipeline flow and post-replacement continuation (D1, D2), no new
  `ReplacementDefinition`.
- `/card-test`: production entry and completion boundary per §12.
- `/add-card-data-pipeline`: coverage fence removal; the coverage report is measured at run-level
  verification (charter decision 5); `card-data.json` is not regenerated in P2.
- `/add-engine-variant`: one coverage variant deleted; no sibling cluster forms; no inventory gate
  for a deletion. `DEFERRED(phase 3)`: the `ShieldKind::Substitution` deletion and its gate. (U3)
  adds a struct field, no enum variant, so no inventory gate.
- (U3) `/add-interactive-effect` continuation pattern: the optional-effect `WaitingFor` round trip
  keeps its resolution context across the prompt (Phase 2 resolver/continuation = D7's capture and
  R1–R4). No new `WaitingFor`/`GameAction`. AI legal actions, routing and frontend are unchanged:
  the same prompts and the same answers. Phase 8 tests: N-5 save/load at the prompt, decline
  paths, UT-1/UT-2.
- (U3) `/add-replacement-effect`: the post-replacement continuation's amount survives a child
  prompt for every frame that holds the action's work. Before D7 that was `PendingContinuation`
  only.

## 16. Tests

### 16.1 Helper changes (Step B1, first)

- `unpreventable_damage_prevention_effects.rs`: new `pub(super) fn fight_driving(runner, attackers,
  blocks, answer: impl FnMut(&mut GameRunner) -> GameAction) -> Combat` holds today's
  `fight_answering` loop; `fight_answering` becomes `fight_driving(.., |runner|
  answer(runner.state()))`. Inside the loop, an action taken at a wait other than `Priority`,
  `OrderTriggers`, `DeclareAttackers`, `DeclareBlockers` or `AssignCombatDamage` is an answer; after
  it, when the new wait is `Priority`, assert `runner.state().pending_combat_lifelink.is_none()` with
  the message "CR 510.2 + CR 615.5: a parked combat batch outlived its answer". Also record each
  prompt's chooser: `Combat` gains `choosers: Vec<PlayerId>` pushed with `prompts`.
- `damage_substitution_replacements.rs`: `combat_answering` gets the same invariant after each
  `choose_source`; `temper` and `cast_temper_at_one` become `pub(super)` (both files are registered
  in `tests/integration/main.rs`, so the cross-module use is allowed).

### 16.2 Converted rows (Phase D; charter test obligations)

| Id | Test (old → new name) | Board | CR / ruling | Today | Derived expectation, per answer |
|---|---|---|---|---|---|
| C-D1 | `gatta_and_luzzus_counter_order_is_settled_after_the_batch` → `…_is_asked_after_the_batch` | P0's Gatta and Luzzu on P0's Grizzly Bears, blocked by P1's **Eager Cadet** (was a 2/2); P0's Hardened Scales + Doubling Season | 615.5, 616.1; Hardened Scales ruling | `[6,7,8].contains`, no prompt | 1 prevented → one counter event; P0 asked once, offered {Scales, Season}: Scales first (1+1)×2 = **4**, Season first 1×2+1 = **3**; without Scales/Season **1**, no prompt. Bears 0 damage; Cadet dies; Bolt kills the Courser |
| C-D2 | `a_lifelink_answer_then_gatta_and_luzzus_rider` | P0's Child of Night (Gatta's target) blocked by P1's **Eager Cadet**; P0's Rhox Faithmender + Leyline of Hope; ± Scales/Season | 702.15b, 616.1, 615.5 | 1 prompt, counters in a set | Prompt 1 (P0) offers Rhox: answered Rhox → life +2×2+1 = **+5**. With Scales/Season, prompt 2 (P0) the counter order: Scales **4**, Season **3** (2 prompts in all); without, **1** counter and 1 prompt. Child 0 damage; Cadet dies |
| C-D3 | `channel_harms_may_is_settled_after_the_batch` → `…_is_asked_after_the_batch` (delivered; the accept arm is red until D7) | P0's Channel Harm targeting P1's Hill Giant; P0's Grizzly Bears attacks; blocked or not | 615.5, 608.2d; P3, P4 | "may" cleared, Giant 2 | Blocked: Giant's 3 prevented, P0 asked once (E4). **Accept** → Channel Harm deals **3** to the Giant (`DamageDealt { source_id: channel_harm, amount: 3, is_combat: false }`), with the Bears' 2 → Giant in the graveyard. **Decline** → Giant `damage_marked == 2`, on the battlefield. Unblocked: nothing prevented, no prompt, P1 at **18**. Bears 0 damage throughout |
| C-D4 | `new_way_forwards_draws_are_kept_under_abundance` → `new_way_forwards_draws_ask_abundance` | P1's New Way Forward choosing P0's unblocked Grizzly Bears; P1's Abundance or not | 615.5, 616.1, 121.2 | declined unasked | P0 life **−2** once; P1 life unchanged; P1 hand **+2** either way. With Abundance: **2** prompts, chooser **P1**; decline → 2 `CardDrawn` (P1); accept + nonland → **0** `CardDrawn`. Without: 2 drawn, no prompt. `strict-failure` line: the card's "When damage is prevented this way" is a trigger (CR 603.2, CR 603.3, CR 510.3a), run here as a rider inside the batch (§13) |
| C-D5 | `a_may_park_and_a_riders_own_prompt_asks_abundance_and_settles_channel_harm` → `a_may_park_and_a_riders_own_prompt_are_both_asked` (delivered; the accept-Harm rows are red until D7) | Two P0 Grizzly Bears; P1's Hill Giant blocks the first, Swans of Bryn Argoll the second; P0's Channel Harm at the Giant; P0's Abundance or not | 616.1, 608.2d, 615.5 | Abundance asked, Channel Harm cleared | Channel Harm prevents the Giant's 3 and the Swans' 4 (**7**); the Swans prevent the Bears' 2. Rows: (Abundance, decline, accept Harm): 2 Abundance prompts then 1 Channel Harm prompt (all P0); 2 `CardDrawn`; Giant dealt **7** noncombat → graveyard. (Abundance, accept, decline Harm): 0 `CardDrawn`, hand +2; Giant `damage_marked == 2`. (no Abundance, accept Harm): 1 prompt; Giant → graveyard. Every row: `prevented == 2`; both Bears alive with 0 damage; Swans 0 damage |

### 16.3 P1 rows that gain the V1 invariant only (no expectation change)

`nefarious_lichs_exile_choice_is_asked_inside_the_batch`, `dralnus_sacrifice_choice_is_asked_inside_the_batch`,
`outfitted_jousters_sacrifice_in_combat` ((2,1) rows), `swans_of_bryn_argolls_draws_ask_abundance`,
`double_strike_asks_abundance_for_the_swans_draw_in_each_damage_step`,
`phytohydras_counter_order_is_asked_inside_the_batch`, every Ironscale row using `fight`/
`fight_answering`, and every `combat_answering` row in `damage_substitution_replacements`. Their
derived values stand (P1). Under E1 the first three are red until Step B6.

Unchanged and must stay green: `a_prevention_rider_that_ends_the_game_ends_the_batchs_riders`,
`nothing_happens_in_the_batch_after_a_lifelink_gain_ends_the_game`,
`nothing_happens_in_the_batch_after_the_game_ends`, `nefarious_lich_ends_the_game_inside_the_batch`,
`a_player_who_loses_inside_the_batch_leaves_and_the_batch_goes_on`, the guard rows, main's #9670
suites (`szadek_combat_damage_to_counters`, `adamantoise_vigor_combat_order`,
`combat_lifelink_replacement_ordering`, `awe_strike_prevention`, `swans_prevention_followup`),
`inkshield_prevented_this_way_token_rider`, `draw_from_general_post_replacement`. The ignored red
witness `outfitted_jouster_double_blocked_sacrifices_one_equipment` stays ignored.

### 16.4 New rows

- **N-1** (unpreventable file, Phase D section) `two_aggregate_riders_are_asked_one_after_the_other`.
  `rider_board()`; P0's Grizzly Bears and Hill Giant attack P1 unblocked. P1 controls Rhox
  Faithmender and Leyline of Hope. P1 casts Awe Strike at the Bears, then a second Awe Strike at the
  Giant (`cast_by_p1` + `drive`). Phase D: the Bears' shield prevented 2, the Giant's 3; each rider
  is a life gain meeting both replacements, P1 orders each (CR 616.1). For each answer pair
  (first, second) ∈ {Rhox, Leyline}²: prompt 1 at life 20; prompt 2 at life **25** (Rhox first:
  2×2+1) or **26** (Leyline first: (2+1)×2); final life: (Rhox, Rhox) **32**, (Rhox, Leyline)
  **33**, (Leyline, Rhox) **33**, (Leyline, Leyline) **34**. Exactly 2 prompts, both P1's, each
  offering {Rhox, Leyline}. At prompt 2, before answering, assert `pending_combat_lifelink` holds
  `replacement.is_none()` and `prevention_tally.len() == 1`, then round-trip the whole `GameState`
  through `serde_json` and continue on the restored state. P1 dealt no combat damage
  (`damage_dealt_to_player(.., P1)` empty); `prevented == 2`; `assert_batch_settled`. Revert probes:
  at the base there are 0 prompts; leaving the shield in the tally until after its rider gives a
  third prompt and +5 more life.
- **N-2** (damage_substitution file, beside `a_resident_rider_from_another_recipient_is_not_lost`)
  `an_earlier_riders_pause_inside_a_combat_leg_is_asked`. P1's Phytohydra blocks P0's Rumbling
  Baloth (4); P0 cast Temper with X = 1 at the Phytohydra (`cast_temper_at_one`); P1 controls
  Hardened Scales and Doubling Season. Prompt 1 (P1): {Temper, Phytohydra} order (CR 616.1; S9) —
  answer Temper. Temper prevents 1; Phytohydra replaces the other 3. Prompt 2 (P1): Temper's one
  counter, {Scales, Season}; prompt 3 (P1): Phytohydra's 3 counters, {Scales, Season} (CR 615.5:
  the prevention's additional effect immediately afterward; CR 616.1f: then the next effect applies
  to what is left). For (a, b) ∈ {Scales, Season}²: counters at prompt 3 = **4** (a = Scales,
  (1+1)×2) or **3** (a = Season, 1×2+1); final = that + **8** (b = Scales, (3+1)×2) or **7**
  (b = Season, 3×2+1): totals 12 / 11 / 11 / 10. Phytohydra damage 0; no post-replacement drain left;
  the batch completed by the last answer. Siblings: Phytohydra first → 1 prompt then one counter
  prompt of 4 (Scales (4+1)×2 = 10 / Season 4×2+1 = 9), Temper unused; Doubling Season alone →
  1 prompt, counters 1×2 + 3×2 = **8**. Revert probe: at the base, prompt 2 is settled with
  candidate 0 (2 prompts in all).
- **N-3** (unpreventable file, guard section) `undergrowth_champion_keeps_its_batch_reading_across_a_combat_park`.
  P0's Undergrowth Champion with no counter attacks; P1's two Grizzly Bears block it; P0 cast Temper
  with X = 1 at the Champion; P0 controls two Hardened Scales and Doubling Season. The Champion
  assigns 2 to the first Bear (CR 510.1c). The batch opens with the Champion's guard false, so its
  shield applies to none of the 4 damage (ruling). The first Bear's 2: Temper prevents 1 and its
  counter meets three effects: prompt 1 (P0) offers {Scales, Scales, Season}, prompt 2 offers the two
  left (CR 616.1f). Answer policies: Scales, Scales → **6**; Scales, Season → **5**
  ((1+1)×2+1). Variant A in memory; variant B round-trips `GameState` through `serde_json` at
  prompt 2. Both: exactly 2 prompts, none offering the Champion; Champion `damage_marked == 3` (1
  after Temper + the second Bear's 2), on the battlefield; `prevented == 1`. Revert probes: at the
  base (fresh read at the resume) the second Bear's damage is prevented by the Champion: damage 1,
  counters 5 / 4; with `settled_guards` not serialized, variant B gives the same wrong result.
- **N-4** (unpreventable file, Phase D section) `a_phase_d_rider_prompt_whose_player_leaves_lets_the_batch_finish`.
  `GameScenario::new_n_player(3, 42)` at P0's precombat main, eight library cards each. P1 controls
  Rhox Faithmender and Leyline of Hope and casts Awe Strike at P0's Grizzly Bears; the Bears attack
  P1 unblocked. At P1's Phase D prompt (assert `ReplacementChoice { player: P1 }` and the record's
  `replacement.is_none()`), P1 concedes. Then act on at most 16 waits (pass priority, order triggers)
  until `pending_combat_lifelink.is_none()`. Expected: drained, with `regular_damage_done` true;
  P1 eliminated, no further prompt to P1; P0 and P2 at 20. The departed P1 receives no gain.

### 16.5 New rows for U3 (D7; red at base)

Entry and completion boundary (card-test): `GameRunner` actions through `GameRunner::act` (cast,
pass priority, `DecideOptionalEffect`). The answering action returns `Priority`, then
`assert_settled` (no frame, an empty stack). For N-6 the boundary is the end of combat with an
empty stack. Section header in `unpreventable_damage_prevention_effects.rs`, after the Phase D
rows: "A rider's own "may" keeps the prevented amount (CR 614.6 + CR 615.5 + CR 608.2d)".

- **N-5** `channel_harms_may_deals_the_prevented_amount_after_a_spell`.
  - Board: `rider_board()`. P0's Grizzly Bears; P1's Hill Giant (3/3); P0's Channel Harm
    (`spell(.., CHANNEL_HARM, true)`); P1's Lightning Bolt
    (`scenario.add_spell_to_hand_from_oracle(P1, "Lightning Bolt", true, BOLT)`).
  - P0 casts Channel Harm at the Giant (`cast`). P1 casts Bolt (`cast_by_p1`) at P0's Bears
    (`drive` with the Bears as its target), or at P0 in the player variant.
  - At the prompt, assert:
    - `WaitingFor::OptionalEffectChoice { player: P0, .. }` (E4's sibling);
    - `state.active_optional_effect_frame().unwrap().replacement_action ==
      Some(ReplacementActionScope { amount: 3 })`;
    - `prevented(&events) == 1`.
  - Rows (target, answer, save/load at the prompt):

    | Bolt target | Answer | Save/load | Expected |
    |---|---|---|---|
    | Bears | accept | no | exactly one `DamageDealt { source_id: channel_harm, amount: 3, is_combat: false }`, to the Giant; Giant in the graveyard |
    | Bears | accept | yes | the same |
    | Bears | decline | no | no Channel Harm damage; Giant `damage_marked == 0`, on the battlefield |
    | P0 | accept | no | the same as the first row; P0 at 20 |

  - Every row: the Bears have 0 damage and stay on the battlefield, P0 is at 20, and the board ends
    at `assert_settled`.
  - Revert probes:
    - at the base (E5) nothing is dealt and the Giant survives;
    - with `#[serde(skip)]` on the field, the save/load row deals nothing.
- **N-6** `channel_harms_may_reads_the_prevented_amount_inside_a_resolving_trigger`.
  - Board: `rider_board()`. P0's Shockmaw Dragon
    (`creature_with_keywords(.., P0, "Shockmaw Dragon", (4, 4), &["Flying"], SHOCKMAW_DRAGON)`,
    the constant copied from `damage_substitution_replacements.rs`); P1's Grizzly Bears; P1's
    Channel Harm in hand (`add_spell_to_hand_from_oracle(P1, ..)`).
  - A dedicated drive loop, because `fight_driving` passes every priority:
    1. P0 attacks P1 with the Dragon, unblocked, and the loop passes priority through combat damage.
    2. With Shockmaw's trigger on the stack and `WaitingFor::Priority { player: P1 }`, P1 casts
       Channel Harm targeting the Dragon.
    3. The loop passes priority until Channel Harm, then the trigger, resolves.
  - At the prompt, assert:
    - `OptionalEffectChoice { player: P1 }`;
    - P1 at 16 (Channel Harm was cast after the combat damage);
    - `prevented == 1`;
    - the frame's amount is 1.
  - Accept → exactly one `DamageDealt { source_id: channel_harm, amount: 1, is_combat: false }`, to
    the Dragon, which stays on the battlefield with `damage_marked == 1`.
  - Decline → the Dragon has 0 damage.
  - Both: P1's Bears have 0 damage; the combat ends with an empty stack.
  - Revert probe: at the base the accept arm reads the trigger's event amount (E6: 4), and the
    Dragon is destroyed.
- **N-7** `two_channel_harms_loop_until_a_player_declines` (ruling P4).
  - Board: `rider_board()`. P0's Grizzly Bears and Centaur Courser; P1's Hill Giant; P0's Channel
    Harm at the Giant; P1's Channel Harm at P0's Bears; P1's Lightning Bolt at P0's Courser.
  - P0 casts and resolves its Harm, then P1 casts and resolves its Harm (`cast_by_p1` + `drive`),
    then P1 casts the Bolt.
  - Answer the prompts in order from each row's list, recording each chooser and its frame amount:

    | Answers | Choosers | Note |
    |---|---|---|
    | [accept, accept, decline] | [P0, P1, P0] | each prompt's amount 3 |
    | [accept, decline] | [P0, P1] | |
    | [decline] | [P0] | |

  - Every row: no `DamageDealt` event at all ("no damage will be dealt in either case"); the
    Courser, Giant and Bears have 0 damage and stay on the battlefield; `assert_settled`.
  - Revert probe: at the base the first row has one prompt (E7).
- **UT-1** (`crates/engine/src/game/effects/mod.rs`, module `replacement_action_continuation_tests`,
  beside its typed seam witnesses, with the module's "no printed card" comment convention: the
  nearest printed reader is Channel Harm, N-5)
  `a_may_reached_inside_a_replacement_action_resolves_inside_it`.
  - Ability: `let mut may = action_life(); may.optional = true;` ("you may gain that much life").
  - (a) In scope: resolve it with `effects::resolve_ability_chain` inside
    `with_replacement_action_scope(Some(2), ..)`. Assert `WaitingFor::OptionalEffectChoice` and the
    frame's `Some(ReplacementActionScope { amount: 2 })`. Then, outside the scope, answer
    `DecideOptionalEffect { accept: true }` through the engine's action entry (`engine::apply`, or
    a `GameRunner` as the module's tests use). Expect life +2 and `current_replacement_action() ==
    None` afterwards. Decline gives life +0.
  - (b) The same ability with no scope: the frame's field is `None`. This pairs with (a)'s `Some(2)`
    as the positive control.
  - (c) `with_replacement_action_scope(None, ..)`: `None`.
  - Revert probe: without R1, (a) accept gains 0.
- **UT-2** (same module) `an_any_player_may_inside_a_replacement_action_resolves_inside_it`.
  - Ability: `action_life()` with `optional_for` set to the opponents' scope, and a sub-ability
    gated by the "no one performed it" condition `declined_by_every_seat_branch` reads ("If no one
    does, gain that much life", built the way that function reads it).
  - Inside `Some(2)`: the `OpponentMayChoice` frame carries `Some(2)`.
  - Accept by P1 → the accepted gain is 2 (R2). The re-offer, if one is made, carries `Some(2)`
    (the copy).
  - All decline → the "if no one does" gain is 2 (R3).
  - After each answer, `current_replacement_action() == None`.
  - Revert probes: without R2, the accept gains 0; without R3, the decline-all gains 0.

N-5–N-7 use `fight_driving`'s invariant only where they use it (N-6's own loop asserts no
`pending_combat_lifelink` after the combat, as `assert_batch_settled` does).

## 17. Step-by-step plan

Commits on top of `00396db22d`; only the candidate must be green. **Commits A–C are delivered
as the checkpoint `8b16c4a374`; the next executor round starts there and adds Commit D.** Run `cargo fmt --all` after each
step; Tilt-first verification as at the end.

### Commit A — U2, guards across a park

- **A1** `crates/engine/src/game/replacement.rs`: `CombatDamageReplacementPass` gains
  `settled_guards` (doc: CR 608.2f + CR 700.1, Undergrowth Champion ruling); `new(state: &GameState,
  events)`; `replace_combat_damage_batch` installs `take(&mut pass.settled_guards)` and moves
  `applications` and `settled_guards` back on exit; rewrite its doc's last paragraph ("installed on
  every entry with the pass's one-shot applications and the guard reading the pass took when it was
  created"). Fix the test at the `record_proposed_events` site to `CombatDamageReplacementPass::new(&state, [..])`
  (clone first if the borrow checker requires it).
- **A2** `crates/engine/src/types/game_state.rs`: `PendingCombatDamageReplacement.settled_guards`.
- **A3** `crates/engine/src/game/combat_damage.rs`: `apply_combat_damage` → `new(state, ..)`;
  park and restore `settled_guards` in `run_combat_damage_replacements` /
  `resume_combat_damage_replacements`.
- **A4** `crates/engine/src/game/effects/prevent_damage.rs` test: `new(&state, [proposed])`.
- **A5** N-3 (needs `temper` / `cast_temper_at_one` `pub(super)`; uses `fight_driving` from Step B1 —
  so add `fight_driving` here, without the invariant, and the invariant in B1).
- **A6** Protocol record reason (§14) + `generate` + `check`.

### Commit B — U1, every combat rider pause asked

- **B1** Helpers (§16.1): the invariant in `fight_driving` and `combat_answering`; `choosers`. Run
  `dralnus_sacrifice_choice_is_asked_inside_the_batch` alone to establish E1 (expected red).
- **B2** `replacement.rs`: `dispatch_damage_rider_outside_batch` (doc: CR 700.1); `drain_damage_rider`
  without its combat branch, on top of it (doc: CR 614.6 + CR 615.5 + CR 608.2c, now naming the
  combat leg that parks on the pause); `drain_combat_damage_rider -> Option<WaitingFor>` (doc: CR 615.5
  + CR 510.2 + CR 700.1: returns the pause, settles nothing); `drain_combat_damage_continuation`
  calls it. Delete `CombatRiderPause`, `settle_combat_rider_pauses`, `answer_combat_rider_park`,
  `abandon_combat_rider_pause`. Update `DamageRiderDrain` variant docs.
- **B3** `replacement.rs`: D2 in `settle_combat_damage_replacement` + `take_combat_leg_paused_rider`
  (doc: CR 615.5 + CR 608.2c + CR 616.1g). `types/game_state.rs`: `DamageBatchLedger::paused_rider`
  doc ("In combat the leg it paused in parks on it (`settle_combat_damage_replacement`)").
- **B4** `combat_damage.rs`: D3 (`fire_combat_prevention_riders` signature, pop-first loop, latched
  `event_amount`, return the pause; `drain_combat_lifelink` parks). Docs: `fire_combat_prevention_riders`
  (CR 615.5 + CR 615.13 + CR 608.2d + CR 104.1), `drain_combat_lifelink`, `CombatDamageBatch::Paused`
  (fourth park point). `types/game_state.rs`: `PendingCombatLifelink` and its `prevention_tally` docs.
- **B5** Tests C-D1…C-D5, N-1, N-2, N-4; rewrite the converted rows' strict-failure comments (the
  #140 sentences go; C-D4 gains the trigger-timing line).
- **B6** (if E1 held) `combat_damage.rs`: `resume_parked_combat_damage_after_answer` (doc: CR 510.2
  + CR 510.3 + CR 510.3a + CR 615.5 + CR 616.1g); `engine.rs`: capture `action_event_start`, call it
  after `resume_parked_counter_cost_root_at_action_boundary` (comment: CR 510.2 + CR 615.5);
  narrow `resolve_combat_damage`'s re-entry comment to the departure paths (CR 800.4a); comment-only
  update in `engine_replacement::handle_replacement_choice`'s doc.

### Commit C — fence and documents

- **C1** `coverage.rs`: D6 deletions; grep each removed import. `resolution_prompt.rs`: restore the
  two functions' private visibility (optional, §19).
- **C2** Test-module docs: `unpreventable_damage_prevention_effects.rs` lines 19–27 become: "A
  rider's pause inside a combat damage batch is asked of the player it belongs to, at the
  per-portion drain and at the aggregate drain after the batch's damage (CR 615.5, CR 616.1,
  CR 608.2d), and the answer completes the batch in the same action: CR 510.2 rules out a priority
  window, not the choice. The last section holds those rows; the Ironscale Hydra rows are in
  `ironscale_hydra_combat_prevention`." `damage_substitution_replacements.rs` lines 15–18 become:
  "- Combat damage asks every CR 616.1 order inside its batch, and every prompt a substitute or a
  rider raises there, of the player it belongs to (CR 510.2 rules out a priority window, not the
  choice); the answer completes the batch." The Dralnu comment at the
  `damage_replacement_choice_unasked_in_combat` mention keeps its CR 614.6 + CR 615.5 / #172 order
  note and names `structural:damage_substitution_choice_incomplete` as what keeps Dralnu unsupported.
  The two "Rider pauses inside a combat batch (CR 510.2, CR 616.1; Command Zone #140)" section
  headers become "Rider pauses inside a combat batch, asked (CR 510.2, CR 616.1)".
  `combat_lifelink_replacement_ordering.rs`: the parameter-type doc line now reads
  `&mut Vec<(AppliedReplacementKey, i32)>`, popped one shield at a time.
- **C3** `python3 scripts/post_replacement_continuation_census.py --write`, then run it without
  `--write`. Expected delta: rows `replacement.rs drain_combat_damage_continuation consumer 1`,
  `replacement.rs drain_combat_damage_rider consumer 1`, `replacement.rs drain_damage_rider consumer 1`
  go; `replacement.rs dispatch_damage_rider_outside_batch consumer 1` comes. Review each row in the
  commit message.
- **C4** `grep -rn "damage_replacement_choice_unasked_in_combat\|DamageReplacementChoiceUnaskedInCombat\|settle_combat_rider_pauses\|CombatRiderPause\|answer_combat_rider_park\|abandon_combat_rider_pause\|Command Zone #140" crates/ scripts/` returns nothing.

### Commit D — U3, a replacement action's "may" keeps its amount (defective-reference repair)

- **S1 (first, establishes E5–E7):** add N-5, N-6, N-7, UT-1 and UT-2 (§16.5) and run them before
  any production edit of this commit. Each accept arm must fail as §3 predicts. Report the measured
  base values (they go to the phase-fit record, §19).
- **S2** `crates/engine/src/types/resolution.rs`:
  - add the `replacement_action` field and its doc (D7, CR 614.6 + CR 615.5 + CR 608.2c +
    CR 608.2d);
  - add `OptionalEffectFrame::live_offer(ability, state)`, carrying the moved CR 608.2 /
    CR 603.2c field comments;
  - set `replacement_action: None` in `LegacyOptionalEffectWire::into_frame`, with the comment "v1
    wire predates the replacement-action carrier".
- **S3** `crates/engine/src/game/effects/mod.rs`: the three park sites use `live_offer`
  (`offer_optional_in_apnap_order`, both installs in `resolve_chain_body`). Add the CR 614.6 +
  CR 615.5 comment at the `OptionalEffectChoice` site: "a "may" reached inside a replacement's
  own action is offered with that action's amount (`OptionalEffectFrame::replacement_action`)".
- **S4** `crates/engine/src/game/engine_payment_choices.rs`: R1–R4 and the re-offer struct update,
  each with a CR 614.6 + CR 615.5 + CR 608.2c comment naming what the guard covers and that it
  drops before `resume_pending_continuation_if_priority`.
- **S5** Compiler-forced: `replacement_action: None,` in every remaining `OptionalEffectFrame {..}`
  literal (§5 paths 16–28, plus the test literals in `engine_payment_choices.rs`,
  `types/resolution.rs`, `types/game_state.rs`, `game/engine.rs` and `effects/mod.rs`). Regenerate
  the list with `grep -rn "OptionalEffectFrame {" crates/ --include=*.rs`. No other change in those
  files.
- **S6** Protocol: `node scripts/protocol-versions.mjs bump game --reason "<§14 text>"`, then
  `generate`, `check`, and `node scripts/check-protocol-version.mjs --base 91639fdf02`.
- **S7** Re-run S1's rows (green), C-D3 and C-D5 (accept arms green), and the module tests of
  `engine_payment_choices`, `effects` and `types::resolution` (the legacy-wire and frame-transition
  tests). Run `cargo fmt --all`.

### Verification (Tilt first; never both Tilt and direct cargo)

`tilt get uiresource clippy` to detect Tilt. With Tilt: `./scripts/tilt-wait.sh clippy test-engine`
(exit 3 = cannot answer, not a failure). Focused first: the three P2 test files, main's #9670 suites
named in §16.3, `combat_lifelink_replacement_ordering`, `inkshield_prevented_this_way_token_rider`,
`awe_strike_prevention`, the coverage unit tests, `game_state` serde tests; for Commit D also
`damage_substitution_replacements` (`a_replacements_action_reads_its_own_amount_inside_a_resolving_trigger`),
`optional_effect_remember_legal_actions`, `cr733_resolved_frame_transition`,
`captain_marvel_apex_avenger`, the `phase-ai` unit tests that build frames (`sacrifice_value`,
`planner`), and the engine lib tests of `engine_payment_choices`, `effects`, `types::resolution`
and `elimination`. Then the census check and
the protocol `check`. Deferred to run-level verification (charter): `cargo coverage` flip list (V11),
workspace clippy, all engine shards, phase-ai `--lib`, wasm32, GameState size (unchanged: the
frame lives in the boxed resolution stack), the Kathril stack test (U3 widens
`OptionalEffectFrame` by one `Option<ReplacementActionScope>`, which matters only if that frame
is the widest `ResolutionFrame` variant).

## 18. Unprobed assertions (each settled at implementation by the named test)

- E1 (B1 run of the Dralnu row), E2 (N-2 prompt 2's offer), E3 (N-1 prompt-2 snapshot), E4 (C-D3).
- Elimination abandons a departed player's paused Phase D rider frame and pending replacement, and
  the priority-pass re-entry then finishes the record (N-4).
- The two identical Hardened Scales plus Doubling Season ask two prompts for one counter event when
  a Scales is chosen first (N-3; Ironscale C-21 in P1 asks the same board).
- Channel Harm's aggregate amount in C-D5 is 7 (3 + 4): `combat_prevention_tally` aggregates per
  shield across the batch (`batched_combat_all_shield`).
- Temper with X = 1 parses as one prevented point and one counter event (its "for each 1 damage"
  repeat runs once).

- (U3) E5, E6, E7 (S1's base runs), and E8 (the focused suites).
- (U3) `drive` stops at `OptionalEffectChoice` (it returns on any wait it does not answer), so N-5
  and N-7 read the prompt after `drive` returns.
- (U3) In N-6, P0 receives priority first with Shockmaw's trigger on the stack, and one pass gives
  P1 priority with the trigger still there (CR 117.4 resolves nothing until all pass). Its drive
  loop asserts this before casting.
- (U3) In N-7, P1's Channel Harm prevents P0's Channel Harm's damage to P1's Giant (P1 doesn't
  control that source), and P0's Channel Harm prevents P1's Channel Harm's damage to P0's Bears.
  The ruling's loop needs both.
- (U3) No `OptionalEffectFrame` literal uses struct-update syntax today, so S5's grep list is
  complete.

## 19. For the orchestrator / human

- **Addendum (scope).** D5 adds one call in `crates/engine/src/game/engine.rs`, outside P2's anchor
  paths; D6's optional cleanup touches `crates/engine/src/game/resolution_prompt.rs`. Neither breaks
  a charter decision: decision 3's park and resume authority stand, and D5 is the call site its
  "asked, never auto-settled" goal needs once E1 is established. If E1 is false, D5 is dropped.
- **Q-1 (scope, D5): accept that P2 also makes P1's per-leg own-prompt rows (Nefarious Lich, Dralnu,
  a two-Equipment Outfitted Jouster) finish in the answering action.** They open a priority window
  at the base (E1). Fixing it in P2 is the defective-reference route; the alternative is a follow-up
  and a fence kept for own-prompt riders, which the charter's goal rules out.
- **Q-2 (protocol), settled by addendum Q-2:** U1/U2 took a new record (`a26a16db…`, delivered).
  U3 takes another new record (§14), not an amendment of `a26a16db…`.
- **Q-3 (follow-up, not P2): New Way Forward's "When damage is prevented this way" runs as a rider,
  not a trigger** (CR 603.2, CR 603.3, CR 510.3a), the same gap as Outfitted Jouster's (P1's
  Q-J). C-D4 keeps a strict-failure line; filing it with Jouster's follow-up is recommended.
- **Q-4 (sizing, U3): the T1∧T2 conjunction fires with D7 in P2** (§5: 3 units, 28 counted paths;
  15 without the compiler-forced test literals). No correct U3 keeps P2 under 13. The two rules
  that apply point different ways, and the orchestrator decides between them.
  - The defective-reference route (engine-implementer SKILL.md) says the repair "is not a stop and
    needs no expansion case under any budget". It goes back for re-charter only if it breaks a
    decision or "sizes above one unit (Sizing T1)", and "Short of that, it never inserts a phase."
    D7 is one unit and breaks no charter decision: decisions 1–5 stand, and D7 touches none of
    their seams.
  - chartered.md's re-adjudication says: "With landed candidates requiring a new split, stop and
    propose a re-charter through an expansion case rather than restarting or truncating the
    phase." `8b16c4a374` is a landed candidate.
  - Two routes, both prepared by this plan:
    - **(a) In-phase** (my reading of the defective-reference route): record the firing and the
      reason in the phase-fit record, then extend `SCOPE_PATHS` with §5 paths 13–28 (an addendum
      line naming decision "none broken; defective-reference repair", card Channel Harm). Dispatch
      Commit D on `8b16c4a374`. Thirteen of the 16 added paths are single-line compiler-forced
      literals.
    - **(b) Split**: re-charter U3 as its own phase first, on `00396db22d` (1 unit, 19 paths: does
      not fire), then the delivered U1+U2 replayed on its candidate. The order keeps every phase
      green; the reverse would leave C-D3's and C-D5's accept arms red.

    Under budget `standard`, (b) needs the expansion case: open item = the red accept arms; spend
    = one charter-mode partition and review, plus a replay of `8b16c4a374`; prediction = C-D3 and
    C-D5 green at the U1+U2 phase's candidate.
- **Phase-fit record (defective-reference step 1):**
  - card: Channel Harm, Oracle text as §0;
  - derived reading: "that much" = the amount prevented, by CR 615.5, whenever the "may" is
    answered;
  - measured reference: 0 at `8b16c4a374` (executor round 1);
  - affected rows: C-D3 accept, C-D5 accept-Harm rows; new N-5, N-6, N-7, UT-1, UT-2.
- **Addendum line** (when route (a)): "(scope, P2 plan r2) `types/resolution.rs`,
  `game/effects/mod.rs`, `game/engine_payment_choices.rs`, plus the compiler-forced
  `OptionalEffectFrame` literal files listed in plan r2 §5 (16–28): carry a replacement action's
  amount across its "may" prompt (CR 614.6 + CR 615.5 + CR 608.2c). Decision touched: none;
  defective-reference repair. Card: Channel Harm ("you may have Channel Harm deal that much damage
  to target creature") deals 0 when accepted."
- **Commit trailer.** `8b16c4a374`'s message ends with a `Co-Authored-By:` trailer that names a
  model. task.md says "Never write model identifiers into code, commits or PR text". Reword that
  commit message before the push (the planner made no change).
- No other human decision. Coverage flips (V11) are measured and listed at run level. U3 changes
  no coverage status by itself (Channel Harm's parse is unchanged). It makes true the support V11
  expects for Channel Harm once D6 removes the fence.
