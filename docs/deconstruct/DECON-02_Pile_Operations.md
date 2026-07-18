# DECON-02: Pile Operations

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice defines the **pile** — an ordered multiset of cards drawn from
one deck vocabulary — and the operations a consumer performs on it:
construction, drawing, sorting, and extraction/filtering. It builds
directly on the card model in DECON-01 (a pile's cards each carry a rank
facet and a suit facet with precedence weight) and is itself the
foundation DECON-03 (shuffling), DECON-05, and DECON-06 (deck families)
build on.

Out of scope for this slice: how a pile is permuted at random (DECON-03),
how a pile is rendered to or parsed from a string in full detail (DECON-04
— though the round-trip half of self-validation is described here and
specified precisely there), the composition of any specific deck family
(DECON-05, DECON-06), and how a consumer defines an entirely new deck
vocabulary that a pile could then be built from (DECON-08).

## Status
| Component | Status |
|---|---|
| Pile construction | Planned |
| Draw semantics | Planned |
| Sort variants | Planned |
| Extraction / filtering | Planned |
| Self-validation | Planned |

## Goals

- Define a **pile** as an ordered multiset of cards, all drawn from the
  same deck vocabulary, that a consumer owns and may freely mutate.
- Define **draw** semantics that are all-or-nothing and never crash.
- Define **sort** variants: suit-major (the default) and rank-major.
- Define **extraction**: deduplicated rank/suit lists, filtering by facet
  type, combinations, and grouping cards by facet.
- Define a **self-validation** concept a rebuild can use to prove its pile
  machinery is internally consistent.

## Scope

- A pile holds zero or more cards, all from one deck vocabulary. Duplicate
  cards (same rank facet + suit facet pair) are permitted where the
  vocabulary itself contains duplicates.
