---
type: Decision
title: rand's std_rng stays unconditional
description: rand's std_rng feature is deliberately NOT gated on cardpack's std feature — moving it re-breaks seeded shuffle for no_std consumers.
tags: [decision, rand, no_std, invariant]
timestamp: 2026-07-22T13:10:00Z
---

# Decision

In `Cargo.toml`, `rand = { …, features = ["std_rng"] }` is unconditional —
**not** listed under cardpack's `std` feature.

# Why

`shuffle_with_seed` / `shuffled_with_seed` use `StdRng` and must work under
`no_std` (a core promise of the [domain kernel](/architecture/domain-kernel.md)
— determinism is part of the DECON-03 contract). In rand 0.9, `std_rng` only
pulls in `rand_chacha`, with no transitive `std`, so keeping it always-on is
sound.

# How to apply

A tidy-minded refactor that moves `std_rng` under `std = [ …, "rand/std_rng" ]`
will pass std-enabled CI but silently break seeded shuffle for `no_std`
consumers. The Cargo.toml comment says: **do NOT move it**. `make no-std`
([build and test](/workflows/build-and-test.md)) is the guard that catches
this.

# Citations

[1] [Cargo.toml rand dependency comment](../../Cargo.toml)
[2] [DECON-03 Shuffling and Determinism](../../docs/deconstruct/DECON-03_Shuffling_And_Determinism.md)
