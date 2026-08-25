//! The public artefacts: a sealed card's bytes and the dealer's ledger.

use alloc::vec::Vec;
use core::fmt;

use crate::common::errors::CardError;
use crate::seal::slot::SlotId;

/// A sealed card: `nonce(24) || ct(2) || tag(16)`. Fixed width, no heap,
/// **public** — every byte is ciphertext or random. `Debug` prints hex.
///
/// Frozen `v1` wire format.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SealedBytes {
    nonce: [u8; 24],
    ct: [u8; 2],
    tag: [u8; 16],
}

impl SealedBytes {
    /// Total length in bytes.
    pub const LEN: usize = 42;

    pub(crate) const fn new(nonce: [u8; 24], ct: [u8; 2], tag: [u8; 16]) -> Self {
        Self { nonce, ct, tag }
    }

    /// The 42 bytes, in wire order.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 42] {
        let mut out = [0u8; 42];
        out[..24].copy_from_slice(&self.nonce);
        out[24..26].copy_from_slice(&self.ct);
        out[26..].copy_from_slice(&self.tag);
        out
    }

    /// From 42 wire bytes. Total: any 42 bytes are *a* `SealedBytes`;
    /// whether they open is for `unseal` to decide.
    #[must_use]
    pub fn from_bytes(b: [u8; 42]) -> Self {
        let mut nonce = [0u8; 24];
        let mut ct = [0u8; 2];
        let mut tag = [0u8; 16];
        nonce.copy_from_slice(&b[..24]);
        ct.copy_from_slice(&b[24..26]);
        tag.copy_from_slice(&b[26..]);
        Self { nonce, ct, tag }
    }

    /// The 24-byte `XChaCha20` nonce.
    #[must_use]
    pub const fn nonce(&self) -> &[u8; 24] {
        &self.nonce
    }

    /// The two ciphertext bytes (an encrypted `Ordinal`).
    #[must_use]
    pub const fn ct(&self) -> &[u8; 2] {
        &self.ct
    }

    /// The 16-byte Poly1305 tag.
    #[must_use]
    pub const fn tag(&self) -> &[u8; 16] {
        &self.tag
    }
}

impl fmt::Debug for SealedBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SealedBytes(")?;
        for b in self.to_bytes() {
            write!(f, "{b:02x}")?;
        }
        f.write_str(")")
    }
}

/// The dealer's public ledger: which sealed bytes stand for which slot.
///
/// A plain value that sits *beside* a `SlotPile` — the shoe knows the order,
/// custody knows the bytes, `Revealed` knows the values. Safe to publish,
/// log, or hand to a spectator. Not a pile: it has no order of its own and
/// nothing to draw from (EPIC-04 gotcha 4).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Vec<(SlotId, SealedBytes)>"))]
pub struct Custody(Vec<(SlotId, SealedBytes)>);

impl TryFrom<Vec<(SlotId, SealedBytes)>> for Custody {
    type Error = CardError;

    fn try_from(pairs: Vec<(SlotId, SealedBytes)>) -> Result<Self, CardError> {
        Self::from_pairs(pairs)
    }
}

impl Custody {
    /// An empty ledger.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// From `(slot, bytes)` pairs, in the given order.
    ///
    /// # Errors
    ///
    /// [`CardError::DuplicateSlot`] on a repeated slot.
    pub fn from_pairs(pairs: Vec<(SlotId, SealedBytes)>) -> Result<Self, CardError> {
        let mut c = Self::new();
        for (slot, sealed) in pairs {
            c.insert(slot, sealed)?;
        }
        Ok(c)
    }

    /// Add one sealed card.
    ///
    /// # Errors
    ///
    /// [`CardError::DuplicateSlot`] if `slot` is already present.
    pub fn insert(&mut self, slot: SlotId, sealed: SealedBytes) -> Result<(), CardError> {
        if self.get(slot).is_some() {
            return Err(CardError::DuplicateSlot(slot.get()));
        }
        self.0.push((slot, sealed));
        Ok(())
    }

