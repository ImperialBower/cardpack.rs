# EPIC-04: Sealed Decks (SEAL)

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Work story-by-story; "default features green" (`cargo test --all`) **and** the purity gates (`cargo build --no-default-features`, `make no-std`, `cargo deny check bans`) are preconditions for every story — if any goes red mid-story, stop and diagnose before moving on. Nothing in this family has landed: every Status row is honest aspiration as of `main` @ `1c14440`, 2026-08-24.

> **Family.** This is the umbrella. The children are:
> [04a Commit–Reveal Shuffle](./EPIC-04a_Commit_Reveal_Shuffle.md) (provably-fair
> shuffling, `commit-reveal` feature),
> [04b Holder-Key Seal](./EPIC-04b_Holder_Key_Seal.md) (per-card encryption a holder
> opens with one token, `seal-aead` feature), and
> [04c Mental Poker Bridge](./EPIC-04c_Mental_Poker_Bridge_spec.md) (the cross-repo
> contract for `pkmental` / `pkcore`). This document ships the **dependency-free
> kernel** all three build on. Sequencing: 04 → (04a ‖ 04b) → 04c review.

**Goal:** Give cardpack a deck it **cannot read**. Add a canonical **`Ordinal`** bijection per deck, a **`Permutation`** type so a shuffle is data that can be stored, inverted and verified, and a **`Seal<D>`** boundary with **`SealedCard`** / **`SealedPile`** so shuffling, cutting and dealing happen *blind* — a card's rank and suit exist only after someone presents a token. This is the substrate for distributed-game security in several strengths: commit–reveal fairness (04a), holder-only readability (04b), and full mental poker (04c), each as a pluggable backend rather than one blessed answer.

**Architecture:** Three additive layers on the existing `basic` engine, none of them requiring a new dependency. (1) `Ordinal` / `Codebook<D>` and `Permutation` land in `src/basic/types/` as plain value types, always on, `no_std` + `alloc`. (2) A new top-level `src/seal/` module declares the `Seal<D>` trait and the two sealed containers; it is generic over the *scheme* and never holds a key. (3) Real backends live in `src/seal/commit/` (04a) and `src/seal/aead/` (04b) behind opt-in features that are deliberately **not** in `full` — the same posture as `std-io` ([`.okf/decisions/std-io-outside-full.md`](../.okf/decisions/std-io-outside-full.md)). The `Seal<D>` shape **mirrors** the `CardSeal` trait designed for `pkcore` in `pkcore/docs/epics/EPIC-79b_Sealed_Deck.md`, with three recorded divergences, so one backend can implement both.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std + alloc discipline, `rand` 0.10 (`RngCore` for the seal seam, `Rng` for blind shuffles — `std_rng` is already unconditional, `Cargo.toml:41-46`), `proptest` for seeded properties, GitHub Actions CI with clippy-pedantic, kernel-purity, no_std, thumb and wasm32 jobs.

---

## Context

The crate today is structurally incapable of holding a card it does not know.

- `BasicCard { suit: Pip, rank: Pip }` (`src/basic/types/basic_card.rs:44`) is transparent data. `Card<DeckType>` (`src/basic/types/card.rs:34`) wraps it with a `PhantomData` brand and derives `Copy`, `Debug`, `Display`, `Ord`, and (under `serde`) `Serialize`. Every one of those reads the value.
- `Pile<DeckType>(Vec<Card<DeckType>>)` (`src/basic/types/pile.rs:72`) is an ordered multiset of readable cards. Its shuffle API is already the right *shape* — `shuffle_with_rng<R: Rng + ?Sized>` (`pile.rs:796`) and `shuffle_with_seed(u64)` (`pile.rs:776`) — but the thing being shuffled is plaintext. The permutation the shuffle applied is never materialised: you cannot store it, invert it, or hand it to a verifier.
- The only integer encoding of a card is the Cactus-Kev bitfield behind `CKCRevised` (`src/basic/types/traits.rs:253`, `src/common/utils.rs:3`). It is a sparse 32-bit value for poker evaluators, not a dense `0..N` index, and it only makes sense for French-suited decks. There is no canonical, deck-relative "card number 17".
- `DeckKind` (`src/basic/decks/registry.rs`) enumerates all 14 shipped decks; `all()` at `:88`. `DeckedBase::base_vec()` (`traits.rs:30`) is each deck's card list in a fixed order — an implicit bijection nobody has promised to keep stable.
- There is **no** cryptography, commitment, or hashing of any kind in `src/`, `tests/`, `docs/`, or `.okf/`. `Hash` is the std derive for map keys; the only "crypto" string in the repo is `crypto.getRandomValues` as a wasm entropy source (`tests/wasm.rs:53`).

Two sibling repositories have already felt this gap:

- **`pkmental`** (`../pkmental`) implements Barnett–Smart threshold ElGamal mental poker. The *only* thing it needs from a card library is a total `Card ↔ 0..52` bijection — and it has to build one itself, at runtime, with two `OnceLock<HashMap>`s over `pkcore::deck::DECK_ARRAY` and a panicking `expect` (`pkmental/src/encode.rs:33-60`). It does not depend on cardpack directly; cardpack reaches it only transitively via `pkcore`.
- **`pkcore`** (`../pkcore`) has a complete, unbuilt design for exactly this boundary — `pkcore/docs/epics/EPIC-79b_Sealed_Deck.md`: `trait CardSeal { type Sealed; type Token; type Error; seal; unseal }`, `SlotId`, `SealedCard<S>` with a redacting `Debug` and no `Display`, `SealedDeck<S>` as a `Vec` (never a set), an `audit` that can count slots but cannot prove distinctness, and a `PlaintextSeal` test double. As of `pkcore` `677e0d15` no `src/seal/` exists there; the doc is design only. `pkcore` pins `cardpack = "0.6.9"` and uses its own `u32 Card`, so nothing here can be adopted there without a version bump on their side.

This EPIC does not fork that design. It **ports it to the generic deck kernel** — where "a deck" means any of 14 vocabularies, from Tiny (4 cards) to Dashavatara (120) to `French::decks(4)` (216) — and records where the port had to diverge (Design decision 3).

**What this EPIC does NOT do:**

- **No cryptography ships in this document.** Not one hash, not one cipher, not one new dependency. `cargo build --no-default-features`, `make no-std`, the thumb target, and `cargo deny check bans` are hard exit criteria. Real backends are 04a and 04b.
- **No keys live in the kernel.** `SealedCard` and `SealedPile` are generic over a scheme *type* `S`, never over an *instance*. There is no field anywhere in the struct graph from which a plaintext could be derived.
- **No multi-party protocol, no threshold keys, no zero-knowledge proofs, no transport.** That is `pkmental`'s job; 04c is the contract, not an implementation.
- **No change to `Card`, `BasicCard`, `Pile`, or `Decked`'s required methods.** `Pile` gains two methods (`permute`, `cut`); `Decked` gains one *default* method (`codebook()`). Nothing existing changes shape.
- **No funky.** `BuffoonCard`/`BuffoonPile` are a separate type family; a sealed Balatro deck would be its own EPIC.
- **No plaintext zeroization.** `Card: Copy` (`card.rs:32`) means revealed cards are copied freely onto the stack. Key zeroization is 04b's concern; plaintext hygiene is explicitly out of scope and stated in the docs.

