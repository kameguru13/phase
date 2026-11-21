# Phase 2 implementation review r1 (task 21, Immortal Obligation, phase-rs/phase#9608)

- Mode: `review-engine-impl` **Phase Mode** (chartered run, phase 2 of 2, deferral allowlist empty) composed with Engine-Implementer Checkpoint Mode.
- Under review: implementation commit `dccb1ec0eeb4c554375539403cb44805a7210d94` on `PHASE_BASE_SHA` `64fac66`, as merged onto phase 1's reviewed head: branch `card/immortal-obligation` at `50dc4841100b06ebf30b9b27b3640b66ee56dff3` (first parent `dccb1ec`, second parent `2b906d3`). Reviewed span `2b906d3..50dc484`.
- Chain: `dccb1ec^ == 64fac66` (first round, `START_SHA == PHASE_BASE_SHA`). `64fac66` is an ancestor of `2b906d3`. The `crates/` diff `64fac66..dccb1ec` equals `2b906d3..50dc484` apart from hunk offsets and index lines (diffed after removing those lines). `git show --remerge-diff 50dc484`: the only conflict is `docs/parser-misparse-backlog.md` header and summary. The resolution is delta-only: 4472/4505 → 4471/4504, summary denominators 4506 → 4505, numerators unchanged, § 21 52 → 51 with 51 bullets. IO is gone from the backlog, Soul Partition is still listed, and `confirmed-misparses.toml` has no IO row.
- Worktree left clean (`git status` empty, HEAD `50dc484`). The one probe used a temporary untracked file, deleted afterwards (§ Probe).

```text
Review Head: 50dc4841100b06ebf30b9b27b3640b66ee56dff3
Completion Gate: DEFERRED
Maintainer-Simulation Gate: FAIL
```

**Completion Gate.** The Step 5 evidence on `50dc484` covers:
- fmt;
- clippy `--workspace --all-targets -D warnings`;
- the parser lib (12921/0);
- all five engine integration shards (0 failed);
- pre-commit hooks;
- a card-data whole-entry diff against the `2b906d3` export: exactly Immortal Obligation and Soul Partition (C2.2).

Plan Step 5 item 9's `combat.rs`/`layers.rs` **engine lib unit tests** were not run: `h2-plan/debug/deps` has no `phase_engine-*` test binary. They are deferred to the final boundary (or PR CI). By source inspection the change cannot move them. `blocker_filter_context(state, src, None)` is `FilterContext::from_source(state, src)`, and every lib unit test that builds a `BlockRestriction` (`combat.rs:23098`, `:23284`, `source_pronoun_cant_block_restriction_scopes_combat_pair`) has `source_controller: None`. The anchor arm cannot stamp them either: they are not grafted. A fix round for F1 re-opens the build-backed checks anyway.

**Maintainer-Simulation Gate: FAIL.** Every row in the plan's matrix has its test, and the tests match the rows (T-P1–T-P5, V1–V8, the V5 partial-removal sibling, V7(b) through Fate Transfer with the CR 611.2b no-resume assertion). The gate fails because the matrix lacks a hostile row the maintainer has already required of this run (phase#11229 F1, addendum phase-1 #2): an ability-removal effect after the restriction is in force. That row fails on the candidate (F1). The plan also maps the bearing ruling "Being goaded isn't an ability … even if it loses all abilities" to V2, and V2 has no ability-removal stimulus.

## Findings

**[HIGH]** `[behavior]` (charter decision finding, D2.3/D2.4/C2.6): a later "lose all abilities" effect ends Immortal Obligation's "can't attack you or a permanent you control" and "can't block creatures you control". Evidence:
- Card: Immortal Obligation: "For as long as that creature has a duty counter on it, it is goaded, can't attack you or a permanent you control, and can't block creatures you control."
- Ability-removal cards: Dress Down: "Creatures lose all abilities." Turn to Frog: "Until end of turn, target creature loses all abilities and becomes a blue Frog with base power and toughness 1/1."
- Rules: these restrictions are effects of a resolved spell, not abilities the creature has. CR 509.1b and CR 508.1c treat restrictions as "effects that say a creature can't block/attack". CR 113.1a says ability grants "usually use the words 'has,' 'have,' 'gains,' or 'gain'". CR 611.2c says such an effect "modifies the rules of the game". CR 613.1f's layer-6 ability removal therefore cannot end them.
- How the change builds them: both restrictions become grafted `static_definitions` on the creature: `GrantStaticAbility{CantAttack}` (the Promise of Loyalty route) and `AddStaticMode{BlockRestriction}` (`subject.rs::build_goaded_restriction_clause` → `build_restriction_clause`).
- What erases them: `layers.rs:13085` `RemoveAllAbilities` runs `obj.static_definitions.clear()`. It keeps only resolution-installed replacements (`replacement_definitions.retain(|d| d.is_resolution_installed())`, already annotated CR 611.2c), so a later-timestamp ability removal erases both grafts.
- **Measured on `50dc484`** (§ Probe; 3 players, production casts of the verbatim Immortal Obligation, then the verbatim Dress Down, both by P0):
  - after Immortal Obligation, `ret` has `[CantAttack/Some(P0), BlockRestriction{Not(Creature You)}/Some(P0)]` and is goaded;
  - after Dress Down, `ret` has `statics=[]` and is still goaded;
  - in P1's `DeclareBlockers` against P0's Grizzly Bears, `ret ∈ valid_blocker_ids` and `declare_blockers([(ret, c0)])` is `Ok(())`. It must be refused.
