# EPIC-04a: Commit–Reveal Shuffle (CRS)

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Child of [EPIC-04 Sealed Decks](./EPIC-04_Sealed_Decks.md); needs its Stories 2–3 (`Codebook` canonical bytes, `Permutation`) landed first. "Default features green" **and** `cargo deny check bans` are preconditions for every story. Nothing has landed as of `main` @ `1c14440`, 2026-08-24.

**Goal:** **Provably-fair shuffling** without full mental poker. Every participant commits to secret entropy, everyone reveals, the combined seed deterministically derives a **`Permutation`**, and anyone holding the public transcript can re-derive the shuffle and check it. Also: blind **commitments to a concrete order** — a `Permutation` or a `Pile` — so a dealer can prove after the fact that the deck they dealt is the deck they committed to. This is the "general security in distributed games" tier: a trusted-but-auditable dealer, a browser client that can verify, a replay log that proves the shuffle.

**Architecture:** A new `src/seal/commit/` module behind the `commit-reveal` feature, pulling exactly one dependency — `sha2` (SHA-256, `default-features = false`, no_std). All public types are fixed-width byte newtypes (`Commitment([u8; 32])`, `Contribution([u8; 32])`, `CombinedSeed([u8; 32])`); no digest trait or type appears in any signature (kernel Invariant 2). The permutation is derived from the seed with SHA-256 in counter mode and Fisher–Yates with rejection sampling — **never** from `StdRng` — because the derivation is the verifier's contract and must not change when `rand` bumps a major (`src/basic/types/pile.rs:761-766`). `Pile::shuffled_by_round` ties it to the existing pile API.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std + alloc, `sha2 0.10` (`default-features = false`), `proptest`, the kernel-purity CI job (`.github/workflows/CI.yaml:210`) extended to ban `sha2` from the pure tree.

---

## Context

- Today a shuffle is an opaque side effect. `Pile::shuffle_with_seed` (`pile.rs:776`) is deterministic **within one `rand` major** and its own doc comment (`pile.rs:761-766`) warns the permutation may change on upgrade. `shuffle_with_rng` (`pile.rs:796`) lets a caller supply a portable RNG, but nothing lets a *verifier* reproduce what a dealer did from public data alone.
- Nothing in the crate hashes anything. `Hash` is the std derive for map keys (`basic_card.rs:43`, `pile.rs:71`); there is no digest dependency, no commitment, no transcript.
- EPIC-04 Story 3 delivers `Permutation` — a shuffle as data, with `canonical_bytes` — and Story 2 delivers `Codebook::encode_pile`, a versioned byte encoding of any pile. Those two are the preimages this EPIC commits to.
- The `kernel-purity` CI job (`CI.yaml:203-225`) asserts a `BANNED` regex of crate names is absent from the `--no-default-features` tree, kept in sync with `deny.toml`'s `[bans].deny` list (`deny.toml:56+`). Any new crypto crate must be added to both so it can never reach the pure kernel by accident.

**What this EPIC does NOT do:**

- **No sealed cards.** Commit–reveal hides the *shuffle*, not the *cards*. A player who sees the deck sees every card. Hiding cards is [04b](./EPIC-04b_Holder_Key_Seal.md) / [04c](./EPIC-04c_Mental_Poker_Bridge_spec.md).
- **No transport, no signatures, no liveness.** `ShuffleRound` is a pure state machine; who delivers the messages, who signs them, and what happens when a participant never reveals are the caller's problem. (A participant who commits and then refuses to reveal *aborts* the round; they cannot bias it. Say so in the docs.)
- **No digest-agnostic trait.** SHA-256 is part of the versioned preimage format (`v1`). A later `v2` tag can switch algorithms without a trait.
- **No `blake3`.** Rejected: needs `cc` unless `pure`, larger tree, and — the decisive point — the whole value of provably-fair is that a verifier in *any* language or browser can recompute with a standard library hash.

---

## Status

Status as of `main` @ `1c14440`, **2026-08-24**. Nothing has landed.