---

## Status

Status as of `main` @ `1c14440`, **2026-08-24**. Nothing has landed.

| Component | Status |
|---|---|
| `Ordinal` newtype + `vocabulary()` | Planned |
| `Codebook<D>` — `ordinal` / `card` bijection, `Decked::codebook()` | Planned |
| Canonical pile bytes (`CANON_V1`, `encode_pile` / `decode_pile`) | Planned |
| `Permutation` — validated, invertible, composable, canonical bytes | Planned |
| `Pile::permute` / `Pile::cut` | Planned |
| `Seal<D>` trait + `SlotId` | Planned |
| `SealedCard<D, S>` + redacting `Debug`, serde bounds | Planned |
| `SealedPile<D, S>` — blind shuffle / permute / cut / draw / take / reveal | Planned |
| `SealAudit` — cardinality + slot uniqueness only | Planned |
| `PlaintextSeal` test double behind `seal-test-double` | Planned |
| `seal_roundtrip` conformance helper (exported under `seal-test-double`) | Planned |
| `CardError` variants (8, ungated, `#[non_exhaustive]`) | Planned |
| `tests/seal_properties.rs` | Planned |
| Docs / CHANGELOG / prelude / 0.11.0 / `.okf/` bundle | Planned |
| Commit–reveal backend | → [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md) |
| Holder-key AEAD backend | → [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) |
| Mental-poker bridge contract | → [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) |

---

## Goals

- A **canonical ordinal** for every card in every deck — total, stable, dense `0..V` — so a card can be hashed, encoded to a curve point, or encrypted as two bytes without any backend building its own lookup table.
- A **shuffle as data.** `Permutation` can be stored, sent, inverted, composed, and re-applied, so "the dealer shuffled fairly" becomes a checkable claim instead of a trust assumption.
- A **sealed card** whose rank and suit are not recoverable from its bytes, its `Debug`, or its serialized form.
- A **sealed pile** that can be shuffled, cut, and dealt by code with **no key and no knowledge** — because every one of those operations is a permutation of labels, and permuting labels needs no knowledge.
- A **narrow reveal seam**: exactly one method turns a sealed card into a `Card<D>`, and it requires a scheme *and* a token supplied by the caller.
- **Zero new dependencies** in this document. The domain kernel stays pure ([`.okf/architecture/domain-kernel.md`](../.okf/architecture/domain-kernel.md)).
- A **backend slot** shaped so `pkmental`'s ElGamal masking, 04b's AEAD, or a future public-key scheme drops in without the kernel changing a line — and shaped close enough to `pkcore`'s `CardSeal` that one backend serves both crates.

## Scope

The rules the new types must obey:

1. `Codebook<D>::ordinal` and `::card` are mutually inverse over the deck's **vocabulary** (its distinct cards). Order is `base_vec()` first-occurrence order. From 0.11.0 on, reordering a shipped deck's `base_vec()` is a **semver-major** change; a golden test pins Standard52.
2. `Card::default()` (the blank card) and any card not in `D::base_vec()` have **no** ordinal — `None`, never a sentinel.
3. `Permutation` is validated on construction: every constructor either yields a bijection over `0..n` or returns `Err`. `apply`, `inverse`, and `then` obey the group laws and are property-tested.
4. `SealedPile` never exposes a `Card<D>`. Not by accessor, iterator, `Deref`, `Index`, `Display`, `Debug`, or `Serialize`. The only door is `reveal`, which takes `(&scheme, &token)`.
5. A wrong token is an `Err`, never a silent wrong card.
6. `SealedPile` is an ordered `Vec`, **not** a set. Set semantics require reading values.
7. `SealedPile::audit` can count and can detect duplicate `SlotId`s. It **cannot** check that the sealed payloads are distinct cards — that is a shuffle-argument property and belongs to a backend (04c). The limit is documented in the doc comment, not hidden.
8. Nothing in `src/basic/types/ordinal.rs`, `permutation.rs`, or `src/seal/` (outside `commit/` and `aead/`) may pull in a dependency that fails `make no-std` or `cargo deny check bans`.
9. `PlaintextSeal` must be impossible to reach in a default build (`#[cfg(any(test, feature = "seal-test-double"))]`).
10. All additions are additive. `CardError` variants ride `#[non_exhaustive]` (`src/common/errors.rs:13`). Version bump: 0.11.0 minor.

---

## Domain

The kata's three layers for this slice.

**Things.**

| Thing | Type | What it is |
|---|---|---|
| The canonical number of a card in its deck | `Ordinal` | dense `0..V`, deck-relative |
| The deck's vocabulary, indexed | `Codebook<D>` | `base_vec()` deduplicated; the bijection, held |
| A shuffle written down | `Permutation` | a bijection over `0..n`, as data |
| "Which card is that?" without knowing *what* it is | `SlotId` | an arbitrary public label |
| The lock-and-key scheme | `trait Seal<D>` | the caller's; the crate never constructs one |
| Permission to turn one card over | `Seal::Token` | presented by the caller |
| A face-down card nobody has read | `SealedCard<D, S>` | `(S::Sealed, SlotId)` |
| The shoe of face-down cards | `SealedPile<D, S>` | ordered `Vec`, never a set |
| The plaintext card | `Card<D>` (`card.rs:34`) | ✅ exists |

**Business Requirements.** (a) *Identifiable* — every card in every deck has exactly one number, and every number in range has exactly one card. (b) *Replayable* — a shuffle can be recorded and re-applied by anyone, and undone. (c) *Blind* — a sealed pile is a first-class deck (shuffle, cut, draw, deal) for code that holds no key. (d) *Honest* — the sealed types never *appear* to check more than they can; a wrong token fails loudly; the audit says what it cannot see.

**Business Logic.** The bijection law (`card(ordinal(c)) == c`) satisfies (a); the permutation group laws and `from_rng ≡ shuffle_with_rng` satisfy (b); "every blind operation is a permutation of `SlotId`s" satisfies (c); the redacting `Debug`, `Err`-on-wrong-token, and `SealAudit`'s documented limit satisfy (d). Each is driven out by a test that fails without it.

---

## Design decisions (settled)

1. **No `seal` feature.** The boundary traits and types are dependency-free, `alloc`-only, and always on. The house rule ([`.okf/architecture/feature-flags.md`](../.okf/architecture/feature-flags.md) "Principle") is that features gate *what a dependency or `std` costs*, not surface area. A dep-free gate would add a `cfg` dimension to `CardError`, `Pile::permute`, the prelude, and every doctest for nothing. The only new features are `seal-test-double = []` (a test double, no dep), `commit-reveal` (04a), `seal-aead` (04b), and the `crypto` umbrella over those two. **None of them is in `full`** — see the new [`.okf/decisions/crypto-features-outside-full.md`](../.okf/decisions/crypto-features-outside-full.md).

