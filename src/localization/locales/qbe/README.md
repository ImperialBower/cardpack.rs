# qbe/ — Belter Creole (Lang Belta) localization

| File | Status | Confidence | Needs |
|---|---|---|---|
| `french.ftl` | DRAFT | Low (~35%) | Lang Belta enthusiast / *Expanse* linguistics fan |
| `skat.ftl`   | DRAFT | Low (~25%) | Lang Belta enthusiast with Skat awareness |
| `tarot.ftl`  | DRAFT | Low (~20%) | Lang Belta enthusiast with Tarot familiarity |

These files were drafted as a starting point. They are wired up automatically
— adding a locale directory under `src/localization/locales/` is enough; the
`static_loader!` macro in `src/localization.rs` discovers the new locale at
build time. A `cargo build` after any edit picks up changes.

## About the locale code

Belter Creole (Lang Belta) — the constructed creole language designed by
linguist Nick Farmer for *The Expanse* (novels by James S.A. Corey; the
Syfy / Amazon Prime TV adaptation) — has no registered ISO 639 / BCP-47
code. We use **`qbe`** from the `qaa`–`qtz` private-use ISO 639 range
because the more semantically accurate form `art-x-belta` is rejected by
the `unic-langid` parser used by `fluent-templates`. See
`src/localization.rs` for the constant declaration.

## Translation policy

Lang Belta is, both in-fiction and structurally, a **creole** drawing
vocabulary from English, German, Spanish, Mandarin, Hindi, Persian,
Japanese, Russian, Arabic, Hebrew, Swahili, French, and Slavic
languages. Published Lang Belta vocabulary (Farmer's notes, *The Expanse*
RPG glossary, fan wikis) is small and contains zero playing-card terms.

This file uses a **loanword-first / coinage-fallback** policy:

- **Loanwords** are taken from one of Lang Belta's source languages —
  most commonly Spanish, which has the heaviest substrate presence in
  Lang Belta on-screen (`spada` Spanish *espada* "sword/spades", `dama`
  Spanish "lady/queen", `korazon` Spanish *corazón* "heart").
- **Coinages** follow Lang Belta phonology and orthography
  conventions: Latin transcription (no diacritics), `k` preferred over
  `c`, `x` for `sh`-sounds, simple syllable structure, frequent
  initial-vowel elision (Spanish *espada* → `spada`).
- **Every loanword** is marked on the line preceding the entry with
  `# loanword: <gloss>` and a source-language note. **Every coinage**
  is marked `# coinage: <derivation>`. This makes the file reviewable
  line-by-line.

## Inline marker convention

```
# loanword: Spanish "corazón" (heart)
name-suit-french-h = korazon
# coinage: "brother joker" (joka + beratna "brother", attested); for big-joker
name-rank-french-b = joka beratna
```

Comments precede the entry rather than trailing it because Fluent (FTL)
syntax treats anything after `=` as the value (no inline comments).

## Numerals

Lang Belta numerals are not formally documented in Farmer's published
notes; the values used here (`wa, du, tre, fo, fi, sik, set, ot, nai,
das`) are coinages following Lang Belta's typical multi-source-language
mixing pattern. Spanish (*tres* → `tre`), English (*four* → `fo`,
*five* → `fi`, *six* → `sik`), and Hindi (*das* "10" → `das`) all
contribute. These are the most likely set of entries to be revised on
review.

## Before merging

A Lang Belta enthusiast — ideally someone with access to Farmer's
published notes, *The Expanse* RPG sourcebooks, or who is active in
the conlang's online communities — should evaluate each file:

- `french.ftl`: Suit names use direct Spanish loanwords. Court cards
  (`kapten`, `dama`, `valet`) borrow from Spanish/French. Numerals
  are coinages and the most volatile part of the file.
- `skat.ftl`: Suits are coinage-heavy because the German Eichel /
  Laub / Herz / Schellen tradition has no Lang Belta analog. The
  `alto`/`bajo` choice for Ober/Unter is a clean Spanish loanword
  pair — semantically exact but reviewers may prefer something more
  "Belter."
- `tarot.ftl`: Most speculative file. Spanish loanwords cover most of
  the Major Arcana cleanly (`tonto`, `mago`, `imperator`, `muerte`,
  `diablo`, `luna`, `sol`, `mundo`); coinage zones are the
  abstract/hierarchical concepts (Hierophant, Hanged Man, Temperance,
  Judgement). Reviewers may prefer alternative Spanish loans or
  genuinely Belter-flavored coinages.

Once reviewed, change each file's first comment block from
`# Status: DRAFT — LOW CONFIDENCE …` to `# Status: REVIEWED by <name>,
<date>` (or remove the status block). The inline `# loanword:` /
`# coinage:` markers can stay as documentation.
