# EPIC-04c: Mental Poker Bridge (MPB) — what cardpack promises a protocol consumer

> **For agentic workers:** This is a **`_spec`** — the stability contract cardpack offers to any crate that builds a hidden-card protocol on top of it. It is written **from cardpack outward**: cardpack owns this contract, and consumers build on it. Nothing here obliges cardpack to another repository's design, and nothing here is a work item for another repository. **No code lands from this document** beyond what [EPIC-04](./EPIC-04_Sealed_Decks.md) already ships. **Reviewed against the shipped code on branch `crypt`, 2026-08-25** — three corrections, listed under "Review findings" at the end; the promises table below is the corrected one.

**Goal:** State, in one place, the **surface a mental-poker or other hidden-card protocol can rely on** — the bijection, the byte encodings, the shuffle-as-data, the slot-based referee state, and the five-item `Seal<D>` adapter — with the stability rule for each. Then show, with one worked example, how a real threshold-ElGamal backend would sit on that surface. The example uses `pkmental` (`../pkmental`), which today builds its own card ↔ integer table at runtime; it is an illustration of the shape, not a dependency in either direction.

**Architecture:** Two tables and a short list. (1) **cardpack promises** — every item a protocol consumer may depend on, with its stability class. (2) **A worked consumer** — how an l-out-of-l threshold-ElGamal scheme maps onto `Seal<D>` and onto the `SlotPile` + `Revealed<D>` referee state. (3) **What a consumer must handle itself** — the three things cardpack deliberately does not decide (bijection naming, token plurality, verification placement).

**Tech Stack:** This document only. Protocol crates are expected to be `std` (curves, signatures, transport); cardpack stays `no_std` + `alloc` and never gains a curve.

---

## Context

**What cardpack ships after EPIC-04** (all landed on branch `crypt`, 2026-08-24/25, with 04a and 04b): `Ordinal` / `Codebook<D>` (a total `Card ↔ 0..V` bijection per deck, over the deduplicated vocabulary), `CANON_V1` canonical pile bytes, `Permutation` with canonical bytes, `SlotId`, the non-generic `SlotPile`, `Revealed<D>` with `reveal` / `reveal_with`, the `Seal<D>` adapter, and the `seal_roundtrip` conformance helper. Its design rule (EPIC-04 decision 2): **the kernel holds a card's slot, its order, and its value once revealed — never ciphertext, never a scheme type parameter.**

**The consumer this document uses as its worked example — `pkmental`** (`../pkmental`, `main` @ `ac72bc1`): Barnett–Smart threshold ElGamal over Pallas, Sako–Kilian cut-and-choose shuffle proofs, DLEQ reveal tokens, an Ed25519-signed hash-chained event log. The *players* hold and shuffle the masked deck with proofs; cards travel only as curve points (`pkmental/src/wire.rs:11`). The single thing it needs from a card library is a total `Card ↔ 0..52` bijection, which it currently builds itself: `point_for_index(i) = G · (i + 1)` (`pkmental/src/encode.rs:33-35`) over `OnceLock<HashMap>` tables scanned from `pkcore::deck::DECK_ARRAY` (`encode.rs:21`, `:38-40`) — so its bijection is **pkcore's array order**, not cardpack's `Codebook` order, which is exactly the case §3.1 warns about. Its crypto seam is `trait CardCrypto` (`pkmental/src/crypto/mod.rs:52`): `MaskedCard: Clone + Eq` (`:60`), `RevealToken: Clone` (`:63`, the untrusted wire form), `VerifiedToken: Clone` (`:70`, mintable only via `verify_reveal_token`), and `MpError` (`:34`, `Clone + Debug + Eq + thiserror::Error`, `&'static str`/`usize` payloads, with `StillMasked` `:38` and `BadProof(&'static str)` `:41`). `pkmental` does not depend on cardpack today.

