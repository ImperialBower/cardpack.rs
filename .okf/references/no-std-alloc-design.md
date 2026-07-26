---
type: Reference
title: Design — no_std + alloc support (0.7.0)
description: The plan for making the crate build without std while keeping alloc-backed types, targeting the 0.7.0 release.
tags: [design, no_std, alloc, portability]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/superpowers/specs/2026-05-01-no-std-alloc-design.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The design for `no_std` + `alloc` support: which types stay, what moves behind
the default-on `std` feature, and how the bare-metal and wasm targets get
verified in CI.

# Context

Written 2026-05-01 targeting 0.7.0. The gates that keep it true today are
`make no-std` and the CI matrix — see
[build and test](/workflows/build-and-test.md) and
[domain kernel](/architecture/domain-kernel.md).

# In-repo path

`docs/superpowers/specs/2026-05-01-no-std-alloc-design.md`

This concept is a pointer, not a copy — the linked document is authoritative.
