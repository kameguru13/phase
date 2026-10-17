# Follow-up issues found while fixing enter-trigger timing

These are bugs on upstream `main` (measured at b1be411) that the enter-trigger timing fix does not cover. Each one is a separate issue, written for the `[Card Bug]` form (`.github/ISSUE_TEMPLATE/card_bug.yml`). Game mode for all: any (engine-level). Build: phase-rs/phase main at b1be411.

When filing, mention the enter-trigger timing PR as "found while working on".

---

## 1. [Card Bug] Miraculous Recovery on Clone drops the +1/+1 counter

**Cards:** Miraculous Recovery, Clone, Hill Giant

**Actual:** Return Clone with Miraculous Recovery ("Return target creature card from your graveyard to the battlefield. Put a +1/+1 counter on it.") and have it copy Hill Giant. Clone ends a 3/3 Hill Giant with no counters. The prompts are the optional-copy choice, then the copy target. With a return spell whose second sentence is "You gain 2 life." instead, the life gain does happen, so only the instruction that refers to the returned card is lost.

**Expected:** CR 707.5 + CR 608.2c. Clone enters as a copy of Hill Giant, then gets a +1/+1 counter and is a 4/4.

**Steps:** Clone in graveyard, Hill Giant on the battlefield. Cast Miraculous Recovery on Clone, choose to copy, choose Hill Giant.

**Likely area:** the copy-entry replacement pause and resume (`engine_replacement.rs` / `zone_pipeline.rs`).

---

## 2. [Card Bug] Helica Glider's entry choice triggers enter abilities twice

**Cards:** Helica Glider, Waterkin Shaman (or Dragon Tempest), any "return target creature card from your graveyard to the battlefield" spell

**Actual:** Return Helica Glider ("This creature enters with your choice of a flying counter or a first strike counter on it.") and pick flying. Waterkin Shaman ("Whenever a creature you control with flying enters, this creature gets +1/+1 until end of turn.") ends 4/3, and Dragon Tempest's first ability triggers twice. One trigger is collected right after the counter choice, the other after the spell finishes.

**Expected:** CR 603.2 + CR 603.6a. One entry event, so each enter ability triggers once (Shaman 3/2).

**Likely area:** `replay_deferred_entry_events` and the post-action trigger scan, which doesn't exclude entries already replayed (`engine_replacement.rs` / `engine_priority.rs`).

---

## 3. [Card Bug] Honor the Past returns the card as a creature

**Cards:** Honor the Past, any creature card with mana value 6 or less

**Actual:** "Return target creature card with mana value 6 or less from your graveyard to the battlefield. It's an artifact. (It's no longer a creature.)" The returned card stays a creature and is not an artifact. The rider parses as `Animate{types: [Artifact], remove_types: [Creature], target: None}` and does nothing at runtime.

**Expected:** CR 611.2e. The card enters as a noncreature artifact.

**Likely area:** `Animate` resolution when `target` is `None` after a forwarded zone move.

---

## 4. [Card Bug] Enter abilities see Bronzehide Lion and Enduring Tenacity as creatures when they return

**Cards:** Bronzehide Lion, Enduring Tenacity, Elemental Bond, Grim Guardian

**Actual:** When Bronzehide Lion ("…return it to the battlefield. It's an Aura enchantment…") or Enduring Tenacity ("…return it to the battlefield under its owner's control. It's an enchantment. (It's not a creature.)") dies and returns, the type change does apply in the end. But Elemental Bond triggers and draws a card, and Grim Guardian ("Whenever this creature or another enchantment you control enters, each opponent loses 1 life.") does not trigger.

**Expected:** CR 611.2e. The returned card is an enchantment as it enters. Elemental Bond does not trigger; Grim Guardian does.

**Notes:** A return followed by "Put two +1/+1 counters on it." is read correctly, so this is specific to how the "It's an …" effect (`return_as_aura::install_aura_continuous_effect`, `Animate`) is applied relative to the entry, not a general timing problem. The fix is to apply the rider together with the entry (CR 611.2e).

---

## 5. [Card Bug] Clear the Land exiles one card instead of the rest