2. **`Seal<D>` is generic over the deck type, not an associated type.** `impl<D: DeckedBase> Seal<D> for HolderKeySeal<D>` is one impl for all 14 decks; an associated `type Deck` would force one impl per deck and make a deck-agnostic backend impossible to write.

3. **Three deliberate divergences from `pkcore`'s `CardSeal`** (`pkcore/docs/epics/EPIC-79b_Sealed_Deck.md` §Design), each recorded in 04c's divergence register:
   - (a) `seal` and `unseal` take the **`SlotId`**. An AEAD backend needs it as associated data — binding payload to slot is what defeats "swap two sealed cards" attacks — and an ElGamal backend can simply ignore it.
   - (b) `seal` takes **`&mut dyn RngCore`**. Every real backend is randomized (AEAD nonces, ElGamal masking). `pkcore`'s `&self`-only `seal` forces interior-mutability RNGs on implementors; passing the RNG is the honest signature. `dyn` rather than a generic keeps the trait object-safe and the backend's monomorphization footprint flat.
   - (c) **`SlotId(u16)`, not `u8`.** `French::decks(4)` is 216 cards; Dashavatara alone is 120.

   Everything else mirrors 79b verbatim: the three associated types and their bounds, the redacting `Debug` and absent `Display`, `Vec`-not-set, audit-cannot-prove-distinctness, the test double's gating. A five-line shim bridges the two (04c).

4. **Ordinal = index into the deck's *vocabulary***, i.e. `base_vec()` with duplicates removed in first-occurrence order — **not** the raw `base_vec()` slot. Pinochle lists `ACE_SPADES` twice (`src/basic/decks/pinochle.rs:31-32`); mental poker encodes plaintext *values*, so both copies must map to the same group element. `vocabulary()` is a free function over `&[BasicCard]` so `DeckKind::all()` sweeps can pin every shipped deck without generics.

5. **Canonical bytes are deck-relative and versioned.** Layout `CANON_V1`: `[0x01][u16 BE name_len][deck_name utf-8][u16 BE count][u16 BE ordinal]*`. Encoding follows *iteration order* (index 0 first) — deliberately independent of the "top = front" convention flagged at `basic_card.rs:38` (`TODO RF`), so a future top-of-deck refactor cannot silently change hashes.

6. **`Permutation` is `Vec<u16>` with the convention `out[i] = in[p[i]]`.** The field is private so every value is validated. `from_rng` is *defined* as `slice::shuffle` applied to the identity, which is exactly what `Pile::shuffle_with_rng` (`pile.rs:796`) does to the cards — so `perm.apply(deck) == deck.shuffled_with_rng(same rng state)` holds by construction and is pinned by a test. `u16` keeps the canonical byte form trivial; `Vec<usize>` was rejected as platform-sized.

7. **Backends own their error enums; `CardError` stays crypto-free.** Same reasoning that keeps `serde_norway::Error` boxed (`src/common/errors.rs:38-44`): `CardError` is `Eq + PartialEq` and alloc-only, and a cipher error type in a public enum would violate kernel Invariant 2. `PlaintextSeal::Error = CardError`; `SealedPile::reveal` wraps the two sources in `SealError<E>`.

8. **`SealedPile::draw(n) -> Option<Self>`**, all-or-nothing, mirroring `Pile::draw` (`pile.rs:227`) rather than `pkcore`'s `Result`. Same crate, same contract.

9. **`seal_shuffled` is the primary constructor; `seal_pile` carries a warning.** Sealing `Standard52::deck()` in order with slots `0..52` publishes the deck (slot == ordinal). The hazard is pinned by a test so nobody "simplifies" it away.

10. **`Sealed: Eq` is for containers and parity, not for meaning.** Under any randomized scheme two seals of A♠ are unequal ciphertexts. The trait doc says so, and it is exactly why `audit` cannot check distinctness (decision 7 of Scope).

---

## Design

### `Ordinal` and `Codebook<D>` — the bijection

`src/basic/types/ordinal.rs` (new), always on:

```rust
/// Canonical index of a card within its deck's vocabulary: `0..V`.
///
/// Deck-relative. `Ordinal(0)` in `Standard52` and `Ordinal(0)` in `Skat`
/// are different cards. Stable from 0.11.0: reordering a shipped deck's
/// `base_vec()` is a semver-major change.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ordinal(u16);

impl Ordinal {
    pub const fn new(i: u16) -> Self;
    pub const fn get(self) -> u16;
    pub const fn index(self) -> usize;
}
impl Display for Ordinal { /* the bare number */ }

/// The deck's vocabulary — `base_vec()` with duplicates removed in
/// first-occurrence order — held as an indexable table. Build once, keep it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Codebook<D: DeckedBase> {
    cards: Vec<BasicCard>,
    deck: PhantomData<D>,
}

impl<D: DeckedBase> Codebook<D> {
    #[must_use] pub fn new() -> Self;                       // vocabulary(&D::base_vec())
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    /// `None` for the blank card and for any card not in the deck.
    pub fn ordinal(&self, card: &Card<D>) -> Option<Ordinal>;
    /// `None` when `ord >= len()`.
    pub fn card(&self, ord: Ordinal) -> Option<Card<D>>;
    pub fn iter(&self) -> impl Iterator<Item = (Ordinal, Card<D>)> + '_;

    /// `CANON_V1` bytes for a pile, in iteration order. Errors on a foreign card.
    pub fn encode_pile(&self, pile: &Pile<D>) -> Result<Vec<u8>, CardError>
    where D: Default + Ord + Copy + Hash;
    pub fn decode_pile(&self, bytes: &[u8]) -> Result<Pile<D>, CardError>
    where D: Default + Ord + Copy + Hash;
}

/// Deduplicate in first-occurrence order. Non-generic so a `DeckKind::all()`
/// sweep can cover every shipped deck.
pub fn vocabulary(cards: &[BasicCard]) -> Vec<BasicCard>;   // itertools::unique

pub const CANON_V1: u8 = 1;
```

`src/basic/types/traits.rs:58` — `Decked` gains one **default** method, `fn codebook() -> Codebook<Self>`. Additive; no implementor changes.

**Why a held struct and not a method on `Card`.** `pkmental`'s two `OnceLock<HashMap>`s (`pkmental/src/encode.rs:38-60`) are the `std` answer; there is no `Sync` static cache in `core`. A `Codebook` you build once and pass around is the pure equivalent, and a linear scan over ≤ 120 entries is not worth a map. `Card::<D>::ordinal()` as a convenience was rejected: it would allocate `base_vec()` per call and hide an O(V) cost behind an innocent name. Putting `ordinal` on the `Decked` trait as a *required* method was rejected because 14 decks implement it; a default method costs nothing.

### `Permutation` — a shuffle as data

`src/basic/types/permutation.rs` (new), always on:

```rust
/// A bijection over `0..len`, as data. Convention: `out[i] = items[p[i]]`.
/// The field is private: every constructor validates.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Permutation(Vec<u16>);

impl Permutation {
    pub fn identity(n: usize) -> Result<Self, CardError>;            // n > u16::MAX → InvalidPermutation
    pub fn try_from_vec(v: Vec<u16>) -> Result<Self, CardError>;     // O(n) bitset check
    /// Fisher–Yates on the identity — by construction identical to what
    /// `Pile::shuffle_with_rng` does to the cards.
    pub fn from_rng<R: Rng + ?Sized>(n: usize, rng: &mut R) -> Result<Self, CardError>;
    /// `StdRng::seed_from_u64`; same cross-`rand`-major caveat as
    /// `Pile::shuffle_with_seed` (`pile.rs:761-766`).
    pub fn from_seed(n: usize, seed: u64) -> Result<Self, CardError>;
    /// The cut: `out = in[at..] ++ in[..at]`.
    pub fn rotation(n: usize, at: usize) -> Result<Self, CardError>;

    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn is_identity(&self) -> bool;
    pub fn as_slice(&self) -> &[u16];

    pub fn apply<T: Clone>(&self, items: &[T]) -> Result<Vec<T>, CardError>;  // PermutationLength on mismatch
    #[must_use] pub fn inverse(&self) -> Self;
    /// `(a.then(b)).apply(x) == b.apply(a.apply(x))`.
    pub fn then(&self, next: &Self) -> Result<Self, CardError>;

    pub fn canonical_bytes(&self) -> Vec<u8>;                          // [u16 BE len][u16 BE]*
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, CardError>;
}
```

`src/basic/types/pile.rs` additions (same bounds as the impl block at `pile.rs:76`):

```rust
pub fn permute(&self, p: &Permutation) -> Result<Self, CardError>;
/// Blind cut. `Err(InvalidCut(at))` if `at > len()`.
pub fn cut(&mut self, at: usize) -> Result<(), CardError>;
```

`Pile` has `get`, `position`, `remove`, `same`, and `unique_cards` today but **no `cut`** — the cut is new for plaintext and sealed piles alike, and both are defined as `rotation`.

### `Seal<D>` — the scheme, owned by the caller

`src/seal/seal.rs` (new), always on:

```rust
/// A card-sealing scheme. cardpack defines the shape; the *caller* provides
/// the implementation, the keys, and the tokens. The crate never constructs
/// an `S` on its own behalf and never stores one inside a pile.
///
/// Mirrors `pkcore`'s `CardSeal` (EPIC-79b) with three divergences: the slot
/// is passed to `seal`/`unseal`, `seal` takes an RNG, and `SlotId` is `u16`.
pub trait Seal<D: DeckedBase> {
    /// The opaque payload. The backend picks the representation: 42 bytes of
    /// AEAD output, an ElGamal ciphertext, or (in tests) a `Card<D>`.
    /// `Eq` is for containers and parity only — under any randomized scheme,
    /// two seals of the same card are unequal.
    type Sealed: Clone + Eq + core::fmt::Debug;
    /// What a caller presents to open exactly one sealed card.
    type Token;
    /// Scheme-specific failure. Associated, so cardpack never names a crypto type.
    type Error: core::error::Error + Send + Sync + 'static;

    /// Lock a plaintext card into `slot`. Called by whoever *has* the key —
    /// never by cardpack itself.
    fn seal(&self, card: Card<D>, slot: SlotId, rng: &mut dyn RngCore)
        -> Result<Self::Sealed, Self::Error>;

    /// Open one sealed payload with a token. The only door in the wall.
    /// A wrong token, wrong slot, or wrong context is `Err` — never a wrong card.
    fn unseal(&self, sealed: &Self::Sealed, slot: SlotId, token: &Self::Token)
        -> Result<Card<D>, Self::Error>;
}
```

**Why `seal` is on the trait when the kernel's sealed types never call it in anger:** so that a single `impl` is the complete, reviewable statement of a scheme, and so the round-trip law `unseal(seal(c, s, rng), s, t) == c` is one generic test (`seal_roundtrip`) every backend runs through — 04b's AEAD, 04c's ElGamal, and the test double.

### `SlotId` — identity without knowledge

`src/seal/slot.rs` (new):

```rust
/// A stable, public handle for one card in a sealed pile.
///
/// Assigned at seal time and carried by the card thereafter, so shuffling
/// permutes *order* while every card keeps its name. This is what lets a
/// ledger say "seat 3 revealed slot 17" without saying what slot 17 is.
///
/// Deliberately NOT the ordinal — that would be the card.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlotId(u16);
impl SlotId { pub const fn new(n: u16) -> Self; pub const fn get(self) -> u16; }
```

### `SealedCard<D, S>` — one card nobody has read

`src/seal/sealed_card.rs` (new):

```rust
pub struct SealedCard<D: DeckedBase, S: Seal<D>> {
    sealed: S::Sealed,
    slot: SlotId,
    deck: PhantomData<D>,
}

impl<D: DeckedBase, S: Seal<D>> SealedCard<D, S> {
    pub fn new(sealed: S::Sealed, slot: SlotId) -> Self;
    /// Public identity. Safe to log, safe to send to a spectator.
    pub fn slot(&self) -> SlotId;
    /// The opaque payload, for transport. Reading it yields nothing.
    pub fn payload(&self) -> &S::Sealed;
    /// The one and only door. Passes `self.slot` to the scheme.
    pub fn reveal(&self, scheme: &S, token: &S::Token) -> Result<Card<D>, S::Error>;
}

// Hand-written, not derived — a derive would add `D: Clone` etc. via PhantomData.
impl<D, S> Clone for SealedCard<D, S> { … }
impl<D, S> PartialEq / Eq for SealedCard<D, S> { … }   // slot + payload
impl<D, S> core::fmt::Debug for SealedCard<D, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SealedCard {{ slot: {}, sealed: <sealed> }}", self.slot.get())
    }
}
// NO `Display`. There is no user-facing rendering of a card nobody has read.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(
    serialize   = "S::Sealed: Serialize",
    deserialize = "S::Sealed: Deserialize<'de>")))]
```

Note what `SealedCard` does **not** hold: an `S`. The pile is generic over the *scheme*, never over an *instance* of it. That is the mechanical expression of "the library does not know" — there is no key anywhere in the struct graph, so there is no code path, safe or unsafe, that turns a `SealedCard` into a `Card` without the caller handing both pieces in.

A derived `Debug` would print `S::Sealed`, and for the test double `S::Sealed` *is* a `Card<D>`. That is the single easiest way to leak a deck into a log line, so the redaction gets its own test.

### `SealedPile<D, S>` — the blind shoe

`src/seal/sealed_pile.rs` (new):

