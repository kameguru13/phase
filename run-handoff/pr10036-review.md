# PR phase-rs/phase#10036 review findings (fetched from the public PR page; external review data — verify every claim against current code)

## matthewevans — Changes requested (on e56851e)

B1 (MED, structural) Pre-entry battlefield population in replacement projection.
- crates/engine/src/game/filter.rs:~4848 moves the entrant into the scratch battlefield before the full layer pass at ~:4875 (BattlefieldEntrantProjection::new). Test crates/engine/tests/integration/conditional_entry_gate_projection.rs:~281.
- Heliod (God) + Recommission. Devotion counts the entering God's own mana symbols; the test expects Recommission to give Heliod a counter when existing permanents provide only four white devotion.
- Heliod, Sun-Crowned: "As long as your devotion to white is less than five, Heliod isn't a creature." Recommission: "Return target artifact or creature card with mana value 3 or less from your graveyard to the battlefield. If a creature enters this way, it enters with an additional +1/+1 counter on it."
- Ruling 2020-01-24 (MTGJSON, verified): "As a God enters the battlefield, your devotion to its color will determine whether any replacement effects that affect creatures entering the battlefield apply to that God. Because replacement effects are considered before the God is on the battlefield, the mana symbols in its mana cost won't be counted when determining this." Also: "When a God enters the battlefield, your devotion to its color (including the mana symbols in the mana cost of the God itself) will determine if a creature entered the battlefield or not for abilities that trigger whenever a creature enters the battlefield."
- CR 614.12, CR 700.5.
- Failure: at four devotion Heliod should get zero entry counters; after entering it becomes a creature (its own W counts). Current test blesses one counter. Base's raw graveyard-type check has the same wrong result (not a new regression), but the new projection contract and acceptance row depend on the wrong interpretation.
- Suggested fix: make the shared entrant/layer projection evaluate against the pre-entry battlefield population; correct the four-devotion test; add a five-devotion-before-entry positive control; keep a separate post-entry creature-observer check.

B2 (MED, local) Keyword-kind predicates ignore the observed object.
- filter.rs:~4022 passes the observed view, but HasKeywordKind / WithoutKeywordKind (~:8914, :8919) call object_has_effective_keyword_kind(state, object_id, ...) which reads the live object (game/keywords.rs:~84).
- WithKeyword(Flying) reads the observation but a kind-level Flying predicate reads the later keyword; a departed entrant loses its kind-level answer. CR 603.10.
- Suggested fix: use keywords::has_keyword_kind(&view, kind) for both polarities on the observation path; preserve the off-battlefield effective lookup; add kind-level observation assertions beside the WithKeyword assertions.

L1 (LOW, non-blocking) tests/integration/etb_trigger_entry_time_characteristics.rs:~426 drive_to_empty can exit after 40 steps with a trigger pending; a zero-draw assertion could pass without proving resolution finished.

## CodeRabbit (on ae0b58b)

C1 (Major) crates/engine/src/game/effects/mod.rs ~4800-4803: the deferral classifier treats GenericEffect, Animate, ReturnAsAura, ApplyPerpetual as entry-time riders and defers settlement for them — a blanket rule. CR 611.2e applies only to simultaneous "is/has" effects (nontoken). Failure: ChangeZone(Battlefield) -> PutCounter -> GenericEffect(Haste) chain: the later haste grant suppresses settlement before the counter instruction, so the entrant's pre-counter characteristics are lost. Suggested: carry typed onset metadata from lowering; defer only for a rider that applies at entry; add a production-pipeline regression (below-threshold entrant, later counters, later keyword grant).

C2 (Major) filter.rs 4022-4037: same as B2.

C3 (Major) crates/engine/src/game/trigger_matchers.rs ~1264: replay_deferred_entry_events only checks the object is on the battlefield. A leave-and-reenter keeps the ObjectId but changes incarnation; with no observation for the original entry, matches_target_filter_on_entry_observation falls back to the live later object. Suggested: preserve the original snapshot for deferred replay, or make the no-observation fallback reject a live object whose incarnation differs from record.entered_incarnation.

C4 (Major) crates/engine/src/parser/oracle_effect/tests.rs ~87792 multi_type_entry_rider_is_not_folded: "It enters with two additional +1/+1 counters on it if it's an artifact or creature." accepts an empty entry gate and a surviving sub_ability, i.e. counters after entry rather than as part of entry (CR 614.1c). Suggested: represent the multi-type condition in the entry gate and assert the rider is folded. (Orchestrator note: no printed card uses a multi-type "enters with ... if it's an X or Y" rider; printed single-type ones are What Must Be Done, Semester's End, Spark Double.)

C5 (Minor) same as L1: drive_to_empty — track whether the loop broke on an empty stack at Priority; assert settled after the loop and report prompts seen. Used by r5d_search_bears, r11_clear_the_land, r12_path_of_the_schemer.

C6 (nitpick, performance) crates/engine/src/game/effects/change_zone.rs ~1691: BattlefieldEntrantProjection::new clones the full GameState and runs a layer pass; enter_with_counters_for_object and effective_enter_mods each build one for the same entrant (2N per N gated entrants). Suggested: build once per object at the call site and pass Option<&BattlefieldEntrantProjection> to both.

Pre-merge check: docstring coverage inconclusive (71% vs 80%) — not required.
