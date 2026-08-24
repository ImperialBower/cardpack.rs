# EPIC-04b: Holder-Key Seal (HKS)

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Child of [EPIC-04 Sealed Decks](./EPIC-04_Sealed_Decks.md); needs its Stories 1 and 4–5 (`Codebook`, `Seal<D>`, `SealedCard`, `SealedPile`, the `seal_roundtrip` helper) landed first. "Default features green" **and** `cargo deny check bans` are preconditions for every story. Nothing has landed as of `main` @ `1c14440`, 2026-08-24.

**Goal:** The first **real `Seal<D>` backend**. Every card is encrypted under its own key, derived from a per-deal master key, so a **holder turns up one card by publishing one 32-byte token** — and anyone holding the public sealed bytes can verify it. This is the "only the holder can read it until it is turned up" tier the NFT framing describes, without a chain: the sealed bytes are the public artefact, the token is the proof of ownership, and revealing one card exposes nothing about any other.

**Architecture:** A new `src/seal/aead/` module behind the `seal-aead` feature. Per-card keys come from HKDF-SHA256 over a `DealKey`; the payload is the card's `Ordinal` (two bytes) encrypted with XChaCha20-Poly1305 under a fresh 24-byte nonce, with the `SlotId`, deck name, and a caller-supplied context bound in as associated data. `HolderKeySeal<D>` runs in **dealer** mode (holds the master, can seal and mint tokens) or **verifier** mode (no secret; can only `unseal` with a token). The sealed form is a fixed 42-byte `SealedBytes`, `Copy`, no heap. A public-key variant (`RecipientSeal`, x25519) is designed here but not built.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std, RustCrypto `chacha20poly1305 0.10` (in-place detached API, no `alloc`, no `getrandom`), `hkdf 0.12`, `sha2 0.10` (shared with [04a](./EPIC-04a_Commit_Reveal_Shuffle.md)), `zeroize 1` — all `default-features = false`; nonces from the caller's `rand::RngCore`. All four banned from the pure tree in CI.

---

## Context

- EPIC-04 ships the boundary — `Seal<D>` (`src/seal/seal.rs`), `SealedCard<D, S>`, `SealedPile<D, S>` — and one implementation, `PlaintextSeal`, which is deliberately no security at all. Without a real backend, the kernel's "a deck it cannot read" claim is only proven for the plumbing.
- The plaintext to protect is tiny: an `Ordinal` (EPIC-04 Story 1) is `u16`, so a card is two bytes. That is what makes a fixed-width, allocation-free sealed form possible.
- The crate has no cipher, no KDF, no key type, and no zeroization anywhere. `rand 0.10` is present with `std_rng` unconditional (`Cargo.toml:41-46`); `RngCore::fill_bytes` is the nonce source.
- `pkcore` EPIC-79b names an AEAD blob as one of the shapes `CardSeal::Sealed` might take ("64 bytes of Ristretto ciphertext, an AEAD blob, or (in tests) a `Card`"). This EPIC is that blob, for the generic kernel.

**What this EPIC does NOT do:**

- **No public-key crypto ships.** `RecipientSeal` (x25519 + the same AEAD body) is designed in this document so the shape is known to fit, and deferred to a future `seal-pk` feature.
- **No on-chain anything.** "NFT-style" here means *holder-only readability with a public artefact and a publishable proof*. Custody, minting, and transfer are out of scope for a card library.
- **No plaintext zeroization.** `Card: Copy`. Keys are zeroized; revealed cards are not (EPIC-04 gotcha 3).
- **No multi-party, no threshold.** One dealer holds the master. Removing the trusted dealer is [04c](./EPIC-04c_Mental_Poker_Bridge_spec.md).
- **No `aes-gcm`.** Rejected: needs AES-NI for constant time on x86 and is table-based on thumb; XChaCha20-Poly1305 is constant-time in pure software everywhere the crate builds.

---

## Status

Status as of `main` @ `1c14440`, **2026-08-24**. Nothing has landed.

