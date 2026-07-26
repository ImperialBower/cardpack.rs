---
type: Reference
title: Map of the in-repo documentation
description: Where the deep documentation lives — DECON regeneration pack, EPICs, audits, design docs, technical debt — and which file is authoritative for what.
tags: [docs, map, reference]
timestamp: 2026-07-26T12:00:00Z
---

# Authoritative sources by topic

Each row links to this bundle's mirror concept; the in-repo path beside it is
the authoritative document.

| Topic | Authoritative doc |
|---|---|
| Language-agnostic behavioral spec + golden vectors | [DECON MANIFEST](/references/decon-manifest.md) — `docs/deconstruct/MANIFEST.md` → DECON-01…08 + `vectors/*.json` (regenerate via `cargo run --example deconstruct_vectors`) |
| Kernel purity assessment | [domain-kernel audit](/references/audit-domain-kernel.md) — `docs/audit-2026-07-18-domain-kernel.md` (earlier: audit-2026-04-29, audit-2026-04-09) |
| Balatro/funky scope & status | [EPIC-01 Funky](/references/epic-01-funky.md) + children 01a–01e (all closed 2026-07-18) |
| Ganjifa decks | [EPIC-02 Ganjifa](/references/epic-02-ganjifa.md) |
| YAML deck serialization | [EPIC-03 YAML Deck Serialization](/references/epic-03-yaml-serialization.md) |
| Effect/mod registry design | [effect registry design](/references/effect-registry-design.md) — `docs/2026-07-11-effect-registry-design.md` |
| Seeded shuffle design | [seeded shuffle design](/references/seeded-shuffle-design.md) — `docs/2026-04-29-seeded-shuffle-design.md` |
| Latin/Klingon locale design | [la/tlh locales design](/references/la-tlh-locales-design.md) — `docs/2026-04-29-la-tlh-locales-design.md` |
| no_std/alloc migration design | [no_std + alloc design](/references/no-std-alloc-design.md) — `docs/superpowers/specs/2026-05-01-no-std-alloc-design.md` |
| Outstanding work | [BACKLOG](/references/backlog.md) and [technical debt](/references/technical-debt.md) (both maintained by the `/backlog` skill) |
| WASM consumer setup | [wasm guide](/references/wasm-guide.md) — `docs/wasm.md` ([wasm workflow](/workflows/wasm.md) is the distilled version) |
| Release history | [CHANGELOG](/references/changelog.md); narrative notes in [release v0.7.0](/references/release-v0-7-0.md) |
| Crate front door | [README](/references/readme.md) |

# Conventions

* **EPIC docs** (`docs/EPIC-NN_*.md`) are durable, phase-structured specs with
  status tables; the EPIC is the single source of truth for its scope —
  TECHNICAL_DEBT.md mirrors it and has gone stale before (noted in-file).
* **DECON docs** are the /deconstruct output: a contract such that any
  implementation reproducing all golden vectors is a functional regeneration
  of this crate, pinned to commit `24ad604` (2026-07-18).
* **Mirror concepts.** Every document above has a stub concept in
  `references/`, so cross-links from elsewhere in the bundle stay
  bundle-relative (SPEC §5.1) instead of escaping the bundle. The stubs are
  pointers, not copies — they carry a description and a `resource:` URI, and
  the in-repo document remains authoritative.

# Citations

[1] [DECON MANIFEST](/references/decon-manifest.md)
