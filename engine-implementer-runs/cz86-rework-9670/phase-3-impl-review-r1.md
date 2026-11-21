# Codex P3 implementation review r1 (magi-55ea6a03) — summary
0 behavior.
1. [LOW machinery] Three combat witnesses can pass with an unfinished damage batch: Szadek protection (damage_substitution_replacements.rs:4282), Szadek shared shield (:4869), two Alchemists (:4910); `combat_events` exits on any non-priority wait (:419) and `combat_answering` stops on any wait other than a replacement choice. Require the planned combat completion boundary and an empty resolution stack in all three.
2. [LOW text] coverage.rs:10410 replace "The card parses, and with no protection it works" with "The card parses. This fence records the missing order choice against protection."
Pre-existing: optional-accept Replace eviction (replacement.rs:16660); combat in-leg earlier-first (replacement.rs:1105, #267).
