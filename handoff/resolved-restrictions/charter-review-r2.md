# Charter review r2: removal-restrictions

Mode: `review-engine-plan` charter mode, plus `engine-implementer/chartered.md`.
Artifact: `phase-charter` (r2). Inputs read: `task.md`, `phase-fit.md`, `plan-r1.md` (as the source draft), `class.json`, `class-drop.json`, `phase-charter.r1`, `charter-review-r1.md`, and `addenda/` (three empty files).
Worktree: `X:/Users/jakob/Documents/dev/phase-wt/resolved-restrictions` at HEAD `7e2a538799e672af23caca367ba938bc24194d66`, clean. The review was read-only: no builds and no edits. Evidence comes from grep, from jq over `data/card-data.json` and `data/mtgjson/AtomicCards.json`, and from a word diff of `phase-charter.r1` against `phase-charter`.

**Verdict: clean round.** There are 0 decision findings and 3 corrections. Once the orchestrator applies the corrections, the charter freezes.

---

## Round-1 findings: status

- **F1 (phase 1 "no behavior change" goal): resolved.**
  - The phase 1 Goal now matches r1's proposed text verbatim.
  - D4 defines the **base collapse predicate**. I checked it against `grammar.rs:1811–1817` at HEAD: `affected == Some(SelfRef) && condition.is_none() && modifications.is_empty()`, keeping only `mode`. The perpetual `TryFrom` uses that predicate exactly.
  - D9(b) is gone, because D9 was replaced as a whole under C3.
  - Two additions go beyond r1's text and are sound:
    - a §6 re-charter trigger for a reading-change card that needs a consumer unit of its own;
    - a rejected alternative in D4 ("keeping the collapse for the predicate-meeting subset on the layer path"). Keeping the collapse would leave the Incite Hysteria / Panic payloads identical, which is the very shape D3 keys on.
  - The perpetual-parity claim holds by construction. I checked that `classify_quoted_inner` (`keyword_grant.rs:2074`) gets its `GrantStaticAbility` only from the collapse's else-branch (`keyword_grant.rs:2157` → `grammar.rs:1819`). So at base, no predicate-meeting `GrantStaticAbility` reaches the `TryFrom` (`ability.rs:22719`), and mapping one to `AddStaticMode { mode }` accepts nothing new.
- **F2 (D6 replacement half): resolved.**
  - D6 is reduced to statics.
  - The replacement half moved to §7 with r1's one-line reason and its Maze of Ith witness.
  - §6 now shows phase 2 as one unit, with the statics conjunct tied to D2 through the Grizzly Bears / Muraganda Petroglyphs witness.
  - The phase 2 Goal says "statics".