| Component | Status |
|---|---|
| `commit-reveal` feature + `sha2` dep + deny/CI ban rows | Planned |
| `Contribution` (secret, redacted `Debug`) + `Commitment` (32 bytes, hex) | Planned |
| `ParticipantId` + `ShuffleRound` state machine | Planned |
| `CombinedSeed::combine` + `::permutation` (SHA-256 counter mode, rejection sampling) | Planned |
| `commit_permutation` / `verify_permutation` | Planned |
| `commit_pile` / `verify_pile` (over `CANON_V1` bytes) | Planned |
| `Pile::shuffled_by_round` | Planned |
| `CardError` variants (6, gated) | Planned |
| `examples/provably_fair.rs` | Planned |
| `tests/commit_reveal.rs` (golden vectors + properties) | Planned |
| Docs / README / CHANGELOG / `.okf/` | Planned |

---

## Goals

- **Nobody can bias the shuffle.** The last participant to reveal learns nothing they can use, because every commitment precedes every reveal and the seed is a hash over all contributions.
- **Anybody can verify.** Given the public transcript (participant ids, commitments, revealed contributions), a verifier with only a SHA-256 implementation reproduces `CombinedSeed` and the exact `Permutation`.
- **The derivation is frozen.** Golden vectors pin `commit`, `combine`, and `permutation(52)` forever; a `rand` upgrade cannot move them.
- **Dealers can be audited after the fact.** `commit_pile` / `commit_permutation` let a dealer publish a hash before dealing and open it after.
- **One dependency, invisible by default.** `sha2` behind `commit-reveal`, banned from the pure tree.

## Scope

1. All new API is `#[cfg(feature = "commit-reveal")]`. A default build gains nothing.
2. No `sha2` / `digest` type appears in any public signature. Outputs are `[u8; 32]` newtypes.
3. Every preimage is **domain-separated** with a versioned ASCII tag (`b"cardpack/commit-reveal/v1/…"`) so a commitment to a contribution can never be confused with a commitment to a pile.
4. `ShuffleRound::reveal` is rejected until **every** participant has committed (`RevealBeforeAllCommitted`). A reveal that does not match its commitment is rejected and leaves the round unchanged (`CommitmentMismatch`).
5. `CombinedSeed::combine` is order-sensitive over `ParticipantId` — the id is part of the preimage, so swapping two participants' contributions changes the seed.
6. `CombinedSeed::permutation(n)` uses exact rejection sampling. No `% n`. The fairness claim is false otherwise.
7. `CombinedSeed::to_u64()` exists for convenience with `shuffle_with_seed` and carries the same cross-`rand`-major warning as `pile.rs:761-766`. The verifier path is `permutation()`.
8. Commitments are compared with plain `==`. They are public values; there is no secret to leak through timing, so `subtle` is not pulled in.

---

## Domain

**Things.**

| Thing | Type |
|---|---|
| A participant's secret entropy | `Contribution([u8; 32])` |
| Their binding + hiding promise about it | `Commitment([u8; 32])` |
| Who they are in this round | `ParticipantId(u16)` |
| The round: commit-all, then reveal-all | `ShuffleRound` |
| The seed everyone agrees on | `CombinedSeed([u8; 32])` |
| The shuffle it determines | `Permutation` (EPIC-04) |
| A dealer's promise about a concrete order | `commit_pile` / `commit_permutation` |

**Business Requirements.** (a) *Binding* — a participant cannot change their contribution after committing. (b) *Hiding* — a commitment reveals nothing about the contribution. (c) *Unbiasable* — no participant can choose their contribution knowing the others'. (d) *Reproducible* — the transcript alone determines the permutation.

**Business Logic.** SHA-256 preimage resistance gives (a); 32 random bytes of input give (b); the state machine's "no reveal before all commits" gives (c); the frozen counter-mode derivation gives (d).

---

## Design decisions (settled)

1. **SHA-256, not BLAKE3.** Verifiable from any language with no library; no build script; shares the `sha2` dep with 04b so `crypto` costs one hash crate.
2. **The hash is part of the format, not a trait parameter.** `v1` = SHA-256. A trait would suggest the algorithm is a free choice; it is not — the verifier on the other side must pick the same one, and the tag tells them which.
3. **Permutation derivation is SHA-256 counter mode + Fisher–Yates + rejection sampling.** `StdRng` was rejected for the reason its own docs give. The derivation is pinned by `derive__golden_permutation_52`.
4. **`Contribution::random` is the only documented way to make one.** `from_bytes` exists for tests and for callers with their own CSPRNG, and its doc says a low-entropy contribution is not hiding.
5. **Six gated `CardError` variants**, `u16`/`String` payloads only, keeping `CardError` `Eq` and crypto-free (EPIC-04 decision 7).
6. **`ShuffleRound` uses `BTreeMap`**, not `HashMap` — deterministic iteration, no_std, no hasher.

