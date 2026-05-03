# la/ — Latin (Latina) localization

| File | Status | Confidence | Needs |
|---|---|---|---|
| `french.ftl` | DRAFT | High (~85%) | Latinist proofread (suit name choices) |
| `tarot.ftl`  | DRAFT | Medium-High (~80%) | Latinist with Renaissance Tarot familiarity |
| `skat.ftl`   | DRAFT | Low-Medium (~55%) | Latinist with Skat awareness; pagat.com cross-reference |

These files were drafted as a starting point. They are wired up automatically
— adding a locale directory under `src/localization/locales/` is enough; the
`static_loader!` macro in `src/localization.rs` discovers the new locale at
build time. A `cargo build` after any edit picks up changes.

## Translation policy

Renaissance and medieval gaming Latin where attested, classical Latin where
unattested but unambiguous. Suit names follow medieval European gaming
conventions (Spathae/Corda/Trifolia/Rhombi for the French deck;
Calices/Gladii/Baculi/Denarii for Tarot — the Italian-Latin tradition). Court
ranks use classical Latin (Rex/Regina/Eques/Famulus); Skat-specific ranks
use Superior/Inferior for Ober/Unter (positional semantics, matching the
German originals).

## Before merging

- `french.ftl`: a Latinist can confirm the suit choices in 5 minutes. Court
  ranks (Rex/Regina/Famulus) and number names (Decem/Novem/Octo/...) are
  uncontroversial classical Latin.
- `tarot.ftl`: a Latinist with Renaissance Tarot familiarity should confirm
  the Major Arcana names. Choices follow attested medieval/Renaissance Latin
  forms: Stultus, Praestigiator, Papissa, Imperatrix/Imperator, Pontifex,
  Suspensus (Hanged Man), Mors, Temperantia, Diabolus, Turris (Tower),
  Iudicium (Judgement), Mundus (World).
- `skat.ftl`: there is no Latin Skat tradition. The translation is direct
  semantic mapping of German suit names (Eichel/Laub/Herz/Schellen ->
  Glandes/Folia/Corda/Tintinnabula). Court ranks use Superior/Inferior for
  Ober/Unter — semantically accurate to the German positional terms.
  rank-d ("Daus") is mapped to "As" — semantically correct (Daus is the
  high trump, functionally an Ace).

Once reviewed, change each file's first comment line from `# Status: DRAFT —
…` to `# Status: REVIEWED by <name>, <date>` (or remove the status block).