| Component | Status |
|---|---|
| `seal-aead` feature + four deps + deny/CI ban rows | Planned |
| `DealKey` / `CardKey` (`Zeroizing`, redacted `Debug`) | Planned |
| `SealedBytes` (42 bytes, `Copy`, serde-gated) | Planned |
| `HolderKeySeal<D>` dealer / verifier modes, `token_for` | Planned |
| `impl Seal<D> for HolderKeySeal<D>` | Planned |
| `AeadSealError` (`#[non_exhaustive]`, single `Unseal` variant — no oracle) | Planned |
| `SealedPile<D, HolderKeySeal<D>>` integration | Planned |
| Golden vector with a constant-byte test RNG | Planned |
| `examples/holder_seal.rs` | Planned |
| `tests/seal_aead.rs` | Planned |
| `RecipientSeal` design section (`seal-pk`, future) | Planned (design only) |
| Docs / README / CHANGELOG / `.okf/` | Planned |

---

## Goals

- **One token, one card.** `token_for(slot)` is a 32-byte key that opens exactly that slot. Publishing it reveals that card and nothing else.
- **Verifiers hold no secret.** `HolderKeySeal::verifier(context)` can `unseal` with a published token and cannot seal or mint.
- **No oracle.** Every unseal failure — wrong token, wrong slot, wrong context, tampered bytes — is the same `AeadSealError::Unseal`.
- **Allocation-free hot path.** `seal`/`unseal` work on fixed buffers; the thumb build proves it.
- **Keys die with their owners.** `DealKey` and `CardKey` are `Zeroizing`; neither derives `Copy`, `Serialize`, or a readable `Debug`.
- **Frozen wire format.** `SealedBytes` is `nonce(24) || ct(2) || tag(16)`, pinned by a golden vector.

## Scope

1. All new API is `#[cfg(feature = "seal-aead")]`.
2. No cipher/KDF/digest type appears in any public signature. `SealedBytes`, `CardKey`, `DealKey` are the only public representations.
3. `K_slot = HKDF-SHA256(ikm = master, salt = deck_name, info = b"cardpack/seal-aead/v1/key" || u16 BE slot)`.
4. `AD = b"cardpack/seal-aead/v1/ad" || u16 BE name_len || deck_name || u16 BE slot || context`.
5. Plaintext = `Ordinal` as `u16 BE`. `seal` on a card with no ordinal is `Err(CardNotInDeck)`.
6. The nonce is 24 fresh bytes from the caller's `RngCore` on **every** `seal`. Never deterministic, never zero.
7. `unseal` in verifier mode recomputes `AD` from `(slot, context)` and decrypts under the *token*; it never touches a master key.
8. `CardKey` does not implement `PartialEq`. Tests compare `to_bytes()`.
9. Do **not** enable `chacha20poly1305/rand_core` — it is `rand_core 0.6`, cardpack is on `rand 0.10`; mixing is a version-mismatch trap.

---

## Domain

**Things.**

| Thing | Type |
|---|---|
| The dealer's secret for one deal | `DealKey` |
| The key for exactly one slot — and therefore the reveal token | `CardKey` |
| A sealed card's public bytes | `SealedBytes` (42) |
| The scheme, with or without the secret | `HolderKeySeal<D>` |
| What went wrong, without saying which | `AeadSealError` |

**Business Requirements.** (a) *Holder-only* — without the slot's token the ordinal is unrecoverable. (b) *Selective* — one token opens one slot. (c) *Verifiable* — a third party with the token and the public bytes gets the same card. (d) *Bound* — a sealed payload cannot be moved to another slot, deck, or context and still open.

**Business Logic.** AEAD confidentiality gives (a); per-slot HKDF keys give (b); verifier mode gives (c); `AD` gives (d).

---

## Design decisions (settled)

1. **Per-card keys via HKDF, not one key per holder.** "Turn up one card" must not expose the holder's other cards; a per-holder key would. HKDF means the dealer stores no per-card table — `token_for` recomputes.
2. **The token *is* the per-card key.** Rejected: random per-card keys generated inside `seal` — the trait returns only `Sealed`, so the token would have nowhere to go.
3. **XChaCha20-Poly1305 over AES-GCM.** Constant-time in software on every target the crate builds for; 24-byte nonce makes random nonces safe at any deal volume.
4. **Random nonce, not deterministic.** Per-slot keys make a zero nonce *almost* safe, but re-sealing the same slot under one master (a redeal) would reuse `(key, nonce)`. 24 random bytes remove the footgun for 24 bytes of cost.
5. **One `Unseal` error.** Distinguishing "bad tag" from "bad AD" from "bad slot" is an oracle. One variant, one message.
6. **`SealedBytes` is `Copy`** — it is public. **Keys are never `Copy`** and never `Serialize`.
7. **Backend owns `AeadSealError`.** `CardError` stays crypto-free (EPIC-04 decision 7).

