Pushed the port: `2fc5a9188e..f995714a3c` (first-parent), with main merged through `206116c5cd`. The description now has a "Port onto #9670" section, and its other claims are updated for the new head.

**Commits (first-parent)**
- `0b067cbdab`: merges main `91639fdf02` onto #9670's authorities.
  - Amount: one `event_amount` from `replaced_event_amount`. For a prevention it is the amount actually prevented, zero included. It is read through the dispatch-bound replacement-action scope; the stamp and the resident `post_replacement_event_amount()` tier are removed.
  - Combat: main's resumable combat pass, with this branch's ledger, the CR 104.1 stops and the CR 800.4a unstarted-leg settlement.
  - Tail: `DamageTail` for Fight, `ResumeDamageBatch` (a carrier only) and `ApplyPostReplacementDamage`; `damage_resolver_owns_tail` and `parked_batch_owns_tail` are deleted.
  - Outcomes: `LossReason` is threaded through the effect-stated outcomes.
  - `a541f28aa8` converts the combat rows to the asked order choice, and `00396db22d` records the size measurement.
- `0b9b8f40af` (+ `6f40e4dc6a`, a protocol wording fix): `OptionalEffectFrame` carries the replacement-action scope, so a replacement action's "may" keeps the replaced amount. Jakob trimmed P2 to this. Making the combat riders' own prompts live (Phase D, and an earlier rider inside a leg) is deferred to Command Zone #267, and those riders keep `damage_replacement_choice_unasked_in_combat`.
- `f9953f5943` + `7758d18832`: one damage-substitution predicate.
  - `ShieldKind::Substitution` is removed. Record `9ebdf88e` is added and `c85ad0f6` is kept.
  - The fence reads `replacement::is_damage_substitution` and states protection only. Soul-Scar Mage, Szadek and Undead Alchemist drop, as you accepted.
  - A substitution now happens before an earlier same-event prevention rider's additional effect (CR 615.5). Combat's in-leg drain keeps the earlier-first order under strict failure.
- `c3ba39e1c5`: merges main `206116c5cd`. #10433's conflicts keep both sides:
  - `handle_replacement_choice_inner` keeps the pre-entry roll view and wraps only the live-board arm in the batch's `hold_riders_for_awaited_portion`; a `RollDice` record never meets a damage batch.
  - `dispatch_pending_post_replacement_effect` keeps main's entrant parking and settlement around this branch's amount scope.
  - `parked_entrants` is threaded into both drain builders.
  - #10433 adds no second amount mechanism: the roll total reads the die-result tier, which the amount scope lets through.
- `f995714a3c`: comment only, naming the two drains that still settle a rider's choice.

**Witnesses**
- **Channel Harm's accepted "may"** (`unpreventable_damage_prevention_effects`):
  - after Lightning Bolt it deals 3, with accept, decline and a save and load at the prompt;
  - inside Shockmaw Dragon's trigger it deals 1, not 4;
  - two Channel Harms loop until one player declines;
  - under Urza's Armor plus Fateful Showdown, the rider deals 4 and the spell still discards and draws 5;
  - each row fails with the scope re-entry reverted, and the Showdown rows draw 4 if the guard spans the tail.
- **CR 615.5 order:** Test of Faith before Soul-Scar Mage gives the -1/-1 counter first, then 3 +1/+1. The same order holds when either side pauses (Hardened Scales with Doubling Season, or Vizier of Remedies). Dralnu's sacrifice and the Hurloon Battle Hymn rows are converted.
- **Szadek:** Teferi's Protection prevents ahead of its choice; the shared-shield row gives 16 life when Mending Hands is applied first.
- **Undead Alchemist:** two copies offer both and replace once.
- **Fight tail:** Prizefight's Treasure appears once after a paused fight.
- **Both suite sets pass at the head:** #9670's (Soul-Scar, Szadek, Undead Alchemist, Adamantoise, combat-hit bookkeeping, Awe Strike) and #10433's as-enters dice suites.

**Verification at `f995714a3c`, at the default test stack:**
- workspace clippy clean; every phase-engine target green; parser, server, `phase-ai --lib` and wasm32 pass;
- the authority gates and Gate A pass;
- Kathril fits 16 MiB, and `GameState` is 15,936 B.

Main has since added #10016 (`4ba131744b`, can't-have-counters statics). It conflicts in `deal_damage.rs`: 2 small hunks. As agreed, I've left that merge to you at approval time.
