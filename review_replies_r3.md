# Replies for PR #10039 review threads (round 3, head 375a58f)

Paste each under the matching review on GitHub.

## matthewevans: changes requested at 2d20628 (F1 and F2)

Thanks, both were real. Fixed at 375a58f, which also merges main d3f002d.

**F1 (an interactive child leaves the activation off the stack).** A counter-cost root now resumes only when its whole replacement subtree has retired, wherever that subtree finishes. The replacement records a typed subtree boundary on the root, and one action-boundary authority (`resume_parked_counter_cost_root_at_action_boundary`) advances an unfinished subtree and resumes a retired root. Answering the Scry in your witness now puts the activation on the stack on both the stack route and the mana route. The same authority covers the other interactive children: mana-colour, discard, sacrifice, unless payment (both players' legs) and an SBA child. Each has a row in `cost_removed_counter_amount.rs`.

**F2 (a prevented child removal vs the outer payment's own removal).** The Prevented arm now classifies by the root's subtree (`Absent` / `Unfinished` / `Retired`) instead of by whether any root exists. A prevented child removal finishes its own `CounterRemovals` queue and its trailing rider before the parent resumes; only the root's own prevented removal settles the payment. Rows cover what you asked for: a two-entry child with a trailing rider, a prevented child with an unrelated outer root parked (mill), a same-subject child, and the root's own prevented removal. The mana cursor and the child resolution frame are both checked as owners.

While doing this I also made the activation's announcement the single owner of the trigger events its payment produces, so a dies or discard trigger from a child goes on the stack once, above the activation. After syncing main I reconciled this with #9941's cost-span custodian: the push commits through `commit_activation_announcement`, and #9941's park stand-down, paid-cost commit flag and sacrifice-settle carrier write are kept. Known limits are listed in the PR body under Validation Failures.

## CodeRabbit: Prevented arm drains only when no root exists (`engine_replacement.rs`)

Fixed at 375a58f. The arm no longer checks `pending_cost_move_resume.is_none()`; it asks whether the prevented removal belongs to the parked root's own payment or to a child inside its replacement subtree. A child removal drains its own queue and rider first, so its `CounterRemovals` frame no longer strands with `in_flight` set, and an unrelated `Cast` / `ActivationMillPayment` root is left parked as on main. Covered by the prevented-child rows in `cost_removed_counter_amount.rs`.
