---
type: Reference
title: Design — deterministic shuffle
description: The design behind shuffle_with_seed / shuffle_with_rng and the reproducibility property the test suite is built on.
tags: [design, shuffle, determinism, no_std]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/2026-04-29-seeded-shuffle-design.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The design for `shuffle_with_seed` / `shuffle_with_rng`: permuting a pile
while leaving its multiset of cards unchanged, and the seeding property that
makes a failing shuffle reproducible from its seed.

# Why it constrains the build

Determinism has to hold under `no_std`, which is why rand's `std_rng` feature
is unconditional — see
[rand's std_rng stays unconditional](/decisions/rand-std-rng-unconditional.md).
The language-agnostic contract is
[DECON-03](/references/decon-03-shuffling.md).

# In-repo path

`docs/2026-04-29-seeded-shuffle-design.md`

This concept is a pointer, not a copy — the linked document is authoritative.
