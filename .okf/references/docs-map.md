---
type: Reference
title: Map of the in-repo documentation
description: Where the deep documentation lives — DECON regeneration pack, EPICs, audits, design docs, technical debt — and which file is authoritative for what.
tags: [docs, map, reference]
timestamp: 2026-07-22T13:10:00Z
---

# Authoritative sources by topic

| Topic | Authoritative doc |
|---|---|
| Language-agnostic behavioral spec + golden vectors | [docs/deconstruct/MANIFEST.md](../../docs/deconstruct/MANIFEST.md) → DECON-01…08 + `vectors/*.json` (regenerate via `cargo run --example deconstruct_vectors`) |
| Kernel purity assessment | [docs/audit-2026-07-18-domain-kernel.md](../../docs/audit-2026-07-18-domain-kernel.md) (earlier: audit-2026-04-29, audit-2026-04-09) |
| Balatro/funky scope & status | [docs/EPIC-01_Funky.md](../../docs/EPIC-01_Funky.md) + children 01a–01e (all closed 2026-07-18); next: [EPIC-02 Ganjifa](../../docs/EPIC-02_Ganjifa.md) |
| Effect/mod registry design | [docs/2026-07-11-effect-registry-design.md](../../docs/2026-07-11-effect-registry-design.md) |
| Seeded shuffle design | [docs/2026-04-29-seeded-shuffle-design.md](../../docs/2026-04-29-seeded-shuffle-design.md) |
| Latin/Klingon locale design | [docs/2026-04-29-la-tlh-locales-design.md](../../docs/2026-04-29-la-tlh-locales-design.md) |
| no_std/alloc migration design | [docs/superpowers/specs/2026-05-01-no-std-alloc-design.md](../../docs/superpowers/specs/2026-05-01-no-std-alloc-design.md) |
| Outstanding work | [BACKLOG.md](../../BACKLOG.md) and [docs/TECHNICAL_DEBT.md](../../docs/TECHNICAL_DEBT.md) (maintained by the `/backlog` skill) |
| WASM consumer setup | [docs/wasm.md](../../docs/wasm.md) ([wasm workflow](/workflows/wasm.md)) |
| Release history | [CHANGELOG.md](../../CHANGELOG.md); [docs/release-v0.7.0.md](../../docs/release-v0.7.0.md) |

# Conventions

* **EPIC docs** (`docs/EPIC-NN_*.md`) are durable, phase-structured specs with
  status tables; the EPIC is the single source of truth for its scope —
  TECHNICAL_DEBT.md mirrors it and has gone stale before (noted in-file).
* **DECON docs** are the /deconstruct output: a contract such that any
  implementation reproducing all golden vectors is a functional regeneration
  of this crate, pinned to commit `24ad604` (2026-07-18).

# Citations

[1] [docs/deconstruct/MANIFEST.md](../../docs/deconstruct/MANIFEST.md)
