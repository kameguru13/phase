jq --arg re "$1" --arg kw "$2" \
   --argjson add "$3" --argjson drop "$4" '
  def hits($mask): [.data | to_entries[]
    | select(any(.value[]; . as $f | ($f.text // "") | gsub("\\([^)]*\\)"; "")
        | if $mask then split($f.faceName // $f.name) | join("~") else . end
        | test($re; "i")))
    | .key];
  .meta.version as $version
  | .data as $data
  | hits(false) as $unmasked
  | hits(true) as $masked
  | ($unmasked - $masked) as $masked_only
  | (($masked + ($add - ($add - $masked_only))) | unique) - $drop
  | . as $cards
  | {version: $version, regex: $re, keyword: $kw, add: $add, drop: $drop,
     count: length, cards: .,
     masked_only: $masked_only,
     rejected_add: ($add - $masked_only),
     rejected_drop: ($drop - $masked - $add),
     keyword_missing: (if $kw == "" then [] else
       [$data | to_entries[] | select(any(.value[]; (.keywords // []) | index($kw))) | .key] - $cards end),
     rulings: ([$cards[] as $card | $data[$card][].rulings[]? | {text, card: $card}]
               | group_by(.text)
               | map({text: .[0].text, cards: (map(.card) | unique)})
               | sort_by(-(.cards | length)))}' /mnt/data/dev/phase/data/mtgjson/AtomicCards.json
