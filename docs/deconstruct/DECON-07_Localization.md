# DECON-07: Localization

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice defines **localized naming** as observable behavior: every rank
facet, every suit facet, and every full card name resolves to a specific
string per supported locale, for any of the vocabularies DECON-05
(French family) and DECON-06 (Tarot, Skat) define. Localization does not
change a card's identity, precedence, or string/numeric forms (DECON-01,
DECON-04) — it is a parallel naming layer over the same facets.

Out of scope: the underlying vocabularies themselves (DECON-05, DECON-06)
and the machinery a consumer uses to add localized names to a brand-new
vocabulary (DECON-08) — this epic specifies only how names resolve per
locale for the vocabularies already defined.

## Status
| Component | Status |
|---|---|
| Rank facet localized names | Planned |
| Suit facet localized names | Planned |
| Full card name composition | Planned |
| Draft-locale confidence tiering | Planned |
| Invalid-name fallback | Planned |

## Goals

- Define **five supported locales**: `en-US` (default/fallback), `de`,
  `fr`, `la`, `tlh`.
- Define that every **rank facet** and every **suit facet**, for every
  vocabulary in the pack, resolves to a specific name string in each of
  the five locales.
- Define how a **full card name** composes from a resolved rank name and
  suit name.
- Record the **confidence tiering** the original itself declares: `en-US`
  and `de` are treated as reviewed/normative; `fr`, `la`, `tlh` are
  author-marked draft and not yet reviewed.
- Describe the **invalid/missing-name fallback** as an observable quirk of
  the original, without binding a rebuild to its exact mechanism.

## Scope

- **Supported locales**: `en-US` (default — used whenever no other locale
  is requested, and as the fallback source for facet fields that carry no
  locale variation, such as precedence weight), `de`, `fr`, `la`, `tlh`.
  `vectors/localization/locales.json` is the normative census of resolved
  rank-name and suit-name strings, keyed per rank/suit by its canonical
  (locale-invariant) index letter, plus three sample full card names per
  locale.
- **Rank and suit facet names.** For every rank and every suit in every
  vocabulary this pack defines, each of the five locales resolves that
  facet to one name string. The census in `locales.json` covers the French
  vocabulary's 13 ranks and 4 suits; the same per-locale resolution
  mechanism applies uniformly to Tarot's and Skat's facets (DECON-06),
  though only French's names are pinned as the golden vector.
- **Full card name composition.** A full card name is the rank name, a
  locale-specific connector, and the suit name — e.g. `en-US`: "Ace of
  Spades"; `de`: "Ass Spaten" (bare-space connector); `fr`: "As de Pique";
  `la`: "As Spathae"; `tlh`: "wa'DIch yan". Two documented exceptions to
  this three-part composition (not covered by the pinned vector, described
  here only): a Major Arcana card's full name is its rank name alone (the
  suit name is suppressed, since every Major Arcana card shares the same
  suit); a Joker's full name prefixes a fixed word before the rank name
  rather than using the rank-connector-suit pattern.
- **Draft-locale confidence tiering (SD-05).** The original's own project
  artifacts (not the code) mark `fr`, `la`, `tlh` as draft translations
  pending native-speaker review, while `en-US` and `de` carry no such
  caveat. See Spec decisions below for how this pack treats the
  distinction.
- **Invalid/missing-name fallback.** When a facet or card name fails to
  resolve to a well-formed name (the underlying identifier is malformed —
  not simply "this locale has no entry"), the original does not always
  fail: one path silently substitutes a fixed placeholder name and
  continues; a separate, stricter path for the same condition signals an
  error instead (cross-ref DECON-04's SD-04, which records the same
  two-contracts-for-one-condition finding from the format/parsing side).
  The domain-observable quirk worth preserving is **"a rebuild need not
  crash on an unresolvable name — degrading to a placeholder is an
  accepted original behavior"** — but the literal placeholder value and
  which path (if any) is silent are not normative; see Not specified.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Rank facet localized name | Resolves to a locale-specific name string, keyed by the rank's canonical index letter | `vectors/localization/locales.json` |
| Suit facet localized name | Resolves to a locale-specific name string, keyed by the suit's canonical index letter | `locales.json` |
| Full card name composition (ordinary cards) | rank name + locale-specific connector + suit name | `locales.json` (`sample_cards`) |
| Full card name composition (Major Arcana / Jokers) | rank name only (Major Arcana); fixed prefix + rank name (Jokers) | None — described only |
| Draft-locale confidence tiering | `en-US`/`de` normative; `fr`/`la`/`tlh` informative | SD-05 |
| Invalid-name fallback | Degrades to a placeholder on one path; hard-errors on a parallel path for the same condition | cross-ref DECON-04's SD-04 |