---

## Design

### Keys

`src/seal/aead/keys.rs` (new):

```rust
/// Dealer-only secret for one deal. Zeroized on drop. `Debug` is redacted. No serde.
pub struct DealKey(Zeroizing<[u8; 32]>);
impl DealKey {
    pub fn random(rng: &mut dyn RngCore) -> Self;
    pub fn from_bytes(b: [u8; 32]) -> Self;
}

/// The reveal token for exactly one slot. Zeroized on drop. `Debug` redacted.
/// Deliberately no `PartialEq` — compare `to_bytes()` in tests.
#[derive(Clone)]
pub struct CardKey(Zeroizing<[u8; 32]>);
impl CardKey {
    pub fn to_bytes(&self) -> [u8; 32];
    pub fn from_bytes(b: [u8; 32]) -> Self;
}
```

### `SealedBytes`

`src/seal/aead/sealed_bytes.rs` (new):

```rust
/// `nonce(24) || ct(2) || tag(16)`. Fixed width, no heap, public.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]          // Debug = hex
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SealedBytes { nonce: [u8; 24], ct: [u8; 2], tag: [u8; 16] }
impl SealedBytes {
    pub const LEN: usize = 42;
    pub fn to_bytes(&self) -> [u8; 42];
    pub fn from_bytes(b: [u8; 42]) -> Self;
}
```

### `HolderKeySeal<D>`

`src/seal/aead/holder_key_seal.rs` (new):

```rust
pub struct HolderKeySeal<D: DeckedBase> {
    master: Option<DealKey>,      // None in verifier mode
    context: Vec<u8>,
    codebook: Codebook<D>,
    deck_name: String,
}

impl<D: DeckedBase> HolderKeySeal<D> {
    pub fn dealer(master: DealKey, context: impl AsRef<[u8]>) -> Self;
    /// Can `unseal` with a token. Cannot seal, cannot mint tokens.
    pub fn verifier(context: impl AsRef<[u8]>) -> Self;
    pub fn is_dealer(&self) -> bool;
    /// `NoMasterKey` in verifier mode.
    pub fn token_for(&self, slot: SlotId) -> Result<CardKey, AeadSealError>;
    pub fn tokens_for(&self, slots: impl IntoIterator<Item = SlotId>)
        -> Result<Vec<(SlotId, CardKey)>, AeadSealError>;
}

impl<D: DeckedBase> Seal<D> for HolderKeySeal<D> {
    type Sealed = SealedBytes;
    type Token  = CardKey;
    type Error  = AeadSealError;

    /// ordinal → [u8; 2]; fresh 24-byte nonce from `rng`;
    /// `encrypt_in_place_detached(nonce, ad, &mut buf)` under `K_slot`.
    fn seal(&self, card: Card<D>, slot: SlotId, rng: &mut dyn RngCore)
        -> Result<SealedBytes, AeadSealError>;

    /// `decrypt_in_place_detached` under *token*; AD recomputed from `(slot, context)`;
    /// ordinal → card. Never touches `master`.
    fn unseal(&self, sealed: &SealedBytes, slot: SlotId, token: &CardKey)
        -> Result<Card<D>, AeadSealError>;
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum AeadSealError {
    #[error("verifier-mode seal has no master key")]
    NoMasterKey,
    #[error("card `{0}` is not in the deck")]
    CardNotInDeck(String),
    #[error("unseal failed: bad token, wrong slot, or wrong context")]
    Unseal,
    #[error("authentic payload decoded to ordinal {0}, which is out of range")]
    InvalidOrdinal(u16),
}
```

`InvalidOrdinal` cannot occur for bytes this scheme produced — the AEAD tag covers the plaintext — but the decoder is total and the variant records that fact rather than `unwrap`ping.

### `RecipientSeal<D>` — the public-key variant (design only, future `seal-pk`)

