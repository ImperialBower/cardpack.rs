//! Cards you cannot see.
//!
//! This module holds a card's **slot** (its public name), its **order**, and
//! its **value once revealed** — and nothing else. No type here holds
//! ciphertext, no type is generic over a sealing scheme, and no key ever lives
//! in this crate. A type that never contains a secret cannot leak one.
//!
//! Design: `docs/EPIC-04_Sealed_Decks.md`. Real backends (a commit–reveal
//! shuffle, a holder-key AEAD seal) are opt-in features that are deliberately
//! **not** part of `full` — see `.okf/decisions/crypto-features-outside-full.md`.
//!
//! # Notes for a protocol crate building on this surface
//!
//! The full contract is `docs/EPIC-04c_Mental_Poker_Bridge_spec.md`. Three
//! things cardpack deliberately leaves to you:
//!
//! 1. **Name your bijection in your transcript.** `Codebook` order is
//!    `base_vec()` first-occurrence order. Other libraries' card arrays are in
//!    other orders. Put a domain tag such as `b"cardpack/Standard 52/v1"` in
//!    your Fiat–Shamir context, or a transcript replayed against a different
//!    bijection decodes to the wrong cards *silently*.
//! 2. **Token plurality is yours.** `Seal::Token` may be a `Vec` (one share
//!    per player). cardpack does not know how many you need.
//! 3. **Verify inside `unseal`.** `Revealed::reveal_with` hands your backend
//!    raw tokens and admits whatever `unseal` returns.
//!
//! And one warning: `Revealed::reveal` trusts the caller. A referee that
//! accepts it from an untrusted peer has skipped its own protocol.

pub mod adapter;
#[cfg(any(test, feature = "seal-test-double"))]
pub mod plaintext;
pub mod revealed;
pub mod slot;
pub mod slot_pile;

pub use adapter::Seal;
#[cfg(any(test, feature = "seal-test-double"))]
pub use plaintext::{PlainSealError, PlainToken, PlaintextSeal, seal_roundtrip};
pub use revealed::{Revealed, SealError};
pub use slot::SlotId;
pub use slot_pile::{SlotAudit, SlotPile};