**Prior art, for the record.** `pkcore` (`../pkcore`, branch `table_decelled` @ `f4bb1f9a` — the state to rely on) has no sealed-card code: no `src/seal/`, no slot type, no seal trait. Its `docs/epics/EPIC-79b_Sealed_Deck.md` is a design sketch of a `CardSeal` trait with the same five-item shape as cardpack's `Seal<D>` — associated `Sealed`/`Token`/`Error`, `seal`, `unseal`. A separate branch built it out as a spike and its author is redoing that work; nothing on it is a fact this document depends on. `pkcore` pins `cardpack = "0.6.9"` and uses its own `u32 Card`. If `pkcore` ever builds a hidden-card table on cardpack, this document is the surface it would build on — and that is the only direction the dependency runs.

**What this spec does NOT do:**

- No `impl Seal<D>` for anything ElGamal lands in cardpack. cardpack has no arkworks, no curves, no `std` requirement, and will not gain them.
- No work items for other repositories. What `pkmental` or `pkcore` do with this surface is theirs to decide and track.
- No shuffle proof, no threshold decryption, no transport. Those are protocol concerns.

---

## Status

Status as of branch `crypt`, **2026-08-25**.

| Component | Status |
|---|---|
| cardpack promises table (this doc) | Complete — checked row by row against the code |
| Worked consumer example (this doc) | Complete — cites re-verified against `pkmental` `main` @ `ac72bc1` |
| "What a consumer must handle" list (this doc) | Complete |
| `seal_roundtrip` exported under `seal-test-double` (EPIC-04 Story 5) | Complete |
| Doc-test that a `std` crate over `Standard52` can satisfy the helper's bounds | Complete — `src/seal/plaintext.rs`, on `seal_roundtrip` |

---

## Goals

- **One bijection, owned by the card library.** A protocol never builds its own card ↔ integer table; `Codebook::ordinal` is total, `Option`-returning, and pinned by a golden test.
- **A referee state with nothing to leak.** `SlotPile` + `Revealed<D>` is the whole of what a referee or spectator holds; a protocol feeds it positions and, at reveal, values.
- **One adapter shape.** A backend that must be *checked* at reveal time implements the five-item `Seal<D>`; cardpack runs the same conformance law against every implementation.
- **Stability stated per item.** Each promise says what would count as breaking it.

## Scope

1. Everything in the promises table is a **stability contract** from 0.11.0. Breaking a *frozen* row is semver-major; an *invariant* row never changes.
2. The worked example is **illustrative**. It uses `pkmental`'s real types so the mapping is concrete, but cardpack does not track or drive that work.
3. The "must handle" list is deliberately short. cardpack decides the surface; the protocol decides the protocol.

---

## Domain

**Things.** The bijection (`Codebook<D>`); canonical bytes (`CANON_V1`, `Permutation::canonical_bytes`); the referee state (`SlotPile`, `Revealed<D>`); the adapter (`Seal<D>`); the conformance law (`seal_roundtrip`).

**Business Requirements.** (a) A protocol can encode any card of any deck to a dense integer and back, forever. (b) A protocol can hash a deck order or a permutation and expect any other implementation to hash the same bytes. (c) A referee can hold a whole game's card state without holding a secret. (d) A backend can be checked against one law.

**Business Logic.** (a) is `Codebook` + the golden tests; (b) is the two frozen byte layouts; (c) is `SlotPile` + `Revealed<D>` having no scheme parameter and no ciphertext; (d) is `seal_roundtrip`.

---

## 1. cardpack promises

