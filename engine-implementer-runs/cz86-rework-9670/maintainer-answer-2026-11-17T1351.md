<!-- pr-review-held -->
**Held at exact head `2fc5a9188eed7eb15c6c1694917885a2db8d0f8c`. Our read on the three choices in your [13:20Z note](https://github.com/phase-rs/phase/pull/10639#issuecomment-6949229216): all three are acceptable, each with one condition.** This read is from source on `main` `da2af2bca5` and this head, with nothing executed. The approval and the port plan still stand.

**1. Uniform fence on Soul-Scar Mage, Szadek and Undead Alchemist: accept. Narrow the claim to protection.**
- On `main`, protection prevents the damage before replacements are offered (`crates/engine/src/game/effects/deal_damage.rs:558` for objects, the player gate just below it, and the same gate in `combat_damage.rs`). CR 702.16e: "Any damage that would be dealt by sources that have the stated quality to a permanent or player with protection is prevented." CR 616.1e: "Any of the applicable replacement and/or prevention effects may be chosen." So the affected player loses the CR 616.1 choice on all three cards. For example: Pyroclasm against Silver Knight under an opposing Soul-Scar Mage, or Szadek's combat damage to a player under Teferi's Protection.
- An honest coverage drop for a real gap is fine; we won't ask you to restore it.
- Please don't claim that the second gap (one choice per source vs. one per event) reproduces uniformly. Force Bubble's ruling gives one choice across sources, but CR 700.1 ("The text of triggered abilities and replacement effects defines the event they're looking for") means that doesn't carry over automatically to source-qualified substitutions like these three.
- The fence should also read the same substitution predicate the runtime uses. The PR's `coverage.rs:11451` has its own `is_damage_substitution` today.

**2. Keep `Effect::ResumeDamageBatch` as the node: accept, as a carrier only.**
- `main`'s per-recipient stash (`DealDamage` remainders and `ApplyPostReplacementDamage`) has no slot for held riders, deferred APNAP choices, split parts, settled outcomes or the answered-portion hand-back, so a carrier is needed.
- One node isn't a second authority as long as it has no tail rule of its own. Have it participate in `main`'s damage dispatch and `DamageTail` contract (`deal_damage.rs:1390`), and delete `damage_resolver_owns_tail` / `parked_batch_owns_tail`.
- Fresh execution, resume, re-pause and the answered-portion hand-back should all go through the same batch runner.

**3. One `event_amount` with a dispatch-bound reader: accept. Capture the replacement-specific amount.**
- Test of Faith ("For each 1 damage prevented this way, put a +1/+1 counter on that creature.") inside Shockmaw Dragon's trigger ("it deals 1 damage to each creature that player controls") must give one counter. Delaying Shield ("put that many delay counters on this enchantment instead") under Farideh's Fireball must give two counters, whatever the d20 face.
- CR 615.5 refers to "the amount of damage that was prevented". So for a prevention rider, capture the amount actually prevented, including zero, not the original proposed amount.
- Removing `REPLACEMENT_AMOUNT_STAMP` and the resident `post_replacement_event_amount()` tier is right because of the dispatch-bound lifetime. Restore the outer scope after nested work, and check that the amount doesn't leak into the enclosing spell's tail.

When you push the merged head, we'll re-review the delta from `2fc5a9188e` against these conditions.

