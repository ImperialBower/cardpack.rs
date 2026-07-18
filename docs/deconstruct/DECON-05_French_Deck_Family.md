# DECON-05: French Deck Family

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice specifies nine deck **vocabularies** belonging to the "French
family" — decks built from the standard four-suit vocabulary established
in DECON-01, either whole, subset, extended, duplicated, locally
re-ranked, or emitted in a non-default order. Each is fully determined by
a suit list, one or more rank ladders, and a composition rule. Builds on
the card model (DECON-01), pile machinery (DECON-02), and string forms
(DECON-04).

Out of scope: the Tarot and Skat families, which use their own vocabularies
entirely (DECON-06); localized names for any facet (DECON-07); and the
general machinery for authoring an entirely new deck vocabulary
(DECON-08) — this epic specifies only the content of nine already-defined
vocabularies.

## Status
| Component | Status |
|---|---|
| French (54) | Planned |
| Standard52 (52) | Planned |
| Short (36) | Planned |
| Spades (52) | Planned |
| Euchre24 (24) | Planned |
| Euchre32 (32) | Planned |
| Pinochle (48) | Planned |
| Canasta (108) | Planned |
| Razz (52) | Planned |

## Goals

- Define each of the nine French-family decks as a **composition rule**
  over a shared or locally extended vocabulary: suits, rank ladder(s),
  copy count, and default emission order.
- Establish that a deck vocabulary is **fully determined** by its suit
  list, rank ladder(s), and composition rule — nothing else is needed to
  reproduce its exact card census, position by position.
- Preserve the family's two rank-ladder deviations: Pinochle's local
  **Ten-outranks-King** reordering, and Razz's **ascending Ace-low**
  emission — the only deck whose default order is not DECON-01's
  descending, suit-major default.

## Scope

Every deck uses the four standard suits (Spades, Hearts, Diamonds, Clubs)
in DECON-01's suit-major precedence order unless stated otherwise. "The
French ladder" means the thirteen-rank Ace-through-Deuce ladder from
DECON-01. All nine decks' full card-index lists are the normative census
in `vectors/french-family/compositions.json`.

- **French — 54.** French ladder × 4 suits, plus two distinct joker ranks
  ranked above every suited card. One copy. Default order: jokers first
  (Big, then Little), then suit-major descending, each suit Ace→Deuce.
- **Standard52 — 52.** Same vocabulary as French with both joker ranks
  excluded — the base 52-card ladder × 4 suits, one copy, same order.
- **Short — 36.** Standard52 restricted to Ace-through-Six (Deuce-through-
  Five removed), 4 suits, no jokers.
- **Spades — 52.** Standard52 + both joker ranks, with the Deuce of Clubs
  and Deuce of Diamonds removed (52 + 2 − 2 = 52). The removal is
  asymmetric by suit: Deuce of Spades and Deuce of Hearts remain.
- **Euchre24 — 24.** 4 suits × a six-rank ladder (Ace, King, Queen, Jack,
  Ten, Nine) — the French ladder's top six ranks, no jokers.
- **Euchre32 — 32.** 4 suits × an eight-rank ladder (Ace through Seven),
  one rank wider than Euchre24, no jokers.
- **Pinochle — 48.** 4 suits × a six-rank ladder (Ace, Ten, King, Queen,
  Jack, Nine), each card **duplicated** (2 copies). This ladder is
  **locally re-ranked**, not French's six-rank subset: within a suit, Ten
  outranks King — emission order Ace, Ten, King, Queen, Jack, Nine (each
  twice). A genuine domain rule, not a construction accident.
