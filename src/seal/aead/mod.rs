//! Holder-key seal: the first real [`Seal`](crate::seal::Seal) backend.
//!
//! Behind the `seal-aead` feature (`chacha20poly1305`, `hkdf`, `sha2`,
//! `zeroize`; all `no_std`). Design: `docs/EPIC-04b_Holder_Key_Seal.md`.
//!
//! **One trusted dealer** seals every card under its own key, derived from a
//! per-deal master. A holder turns up one card by publishing one 32-byte
//! token; anyone with the public sealed bytes verifies it through
//! [`Revealed::reveal_with`](crate::seal::Revealed::reveal_with). Revealing
//! one card exposes nothing about any other.
//!
//! # ⚠ The RNG must be a CSPRNG
//!
//! Every [`seal`](crate::seal::Seal::seal) draws a fresh 24-byte nonce from
//! the caller's `rng`, and [`HolderKeySeal::deal`] draws the shuffle from it
//! too. `rand::rng()` (under `std`) and `StdRng` are fine; a `SmallRng` or a
//! constant is not. Nonce reuse under one key is the only way this scheme
//! breaks.
//!
//! # The `v1` format — frozen
//!
//! | Value | Definition |
//! |---|---|
//! | `K_slot` (the token) | `HKDF-SHA256(ikm = master, salt = deck_name, info = b"cardpack/seal-aead/v1/key" ‖ u16 BE slot)` |
//! | `AD` | `b"cardpack/seal-aead/v1/ad" ‖ u16 BE name_len ‖ deck_name ‖ u16 BE slot ‖ context` |
//! | plaintext | the card's `Ordinal`, `u16` BE (2 bytes) |
//! | cipher | XChaCha20-Poly1305, fresh random 24-byte nonce per seal |
//! | [`SealedBytes`] | `nonce(24) ‖ ct(2) ‖ tag(16)` = 42 bytes |
//!
//! The context is in the AD, not the key: a token is a function of
//! `(master, deck, slot)` only. Golden vectors from an independent Python
//! implementation pin every row (`tests/seal_aead.rs`).
//!
//! # The three plain values
//!
//! * [`SlotPile`](crate::seal::SlotPile) — the shoe: **order** of slot names.
//! * [`Custody`] — the dealer's ledger: **bytes** per slot. Public. Not a pile.
//! * [`Revealed`](crate::seal::Revealed) — **values** per slot, once opened.
//!
//! The scheme, [`HolderKeySeal`], lives inside none of them (EPIC-04
//! decision 2). It runs in **dealer** mode (holds the [`DealKey`]; can seal
//! and mint tokens) or **verifier** mode (no secret; can only `unseal`).
//!
//! # The holder flow
//!
//! ```
//! use cardpack::prelude::*;
//! use rand::SeedableRng;
//! let mut rng = rand::rngs::StdRng::seed_from_u64(1); // a CSPRNG
//!
//! // Dealer.
//! let dealer = HolderKeySeal::<Standard52>::dealer(DealKey::random(&mut rng), b"table-7/hand-12");
//! let (mut shoe, custody) = dealer.deal(&Standard52::deck(), &mut rng)?;
//! let hole = shoe.draw(2).unwrap();                       // two SlotIds, no values
//! let tokens = dealer.tokens_for(hole.slots().iter().copied())?;
//!
//! // Holder turns one card up by publishing (slot, token). A spectator verifies:
//! let (slot, token) = (tokens[0].0, tokens[0].1.clone());
//! let spectator = HolderKeySeal::<Standard52>::verifier(b"table-7/hand-12");
//! let mut revealed = Revealed::<Standard52>::new();
//! let card = revealed.reveal_with(slot, custody.get(slot).unwrap(), &spectator, &token)?;
//! assert!(Standard52::deck().cards().contains(&card));
//! assert!(!revealed.is_revealed(hole.slots()[1]));         // the other card stays sealed
//! # Ok::<(), Box<dyn core::error::Error>>(())
//! ```
//!
//! # Errors are deliberately blunt
//!
//! Wrong token, wrong slot, wrong context, wrong deck, tampered bytes — all
//! are [`AeadSealError::Unseal`]. Telling them apart would be an oracle.
//!
//! # `RecipientSeal` — the public-key variant (not implemented)
//!
//! Sealing *to a holder's static public key*, so the dealer never hands out
//! tokens: `x25519` ephemeral key per card, `K_slot = HKDF-SHA256(x25519(esk,
//! holder_pk), salt = deck_name, info = tag ‖ slot)`, the same AEAD body and
//! AD; `Sealed = { epk[32], body: SealedBytes }` (74 bytes); `Token` stays a
//! [`CardKey`], which the **holder** derives from `(holder_sk, epk)` and
//! publishes to turn the card up. Verifiers, `Custody`, `SlotPile`, and
//! `Revealed` are unchanged. That is the whole "only the holder can read it"
//! story in card-library terms; custody transfer and consensus are not.
//! Planned behind a future `seal-pk` feature; not built.

pub mod error;
pub mod holder_key_seal;
pub mod keys;
pub mod sealed_bytes;

pub use error::AeadSealError;
pub use holder_key_seal::{HolderKeySeal, TAG_AD};
pub use keys::{CardKey, DealKey, TAG_KEY};
pub use sealed_bytes::{Custody, SealedBytes};
