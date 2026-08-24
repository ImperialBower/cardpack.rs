---
type: Decision
title: Crypto backends are opt-in features outside full
description: The seal kernel holds slots, order, and revealed values — never ciphertext and never a scheme type parameter — and is dependency-free and always on; real crypto backends (commit-reveal → sha2; seal-aead → chacha20poly1305/hkdf/sha2/zeroize) sit behind their own features, excluded from `full` like std-io.
tags: [decision, purity, features, crypto, seal]
timestamp: 2026-08-24T12:00:00Z
---

# Decision

The **seal kernel** — `Ordinal`/`Codebook`, `Permutation`, `SlotId`, the
non-generic `SlotPile`, `Revealed<D>`, and the five-item `Seal<D>` adapter —
is dependency-free, `alloc`-only, and **always on**. There is no `seal`
feature. **No kernel type is generic over a scheme, and none holds
ciphertext**: the kernel knows a card's *slot*, its *order*, and its *value
once revealed* — cardpack's own rule (EPIC-04 decision 2), argued on its own terms.
Where a deployment must hold sealed payloads — a single trusted dealer — it
keeps a plain `Vec<(SlotId, Bytes)>` beside a `SlotPile` (EPIC-04b
`Custody`), never a `SealedPile<D, S>`.

The **real cryptographic backends** are opt-in features that are deliberately
**not** part of the `full` umbrella:

* `commit-reveal` → `sha2` (provably-fair shuffle,
  [EPIC-04a](/references/epic-04-sealed-decks.md))
* `seal-aead` → `chacha20poly1305`, `hkdf`, `sha2`, `zeroize` (holder-key
  per-card encryption, [EPIC-04b](/references/epic-04-sealed-decks.md))
* `crypto` = both of the above; also outside `full`
* `seal-test-double` → no dependency; exposes `PlaintextSeal` (**no
  security**) and the `seal_roundtrip` conformance helper

Every crypto crate is added to `deny.toml` `[bans].deny` and the CI
`kernel-purity` `BANNED` regex, so it can never reach the pure
(`--no-default-features`) tree by accident.

# Why

`full` is the "batteries" stack, not the "trust me with keys" stack. A
cryptographic dependency is a supply-chain and audit commitment a consumer
must name explicitly — the same reasoning that keeps the one filesystem seam
out of `full` ([std-io decision](/decisions/std-io-outside-full.md)).

The kernel types stay in the kernel because they cost nothing and hide
nothing: a type that never contains a secret cannot leak one, and a type with
no scheme parameter derives `Clone`/`Eq`/`Debug`/`Serialize` without
hand-written impls — so the [domain-kernel](/architecture/domain-kernel.md)
invariants hold (no I/O, no format or cipher type in a public signature, pure
by default) *and* the "a rejected operation changed nothing" property is one
`assert_eq!`. Prior art agrees: a sibling repo's spike that put a scheme
parameter on its table paid 19 `where` bounds and hand-written derives for it
and is being redone. cardpack is designed to be built *on*, not to link to
anything; it does not repeat the experiment. Gating a dependency-free kernel would
add a `cfg` dimension to `CardError`, `Pile::permute`, the prelude and every
doctest for no benefit ([feature flags](/architecture/feature-flags.md)
"Principle").

# How to apply

* A new backend gets its **own** `seal-*` feature and its own `deny.toml` /
  CI ban entries. Never add a crypto crate to `full` or `default`.
* **Never add a payload or a scheme parameter to `SlotPile`.** The day it
  carries bytes beside the slots it is `SealedDeck<S>` again with the type
  parameter erased, and the derive-free, one-`assert_eq!` property is gone.
* Cipher, KDF and digest types **never** appear in public signatures. Outputs
  are newtypes over `[u8; N]` (`Commitment`, `SealedBytes`); backends own their
  error enums; `CardError` stays crypto-free.
* Every crypto feature must build on `thumbv7em-none-eabihf` and
  `wasm32-unknown-unknown` with `--no-default-features` plus that feature —
  the proof that the backend is no_std and (for AEAD) allocation-free on the
  hot path.
* Never enable `chacha20poly1305/rand_core` — it is `rand_core 0.6`; cardpack
  is on `rand 0.10`. Nonces come from the caller's `rand::RngCore`.

# Citations

[1] [docs/EPIC-04_Sealed_Decks.md](/references/epic-04-sealed-decks.md)
[2] [std-io is excluded from full](/decisions/std-io-outside-full.md)
[3] [Cargo.toml](../../Cargo.toml)
[4] [deny.toml](../../deny.toml)
