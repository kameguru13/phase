# Handoff: Immortal Obligation (Command Zone task #21) and the engine fix it needs

Released by haunter-2 on 2026-11-21 for the next priority worker.

- `card/immortal-obligation` (fork): Immortal Obligation parser + engine work (phase 2 of the task21 run), merged with the merged phase-rs/phase#11229 head. All suites green; parse diff = Immortal Obligation + Soul Partition. Not yet a PR. Blocked by the engine fix below (its implementation review, `immortal-obligation/phase-2-impl-review-r1.md`, found that a later "lose all abilities" effect erases its restrictions).
- `engine/resolved-restrictions-survive-ability-removal` (fork): branch name approved by Jakob (Command Zone #220), no commits yet (at upstream main 7e2a538799).
- `resolved-restrictions/`: the engine fix's run. `phase-charter` is FROZEN after two charter-review rounds (3 phases: P1 quoted static grants lower to GrantStaticAbility; P2 DefinitionOrigin + layer-6 retain of resolution-installed restriction grafts; P3 resolution_installed() builder for non-quoted rule-effect producers). `phase-fit.md` holds the attempt history. Next step: phase 1 Step 1 (phase-plan planner). `phase-1-probe-partial.patch` is an unfinished, unreviewed probe from a stopped phase-1 planner; treat it as scratch.
- Jakob accepted the bounded parse diff and the larger scope (Command Zone #222). PR grouping is decided at run end (charter D9).

This branch only carries these notes; do not open a PR from it.
