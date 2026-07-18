# DECON-06: Tarot And Skat

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice specifies two deck **vocabularies** that share no cards, suits,
or ranks with the French family (DECON-05): **Tarot** (78 cards, Major and
Minor Arcana) and **Skat** (32 cards, German-suited). Both are expressed
entirely in terms of DECON-01's card model — a card as (rank facet, suit
facet) with precedence weight — and use DECON-02's pile machinery and
DECON-04's string forms unchanged. No new mechanism is introduced here;
only new vocabulary content.

Out of scope: the French family (DECON-05); localized name strings for
either deck (DECON-07); and the general machinery a consumer would use to
author a third, entirely new vocabulary (DECON-08) — this epic specifies
only the content of two already-defined vocabularies.

## Status
| Component | Status |
|---|---|
| Tarot — Major Arcana | Planned |
| Tarot — Minor Arcana | Planned |
| Tarot — Major-outranks-Minor | Planned |
| Skat — suits and ranks | Planned |
| Skat — composition and order | Planned |

## Goals

- Define **Tarot** (78 cards) as one suit facet holding all 22 **Major
  Arcana** trumps (each an individually named rank), plus four ordinary
  suit facets each carrying a 14-rank **Minor Arcana** ladder with an
  extended court (King, Queen, Knight, Page).
- Establish that **Major Arcana outrank every Minor Arcana card**, as a
  direct consequence of DECON-01's suit-major precedence rule — no special
  case is needed beyond assigning the Major Arcana suit facet the highest
  suit weight.
- Define **Skat** (32 cards) as four German-named suits × eight
  German-named ranks, one copy each, with its own precedence order.
- Record, as historical context rather than a requirement, that the
  original's Skat suit *display symbols* happen to reuse the French suits'
  glyphs — a Provenance-level accident, not evidence Skat borrows from the
  French vocabulary.

## Scope

Both decks' full ordered card lists (index, rank name, suit name, symbol,
position) are the normative census in `vectors/tarot-skat/compositions.json`.

- **Tarot Major Arcana — 22 cards, one suit facet.** All 22 share a single
  suit facet ("Major Arcana"); what varies per card is the rank facet. In
  descending precedence (canonical emission order, highest first): Fool,
  Magician, High Priestess, Empress, Emperor, Hierophant, Lovers, Chariot,
  Strength, Hermit, Wheel of Fortune, Justice, Hanged Man, Death,
  Temperance, Devil, Tower, Star, Moon, Sun, Judgement, World. This is a
  strict total order over 22 distinct rank facets — nothing about the
  sequence itself is ambiguous or contiguous-weight-dependent (see Design).
- **Tarot Minor Arcana — 56 cards, four suit facets.** Suits, descending
  precedence: Wands, Chalices, Swords, Pentacles. Each suit carries the
  same 14-rank ladder, descending: King, Queen, Knight, Page, Ten, Nine,
  Eight, Seven, Six, Five, Four, Three, Deuce, Ace — the court is extended
  with Knight and Page in place of a single Jack.
- **Major-outranks-Minor.** The Major Arcana suit facet's precedence weight
  exceeds every Minor Arcana suit facet's weight. Combined with DECON-01's
  suit-major canonical order, this alone produces "all 22 Major Arcana
  cards emit before any Minor Arcana card" — no additional ordering rule
  is needed. Within Minor Arcana, suit-major-then-rank applies exactly as
  DECON-01 defines it.
- **Skat — 32 cards, one copy each.** Suits, descending precedence: Eichel,
  Laub, Herz, Schellen. Ranks, descending precedence: Daus, Zehn, König,
  Ober, Unter, Neun, Acht, Sieben. Canonical emission order is suit-major
  descending, exactly DECON-01's default rule — Eichel's eight ranks
  (Daus→Sieben), then Laub's, then Herz's, then Schellen's.
- **Skat naming is fully German**, both the long names above and each
  suit's/rank's one-character index letter (Eichel→`E`, Laub→`L`, Herz→`H`,
  Schellen→`S`; Daus→`D`, Zehn→`Z`, König→`K`, Ober→`O`, Unter→`U`,
  Neun→`9`, Acht→`8`, Sieben→`7`). Skat's rank names are not French-rank
  synonyms in disguise — "Daus" names Skat's own highest-ranking trump
  slot, which is *not* semantically "Ace" or "Two" in the ordinary sense;
  it is simply the top of Skat's own eight-rank ladder.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Tarot Major Arcana rank ladder | 22 distinct, individually named ranks under one suit facet, strict total order Fool→World | `vectors/tarot-skat/compositions.json` (`"tarot"`, positions 0–21) |
