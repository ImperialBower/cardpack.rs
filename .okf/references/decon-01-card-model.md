---
type: Reference
title: DECON-01 — Card model and ordering
description: "The atomic unit of the domain: a card as a rank facet plus a suit facet, and the rule that gives a set of cards one canonical order."
tags: [decon, regeneration-spec, card-model, ordering]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/deconstruct/DECON-01_Card_Model_And_Ordering.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

A **card** built from two independent facets — a rank facet and a suit facet —
plus the rule that turns a set of cards into one canonical order. Every other
epic in the pack builds on this slice.

# Relationship to the implementation

The distilled view of how this crate does it is
[card model](/architecture/card-model.md). Nothing in the DECON doc mandates
the Rust implementation; source citations there are non-normative.

# In-repo path

`docs/deconstruct/DECON-01_Card_Model_And_Ordering.md`

This concept is a pointer, not a copy — the linked document is authoritative.