---

## Design

### `Contribution` and `Commitment`

`src/seal/commit/commitment.rs` (new):

```rust
/// 32 uniformly random bytes. Secret until revealed. `Debug` is redacted.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Contribution([u8; 32]);

impl Contribution {
    /// The only documented constructor. `rng` must be a CSPRNG.
    pub fn random(rng: &mut dyn RngCore) -> Self;
    /// For tests and callers with their own entropy. Low-entropy input is NOT hiding.
    pub const fn from_bytes(b: [u8; 32]) -> Self;
    pub const fn as_bytes(&self) -> &[u8; 32];
    /// `SHA-256(b"cardpack/commit-reveal/v1/contribution" || bytes)`.
    pub fn commit(&self) -> Commitment;
}

/// A binding, hiding commitment. Public. `Debug` prints hex.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Commitment([u8; 32]);

impl Commitment {
    pub const fn as_bytes(&self) -> &[u8; 32];
    pub fn to_hex(&self) -> String;
    pub fn from_hex(s: &str) -> Result<Self, CardError>;      // InvalidHex
    /// Recompute and compare. Plain `==`: both sides are public.
    pub fn verify(&self, c: &Contribution) -> bool;
}
```

### `ParticipantId`, `ShuffleRound`, `CombinedSeed`

`src/seal/commit/round.rs` (new):

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParticipantId(pub u16);

/// Phase A: every participant commits. Phase B: every participant reveals.
/// No reveal is accepted before all commitments are in.
#[derive(Clone, Debug)]
pub struct ShuffleRound {
    participants: Vec<ParticipantId>,
    commitments: BTreeMap<ParticipantId, Commitment>,
    reveals: BTreeMap<ParticipantId, Contribution>,
}

impl ShuffleRound {
    /// At least one participant; ids unique.
    pub fn new(participants: impl IntoIterator<Item = ParticipantId>) -> Result<Self, CardError>;
    pub fn commit(&mut self, who: ParticipantId, c: Commitment) -> Result<(), CardError>;   // UnknownParticipant, AlreadyCommitted
    pub fn all_committed(&self) -> bool;
    pub fn reveal(&mut self, who: ParticipantId, c: Contribution) -> Result<(), CardError>;  // RevealBeforeAllCommitted, CommitmentMismatch
    pub fn is_complete(&self) -> bool;
    /// Only when complete. `RoundIncomplete` otherwise.
    pub fn seed(&self) -> Result<CombinedSeed, CardError>;
}

/// The seed every honest verifier reaches. `Debug` prints hex.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CombinedSeed([u8; 32]);

impl CombinedSeed {
    /// `SHA-256(b"cardpack/commit-reveal/v1/seed" || u16 BE n || (u16 BE id || 32 bytes)*)`,
    /// pairs sorted by `ParticipantId`.
    pub fn combine(parts: &[(ParticipantId, Contribution)]) -> Self;
    /// Fisher–Yates over the identity, drawing from the SHA-256 counter-mode
    /// stream `SHA-256(seed || u32 BE counter)` with exact rejection sampling.
    /// Frozen: this is the verifier's contract.
    pub fn permutation(&self, n: usize) -> Result<Permutation, CardError>;
    /// First 8 bytes, big-endian — for `shuffle_with_seed` convenience only.
    /// NOT verifier-stable across `rand` majors; use `permutation()`.
    pub fn to_u64(&self) -> u64;
    pub const fn as_bytes(&self) -> &[u8; 32];
}
```

### Blind commitments to a concrete order

`src/seal/commit/pile.rs` (new):

```rust
/// `SHA-256(b"cardpack/commit-reveal/v1/permutation" || blind || p.canonical_bytes())`.
pub fn commit_permutation(p: &Permutation, blind: &Contribution) -> Commitment;
pub fn verify_permutation(c: &Commitment, p: &Permutation, blind: &Contribution) -> bool;