| Promise | Where | Stability |
|---|---|---|
| `Codebook<D>::ordinal` / `::card` is a total bijection over the deck **vocabulary** (`base_vec()` deduplicated, first-occurrence order) | EPIC-04 Story 1 | Reordering a shipped `base_vec()` is **semver-major** from 0.11.0. `codebook__standard52_golden` pins Standard52. |
| `Card::default()` and foreign cards have no ordinal (`None`) | EPIC-04 Story 1 | Invariant |
| Canonical pile bytes `CANON_V1` — `[0x01][u16 BE name_len][deck_name][u16 BE count][u16 BE ordinal]*`, iteration order | EPIC-04 Story 2 | Frozen; new layouts get a new version byte |
| `Permutation` with `out[i] = in[p[i]]`; `canonical_bytes` = `[u16 BE len][u16 BE]*` | EPIC-04 Story 3 | Frozen; inverse/compose laws tested |
| `Permutation::from_rng` ≡ `Pile::shuffle_with_rng` ≡ `SlotPile::shuffle_with_rng` for the same RNG state | EPIC-04 Stories 3–4 | Pinned by the agreement tests |
| `SlotId(u16)`; `SlotPile` holds **only** slots and no signature in it names `Card<D>` (it names `CardError`, the crate's one error type) | EPIC-04 Story 4 | Invariant |
| `Revealed<D>` is the **only** kernel `SlotId → Card<D>` map; `reveal` (caller vouches) and `reveal_with` (checked via `Seal<D>`) both end there | EPIC-04 Story 5 | Invariant |
| `Seal<D>` shape: `Sealed: Clone + Eq + Debug`, `Token`, `Error: Error + Send + Sync + 'static`, `seal(&self, Card<D>, SlotId, &mut dyn rand::Rng)`, `unseal(&self, &Sealed, SlotId, &Token)`; **no cardpack struct is generic over `S`** (`src/seal/adapter.rs`) | EPIC-04 Story 5 | Additions only via **new** traits; never a new required method. The RNG type is `rand 0.10`'s core trait; a `rand` major bump that renames it is semver-major here too |
| `seal_roundtrip<D, S>` conformance helper under `seal-test-double`, bounds `D: Decked<D> + Default + Ord + Copy + Hash + Debug`, `S: Seal<D>`; `token_for: impl Fn(SlotId) -> S::Token`; `rng: &mut dyn rand::Rng` | EPIC-04 Story 5 | Runs unchanged against any backend; the doc-test on it is a foreign impl over `Standard52` |
| `commit-reveal` `v1` preimages — contribution, seed, permutation, pile — and the SHA-256 counter-mode permutation derivation | EPIC-04a (`src/seal/commit/mod.rs` table) | Frozen; golden vectors from an independent Python reference in `tests/commit_reveal.rs` |
| `seal-aead` `v1` — `K_slot = HKDF-SHA256(master, salt = deck_name, info = tag ‖ slot)`, `AD = tag ‖ name_len ‖ deck_name ‖ slot ‖ context`, plaintext = ordinal `u16` BE, XChaCha20-Poly1305, `SealedBytes = nonce(24) ‖ ct(2) ‖ tag(16)` | EPIC-04b (`src/seal/aead/mod.rs` table) | Frozen; golden vectors from Python in `tests/seal_aead.rs` |
| `HolderKeySeal<D>` is the **reference `Seal<D>` implementation**: dealer/verifier modes, one blunt `Unseal` error, tokens minted per slot, `Custody` a plain `Vec<(SlotId, SealedBytes)>` beside a `SlotPile` | EPIC-04b | The pattern a backend author copies; its API is semver-minor-stable like the rest |

## 2. A worked consumer — threshold ElGamal on the surface

**2a. The referee needs no ciphertext.** A referee or spectator folding a protocol's event log needs exactly: the deck as an ordered list of slots (`SlotPile`), the permutation each verified shuffle applied (`Permutation` — and the protocol's shuffle-proof transcript should hash `Permutation::canonical_bytes` so every implementation hashes the same thing), and a map from slot to value once enough shares have been verified (`Revealed<D>::reveal` — "caller vouches," because the protocol already verified). No ciphertext enters cardpack on this path, which is the common one.

**2b. When the referee wants to check a reveal itself**, the backend implements `Seal<D>` and the referee calls `Revealed::reveal_with`. For an l-out-of-l threshold-ElGamal scheme like `pkmental`'s, holding **public** material only (`AggregateKey`, every `PublicKey`, the binding context, a `Codebook<D>`):