```rust
#[derive(Default)]
pub struct SealedPile<D: DeckedBase, S: Seal<D>>(Vec<SealedCard<D, S>>);

impl<D: DeckedBase, S: Seal<D>> SealedPile<D, S> {
    /// Build from pre-sealed cards. Rejects duplicate `SlotId`s.
    pub fn from_sealed(cards: Vec<SealedCard<D, S>>) -> Result<Self, CardError>;

    /// Seals in pile order with slots `0..n`. **Hazard:** sealing an unshuffled
    /// deck this way makes slot == ordinal — the deck is public. Prefer
    /// `seal_shuffled`. (The hazard is pinned by a test.)
    pub fn seal_pile(scheme: &S, pile: &Pile<D>, rng: &mut dyn RngCore)
        -> Result<Self, S::Error> where D: Default + Ord + Copy + Hash;
    /// Shuffles a clone with `rng` first, then seals. The recommended constructor.
    pub fn seal_shuffled<R: Rng + ?Sized>(scheme: &S, pile: &Pile<D>, rng: &mut R)
        -> Result<Self, S::Error> where D: Default + Ord + Copy + Hash;

    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    /// Every slot still in the shoe. Public, leaks nothing.
    pub fn slots(&self) -> impl Iterator<Item = SlotId> + '_;
    pub fn get(&self, slot: SlotId) -> Option<&SealedCard<D, S>>;
    /// Remove by label. Needs no knowledge.
    pub fn take(&mut self, slot: SlotId) -> Option<SealedCard<D, S>>;

    pub fn draw_first(&mut self) -> Option<SealedCard<D, S>>;
    /// All-or-nothing, mirroring `Pile::draw` (`pile.rs:227`).
    pub fn draw(&mut self, n: usize) -> Option<Self>;

    /// Blind Fisher–Yates. Mirrors `Pile::shuffle_with_rng` (`pile.rs:796`).
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R);
    pub fn shuffle_with_seed(&mut self, seed: u64);
    pub fn permute(&self, p: &Permutation) -> Result<Self, CardError>;
    pub fn cut(&mut self, at: usize) -> Result<(), CardError>;

    /// Reveal one slot. `SealError::Slot` if the slot is absent,
    /// `SealError::Backend` if the scheme refuses.
    pub fn reveal(&self, slot: SlotId, scheme: &S, token: &S::Token)
        -> Result<Card<D>, SealError<S::Error>>;

    /// Counts cards and checks `SlotId` uniqueness. It does **not** and
    /// cannot check that the payloads are distinct cards — see Scope 7.
    pub fn audit(&self, expected: usize) -> SealAudit;
}

#[derive(Debug)]
pub enum SealError<E> { Slot(CardError), Backend(E) }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SealAudit { pub expected: usize, pub actual: usize, pub duplicate_slots: Vec<SlotId> }
impl SealAudit { pub fn is_ok(&self) -> bool; }
```

**Methods deliberately absent**, each because it would require knowledge: `sort` (ordering by rank is knowledge); `contains(&Card)` / `position(&Card)` (matching by value is knowledge); `Deref`, `Index`, `Display`, and any `IntoIterator` yielding something evaluable.

**Why a `Vec` and not a set.** A set dedups by *value*. Deduping requires reading. A sealed pile is an ordered list whose invariants are maintained over `SlotId`, not over cards.

### `PlaintextSeal` — the test double, hard to reach on purpose

`src/seal/plaintext.rs` (new), `#[cfg(any(test, feature = "seal-test-double"))]`:

```rust
/// **NO SECURITY WHATSOEVER.** `Sealed = Card<D>`; "sealing" is the identity
/// function and `unseal` checks `token.0 == self.secret`. It exists to test
/// the *plumbing* — draw, shuffle, cut, reveal accounting, redaction — not
/// secrecy. Never reachable in a default build.
pub struct PlaintextSeal { secret: u64 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainToken(pub u64);

impl<D: DeckedBase> Seal<D> for PlaintextSeal {
    type Sealed = Card<D>;
    type Token = PlainToken;
    type Error = CardError;
    …
}

/// Generic conformance helper, exported under `seal-test-double` so backends
/// in other crates (04c) can run the same law.
pub fn seal_roundtrip<D, S: Seal<D>>(scheme: &S, token_for: impl Fn(SlotId) -> S::Token, rng: &mut dyn RngCore)
where D: Decked<D> + Default + Ord + Copy + Hash;
```

`Sealed = Card<D>` is chosen *because* its derived `Debug` prints the card — that is what makes `sealed_card__debug_never_prints_a_card` a real test rather than a tautology. `Token = ()` was rejected: "wrong token ⇒ `Err`" would be untestable on the double.

### `CardError` additions

`src/common/errors.rs:13`, all ungated, `String`/`usize`/`u16` payloads only (kernel Invariant 2, and `CardError` keeps `Eq`):

```rust
#[error("Invalid ordinal: `{0}` is out of range for this deck")]   InvalidOrdinal(u16),
#[error("Card `{0}` is not in this deck")]                            CardNotInDeck(String),
#[error("Invalid permutation: {0}")]                                  InvalidPermutation(String),
#[error("Permutation length `{expected}` does not match `{actual}` items")]
                                                                      PermutationLength { expected: usize, actual: usize },
#[error("Cannot cut at `{0}`: out of range")]                          InvalidCut(usize),
#[error("Malformed canonical bytes: {0}")]                            CanonicalMalformed(String),
#[error("Duplicate slot `{0}` in sealed pile")]                       DuplicateSlot(u16),
#[error("Slot `{0}` not found in sealed pile")]                       SlotNotFound(u16),
```

### Module layout

```
src/basic/types/ordinal.rs        Ordinal, Codebook<D>, vocabulary(), CANON_V1, encode/decode_pile
src/basic/types/permutation.rs    Permutation
src/basic/types/pile.rs           + permute, cut
src/basic/types/traits.rs         + Decked::codebook() default method
src/seal/mod.rs                   "no keys live here" header; re-exports
src/seal/seal.rs                  trait Seal<D>
src/seal/slot.rs                  SlotId
src/seal/sealed_card.rs           SealedCard<D, S>, redacting Debug
src/seal/sealed_pile.rs           SealedPile<D, S>, SealAudit, SealError<E>
src/seal/plaintext.rs             PlaintextSeal, seal_roundtrip  (cfg test | seal-test-double)
src/seal/commit/                  EPIC-04a  (cfg commit-reveal)
src/seal/aead/                    EPIC-04b  (cfg seal-aead)
src/lib.rs:358                    + pub mod seal;
src/prelude.rs                    + Ordinal, Codebook, Permutation, SlotId, Seal, SealedCard, SealedPile, SealAudit, SealError
tests/seal_properties.rs          proptest suite (cfg not wasm32), pattern from tests/properties.rs
```

### Feature flags / `Cargo.toml`

```toml
# Test double for the seal boundary. Off by default; adds no dependency.
seal-test-double = []
# Umbrella for the real crypto backends (EPIC-04a, EPIC-04b). Deliberately
# NOT in `full` — see .okf/decisions/crypto-features-outside-full.md.
crypto = ["commit-reveal", "seal-aead"]
```

No new dependencies in this document. Nothing here implies `std`; `alloc` is already unconditional.

---

## Story 0: Prerequisites & gating (`src/lib.rs`, `Cargo.toml`, `src/common/errors.rs`)