/// `SHA-256(b"cardpack/commit-reveal/v1/pile" || blind || codebook.encode_pile(pile)?)`.
pub fn commit_pile<D>(codebook: &Codebook<D>, pile: &Pile<D>, blind: &Contribution) -> Result<Commitment, CardError>;
pub fn verify_pile<D>(c: &Commitment, codebook: &Codebook<D>, pile: &Pile<D>, blind: &Contribution) -> Result<bool, CardError>;

impl<D> Pile<D> {
    /// `self.permute(&round.seed()?.permutation(self.len())?)`.
    pub fn shuffled_by_round(&self, round: &ShuffleRound) -> Result<Self, CardError>;
}
```

The `blind` is a fresh `Contribution`: without it, a commitment to one of 52! orders is still brute-forceable in the "did the dealer commit to *this* specific deal" sense for small piles (a 2-card commitment has 52·51 preimages). The blind makes every commitment hiding regardless of pile size.

### `CardError` additions

`src/common/errors.rs`, `#[cfg(feature = "commit-reveal")]`:

```rust
UnknownParticipant(u16), AlreadyCommitted(u16), RevealBeforeAllCommitted,
CommitmentMismatch(u16), RoundIncomplete, InvalidHex(String),
```

### `Cargo.toml`

```toml
commit-reveal = ["dep:sha2"]
sha2 = { version = "0.10", default-features = false, optional = true }
```

Implies neither `std` nor `alloc` beyond what the kernel already has. **At implementation time** check `cargo info sha2`: if the RustCrypto 0.11 stable line has shipped and holds MSRV ≤ 1.85, prefer it, and move `sha2`/`hkdf`/`digest` together with 04b. `0.10` is the known-good no_std baseline. Add `{ name = "sha2" }` to `deny.toml` `[bans].deny` and `sha2` to the `BANNED` regex (`CI.yaml:210`).

### Module layout

```
src/seal/commit/mod.rs          protocol doc + the preimage format table (the verifier's contract)
src/seal/commit/commitment.rs   Contribution, Commitment, tags
src/seal/commit/round.rs        ParticipantId, ShuffleRound, CombinedSeed
src/seal/commit/derive.rs       SHA-256 counter-mode permutation derivation
src/seal/commit/pile.rs         commit_pile / verify_pile / commit_permutation / Pile::shuffled_by_round
examples/provably_fair.rs       required-features = ["commit-reveal", "std"]
tests/commit_reveal.rs          golden vectors + proptests
```

---

## Story 0: Feature, dependency, bans (`Cargo.toml`, `deny.toml`, `.github/workflows/CI.yaml`)

**Acceptance:** `commit-reveal` exists; `sha2` is absent from `cargo tree --no-default-features`; `cargo deny check bans` is green; the thumb target builds with the feature on.

### Tasks

- [ ] Add the feature and the optional dep; add `sha2` to `deny.toml` bans and `CI.yaml:210` `BANNED`
- [ ] `cargo tree --no-default-features --edges normal | grep -c sha2` is `0`
- [ ] `cargo build --no-default-features --features commit-reveal --target thumbv7em-none-eabihf` green

---

## Story 1: `Contribution` + `Commitment` (`src/seal/commit/commitment.rs`)

**Acceptance:** `commit().verify()` round-trips; a fixed input yields a fixed hex digest; `Debug` of a `Contribution` contains no hex of its bytes; hex round-trips and rejects garbage.

### Tasks

- [ ] Types, tags, `random`/`from_bytes`/`commit`/`verify`/`to_hex`/`from_hex`
- [ ] Tests: `commitment__golden_vector` (`from_bytes([0x11; 32]).commit().to_hex()` pinned at implementation time), `commitment__verify_roundtrip` (prop), `commitment__verify_rejects_other` (prop), `contribution__debug_redacted`, `commitment__hex_roundtrip`, `commitment__from_hex_rejects_garbage`

---

## Story 2: `CombinedSeed` (`src/seal/commit/round.rs`, `derive.rs`)

