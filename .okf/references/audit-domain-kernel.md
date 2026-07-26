---
type: Reference
title: Domain-kernel audit (2026-07-18)
description: "The kernel purity assessment at v0.7.1: the invariants the crate holds itself to and where it was found to bend them."
tags: [audit, domain-kernel, purity, invariants]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/audit-2026-07-18-domain-kernel.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

A Mode-A (assess) run of the `domain-kernel` framework against cardpack at
v0.7.1 on `main`: how close the crate is to a pure, delivery-agnostic core,
scored against explicit invariants — no I/O in the core, no format crate in
the public API, no default-on serialization.

# Authoritative for

* **The numbered purity invariants** the rest of the bundle cites by number.
  The distilled version is [domain kernel](/architecture/domain-kernel.md);
  the two invariants that constrain new work most are the single filesystem
  seam ([std-io outside full](/decisions/std-io-outside-full.md)) and boxed
  parse errors.

Earlier rounds — `docs/audit-2026-04-29.md` and `docs/audit-2026-04-09.md` —
are superseded by this one.

# In-repo path

`docs/audit-2026-07-18-domain-kernel.md`

This concept is a pointer, not a copy — the linked document is authoritative.
