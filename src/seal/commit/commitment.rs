//! A participant's secret entropy and the public promise about it.

use alloc::string::{String, ToString};
use core::fmt;

use rand::Rng;
use sha2::{Digest, Sha256};

use crate::common::errors::CardError;
use crate::seal::commit::hex;

/// Domain-separation tag for [`Contribution::commit`]. Part of the frozen
/// `v1` format — a verifier in any language must prepend exactly these bytes.
pub const TAG_CONTRIBUTION: &[u8] = b"cardpack/commit-reveal/v1/contribution";

/// 32 uniformly random bytes. Secret until revealed.
///
/// `Debug` is redacted so a contribution can never reach a log line by
/// accident; there is deliberately no `Display` and no `serde`.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Contribution([u8; 32]);

impl Contribution {
    /// The only documented constructor. `rng` **must** be a cryptographically
    /// secure pseudorandom number generator
    /// [CSPRNG](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator)
    /// (for example `rand::rng()` under `std`); a contribution is only *hiding*
    /// if an adversary cannot guess it.
    pub fn random(rng: &mut dyn Rng) -> Self {
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    /// For tests and for callers with their own entropy source.
    ///
    /// A low-entropy input (a counter, a timestamp, a short password) is
    /// **not hiding**: the commitment can be opened by brute force.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// The raw bytes. This is the value a participant publishes at reveal time.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// `SHA-256(TAG_CONTRIBUTION || bytes)`.
    #[must_use]
    pub fn commit(&self) -> Commitment {
        let mut h = Sha256::new();
        h.update(TAG_CONTRIBUTION);
        h.update(self.0);
        Commitment(h.finalize().into())
    }
}

impl fmt::Debug for Contribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Contribution(<redacted>)")
    }
}

/// A binding, hiding commitment to a [`Contribution`]. Public.
///
/// Compared with plain `==`: both sides are public values, so timing leaks
/// nothing. `Debug` and `Display` print lowercase hex.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Commitment([u8; 32]);

impl Commitment {
    /// Wraps raw digest bytes. Used by the other commitment constructors in
    /// this module; a caller has no reason to build one by hand.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// The 32 digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Lowercase hex, 64 characters.
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// Parses 64 hex digits of either case.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidHex`] with the offending input on wrong length or
    /// a non-hex character.
    pub fn from_hex(s: &str) -> Result<Self, CardError> {
        hex::decode_32(s)
            .map(Self)
            .ok_or_else(|| CardError::InvalidHex(s.to_string()))
    }

    /// Recompute the commitment of `c` and compare.
    #[must_use]
    pub fn verify(&self, c: &Contribution) -> bool {
        c.commit() == *self
    }
}

impl fmt::Debug for Commitment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Commitment({})", self.to_hex())
    }
}

impl fmt::Display for Commitment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__commit__commitment_tests {
    use super::*;
    use alloc::format;
    use alloc::string::ToString;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    /// Independently computed with Python:
    /// `hashlib.sha256(b"cardpack/commit-reveal/v1/contribution" + b"\x11"*32).hexdigest()`
    const GOLDEN: &str = "a070e19c3356bc6b0c8c012c303815182a16674d480faf5169f5381f14f6942e";

    #[test]
    fn commitment__golden_vector() {
        let c = Contribution::from_bytes([0x11; 32]).commit();
        assert_eq!(c.to_hex(), GOLDEN);
    }

    #[test]
    fn commitment__verify_roundtrip() {
        for b in [0x00u8, 0x11, 0x7f, 0xff] {
            let c = Contribution::from_bytes([b; 32]);
            assert!(c.commit().verify(&c));
        }
    }

    #[test]
    fn commitment__verify_rejects_other() {
        let a = Contribution::from_bytes([0x11; 32]);
        let mut other = [0x11; 32];
        other[31] ^= 1;
        let b = Contribution::from_bytes(other);
        assert!(!a.commit().verify(&b));
    }

    #[test]
    fn contribution__debug_redacted() {
        let c = Contribution::from_bytes([0xab; 32]);
        let dbg = format!("{c:?}");
        assert!(!dbg.contains("ab"), "leaked: {dbg}");
        assert!(!dbg.contains("171"), "leaked: {dbg}");
        assert_eq!(dbg, "Contribution(<redacted>)");
    }

    #[test]
    fn contribution__as_bytes_roundtrip() {
        let c = Contribution::from_bytes([0x42; 32]);
        assert_eq!(c.as_bytes(), &[0x42; 32]);
    }

    #[test]
    fn contribution__random_differs() {
        let mut rng = StdRng::seed_from_u64(7);
        let a = Contribution::random(&mut rng);
        let b = Contribution::random(&mut rng);
        assert_ne!(a, b);
    }

    #[test]
    fn commitment__hex_roundtrip() {
        let c = Contribution::from_bytes([0x11; 32]).commit();
        let hex = c.to_hex();
        assert_eq!(hex.len(), 64);
        assert_eq!(Commitment::from_hex(&hex).unwrap(), c);
        assert_eq!(Commitment::from_hex(&hex.to_uppercase()).unwrap(), c);
    }

    #[test]
    fn commitment__debug_is_hex() {
        let c = Contribution::from_bytes([0x11; 32]).commit();
        assert_eq!(format!("{c:?}"), format!("Commitment({GOLDEN})"));
    }

    #[test]
    fn commitment__from_hex_rejects_garbage() {
        assert_eq!(
            Commitment::from_hex("zz"),
            Err(CardError::InvalidHex("zz".to_string()))
        );
        let short = "ab".repeat(31);
        assert_eq!(
            Commitment::from_hex(&short),
            Err(CardError::InvalidHex(short.clone()))
        );
        let bad_char = format!("{}g", "ab".repeat(31));
        assert_eq!(
            Commitment::from_hex(&bad_char),
            Err(CardError::InvalidHex(bad_char.clone()))
        );
    }

    #[test]
    fn commitment__as_bytes_is_32() {
        let c = Contribution::from_bytes([0x11; 32]).commit();
        assert_eq!(c.as_bytes().len(), 32);
    }
}
