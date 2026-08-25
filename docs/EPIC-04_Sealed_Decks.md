# EPIC-04: Sealed Decks (SEAL)

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Work story-by-story; "default features green" (`cargo test --all`) **and** the purity gates (`cargo build --no-default-features`, `make no-std`, `cargo deny check bans`) are preconditions for every story — if any goes red mid-story, stop and diagnose before moving on. The umbrella's kernel (Stories 0–7) landed on branch `crypt` on 2026-08-24; 04a landed 2026-08-25; 04b and the 04c review have not started.

> **Family.** This is the umbrella. The children are:
> [04a Commit–Reveal Shuffle](./EPIC-04a_Commit_Reveal_Shuffle.md) (provably-fair
> shuffling, `commit-reveal` feature),
> [04b Holder-Key Seal](./EPIC-04b_Holder_Key_Seal.md) (per-card encryption a holder
> opens with one token, `seal-aead` feature), and
> [04c Mental Poker Bridge](./EPIC-04c_Mental_Poker_Bridge_spec.md) (the cross-repo
> surface cardpack promises to protocol crates). This document ships the **dependency-free
> kernel** all three build on. Sequencing: 04 → (04a ‖ 04b) → 04c review.

> **Reshaped 2026-08-24, same day as drafting.** The first draft carried generic
> `SealedCard<D, S>` / `SealedPile<D, S>` containers. They are gone. The kernel
> now holds a card's *slot*, its *order*, and its *value once revealed* — never
> ciphertext, never a scheme type parameter. The reasons are cardpack's own
> (decision 2); the sibling repositories are cited below only as prior art and
> as possible consumers. **This crate is its own boss.** It is designed to be
> built *on*, not to link to anything.

**Goal:** Give cardpack a deck it **cannot read** — because it never holds one. Add a canonical **`Ordinal`** bijection per deck, a **`Permutation`** type so a shuffle is data that can be stored, inverted and verified, a **`SlotPile`** of card *names* that can be shuffled, cut and dealt with no knowledge at all, and a **`Revealed<D>`** map that is the only place a card value ever appears. A small **`Seal<D>`** trait is the optional adapter through which a reveal can be *verified* against a backend's ciphertext and token — but no cardpack type is generic over the scheme. This is the substrate for distributed-game security in several strengths: commit–reveal fairness (04a), holder-only readability (04b), and full mental poker (04c), each as a pluggable backend rather than one blessed answer.

**Architecture:** Three additive layers on the existing `basic` engine, none requiring a new dependency, and **nothing generic over a scheme.** (1) `Ordinal` / `Codebook<D>` and `Permutation` land in `src/basic/types/` as plain value types, always on, `no_std` + `alloc`. (2) A new top-level `src/seal/` module holds `SlotId`, the non-generic `SlotPile(Vec<SlotId>)`, `Revealed<D>` (a `BTreeMap<SlotId, Card<D>>` with the same bounds as `Pile<D>`), and the `Seal<D>` trait. Every one of them derives `Clone`/`Eq`/`Debug`/`Serialize` cleanly, because nothing is parameterised over `S`. (3) Real backends live in `src/seal/commit/` (04a) and `src/seal/aead/` (04b) behind opt-in features that are deliberately **not** in `full` — the same posture as `std-io` ([`.okf/decisions/std-io-outside-full.md`](../.okf/decisions/std-io-outside-full.md)). Custody of ciphertext, where a deployment needs it, is a plain `Vec<(SlotId, Bytes)>` owned by the dealer (04b), not a kernel type. No cardpack type, trait, or test depends on any other repository.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std + alloc discipline, `rand` 0.10 (`Rng` for blind shuffles, `RngCore` for the seal seam — `std_rng` is already unconditional, `Cargo.toml:41-46`), `proptest` for seeded properties, GitHub Actions CI with clippy-pedantic, kernel-purity, no_std, thumb and wasm32 jobs.

---

## Context

The crate today is structurally incapable of dealing a card it does not know.

- `BasicCard { suit: Pip, rank: Pip }` (`src/basic/types/basic_card.rs:44`) is transparent data. `Card<DeckType>` (`src/basic/types/card.rs:34`) wraps it with a `PhantomData` brand and derives `Copy`, `Debug`, `Display`, `Ord`, and (under `serde`) `Serialize`. Every one of those reads the value.
- `Pile<DeckType>(Vec<Card<DeckType>>)` (`src/basic/types/pile.rs:72`) is an ordered multiset of readable cards. Its shuffle API is already the right *shape* — `shuffle_with_rng<R: Rng + ?Sized>` (`pile.rs:796`) and `shuffle_with_seed(u64)` (`pile.rs:776`) — but the thing being shuffled is plaintext, and the permutation the shuffle applied is never materialised: you cannot store it, invert it, or hand it to a verifier.
- The only integer encoding of a card is the Cactus-Kev bitfield behind `CKCRevised` (`src/basic/types/traits.rs:253`, `src/common/utils.rs:3`). It is a sparse 32-bit value for poker evaluators, not a dense `0..N` index, and it only makes sense for French-suited decks. There is no canonical, deck-relative "card number 17".
- `DeckKind` (`src/basic/decks/registry.rs`) enumerates all 14 shipped decks; `all()` at `:88`. `DeckedBase::base_vec()` (`traits.rs:30`) is each deck's card list in a fixed order — an implicit bijection nobody has promised to keep stable.
- There is **no** cryptography, commitment, or hashing of any kind in `src/`, `tests/`, `docs/`, or `.okf/`. `Hash` is the std derive for map keys; the only "crypto" string in the repo is `crypto.getRandomValues` as a wasm entropy source (`tests/wasm.rs:53`).

Two sibling repositories are worth knowing about — as **possible consumers and prior art**, not as inputs:

- **`pkmental`** (`../pkmental`, `main` @ `ac72bc1`) implements Barnett–Smart threshold ElGamal mental poker. The *only* thing it needs from a card library is a total `Card ↔ 0..52` bijection — and it builds one itself, at runtime, with two `OnceLock<HashMap>`s over a 52-entry array and a panicking `expect` (`pkmental/src/encode.rs:33-60`). The *players* hold and shuffle the masked deck, with proofs. It never wanted a card library to hold ciphertext. It does not depend on cardpack.
- **`pkcore`** (`../pkcore`, branch `table_decelled` @ `f4bb1f9a` — the state to rely on) has no sealed-card code: no `src/seal/`, no slot type, no seal trait; its `docs/epics/EPIC-79b_Sealed_Deck.md` is a design sketch of a five-item `CardSeal` trait. A separate branch built that sketch out as a spike — generic `SealedDeck<S>`, a table parameterised over the scheme, and the `where` bounds and hand-written derives that came with it — and its author is redoing that work because of the complexity. That experience is evidence for decision 2; it is not a design this crate follows. `pkcore` pins `cardpack = "0.6.9"` and uses its own `u32 Card`. If it ever builds a hidden-card table, it would build on this EPIC — the dependency runs only that way.