- **Canasta — 108.** Extended vocabulary in two groups: (1) 16
  "wild-relevant" cards — 2 copies each of Trey of Hearts, Trey of
  Diamonds (the **red** treys), 2 copies each of the two joker ranks, and
  2 copies of every suit's Deuce; (2) 92 cards — two duplicated copies of
  the remaining Ace-through-Trey ladder per suit, except Hearts/Diamonds
  stop at Four (their Treys are already in group 1) while Spades/Clubs run
  the full ladder. Default order: wild-relevant group first (red treys,
  jokers, all four suits' deuces), then the duplicated suit ladders.
- **Razz — 52.** Same vocabulary and composition as Standard52, but
  **default emission is ascending, Ace-low**, within each suit — Ace,
  Deuce, Trey, … King. The only deck in this family breaking DECON-01's
  descending default. Suit-major grouping is unchanged.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| French vocabulary & composition | 54: 2 distinct jokers + French ladder × 4 suits, descending suit-major | `compositions.json` (`"French"`) |
| Standard52 vocabulary & composition | 52: French vocabulary minus both jokers | `compositions.json` (`"Standard 52"`) |
| Short vocabulary & composition | 36: French ladder restricted to Six-and-above, no jokers | `compositions.json` (`"Short"`) |
| Spades vocabulary & composition | 52: Standard52 + 2 jokers, minus Deuce-Clubs/Deuce-Diamonds | `compositions.json` (`"Spades"`) |
| Euchre24 vocabulary & composition | 24: 4 suits × {A,K,Q,J,T,9}, no jokers | `compositions.json` (`"Euchre 24"`) |
| Euchre32 vocabulary & composition | 32: 4 suits × {A,K,Q,J,T,9,8,7}, no jokers | `compositions.json` (`"Euchre 32"`) |
| Pinochle vocabulary, local ladder & composition | 48: 4 suits × {A,T,K,Q,J,9} ×2; **Ten outranks King** | `compositions.json` (`"Pinochle"`) |
| Canasta vocabulary & composition | 108: 16 wild-relevant + 92 duplicated suit-ladder cards | `compositions.json` (`"Canasta"`) |
| Razz vocabulary & emission order | 52: Standard52 vocabulary, **ascending Ace-low** suit-major | `compositions.json` (`"Razz"`) |

*(File: `vectors/french-family/compositions.json`.)*

## Design

A vocabulary in this family is fully determined by a suit list, one or
more rank ladder(s), and a composition rule (single copy, duplicated
copies, a rank subset, a union with an extension set, or a locally
re-ranked ladder scoped to one deck). No deck needs anything beyond these
three ingredients — `compositions.json` is the census a rebuild's
construction logic must match, not a description of how the original
assembles it.

A local re-ranking (Pinochle) or wild-relevant grouping (Canasta) is
scoped to the one deck that declares it; neither alters what a Ten, Trey,
or Deuce *is* for any other deck in the family. Default emission order is,
for eight of nine decks, DECON-01's descending suit-major rule inherited
unchanged; Razz is the one deviation, so a rebuild's per-deck order must
be data the vocabulary itself carries, not a single hardcoded default.

At least one deck (Razz) is, in the original, sourced from external data
rather than compiled-in constants — a concrete instance of the general
"vocabulary source mechanism is the rebuild's choice" freedom DECON-01
leaves open. Nothing about Razz's census requires a data-driven source;
only the original implementation happened to use one.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice — vocabulary-authoring machinery is DECON-08; this epic only specifies nine already-defined vocabularies. |
| Administrative | Enumerate and instantiate any of the nine deck kinds by name | Alter a shipped deck's suits, rank ladder(s), or composition rule | Instantiating one deck never affects another, nor the deck's own definition (cross-ref DECON-08's registry). |
| User/client | Hold, draw from, sort, and shuffle pile instances of any of the nine decks | Alter which cards define one of these nine decks | Each pile instance is an independent copy (DECON-01/02's value semantics, unchanged here). |
| Observer/operator | — | — | N/A for this slice — see "Not specified" for the one load-failure quirk this family surfaces. |
| Flexibility *(lens)* | — | — | At least one deck (Razz) is sourced from external data rather than compiled-in constants, demonstrating a composition rule can be authored as data — informative unless SD-flagged. |

## Work Items
### Phase 0 — Baseline vocabularies
- [ ] **0a.** French: ladder × 4 suits + 2 jokers, descending order.
  Proven by `compositions.json` (`"French"`).
- [ ] **0b.** Standard52: French minus both jokers. Proven by
  `compositions.json` (`"Standard 52"`).
- [ ] **0c.** Short: Standard52 restricted to Six-and-above. Proven by
  `compositions.json` (`"Short"`).

### Phase 1 — Extension and removal variant
- [ ] **1a.** Spades: Standard52 + 2 jokers, minus Deuce-Clubs/Diamonds.
  Proven by `compositions.json` (`"Spades"`).

### Phase 2 — Rank-subset variants
- [ ] **2a.** Euchre24: 4 suits × {A,K,Q,J,T,9}. Proven by
  `compositions.json` (`"Euchre 24"`).
- [ ] **2b.** Euchre32: 4 suits × {A,K,Q,J,T,9,8,7}. Proven by
  `compositions.json` (`"Euchre 32"`).

### Phase 3 — Local re-ranking and duplication
- [ ] **3a.** Pinochle: 4 suits × {A,T,K,Q,J,9} ×2, Ten above King. Proven
  by `compositions.json` (`"Pinochle"`).

### Phase 4 — Extended/wild vocabulary
- [ ] **4a.** Canasta: 16 wild-relevant + 92 duplicated suit-ladder cards.
  Proven by `compositions.json` (`"Canasta"`).

### Phase 5 — Emission-order override
- [ ] **5a.** Razz: Standard52 vocabulary, ascending Ace-low default.
  Proven by `compositions.json` (`"Razz"`).

## Test Plan

- **Given** each of the nine deck kinds, **when** constructed in default
  order, **then** its card-index list matches its `compositions.json`
  entry exactly, including count.
- **Given** the Pinochle deck, **when** one suit's twelve cards are
  inspected, **then** Ten appears immediately after Ace, before King.
- **Given** the Razz deck, **when** one suit is inspected, **then** ranks
  ascend Ace→King rather than descend.
- **Given** the Canasta deck, **when** its 108 cards are inspected,
  **then** exactly 2 copies each of Trey-Hearts, Trey-Diamonds, both
  jokers, and all four suits' deuces form the leading wild-relevant group.

## Not specified (implementer's choice)

- **Vocabulary source mechanism** for any deck — compiled-in constants vs.
  runtime-loaded data — is free, consistent with DECON-01. Razz's original
  is data-driven; nothing here requires that.
- **Load-failure behavior when a composition is sourced from external
  data.** The original's Razz path, on load failure, **silently
  substitutes an empty deck** and only logs — no exception, no error
  return, no signal in the deck's own value. This is a **known quirk of
  the original**, not required behavior: a rebuild may fail loudly, refuse
  construction, retry, or otherwise, as long as it never silently
  fabricates cards outside the intended vocabulary. Do not spec the
  silent-empty-deck behavior as a requirement.
- **Internal grouping/tagging of Canasta's wild-relevant cards** as a
  distinct category — the census is normative, the tagging mechanism is
  not.
- **Representation of Pinochle's local rank ladder** — a second deck-
  scoped ladder, a per-deck weight override, or otherwise — only the
  resulting order is normative.
- **Whether Euchre24/32 are modeled as independent ladders or as "the
  French ladder with a sub-range selected"** — only the resulting card set
  and order is normative.

## Spec decisions

None specific to this epic (SD-01, SD-02, SD-04 are inherited from
DECON-03/DECON-04 where relevant but raise no new question here).

## Verification

Any implementation must reproduce every entry in
`vectors/french-family/compositions.json`:
1. Each of the nine decks' card counts match exactly (54, 52, 36, 52, 24,
   32, 48, 108, 52).
2. Each deck's default-order card-index list matches its entry exactly,
   position by position.
3. Pinochle's per-suit emission places Ten after Ace, before King. Razz's
   emission ascends Ace-low.
4. Canasta's leading 16 cards are the wild-relevant group (2 each:
   Trey-Hearts, Trey-Diamonds, Big Joker, Little Joker, and all four
   suits' Deuces), followed by the duplicated suit ladders.

## Dependencies

**Builds on:** DECON-01, DECON-02, DECON-04. **Blocks:** DECON-07.

## Provenance (non-normative)

- French: deck array `src/basic/decks/french.rs:19-87,31-86`; suit
  weights `src/basic/decks/cards/french.rs:229-265`; rank weights
  `:267-374`; canonical `to_string()` `src/lib.rs:79`.
- Standard52: `src/basic/decks/standard52.rs:18-113`.
- Short: `src/basic/decks/short.rs:21-69,100-107`.
- Spades: `src/basic/decks/spades.rs:19-102,29-82,118-121` (confirms
  Trey-Diamonds/Trey-Clubs as each suit's last card).
- Euchre24: `src/basic/decks/euchre24.rs:21-58,90-93`.
- Euchre32: `src/basic/decks/euchre32.rs:21-65,98-102`.
- Pinochle: `src/basic/decks/pinochle.rs:19-101`; local rank weights (Ten
  above King) `src/basic/decks/cards/pinochle.rs:77-106`.
- Canasta: `src/basic/decks/canasta.rs:19-160`; wild-relevant cards
  `src/basic/decks/cards/canasta.rs`; distinct-suit-pip count
  `canasta.rs:187-199`; `to_string()` `:172-174`; `ranks_index("~")`
  `:180-184`.
- Razz: `src/basic/decks/razz.rs:25-50,58-66` (ascending order); data-
  driven load `:29-34` (via `src/basic/types/basic_card.rs:77-97`);
  **silent-empty-deck quirk**: `razz.rs:30-33` (logs `log::error!`,
  returns an empty `Vec<BasicCard>` — infallible constructor signature,
  `src/basic/types/traits.rs:34`); broken-YAML test fixture
  `src/basic/decks/yaml/razz_bad.yml` (referenced `razz.rs:20`).
- Cross-deck composition/round-trip smoke test exercised by every deck in
  this family: `src/basic/types/traits.rs:133-140`.
