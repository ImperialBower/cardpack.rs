# References

Start here, then follow into the mirrored document you need.

* [Map of the in-repo documentation](docs-map.md) - Where the deep documentation lives — DECON regeneration pack, EPICs, audits, design docs, technical debt — and which file is authoritative for what.

# Mirrored documents

Pointer concepts for in-repo documents outside the bundle, so cross-links from
elsewhere in the bundle stay bundle-relative. Each carries a description and a
`resource:` URI; the in-repo document is authoritative.

## Regeneration pack (DECON)

* [DECON MANIFEST — regeneration pack](decon-manifest.md) - The contract for the /deconstruct regeneration pack: satisfy every DECON epic and reproduce all golden vectors, in any language.
* [DECON-01 — Card model and ordering](decon-01-card-model.md) - The atomic unit of the domain: a card as a rank facet plus a suit facet, and the rule that gives a set of cards one canonical order.
* [DECON-02 — Pile operations](decon-02-pile-operations.md) - The pile as an ordered multiset over one deck vocabulary, and the operations on it: construction, drawing, sorting, extraction.
* [DECON-03 — Shuffling and determinism](decon-03-shuffling.md) - Permuting a pile without changing its multiset, plus the seeding property that makes shuffling reproducible and testable.
* [DECON-05 — French deck family](decon-05-french-deck-family.md) - Nine deck vocabularies built from the standard four-suit vocabulary — whole, subset, extended, duplicated, re-ranked, or reordered.
* [DECON-06 — Tarot and Skat](decon-06-tarot-and-skat.md) - Two vocabularies sharing no cards, suits, or ranks with the French family: Tarot (78) and Skat (32).
* [DECON-07 — Localization](decon-07-localization.md) - Localized naming as observable behavior: every rank, suit, and card name resolves to a specific string per supported locale.
* [DECON-08 — Extension and registry](decon-08-extension-and-registry.md) - The extension surface: defining a new deck vocabulary from the same public machinery every shipped deck uses, plus the administrative registry.

## EPIC docs

* [EPIC-01 — Funky (Balatro-style cards)](epic-01-funky.md) - The Balatro-style joker/effect engine: scope, the five child EPICs, and the deferrals recorded at close-out.
* [EPIC-02 — Ganjifa decks (Mughal + Dashavatara)](epic-02-ganjifa.md) - Adding the two Ganjifa decks with per-suit inverted pip ranking, full localization, and registry integration.
* [EPIC-03 — YAML deck serialization](epic-03-yaml-serialization.md) - The DeckYaml envelope, the YamlDecked blanket trait, and the three-layer test suite that makes every deck round-trip through YAML.

## Audits and design docs

* [Domain-kernel audit (2026-07-18)](audit-domain-kernel.md) - The kernel purity assessment at v0.7.1: the invariants the crate holds itself to and where it was found to bend them.
* [Design — open effect interpretation (mod / effect registry)](effect-registry-design.md) - How funky effects are dispatched openly rather than through a closed match, written for EPIC-01 Story 8.
* [Design — deterministic shuffle](seeded-shuffle-design.md) - The design behind shuffle_with_seed / shuffle_with_rng and the reproducibility property the test suite is built on.
* [Design — Latin (la) and Klingon (tlh) locales](la-tlh-locales-design.md) - Adding two non-natural-language locales, and the Fluent schema fixes that came with them.
* [Design — no_std + alloc support (0.7.0)](no-std-alloc-design.md) - The plan for making the crate build without std while keeping alloc-backed types, targeting the 0.7.0 release.

## Project documents

* [README](readme.md) - The crate's front door: what cardpack is, the cargo feature table, install snippets, and the deck roster.
* [CHANGELOG](changelog.md) - Keep-a-Changelog per-version history; the authoritative record of what changed in each release and what broke.
* [Release notes — v0.7.0](release-v0-7-0.md) - A single narrative summary of the v0.6.8 → v0.7.0 series, complementing the per-version changelog.
* [Using cardpack on WebAssembly](wasm-guide.md) - Consumer-side setup for wasm32-unknown-unknown: the getrandom wasm_js backend, the verified feature matrix, and runtime gotchas.
* [BACKLOG](backlog.md) - Index of outstanding work aggregated from EPIC docs, GitHub issues, and code comments by the /backlog skill.
* [Technical debt register](technical-debt.md) - Tracked debt sourced from TODO / TODO RF / TODO: HACK comments and EPIC docs, maintained by the /backlog skill.