| Tarot Minor Arcana suits & ladder | 4 suits (Wands, Chalices, Swords, Pentacles) × 14-rank court ladder (King…Ace) | `compositions.json` (`"tarot"`, positions 22–77) |
| Major-outranks-Minor invariant | Major Arcana suit facet's weight exceeds all Minor Arcana suit facets' weights | `compositions.json` (`"tarot"`, full sequence) |
| Skat suit vocabulary & order | Eichel > Laub > Herz > Schellen, German names + index letters | `compositions.json` (`"skat"`) |
| Skat rank vocabulary & order | Daus > Zehn > König > Ober > Unter > Neun > Acht > Sieben | `compositions.json` (`"skat"`) |
| Skat composition | 32 = 4 suits × 8 ranks, one copy each, suit-major descending | `compositions.json` (`"skat"`) |

*(File: `vectors/tarot-skat/compositions.json`.)*

## Design

Both decks fit DECON-01's card model without extension. Tarot's Major
Arcana is not a special case mechanically — it is one suit facet like any
other, just one whose 22 member cards each carry a unique, individually
named rank facet rather than sharing a small rank ladder across four
suits. Giving that one suit facet the highest suit-precedence weight is
sufficient, by DECON-01's suit-major rule, to make every Major Arcana card
outrank every Minor Arcana card — a rebuild needs no Tarot-specific
ordering logic beyond assigning that one weight correctly.