Sealing *to a holder's static public key* so the dealer never needs to hand out tokens:

- `x25519-dalek 2` (`default-features = false`). Dealer generates an ephemeral keypair per card; `K_slot = HKDF-SHA256(x25519(esk, holder_pk), salt = deck_name, info = tag || slot)`; same XChaCha20-Poly1305 body and `AD`.
- `Sealed = { epk: [u8; 32], body: SealedBytes }` (74 bytes). `Token = CardKey`, exactly as today — the **holder** derives `K_slot` from `(holder_sk, epk)` and publishes it to turn the card up.
- Verifiers `unseal` exactly as in HKS. `SealedPile`, any ledger, and 04a transcripts are unchanged; only `seal` and who can mint the token differ.

That is the whole "NFT-style" story in card-library terms: a public artefact per card, a private key per holder, a publishable proof per reveal. What a chain adds — custody, transfer, consensus — is not a card-library concern.

### `Cargo.toml`

```toml
seal-aead = ["dep:chacha20poly1305", "dep:hkdf", "dep:sha2", "dep:zeroize"]
chacha20poly1305 = { version = "0.10", default-features = false, optional = true }   # no `alloc`, no `getrandom`, no `rand_core`
hkdf    = { version = "0.12", default-features = false, optional = true }
sha2    = { version = "0.10", default-features = false, optional = true }            # shared with commit-reveal
zeroize = { version = "1",    default-features = false, optional = true }            # no `derive` — wrap with Zeroizing
```

Implies no `std`. `hkdf 0.12` ↔ `sha2 0.10` ↔ `digest 0.10` must line up; if RustCrypto 0.11 has shipped at implementation time, move all three together. Add `chacha20poly1305`, `hkdf`, `sha2` to `deny.toml` `[bans].deny` and the `BANNED` regex (`.github/workflows/CI.yaml:210`).

### Module layout

```
src/seal/aead/mod.rs              key schedule, AD layout, wire-format table
src/seal/aead/keys.rs             DealKey, CardKey
src/seal/aead/sealed_bytes.rs     SealedBytes
src/seal/aead/holder_key_seal.rs  HolderKeySeal<D> + Seal impl
src/seal/aead/error.rs            AeadSealError
examples/holder_seal.rs           required-features = ["seal-aead", "std"]
tests/seal_aead.rs                generic law across all decks, negatives, golden vector
```

---

## Story 0: Feature, dependencies, bans (`Cargo.toml`, `deny.toml`, `.github/workflows/CI.yaml`)

**Acceptance:** `seal-aead` builds on `thumbv7em-none-eabihf` and `wasm32-unknown-unknown`; none of the four crates is in the pure tree; `cargo deny check bans` green.

### Tasks

- [ ] Feature + four optional deps; ban rows in `deny.toml` and `CI.yaml:210`
- [ ] `cargo build --no-default-features --features seal-aead --target thumbv7em-none-eabihf` green (proves no_std **and** no alloc on the hot path)
- [ ] `cargo tree --no-default-features --edges normal | grep -Ec 'chacha20poly1305|hkdf|sha2|zeroize'` is `0`

---

## Story 1: Keys (`src/seal/aead/keys.rs`)

**Acceptance:** `DealKey`/`CardKey` are zeroized, redacted, non-`Copy`, non-serde; `token_for` is deterministic per `(master, deck, slot)` and differs across slots; verifier mode cannot mint.

### Tasks

- [ ] `DealKey`, `CardKey`, HKDF key schedule
- [ ] Tests: `deal_key__debug_redacted`, `card_key__debug_redacted`, `token_for__deterministic`, `token_for__differs_per_slot`, `token_for__differs_per_deck` (same master, `French` vs `Skat`), `verifier__cannot_mint_tokens`

---

## Story 2: `SealedBytes` (`src/seal/aead/sealed_bytes.rs`)

**Acceptance:** 42 bytes round-trip; `Debug` is hex; serde round-trips under `serde,seal-aead`.

### Tasks

- [ ] Type + `to_bytes`/`from_bytes`/`Debug`
- [ ] Tests: `sealed_bytes__to_from_bytes_roundtrip`, `sealed_bytes__len_is_42`, `sealed_bytes__serde_roundtrip`

---