| `Seal<D>` item | ElGamal construct (pkmental names) | Notes |
|---|---|---|
| `Sealed` | `MaskedCard` (`pkmental/src/crypto/mod.rs:60`) | `Clone + Eq` already; `Debug` may need a wrapper |
| `Token` | `Vec<RevealToken>` — **unverified**, one share per player | Not `VerifiedToken`: whoever calls `reveal_with` cannot mint one. An associated type is free to be a collection |
| `Error` | `MpError` (`mod.rs:34`) | `Send + Sync + 'static` holds — payloads are `&'static str`/`usize` |
| `seal(card, slot, rng)` | `codebook.ordinal(&card)` → point `G · (i + 1)` → `mask(agg_key, point, rng)` | This is what replaces `encode.rs:33-60`. `slot` ignored. The RNG is passed in — the scheme instance needs no interior mutability |
| `unseal(sealed, slot, token)` | ∀ shares: `verify_reveal_token(pk_i, sealed, t_i)` → `VerifiedToken`; fold; `unmask` → point → `codebook.card(ordinal)` | **Verification inside `unseal`.** Fewer than *l* shares → `Err(StillMasked)`; a bad proof → `Err(BadProof)`. `slot` ignored |

Conformance: `seal_roundtrip::<Standard52, ElGamalSeal<_>>` runs unchanged. Tests a backend author would want beyond it — a bad DLEQ proof is rejected; `l − 1` shares stay masked; the new bijection reproduces the old table's points — are the backend's to write.

## 3. What a consumer must handle itself

1. **Name your bijection in your transcript.** cardpack's `Codebook` order is `base_vec()` first-occurrence order. Other libraries' 52-card arrays are in other orders. A protocol that hashes ordinals must put a domain tag such as `b"cardpack/Standard 52/v1"` in its Fiat–Shamir context, or a transcript replayed against a different bijection decodes to the wrong cards *silently*. cardpack cannot see the other bijection; only the protocol can name it.
2. **Token plurality is yours.** `Seal::Token` can be a `Vec`. cardpack does not know how many shares your scheme needs, cannot tell you which one is missing, and does not want to. `Err(StillMasked)`-style errors are the backend's.
3. **Verify inside `unseal`.** `Revealed::reveal_with` hands your backend raw tokens and admits whatever `unseal` returns. If your backend has a "verified token" newtype, the verification step moves *into* `unseal`; the compile-time guarantee that nobody forgets it is still yours to keep.
4. **`Sealed: Eq` compares ciphertexts, not cards.** Remasking makes equal cards unequal. Do not use it for distinctness; that is what a shuffle proof is for.
5. **`Revealed::reveal` trusts the caller.** It exists because most protocols verify before cardpack ever sees a value. A referee that accepts `reveal` from an untrusted peer has skipped its own protocol.

---

## Work Items

### Story S: Write the surface down and prove the helper's bounds

- [x] **S1.** The promises table, checked line-by-line against the **shipped code** (2026-08-25) — three corrections, see "Review findings"
- [x] **S2.** The worked example, checked against `pkmental/src/crypto/mod.rs:34-70` and `encode.rs:21-40` at `ac72bc1` (illustration only; cites corrected)
- [x] **S3.** A doc-test under `seal-test-double` showing `seal_roundtrip` called from an external harness over `Standard52` with a hand-rolled `Seal` impl (`XorSeal`, no security) — `src/seal/plaintext.rs`; passes under `cargo test --features seal-test-double --doc seal_roundtrip`
- [x] **S4.** Cross-link from `src/seal/mod.rs` ("Notes for a protocol crate building on this surface") — landed with EPIC-04 Story 7

---

## Test Plan

| Test | Asserts |
|---|---|
| `seal_roundtrip__callable_from_external_harness` (doc-test) | The exported helper's bounds are satisfiable by a foreign `Seal<Standard52>` impl |
| (everything else) | EPIC-04's own suite — this document adds no behaviour |