The Major Arcana rank ladder's precedence weights are **not contiguous** in
the original between its two lowest-ranked trumps (Judgement, then World):
one weight value is skipped. This is exactly the kind of accident DECON-01
already anticipates and permits (see DECON-01's "Whether weights are
contiguous" freedom) — the *observable* fact a rebuild must reproduce is
the ordering itself (Judgement immediately outranks World, both below Sun,
both above nothing), not any particular weight number. Do not encode the
gap as meaningful; it carries no domain rule.

Skat needs nothing beyond a second, independent suit list and rank ladder
alongside French's (DECON-05) and Tarot's — the three vocabularies never
mix. Skat's suit *display symbols* in the original happen to be the same
glyphs (♣♠♥♦) used for the French suits, even though Skat's suit *names*
and *index letters* are entirely German and distinct from French's. This
is worth recording precisely because it could be misread as a hidden
kinship between the two vocabularies: it is not one. `compositions.json`'s
`symbol` field pins the original's exact choice as part of the full
census; a rebuild reproducing every field of that vector will reproduce
the coincidence along with everything else, but the coincidence itself
carries no domain meaning (see Provenance).

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice — vocabulary-authoring machinery is DECON-08; this epic only specifies two already-defined vocabularies. |
| Administrative | Enumerate and instantiate the Tarot or Skat deck kind by name | Alter either deck's suits, rank ladder(s), or composition | Instantiating Tarot or Skat never affects the other, nor either deck's own definition (cross-ref DECON-08's registry). |
| User/client | Hold, draw from, sort, and shuffle pile instances of either deck | Alter which cards define Tarot or Skat, or move a card between the Major and Minor Arcana suit facets | Each pile instance is an independent copy (DECON-01/02's value semantics, unchanged here). |
| Observer/operator | — | — | N/A for this slice. |

## Work Items
### Phase 0 — Skat vocabulary
- [ ] **0a.** Define Skat's four suit facets (Eichel, Laub, Herz, Schellen)
  and eight rank facets (Daus…Sieben) with their precedence order. Proven
  by `compositions.json` (`"skat"`).
- [ ] **0b.** Compose Skat's 32-card census, one copy each, suit-major
  descending. Proven by `compositions.json` (`"skat"`).

### Phase 1 — Tarot Major Arcana
- [ ] **1a.** Define the Major Arcana suit facet and its 22 individually
  named rank facets, Fool→World descending. Proven by `compositions.json`
  (`"tarot"`, positions 0–21).

### Phase 2 — Tarot Minor Arcana
- [ ] **2a.** Define the four Minor Arcana suit facets (Wands, Chalices,
  Swords, Pentacles) and the shared 14-rank court ladder. Proven by
  `compositions.json` (`"tarot"`, positions 22–77).

### Phase 3 — Combined Tarot ordering
- [ ] **3a.** Assign the Major Arcana suit facet the highest suit-precedence
  weight so that DECON-01's suit-major rule alone produces "all Major
  Arcana before any Minor Arcana." Proven by `compositions.json` (`"tarot"`,
  full 78-card sequence).

## Test Plan

- **Given** the Tarot deck constructed in canonical order, **when** its
  78 cards are inspected, **then** positions 0–21 are the Major Arcana
  Fool→World in that exact order, and positions 22–77 are the four Minor
  Arcana suits (Wands, Chalices, Swords, Pentacles) each King→Ace, matching
  `compositions.json` (`"tarot"`) position by position.
- **Given** the Skat deck constructed in canonical order, **when** its 32
  cards are inspected, **then** the sequence is Eichel's eight ranks
  (Daus→Sieben), then Laub's, then Herz's, then Schellen's, matching
  `compositions.json` (`"skat"`) position by position.
- **Given** any Major Arcana card and any Minor Arcana card, **when** their
  precedence is compared, **then** the Major Arcana card always outranks
  the Minor Arcana card.

## Not specified (implementer's choice)

- **Whether Major Arcana is modeled as a distinct facet *kind* (a tag
  separate from an ordinary suit) or as an ordinary suit facet with the
  highest weight.** Only the resulting outranking behavior is normative;
  DECON-01 already leaves storage representation of facets open.
- **Whether the Major Arcana rank-weight gap (Judgement/World) is
  reproduced as a literal gap or closed up**, per DECON-01's existing
  "weights need not be contiguous" freedom — only the ordering is
  normative.
- **Skat's suit display-symbol glyphs**, beyond matching
  `compositions.json` for full vector conformance: nothing requires a
  rebuild's own from-scratch Skat implementation to reuse French glyphs
  (acorn/leaf/heart/bell iconography is traditional for Skat and equally
  valid) — a rebuild targeting only the domain-essential behavior (names,
  index letters, order, census) rather than exact vector reproduction is
  free to choose its own suit symbols.
- **Internal grouping/tagging of Tarot's court cards** (Knight/Page as a
  distinct category from the numeral ranks) — the ladder order is
  normative, the tagging mechanism is not.

## Spec decisions

None specific to this epic (the Major Arcana weight gap is covered by
DECON-01's existing non-contiguous-weight freedom, not a new decision).

## Verification

Any implementation must reproduce every entry in
`vectors/tarot-skat/compositions.json`:
1. Tarot's 78-card census matches exactly, including the Major/Minor split
   at position 21/22 and the full Fool→World and per-suit King→Ace
   sequences.
2. Skat's 32-card census matches exactly, suit-major Eichel→Laub→Herz→
   Schellen, each suit Daus→Sieben.
3. Every Major Arcana card compares as outranking every Minor Arcana card.

## Dependencies

**Builds on:** DECON-01, DECON-02, DECON-04. **Blocks:** DECON-07.

## Provenance (non-normative)

- Major Arcana suit facet (`pip_type: Special`, weight above all Minor
  Arcana suits): `src/basic/decks/cards/tarot.rs:327-333`; used elsewhere
  as the filter for "is this card a Major Arcana card"
  (`src/basic/types/traits.rs:317-328`, tested against 22 cards).
- Minor Arcana suit weights (Wands > Cups > Swords > Pentacles in the
  source's own naming — the golden vector renders the Cups suit as
  "Chalices"): `src/basic/decks/cards/tarot.rs:326-361`.
- Major Arcana rank weights, Fool(w22)→World(w0), with the Judgement(w2)/
  World(w0) weight-1 gap: `src/basic/decks/cards/tarot.rs:364-520`.
- Minor Arcana rank weights (King…Ace, full court):
  `src/basic/decks/cards/tarot.rs:523-620`.
- Skat suit weights, index letters, and symbol assignment (Eichel/Laub/
  Herz/Schellen mapped to the French glyphs ♣♠♥♦): `src/basic/decks/cards/skat.rs:144-173`.
- Skat rank weights and index letters (Daus…Sieben):
  `src/basic/decks/cards/skat.rs:175-232`.
- Skat deck order and exact `to_string()`/`index()` assertion:
  `src/basic/decks/skat.rs:27-60,106-113`.
- Cross-deck composition/round-trip smoke test exercised by both decks:
  `src/basic/types/traits.rs:133-140`.
