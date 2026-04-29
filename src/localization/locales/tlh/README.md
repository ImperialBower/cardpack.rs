# tlh/ — Klingon (tlhIngan Hol) localization

| File | Status | Confidence | Needs |
|---|---|---|---|
| `french.ftl` | DRAFT | Low (~40%) | KLI-savvy reviewer |
| `skat.ftl`   | DRAFT | Low (~30%) | KLI-savvy reviewer with Skat awareness |
| `tarot.ftl`  | DRAFT | Low (~25%) | KLI-savvy reviewer with Tarot familiarity |

These files were drafted as a starting point. They are wired up automatically
— adding a locale directory under `src/localization/locales/` is enough; the
`static_loader!` macro in `src/localization.rs` discovers the new locale at
build time. A `cargo build` after any edit picks up changes.

## Translation policy

Klingon (tlhIngan Hol) canon — Marc Okrand's *The Klingon Dictionary* (TKD),
*Klingon for the Galactic Traveler* (KGT), and *Power Klingon* — has zero
playing-card vocabulary. This file uses a **pragmatic canon + marked
coinages** policy:

- **Attested roots** are used verbatim where canon supports them
  (`tIq` heart, `ta'` emperor, `Hov` star, `Hegh` death, `ruv` justice,
  numbers `wa'` through `wa'maH`, etc.).
- **Coinages** follow Klingon morphology rules (`-Hom` diminutive, `-'a'`
  augmentative, `-wI'` agentive, `-pu'` plural, `be'` female; noun-noun
  compounds in the standard Klingon order).
- **Every coinage** is marked on the line preceding the entry with
  `# coinage: <back-translation>`. **Every attested term** is marked
  `# attested: <gloss> (<source>)`. This makes the file reviewable
  line-by-line.

Standard Latin transcription with Okrand's case distinctions (`q/Q/H/'`,
`tlh/ng`). No pIqaD (Klingon native script) — the FTL pipeline ships ASCII-
representable strings to terminal contexts.

## Inline marker convention

```
# attested: "heart" (TKD)
name-suit-french-h = tIq
# coinage: "great fool" (qoH + -'a' augmentative); for big-joker
name-rank-french-b = qoH'a'
```

Comments precede the entry rather than trailing it because Fluent (FTL)
syntax treats anything after `=` as the value (no inline comments).

## Before merging

A KLI-savvy reviewer (Klingon Language Institute member or someone with
strong Okrand-canon familiarity) should evaluate each file:

- `french.ftl`: ~half attested roots (suits like `tIq`, ranks like the
  numbers and `ta'`/`toy'wI'`). Coinages cluster around the joker tier
  (`qoH'a'`, `qoHHom`) and the Queen (`ta'be'`). Most coinages should be
  uncontroversial within Klingon morphology rules.
- `skat.ftl`: Suit names are nearly all coinages (`Sor naH` for Acorns,
  `Sor Hap` for Leaves, `wabHom` for Bells). Rank choices reuse
  `wa'DIch` (Ace) for Daus and the directional pair `Dung`/`bIng` for
  Ober/Unter — these are attested and semantically exact.
- `tarot.ftl`: Most speculative file. Major Arcana with attested
  vocabulary: Fool (`qoH`), Emperor (`ta'`), Strength (`HoS`), Justice
  (`ruv`), Death (`Hegh`), Devil (`veqlargh`), Star/Moon/Sun
  (`Hov`/`maS`/`jul`), World (`qo'`), Lovers (`parmaqqaypu'`). Heavy
  coinage zone: Magician, High Priestess, Hierophant, Hermit, Hanged Man,
  Wheel of Fortune, Tower, Judgement. Reviewers may prefer alternative
  coinages — current choices prioritize literal back-translatability
  over evocative resonance.

Once reviewed, change each file's first comment line from `# Status: DRAFT —
…` to `# Status: REVIEWED by <name>, <date>` (or remove the status block).
The inline `# attested:` / `# coinage:` markers can stay as documentation.
