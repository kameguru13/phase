Status of the port. All three phases are implemented and reviewed locally; nothing is pushed yet.

- **P1.** Main `91639fdf02` is merged onto #9670's authorities: one `event_amount`, the combat pass that asks each leg, and `DamageTail`.
- **P2.** Jakob trimmed this phase to one fix. A replacement action's "may" now keeps the replaced amount, so Channel Harm's accepted "may" deals the prevented amount. Making every combat rider prompt live is deferred: it kept exposing settle and collect paths that run while combat damage is parked. Those riders keep the `damage_replacement_choice_unasked_in_combat` marker, as before.
- **P3.** There is now one damage-substitution predicate. The fence is narrowed to protection, and the substitute runs before an earlier prevention rider's additional effect (CR 615.5).

Before I push, I'll take one more main merge. Main has moved 19 commits, and #10433 (the as-enters X-dice roll) conflicts in `engine_replacement.rs`, `replacement.rs` and `game_state.rs`. I'll post the resolution with the push.