**Cards:** Clear the Land

**Actual:** "Each player reveals the top five cards of their library, puts all land cards revealed this way onto the battlefield tapped, and exiles the rest." After the lands enter, the game asks for one choice of exactly one card among all ten revealed cards (the lands now on the battlefield included). One card is exiled; the other nonland cards stay in the libraries.

**Expected:** Every revealed nonland card is exiled, with no choice.

**Notes:** Each player's leg parses as `ChangeZone{Exile, target: TrackedSet}`.

---

## 6. [Card Bug] Oviya, Automech Artisan's counters are treated as "enters with"

**Cards:** Oviya, Automech Artisan, Memnite, Elemental Bond

**Actual:** "{G}, {T}: You may put a creature or Vehicle card from your hand onto the battlefield. If you put an artifact onto the battlefield this way, put two +1/+1 counters on it." The counters are folded into the zone move as `conditional_enter_with_counters`, so Memnite enters as a 3/3 and Elemental Bond draws.

**Expected:** CR 608.2c. The counters come from a later instruction, not an "enters with" replacement (CR 614.1c). Memnite enters as a 1/1, so Bond does not trigger.

**Notes:** Same class as the token "create, then put counters on it" lift in the Zimone PR (#10004). The fold lives in `fold_enters_this_way_counter_rider`. Could be filed as its own issue or picked up alongside the Zimone change.

---

## 7. [Card Bug] Moritte of the Frost drops "snow" and its +1/+1 counters

**Cards:** Moritte of the Frost

**Actual:** "…except it's legendary and snow in addition to its other types and, if it's a creature, it enters with two additional +1/+1 counters on it and has changeling." The copy's modifications are only `AddSupertype Legendary` and `AddKeyword Changeling`. Snow and the conditional counters are dropped.

**Expected:** CR 707.9b. The copy is legendary and snow, and if it's a creature it enters with two more +1/+1 counters and has changeling.

---

## 8. [Engine] Evolve and intervening-if checks still see counters added after entry

**Cards:** Match the Odds, any creature with evolve

**Actual:** With an evolve creature on the battlefield, Match the Odds' token (1/1, then +1/+1 counters added by the next instruction) still evolves the creature when it ends up bigger, because the evolve comparison (CR 702.100a) and other intervening-if checks (CR 603.4) read the token after the whole spell resolves.

**Expected:** The comparison uses the token as it entered (a 1/1).

**Notes:** The enter-trigger timing PR fixes which triggers fire, but deliberately leaves the intervening-if recheck at trigger resolution unchanged. Fixing this needs a typed way for the fire-time check to read the stored entry observation.

---

## 9. [Card Bug] Call a Surprise Witness's flying counter is seen as if the creature entered with it

**Cards:** Call a Surprise Witness, Dragon Tempest (or Waterkin Shaman)

**Actual:** "Return target creature card with mana value 3 or less from your graveyard to the battlefield. Put a flying counter on it. It's a Spirit in addition to its other types." A non-flying creature returned this way triggers Dragon Tempest's "Whenever a creature you control with flying enters" ability. Enter observers read it after the flying counter is placed, because the "It's a Spirit" rider holds back the entry reading until the end of the spell.

**Expected:** CR 611.2e + CR 608.2c. The "It's a Spirit" effect applies as the creature enters, but the flying counter comes from a later instruction. Enter observers should see a non-flying Spirit, so Dragon Tempest's first ability doesn't trigger.

**Notes:** Same class as #4 (Bronzehide Lion / Enduring Tenacity): an "is/has" rider needs to be applied together with the entry rather than after later instructions. Found while addressing review on the enter-trigger timing PR (#10036).

---

## 10. [Engine] A plain "It enters with … counters on it if it's a [type]." rider is unsupported

**Cards:** none printed in this exact non-modal form (the printed What Must Be Done is modal and works).

**Actual:** A non-modal line such as "Return target creature card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature." parses to an `Unimplemented "enters_with_gate"` gap, for a single type or several.

**Expected:** CR 614.1c. The rider is a conditional entry replacement, like What Must Be Done's.

**Notes:** Low priority; no printed card is affected today.
