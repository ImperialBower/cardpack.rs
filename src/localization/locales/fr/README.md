# fr/ — French (français) localization

| File | Status | Confidence | Needs |
|---|---|---|---|
| `french.ftl` | DRAFT | High (~95%) | Quick proofread by any French speaker |
| `tarot.ftl` | DRAFT | Medium (~75%) | Tarot practitioner review (Marseille vs RWS choices) |
| `skat.ftl`  | DRAFT | Low (~50%) | **French Skat player or pagat.com cross-reference** |

These files were drafted as a starting point. They are wired up automatically
— adding a locale directory under `src/localization/locales/` is enough; the
`static_loader!` macro in `src/localization.rs` discovers the new locale at
build time. A `cargo build` after any edit picks up changes.

## Before merging

- `french.ftl`: any French speaker can verify in 5 minutes. The conventional
  rank/suit names are well-fixed (As/Roi/Dame/Valet, Pique/Cœur/Carreau/Trèfle).
- `tarot.ftl`: a Tarot practitioner should review the Major Arcana — they're in
  the Marseille tradition here (Le Mat, Le Bateleur, La Papesse) but a reviewer
  may prefer RWS-French equivalents (Le Fou, Le Magicien, La Grande Prêtresse).
- `skat.ftl`: a French Skat player or [pagat.com](https://www.pagat.com/) is the
  right gate. The most error-prone entry is `name-rank-skat-d` — "Daus" is
  Skat's high trump (functionally an Ace, not a "Deux"); a literal translation
  would be semantically wrong. Translated as "As" here.

Once reviewed, change each file's first comment line from `# Status: DRAFT — …`
to `# Status: REVIEWED by <name>, <date>` (or remove the status block).