## Key Files

| File | Role |
|---|---|
| `src/seal/{slot,slot_pile,revealed,seal,plaintext}.rs` (EPIC-04) | The surface |
| `src/basic/types/ordinal.rs`, `permutation.rs` (EPIC-04) | The bijection and the shuffle-as-data |
| `src/seal/mod.rs` | Cross-link to this doc |
| `pkmental/src/crypto/mod.rs:52`, `pkmental/src/encode.rs:33-60` | The worked example's source, cited not depended on |

## Reuse (do NOT recreate)

- The promises table *is* the reuse list: a protocol crate reuses `Codebook`, `Permutation`, `SlotPile`, `Revealed<D>`, `Seal<D>`, `seal_roundtrip`. It does not fork a bijection, a slot deck, or a round-trip law.

## Compatibility

- **cardpack:** nothing beyond EPIC-04. This document adds one doc-test and one cross-link.
- **Consumers:** additive on their side; nothing here changes an existing consumer.

## Dependencies

- **Built on:** [EPIC-04](./EPIC-04_Sealed_Decks.md) (all stories).
- **Cites:** [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md) (transcript preimage formats), [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) (the reference `Seal<D>` implementation to pattern-match).
- **Blocks:** nothing in this repo.

## Verification

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features crypto,seal-test-double
cargo test --features seal-test-double --doc -- seal_roundtrip
! grep -rnE 'struct \w+<[^>]*S: Seal' src/seal/
```

Exit criteria:

1. Every row of the promises table names a real item in EPIC-04's sketches, with its stability class.
2. The doc-test proves `seal_roundtrip` is callable from outside the crate over `Standard52`.
3. The document contains no work item for another repository.

---

## Gotchas

1. **Do not let this document become a cross-repo to-do list.** cardpack owns its surface; what consumers do with it is theirs to track. If a consumer's needs change the surface, that is an EPIC-04 corrigendum here, argued on cardpack's terms.
2. **The bijection-naming rule is the one most likely to bite.** It is invisible until two implementations disagree, and then every card is wrong. Put it first in `src/seal/mod.rs`'s consumer notes.
3. **Protocol crates are `std`; cardpack is not.** Nothing in this spec licenses a `std` requirement on cardpack, and no curve, signature, or transport type ever appears in a cardpack signature.

---

## Review findings (2026-08-25)

The promises table was written against EPIC-04's sketches and reviewed
against the code that shipped. Three rows were wrong and are corrected above:

1. **`&mut dyn Rng`, not `&mut dyn RngCore`.** `rand 0.10` folded `RngCore`
   into `Rng`. Same delta as the EPIC-04 corrigendum; the table now says so
   and notes that a future `rand` rename would be semver-major on this seam.
2. **`seal_roundtrip` also requires `D: Debug`** (for its panic messages).
   Every shipped deck satisfies it; a consumer's own deck type must too.
3. **The `SlotPile` "no `Card`" invariant is about `Card<D>`.** The type
   names `CardError` in its `Result`s. The row now says exactly that.

Two rows were added because 04a and 04b landed the same day: the `seal-aead`
`v1` format joins the frozen list, and `HolderKeySeal<D>` is named as the
reference `Seal<D>` implementation. The worked example's `pkmental` line
citations were off by a few lines and are corrected; the one substantive
addition there is that `pkmental`'s bijection is `pkcore::deck::DECK_ARRAY`
order — a live instance of §3.1.

One thing outside this document's scope was fixed so its own verification
command passes: three intra-doc links pointed at feature-gated items from
ungated docs — `Razz` and `YamlDecked` (behind `yaml`, `src/lib.rs`,
pre-existing) and `seal_roundtrip` (behind `seal-test-double`,
`src/seal/adapter.rs`, from EPIC-04) — so `cargo doc -D warnings` failed on
every feature set that lacked them. They are plain text now; `cargo doc`
is clean with `--no-default-features`, `--features crypto,seal-test-double`,
and `--all-features`.
