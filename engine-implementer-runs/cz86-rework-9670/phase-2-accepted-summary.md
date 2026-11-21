# P2 accepted summary (trimmed by the user, Command Zone #162)

P2 = unit U3 only, candidate 6f40e4dc6a (0b9b8f40af + protocol reason fix), on P1's 00396db22d.
- `OptionalEffectFrame.replacement_action: Option<ReplacementActionScope>` (serialized, protocol record 2ffba56b), captured by `OptionalEffectFrame::live_offer` at the three offer sites and re-entered at R1 (`handle_optional_effect_choice_inner`), R2 (opponent/any-player accept), R3 (all-declined completion), R4 (parked poll) and the re-offer copy; each guard ends before the enclosing tail.
- Channel Harm's accepted "may" now deals the prevented amount outside combat; inside a trigger it reads the prevented amount, not the trigger's.
- Dropped (follow-up Command Zone #267, local branch backup/cz86-p2-full): live combat rider prompts, Phase D park, guard carry across a park, auto-settle removal, answer convergence. Combat rider pauses are still settled inside the batch under the `damage_replacement_choice_unasked_in_combat` coverage marker, exactly as P1 left them.
