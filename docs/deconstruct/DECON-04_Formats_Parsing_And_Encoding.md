# DECON-04: Formats, Parsing And Encoding

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice defines every **observable string and numeric representation**
of a card and a pile, and the round trips between them: the compact index
string, the display/symbol string, pile renderings of both, tolerant
parsing back into cards, the set of detectable error conditions, and a
handful of optional interop capabilities (a poker-evaluator-compatible
numeric encoding, structured serialization, YAML card lists, colored
terminal output). Builds on the card model (DECON-01) and the pile
(DECON-02); DECON-05/06/08 assume these representations exist.

Out of scope: any specific deck's vocabulary (DECON-05, DECON-06),
localized name strings (DECON-07), and consumer vocabulary-authoring
machinery (DECON-08) — this epic specifies string/numeric forms only,
independent of which vocabulary a card was drawn from.

## Status
| Component | Status |
|---|---|
| Index string | Planned |
| Symbol/display string | Planned |
| Pile string rendering | Planned |
| Tolerant round-trip parsing | Planned |
| Error contract | Planned |
| Numeric interop encoding | Planned |
| Structured serialization | Planned |
| Colored terminal display | Planned |

## Goals

- Define the **index string** and **symbol/display string** as two
  independent, facet-derived presentations of one card.
- Define **pile rendering** of both forms as a space-joined sequence.
- Define **tolerant round-trip parsing**: a token parses regardless of
  letter case or which string form it uses, plus a reserved blank token.
- Define the **error contract** as a normative *set* of detectable
  conditions, leaving the signaling shape to the rebuild.
- Define, as **optional capabilities**: numeric interop encoding,
  structured serialization, YAML card lists, colored terminal output.

## Scope

- **Index string**: rank facet index char + suit facet index char, e.g.
  Jack of Diamonds = `"JD"`. **Symbol/display string**: rank symbol +
  suit symbol, e.g. `"J♦"` (symbols may be multi-byte glyphs). Both derive
  purely from the two facets DECON-01 defines; neither is computed from
  the other.
- **Pile rendering**: a pile's index (or symbol) string is its cards'
  strings joined by a single space, in the pile's current order. See
  `vectors/formats/roundtrip.json` for a full 54-card deck rendered both
  ways, position by position.
- **Tolerant parsing** of one token: accepts index or symbol form, any
  letter case, whitespace-trimmed — `"AS"`, `"as"`, `"A♠"`, `"a♠"` all
  parse to the same card. The reserved blank token `"__"` parses to a
  defined blank card rather than being rejected. Any token resolving to no
  card and not the blank token is **detectably invalid**. See
  `vectors/formats/parse-cases.json`.
- **Pile parsing**: whitespace-separated tokens, each parsed by the rule
  above. At least one mode must exist where any single invalid token
  invalidates the whole parse — no partial result, and the caller can
  distinguish this failure from a successful parse.