This EPIC ports to the generic *deck* kernel — where "a deck" means any of 14 vocabularies, from Tiny (4 cards) to Dashavatara (120) to `French::decks(4)` (216) — the three things every hidden-card design needs (a bijection, a slot label, a shuffle as data) and one rule: **nothing in the kernel is generic over a scheme, and nothing in it holds ciphertext.**

**What this EPIC does NOT do:**

- **No cryptography ships in this document.** Not one hash, not one cipher, not one new dependency. `cargo build --no-default-features`, `make no-std`, the thumb target, and `cargo deny check bans` are hard exit criteria. Real backends are 04a and 04b.
- **No ciphertext lives in the kernel.** There is no `SealedCard<D, S>`, no `SealedPile<D, S>`, no type with a scheme parameter. A deployment that must hold sealed payloads (a single trusted server — 04b) keeps them in a plain `Vec<(SlotId, Bytes)>` beside a `SlotPile`. The kernel gives it the labels and the shuffle; it does not give it a home for secrets.
- **No keys live in the kernel.** `Seal<D>` is implemented by the caller; cardpack never constructs or stores one.
- **No multi-party protocol, no threshold keys, no zero-knowledge proofs, no transport.** That is `pkmental`'s job; 04c is the contract, not an implementation.
- **No change to `Card`, `BasicCard`, `Pile`, or `Decked`'s required methods.** `Pile` gains two methods (`permute`, `cut`); `Decked` gains one *default* method (`codebook()`). Nothing existing changes shape.
- **No funky.** `BuffoonCard`/`BuffoonPile` are a separate type family; a slot-dealt Balatro deck would be its own EPIC.
- **No plaintext zeroization.** `Card: Copy` (`card.rs:32`) means revealed cards are copied freely onto the stack. Key zeroization is 04b's concern; plaintext hygiene is explicitly out of scope and stated in the docs.

---

## Status