## Story 3: The `Seal` impl (`src/seal/aead/holder_key_seal.rs`)

**Acceptance:** the generic round-trip law holds for every shipped deck; every wrong-thing is `Err(Unseal)`; nonces are fresh; the golden vector pins the wire format.

### Tasks

- [ ] `dealer`/`verifier`/`seal`/`unseal`
- [ ] Tests: `hks__roundtrip_law` (the EPIC-04 `seal_roundtrip` helper over all 14 decks via a macro), `hks__wrong_token_errors`, `hks__token_for_other_slot_errors`, `hks__wrong_context_errors`, `hks__wrong_deck_errors` (bytes from `HolderKeySeal<French>` through `HolderKeySeal<Skat>`), `hks__tampered_ciphertext_errors` (prop: any single flipped bit of 42 → `Err(Unseal)`), `hks__nonce_is_fresh` (two seals of the same card/slot differ in `nonce` and `tag`), `hks__verifier_can_unseal`, `hks__verifier_cannot_seal`, `hks__blank_card_errors` (`CardNotInDeck`), `hks__golden_vector` (master `[0x01; 32]`, context `"test"`, Standard52, slot 7, A♠, a test-only `RngCore` yielding `[0x02; 24]` → pinned 42 bytes)

---

## Story 4: `SealedPile` integration (`tests/seal_aead.rs`)

**Acceptance:** a shuffled-then-sealed Standard52 reveals, slot by slot with `tokens_for`, to a permutation of the deck; the serialized sealed pile contains no plaintext.

### Tasks

- [ ] Tests: `hks__seal_shuffled_then_reveal_all_is_permutation_of_deck` (`Pile::same`), `hks__sealed_pile_serde_wire_has_no_plaintext` (`serde_json` string contains no `"index"`, no rank/suit glyphs, no ordinal-looking small integers beyond `slot`), `hks__take_slot_then_reveal_with_published_token` (the holder flow)

---

## Story 5: Example and docs (`examples/holder_seal.rs`, `README.md`, `CHANGELOG.md`, `.okf/`)

**Acceptance:** the example deals two hands, publishes one token, and shows a verifier turning up exactly that card; docs carry the key schedule and the `RecipientSeal` section.

### Tasks

- [ ] `examples/holder_seal.rs` with `[[example]] required-features = ["seal-aead", "std"]`
- [ ] `src/seal/aead/mod.rs` doc: key schedule, `AD` layout, wire format, the CSPRNG requirement, the `RecipientSeal` design
- [ ] README row; CHANGELOG `Added`; `.okf/architecture/feature-flags.md` row live; `.okf/log.md`
- [ ] Flip Status rows

---

## Test Plan

| Test | Asserts |
|---|---|
| `hks__roundtrip_law` (all decks) | `unseal(seal(c, s, rng), s, token_for(s)) == c` |
| `hks__wrong_token_errors` / `_token_for_other_slot_errors` | Selectivity: one token, one slot |
| `hks__wrong_context_errors` / `_wrong_deck_errors` | Binding via `AD` |
| `hks__tampered_ciphertext_errors` (prop) | Integrity: every bit of the 42 is covered |
| `hks__nonce_is_fresh` | Randomized sealing; catches a constant-RNG mistake |
| `hks__verifier_can_unseal` / `_cannot_seal` | The two modes are what they claim |
| `hks__golden_vector` | The wire format is frozen |
| `keys__debug_redacted` | No key reaches a log line |
| `hks__sealed_pile_serde_wire_has_no_plaintext` | The public artefact is actually opaque |

**Gold Standard check:** remove `slot` from `AD`, remove `deck_name` from the HKDF salt, and replace the nonce with zeros in turn; each must redden a named test.

## Key Files

| File | Role |
|---|---|
| `src/seal/aead/mod.rs` | **New.** Protocol doc, re-exports |
| `src/seal/aead/keys.rs` | **New.** `DealKey`, `CardKey`, key schedule |
| `src/seal/aead/sealed_bytes.rs` | **New.** `SealedBytes` |
| `src/seal/aead/holder_key_seal.rs` | **New.** `HolderKeySeal<D>`, `Seal` impl |
| `src/seal/aead/error.rs` | **New.** `AeadSealError` |
| `Cargo.toml` | `seal-aead` feature, four deps |
| `deny.toml`, `.github/workflows/CI.yaml:210` | Ban rows |
| `examples/holder_seal.rs` | **New.** Holder flow demo |
| `tests/seal_aead.rs` | **New.** Law, negatives, golden, integration |

