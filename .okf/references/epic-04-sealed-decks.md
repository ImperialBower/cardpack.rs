---
type: Reference
title: EPIC-04 — Sealed Decks (family)
description: A deck cardpack cannot read because it never holds one — the Ordinal/Codebook bijection, Permutation-as-data, a non-generic SlotPile of card names, Revealed<D> as the only value map, and a five-item Seal<D> adapter — plus three children for commit-reveal shuffles, holder-key AEAD seals with a plain Custody ledger, and the mental-poker bridge to pkmental/pkcore.
tags: [epic, seal, crypto, ordinal, permutation, active]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-04_Sealed_Decks.md
timestamp: 2026-08-24T12:00:00Z
---

# What it covers

The umbrella ships a **dependency-free kernel**: `Ordinal` and `Codebook<D>`
(a total `Card ↔ 0..V` bijection per deck, over the deduplicated vocabulary),
versioned canonical pile bytes (`CANON_V1`), `Permutation` (a shuffle as data,
defined to agree with `Pile::shuffle_with_rng`), `Pile::permute`/`cut`, and a
new `src/seal/` module built on one rule of cardpack's own — *the kernel holds
a card's slot, its order, and its value once revealed; never ciphertext, never
a scheme parameter*: `SlotId`, a **non-generic**
`SlotPile(Vec<SlotId>)` that shuffles, cuts and deals blind and derives
everything, `Revealed<D>` as the **only** slot → card map (with `reveal` and a
verified `reveal_with`), a five-item `Seal<D>` adapter that no container is
generic over, a `SlotAudit` that counts but cannot prove distinctness, and a
`PlaintextSeal` test double behind `seal-test-double`. Reshaped the same day it
was drafted; the first draft carried generic containers.

Children:

* **EPIC-04a Commit–Reveal Shuffle** (`commit-reveal` → `sha2`) — provably-fair
  shuffling: commit-all-then-reveal, SHA-256 counter-mode permutation
  derivation frozen by golden vectors, blind commitments to a pile order.
* **EPIC-04b Holder-Key Seal** (`seal-aead` → `chacha20poly1305`/`hkdf`/
  `sha2`/`zeroize`) — the first real `Seal<D>` backend and the one
  dealer-custody shape: per-card HKDF keys, a 42-byte `SealedBytes`, a plain
  `Custody(Vec<(SlotId, SealedBytes)>)` ledger beside a `SlotPile`, dealer vs
  verifier mode, one token reveals one card through `Revealed::reveal_with`;
  a public-key `RecipientSeal` is designed, not built.
* **EPIC-04c Mental Poker Bridge (`_spec`)** — the surface cardpack promises
  to any protocol crate that builds on it (bijection, byte layouts,
  `SlotPile` + `Revealed<D>`, the `Seal<D>` shape, `seal_roundtrip`), a worked
  example of a threshold-ElGamal backend on that surface (using `pkmental`'s
  real types as illustration, not dependency), and the short list of things a
  consumer must decide for itself (name your bijection, token plurality,
  verify inside `unseal`). Written from cardpack outward; no work items for
  other repositories.

# Authoritative for

* **Slots, not custody** (EPIC-04 decision 2): no kernel type is generic over
  a scheme and none holds ciphertext — `SlotPile` + `Revealed<D>` are the
  referee state. Five reasons, all cardpack's own.
* **The `Seal<D>` signature** (decision 3): `seal`/`unseal` take the `SlotId`;
  `seal` takes `&mut dyn RngCore`; `SlotId` is `u16`. Convergent with a sibling
  repo's design sketch, not a compatibility goal.
* **cardpack is its own boss**: designed to be built *on*; it links to nothing
  and tracks no other repository's work.
* **The `CANON_V1` byte layout** and the rule that a shipped deck's
  `base_vec()` order is a semver contract from 0.11.0.
* **The `Permutation` convention** `out[i] = in[p[i]]`.
* **No `seal` feature** — the boundary is always on; only backends are gated
  ([crypto decision](/decisions/crypto-features-outside-full.md)).

# Status

Designed 2026-08-24 at `1c14440`. **The umbrella's kernel (Stories 0–7) landed
the same day on branch `crypt`** — 320 lib tests, 8 mutation-checked property
tests, every purity gate green, version 0.11.0. 04a, 04b, 04c remain Planned.
Sequencing: 04 → (04a ‖ 04b) → 04c review.
Whether `pkcore` or `pkmental` ever build on it is their call and is not tracked here.

# In-repo paths

`docs/EPIC-04_Sealed_Decks.md`,
`docs/EPIC-04a_Commit_Reveal_Shuffle.md`,
`docs/EPIC-04b_Holder_Key_Seal.md`,
`docs/EPIC-04c_Mental_Poker_Bridge_spec.md`

This concept is a pointer, not a copy — the linked documents are authoritative.
