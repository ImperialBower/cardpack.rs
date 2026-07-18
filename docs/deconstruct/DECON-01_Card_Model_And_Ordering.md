# DECON-01: Card Model And Ordering

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice defines the atomic unit of the whole domain: a **card**, built
from two independent **facets** — a rank facet and a suit facet — plus the
rule that turns a set of cards into one **canonical order**. Every other
epic in this pack (piles, shuffling, formats, the individual deck families,
localization, extension) is built on top of the card model defined here.

Out of scope for this slice: how cards are collected into piles (DECON-02),
how they are permuted (DECON-03), how they are rendered to or parsed from
strings (DECON-04), the specific vocabularies of any shipped deck family
(DECON-05, DECON-06), localized names (DECON-07), and how a consumer
defines a brand-new deck vocabulary (DECON-08) — this epic specifies only
the shape a card and its facets must have, and the ordering rule that falls
out of that shape.

## Status
| Component | Status |
|---|---|
| Rank facet | Planned |
| Suit facet | Planned |
| Card (two-facet identity) | Planned |
| Canonical emission order | Planned |

## Goals

- Define a **card** as the combination of exactly one **rank facet** and
  exactly one **suit facet**, drawn from a single deck's vocabulary.
- Define what a **facet** carries: a name, a precedence weight, a
  one-character index, a display symbol, and an optional numeric value.
- Define **precedence** as a strict total order over the facets of one kind
  within one deck's vocabulary, driven by weight.
- Define the **canonical emission order** for a full set of cards: highest
  precedence to lowest, suit-major.

## Scope