## Reuse (do NOT recreate)

- `Codebook<D>` (EPIC-04 Story 1) — the plaintext *is* the ordinal. Do not encode `BasicCard` fields.
- `Seal<D>`, `SlotId`, `SealedCard`, `SealedPile` (EPIC-04 Stories 4–5) — this EPIC adds an `impl`, no new container.
- `seal_roundtrip` (EPIC-04 Story 4) — the conformance law; run it, do not rewrite it.
- `DeckedBase::deck_name` (`src/basic/types/traits.rs:30`) — the salt and the `AD` domain tag.
- `rand::RngCore::fill_bytes` — the nonce source. Do not enable `chacha20poly1305/rand_core`.

## Compatibility

- **Preserves:** everything; a default build is unchanged; `CardError` is untouched.
- **Adds:** the `seal-aead` feature, `src/seal/aead/*`, `AeadSealError`.
- **Breaks:** nothing.
- **Frozen from first release:** the HKDF `info`/salt layout, the `AD` layout, and the 42-byte wire format (`v1` tags).

## Dependencies

- **Built on:** [EPIC-04](./EPIC-04_Sealed_Decks.md) Stories 1, 4, 5.
- **Independent of:** [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md) (shares only the `sha2` dep).
- **Related:** [EPIC-04c](./EPIC-04c_Mental_Poker_Bridge_spec.md) cites HKS as the reference `Seal<D>` implementation.

## Verification

```bash
cargo build --no-default-features --features seal-aead --target thumbv7em-none-eabihf
cargo build --no-default-features --features seal-aead --target wasm32-unknown-unknown
cargo test --features seal-aead
cargo test --features seal-aead,serde
cargo test --features crypto,full            # both backends + everything: no feature conflict
cargo run --features seal-aead,std --example holder_seal
cargo deny check bans
! cargo tree --no-default-features --edges normal | grep -Eq '^(chacha20poly1305|hkdf|sha2|zeroize) '
cargo clippy --all-features --all-targets -- -D warnings -D clippy::pedantic
```

Exit criteria:

1. `hks__roundtrip_law` passes for all `DeckKind::all()` decks and all 14 marker types.
2. The golden vector passes and is recorded with its inputs.
3. The thumb build with `seal-aead` is green — no `alloc` on the seal/unseal path.
4. All four crates are absent from the pure tree; `cargo deny check bans` green.
5. `RecipientSeal` is documented in `src/seal/aead/mod.rs` as *not implemented*.

---

## Gotchas

1. **Nonce reuse is the only way this design breaks.** The caller's `RngCore` must be a CSPRNG (`StdRng` from `rand` is; a `SmallRng` is not). Say so at the top of the module doc. `hks__nonce_is_fresh` catches a constant RNG in tests, not in production.
2. **Don't derive `PartialEq` on `CardKey`.** `==` on `Zeroizing` is not constant-time. Tokens are compared only in tests; force `to_bytes()` there.
3. **`Copy` on `SealedBytes` is fine; `Copy` on a key never is.** `Zeroizing` on a `Copy` type is meaningless.
4. **`Card: Copy` means the ordinal sits on the stack unzeroized after `unseal`.** Out of scope, stated (EPIC-04 gotcha 3).
5. **`AD` includes `deck_name`, which is public anyway.** A two-byte plaintext leaks nothing about *which* deck beyond that.
6. **One error variant is deliberate.** Someone will want `Unseal::BadTag` vs `Unseal::BadSlot` for debugging. That is an oracle. Debug with a test, not a variant.
7. **Version lockstep.** `hkdf`, `sha2`, `chacha20poly1305` share `digest`/`crypto-common`; a partial bump splits the tree and `cargo deny` warns on duplicates. Bump together.
8. **Construction allocates; the hot path does not.** `HolderKeySeal::dealer` builds a `Codebook` and a `String`. That is fine — it is per deal, not per card — but the thumb build only proves the *hot path*, so keep `seal`/`unseal` on fixed buffers.
