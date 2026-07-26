---
type: Reference
title: DECON-03 — Shuffling and determinism
description: Permuting a pile without changing its multiset, plus the seeding property that makes shuffling reproducible and testable.
tags: [decon, regeneration-spec, shuffle, determinism]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/deconstruct/DECON-03_Shuffling_And_Determinism.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

**Shuffling** a pile — permuting its cards while leaving the multiset itself
unchanged — plus the one determinism property that makes shuffling testable:
seeding. Builds on [DECON-02](/references/decon-02-pile-operations.md).

# Why it is load-bearing here

The determinism clause is what makes rand's `std_rng` non-negotiable under
`no_std` — see
[rand's std_rng stays unconditional](/decisions/rand-std-rng-unconditional.md).

# In-repo path

`docs/deconstruct/DECON-03_Shuffling_And_Determinism.md`

This concept is a pointer, not a copy — the linked document is authoritative.
