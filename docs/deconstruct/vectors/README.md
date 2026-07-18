# Golden vectors — format contract

Every file under `vectors/` was machine-extracted from the source repo at
the commit pinned in `../MANIFEST.md`, by running `cargo run --example
deconstruct_vectors` in the source repo. Values are never hand-written.
Regenerating at the same commit must reproduce every file byte-identically.

## Envelope

```json
{ "behavior": "<slug>", "data": {}, "epic": "DECON-NN" }
```

Key order is alphabetical (the dumper's JSON serializer's default map
ordering) and carries no meaning — JSON objects are unordered; an
implementation compares fields by name, never by position.

## Determinism rules

UTF-8, LF, 2-space indent, trailing newline. No timestamps. Arrays in
domain order. Fixed seeds (listed in the owning epic). Byte-identical
across runs.

## Consuming

An implementation passes a vector iff computing the described behavior
yields data deep-equal to the file's `data` field. Field names inside
`data` describe the domain (defined per-file in the owning epic's Domain
map); they do not prescribe your API.

## Normative vs. informative

Three vector files carry a condition or partial scope; every other file in
this pack is unconditionally normative.

- **`shuffling/seeded-shuffle.json`** — **informative**, per **SD-01**
  (indexed in `../MANIFEST.md`, spelled out in
  `DECON-03_Shuffling_And_Determinism.md`). It demonstrates the
  seeded-shuffle determinism *property* (same seed twice ⇒ identical
  permutation) and multiset preservation, but its literal permutation
  strings are not a cross-implementation contract.
- **`localization/locales.json`** — **mixed**, per **SD-05** (spelled out
  in `DECON-07_Localization.md`). The `en-US` and `de` locale entries are
  normative; the `fr`, `la`, and `tlh` locale entries are informative — a
  working target, not a pass/fail contract.
- **`formats/ckc-encoding.json`** — **conditionally normative**, per
  **SD-02** (spelled out in `DECON-04_Formats_Parsing_And_Encoding.md`).
  It is normative only for an implementation that claims poker-evaluator
  interop capability; an implementation that doesn't claim that capability
  may disregard this vector entirely.

Every other vector file in this pack is normative.

## Files
| File | Epic | Behavior |
|---|---|---|
| `card-model/canonical-order.json` | DECON-01 | Full 52-card canonical order (high-to-low precedence, suit-major) for Standard52, with index/symbol/rank/suit per position |
| `card-model/card-anatomy.json` | DECON-01 | Sample cards showing rank-facet and suit-facet fields (name, index, symbol, weight) and the composed full name/index/symbol |
| `pile-ops/draw-semantics.json` | DECON-02 | `draw_first`/`draw_last`/`draw(3)`/`draw(0)`/`draw(1000)` applied in sequence to a 52-card pile, showing all-or-nothing over-draw behavior |
| `pile-ops/extraction.json` | DECON-02 | Deduplicated rank and suit lists (descending precedence) and the 2-card combination count for a 52-card pile |
| `pile-ops/sort-variants.json` | DECON-02 | A seed-42-shuffled 52-card pile plus its default (suit-major) and rank-major sorted forms |
| `shuffling/seeded-shuffle.json` | DECON-03 | Seed-42 repeatability check plus three seeds' (42, 1337, 2026) resulting permutations for Standard52 — **informative** (SD-01) |
| `formats/ckc-encoding.json` | DECON-04 | Poker-evaluator-compatible numeric (CKC) encoding per card index across a deck |
| `formats/parse-cases.json` | DECON-04 | Round-trip parse cases: lowercase index, blank sentinel token, and an invalid/unrecognized token |
| `formats/roundtrip.json` | DECON-04 | A full deck's cards through index/symbol string forms, position by position |
| `french-family/compositions.json` | DECON-05 | Full ordered card-index list per French-family deck (French, Standard52, Short, Spades, Euchre24/32, Pinochle, Canasta, Razz) |
| `tarot-skat/compositions.json` | DECON-06 | Full Skat and Tarot compositions with per-card facet fields (index, rank, suit, symbol, position) |
| `localization/locales.json` | DECON-07 | Per-locale rank and suit name tables for en-US, de, fr, la, tlh |
| `extension-registry/registry.json` | DECON-08 | Registry enumeration of every shipped deck kind, with display name and card count |