**Acceptance:** `combine` is pinned by a golden vector; `permutation(n)` is a valid `Permutation` for every `n` in `1..=216`; `permutation(52)` is pinned; different seeds give different permutations.

### Tasks

- [ ] `combine` with the sorted-pairs preimage
- [ ] `derive.rs`: counter-mode byte stream, exact rejection sampling, Fisher–Yates over `Permutation::identity`
- [ ] Tests: `combine__golden_vector`, `combine__order_of_input_is_irrelevant` (sorted internally), `derive__is_valid_permutation` (prop over `n`, seed — via `Permutation::try_from_vec`), `derive__golden_permutation_52`, `derive__differs_per_seed`, `derive__unbiased_smoke` (`#[ignore]`, 10k seeds, loose chi-square on the first element)

---

## Story 3: `ShuffleRound` (`src/seal/commit/round.rs`)

**Acceptance:** the state machine enforces commit-all-then-reveal; mismatched reveals are rejected without mutating; a verifier rebuilding the round from the public transcript reaches the same seed and permutation.

### Tasks

- [ ] `new`/`commit`/`all_committed`/`reveal`/`is_complete`/`seed`
- [ ] Tests: `round__new_rejects_empty_and_duplicates`, `round__commit_unknown_participant_errors`, `round__double_commit_errors`, `round__reveal_before_all_committed_errors`, `round__mismatched_reveal_errors_and_leaves_round_unchanged`, `round__two_party_provably_fair` (dealer + player), `round__any_verifier_reproduces_seed`, `round__reorder_of_participants_changes_seed`

---

## Story 4: Order commitments (`src/seal/commit/pile.rs`)

**Acceptance:** a committed pile verifies only in its exact order; swapping two cards fails; a wrong blind fails; `Pile::shuffled_by_round` preserves the multiset.

### Tasks

- [ ] `commit_permutation` / `verify_permutation` / `commit_pile` / `verify_pile` / `Pile::shuffled_by_round`
- [ ] Tests: `commit_pile__verifies_exact_order`, `commit_pile__detects_swapped_cards`, `commit_pile__wrong_blind_fails`, `commit_permutation__roundtrip`, `commit_permutation__wrong_blind_fails`, `shuffled_by_round__preserves_multiset` (prop), `shuffled_by_round__incomplete_round_errors`

---

## Story 5: Example and docs (`examples/provably_fair.rs`, `README.md`, `CHANGELOG.md`, `.okf/`)

**Acceptance:** the example runs a two-party round end to end and prints the transcript a verifier would need; README and `.okf` feature rows are live.

### Tasks

- [ ] `examples/provably_fair.rs` with `[[example]] required-features = ["commit-reveal", "std"]`
- [ ] Module doc in `src/seal/commit/mod.rs`: the preimage format table, the "a non-revealer aborts, never biases" note, and a worked verifier pseudo-code block
- [ ] README feature row; CHANGELOG `Added`; `.okf/architecture/feature-flags.md` row live; `.okf/log.md`
- [ ] Flip Status rows

---

## Test Plan

| Test | Asserts |
|---|---|
| `commitment__golden_vector` | The `v1` contribution preimage is frozen |
| `commitment__verify_roundtrip` / `_rejects_other` (prop) | Binding: only the committed contribution opens |
| `contribution__debug_redacted` | The secret never reaches a log line |
| `combine__golden_vector` | The `v1` seed preimage is frozen |
| `derive__is_valid_permutation` (prop n, seed) | Every derived permutation is a bijection |
| `derive__golden_permutation_52` | The derivation is frozen — a `rand` bump cannot move it |
| `round__reveal_before_all_committed_errors` | Unbiasability: nobody reveals early |
| `round__mismatched_reveal_errors_and_leaves_round_unchanged` | A cheat is rejected, not absorbed |
| `round__any_verifier_reproduces_seed` | Reproducibility from the public transcript alone |
| `commit_pile__detects_swapped_cards` | Order commitments are order-sensitive |
| `shuffled_by_round__preserves_multiset` (prop) | The pile API integration is a shuffle, not a transform |

**Gold Standard check:** delete the `all_committed` guard in `reveal`, the sort in `combine`, the rejection loop in `derive`, and the `blind` from `commit_pile` in turn; each must redden a named test.

