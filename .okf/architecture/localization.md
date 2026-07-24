---
type: Architecture
title: Localization (Fluent i18n)
description: Card, rank, and suit names resolve per locale via Project Fluent — en-US, de, fr, la (Latin), tlh (Klingon) — behind the i18n feature.
tags: [i18n, fluent, locales]
timestamp: 2026-07-23T00:00:00Z
---

# How it works

Behind the `i18n` feature (implies `std`), `src/localization/` uses
[fluent-templates](https://github.com/XAMPPRocky/fluent-templates) to resolve
card names. `Card::fluent_name_default()` gives English;
`Card::fluent_name(&FluentName::DEUTSCH)` etc. select a locale. Each deck type
carries a `fluent_deck_key()` ([card model](/architecture/card-model.md))
that routes lookups to the right `.ftl` resource set (`french`, `skat`,
`tarot`, `mughal`, `dashavatara`).

# Locales

| Locale | Dir | Notes |
|---|---|---|
| English | `locales/en-US/` | reference locale |
| German | `locales/de/` | |
| French | `locales/fr/` | has a README on translation confidence |
| Latin | `locales/la/` | draft-tier; see `docs/2026-04-29-la-tlh-locales-design.md` |
| Klingon | `locales/tlh/` | draft-tier; same design doc |

Draft locales use **confidence tiering** — see DECON-07 for the contract.

# Known tension

The `fluent_*` path in `card.rs` carries `HACK` markers: the author suspects
fluent-templates may be outliving its usefulness, with "deck from YAML"
floated as the successor direction
([docs/TECHNICAL_DEBT.md](../../docs/TECHNICAL_DEBT.md)). Treat major
investment here with caution.

# Citations

[1] [src/localization/](../../src/localization.rs)
[2] [DECON-07 Localization](../../docs/deconstruct/DECON-07_Localization.md)
[3] [docs/2026-04-29-la-tlh-locales-design.md](../../docs/2026-04-29-la-tlh-locales-design.md)
