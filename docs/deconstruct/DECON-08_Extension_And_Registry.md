# DECON-08: Extension And Registry

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice specifies the domain's **extension surface**: how a consumer
defines a brand-new deck vocabulary using the same public machinery every
shipped deck (DECON-05, DECON-06) is built from, and gains full pile,
format, and validation behavior automatically; how an **administrative
registry** enumerates and instantiates every shipped deck kind by name;
and the **vocabulary immutability invariant** that holds across the whole
pack — nothing a consumer does to a pile or a card can alter a vocabulary
definition, shipped or consumer-authored. This is the "god-mode +
administrative" epic: it is where the pack's two most powerful actor
perspectives are defined precisely, and where the boundary that keeps
user/client consumption safe is restated as the thing god-mode and
administrative access must never cross either.

Out of scope: the content of any specific vocabulary (DECON-05, DECON-06);
string/numeric forms and localized names (DECON-04, DECON-07) — this epic
specifies only the authoring and enumeration machinery, not any one
vocabulary's data.

## Status
| Component | Status |
|---|---|
| Minimal vocabulary-authoring contract | Planned |
| Automatic capability grant (pile/format/validation) | Planned |
| Deck-kind registry (enumerate + instantiate) | Planned |
| Vocabulary immutability invariant | Planned |

## Goals

- Define the **minimal contract** a consumer must satisfy to introduce a
  new deck vocabulary: a composition (an ordered list of rank-facet/
  suit-facet pairs), a display name, and a localization namespace key.
- Establish that satisfying this contract is **sufficient, on its own**,
  to gain every pile operation (DECON-02), shuffle mode (DECON-03), string
  form and parse rule (DECON-04), and the deck-level self-validation check
  — with no additional code from the consumer for any of those.
- Define a **registry**: a closed enumeration of every shipped deck kind,
  addressable by name, that can produce each one's card list without the
  caller needing to know that deck's specific vocabulary shape in
  advance.
- State the **vocabulary immutability invariant** precisely: no pile or
  card operation — draw, sort, shuffle, filter, map, parse, render,
  serialize — can alter which facets exist in a vocabulary, their names,
  weights, indices, symbols, or values, nor a deck's composition or count,
  for any vocabulary, shipped or consumer-authored.

## Scope

- **Vocabulary-authoring contract.** A consumer supplies: (1) a
  **composition** — an ordered list of (rank facet, suit facet) pairs,
  each facet either reused from an existing vocabulary or newly minted
  with its own name, precedence weight, index character, symbol, and
  optional value (DECON-01); (2) a **display name** for the new deck kind;
  (3) a **localization namespace key** it resolves names through
  (DECON-07); (4) optionally, a suit-to-display-color mapping. Nothing
  else is required. The original documents this precisely with a **worked
  four-card example deck** (two ranks × two suits, reusing existing
  French-vocabulary facets) showing the contract satisfied in full.
- **Automatic capability grant.** Once the contract above is satisfied,
  the new vocabulary's deck: constructs in canonical order and concatenates
  into multi-deck piles (DECON-01/02); draws, sorts (both variants), and
  shuffles (all modes, including seeded) exactly as any shipped deck's pile
  does (DECON-02/03); renders to and parses from index and symbol string
  forms, including the tolerant round-trip and blank-token rules
  (DECON-04); and passes the same round-trip-plus-shuffle/sort
  self-validation check every shipped deck's own construction is proven
  against. None of this requires the consumer to write pile, format, or
  validation logic themselves — it is inherited entirely from satisfying
  the authoring contract.
- **Registry.** A closed, name-addressable enumeration of every shipped
  deck kind — display name and card count for each — that can produce any
  one deck's card list on demand. `vectors/extension-registry/registry.json`
  is the normative census: 12 entries (Canasta 108, Euchre 24, Euchre 32,
  French 54, Pinochle 48, Razz 52, Short 36, Skat 32, Spades 52,
  Standard 52 52, Tarot 78, Tiny 4). The registry is **closed over shipped
  decks only** — a consumer's own newly authored vocabulary is not
  automatically added to it; see Not specified.