## Key Files

| File | Role |
|---|---|
| `src/seal/commit/mod.rs` | **New.** Protocol doc, preimage table, re-exports |
| `src/seal/commit/commitment.rs` | **New.** `Contribution`, `Commitment` |
| `src/seal/commit/round.rs` | **New.** `ParticipantId`, `ShuffleRound`, `CombinedSeed` |
| `src/seal/commit/derive.rs` | **New.** Counter-mode derivation |
| `src/seal/commit/pile.rs` | **New.** Order commitments, `Pile::shuffled_by_round` |
| `src/common/errors.rs` | Six gated variants |
| `Cargo.toml` | `commit-reveal` feature, `sha2` dep |
| `deny.toml`, `.github/workflows/CI.yaml:210` | `sha2` ban rows |
| `examples/provably_fair.rs` | **New.** Two-party demo |
| `tests/commit_reveal.rs` | **New.** Golden vectors + properties |

## Reuse (do NOT recreate)

- `Permutation` and `Permutation::canonical_bytes` (EPIC-04 Story 3) — the derivation *produces* one and the commitment *hashes* one.
- `Codebook::encode_pile` (EPIC-04 Story 2) — the pile preimage. Do not invent a second byte encoding.
- `Pile::permute` (EPIC-04 Story 3) — `shuffled_by_round` is one line over it.
- `src/basic/types/pile.rs:761-766` — the `rand`-major caveat, quoted verbatim on `to_u64`.
- `tests/properties.rs` — proptest conventions.

## Compatibility

- **Preserves:** everything; a default build is unchanged.
- **Adds:** the `commit-reveal` feature, `src/seal/commit/*`, `Pile::shuffled_by_round`, six gated `CardError` variants.
- **Breaks:** nothing. `CardError` is `#[non_exhaustive]`.
- **Frozen from first release:** the three `v1` preimage formats and the `permutation()` derivation. Changing any of them is a `v2` tag, not an edit.

## Dependencies

- **Built on:** [EPIC-04](./EPIC-04_Sealed_Decks.md) Stories 2–3.
- **Independent of:** [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) (shares only the `sha2` dep).
- **Related:** [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) cites the preimage format for shuffle-proof transcripts; `docs/2026-04-29-seeded-shuffle-design.md` for the reproducibility lineage.

## Verification

```bash
cargo build --no-default-features --features commit-reveal --target thumbv7em-none-eabihf
cargo build --no-default-features --features commit-reveal --target wasm32-unknown-unknown
cargo test --features commit-reveal
cargo test --features commit-reveal,serde
cargo run --features commit-reveal,std --example provably_fair
cargo deny check bans
! cargo tree --no-default-features --edges normal | grep -q '^sha2 '
cargo clippy --all-features --all-targets -- -D warnings -D clippy::pedantic
```

Exit criteria:

1. Two independent `ShuffleRound`s built from the same public transcript yield equal `CombinedSeed` and equal `Permutation`.
2. The three golden vectors pass and are recorded in `tests/commit_reveal.rs` with the inputs that produced them.
3. `sha2` is absent from the pure tree; `cargo deny check bans` green.
4. Every negative test matches a named `CardError` variant.

---

## Gotchas

1. **`to_u64()` + `shuffle_with_seed` is not verifiable across `rand` majors.** It exists because callers will reach for it; the docs must steer them to `permutation()` in the first sentence.
2. **Rejection sampling must be exact.** `% n` biases small indices; the golden vector pins the exact loop, so a "faster" replacement that changes output is caught.
3. **A commitment to low-entropy input is not hiding.** `Contribution::random` is the documented path; `from_bytes` carries the warning.
4. **Do not add `subtle`.** Commitments are public; timing on `==` leaks nothing. Adding a constant-time dep here would be cargo-cult.
5. **Keep `sha2` `default-features = false`.** Another feature enabling `sha2/std` by accident would silently make `commit-reveal` `std`-only; the thumb build in Story 0 is the guard.
6. **Multi-deck piles are fine.** `encode_pile` includes duplicates; the codebook must simply be the pile's own deck type.
7. **`cpufeatures` on wasm32** is a no-op; the wasm build in Verification proves it links.