- **C1 (Venser −1): applied verbatim** (§1).
- **C2 (AC4 accepted, #222): applied verbatim** (§0 correction 2, §8.1). It is consistent with `phase-fit.md`: "Jakob #222 … AC4 superseded by his answer".
- **C3 (D9 / §8.2 PR grouping): applied verbatim.**
- **C4 (delete "establishes" blocks and acceptance-row detail): applied.**
  - The phase 1 and phase 2 "establishes" blocks are deleted.
  - The phase 2 "Examples:" are deleted.
  - The phase 3 census line is replaced with r1's text.
  - The AC2 attribution was also trimmed of row detail, which is consistent with C4.

## Review of the revision as new work (no finding)

- **Premise.**
  - Verbatim Oracle text checks out in AtomicCards for the four added cards:
    - Elrond of the White Council: "…they gain "This creature can't attack its owner."…"
    - Case of the Uneaten Feast: Solved line.
    - Kethis, the Hidden Hand: second line.
    - Mine Security: "It perpetually gains "You may pay {0} rather than pay this spell's mana cost.""
  - Step 0.5 record: the saved `class.json` gives `jq -cS . | sha256sum` = `bc328027…f208` with jq-1.8.2, matching §1. r1 re-ran the recorded command and reproduced that hash. The file predates that review and is unchanged.
- **Provenance rule and CR grep.** The two added CR numbers exist in `docs/MagicCompRules.txt` and are paraphrased accurately:
  - 113.6 (l.773) and 113.6b (l.777);
  - 508.1c (l.2273).
  - 611.2c's "modifies the rules of the game" is still its second-to-last sentence (l.2915).
  - The rule is unchanged from r1, which verified it as CR-correct: D3 is transient-only and fail-closed, quoted grants are abilities (CR 113.10), and layer 6 acts in timestamp order (CR 613.1f, CR 613.7).
- **One unit per phase, and green-tree seams.**
  - Phase 1's reading changes come from one representation change (one unit). If one needs a consumer unit of its own, the new §6 trigger routes it to a re-charter.
  - P1 → P2: each reading-change row fails at base and passes after; parity rows pass on both sides. Coverage stays honest, and no strict-failure tags are needed.
  - P2 → P3: vehicles stay `Characteristic`, and the Promise row is `DEFERRED(phase 3)`.
- **Discriminating tests or a justified DEFERRED.**
  - Phase 1 has a parser row that fails at base, plus a reading-change row per changed card. The hostile witnesses' discriminating role is `DEFERRED(phase 2)`.
  - Phase 2 has discriminating runtime rows. Promise is `DEFERRED(phase 3)`.
  - Phase 3 has its Promise + Humility row.
  - Every deferral names its landing phase.
- **Architecture only.**
  - No new file inventory: the phase 1 scope still uses literal paths, with no globs.
  - The new code-state sentences are labelled ("charter review measured"), except the one in C-r2-3 below.
- **Deferrals.** The new §7 item carries a reason and has a triage owner. The others are unchanged.

---

## Corrections (orchestrator applies; old → new)

### C-r2-1. D4's new "non-layer" qualifier narrows the reader census against §5 and the phase 1 scope rule

**Where:** §2 D4, fifth bullet. This sentence was added in r2; r1 did not supply it.

**Evidence.**
- The qualifier excludes every `layers.rs` reader from the parity-or-reason obligation.
- `layers.rs` holds `AddStaticMode { mode }` pattern matches other than the graft arms:
  - the TCE gather skip at ~9982 (`MayLookAtFaceDown | ReduceAbilityCost | CastFromHandFree | Goaded | CantBeTargeted | CantBeCountered`, plus `transient_relative_blocker_filter`);
  - `continuous_gather_skips` at ~8187.
- §5 still lists `layers.rs` for "phase 1 parity readers if the census finds any", and phase 1's scope rule admits readers under `crates/engine/src/game/`.
- No decision rests on the qualifier, and no printed card in r1's 18-card census uses a skip-listed mode. So this is wording.

**Old:**
> - Every non-layer reader that pattern-matches `AddStaticMode{mode}` for a mode in phase 1's regenerated diff reads a `GrantStaticAbility` that meets the base collapse predicate the same way, or records a one-line reason.

**New:**
> - Every reader other than the layer-6 graft arms themselves, in `layers.rs` or elsewhere, that pattern-matches `AddStaticMode{mode}` for a mode in phase 1's regenerated diff reads a `GrantStaticAbility` that meets the base collapse predicate the same way, or records a one-line reason.

### C-r2-2. "Discarded a field" does not imply "changes reading", but the seam requires every named candidate to fail at base

**Where:** §2 D4, fourth bullet; §3 Phase 1 Seam, last sub-bullet.

**Evidence.**
- The Goal names candidates and says "the phase plan measures which cards".
- A candidate whose discarded field no consumer reads keeps its base reading. Case of the Uneaten Feast's graveyard permission may be one: its `active_zones` may be unused if the base graft already functions there.
- For such a card, "a runtime row that fails at base" cannot be written. As worded, that would cost a phase 1 plan round.

**Old (D4):**
> A quoted card whose definition carries a field the base collapse discarded therefore changes reading in phase 1, and that reading follows its verbatim Oracle text (Phase 1 Goal).

**New (D4):**
> A quoted card whose definition carries a field the base collapse discarded can therefore change reading in phase 1, and its reading follows its verbatim Oracle text (Phase 1 Goal).

**Old (Phase 1 Seam):**
> - each reading-change card the Goal names has a runtime row that fails at base and passes after.

**New (Phase 1 Seam):**
> - each card the phase plan measures as changing reading has a runtime row that fails at base and passes after; a discarded-field candidate whose reading is already right at base gets a parity row.

### C-r2-3. D4's Mine Security parenthetical is an unlabelled code-state assertion about a path the change does not reach

**Where:** §2 D4, third bullet.

**Evidence.**
- `oracle_effect/mod.rs:14017` dispatches `try_parse_perpetual_grant_self_alt_cost` before `try_parse_perpetual_grant_ability` (:14045).
- The self-alt-cost route (:15464–15476) builds `AddStaticMode{CastWithAlternativeCost}` directly. It does not go through `classify_quoted_inner` or the collapse.
- So Mine Security does not exercise the predicate mapping it is cited for.
- Parity follows by construction (see the F1 status above), so no decision rests on the parenthetical. The repair is to delete it.

**Old:**
> The perpetual path is therefore unchanged (Mine Security's perpetual quoted grant lowers as it did at base), and it stays fail-closed.

**New:**
> The perpetual path is therefore unchanged, and it stays fail-closed.

---

## Pre-existing

None new. r1's list stands and is not repeated.

## Residual notes for the phase-plan reviewers (not charter findings)

- **Phase 1, TCE skip list.** A quoted grant of a skip-listed mode would, after phase 1, be grafted (so it is removable, CR 113.10) instead of read off the TCE.
  - r1 measured none among the 18 resolving quoted collapses.
  - The phase 1 census should confirm that on its regenerated list. This is the reader C-r2-1 keeps in scope.
- **Phase 1, Elrond.** Non-layer readers such as the AI, reading the predicate-meeting `GrantStaticAbility` "the same way" as a bare `CantAttack`, stay conservative: they still treat the creature as unable to attack. Only the rules path gains `attack_defended: Owner`. That is acceptable parity for those readers, but the phase plan should say so.