- **Vocabulary immutability invariant.** This is the pack-wide guarantee
  that makes every other epic's "consumers cannot alter the vocabulary"
  language true in practice: a card handed to a consumer is a value copy
  of vocabulary data, never a live reference into it (DECON-01); every
  deck instance produced by construction, registry instantiation, or
  parsing is independent of every other instance and of the vocabulary
  definition itself (DECON-02, DECON-05's Administrative row). This holds
  identically whether the vocabulary is one of the twelve shipped decks or
  one a consumer authored using the contract above — authoring a
  vocabulary is a one-time, out-of-band act, not a standing capability
  exposed to anyone holding a pile.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Vocabulary-authoring contract | composition + display name + localization key (+ optional color map) is sufficient to define a new deck | None — described only; cross-ref registry.json's `"Tiny"` entry as the shipped worked example's resulting census |
| Automatic capability grant | pile ops, formats, and self-validation apply unchanged to a newly authored vocabulary | None — proven qualitatively; the shipped decks' own DECON-05/06 vectors are produced by this same contract |
| Deck-kind registry | enumerates every shipped deck by name + count; instantiates each's card list | `vectors/extension-registry/registry.json` |
| Vocabulary immutability | no pile/card operation alters a vocabulary's facets, composition, or count | cross-ref DECON-01, DECON-02, DECON-05 |

*(File: `vectors/extension-registry/registry.json`.)*

## Design

The authoring contract and the pile/format/validation machinery are
deliberately decoupled: DECON-01 through DECON-04 specify behavior in
terms of "a card" and "a vocabulary," never in terms of "one of the twelve
shipped decks." This epic is where that decoupling becomes an explicit,
testable claim: satisfying the minimal contract is *sufficient*, and
DECON-05/06's nine-plus-two shipped vocabularies are not special-cased instances of some
richer internal machinery — DECON-05's nine French-family decks and
DECON-06's Tarot and Skat all satisfy the exact same contract a
consumer's new vocabulary would.

The registry is a separate, smaller concern layered on top: it exists so
that a caller can list and instantiate deck kinds by name at runtime
without needing to know each one's specific vocabulary shape in advance.
The registry is **administrative, not authoring** — it dispatches to
already-defined vocabularies; it grants no ability to change what any of
them is. A consumer's freshly authored vocabulary is real and fully
functional (it satisfies the authoring contract) whether or not it is
ever added to a name-based registry — the registry is a convenience for
enumeration, not a requirement for a vocabulary to exist or work.

The immutability invariant is stated once, here, as the pack's central
domain law, because god-mode (defining a vocabulary) and administrative
access (enumerating/instantiating vocabularies) are exactly the two
perspectives with the most apparent power to break it. Defining a
vocabulary is authoring *data* — once authored, that data is exactly as
immutable to every subsequent consumer, including the author, as a
shipped deck's data is. There is no "owner" capability that lets a
vocabulary's author later reach into a pile a user is holding and change
what a card in it means.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | Define an entirely new deck vocabulary — new or reused rank/suit facets, a composition, a display name, a localization key — using the same public machinery every shipped deck uses; thereby gain full pile/format/validation behavior for it | Mutate a facet or composition after authoring it; grant itself any standing ability to alter a vocabulary's data once defined | Authoring a vocabulary is a one-time, out-of-band act. The moment a vocabulary exists, it is exactly as immutable to every consumer — including its own author — as a shipped deck's vocabulary is. |
| Administrative | Enumerate every shipped deck kind by name; instantiate any of them to obtain its card list, without needing to know its specific vocabulary shape in advance | Alter any shipped deck's suits, rank ladder(s), or composition through the registry; use the registry to reach into or modify a consumer-authored vocabulary | Enumerating or instantiating a deck kind never mutates that deck's own definition, nor any other deck kind's. The registry is closed over shipped decks; it grants no authoring power. |
| User/client | Hold, draw from, sort, shuffle, and render pile instances of any deck — shipped or consumer-authored | Alter which cards define any deck kind, shipped or consumer-authored, through any pile or card operation | Every deck instance a user holds is an independent copy; no operation available to a user can reach back to the vocabulary definition that produced it (restates DECON-01's card-model invariant at the whole-pack level). |
| Observer/operator | Detect that instantiating a deck kind (registry or otherwise) failed to produce a valid vocabulary, via whatever signal the error contract exposes (cross-ref DECON-05's load-failure quirk) | Alter an authoring, registry, or pile outcome by observing it | Observation never mutates a vocabulary, a deck instance, or the registry's own enumeration. |

## Work Items
### Phase 0 — Minimal vocabulary contract
- [ ] **0a.** Define the composition contract: an ordered list of
  (rank facet, suit facet) pairs, a display name, and a localization
  namespace key are sufficient to define a new deck vocabulary.
- [ ] **0b.** Reproduce the worked four-card example (two ranks × two
  suits, reusing existing facets) and confirm its resulting census matches
  `registry.json`'s `"Tiny"` entry (4 cards).

### Phase 1 — Automatic capability grant
- [ ] **1a.** Confirm pile construction, concatenation, draw, sort (both
  variants), and shuffle (all modes) apply unchanged to an instance built
  from a newly authored vocabulary.
- [ ] **1b.** Confirm index/symbol string rendering and tolerant round-trip
  parsing apply unchanged to a newly authored vocabulary.
- [ ] **1c.** Confirm the round-trip-plus-shuffle/sort self-validation
  check applies unchanged to a newly authored vocabulary.

### Phase 2 — Registry
- [ ] **2a.** Enumerate every shipped deck kind by name, in a stable order.
  Proven by `registry.json`.
- [ ] **2b.** Instantiate each shipped deck kind's card list by name,
  matching its count. Proven by `registry.json`.

### Phase 3 — Immutability
- [ ] **3a.** Confirm no pile or card operation — including ones exercised
  in Phase 1 — alters the originating vocabulary's facets, composition, or
  count, for either a shipped or a consumer-authored vocabulary.

## Test Plan

- **Given** a consumer-defined four-card vocabulary (2 ranks × 2 suits,
  reusing existing facet definitions) built via the minimal contract,
  **when** its canonical deck is constructed, **then** it round-trips
  through string form and satisfies the same self-validation check every
  shipped deck satisfies, and its census matches `registry.json`'s
  `"Tiny"` entry.
- **Given** the registry, **when** every shipped deck kind is enumerated,
  **then** each entry's name and card count matches `registry.json`
  exactly, with no duplicates.
- **Given** any deck instance (shipped or consumer-authored), **when** it
  is drawn from, sorted, shuffled, or parsed into, **then** re-instantiating
  the same deck kind afterward produces an identical, unaffected
  vocabulary.

## Not specified (implementer's choice)

- **How a consumer-authored vocabulary becomes part of a name-based
  enumeration.** The shipped registry is closed over the twelve shipped
  decks; nothing in the original adds a consumer's new vocabulary to it
  automatically. Extending or replacing the registry to include consumer
  vocabularies is a further, unspecified consumer action — a rebuild may
  offer an open/extensible registry, a separate consumer-side registry
  type, or no registry extension mechanism at all.
- **Vocabulary source mechanism** for a consumer-authored deck — compiled
  constants, a builder API, runtime-loaded data, or macros — restates
  DECON-01's existing freedom for the authoring act itself.
- **Internal tagging distinguishing "shipped" from "consumer-authored"
  vocabularies** — none is required; both must satisfy the identical
  contract indistinguishably from a rebuild's perspective.
- **Naming collisions** between a consumer-authored vocabulary and a
  shipped one (e.g. reusing "French" as a display name) — the original
  does not address this; a rebuild may allow, warn on, or forbid it.
- **Whether the registry's per-kind instantiation requires type-specific
  code at the call site** — only the observable behavior (name in, card
  list out) is normative; how a rebuild's type system expresses that is
  free.
- **The specific enumeration order.** The original yields its twelve deck
  kinds in one fixed sequence (pinned verbatim in `registry.json` for
  vector-reproduction purposes) and that sequence never changes between
  calls within one build. Only that per-build stability is normative — a
  rebuild is free to enumerate its shipped deck kinds in any order (name
  order, declaration order, or otherwise) as long as one build always
  yields the same order every time it is asked.

## Spec decisions

None specific to this epic.

## Verification

Any implementation must reproduce every entry in
`vectors/extension-registry/registry.json`:
1. All 12 listed deck kinds' names and card counts match exactly, with no
   duplicates. Positional order need not match `registry.json`'s array
   order (see "Not specified"); what is normative is that one build's own
   enumeration order is stable across repeated calls.
2. Each listed deck kind can be instantiated to produce a card list whose
   count matches its `registry.json` entry (cross-ref DECON-05/06's own
   compositions for full per-card content).
3. A vocabulary authored via the minimal contract described in Scope —
   exercised with a small (e.g. four-card) test composition — passes the
   same pile/format/validation checks any shipped deck passes, proven
   qualitatively: the pack's own shipped-deck vectors (DECON-05, DECON-06)
   are themselves evidence the contract is uniform, since they are
   produced by no richer machinery than what this epic specifies.
4. No operation exercised in step 3 leaves the originating vocabulary
   altered when re-instantiated afterward.

## Dependencies

**Builds on:** DECON-01, DECON-02, DECON-04. **Blocks:** — (terminal for
this pack).

## Provenance (non-normative)

- Minimal authoring contract (`base_vec`, `deck_name`,
  `fluent_deck_key`, `colors`): `src/basic/types/traits.rs:22-48`.
- Automatic capability grant (`deck()`, `decks(n)`, `deckvec()`,
  `validate()` as default methods over the minimal contract):
  `src/basic/types/traits.rs:50-141`.
- Facet-authoring primitives (`const fn` constructors, usable by a
  consumer to mint new rank/suit facets): `src/basic/types/pips.rs:118`
  (facet constructor), `src/basic/types/basic_card.rs:58` (card
  constructor).
- Worked four-card example deck ("Custom Deck example," two ranks × two
  suits reusing existing French facets, full walkthrough including
  construction, `combos`, `draw_first`/`draw_last`, and `validate()`):
  `src/lib.rs:190-284` (crate-doc form); shipped equivalent
  `src/basic/decks/tiny.rs:1-64`.
- Registry (closed enumeration, dispatch to each vocabulary's minimal
  contract without exposing or mutating it):
  `src/basic/decks/registry.rs:43-57` (enumeration type),
  `:59-190` (`all()`, `deck_name()`, `base_vec()`, `fluent_deck_key()`
  dispatch).
- Registry closed-over-shipped-decks-only, no auto-registration path for
  consumer types: absence of any generic/dynamic-registration API in
  `src/basic/decks/registry.rs` (confirmed by full-file read).
- Vocabulary immutability restated at the whole-pack level: DECON-01's
  card-as-value-copy design (`src/basic/types/basic_card.rs:36-38`,
  `Copy` derive) and DECON-05's Administrative-perspective evidence
  (instantiating one deck never affects another).