*(File: `vectors/localization/locales.json`.)*

## Design

Localization is a lookup layered over the facets DECON-01 defines: given a
rank or suit facet's identity and a requested locale, resolution produces
one name string. Nothing about a facet's precedence weight, index
character, or symbol changes across locales — only its display name does.
A full card name is a pure composition of two resolved names plus a
locale-specific connector string; a rebuild needs one connector value per
locale (a word, or a bare space) rather than a locale-specific sentence
template, since the three-part pattern (rank, connector, suit) is uniform
across every ordinary card in every locale.

`locales.json`'s `index` field is a stable, locale-invariant row key
(the facet's canonical index letter, e.g. `"A"`, `"J"`) used only to line
up which rank or suit a given localized `name` belongs to across the five
locale blocks — it is **not** a claim that the displayed index letter
itself is the same in every locale. The original additionally localizes
the index letter shown in a card's compact index string for at least one
locale (German), independent of the long name. That capability is real in
the original but is not captured by the pinned vector's row-key field, so
this epic does not bind a rebuild to reproducing per-locale index letters
— see Not specified.

The confidence-tiering finding is not a code fact; it is a project-process
fact (the original's own locale-file headers and READMEs, not any runtime
behavior) that this pack elevates into a spec decision so a rebuilder
knows which locale strings to trust as a stable target and which to treat
as a snapshot of an unfinished translation.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice — see DECON-08 for how a consumer attaches localized names to a brand-new vocabulary. |
| Administrative | — | — | N/A for this slice. |
| User/client | Request a rank, suit, or full card name in any of the five supported locales; fall back to `en-US` when no locale is requested | Cause a name lookup to mutate the underlying facet or vocabulary | Resolving a name in any locale, or failing to, never alters a card's identity, precedence, or string/numeric forms (DECON-01/04). |
| Observer/operator | Detect that a name failed to resolve cleanly, via whatever signal the error contract exposes (cross-ref DECON-04) | Alter a resolution outcome by observing it | Observing a resolution failure never mutates a pile, card, or vocabulary. |

## Work Items
### Phase 0 — Rank and suit name resolution
- [ ] **0a.** Resolve every French-vocabulary rank facet to a name string
  in each of the five locales. Proven by `locales.json` (`ranks` per
  locale block).
- [ ] **0b.** Resolve every French-vocabulary suit facet to a name string
  in each of the five locales. Proven by `locales.json` (`suits` per
  locale block).

### Phase 1 — Full card name composition
- [ ] **1a.** Compose an ordinary card's full name as rank name + locale
  connector + suit name. Proven by `locales.json` (`sample_cards`).
- [ ] **1b.** Suppress the suit name for a Major Arcana card's full name;
  prefix a fixed word for a Joker's full name.

### Phase 2 — Confidence tiering and fallback
- [ ] **2a.** Record `en-US`/`de` as normative and `fr`/`la`/`tlh` as
  informative per SD-05.
- [ ] **2b.** Confirm an unresolvable name degrades rather than always
  crashing, without binding the exact mechanism.

## Test Plan

- **Given** each of the 13 French rank facets and 4 suit facets, **when**
  resolved in each of the five locales, **then** the resulting name string
  matches `locales.json` exactly.
- **Given** the three sample cards (`AS`, `KH`, `2C`), **when** their full
  names are composed in each locale, **then** they match `locales.json`'s
  `sample_cards` exactly, including each locale's connector (` of `,
  bare space, ` de `, ` `, ` `).
- **Given** a Major Arcana card, **when** its full name is composed,
  **then** the suit name is absent — only the rank name appears.
- **Given** a facet name that fails to resolve to a well-formed name,
  **when** resolution is attempted, **then** the outcome is either a
  defined placeholder or a detectable error — never a silent crash and
  never a fabricated, unrelated name.

## Not specified (implementer's choice)

