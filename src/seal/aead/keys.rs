//! The dealer's secret and the per-slot keys derived from it.
//!
//! Key schedule (frozen `v1`):
//! `K_slot = HKDF-SHA256(ikm = master, salt = deck_name, info = TAG_KEY || u16 BE slot)`.
//! The context is **not** in the key — it binds the ciphertext through the
//! associated data instead — so a token is a function of `(master, deck, slot)`.

use core::fmt;

use hkdf::Hkdf;
use rand::Rng;
use sha2::Sha256;
use zeroize::Zeroizing;

/// HKDF `info` prefix for [`DealKey::slot_key`]. Part of the frozen `v1` format.
pub const TAG_KEY: &[u8] = b"cardpack/seal-aead/v1/key";

/// Dealer-only secret for one deal. Zeroized on drop. `Debug` is redacted.
/// Deliberately no `Copy`, no `Clone`, no `serde`, no `PartialEq`.
pub struct DealKey(Zeroizing<[u8; 32]>);

impl DealKey {
    /// The documented constructor. `rng` **must** be a CSPRNG.
    pub fn random(rng: &mut dyn Rng) -> Self {
        let mut b = Zeroizing::new([0u8; 32]);
        rng.fill_bytes(b.as_mut());
        Self(b)
    }

    /// For tests and for callers with their own key management.
    #[must_use]
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(Zeroizing::new(bytes))
    }

    /// The key — and therefore the reveal token — for one slot of one deck.
    // HKDF-SHA256 `expand` fails only above 255·32 = 8160 output bytes; we ask
    // for 32. The `expect` is a compile-time-true invariant, not a panic path.
    #[allow(clippy::expect_used, clippy::missing_panics_doc)]
    #[must_use]
    pub fn slot_key(&self, deck_name: &str, slot: u16) -> CardKey {
        let hk = Hkdf::<Sha256>::new(Some(deck_name.as_bytes()), self.0.as_ref());
        let mut info = [0u8; TAG_KEY.len() + 2];
        info[..TAG_KEY.len()].copy_from_slice(TAG_KEY);
        info[TAG_KEY.len()..].copy_from_slice(&slot.to_be_bytes());
        let mut okm = Zeroizing::new([0u8; 32]);
        // 32 bytes is far below HKDF-SHA256's 8160-byte limit; cannot fail.
        hk.expand(&info, okm.as_mut())
            .expect("32-byte HKDF-SHA256 expand cannot fail");
        CardKey(okm)
    }
}

impl fmt::Debug for DealKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DealKey(<redacted>)")
    }
}

/// The reveal token for exactly one slot. Zeroized on drop. `Debug` is
/// redacted. Deliberately no `PartialEq` (`==` on key bytes is not
/// constant-time) — compare [`to_bytes`](Self::to_bytes) in tests only.
#[derive(Clone)]
pub struct CardKey(Zeroizing<[u8; 32]>);

impl CardKey {
    /// A copy of the key bytes. This is the value a holder publishes to turn
    /// a card up.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 32] {
        *self.0
    }

    /// Rebuild a token from published bytes.
    #[must_use]
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(Zeroizing::new(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for CardKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CardKey(<redacted>)")
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__aead__keys_tests {
    use super::*;
    use alloc::format;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn deal_key__debug_redacted() {
        let k = DealKey::from_bytes([0xab; 32]);
        assert_eq!(format!("{k:?}"), "DealKey(<redacted>)");
    }

    #[test]
    fn card_key__debug_redacted() {
        let k = CardKey::from_bytes([0xab; 32]);
        assert_eq!(format!("{k:?}"), "CardKey(<redacted>)");
    }

    #[test]
    fn card_key__to_from_bytes_roundtrip() {
        let k = CardKey::from_bytes([0x42; 32]);
        assert_eq!(k.to_bytes(), [0x42; 32]);
        let copy = k.clone();
        drop(k);
        assert_eq!(copy.to_bytes(), [0x42; 32]);
    }

    #[test]
    fn deal_key__random_differs() {
        let mut rng = StdRng::seed_from_u64(1);
        let a = DealKey::random(&mut rng);
        let b = DealKey::random(&mut rng);
        assert_ne!(a.slot_key("x", 0).to_bytes(), b.slot_key("x", 0).to_bytes());
    }

    /// Python: `hkdf_sha256(ikm=b"\x01"*32, salt=b"Standard 52",
    /// info=b"cardpack/seal-aead/v1/key" + (7).to_bytes(2, "big"))`.
    #[test]
    fn slot_key__golden_vector() {
        let k = DealKey::from_bytes([0x01; 32]).slot_key("Standard 52", 7);
        let hex = k
            .to_bytes()
            .iter()
            .fold(alloc::string::String::new(), |mut s, b| {
                use core::fmt::Write;
                let _ = write!(s, "{b:02x}");
                s
            });
        assert_eq!(
            hex,
            "5190d7a16acadb19a221b6ab69ffbea0592a7cbccfc728e5d32168d801bcf73e"
        );
    }
}