Status as of branch `crypt`, **2026-08-24** (kernel landed the same day it was designed; commits pending on the user's side, so rows cite the branch and date rather than a hash). 04a (commit–reveal) landed 2026-08-25; 04b and the 04c review have not started.

| Component | Status |
|---|---|
| `Ordinal` newtype + `vocabulary()` | **Complete** |
| `Codebook<D>` — `ordinal` / `card` bijection, `Decked::codebook()` | **Complete** |
| Canonical pile bytes (`CANON_V1`, `encode_pile` / `decode_pile`) | **Complete** |
| `Permutation` — validated, invertible, composable, canonical bytes | **Complete** |
| `Pile::permute` / `Pile::cut` | **Complete** |
| `SlotId` | **Complete** |
| `SlotPile` — non-generic; blind shuffle / permute / cut / draw / take / audit | **Complete** |
| `SlotAudit` — cardinality + slot uniqueness | **Complete** |
| `Revealed<D>` — the only home for a revealed value; `reveal`, `reveal_with` | **Complete** |
| `Seal<D>` trait (adapter only; no container is generic over it) | **Complete** |
| `PlaintextSeal` test double behind `seal-test-double` | **Complete** |
| `seal_roundtrip` conformance helper (exported under `seal-test-double`) | **Complete** |
| `CardError` variants (9, ungated, `#[non_exhaustive]`) | **Complete** |
| `tests/seal_properties.rs` | **Complete** |
| Docs / CHANGELOG / prelude / 0.11.0 / `.okf/` bundle | **Complete** |
| Commit–reveal backend | Complete — [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md), 2026-08-25 |
| Holder-key AEAD backend + dealer custody ledger | → [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) |
| Mental-poker bridge contract | → [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) |

---

## Goals

- A **canonical ordinal** for every card in every deck — total, stable, dense `0..V` — so a card can be hashed, encoded to a curve point, or encrypted as two bytes without any backend building its own lookup table.
- A **shuffle as data.** `Permutation` can be stored, sent, inverted, composed, and re-applied, so "the dealer shuffled fairly" becomes a checkable claim instead of a trust assumption.
- A **deck of names.** `SlotPile` shuffles, cuts, and deals with **no key and no knowledge** — every one of those operations is a permutation of labels, and permuting labels needs no knowledge. It is a plain value: `Clone + Eq + Debug + Serialize` by derive, one `assert_eq!` to prove "a rejected operation changed nothing."
- **One place a value can be.** `Revealed<D>` is the only type in the kernel that maps a slot to a card. If it is empty, no card value exists anywhere in the game state — trivially true by type, asserted anyway.
- A **narrow verified-reveal seam.** `Seal<D>` lets a reveal arrive as `(slot, ciphertext, scheme, token)` and be checked before it enters `Revealed`. The trait has five items and no container depends on it.
- **Zero new dependencies and zero scheme generics** in this document. The domain kernel stays pure ([`.okf/architecture/domain-kernel.md`](../.okf/architecture/domain-kernel.md)) and stays *simple*.

## Scope

The rules the new types must obey:

1. `Codebook<D>::ordinal` and `::card` are mutually inverse over the deck's **vocabulary** (its distinct cards). Order is `base_vec()` first-occurrence order. From 0.11.0 on, reordering a shipped deck's `base_vec()` is a **semver-major** change; a golden test pins Standard52.
2. `Card::default()` (the blank card) and any card not in `D::base_vec()` have **no** ordinal — `None`, never a sentinel.
3. `Permutation` is validated on construction: every constructor either yields a bijection over `0..n` or returns `Err`. `apply`, `inverse`, and `then` obey the group laws and are property-tested.
4. `SlotPile` contains **only `SlotId`s**. It has no payload field, no type parameter, and no method that takes or returns a `Card`.
5. `SlotPile` is an ordered `Vec`, not a set, and its one invariant — slot uniqueness — is enforced on construction.
6. `Revealed<D>` is the **only** kernel type that holds a `Card<D>` keyed by slot. Revealing a slot twice is an error, not a silent overwrite.
7. `Revealed::reveal_with` is the only path from a backend's ciphertext to a `Card<D>`; it takes `(&scheme, &token)` and a wrong token is `Err`, never a wrong card.
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
| The shoe, as names | `SlotPile` | `Vec<SlotId>`; shuffles, cuts, deals blind |
| A card whose value is now known | `Revealed<D>` | `SlotId → Card<D>`, the only such map |
| The lock-and-key scheme, when a reveal must be checked | `trait Seal<D>` | the caller's; cardpack never constructs one |
| The plaintext card | `Card<D>` (`card.rs:34`) | ✅ exists |

**Business Requirements.** (a) *Identifiable* — every card in every deck has exactly one number, and every number in range has exactly one card. (b) *Replayable* — a shuffle can be recorded and re-applied by anyone, and undone. (c) *Blind* — the shoe is a first-class deck (shuffle, cut, draw, deal) for code that holds no key and no value. (d) *Contained* — a card value exists in exactly one kernel type, and only after a reveal. (e) *Honest* — a wrong token fails loudly; an audit says what it cannot see.

**Business Logic.** The bijection law (`card(ordinal(c)) == c`) satisfies (a); the permutation group laws and `from_rng ≡ shuffle_with_rng` satisfy (b); "`SlotPile` has no `Card` in its signature" satisfies (c) by construction; `Revealed<D>` being the sole `SlotId → Card<D>` map satisfies (d); `Err`-on-wrong-token in `reveal_with` and `SlotAudit`'s documented limit satisfy (e). Each is driven out by a test that fails without it.

---

## Design decisions (settled)

1. **No `seal` feature.** The kernel types are dependency-free, `alloc`-only, and always on. The house rule ([`.okf/architecture/feature-flags.md`](../.okf/architecture/feature-flags.md) "Principle") is that features gate *what a dependency or `std` costs*, not surface area. The only new features are `seal-test-double = []` (a test double, no dep), `commit-reveal` (04a), `seal-aead` (04b), and the `crypto` umbrella over those two. **None of them is in `full`** — see [`.okf/decisions/crypto-features-outside-full.md`](../.okf/decisions/crypto-features-outside-full.md).

2. **Slots, not custody.** The kernel holds a card's *name* (`SlotId`), its *order* (`SlotPile`, `Permutation`), and its *value once revealed* (`Revealed<D>`). It does not hold ciphertext. Five reasons, all cardpack's own: (i) a type that never contains a secret cannot leak one — secrecy by absence, the same move as "no I/O imports means no I/O" in the [domain kernel](../.okf/architecture/domain-kernel.md); (ii) shuffle, cut, and draw only ever move *positions*, so a payload inside the container was dead weight; (iii) a type with no scheme parameter derives `Clone`/`Eq`/`Debug`/`Serialize`, and "a rejected operation changed nothing" is one `assert_eq!`; (iv) a real mental-poker protocol keeps the masked deck with the *players*, so a referee-side library that holds ciphertext is holding the wrong thing; (v) the one deployment that legitimately holds ciphertext — a single trusted dealer — is served by a plain `Vec<(SlotId, Bytes)>` beside a `SlotPile` (04b), which loses nothing. Prior art agrees: the `pkcore` spike that put a scheme parameter on its table paid 19 `where` bounds and a cascade of hand-written impls for it, and is being redone.

3. **`Seal<D>` is a five-item adapter, generic over the deck, and no container depends on it.** Three associated types (`Sealed`, `Token`, `Error`) and two methods. Each signature choice is on cardpack's terms:
   - `seal` and `unseal` take the **`SlotId`** — an AEAD backend binds payload to slot as associated data (defeats "swap two sealed cards"); an ElGamal backend ignores it.
   - `seal` takes **`&mut dyn RngCore`** — every real backend is randomized (nonces, masking). A `&self`-only `seal` forces interior-mutability RNGs on implementors; passing the RNG is the honest signature, and `dyn` keeps the trait object-safe.
   - **`SlotId(u16)`** — `French::decks(4)` is 216 cards; Dashavatara alone is 120.

   The trait's only kernel caller is `Revealed::reveal_with`, generic *at the method*. Prior art: `pkcore`'s EPIC-79b design sketch describes a trait of the same five-item shape (without the slot and RNG parameters). The resemblance is convergent, not a compatibility goal.

4. **Ordinal = index into the deck's *vocabulary***, i.e. `base_vec()` with duplicates removed in first-occurrence order — **not** the raw `base_vec()` slot. Pinochle lists `ACE_SPADES` twice (`src/basic/decks/pinochle.rs:31-32`); mental poker encodes plaintext *values*, so both copies must map to the same group element. `vocabulary()` is a free function over `&[BasicCard]` so `DeckKind::all()` sweeps can pin every shipped deck without generics.

5. **Canonical bytes are deck-relative and versioned.** Layout `CANON_V1`: `[0x01][u16 BE name_len][deck_name utf-8][u16 BE count][u16 BE ordinal]*`. Encoding follows *iteration order* (index 0 first) — deliberately independent of the "top = front" convention flagged at `basic_card.rs:38` (`TODO RF`), so a future top-of-deck refactor cannot silently change hashes.

6. **`Permutation` is `Vec<u16>` with the convention `out[i] = in[p[i]]`.** The field is private so every value is validated. `from_rng` is *defined* as `slice::shuffle` applied to the identity, which is exactly what `Pile::shuffle_with_rng` (`pile.rs:796`) does to the cards — so `perm.apply(deck) == deck.shuffled_with_rng(same rng state)` holds by construction and is pinned by a test. `SlotPile::shuffle_with_rng` is the same call on labels, so a sealed and a clear deal of the same seed agree slot-for-slot.

7. **`SlotPile::new(n)` yields slots `0..n` in order — which is *also* the ordinal order if you then seal `Codebook` order into it.** Sealing an unshuffled `Pile` into unshuffled slots publishes the deck (slot == ordinal). The kernel cannot prevent a caller from doing that; 04b's dealer helper shuffles first, and the hazard has a named test so the doc warning cannot be "simplified" away.

8. **Backends own their error enums; `CardError` stays crypto-free.** Same reasoning that keeps `serde_norway::Error` boxed (`src/common/errors.rs:38-44`): `CardError` is `Eq + PartialEq` and alloc-only. `PlaintextSeal::Error = CardError`; `Revealed::reveal_with` wraps the two sources in `SealError<E>`.

9. **`SlotPile::draw(n) -> Option<Self>`**, all-or-nothing, mirroring `Pile::draw` (`pile.rs:227`). Same crate, same contract.

10. **`Revealed<D>` carries `Pile<D>`'s bounds** (`D: DeckedBase + Default + Ord + Copy + Hash`, `pile.rs:72-74`) so `#[derive(Clone, Debug, Default, Eq, PartialEq)]` works without hand-written impls — the C4 problem is structurally absent when nothing is generic over `S`.

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

**Why a held struct and not a method on `Card`.** `pkmental`'s two `OnceLock<HashMap>`s (`pkmental/src/encode.rs:38-60`) are the `std` answer; there is no `Sync` static cache in `core`. A `Codebook` you build once and pass around is the pure equivalent, and a linear scan over ≤ 120 entries is not worth a map. `Card::<D>::ordinal()` as a convenience was rejected: it would allocate `base_vec()` per call and hide an O(V) cost behind an innocent name.

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
/// Cut. `Err(InvalidCut(at))` if `at > len()`.
pub fn cut(&mut self, at: usize) -> Result<(), CardError>;
```

`Pile` has `get`, `position`, `remove`, `same`, and `unique_cards` today but **no `cut`** — the cut is new for plaintext and slot piles alike, and both are defined as `rotation`.

### `SlotId` — identity without knowledge

`src/seal/slot.rs` (new):

```rust
/// A stable, public handle for one card in a shoe.
///
/// Assigned at deal-setup time and carried thereafter, so shuffling
/// permutes *order* while every card keeps its name. This is what lets a
/// ledger say "seat 3 revealed slot 17" without saying what slot 17 is.
///
/// Deliberately NOT the ordinal — that would be the card.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlotId(u16);
impl SlotId { pub const fn new(n: u16) -> Self; pub const fn get(self) -> u16; pub const fn index(self) -> usize; }
impl Display for SlotId { /* the bare number */ }
```

### `SlotPile` — the shoe, as names

`src/seal/slot_pile.rs` (new). **Not generic.** Everything derives.

```rust
/// An ordered shoe of card *names*. Holds no card, no payload, no scheme.
/// Shuffle, cut, draw and deal are permutations of labels and need no
/// knowledge — so this type can be handed to a referee, a spectator, or a
/// log without leaking anything, because there is nothing to leak.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlotPile(Vec<SlotId>);

impl SlotPile {
    /// Slots `0..n`, in order. See decision 7 for the slot == ordinal hazard.
    pub fn new(n: u16) -> Self;
    /// Rejects duplicate slots (`DuplicateSlot`).
    pub fn from_slots(slots: Vec<SlotId>) -> Result<Self, CardError>;

    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn slots(&self) -> &[SlotId];
    pub fn contains(&self, slot: SlotId) -> bool;
    pub fn position(&self, slot: SlotId) -> Option<usize>;

    /// Remove by name. Needs no knowledge.
    pub fn take(&mut self, slot: SlotId) -> Option<SlotId>;
    pub fn draw_first(&mut self) -> Option<SlotId>;
    /// All-or-nothing, mirroring `Pile::draw` (`pile.rs:227`).
    pub fn draw(&mut self, n: usize) -> Option<Self>;

    /// Blind Fisher–Yates. Mirrors `Pile::shuffle_with_rng` (`pile.rs:796`);
    /// the same RNG state gives the same permutation on a `Pile` of equal length.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R);
    pub fn shuffle_with_seed(&mut self, seed: u64);
    pub fn permute(&self, p: &Permutation) -> Result<Self, CardError>;
    pub fn cut(&mut self, at: usize) -> Result<(), CardError>;

    /// Counts and checks slot uniqueness. That is *all* a deck of names can
    /// check; distinctness of what the names stand for is a backend property
    /// (a verifiable shuffle argument — 04c).
    pub fn audit(&self, expected: usize) -> SlotAudit;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotAudit { pub expected: usize, pub actual: usize, pub duplicate_slots: Vec<SlotId> }
impl SlotAudit { pub fn is_ok(&self) -> bool; }
```

**Why a `Vec` and not a set.** Order is the whole point of a shoe; a set would forget it. Uniqueness is enforced on construction and re-checked by `audit`, not by the container type.

**Methods deliberately absent:** anything taking or returning a `Card<D>`. `SlotPile` has no `D`. If you find yourself wanting `SlotPile::card_at`, you want `Revealed<D>`.

### `Revealed<D>` — the only place a value can be

`src/seal/revealed.rs` (new). Bounds match `Pile<D>` (`pile.rs:72-74`) so derives work.

```rust
/// Slot → card, for slots whose value has been turned up. The *only* kernel
/// type that maps a name to a value. If this is empty, no card value exists
/// anywhere in the game state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Revealed<D: DeckedBase + Default + Ord + Copy + Hash>(BTreeMap<SlotId, Card<D>>);

impl<D: DeckedBase + Default + Ord + Copy + Hash> Revealed<D> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn get(&self, slot: SlotId) -> Option<Card<D>>;
    pub fn is_revealed(&self, slot: SlotId) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = (SlotId, Card<D>)> + '_;
    /// The revealed cards for `slots`, in that order. `SlotNotFound` if any is unrevealed.
    pub fn pile_for(&self, slots: &[SlotId]) -> Result<Pile<D>, CardError>;

    /// An unverified reveal: the caller vouches for `(slot, card)`.
    /// `SlotAlreadyRevealed` if the slot is present.
    pub fn reveal(&mut self, slot: SlotId, card: Card<D>) -> Result<(), CardError>;

    /// A verified reveal: the backend's ciphertext and token are checked by
    /// `scheme` before the value is admitted. Generic at the method only —
    /// `Revealed<D>` itself knows nothing about `S`.
    pub fn reveal_with<S: Seal<D>>(
        &mut self,
        slot: SlotId,
        sealed: &S::Sealed,
        scheme: &S,
        token: &S::Token,
    ) -> Result<Card<D>, SealError<S::Error>>;
}

#[derive(Debug)]
pub enum SealError<E> { Slot(CardError), Backend(E) }
```

**Why two reveal paths.** In mental poker the value arrives from the players' protocol already proven; the referee records it (`reveal`). In dealer custody (04b) the value arrives as a token and must be checked against the stored ciphertext (`reveal_with`). Both end in the same map, so downstream code — showdown, hand history, display — sees one type.

### `Seal<D>` — the adapter, owned by the caller

`src/seal/seal.rs` (new), always on:

```rust
/// A card-sealing scheme. cardpack defines the shape; the *caller* provides
/// the implementation, the keys, and the tokens. cardpack never constructs
/// an `S` and no cardpack type is generic over one — the only kernel caller
/// is `Revealed::reveal_with`.
///
/// The slot is passed to `seal`/`unseal` (backends may bind it), `seal` takes
/// an RNG (real backends are randomized), and `SlotId` is `u16`.
pub trait Seal<D: DeckedBase> {
    /// The opaque payload. The backend picks the representation: 42 bytes of
    /// AEAD output, an ElGamal ciphertext, or (in tests) a `Card<D>`.
    /// `Eq` is for containers only — under any randomized scheme,
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

    /// Open one sealed payload with a token. A wrong token, wrong slot, or
    /// wrong context is `Err` — never a wrong card.
    fn unseal(&self, sealed: &Self::Sealed, slot: SlotId, token: &Self::Token)
        -> Result<Card<D>, Self::Error>;
}
```

**Why `seal` is on the trait when the kernel never calls it:** so that a single `impl` is the complete, reviewable statement of a scheme, and so the round-trip law `unseal(seal(c, s, rng), s, t) == c` is one generic test (`seal_roundtrip`) every backend runs through — 04b's AEAD, 04c's ElGamal, and the test double.

### `PlaintextSeal` — the test double, hard to reach on purpose

`src/seal/plaintext.rs` (new), `#[cfg(any(test, feature = "seal-test-double"))]`:

```rust
/// **NO SECURITY WHATSOEVER.** `Sealed = Card<D>`; "sealing" is the identity
/// function and `unseal` checks `token.0 == self.secret`. It exists to test
/// `reveal_with` and the conformance helper — not secrecy. Never reachable
/// in a default build.
pub struct PlaintextSeal { secret: u64 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainToken(pub u64);

impl<D: DeckedBase> Seal<D> for PlaintextSeal {
    type Sealed = Card<D>; type Token = PlainToken; type Error = CardError; …
}

/// Generic conformance helper, exported under `seal-test-double` so backends
/// in other crates (04c) can run the same law.
pub fn seal_roundtrip<D, S: Seal<D>>(scheme: &S, token_for: impl Fn(SlotId) -> S::Token, rng: &mut dyn RngCore)
where D: Decked<D> + Default + Ord + Copy + Hash;
```

`Token = ()` was rejected: "wrong token ⇒ `Err`" would be untestable on the double.

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
#[error("Duplicate slot `{0}`")]                                      DuplicateSlot(u16),
#[error("Slot `{0}` not found")]                                      SlotNotFound(u16),
#[error("Slot `{0}` is already revealed")]                            SlotAlreadyRevealed(u16),
```

### Module layout

```
src/basic/types/ordinal.rs        Ordinal, Codebook<D>, vocabulary(), CANON_V1, encode/decode_pile
src/basic/types/permutation.rs    Permutation
src/basic/types/pile.rs           + permute, cut
src/basic/types/traits.rs         + Decked::codebook() default method
src/seal/mod.rs                   "no values, no keys, no ciphertext live here" header; re-exports
src/seal/slot.rs                  SlotId
src/seal/slot_pile.rs             SlotPile, SlotAudit          (non-generic)
src/seal/revealed.rs              Revealed<D>, SealError<E>
src/seal/seal.rs                  trait Seal<D>
src/seal/plaintext.rs             PlaintextSeal, seal_roundtrip  (cfg test | seal-test-double)
src/seal/commit/                  EPIC-04a  (cfg commit-reveal)
src/seal/aead/                    EPIC-04b  (cfg seal-aead)
src/lib.rs:358                    + pub mod seal;
src/prelude.rs                    + Ordinal, Codebook, Permutation, SlotId, SlotPile, SlotAudit, Revealed, Seal, SealError
tests/seal_properties.rs          proptest suite (cfg not wasm32), pattern from tests/properties.rs
```

### Feature flags / `Cargo.toml`

```toml
# Test double for the seal adapter. Off by default; adds no dependency.
seal-test-double = []
# Umbrella for the real crypto backends (EPIC-04a, EPIC-04b). Deliberately
# NOT in `full` — see .okf/decisions/crypto-features-outside-full.md.
crypto = ["commit-reveal", "seal-aead"]
```

No new dependencies in this document. Nothing here implies `std`; `alloc` is already unconditional.

---

## Story 0: Prerequisites & gating (`src/lib.rs`, `Cargo.toml`, `src/common/errors.rs`)

**Acceptance:** an empty `seal` module and the new features exist; the nine `CardError` variants compile ungated; every purity gate is green before any type is written.

**Files:**
- Modify: `src/lib.rs:358` (add `pub mod seal;`)
- Create: `src/seal/mod.rs` (header comment + empty)
- Modify: `Cargo.toml:24-35` (`seal-test-double`, `crypto` features)
- Modify: `src/common/errors.rs:13` (nine variants)

### Tasks

- [x] Add `pub mod seal;` and the two features; confirm `cargo build --no-default-features` is green with the empty module
- [x] Add the nine `CardError` variants; confirm `CardError` keeps `Eq + PartialEq` (test `card_error__seal_variants_display`)
- [x] `make no-std`, `make no-std-thumbv7`, `cargo deny check bans` green
- [x] `cargo test --all` green

---

## Story 1: `Ordinal` + `Codebook<D>` (`src/basic/types/ordinal.rs`)

**Acceptance:** `card(ordinal(c)) == c` for every card of every shipped deck; the blank card and a foreign card yield `None`; Pinochle's vocabulary is 24, not 48; Standard52's ordinal table is pinned.

**Files:**
- Create: `src/basic/types/ordinal.rs`
- Modify: `src/basic/types.rs` (declare module)
- Modify: `src/basic/types/traits.rs:58` (`Decked::codebook()` default)
- Modify: `src/prelude.rs` (re-exports)

### Tasks

- [x] `Ordinal` newtype with `new`/`get`/`index`/`Display` and the serde-gated derive
- [x] `vocabulary(&[BasicCard])` via `itertools::unique` (test `vocabulary__dedups_first_occurrence` on Pinochle: 48 in, 24 out, order preserved)
- [x] `Codebook<D>::new/len/ordinal/card/iter` (test `codebook__roundtrip_every_shipped_deck` — a macro over the 14 marker types **and** a `DeckKind::all()` sweep via `vocabulary(&kind.base_vec())`, so deck 15 cannot dodge it)
- [x] `codebook__blank_has_no_ordinal` (`Card::<French>::default()` → `None`; `Ordinal(V)` → `None`)
- [x] `codebook__standard52_golden` — read `Standard52::DECK` (`src/basic/decks/standard52.rs:90`), pin the full table; from here on a reorder is a breaking change
- [x] `Decked::codebook()` default method (doctest on `French`)
- [x] `cargo test --no-default-features --lib` green

---

## Story 2: Canonical bytes (`src/basic/types/ordinal.rs`)

**Acceptance:** `decode_pile(encode_pile(p)) == p` for a full deck, a shuffled deck, a 5-card hand, and an empty pile; the header bytes for Standard52 are pinned; version, truncation, and deck-name mismatches each error with a named variant.

### Tasks

- [x] `CANON_V1` + `encode_pile` (iteration order; `CardNotInDeck` on a foreign card)
- [x] `decode_pile` with strict length checks (`CanonicalMalformed` on trailing or missing bytes)
- [x] Tests: `canonical__roundtrip`, `canonical__golden_standard52_prefix` (`01 00 0B "Standard 52" 00 34 …`), `canonical__bad_version`, `canonical__truncated`, `canonical__wrong_deck_name` (Standard52 bytes decoded through `Codebook<Skat>` → `CanonicalMalformed`)

---

## Story 3: `Permutation` + `Pile::permute` / `Pile::cut` (`src/basic/types/permutation.rs`, `pile.rs`)

**Acceptance:** every constructor validates; `apply`/`inverse`/`then` obey the group laws under `proptest`; `from_rng` reproduces `Pile::shuffle_with_rng` bit-for-bit; `rotation` is the cut; canonical bytes round-trip.

**Files:**
- Create: `src/basic/types/permutation.rs`
- Modify: `src/basic/types/pile.rs` (`permute`, `cut`)
- Modify: `src/prelude.rs`

### Tasks

- [x] Constructors + validation (`permutation__rejects_duplicate`, `permutation__rejects_out_of_range`, `permutation__identity_too_large_errors`)
- [x] `apply` / `inverse` / `then` (`permutation__inverse_roundtrip`, `permutation__compose_law`, `permutation__apply_length_mismatch_errors`)
- [x] `from_rng` ≡ `shuffle_with_rng` (`permutation__from_rng_matches_pile_shuffle`: `Permutation::from_rng(52, &mut StdRng::seed_from_u64(s)).apply(deck.cards()) == deck.shuffled_with_seed(s).cards()`)
- [x] `rotation` (`permutation__rotation_is_cut`)
- [x] `canonical_bytes` / `from_canonical_bytes` (`permutation__canonical_roundtrip`, `permutation__canonical_rejects_invalid`)
- [x] `Pile::permute` / `Pile::cut` (`pile__permute_preserves_multiset` via `same`, `pile__cut_preserves_multiset`, `pile__cut_past_end_errors`, `pile__cut_at_len_is_identity`)

---

## Story 4: `SlotId` + `SlotPile` (`src/seal/slot.rs`, `src/seal/slot_pile.rs`)

**Acceptance:** `SlotPile` derives everything; construction rejects duplicate slots; blind shuffle/permute/cut are permutations of the slot set and agree slot-for-slot with a `Pile` shuffled from the same RNG state; `draw` is all-or-nothing; a rejected operation changes nothing (`assert_eq!` on the whole value).

**Files:**
- Create: `src/seal/slot.rs`, `src/seal/slot_pile.rs`
- Modify: `src/seal/mod.rs`, `src/prelude.rs`

### Tasks

- [x] `SlotId` with `new`/`get`/`index`/`Display`
- [x] `SlotPile::new`/`from_slots` (`slot_pile__new_is_identity_order`, `slot_pile__from_slots_rejects_duplicates`)
- [x] `take`/`draw_first`/`draw`/`contains`/`position` (`slot_pile__draw_all_or_nothing`, `slot_pile__take_by_name`, `slot_pile__rejected_draw_changes_nothing` — `assert_eq!(before, after)`)
- [x] `shuffle_with_rng`/`shuffle_with_seed`/`permute`/`cut` (`slot_pile__shuffle_permutes_slot_set`, `slot_pile__shuffle_agrees_with_pile_shuffle_for_same_rng` — the slot at position *i* names the card at position *i* of the equally-shuffled `Pile`, `slot_pile__cut_matches_permutation_rotation`)
- [x] `audit` (`slot_pile__audit_counts_and_finds_duplicates`; doc comment states what it cannot check)
- [x] `slot_pile__serde_roundtrip` (under `serde`)
- [x] Compile-time assertion that `SlotPile` has no method mentioning `Card` — a `#[cfg(test)]` doc note plus a review item, not a test

---

## Story 5: `Revealed<D>` + `Seal<D>` + `PlaintextSeal` (`src/seal/revealed.rs`, `seal.rs`, `plaintext.rs`)

**Acceptance:** `Revealed<D>` derives cleanly with `Pile`'s bounds; `reveal` refuses a second reveal; `reveal_with` round-trips under the double and a wrong token is `Err` with the map unchanged; `pile_for` errors on an unrevealed slot; `Seal<D>` is object-safe.

**Files:**
- Create: `src/seal/revealed.rs`, `src/seal/seal.rs`, `src/seal/plaintext.rs`
- Modify: `src/seal/mod.rs`, `src/prelude.rs`

### Tasks

- [x] `Seal<D>` trait (compile-time check that `&dyn Seal<French, Sealed = …, Token = …, Error = …>` is nameable)
- [x] `Revealed<D>` with `reveal`/`get`/`is_revealed`/`iter`/`pile_for` (`revealed__reveal_twice_errors`, `revealed__pile_for_unrevealed_errors`, `revealed__pile_for_preserves_order`, `revealed__serde_roundtrip`)
- [x] `Revealed::reveal_with` + `SealError` (`revealed__reveal_with_roundtrip`, `revealed__reveal_with_wrong_token_errors_and_map_unchanged`, `revealed__reveal_with_never_yields_other_card`)
- [x] `PlaintextSeal` + `PlainToken` + `seal_roundtrip` helper (`seal__roundtrip_law` — first caller)
- [x] Confirm `PlaintextSeal` is absent from `cargo doc --no-default-features` output

---

## Story 6: Property tests (`tests/seal_properties.rs`)

**Acceptance:** the group laws, the slot-shuffle/pile-shuffle agreement, and "empty `Revealed` means no value anywhere" hold for arbitrary seeds, reproducible from a failing seed.

**Files:**
- Create: `tests/seal_properties.rs` (header conventions from `tests/properties.rs:1-20`; `#![cfg(not(target_arch = "wasm32"))]`)

### Tasks

- [x] `permutation__inverse_roundtrip` (seed), `permutation__compose_law` (two seeds), `permutation__from_rng_matches_pile_shuffle` (seed)
- [x] `slot_pile__shuffle_permutes_slot_set` (seed), `slot_pile__shuffle_agrees_with_pile_shuffle` (seed), `slot_pile__rejected_ops_change_nothing` (seed, random illegal `draw`/`cut`)
- [x] `deal__slots_then_reveal_all_equals_clear_deal` (seed): shuffle a `SlotPile` and a `Pile` from the same seed, deal *n* slots, reveal them via `Codebook` order, compare to the clear deal — meaningful because the slot path never held a value, so passing is a property of the design, not of a test double
- [x] `cargo test --features seal-test-double --test seal_properties` green

---

## Story 7: Docs, prelude, release (`README.md`, `CHANGELOG.md`, `Cargo.toml`, `.okf/`)

**Acceptance:** every new public item is re-exported from the prelude and documented with an ungated doctest; README's feature table lists the new features; CHANGELOG `Added`; version `0.11.0`; `.okf/` reflects the shipped state.

### Tasks

- [x] Prelude re-exports; doctests use `Permutation` / `Codebook` / `SlotPile` (all ungated), never `PlaintextSeal`
- [x] `src/seal/mod.rs` header: the rule in one paragraph — slots, order, revealed values; never ciphertext, never keys, never a scheme parameter — plus the consumer notes from [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) §3
- [x] README feature rows (`seal-test-double`, `crypto`) with the "not in `full`" note
- [x] CHANGELOG `Added` + `Cargo.toml` `0.11.0`
- [x] `.okf/architecture/feature-flags.md` rows flipped from *planned* to live; `.okf/references/epic-04-sealed-decks.md` tags `planned` → `active`; `.okf/log.md` entry; `/okf:validate .okf --strict`
- [x] Flip this document's Status rows; cross-link 04a/04b/04c from BACKLOG

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
| `slot_pile__shuffle_agrees_with_pile_shuffle_for_same_rng` | A blind shuffle and a clear shuffle from one seed agree slot-for-slot |
| `slot_pile__rejected_draw_changes_nothing` | Plain-value payoff: one `assert_eq!` on the whole shoe |
| `slot_pile__audit_counts_and_finds_duplicates` | The audit's honest scope |
| `revealed__reveal_twice_errors` | A value cannot be silently replaced |
| `revealed__reveal_with_wrong_token_errors_and_map_unchanged` | `Err`, never `Ok(other_card)`, and nothing admitted |
| `seal__roundtrip_law` | The generic law every backend (04b, 04c) reuses |
| `deal__slots_then_reveal_all_equals_clear_deal` (prop) | The slot path is a faithful deal — meaningful by absence of values |

**Gold Standard check:** before closing, delete each guard in turn — the `try_from_vec` bitset check, the `from_slots` duplicate check, the `reveal` already-revealed check, the token comparison in `PlaintextSeal::unseal` — and confirm a named test goes red.

## Key Files

| File | Role |
|---|---|
| `src/basic/types/ordinal.rs` | **New.** `Ordinal`, `Codebook<D>`, `vocabulary`, `CANON_V1`, canonical bytes |
| `src/basic/types/permutation.rs` | **New.** `Permutation` |
| `src/basic/types/pile.rs:72` | `Pile::permute`, `Pile::cut` |
| `src/basic/types/traits.rs:58` | `Decked::codebook()` default method |
| `src/seal/mod.rs` | **New.** Module header (the rule), consumer notes, re-exports |
| `src/seal/slot.rs` | **New.** `SlotId` |
| `src/seal/slot_pile.rs` | **New.** `SlotPile`, `SlotAudit` — non-generic |
| `src/seal/revealed.rs` | **New.** `Revealed<D>`, `SealError<E>` |
| `src/seal/seal.rs` | **New.** `trait Seal<D>` |
| `src/seal/plaintext.rs` | **New.** `PlaintextSeal`, `seal_roundtrip` (gated) |
| `src/common/errors.rs:13` | Nine new variants |
| `src/lib.rs:358`, `src/prelude.rs` | Module declaration, re-exports |
| `Cargo.toml:24-35` | `seal-test-double`, `crypto` features |
| `tests/seal_properties.rs` | **New.** Property suite |
| `.okf/decisions/crypto-features-outside-full.md` | **New.** Why backends stay out of `full`; why the kernel holds slots |
| `.okf/references/epic-04-sealed-decks.md` | **New.** Mirror pointer |

## Reuse (do NOT recreate)

- `src/basic/types/pile.rs:796` `shuffle_with_rng` / `:776` `shuffle_with_seed` — `SlotPile` and `Permutation::from_rng` are *defined* to agree with them.
- `src/basic/types/pile.rs:227` `draw` — the all-or-nothing contract `SlotPile::draw` copies.
- `src/basic/types/pile.rs:733` `same` and `:443` `unique_cards` — multiset assertions in tests.
- `src/basic/types/traits.rs:30` `DeckedBase::base_vec` / `deck_name` — the **sole** source of the vocabulary and the domain tag. Do not add a parallel card list.
- `src/basic/decks/registry.rs:88` `DeckKind::all()` — every-deck sweeps.
- `src/common/errors.rs:13` `CardError` (`#[non_exhaustive]`) — extend; never introduce a second kernel error enum.
- `tests/properties.rs` — the proptest header, seeding idiom, and `name__property` naming.
- `itertools::unique` — already a dependency with `use_alloc`.
- Prior art only, no dependency: `pkcore`'s `docs/epics/EPIC-79b_Sealed_Deck.md` (a design sketch of a five-item seal trait) and `pkmental/src/encode.rs:33-60` (the runtime bijection `Codebook<D>` makes unnecessary).

## Compatibility

- **Preserves:** every existing public signature. `Card`, `BasicCard`, `Pile`, `Decked`'s required methods, and all 14 decks are untouched. A `default = []` build gains the new value types and no dependencies.
- **Adds:** `Ordinal`, `Codebook<D>`, `vocabulary`, `CANON_V1`, `Permutation`, `Pile::permute`/`cut`, `Decked::codebook()` (default), the `seal` module (`SlotId`, `SlotPile`, `SlotAudit`, `Revealed<D>`, `Seal<D>`, `SealError`), nine `CardError` variants, two features.
- **Breaks:** nothing intended. `CardError` is already `#[non_exhaustive]`. **New contract:** the order of every shipped deck's `base_vec()` is now stable; reordering is semver-major from 0.11.0.
- **Package size:** unchanged in kind — only `src/**` ships (`Cargo.toml:13`).

## Dependencies

- **Blocks:** [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md) (needs Stories 2–3), [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) (needs Stories 1, 4–5), [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) (needs everything).
- **Built on:** the seeded-shuffle work (`docs/2026-04-29-seeded-shuffle-design.md`); the `DeckKind` registry (EPIC-02); the `#[non_exhaustive]` precedent on `CardError` (EPIC-03); the domain-kernel invariants (`docs/audit-2026-07-18-domain-kernel.md`).
- **Related (prior art / possible consumers, no dependency either way):** `pkmental` (the protocol whose `encode.rs` bijection `Codebook<D>` makes unnecessary); `pkcore` EPIC-79b (a design sketch of a similar seal trait, and a spike whose complexity is evidence for decision 2).

## Verification

```bash
# Purity gates — the kernel must stay dependency-free
cargo build --no-default-features
make no-std
make no-std-thumbv7
cargo deny check bans
cargo test --no-default-features --lib

# The seal adapter with its test double
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

# The kernel is not generic over a scheme
! grep -rnE 'struct \w+<[^>]*S: Seal' src/seal/

# Portability
cargo build --target wasm32-unknown-unknown --all-features
make ayce
```

Exit criteria:

1. `Codebook<D>` round-trips every card of every `DeckKind` and every marker type; the Standard52 golden table passes.
2. `Permutation::from_rng(n, rng).apply(&pile) == pile.shuffled_with_rng(rng)` and `SlotPile` agrees slot-for-slot, for the same RNG state.
3. No struct in `src/seal/` carries a scheme type parameter (the `grep` above), and `SlotPile` has no method mentioning `Card`.
4. `Revealed<D>` is the only `SlotId → Card<D>` map in the crate; `deal__slots_then_reveal_all_equals_clear_deal` passes.
5. Every negative test matches a named `CardError` variant, not bare `is_err()`.
6. `cargo build --no-default-features`, both bare-metal targets, and `cargo deny check bans` are green: nothing here leaks into the pure kernel.
7. `PlaintextSeal` does not appear in default-build docs.
8. `.okf/` updated and `/okf:validate .okf --strict` is clean.

---

## Gotchas

1. **The slot == ordinal leak is the whole game.** `SlotPile::new(52)` beside `Standard52::deck()` in `Codebook` order is a public deck wearing a costume. Shuffle the slots (or the pile) first. 04b's dealer helper does; the hazard has a named test there, and decision 7 says so here so nobody removes the warning to tidy the docs.

2. **`Sealed: Eq` compares ciphertexts, not cards.** Under any scheme worth using, sealing is randomized, so `Eq` proves nothing about distinctness. It is on the trait so sealed payloads can live in ordinary containers. `SlotAudit` cannot do more than count for the same reason.

3. **`Card: Copy` means plaintext is never zeroized.** A revealed card is copied onto the stack, into a `Vec`, into a log line. Key hygiene is 04b's job; plaintext hygiene is out of scope and must be stated in the `seal` module docs.

4. **Do not add a payload to `SlotPile` "for convenience."** The day it gains a `Vec<Bytes>` beside the slots, it is `SealedDeck<S>` again with the type parameter erased into `Vec<u8>` — and the derive-free, one-`assert_eq!` property is gone. Custody is a `Vec<(SlotId, Bytes)>` *beside* a `SlotPile` (04b), never inside it.

5. **One `Permutation` convention, stated once.** `out[i] = in[p[i]]` must be used identically in `apply`, `inverse`, `rotation`, `Pile::permute`, `SlotPile::permute`, and 04a's derivation. The compose-law and agreement tests are the only guard.

6. **`u16` is a real limit.** `Permutation::identity(70_000)` errs, and so does a `Codebook` for a hypothetical deck with more than 65 535 distinct cards. Document it; do not widen to `u32` "just in case" — the canonical byte formats are frozen at `u16`.

7. **The `seal` module is unconditional, so it must build on thumb.** Anything it `use`s has to be `core`/`alloc`/`rand` without `std`. `rand::RngCore` is fine — `std_rng` is already unconditional ([`.okf/decisions/rand-std-rng-unconditional.md`](../.okf/decisions/rand-std-rng-unconditional.md)). `BTreeMap` is `alloc`; `HashMap` is not.

8. **Doctests must stay flag-free.** `PlaintextSeal` is gated, so doctests demonstrate `Permutation`, `Codebook`, `SlotPile`, and `Revealed::reveal` (all ungated) — the standing rule in [`.okf/architecture/feature-flags.md`](../.okf/architecture/feature-flags.md) is *prefer the ungated API first*.

9. **`DeckKind::all()` is 13 without `yaml`** (`Razz` is gated). Registry sweeps read `DeckKind::all().len()`, never a literal 14.

10. **Iteration order vs top-of-deck.** `basic_card.rs:38` carries a `TODO RF` to flip the deck so the *end* of the vector is the top. Canonical bytes, `Permutation`, and `SlotPile` are defined over *iteration order* precisely so that refactor changes nothing here.

---

## Implementation corrigendum

Written 2026-08-24 on branch `crypt`, after Stories 0–7 landed test-first (RED
compile failures → GREEN → every gate). Deltas between the design above and
what shipped:

### 1. `vocabulary()` uses a `BTreeSet`, not `itertools::unique`

`Itertools::unique` is gated on `use_std` (`itertools-0.15.0/src/lib.rs:1687`);
the crate depends on `itertools` with `use_alloc` only. A `BTreeSet<BasicCard>`
seen-set is the `alloc` equivalent. Test: `vocabulary__dedups_first_occurrence`.

### 2. The seal seam is `&mut dyn Rng`, not `&mut dyn RngCore`

`rand 0.10` renamed the core trait: `rand::Rng` *is* the object-safe core
(`rand-0.10.2/src/lib.rs:59`) and `RngCore` no longer exists at the root.
`Seal::seal`, `PlaintextSeal`, and `seal_roundtrip` take `&mut dyn Rng`. The
extension methods live on `RngExt`. Test: `seal__trait_is_object_safe`.

### 3. `src/seal/seal.rs` is `src/seal/adapter.rs`

`pub mod seal;` inside `seal/mod.rs` trips clippy-pedantic `module_inception`.
The trait is still `crate::seal::Seal` via re-export; only the file moved.

### 4. `PlaintextSeal::Error` is its own `PlainSealError`, not `CardError`

No `CardError` variant honestly means "wrong token", and decision 8 says
backends own their error enums — that applies to the double too. One variant,
`WrongToken`. Test: `plaintext_seal__wrong_token_errors`.

### 5. The `crypto` umbrella feature waits for 04a/04b

Cargo rejects a feature list naming features that do not exist, so
`crypto = ["commit-reveal", "seal-aead"]` lands with whichever child ships
first. Only `seal-test-double` was added here.

### 6. `SealError<E>` is not `Clone`

`CardError` is not `Clone` (it never was), so the derive was dropped.
`Debug + Eq + PartialEq + Display + Error` remain.

### 7. `Revealed<D>` needs `#[serde(bound = "")]`

`serde`'s derive would demand `D: Serialize` for `BTreeMap<SlotId, Card<D>>`.
Only `Card<D>` must serialize (its own derive skips the `PhantomData` brand), so
the bound is emptied explicitly. Test: `revealed__serde_roundtrip`.

### 8. `Codebook::new` truncates the vocabulary at `u16::MAX`

So every index fits an `Ordinal` without a fallible cast in `iter`. No shipped
deck comes near it (Dashavatara is 120); documented on the constructor.

### 9. `SlotPile::audit` can only find duplicates the constructors forbid

Every public constructor — `new`, `from_slots`, and the `serde` deserializer via
`TryFrom` — rejects duplicates, so `duplicate_slots` is reachable only by a
module-private construction. The unit test builds one that way to prove the
check works; the field stays because a future constructor must not silently
lose it.

### Phase status summary

| Story | Status | Notes |
|---|---|---|
| 0 (prereqs, features, `CardError`) | Shipped | `crypto` umbrella deferred (item 5) |
| 1 (`Ordinal`, `Codebook`) | Shipped | 7 unit tests + 3 doctests |
| 2 (canonical bytes) | Shipped | 9 unit tests; golden prefix pinned |
| 3 (`Permutation`, `Pile::permute`/`cut`) | Shipped | 17 unit tests; `from_rng ≡ shuffle_with_rng` pinned for 5 seeds |
| 4 (`SlotId`, `SlotPile`) | Shipped | 12 unit tests; serde `TryFrom` guard |
| 5 (`Revealed`, `Seal`, `PlaintextSeal`) | Shipped | 12 unit tests; object-safety compile check |
| 6 (property suite) | Shipped | 8 properties; mutations `then()` order and shuffle no-op both caught |
| 7 (docs, prelude, 0.11.0, `.okf`) | Shipped | this section |

### Pre-existing debt

None touched. The `TODO RF` at `basic_card.rs:38` (top-of-deck orientation) is
unaffected by design — canonical bytes and permutations are defined over
iteration order.
