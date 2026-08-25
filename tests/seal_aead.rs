//! Holder-key seal (EPIC-04b): properties, negatives, and the wire check.
//!
//! The golden vector lives in `src/seal/aead/holder_key_seal.rs`
//! (`hks__golden_vector`); it was produced by an independent Python
//! implementation (pycryptodome XChaCha20-Poly1305 + a hand-written
//! HKDF-SHA256), reproduced here:
//!
//! ```python
//! import hashlib, hmac
//! from Crypto.Cipher import ChaCha20_Poly1305
//! TAG_KEY = b"cardpack/seal-aead/v1/key"
//! TAG_AD  = b"cardpack/seal-aead/v1/ad"
//!
//! def hkdf_sha256(ikm, salt, info, length=32):
//!     prk = hmac.new(salt, ikm, hashlib.sha256).digest()
//!     okm, t, i = b"", b"", 1
//!     while len(okm) < length:
//!         t = hmac.new(prk, t + info + bytes([i]), hashlib.sha256).digest()
//!         okm += t; i += 1
//!     return okm[:length]
//!
//! def slot_key(master, deck_name, slot):            # DealKey::slot_key
//!     return hkdf_sha256(master, deck_name.encode(), TAG_KEY + slot.to_bytes(2, "big"))
//!
//! def ad(deck_name, slot, context):
//!     n = deck_name.encode()
//!     return TAG_AD + len(n).to_bytes(2, "big") + n + slot.to_bytes(2, "big") + context
//!
//! def seal(master, deck_name, slot, context, ordinal, nonce):   # HolderKeySeal::seal
//!     c = ChaCha20_Poly1305.new(key=slot_key(master, deck_name, slot), nonce=nonce)
//!     c.update(ad(deck_name, slot, context))
//!     ct, tag = c.encrypt_and_digest(ordinal.to_bytes(2, "big"))
//!     return nonce + ct + tag                                    # 42 bytes
//!
//! master = bytes([0x01]) * 32
//! print(slot_key(master, "Standard 52", 7).hex())                # slot_key__golden_vector
//! print(seal(master, "Standard 52", 7, b"test", 0, bytes([0x02]) * 24).hex())  # hks__golden_vector
//! ```
//!
//! Skipped on `wasm32-unknown-unknown` because proptest's transitive
//! `wait-timeout` crate is unix-only.

#![cfg(all(feature = "seal-aead", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

use cardpack::prelude::*;
use proptest::prelude::*;
use rand::SeedableRng;
use rand::rngs::StdRng;

fn master() -> impl Strategy<Value = DealKey> {
    any::<[u8; 32]>().prop_map(DealKey::from_bytes)
}

proptest! {
    /// The round-trip law under random masters, contexts, slots, and cards.
    #[test]
    fn hks__roundtrip(m in master(), ctx in any::<Vec<u8>>(), slot in any::<u16>(), i in 0usize..52, seed in any::<u64>()) {
        let scheme = HolderKeySeal::<Standard52>::dealer(m, &ctx);
        let card = Standard52::deck().cards()[i];
        let slot = SlotId::new(slot);
        let sealed = scheme.seal(card, slot, &mut StdRng::seed_from_u64(seed)).unwrap();
        let verifier = HolderKeySeal::<Standard52>::verifier(&ctx);
        prop_assert_eq!(verifier.unseal(&sealed, slot, &scheme.token_for(slot).unwrap()).unwrap(), card);
    }

    /// Integrity: any single flipped bit anywhere in the 42 bytes is rejected.
    #[test]
    fn hks__tampered_bit_errors(m in master(), bit in 0usize..(42 * 8), seed in any::<u64>()) {
        let scheme = HolderKeySeal::<Standard52>::dealer(m, b"prop");
        let slot = SlotId::new(1);
        let sealed = scheme.seal(Standard52::deck().cards()[5], slot, &mut StdRng::seed_from_u64(seed)).unwrap();
        let mut b = sealed.to_bytes();
        b[bit / 8] ^= 1 << (bit % 8);
        let token = scheme.token_for(slot).unwrap();
        prop_assert_eq!(scheme.unseal(&SealedBytes::from_bytes(b), slot, &token), Err(AeadSealError::Unseal));
    }

    /// Selectivity: a token for slot `a` never opens slot `b`.
    #[test]
    fn hks__token_is_slot_specific(m in master(), a in any::<u16>(), b in any::<u16>(), seed in any::<u64>()) {
        prop_assume!(a != b);
        let scheme = HolderKeySeal::<Standard52>::dealer(m, b"prop");
        let sealed = scheme.seal(Standard52::deck().cards()[7], SlotId::new(a), &mut StdRng::seed_from_u64(seed)).unwrap();
        let wrong = scheme.token_for(SlotId::new(b)).unwrap();
        prop_assert_eq!(scheme.unseal(&sealed, SlotId::new(a), &wrong), Err(AeadSealError::Unseal));
    }

    /// `deal` is a shuffle: revealing every slot gives the same multiset.
    #[test]
    fn hks__deal_preserves_multiset(m in master(), seed in any::<u64>()) {
        let scheme = HolderKeySeal::<Pinochle>::dealer(m, b"prop");
        let deck = Pinochle::deck();
        let (shoe, custody) = scheme.deal(&deck, &mut StdRng::seed_from_u64(seed)).unwrap();
        let mut revealed = Revealed::<Pinochle>::new();
        for &slot in shoe.slots() {
            revealed.reveal_with(slot, custody.get(slot).unwrap(), &scheme, &scheme.token_for(slot).unwrap()).unwrap();
        }
        prop_assert!(deck.same(&revealed.pile_for(shoe.slots()).unwrap()));
    }
}

/// What a spectator receives — `Custody` + `SlotPile` — contains no plaintext:
/// no card index strings, no rank/suit glyphs, and no `Card`/`BasicCard`
/// field names.
#[cfg(feature = "serde")]
#[test]
fn hks__wire_has_no_plaintext() {
    let scheme = HolderKeySeal::<Standard52>::dealer(DealKey::from_bytes([7; 32]), b"wire");
    let (shoe, custody) = scheme
        .deal(&Standard52::deck(), &mut StdRng::seed_from_u64(1))
        .unwrap();
    let wire = format!(
        "{}{}",
        serde_json::to_string(&custody).unwrap(),
        serde_json::to_string(&shoe).unwrap()
    );
    for forbidden in [
        "rank", "suit", "weight", "index", "♠", "♥", "♦", "♣", "AS", "KH",
    ] {
        assert!(!wire.contains(forbidden), "wire leaks `{forbidden}`");
    }
    assert!(
        wire.contains("nonce"),
        "sanity: custody serializes as sealed bytes"
    );
}