- **Error contract (SD-04)**: these conditions must each be detectable —
  distinguishable from success and from each other — by whatever
  mechanism the rebuild chooses: (1) a token resolving to no card and not
  the blank token; (2) a pile string containing such a token; (3) a
  request for more cards than a pile holds in an all-or-nothing operation
  (cross-ref DECON-02's draw). The *shape* of signaling — exceptions, an
  error enum, tagged results, sentinels — is not normative.
- **Numeric interop encoding (SD-02, optional)**: a rebuild *may* offer a
  32-bit card-number scheme used by external poker evaluators — one
  integer per card, collision-free within a deck. Entirely optional; a
  rebuild claiming this capability must reproduce every integer in
  `vectors/formats/ckc-encoding.json` (52 cards, Standard52) exactly — the
  vector is the contract, the internal bit layout is not.
- **Structured serialization (optional)**: a card, facet, and pile can
  each round-trip through a structured encoding to an equal value (no
  golden vector accompanies this capability). **YAML card lists
  (optional)**: a card list specifically round-trips through a plain YAML
  sequence — the same mechanism a data-driven deck can use to source its
  composition externally (see DECON-05's Razz).
- **Colored terminal display (optional)**: only the capability's
  existence is normative — "cards can render with suit-colored output" —
  color mapping and escape mechanism are the implementer's choice.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Index string | rank index char + suit index char | `vectors/formats/roundtrip.json` |
| Symbol/display string | rank symbol + suit symbol | `vectors/formats/roundtrip.json` |
| Pile rendering | per-card strings joined by a space, pile order preserved | `vectors/formats/roundtrip.json` (derived) |
| Tolerant single-token parsing | case-insensitive; index or symbol form; blank token; invalid detectable | `vectors/formats/parse-cases.json` |
| Error contract | invalid token, invalid pile string, not enough cards — set normative, shape free | `vectors/formats/parse-cases.json` (partial); cross-ref `vectors/pile-ops/draw-semantics.json` |
| Numeric interop encoding | one integer per card, collision-free, pinned when claimed | `vectors/formats/ckc-encoding.json` |
| Structured serialization | round-trips a card/facet/pile through structured encoding | None — described only |
| YAML card lists | round-trips a card list through a plain YAML sequence | None — described only |
| Colored terminal display | cards/piles can render with suit-colored output | None — described only |

## Design

Index and symbol strings are independent presentations of the same two
facets (DECON-01); a rebuild stores or computes both from facet data
directly, never derives one from the other. Pile rendering is a trivial
fold; no separate "pile format" exists beyond the join character.

Tolerant parsing recognizes a token regardless of which presentation
produced it — a consumer-ergonomics guarantee, not a security boundary: an
invalid token is rejected, never coerced into the nearest valid card. The
blank token is a reserved sentinel, always resolving to one specific
blank card, not a wildcard.

The error contract is scoped to *conditions*, not *shapes*, in direct
response to a real inconsistency found during extraction: the original
applies two different contracts to the same class of validation failure
depending on entry point — one path silently degrades to a default value
and logs, a parallel path for the same check returns a hard error (see
Provenance). Binding a rebuild to either would encode which entry point
happened to be used, not a domain requirement — hence SD-04 pins only the
condition set.

The three remaining capabilities layer independently over the core
contract above — a minimal rebuild satisfies this epic with none of them;
one aiming at poker-evaluator interop must additionally satisfy the
numeric-encoding vector.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice — see DECON-08. |
| Administrative | — | — | N/A for this slice — see DECON-08. |
| User/client | Render any card/pile to index/symbol strings; parse tolerant text back into cards; opt into numeric-interop, serialization, YAML, or colored-display | Cause a malformed token to resolve to some unrelated card; corrupt the vocabulary via a parse | Rendering/parsing never mutate a deck vocabulary; an invalid token is always distinguishable from a valid one under strict parsing. |
| Observer/operator | Detect that a parse attempt failed, via whatever mechanism the error contract exposes | Alter a parse or render outcome by observing it | Observing a parse failure never mutates a pile or vocabulary; a routine successful parse need not be observable at all. |
| Flexibility *(lens)* | — | — | The optional capability set (numeric encoding, serialization, YAML, colored display) layers independently; core index/symbol parsing carries no such layering — informative unless SD-flagged. |

## Work Items
### Phase 0 — String forms
- [ ] **0a.** Derive index and symbol strings from a card's two facets.
  Proven by `vectors/formats/roundtrip.json`.
- [ ] **0b.** Derive pile-level index/symbol renderings as a space-joined
  sequence in pile order. Proven by `vectors/formats/roundtrip.json`.

### Phase 1 — Parsing
- [ ] **1a.** Parse a single token tolerant of case and form, plus the
  blank token. Proven by `vectors/formats/parse-cases.json`.
- [ ] **1b.** Detect and reject a token resolving to no card. Proven by
  `vectors/formats/parse-cases.json`.
- [ ] **1c.** Parse a whitespace-separated pile string, with a mode where
  any invalid token invalidates the whole parse.

### Phase 2 — Error contract
- [ ] **2a.** Make each SD-04 condition (invalid token, invalid pile
  string, not enough cards) independently detectable.

### Phase 3 — Optional capabilities
- [ ] **3a.** *(optional)* Numeric interop encoding, pinned to
  `vectors/formats/ckc-encoding.json` when claimed.
- [ ] **3b.** *(optional)* Structured serialization round-trip.
- [ ] **3c.** *(optional)* YAML card-list round-trip.
- [ ] **3d.** *(optional)* Suit-colored terminal rendering.

## Test Plan

- **Given** every card in a full deck, **when** its index and symbol
  strings are derived, **then** they match `roundtrip.json` position by
  position.
- **Given** the tokens `"as"`, `"__"`, `"ZZ"`, **when** each is parsed,
  **then** the outcome matches `parse-cases.json`.
- **Given** a pile string with one invalid token among valid ones,
  **when** parsed strictly, **then** the whole parse fails detectably.
- **Given** a rebuild claiming numeric-interop capability, **when** every
  Standard52 card is encoded, **then** the integers match
  `ckc-encoding.json` exactly, with no collisions.

## Not specified (implementer's choice)

- **Error taxonomy/shape** — exceptions, an error enum, tagged results, or
  sentinels are all acceptable, as long as the three SD-04 conditions
  remain independently detectable.
- **Whether a never-fails pile-parsing mode exists.** The original ships
  one, but it cannot distinguish "input was empty" from "input was
  rejected" without an out-of-band side channel (a log line, not part of
  the return value). A rebuild may offer such a mode, make it
  distinguishable by other means, or omit it.
- **The internal algorithm behind the numeric interop encoding** — only
  the card→integer mapping in `ckc-encoding.json` is pinned; behavior for
  cards outside the 52-card vector (jokers, blanks, other families'
  special ranks) is unspecified.
- **Structured serialization's wire format**, beyond "round-trips to an
  equal value." YAML is only required where that capability is claimed.
- **Colored display's color-to-suit mapping and rendering mechanism**, and
  fallback behavior for an unmapped suit.
- **Which token failed within an invalid pile string** — full diagnostic
  detail or only whole-string failure are both acceptable.

## Spec decisions

> **SD-04:** Must error signaling mirror the original's exact per-entry-
> point contract, or only the set of detectable conditions? **Options:**
> bind to the original's shape (including its own inconsistency) / bind
> only to the condition set. **Chosen:** the condition set — invalid
> token, invalid pile string, not enough cards — is normative; shape is
> free. **Why:** the original applies two different contracts to the same
> validation failure depending on call path (one silently degrades and
> logs, a parallel path hard-errors) — see Provenance. Binding to either
> would encode which entry point was used, not a domain requirement.

> **SD-02:** Is numeric interop encoding required, and how tightly
> pinned? **Options:** omit / optional with unspecified mapping / optional
> with pinned mapping. **Chosen:** optional, pinned when claimed —
> `ckc-encoding.json` is the exact contract for any rebuild claiming
> poker-evaluator interop; others skip this capability entirely.

## Verification

Any implementation must reproduce every file under `vectors/formats/`:
1. Index and symbol strings for a full deck match `roundtrip.json`
   position by position.
2. The three cases in `parse-cases.json` each produce the documented
   outcome.
3. If numeric-interop capability is claimed, every card's encoded integer
   matches `ckc-encoding.json`, with no collisions.
4. Each of the three SD-04 conditions is independently detectable (cross-
   ref `vectors/pile-ops/draw-semantics.json` for condition 3).

## Dependencies

**Builds on:** DECON-01. **Blocks:** DECON-05, DECON-06, DECON-08.

## Provenance (non-normative)

- Index string: `src/basic/types/basic_card.rs:99-115`; pile-level join
  `src/basic/types/pile.rs:354-370`.
- Symbol/display string: `src/basic/types/basic_card.rs:163-167`;
  pile-level join `src/basic/types/pile.rs:973-984`.
- Tolerant parsing: `src/basic/types/card.rs:373-401`, round-trip proof
  over a full base vec `:504-521`. Blank token `:385-387,401`.
- Strict pile parse (whole-string failure): `src/basic/types/pile.rs:1008-1027,1011-1019`.
- Never-fails pile parse: `src/basic/types/pile.rs:331-336`.
- Error-contract inconsistency motivating SD-04: `src/localization.rs:233-246,287-297`
  (silent-default vs. hard-error variants of the same validation),
  `:326-336,340-363` (tests exercising both — `FluentName::new` defaults
  silently, `FluentName::from_str` hard-errors on the same malformed
  input), `src/common/errors.rs:5-34` (`CardError` enum).
- Numeric interop encoding: trait `src/basic/types/traits.rs:148-166`,
  impl `src/basic/types/basic_card.rs:124-161`, fixed table
  `src/basic/types/pips.rs:99-103`; wire-compatibility verified over 53
  parametrized cases, `src/basic/types/basic_card.rs:254-314`.
- Structured serialization: derives on facet/card/pile —
  `src/basic/types/pips.rs:77-79`, `basic_card.rs:36-38`, `card.rs:32-34`,
  `pile.rs:63-65`; the deck-agnostic raw-collection type does *not*
  derive this, evidence it's a genuinely optional layer.
- YAML card lists: `src/basic/types/basic_card.rs:77-97`, round-trip
  doctest `:206-212` (54 cards).
- Colored terminal display: `src/basic/types/card.rs:80-124`,
  `src/basic/types/pile.rs:913-943`; unmapped-suit fallback `card.rs:82-87`.
