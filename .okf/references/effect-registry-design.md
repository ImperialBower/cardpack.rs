---
type: Reference
title: Design — open effect interpretation (mod / effect registry)
description: How funky effects are dispatched openly rather than through a closed match, written for EPIC-01 Story 8.
tags: [design, funky, effects, registry]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/2026-07-11-effect-registry-design.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The design for open effect interpretation in the funky engine — letting
effects be registered and dispatched rather than hard-matched, so a consumer
can add one without editing the crate.

# Context

Written 2026-07-11 on the `funky` branch for
[EPIC-01](/references/epic-01-funky.md) Story 8. The distilled architecture
view is [funky engine](/architecture/funky-engine.md).

# In-repo path

`docs/2026-07-11-effect-registry-design.md`

This concept is a pointer, not a copy — the linked document is authoritative.
