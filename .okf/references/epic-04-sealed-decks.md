---
type: Reference
title: EPIC-04 — Sealed Decks (family)
description: A deck cardpack cannot read — the Ordinal/Codebook bijection, Permutation-as-data, and the Seal<D> boundary with SealedCard/SealedPile — plus three children for commit-reveal shuffles, holder-key AEAD seals, and the mental-poker bridge to pkmental/pkcore.
tags: [epic, seal, crypto, ordinal, permutation, planned]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-04_Sealed_Decks.md
timestamp: 2026-08-24T12:00:00Z
---

# What it covers

The umbrella ships a **dependency-free kernel**: `Ordinal` and `Codebook<D>`
(a total `Card ↔ 0..V` bijection per deck, over the deduplicated vocabulary),
versioned canonical pile bytes (`CANON_V1`), `Permutation` (a shuffle as data,
defined to agree with `Pile::shuffle_with_rng`), `Pile::permute`/`cut`, and a
new `src/seal/` module — `trait Seal<D>`, `SlotId`, `SealedCard<D, S>` with a
redacting `Debug`, `SealedPile<D, S>` that shuffles, cuts and deals blind, a
`SealAudit` that counts but cannot prove distinctness, and a `PlaintextSeal`
test double behind `seal-test-double`.

Children:

* **EPIC-04a Commit–Reveal Shuffle** (`commit-reveal` → `sha2`) — provably-fair
  shuffling: commit-all-then-reveal, SHA-256 counter-mode permutation
  derivation frozen by golden vectors, blind commitments to a pile order.
* **EPIC-04b Holder-Key Seal** (`seal-aead` → `chacha20poly1305`/`hkdf`/
  `sha2`/`zeroize`) — the first real `Seal<D>` backend: per-card HKDF keys, a
  42-byte `SealedBytes`, dealer vs verifier mode, one token reveals one card;
  a public-key `RecipientSeal` is designed, not built.
* **EPIC-04c Mental Poker Bridge (`_spec`)** — the cross-repo contract: what
  cardpack promises, how `pkmental`'s Barnett–Smart `CardCrypto` maps onto
  `Seal<D>` (verification *inside* `unseal`), the five-line shim to `pkcore`'s
  `CardSeal`, and the divergence register.

# Authoritative for

* **The three divergences from pkcore EPIC-79b's `CardSeal`**: `seal`/`unseal`
  take the `SlotId`; `seal` takes `&mut dyn RngCore`; `SlotId` is `u16`.
* **The `CANON_V1` byte layout** and the rule that a shipped deck's
  `base_vec()` order is a semver contract from 0.11.0.
* **The `Permutation` convention** `out[i] = in[p[i]]`.
* **No `seal` feature** — the boundary is always on; only backends are gated
  ([crypto decision](/decisions/crypto-features-outside-full.md)).

# Status

Designed 2026-08-24 at `1c14440`. **Nothing has landed.** Every Status row in
all four documents reads Planned. Sequencing: 04 → (04a ‖ 04b) → 04c review.
Adoption by `pkcore`/`pkmental` is blocked on pkcore's `cardpack = "0.6.9"` pin.

# In-repo paths

`docs/EPIC-04_Sealed_Decks.md`,
`docs/EPIC-04a_Commit_Reveal_Shuffle.md`,
`docs/EPIC-04b_Holder_Key_Seal.md`,
`docs/EPIC-04c_Mental_Poker_Bridge_spec.md`

This concept is a pointer, not a copy — the linked documents are authoritative.
