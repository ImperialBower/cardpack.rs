---
type: Reference
title: DECON MANIFEST — regeneration pack
description: "The contract for the /deconstruct regeneration pack: satisfy every DECON epic and reproduce all golden vectors, in any language."
tags: [decon, regeneration-spec, manifest, contract]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/deconstruct/MANIFEST.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The index and contract for the regeneration pack: an implementation **in any
language** that satisfies every DECON epic and reproduces all golden vectors
is a functional regeneration of this crate. Language, layout, and internal
design are the implementer's choice.

# Pinning

Pinned to commit `24ad604f4bb29e51de0c2835d149c8e0aea91854` (2026-07-18).
Golden vectors under `docs/deconstruct/vectors/*.json` are regenerated with
`cargo run --example deconstruct_vectors`.

# The epics

[DECON-01](/references/decon-01-card-model.md),
[02](/references/decon-02-pile-operations.md),
[03](/references/decon-03-shuffling.md),
[05](/references/decon-05-french-deck-family.md),
[06](/references/decon-06-tarot-and-skat.md),
[07](/references/decon-07-localization.md),
[08](/references/decon-08-extension-and-registry.md).

# In-repo path

`docs/deconstruct/MANIFEST.md`

This concept is a pointer, not a copy — the linked document is authoritative.