- **Construction** forms: the full canonical deck for a vocabulary (see
  DECON-01's canonical order); *n* concatenated copies of a canonical deck;
  an explicit list of cards; a pile parsed from a string of card tokens
  (full grammar in DECON-04).
- **Draw** operations: draw the first card, draw the last card, draw *n*
  cards from the front, draw one card at random. Draw is **all-or-nothing**:
  asking to draw more cards than the pile holds yields nothing at all and
  leaves the pile completely unchanged — never a partial draw, never a
  crash. `draw(0)` is a **no-op**: empty result, pile unchanged (not
  treated as an over-draw error case).
- **Sort** variants: **default (suit-major)** — highest suit precedence
  first, then within a suit highest rank precedence first (reproduces
  DECON-01's canonical order over a full deck); **rank-major** — highest
  rank precedence first across all suits, with cards of equal rank
  precedence keeping their prior relative order (stable with respect to
  whatever order the pile was already in).
- **Extraction / filtering:** the deduplicated list of rank facets present
  in a pile, and separately of suit facets, each in descending precedence
  order; filtering cards by facet type (e.g. isolating a "special"/trump
  suit facet, or isolating jokers — see DECON-06 for a worked Tarot
  example); all length-*k* combinations of a pile's cards, deduplicated;
  grouping a pile's cards into buckets keyed by rank facet, or separately
  by suit facet.
- **Self-validation:** a rebuild's pile machinery is self-consistent if,
  for any full deck: (a) shuffling the deck and then sorting it with the
  default sort reproduces the original canonical order exactly, and (b)
  converting the deck to its string form and parsing that string back
  reproduces the original pile exactly.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Pile construction | Full canonical deck as a pile; ordered per DECON-01 | `vectors/card-model/canonical-order.json` |
| Draw (first/last/n/random) | All-or-nothing; over-draw and `draw(0)` never mutate the pile or crash | `vectors/pile-ops/draw-semantics.json` |
| Sort (default) | Suit-major, descending precedence throughout | `vectors/pile-ops/sort-variants.json` |
| Sort (by rank) | Rank-major, descending rank precedence, stable across suits | `vectors/pile-ops/sort-variants.json` |
| Extraction (ranks/suits) | Deduplicated, descending precedence order | `vectors/pile-ops/extraction.json` |
| Extraction (combinations) | All length-*k* combinations, deduplicated | `vectors/pile-ops/extraction.json` |
| Self-validation | shuffle→sort reproduces canonical order; string round trip | `vectors/pile-ops/sort-variants.json`, `vectors/formats/roundtrip.json` |

## Design

A pile is owned by whoever holds it: drawing, sorting, or filtering a pile
mutates (or produces a new value from) that instance only — it can never
reach back into the deck vocabulary the cards came from and never affects
any other pile. This is the same value-semantics guarantee DECON-01
establishes for individual cards, extended to collections of them.

Draw's all-or-nothing rule and its "never a crash" guarantee are the
essential contract — a rebuild is free to signal "nothing was drawn" by
whatever mechanism fits its language, as long as the caller can
distinguish "drew nothing" from "drew something" without the pile having
been touched. Rank-major sort's stability requirement is likewise
essential (predictable behavior when several cards share a rank
precedence); *how* that stability is achieved internally is not.

Self-validation is a design concept a rebuild should carry forward as its
own smoke test for every deck vocabulary it supports, not a single
operation with its own vector — it composes DECON-01's canonical order,
this epic's default sort and shuffle-then-sort round trip, and DECON-04's
string round trip.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice — see DECON-08 for vocabulary-authoring machinery. |
| Administrative | — | — | N/A for this slice — see DECON-08 for deck-kind enumeration. |
| User/client | Draw, sort, filter, extract from, and mutate their own pile instance freely | Alter the deck vocabulary (facet definitions, weights, or composition) that the pile's cards were drawn from | A user owns their pile outright; every pile instance is an independent copy of vocabulary data, and mutating one pile never affects the vocabulary or any other pile. |
| Observer/operator | — | — | N/A for this slice — see DECON-03 for the one observability note this pack carries (routine shuffles are silent). |

## Work Items
### Phase 0 — Pile construction
- [ ] **0a.** Construct the full canonical deck for a vocabulary as a
  pile, in DECON-01's canonical order. Proven by
  `vectors/card-model/canonical-order.json`.
- [ ] **0b.** Construct *n* concatenated copies of a canonical deck, and a
  pile from an explicit list of cards.

### Phase 1 — Draw semantics
- [ ] **1a.** Draw the first card; draw the last card; draw *n* cards from
  the front, all-or-nothing on insufficient cards (pile left unchanged on
  failure); `draw(0)` is a no-op. Proven by
  `vectors/pile-ops/draw-semantics.json`.
- [ ] **1b.** Draw one card at random from the pile (uniform over
  remaining cards; forward-references DECON-03 for randomness sourcing).

### Phase 2 — Sort variants
- [ ] **2a.** Default sort: suit-major, descending precedence. Proven by
  `vectors/pile-ops/sort-variants.json`.
- [ ] **2b.** Rank-major sort: descending rank precedence, stable across
  ties. Proven by `vectors/pile-ops/sort-variants.json`.

### Phase 3 — Extraction and filtering
- [ ] **3a.** Deduplicated rank list and suit list, descending precedence
  order; filter cards by facet type/kind. Proven by
  `vectors/pile-ops/extraction.json`.
- [ ] **3b.** All length-*k* combinations, deduplicated. Proven by
  `vectors/pile-ops/extraction.json` (`combos_2_count`).
- [ ] **3c.** Group cards into buckets keyed by rank facet, and separately
  by suit facet.

### Phase 4 — Self-validation
- [ ] **4a.** Shuffle a full deck, sort it with the default sort, and
  confirm the result equals the original canonical order. Proven by
  `vectors/pile-ops/sort-variants.json` combined with
  `vectors/card-model/canonical-order.json`.
- [ ] **4b.** Convert a full deck to its string form and parse it back;
  confirm the result equals the original pile. Proven by
  `vectors/formats/roundtrip.json` (full grammar in DECON-04).

## Test Plan

- **Given** a 52-card pile, **when** `draw_first` then `draw_last` then
  `draw(3)` then `draw(0)` then `draw(1000)` are applied in sequence,
  **then** the results and pile sizes after each step match
  `vectors/pile-ops/draw-semantics.json` — including the `draw(0)` no-op
  leaving the pile unchanged, and the final over-draw yielding nothing and
  leaving the pile at its prior size.
- **Given** a pile shuffled with a fixed seed, **when** the default sort
  and the rank-major sort are each applied, **then** the resulting index
  strings match `sorted_default_index` and `sorted_by_rank_index` in
  `vectors/pile-ops/sort-variants.json`.
- **Given** a full 52-card pile, **when** ranks and suits are extracted
  and 2-card combinations are counted, **then** the results match
  `vectors/pile-ops/extraction.json`.
- **Given** a full deck, **when** shuffled then sorted with the default
  sort, **then** the result equals the canonical order in
  `vectors/card-model/canonical-order.json`.

## Not specified (implementer's choice)

- Internal storage structure of a pile (array, linked list, growable
  vector, …), and the exact representation of "nothing was drawn" (absent
  value, empty collection, sentinel, flag) — only the observable contract
  (unchanged pile, no partial result, no crash) is normative.
- The randomness source backing random draw — see DECON-03 for the
  broader randomness contract; this epic only requires that a random draw
  removes and returns exactly one card.
- Whether combinations/groupings are computed eagerly or lazily, and the
  exact enumeration of "facet type" categories usable for filtering
  beyond the ones exercised by shipped deck families (rank, suit, joker,
  special/trump, blank) — a rebuild may add its own.
- Whether self-validation is exposed as a callable operation, a test
  helper, or left entirely to the rebuild's own test suite — it is a
  design concept in this pack, not a required public operation.
- **Named predicate conveniences over the generic facet-type filter** —
  the original additionally exposes a family of small, named queries
  (e.g. "all cards of a given rank," "all cards sharing one rank across
  the pile," "all cards sharing one suit," "does this pile's cards form a
  run of consecutive rank precedence," "all cards at or above a given
  rank," and, on a combination set, "keep only combinations that are such
  a run" or "keep only same-suited/only mixed-suit combinations"). None of
  these is a distinct required primitive: each is fully derivable from the
  generic facet-type/predicate filter and the combinations operation this
  epic already specifies. A rebuild may offer them as named conveniences
  or leave every one of them to a caller composing the general filter —
  either satisfies this epic.
- **Whether combination grouping is bucketed by rank and ordered by group
  size**, or left as the flat deduplicated combination list this epic
  specifies — a rebuild may offer either as a convenience over the same
  underlying combination set.

## Spec decisions

None specific to this epic.

## Verification

Any implementation must reproduce every file under `vectors/pile-ops/`
(cross-referencing `vectors/card-model/` and `vectors/formats/` for the
self-validation checks):
1. Draw sequence results and pile sizes match `draw-semantics.json` at
   every step, including the all-or-nothing over-draw and the `draw(0)`
   no-op.
2. Default and rank-major sorted index strings match `sort-variants.json`.
3. Deduplicated rank/suit lists and the 2-combination count match
   `extraction.json`.
4. Shuffle-then-default-sort of a full deck reproduces
   `card-model/canonical-order.json`; a full deck's string round trip
   reproduces `formats/roundtrip.json`.

## Dependencies

**Builds on:** DECON-01. **Blocks:** DECON-03, DECON-05, DECON-06.

## Provenance (non-normative)

- Pile construction (full deck, n decks, explicit list):
  `src/basic/types/traits.rs:70-72` (canonical deck), `:96-98` (n
  concatenated decks).
- Draw: `src/basic/types/pile.rs:219-229` (`draw(n)`, all-or-nothing —
  `None` if requested count exceeds length), `:255-260` (draw first),
  `:282-284` (draw last), `:298-302` (draw random). All-or-nothing and
  no-op-on-zero confirmed by property tests:
  `tests/properties.rs:90,101`.
- Sort default (suit-major, descending): comparator
  `src/basic/types/basic_card.rs:178-186`; end-to-end deck check
  `src/basic/types/pile.rs:1336-1351`.
- Sort by rank (rank-major, stable): `src/basic/types/pile.rs:897-899`;
  worked example `:829-838`.
- Extraction — ranks()/suits() deduplicated descending: `src/basic/types/traits.rs:352-360,398-401,436-439`.
  Filter by facet type: `traits.rs:312-350`. Combinations: `traits.rs:187-212`.
  Grouping by facet: `traits.rs:362-370`, `src/basic/types/pile.rs:493-503`.
- Self-validation concept: `src/basic/types/traits.rs:133-140`
  (`deck == deck.shuffled_with_seed(42).sorted()` and a string round
  trip), exercised by every shipped deck's own test suite.
- String round trip (full grammar in DECON-04):
  `src/basic/types/pile.rs:1008-1027` (strict parse), `:331-336`
  (forgiving parse).
