---
type: Architecture
title: Domain-kernel posture
description: cardpack is a value-type domain kernel — pure by default, no I/O in the core, format crates kept out of the public API.
tags: [architecture, purity, domain-kernel, audit]
timestamp: 2026-07-22T13:10:00Z
---

# The pattern

The crate follows the **domain-kernel** pattern: a pure, delivery-agnostic
core behind a narrow boundary. It is a *value/collection* kernel (cards, pips,
piles, decks) rather than a state-machine kernel — there is no game state, no
acting parties, no hidden information. The kernel's value is **purity +
portability** (no_std, wasm, bare-metal), not a transition contract; a WIT/
component boundary was assessed and explicitly *not* recommended.

# Invariants enforced

1. **Pure — no I/O of its own.** The only filesystem touch in the whole crate
   is `BasicCard::cards_from_yaml_file`, quarantined behind the `std-io`
   feature ([decision](/decisions/std-io-outside-full.md)). The historical
   violation — `Razz::base_vec()` reading `razz.yaml` from the CWD at runtime,
   silently returning an empty deck on failure — was fixed by embedding the
   YAML with `include_str!`.
2. **No format/transport crate in the public API.** Errors are boxed;
   `serde_norway::Error` does not leak.
3. **Pure by default.** `default = []`
   ([feature flags](/architecture/feature-flags.md)). This was the other 2026-07
   audit finding: defaults used to pull in presentation, YAML, i18n, and
   fs-loading.
4. **Delivery-agnostic.** CI gates no_std/alloc, wasm32, and bare-metal
   (`thumbv7em-none-eabihf`) builds ([build and test](/workflows/build-and-test.md)).

# Where it's documented

The authoritative assessment is
[docs/audit-2026-07-18-domain-kernel.md](../../docs/audit-2026-07-18-domain-kernel.md)
(scorecard, findings, fix sequencing). Cargo.toml's `[features]` comment cites
it as "Invariant 3."

# Citations

[1] [docs/audit-2026-07-18-domain-kernel.md](../../docs/audit-2026-07-18-domain-kernel.md)
[2] [Cargo.toml](../../Cargo.toml)