    /// The sealed bytes for `slot`, if present.
    #[must_use]
    pub fn get(&self, slot: SlotId) -> Option<&SealedBytes> {
        self.0.iter().find(|(s, _)| *s == slot).map(|(_, b)| b)
    }

    /// Number of sealed cards.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// `true` if no cards are sealed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// `(slot, bytes)` pairs in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (SlotId, &SealedBytes)> + '_ {
        self.0.iter().map(|(s, b)| (*s, b))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__aead__sealed_bytes_tests {
    use super::*;
    use alloc::format;

    fn sample() -> SealedBytes {
        let mut b = [0u8; 42];
        for (i, x) in b.iter_mut().enumerate() {
            *x = u8::try_from(i).unwrap();
        }
        SealedBytes::from_bytes(b)
    }

    #[test]
    fn sealed_bytes__len_is_42() {
        assert_eq!(SealedBytes::LEN, 42);
        assert_eq!(sample().to_bytes().len(), 42);
    }

    #[test]
    fn sealed_bytes__to_from_bytes_roundtrip() {
        let s = sample();
        assert_eq!(SealedBytes::from_bytes(s.to_bytes()), s);
    }

    #[test]
    fn sealed_bytes__parts_are_nonce_ct_tag() {
        let s = sample();
        assert_eq!(s.nonce()[0], 0);
        assert_eq!(s.nonce()[23], 23);
        assert_eq!(s.ct(), &[24, 25]);
        assert_eq!(s.tag()[0], 26);
        assert_eq!(s.tag()[15], 41);
    }

    #[test]
    fn sealed_bytes__debug_is_hex() {
        let dbg = format!("{:?}", sample());
        assert_eq!(
            dbg,
            "SealedBytes(000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f2021222324252627282 9)"
                .replace(' ', "")
        );
    }

    #[test]
    fn sealed_bytes__is_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<SealedBytes>();
    }

    #[test]
    fn custody__insert_get_len_iter() {
        let mut c = Custody::new();
        assert!(c.is_empty());
        c.insert(SlotId::new(3), sample()).unwrap();
        c.insert(SlotId::new(1), sample()).unwrap();
        assert_eq!(c.len(), 2);
        assert_eq!(c.get(SlotId::new(3)), Some(&sample()));
        assert_eq!(c.get(SlotId::new(9)), None);
        let slots: alloc::vec::Vec<SlotId> = c.iter().map(|(s, _)| s).collect();
        assert_eq!(slots, [SlotId::new(3), SlotId::new(1)]);
    }

    #[test]
    fn custody__rejects_duplicate_slot() {
        let mut c = Custody::new();
        c.insert(SlotId::new(3), sample()).unwrap();
        assert_eq!(
            c.insert(SlotId::new(3), sample()).unwrap_err(),
            CardError::DuplicateSlot(3)
        );
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn custody__from_pairs_rejects_duplicate() {
        let pairs = alloc::vec![(SlotId::new(1), sample()), (SlotId::new(1), sample())];
        assert_eq!(
            Custody::from_pairs(pairs).unwrap_err(),
            CardError::DuplicateSlot(1)
        );
    }

    #[test]
    fn custody__debug_contains_only_slots_and_hex() {
        let mut c = Custody::new();
        c.insert(SlotId::new(3), sample()).unwrap();
        let dbg = format!("{c:?}");
        assert!(dbg.contains("SealedBytes(0001"), "{dbg}");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn sealed_bytes__serde_roundtrip() {
        let s = sample();
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(serde_json::from_str::<SealedBytes>(&json).unwrap(), s);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn custody__serde_roundtrip_and_rejects_duplicate() {
        let mut c = Custody::new();
        c.insert(SlotId::new(3), sample()).unwrap();
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(serde_json::from_str::<Custody>(&json).unwrap(), c);
        let dup = alloc::format!(
            "[[3,{0}],[3,{0}]]",
            serde_json::to_string(&sample()).unwrap()
        );
        assert!(serde_json::from_str::<Custody>(&dup).is_err());
    }
}