**Acceptance:** an empty `seal` module and the new features exist; the eight `CardError` variants compile ungated; every purity gate is green before any type is written.

**Files:**
- Modify: `src/lib.rs:358` (add `pub mod seal;`)
- Create: `src/seal/mod.rs` (header comment + empty)
- Modify: `Cargo.toml:24-35` (`seal-test-double`, `crypto` features)
- Modify: `src/common/errors.rs:13` (eight variants)

### Tasks

- [ ] Add `pub mod seal;` and the two features; confirm `cargo build --no-default-features` is green with the empty module
- [ ] Add the eight `CardError` variants; confirm `CardError` keeps `Eq + PartialEq` (test `card_error__seal_variants_display`)
- [ ] `make no-std`, `make no-std-thumbv7`, `cargo deny check bans` green
- [ ] `cargo test --all` green

---

## Story 1: `Ordinal` + `Codebook<D>` (`src/basic/types/ordinal.rs`)

**Acceptance:** `card(ordinal(c)) == c` for every card of every shipped deck; the blank card and a foreign card yield `None`; Pinochle's vocabulary is 24, not 48; Standard52's ordinal table is pinned.

**Files:**
- Create: `src/basic/types/ordinal.rs`
- Modify: `src/basic/types.rs` (declare module)
- Modify: `src/basic/types/traits.rs:58` (`Decked::codebook()` default)
- Modify: `src/prelude.rs` (re-exports)

### Tasks

- [ ] `Ordinal` newtype with `new`/`get`/`index`/`Display` and the serde-gated derive
- [ ] `vocabulary(&[BasicCard])` via `itertools::unique` (test `vocabulary__dedups_first_occurrence` on Pinochle: 48 in, 24 out, order preserved)
- [ ] `Codebook<D>::new/len/ordinal/card/iter` (test `codebook__roundtrip_every_shipped_deck` — a macro over the 14 marker types **and** a `DeckKind::all()` sweep via `vocabulary(&kind.base_vec())`, so deck 15 cannot dodge it)
- [ ] `codebook__blank_has_no_ordinal` (`Card::<French>::default()` → `None`; `Ordinal(V)` → `None`)
- [ ] `codebook__standard52_golden` — read `Standard52::DECK` (`src/basic/decks/standard52.rs:90`), pin the full table; from here on a reorder is a breaking change
- [ ] `Decked::codebook()` default method (doctest on `French`)
- [ ] `cargo test --no-default-features --lib` green

---

## Story 2: Canonical bytes (`src/basic/types/ordinal.rs`)

**Acceptance:** `decode_pile(encode_pile(p)) == p` for a full deck, a shuffled deck, a 5-card hand, and an empty pile; the header bytes for Standard52 are pinned; version, truncation, and deck-name mismatches each error with a named variant.

**Files:**
- Modify: `src/basic/types/ordinal.rs`

### Tasks

- [ ] `CANON_V1` + `encode_pile` (iteration order; `CardNotInDeck` on a foreign card)
- [ ] `decode_pile` with strict length checks (`CanonicalMalformed` on trailing or missing bytes)
- [ ] Tests: `canonical__roundtrip`, `canonical__golden_standard52_prefix` (`01 00 0B "Standard 52" 00 34 …`), `canonical__bad_version`, `canonical__truncated`, `canonical__wrong_deck_name` (Standard52 bytes decoded through `Codebook<Skat>` → `CanonicalMalformed`)

---

## Story 3: `Permutation` + `Pile::permute` / `Pile::cut` (`src/basic/types/permutation.rs`, `pile.rs`)

**Acceptance:** every constructor validates; `apply`/`inverse`/`then` obey the group laws under `proptest`; `from_rng` reproduces `Pile::shuffle_with_rng` bit-for-bit; `rotation` is the cut; canonical bytes round-trip.

**Files:**
- Create: `src/basic/types/permutation.rs`
- Modify: `src/basic/types/pile.rs` (`permute`, `cut`)
- Modify: `src/prelude.rs`

### Tasks

- [ ] Constructors + validation (`permutation__rejects_duplicate`, `permutation__rejects_out_of_range`, `permutation__identity_too_large_errors`)
- [ ] `apply` / `inverse` / `then` (`permutation__inverse_roundtrip`, `permutation__compose_law`, `permutation__apply_length_mismatch_errors`)
- [ ] `from_rng` ≡ `shuffle_with_rng` (`permutation__from_rng_matches_pile_shuffle`: `Permutation::from_rng(52, &mut StdRng::seed_from_u64(s)).apply(deck.cards()) == deck.shuffled_with_seed(s).cards()`)
- [ ] `rotation` (`permutation__rotation_is_cut`)
- [ ] `canonical_bytes` / `from_canonical_bytes` (`permutation__canonical_roundtrip`, `permutation__canonical_rejects_invalid`)
- [ ] `Pile::permute` / `Pile::cut` (`pile__permute_preserves_multiset` via `same`, `pile__cut_preserves_multiset`, `pile__cut_past_end_errors`, `pile__cut_at_len_is_identity`)

---

## Story 4: The seal boundary (`src/seal/{seal,slot,sealed_card,plaintext}.rs`)

**Acceptance:** `Seal<D>` compiles object-safe; `SealedCard`'s `Debug` never prints a card; `reveal` round-trips under the double and a wrong token is `Err`; serde round-trips a `SealedCard<French, PlaintextSeal>` under `serde,seal-test-double`.

**Files:**
- Create: `src/seal/seal.rs`, `src/seal/slot.rs`, `src/seal/sealed_card.rs`, `src/seal/plaintext.rs`
- Modify: `src/seal/mod.rs`, `src/prelude.rs`

### Tasks

- [ ] `Seal<D>` trait + `SlotId` (compile-time test that `&dyn Seal<French, Sealed = …>` is nameable — object safety)
- [ ] `SealedCard` with hand-written `Clone`/`PartialEq`/`Eq`/`Debug` and the serde `bound` attribute
- [ ] `PlaintextSeal` + `PlainToken` + `seal_roundtrip` helper
- [ ] Tests: `sealed_card__debug_never_prints_a_card` (`format!("{:?}")` of a sealed A♠ contains `<sealed>` and none of `A♠`, `AS`, `Ace`, `Spades`), `sealed_card__reveal_roundtrip`, `sealed_card__wrong_token_errors` (asserts `Err`, and separately that it is never `Ok(other_card)`), `seal__roundtrip_law` (first caller of the generic helper), `sealed_card__serde_roundtrip`
- [ ] Confirm `PlaintextSeal` is absent from `cargo doc --no-default-features` output

---

## Story 5: `SealedPile<D, S>` (`src/seal/sealed_pile.rs`)

**Acceptance:** construction rejects duplicate slots; the slot == ordinal hazard is pinned; blind shuffle/permute/cut are permutations of the slot set and agree with their plaintext counterparts after reveal-all; `draw` is all-or-nothing; `audit` counts and cannot prove distinctness.