- The goad part is correct, because `combat.rs:7535-7550` reads the designation off the live TCE ("removing abilities from the recipient does not remove that designation"). The two restriction parts are not.
- In-reach witness with printed cards only: P1 casts Turn to Frog on its own returned creature during P0's turn, then blocks P0's attacker with it. Or Dress Down is in play during P1's next turn, and the creature attacks P0.

Why it matters:
- Immortal Obligation is the card this phase claims, and its restrictions end before the stated duration (CR 611.2b).
- This is the same CR 613.1f defect class that the maintainer held phase#11229 on (F1 at `4db932ad`). That PR fixed it by hosting the restriction on its source. A resolving spell has no permanent source, so that fix does not carry over, and the phase-2 PR will draw the same review.

Suggested fix (a decision for the orchestrator: the seam is outside this phase's `SCOPE_PATHS` and C2.6 asserted "no engine change" for the attack part, so this is an addendum on D2.4/C2.6 or a re-charter, not an in-scope fix):
- Make resolution-installed restriction grafts survive a later `RemoveAllAbilities`. The seam is the `layers.rs` `RemoveAllAbilities` arm (CR 613.1f + CR 611.2c), extended for statics grafted by a resolving spell's TCE the way it already retains resolution-installed replacements. The alternative is to read them off the live TCE at the combat consumers, as `goading_players_for_creature_gated` does for goad.
- Add an Immortal Obligation runtime row: production cast of Immortal Obligation, then a production cast of Dress Down. Assert:
  - `ret` is still goaded (this also discharges the plan's V2 ruling mapping);
  - `ret ∉ valid_blocker_ids` against P0's creature, and its block is refused with the block-restriction text;
  - with Dress Down in force on P1's turn, `[(ret, P0)]` is refused with `(CR 508.1c/d attack restriction)`;
  - a paired positive: `ret` may block a P2 attacker.
- Charter X1 holds for either fix (no new variant).

**[LOW]** `[text]`: the docs that list the installing-player anchor's members are stale after the new `BlockRestriction` arm. Evidence: `layers.rs:11497` now stamps `source_controller` for a controller-relative `BlockRestriction`, an attacker filter. Predicate and command: `rg -n 'MustBeBlocked\*` filters|controller-relative blocker filter' crates/engine/src crates/phase-ast/src` finds these two sites, and both need the edit:

1. `crates/phase-ast/src/types/ability.rs:43329-43331` (`StaticDefinition::source_controller` field doc; the executor's reported item; outside `SCOPE_PATHS`, comment-only standing class).
   - Old: `    /// sets it for controller-relative \`MustBeBlocked*\` filters and` / `    /// \`MustAttackAwayFromSource\`. Other granted statics, including quoted`
   - New: `    /// sets it for controller-relative \`MustBeBlocked*\` and \`BlockRestriction\`` / `    /// filters and for \`MustAttackAwayFromSource\`. Other granted statics, including quoted`
2. `crates/engine/src/game/layers.rs:13320` (the `AddStaticMode` stamp-site comment).
   - Old: `                // mode — a controller-relative blocker filter ("your opponents")`
   - New: `                // mode — a controller-relative blocker or attacker filter ("your opponents", "creatures you control")`

Why it matters: both list the stamped modes as `MustBeBlocked*` and `MustAttackAwayFromSource` only. Suggested fix: apply the two replacements as comment-only corrections. If F1's fix changes which defs are stamped, re-check the wording.

## Verified (no finding)

- **Phase-1 interface (addendum phase-1 #2).** At `2b906d3`, Burden of Proof's restriction is a source-hosted static (`grammar.rs::split_trailing_restriction_conjunct`). `static_mode_needs_grant_propagation` still admits `BlockRestriction`, with its comment now naming resolving-effect grants. `parse_restriction_modes`' "can't block <object>" arm is unchanged. Immortal Obligation's block conjunct reaches that arm and is propagated as `AddStaticMode{BlockRestriction{Not(Creature{You})}}` (card-data export). Phase 2 edits neither function.
- **C2.1 on the merged tree.** A `jq` census of `client/public/card-data.json` for `AddStaticMode` whose mode holds a `BlockRestriction` finds Cloudseeder (no controller), Craterous Stomp (no controller) and Immortal Obligation (controller `You`). Burden of Proof no longer grafts one. So the anchor arm stamps only Immortal Obligation.
- **Consumers.** `blocker_allowed_statics_for_from_precomputed` is read only at `combat.rs:3891` and `:8419`, and both now use `blocker_filter_context`. The remaining `BlockRestriction` references (`ai_support/filter.rs`, `analysis/resource.rs`, `ability_scan.rs`, `coverage.rs`, `effects/mod.rs`) never evaluate the filter against an attacker.
- **Parser.**
  - U1, U2 and U4 are nom-composed. The predicate list is `separated_list1` over a negation-gated join with `many_till(anychar, peek(...))`.
  - U4 declines without a stated duration, without a `ParentTarget` binding, on a non-restriction conjunct, and on a conjunct carrying its own duration.
  - The shared `goaded_designation_installer` serves both the plural arm and the new singular arm.
  - The singular no-tail forms keep their lowering.
  - The export's Soul Partition reading matches C2.2: `statics` empty and one `Unimplemented` cost sentence.
- **Tests.**
  - Error attribution strings match `combat.rs:3905`, `:7186` and `:7198`.
  - Production turn starts are used, and `summoning_sick` is never written.
  - The defending player always has a second legal blocker.
  - The F8 paired positives keep `ret` in the declaration.
  - V7(b) uses Fate Transfer (plan-review note 1) and asserts that the effect does not resume.
  - Every synthetic text carries a real-cards statement.
- **CR citations** in the new code (109.5, 110.2a, 122.6, 302.6, 310.5, 400.7, 508.1b/c/d, 508.2, 509.1b, 608.2b/c, 611.2a/b, 701.15a/b, 117.3d) resolve in `docs/MagicCompRules.txt` and fit their use.

## Probe

One probe, aimed at F1's claim. I added a temporary untracked `crates/engine/tests/zz_probe_io_dress_down.rs`, an auto-discovered top-level test target that is never committed, and ran it alone. `CARGO_TARGET_DIR=…/cargo-targets/h2-plan`, `CARGO_BUILD_JOBS=4`, `CARGO_INCREMENTAL=0`, command `cargo test -p phase-engine --test zz_probe_io_dress_down -- --nocapture`, `test result: ok. 1 passed`, run window 1795240837–1795240843. I deleted the file immediately; `git status` was empty afterwards.

Board: `GameScenario::new_n_player(3, 42)`, P0's precombat main. P0 has 2 Plains, 2 Islands and Grizzly Bears `c0`. P1 has Grizzly Bears `p1c`, P2 has Grizzly Bears, and P1's graveyard holds Grizzly Bears, the target. P0 casts the verbatim Immortal Obligation, then Dress Down (verbatim, `{1}{U}`, via `add_enchantment_to_hand_from_oracle`), followed by a layer refresh. P0 passes priority to `DeclareAttackers`, `c0` attacks P1, and P0 passes to P1's `DeclareBlockers`. Output:

```
ZZ after IO: goaded=true statics=["CantAttack/Some(PlayerId(0))", "BlockRestriction { filter: Not { filter: Typed(TypedFilter { type_filters: [Creature], controller: Some(You), properties: [] }) } }/Some(PlayerId(0))"]
ZZ dress down zone=Battlefield stack_empty=true wf="Priority"
ZZ after Dress Down: goaded=true statics=[]
ZZ blockers prompt for PlayerId(1): ret_valid=true p1c_valid=true
ZZ ret blocks c0 after Dress Down: Ok(())
```

## Pre-existing

- `layers.rs:13085` (`RemoveAllAbilities` runs `obj.static_definitions.clear()`) also erases every other resolution-installed restriction grafted as a layer-6 static. That includes Promise of Loyalty's caster-latched `GrantStaticAbility{CantAttack}` ("Each of those creatures can't attack you or planeswalkers you control for as long as it has a vow counter on it.") and resolving `AddStaticMode` restrictions such as Craterous Stomp's `BlockRestriction`. This is derived from the same arm and was measured only for Immortal Obligation. It is outside this phase's claim except as composed in F1.
