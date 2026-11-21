# P1 accepted summary (for later phases)

P1 = merge of upstream 91639fdf02 onto main's #9670 authorities. Candidate 00396db22d (merge 0b067cbdab, tests a541f28aa8, size note 00396db22d).
- Seam 1 done: one `PostReplacementDrain::event_amount` installed through `ReplacedEventContext` from `replacement::replaced_event_amount` (prevention: amount actually prevented, zero included); reader = dispatch-bound replacement-action scope (`replacement_action_amount`), carried on `PendingContinuation::replacement_action`. Stamp thread-local and `post_replacement_event_amount()` removed. `damage_shape_prevents` is the shared shape check.
- Seam 4 done: main's `DamageTail` / `resolve_instruction` table is the single tail authority, including Fight, ResumeDamageBatch (carrier only) and ApplyPostReplacementDamage; branch tail inference removed.
- Seam 3 legs: main's resumable combat pass asks each leg's CR 616.1 order (APNAP), carrying the branch's DamageBatchLedger, CR 104.1 game-end stops and the CR 800.4a unstarted-leg settlement. Still in-batch-settled (deferred to P2): Phase D aggregate prevention riders and an in-leg earlier rider's prompt; `damage_replacement_choice_unasked_in_combat` fence stays for them. Guards read once per pass entry (carrying them across a park is P2).
- Mechanical: LossReason threaded (EffectStatedLoss / OpponentWon, unioned), one LazyBoxedVec, GameState 15,856 B under 16,384.
- Deviation accepted: a one-shot prevention that prevents nothing does not queue its rider (Awe Strike, CR 609.7b).
- Follow-ups (not phases): Outfitted Jouster double block (ignored red witness), The Last Agni Kai excess after a resumed fight.