**Files:**
- Create: `src/seal/sealed_pile.rs`
- Modify: `src/seal/mod.rs`, `src/prelude.rs`

### Tasks

- [ ] `from_sealed` / `seal_pile` / `seal_shuffled` (`sealed_pile__from_sealed_rejects_duplicate_slots`, `sealed_pile__seal_pile_of_sorted_deck_leaks_slot_eq_ordinal`, `sealed_pile__seal_shuffled_breaks_slot_ordinal_identity`)
- [ ] `get` / `take` / `draw_first` / `draw` (`sealed_pile__draw_all_or_nothing`, `sealed_pile__take_by_slot`)
- [ ] `shuffle_with_rng` / `shuffle_with_seed` / `permute` / `cut` (`sealed_pile__shuffle_permutes_slot_set`, `sealed_pile__shuffle_deterministic_for_seed`, `sealed_pile__permute_matches_plaintext_permute` — seal, permute both sides, reveal all, compare; `sealed_pile__cut_matches_plaintext_cut`)
- [ ] `reveal` + `SealError` (`sealed_pile__reveal_unknown_slot_errors`, `sealed_pile__reveal_wrong_token_is_backend_error`)
- [ ] `audit` + `SealAudit` (`sealed_pile__audit_counts_but_cannot_prove_distinctness` — the same card sealed into two slots passes `audit(52).is_ok()`; the doc comment says why)

---

## Story 6: Property tests (`tests/seal_properties.rs`)

**Acceptance:** the group laws, the blind-shuffle-is-a-permutation claim, and the plaintext/sealed agreement hold for arbitrary seeds, reproducible from a failing seed.

**Files:**
- Create: `tests/seal_properties.rs` (header conventions from `tests/properties.rs:1-20`; `#![cfg(not(target_arch = "wasm32"))]`)

### Tasks

- [ ] `permutation__inverse_roundtrip` (seed), `permutation__compose_law` (two seeds), `permutation__from_rng_matches_pile_shuffle` (seed)
- [ ] `sealed_pile__shuffle_permutes_slot_set` (seed), `sealed_pile__shuffle_deterministic_for_seed` (seed), `sealed_pile__permute_matches_plaintext_permute` (seed)
- [ ] `cargo test --features seal-test-double --test seal_properties` green

---

## Story 7: Docs, prelude, release (`README.md`, `CHANGELOG.md`, `Cargo.toml`, `.okf/`)

**Acceptance:** every new public item is re-exported from the prelude and documented with an ungated doctest; README's feature table lists the new features; CHANGELOG `Added`; version `0.11.0`; `.okf/` reflects the shipped state.

### Tasks

- [ ] Prelude re-exports; doctests use `Permutation` / `Codebook` (ungated), never `PlaintextSeal`
- [ ] README feature rows (`seal-test-double`, `crypto`) with the "not in `full`" note
- [ ] CHANGELOG `Added` + `Cargo.toml` `0.11.0`
- [ ] `.okf/architecture/feature-flags.md` rows flipped from *planned* to live; `.okf/references/epic-04-sealed-decks.md` tags `planned` → `active`; `.okf/log.md` entry; `/okf:validate .okf --strict`
- [ ] Flip this document's Status rows; cross-link 04a/04b/04c from BACKLOG

---

## Test Plan

| Test | Asserts |
|---|---|
| `codebook__roundtrip_every_shipped_deck` | ∀ deck, ∀ card in vocabulary: `card(ordinal(c)) == c`, and `len == unique_cards().len()` — driven off the registry so it extends to deck 15 |
| `codebook__blank_has_no_ordinal` | `Card::<French>::default()` → `None`; `Ordinal(V)` → `None` |
| `codebook__standard52_golden` | The fixed ordinal table; makes `base_vec()` order a contract |
| `vocabulary__dedups_first_occurrence` | Pinochle 48 → 24, first occurrence wins |
| `canonical__roundtrip` / `canonical__golden_standard52_prefix` | Byte layout is stable and self-describing |
| `canonical__wrong_deck_name` | A deck cannot be decoded through another deck's codebook |
| `permutation__from_rng_matches_pile_shuffle` | `Permutation` and `Pile::shuffle_with_rng` are the same Fisher–Yates |
| `permutation__inverse_roundtrip` (prop) | `p.inverse().apply(p.apply(x)) == x` |
| `permutation__compose_law` (prop) | `a.then(b).apply(x) == b.apply(a.apply(x))` |
| `permutation__rejects_*` | Every invalid construction is `Err(InvalidPermutation)` / `Err(PermutationLength)` |
| `sealed_card__debug_never_prints_a_card` | The redaction — the single easiest leak, pinned |
| `sealed_card__wrong_token_errors` | `Err`, and never `Ok(other_card)` |
| `seal__roundtrip_law` | The generic law every backend (04b, 04c) reuses |
| `sealed_pile__seal_pile_of_sorted_deck_leaks_slot_eq_ordinal` | The documented hazard is real — so the warning cannot be "simplified" away |
| `sealed_pile__shuffle_permutes_slot_set` (prop) | `BTreeSet<SlotId>` before == after |
| `sealed_pile__permute_matches_plaintext_permute` (prop) | Reveal-all of a permuted sealed pile equals the permuted plaintext pile |
| `sealed_pile__audit_counts_but_cannot_prove_distinctness` | The audit's honesty: a duplicated card still passes |
| `sealed_card__serde_roundtrip` | The serde `bound` attribute is right |

**Gold Standard check:** before closing, delete each guard in turn — the `try_from_vec` bitset check, the `from_sealed` duplicate-slot check, the hand-written `Debug`, the token comparison in `PlaintextSeal::unseal` — and confirm a named test goes red.

## Key Files

| File | Role |
|---|---|
| `src/basic/types/ordinal.rs` | **New.** `Ordinal`, `Codebook<D>`, `vocabulary`, `CANON_V1`, canonical bytes |
| `src/basic/types/permutation.rs` | **New.** `Permutation` |
| `src/basic/types/pile.rs:72` | `Pile::permute`, `Pile::cut` |
| `src/basic/types/traits.rs:58` | `Decked::codebook()` default method |
| `src/seal/mod.rs` | **New.** Module header, re-exports |
| `src/seal/seal.rs` | **New.** `trait Seal<D>` |
| `src/seal/slot.rs` | **New.** `SlotId` |
| `src/seal/sealed_card.rs` | **New.** `SealedCard<D, S>` |
| `src/seal/sealed_pile.rs` | **New.** `SealedPile<D, S>`, `SealAudit`, `SealError<E>` |
| `src/seal/plaintext.rs` | **New.** `PlaintextSeal`, `seal_roundtrip` (gated) |
| `src/common/errors.rs:13` | Eight new variants |
| `src/lib.rs:358`, `src/prelude.rs` | Module declaration, re-exports |
| `Cargo.toml:24-35` | `seal-test-double`, `crypto` features |
| `tests/seal_properties.rs` | **New.** Property suite |
| `.okf/decisions/crypto-features-outside-full.md` | **New.** Why backends stay out of `full` |
| `.okf/references/epic-04-sealed-decks.md` | **New.** Mirror pointer |