- A card's identity is the pair (rank facet, suit facet). Two cards with
  the same pair are indistinguishable — a deck vocabulary may legitimately
  contain duplicate pairs (see DECON-05's Pinochle/Canasta compositions),
  and nothing about the card model itself prevents that.
- A rank facet and a suit facet are each independently defined records
  with:
  - **name** — the facet's domain-meaningful name (e.g. "Ace", "Spades").
  - **precedence weight** — an ordinal number establishing this facet's
    rank among all facets of the same kind in the same deck's vocabulary.
  - **index** — a single character identifying the facet in the compact
    index string form (see DECON-04).
  - **symbol** — the character(s) used to display the facet (may be a
    multi-byte glyph, e.g. an emoji).
  - **value** — an optional numeric value, present only where a deck's
    domain assigns one (most facets carry no numeric value).
- Within one deck's vocabulary, precedence weight induces a strict total
  order over that vocabulary's rank facets, and a separate strict total
  order over its suit facets. No two rank facets in the same vocabulary
  share a weight; likewise for suit facets.
- **Canonical emission order** for a full card set is: suit-major, each
  suit's cards ordered highest rank precedence to lowest, and suits
  themselves ordered highest suit precedence to lowest. The observable
  contract is simply: **default emission of a card set is descending
  precedence, suit grouped before rank.** See `vectors/card-model/canonical-order.json`
  for the reference sequence.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Card | Identity = (rank facet, suit facet); duplicates permitted in a vocabulary | `vectors/card-model/card-anatomy.json` |
| Rank facet | Carries name, weight, index, symbol, optional value | `vectors/card-model/card-anatomy.json` |
| Suit facet | Carries name, weight, index, symbol, optional value | `vectors/card-model/card-anatomy.json` |
| Deck vocabulary | The set of (rank facet, suit facet) pairs that make up one deck type | `vectors/card-model/canonical-order.json` |
| Canonical emission order | Full card set, suit-major, descending precedence | `vectors/card-model/canonical-order.json` |

## Design

A card is a **value**, not a reference into shared state: constructing or
copying a card never allows mutation of the vocabulary it was drawn from
(see Perspectives, below). A facet record is small and self-contained —
nothing about it depends on other facets, decks, or runtime configuration.

Precedence weight is the single source of truth for ordering. A rebuild
must be able to derive the canonical emission order for any deck purely
from the weights assigned to that deck's facets — no separate, hand-written
ordering table is needed or permitted. Weights need not be contiguous
within a vocabulary (a rebuild must not assume "weight N implies exactly
N+1 distinct facets below it"); they only need to be distinct within one
facet kind in one vocabulary, establishing the strict total order.

The index character and the display symbol are two independent
presentations of the same facet — a rebuild must not derive one from the
other algorithmically; both are part of the facet's defined data.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice — vocabulary-authoring machinery (defining new rank/suit facets and deck compositions) is specified in DECON-08. |
| Administrative | — | — | N/A for this slice — deck-kind enumeration is specified in DECON-08. |
| User/client | Hold, copy, and compare cards freely | Alter a facet's name, weight, index, symbol, or value; alter which facets exist in a deck's vocabulary | A card is a value copy of vocabulary data; no path exists from a card a user holds back to the vocabulary definition that produced it — cards are value-like and the vocabulary itself is immutable to consumers. |
| Observer/operator | — | — | N/A for this slice — no domain events are emitted purely by constructing or ordering cards. |

## Work Items
### Phase 0 — Facet definition
- [ ] **0a.** Define a rank facet record: name, precedence weight, index
  character, display symbol, optional numeric value. Proven by
  `vectors/card-model/card-anatomy.json`.
- [ ] **0b.** Define a suit facet record with the same shape. Proven by
  `vectors/card-model/card-anatomy.json`.

### Phase 1 — Card identity
- [ ] **1a.** Define a card as the pair (rank facet, suit facet); confirm
  duplicate pairs are permitted within one vocabulary (not rejected or
  deduplicated at construction). Proven by `vectors/card-model/card-anatomy.json`.
- [ ] **1b.** Derive a card's index string (rank index + suit index) and
  display symbol string (rank symbol + suit symbol) from its two facets.
  Proven by `vectors/card-model/card-anatomy.json`.

### Phase 2 — Canonical ordering
- [ ] **2a.** Establish precedence weight as a strict total order over one
  vocabulary's rank facets, and separately over its suit facets.
- [ ] **2b.** Derive canonical emission order for a full card set:
  suit-major, descending precedence throughout. Proven by
  `vectors/card-model/canonical-order.json`.

## Test Plan

- **Given** a rank facet and a suit facet from the same vocabulary,
  **when** combined into a card, **then** the card's index and symbol
  strings equal the concatenation of the two facets' index/symbol
  characters. (`vectors/card-model/card-anatomy.json`)
- **Given** a full deck vocabulary, **when** its cards are placed in
  canonical emission order, **then** the sequence matches
  `vectors/card-model/canonical-order.json` exactly, position by position.
- **Given** two facets of the same kind in one vocabulary, **when** their
  weights are compared, **then** the comparison is total (never equal,
  never ambiguous) for any two distinct facets.

## Not specified (implementer's choice)

- **Comparator direction.** The original computes canonical order via a
  comparator that is internally inverted relative to naive integer-weight
  ordering (an implementation accident of how its underlying collection
  sorts). Only the *observable* result is normative: **default emission is
  descending precedence, suit-major.** A rebuild may implement ascending,
  descending, or any other internal comparator convention as long as the
  emitted order matches.
- **Storage representation** of facets and cards (struct, class, tagged
  union, record, map, …).
- **Numeric type/width** for precedence weight and the optional value
  field, beyond "sufficient to hold every weight in the largest shipped
  vocabulary distinctly."
- **Whether weights are contiguous.** The original has at least one
  vocabulary with a gap in its weight sequence; a rebuild is free to use
  contiguous or non-contiguous weights as long as the total order and the
  emitted sequence are correct.
- **Whether/how the optional value field is populated** for facets whose
  domain does not assign one — leaving it absent, zero, or null are all
  acceptable.
- **Vocabulary source mechanism** (compiled-in constants vs. data loaded at
  runtime) — this epic only specifies the resulting facet/card shape, not
  how a vocabulary is authored or stored (see DECON-08).

## Spec decisions

None specific to this epic.

## Verification

Any implementation must reproduce every file under `vectors/card-model/`:
1. Every card's facet fields (name, index, symbol, weight) match
   `card-anatomy.json` for the sampled cards.
2. A full deck's canonical emission order, position by position, matches
   `canonical-order.json`.
3. Precedence weight comparisons are total and consistent with the emitted
   order — no two distinct facets of the same kind compare equal.

## Dependencies

**Builds on:** — (foundational epic). **Blocks:** DECON-02, DECON-04,
DECON-05, DECON-06, DECON-08.

## Provenance (non-normative)

- Card = rank + suit facet pairing: `src/basic/types/basic_card.rs:36-38`
  (struct + derives), `:99-115` (index string), `:163-167` (display
  string).
- Facet record shape (name, weight, index, symbol): `src/basic/types/pips.rs:24-33`
  (kind enum), `:77-79` (derives), `:118` (constructor), `:99-103` (a
  fixed per-facet numeric table used elsewhere for encoding, evidence that
  facets carry more than display data).
- French vocabulary weights (suits): `src/basic/decks/cards/french.rs:229-265`.
- French vocabulary weights (ranks): `src/basic/decks/cards/french.rs:267-374`.
- Canonical high-to-low, suit-major emission, asserted end-to-end for a
  full 54-card vocabulary: `src/basic/types/card.rs:474-487`,
  `src/basic/types/pile.rs:1336-1351`, display string
  `src/lib.rs:79`.
- Non-contiguous weight gap (Major Arcana rank ladder skips one weight
  value): `src/basic/decks/cards/tarot.rs:364-520`.
- Comparator-direction accident: the ordering comparator is explicitly
  documented as reversed relative to naive ascending-weight sort, so that
  default collection sort naturally yields highest-first —
  `src/basic/types/basic_card.rs:178-186` (comment: "Inverts the order so
  that the highest card comes first").