- **Per-locale index-letter localization.** The original localizes at
  least one facet's displayed index letter independent of its long name
  (e.g. a German-locale index letter differing from the English one for
  the same rank slot). `locales.json` does not pin this — its `index`
  field is a fixed cross-locale row key, not a per-locale value. A rebuild
  may localize index letters, or may keep one fixed index-letter scheme
  across all locales (as DECON-04's index string already assumes); either
  is acceptable.
- **The exact placeholder value and signaling shape for an unresolvable
  name** — a fixed sentinel string, an error return, an exception, or any
  other mechanism, as long as it neither crashes irrecoverably nor
  fabricates an unrelated valid-looking name. Cross-ref DECON-04's SD-04
  for the parallel finding on the parsing side.
- **Which facets/vocabularies beyond French have a fully pinned name
  census.** `locales.json` pins only the French vocabulary's names; Tarot's
  and Skat's per-locale names are asserted to exist and follow the same
  resolution mechanism (cross-ref DECON-06), but no golden vector for them
  ships in this pack.
- **The connector string's storage/representation** — a per-locale
  constant, a template, or otherwise — only the composed output string is
  normative.
- **Whether/how additional locales beyond these five could be added** —
  this epic specifies the five the original ships, not an extensibility
  contract for more (see DECON-08 for the general vocabulary-authoring
  contract, which is a separate question from locale-authoring).

## Spec decisions

> **Spec decision SD-05:** Do draft, author-marked-unreviewed locale
> strings (`fr`, `la`, `tlh`) carry the same normative weight as the
> reviewed ones (`en-US`, `de`)? **Options:** bind a rebuild to all five
> locales' exact strings equally / treat only `en-US` and `de` as
> normative, with `fr`/`la`/`tlh` informative. **Chosen:** the latter —
> `en-US` and `de` are normative; `fr`, `la`, and `tlh` strings in
> `locales.json` are informative (useful as a working target, not a
> pass/fail contract). **Why:** the original's own locale-directory
> READMEs and changelog mark `fr`, `la`, and `tlh` as draft pending
> native-speaker or subject-matter review, while `en-US` and `de` carry no
> such caveat and are the two locales the original's own test suite
> exercises beyond basic wiring checks — binding a rebuild to draft
> strings the source authors themselves flagged as unreliable would freeze
> translations that are expected to change.

## Verification

Any implementation must reproduce every file under `vectors/localization/`:
1. Every French rank and suit facet's name matches `locales.json` for
   `en-US` and `de` exactly (normative per SD-05).
2. The three sample full card names match `locales.json` for `en-US` and
   `de` exactly, including each locale's connector.
3. `fr`, `la`, and `tlh` entries in `locales.json` are reproduced as a
   working target; mismatches there do not fail conformance (SD-05).
4. An unresolvable name produces a detectable, non-crashing outcome.

## Dependencies

**Builds on:** DECON-05, DECON-06. **Blocks:** — (terminal for this pack's
localization slice).

## Provenance (non-normative)

- Fluent key scheme (`"{name}-{key_section}"`, `key_section` ∈
  `index|long|symbol|weight|prime`): `src/localization.rs:92-95`.
- Per-locale index-letter localization (German Jack index `"B"`, distinct
  from the English `"J"`): `src/localization.rs:107-109` (doctest).
- Full card name composition, connector words per locale (` of ` default,
  ` de ` French, bare space for German/Latin/Klingon), Major Arcana
  suit-name suppression, Joker prefix: `src/basic/types/card.rs:220-247,240-247,652-661`.
- Invalid-name fallback split: `FluentName::new` silently defaults to a
  placeholder and logs a warning, `FluentName::from_str` returns a hard
  error for the same malformed input —
  `src/localization.rs:233-246,287-297`, tests at `:326-336,340-363`.
- Locale directories present: `src/localization/locales/{en-US,de,fr,la,tlh}`.
- Draft-tiering source (project artifacts, not code): `fr`/`la`/`tlh`
  locale-directory `README.md` confidence tables, DRAFT status per file —
  `src/localization/locales/fr/README.md:5-7`,
  `src/localization/locales/la/README.md:5-7`,
  `src/localization/locales/tlh/README.md:5-7`; summarized at release in
  `CHANGELOG.md:113-115` ("New locale drafts... All marked as DRAFT
  pending native-speaker / KLI review").
- German Ten name: `src/localization/locales/de/french.ftl:14` renders the
  Ten rank as `Zhen`, suspected to be an upstream typo for `Zehn` (German
  for "ten"); conformance targets `locales.json`'s `Zhen` value as
  extracted (SD-05 treats `de` as normative), and any upstream correction
  will be captured as a Drift log entry in `../MANIFEST.md`, not a silent
  edit to this pack.
