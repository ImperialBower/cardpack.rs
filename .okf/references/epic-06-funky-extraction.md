---
type: Reference
title: EPIC-06 — Funky Extraction
description: Move the Balatro-style engine out of cardpack into its own repo, buffoonery, with its history, deprecate cardpack's funky feature in 0.11.2, and remove it in 0.12.0.
tags: [epic, funky, extraction, semver, active]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-06_Funky_Extraction.md
timestamp: 2026-10-18T00:00:00Z
---

# What it covers

The mechanical move behind the [funky-moves-to-own-repo](/decisions/funky-moves-to-own-repo.md)
decision: carry `src/funky/` and its examples into a new repo, `buffoonery`,
with `git filter-repo` so the history survives (history first, scaffold second), rewrite `crate::` paths
to `cardpack::`, and prove parity (the same test counts, and the `buffoon`
example's golden score). Then cardpack deprecates the `funky` feature
(0.11.2) and removes it with its CI, Makefile, alias, README and `.okf/`
mentions (0.12.0).

# Key rules (see the EPIC)

* **No behavior change during the move.** Behavior changes (EPIC-05 Story S1
  onward) start only once the engine passes in the new repo.
* **Removal is semver-major** for cardpack; `cargo-semver-checks` will flag it.
* **Keep the history**: move the files, do not copy-paste them.

# Status

Planned since 2026-10-18. The live status is the `## Status` table in
`docs/EPIC-06_Funky_Extraction.md`; this page does not track it.

# In-repo paths

`docs/EPIC-06_Funky_Extraction.md`

This concept is a pointer, not a copy — the linked document is authoritative.