## Reuse (do NOT recreate)

- `src/basic/types/pile.rs:796` `shuffle_with_rng` / `:776` `shuffle_with_seed` — the blind shuffle mirrors these exactly; `Permutation::from_rng` is *defined* to agree with them.
- `src/basic/types/pile.rs:227` `draw` — the all-or-nothing contract `SealedPile::draw` copies.
- `src/basic/types/pile.rs:733` `same` and `:443` `unique_cards` — multiset assertions in tests.
- `src/basic/types/traits.rs:30` `DeckedBase::base_vec` / `deck_name` — the **sole** source of the vocabulary and the domain tag. Do not add a parallel card list.
- `src/basic/decks/registry.rs:88` `DeckKind::all()` — every-deck sweeps.
- `src/common/errors.rs:13` `CardError` (`#[non_exhaustive]`) — extend; never introduce a second kernel error enum.
- `tests/properties.rs` — the proptest header, seeding idiom, and `name__property` naming.
- `itertools::unique` — already a dependency with `use_alloc`.
- `pkcore/docs/epics/EPIC-79b_Sealed_Deck.md` — the design being ported; read it before changing any signature in `src/seal/`.

## Compatibility

- **Preserves:** every existing public signature. `Card`, `BasicCard`, `Pile`, `Decked`'s required methods, and all 14 decks are untouched. A `default = []` build gains the new value types and no dependencies.
- **Adds:** `Ordinal`, `Codebook<D>`, `vocabulary`, `CANON_V1`, `Permutation`, `Pile::permute`/`cut`, `Decked::codebook()` (default), the `seal` module, eight `CardError` variants, two features.
- **Breaks:** nothing intended. `CardError` is already `#[non_exhaustive]`. **New contract:** the order of every shipped deck's `base_vec()` is now stable; reordering is semver-major from 0.11.0.
- **Package size:** unchanged in kind — only `src/**` ships (`Cargo.toml:13`).

## Dependencies

- **Blocks:** [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md) (needs Stories 2–3), [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) (needs Stories 1, 4–5), [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) (needs everything).
- **Built on:** the seeded-shuffle work (`docs/2026-04-29-seeded-shuffle-design.md`); the `DeckKind` registry (EPIC-02); the `#[non_exhaustive]` precedent on `CardError` (EPIC-03); the domain-kernel invariants (`docs/audit-2026-07-18-domain-kernel.md`).
- **Related:** `pkcore` EPIC-79b (the design ported here), `pkmental` EPIC-79 (the consumer whose `encode.rs` `Codebook` replaces).

## Verification

```bash
# Purity gates — the kernel must stay dependency-free
cargo build --no-default-features
make no-std
make no-std-thumbv7
cargo deny check bans
cargo test --no-default-features --lib

# The seal boundary with its test double
cargo test --features seal-test-double
cargo test --features seal-test-double,serde
cargo test --features full,seal-test-double --test seal_properties

# Full matrix
cargo test --all-features
cargo test --all-features --doc
cargo clippy --all-features --all-targets -- -D warnings -D clippy::pedantic
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo doc --no-default-features --no-deps && ! grep -rq PlaintextSeal target/doc/cardpack/

# Portability
cargo build --target wasm32-unknown-unknown --all-features
make ayce
```

Exit criteria:

1. `Codebook<D>` round-trips every card of every `DeckKind` and every marker type; the Standard52 golden table passes.
2. `Permutation::from_rng(n, rng).apply(&pile) == pile.shuffled_with_rng(rng)` for the same RNG state.
3. `SealedPile` exposes no path to a `Card<D>` other than `reveal` — verified by reading the public API in `cargo doc` output, and by the redaction test.
4. Every negative test matches a named `CardError` variant, not bare `is_err()`.
5. `cargo build --no-default-features`, both bare-metal targets, and `cargo deny check bans` are green: nothing here leaks into the pure kernel.
6. `PlaintextSeal` does not appear in default-build docs.
7. `.okf/` updated and `/okf:validate .okf --strict` is clean.

---

## Gotchas

1. **The slot == ordinal leak is the whole game.** `seal_pile(Standard52::deck())` with slots `0..52` is a public deck wearing a costume. `seal_shuffled` exists for this reason, the hazard has a named test, and the `seal_pile` doc comment must say so in its first line. Do not remove the warning to tidy the docs.

2. **`Sealed: Eq` compares ciphertexts, not cards.** Under any scheme worth using, sealing is randomized, so `Eq` proves nothing about distinctness. It is on the trait for containers and parity with `pkcore`. This is also why `audit` cannot do more than count — anyone "improving" `audit` to compare payloads is checking nothing.

3. **`Card: Copy` means plaintext is never zeroized.** A revealed card is copied onto the stack, into a `Vec`, into a log line. Key hygiene is 04b's job; plaintext hygiene is out of scope and must be stated in the `seal` module docs so nobody assumes otherwise.

4. **Derives on generic sealed types pull in phantom bounds.** `#[derive(Clone)]` on `SealedCard<D, S>` demands `D: Clone` and `S: Clone` through `PhantomData`. Hand-write `Clone`/`PartialEq`/`Eq`/`Debug`, and use `#[serde(bound(...))]` — the same trap `docs/generic-decks.md` records for `Pile`.

5. **One `Permutation` convention, stated once.** `out[i] = in[p[i]]` must be used identically in `apply`, `inverse`, `rotation`, `SealedPile::permute`, and 04a's derivation. The compose-law and `from_rng` tests are the only guard; if either goes red after a "simplification", the convention drifted.

6. **`u16` is a real limit.** `Permutation::identity(70_000)` errs, and so does a `Codebook` for a hypothetical deck with more than 65 535 distinct cards. Document it; do not widen to `u32` "just in case" — the canonical byte formats are frozen at `u16`.

7. **The `seal` module is unconditional, so it must build on thumb.** Anything it `use`s has to be `core`/`alloc`/`rand` without `std`. `rand::RngCore` is fine — `std_rng` is already unconditional ([`.okf/decisions/rand-std-rng-unconditional.md`](../.okf/decisions/rand-std-rng-unconditional.md)). `std::collections::HashMap` is not.

8. **Doctests must stay flag-free.** `PlaintextSeal` is gated, so seal doctests should demonstrate `Permutation` and `Codebook` (ungated) or be written against 04b's `HolderKeySeal` under `ignore` with a comment — the standing rule in [`.okf/architecture/feature-flags.md`](../.okf/architecture/feature-flags.md) is *prefer the ungated API first*.

9. **`DeckKind::all()` is 13 without `yaml`** (`Razz` is gated). Registry sweeps read `DeckKind::all().len()`, never a literal 14.

10. **Iteration order vs top-of-deck.** `basic_card.rs:38` carries a `TODO RF` to flip the deck so the *end* of the vector is the top. Canonical bytes and `Permutation` are defined over *iteration order* precisely so that refactor changes nothing here. If it ever lands, re-run the golden tests and expect them to pass.
