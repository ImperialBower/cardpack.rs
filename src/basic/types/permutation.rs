//! `Permutation` — a shuffle as data.
//!
//! Design: `docs/EPIC-04_Sealed_Decks.md`, Story 3.

use crate::basic::types::ordinal::take_u16;
use crate::common::errors::CardError;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A bijection over `0..len`, as data. Convention: `out[i] = items[p[i]]`.
///
/// The field is private: every constructor validates, and the `serde`
/// deserializer goes through [`TryFrom`] so an invalid permutation can never
/// be built from the wire.
///
/// [`from_rng`](Self::from_rng) is *defined* as the same Fisher–Yates
/// `Pile::shuffle_with_rng` runs, so for the same RNG state
/// `p.apply(deck.cards()) == deck.shuffled_with_rng(rng).cards()`.
///
/// ```
/// use cardpack::prelude::*;
///
/// let p = Permutation::try_from_vec(vec![2, 0, 1]).unwrap();
/// assert_eq!(p.apply(&["a", "b", "c"]).unwrap(), vec!["c", "a", "b"]);
/// assert!(p.then(&p.inverse()).unwrap().is_identity());
///
/// let deck = Standard52::deck();
/// let shuffle = Permutation::from_seed(52, 7).unwrap();
/// assert_eq!(deck.permute(&shuffle).unwrap(), deck.shuffled_with_seed(7));
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Vec<u16>"))]
pub struct Permutation(Vec<u16>);

impl TryFrom<Vec<u16>> for Permutation {
    type Error = CardError;

    fn try_from(v: Vec<u16>) -> Result<Self, CardError> {
        Self::try_from_vec(v)
    }
}

impl Permutation {
    /// The identity over `0..n`.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidPermutation`] if `n > u16::MAX`.
    pub fn identity(n: usize) -> Result<Self, CardError> {
        let len = u16::try_from(n)
            .map_err(|_| CardError::InvalidPermutation(format!("length {n} exceeds u16::MAX")))?;
        Ok(Self((0..len).collect()))
    }

    /// Validates that `v` is a bijection over `0..v.len()`.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidPermutation`] on a duplicate, an out-of-range
    /// index, or a length above `u16::MAX`.
    pub fn try_from_vec(v: Vec<u16>) -> Result<Self, CardError> {
        let n = v.len();
        if u16::try_from(n).is_err() {
            return Err(CardError::InvalidPermutation(format!(
                "length {n} exceeds u16::MAX"
            )));
        }
        let mut seen = alloc::vec![false; n];
        for &i in &v {
            let idx = usize::from(i);
            match seen.get_mut(idx) {
                None => {
                    return Err(CardError::InvalidPermutation(format!(
                        "index {i} out of range for length {n}"
                    )));
                }
                Some(true) => {
                    return Err(CardError::InvalidPermutation(format!(
                        "duplicate index {i}"
                    )));
                }
                Some(slot) => *slot = true,
            }
        }
        Ok(Self(v))
    }

    /// Fisher–Yates on the identity — by construction identical to what
    /// `Pile::shuffle_with_rng` does to the cards.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidPermutation`] if `n > u16::MAX`.
    pub fn from_rng<R: Rng + ?Sized>(n: usize, rng: &mut R) -> Result<Self, CardError> {
        let mut p = Self::identity(n)?;
        p.0.shuffle(rng);
        Ok(p)
    }

    /// `StdRng::seed_from_u64`. Same caveat as `Pile::shuffle_with_seed`: the
    /// result is stable **within one `rand` major version** only. For a
    /// verifier-stable derivation use a commit–reveal seed (EPIC-04a).
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidPermutation`] if `n > u16::MAX`.
    pub fn from_seed(n: usize, seed: u64) -> Result<Self, CardError> {
        Self::from_rng(n, &mut StdRng::seed_from_u64(seed))
    }

    /// The cut: `out = in[at..] ++ in[..at]`.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidCut`] if `at > n`; [`CardError::InvalidPermutation`]
    /// if `n > u16::MAX`.
    pub fn rotation(n: usize, at: usize) -> Result<Self, CardError> {
        if at > n {
            return Err(CardError::InvalidCut(at));
        }
        let mut p = Self::identity(n)?;
        p.0.rotate_left(at);
        Ok(p)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.0.iter().enumerate().all(|(i, &p)| usize::from(p) == i)
    }

    #[must_use]
    pub fn as_slice(&self) -> &[u16] {
        &self.0
    }

    /// `out[i] = items[p[i]]` — read `p[i]` as "where item `i` comes **from**".
    ///
    /// # Errors
    ///
    /// [`CardError::PermutationLength`] if `items.len() != self.len()`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// // "Take index 2 first, then index 0, then index 1."
    /// let p = Permutation::try_from_vec(vec![2, 0, 1]).unwrap();
    ///
    /// assert_eq!(p.apply(&['a', 'b', 'c']).unwrap(), vec!['c', 'a', 'b']);
    ///
    /// // The length must match exactly.
    /// assert!(p.apply(&['a', 'b']).is_err());
    /// ```
    pub fn apply<T: Clone>(&self, items: &[T]) -> Result<Vec<T>, CardError> {
        if items.len() != self.len() {
            return Err(CardError::PermutationLength {
                expected: self.len(),
                actual: items.len(),
            });
        }
        Ok(self
            .0
            .iter()
            .map(|&i| items[usize::from(i)].clone())
            .collect())
    }

    /// The permutation that undoes this one.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let deck = Standard52::deck();
    /// let p = Permutation::from_seed(52, 7).unwrap();
    ///
    /// let shuffled = deck.permute(&p).unwrap();
    /// assert_eq!(shuffled.permute(&p.inverse()).unwrap(), deck);
    ///
    /// // Composing the two really is the identity.
    /// assert!(p.then(&p.inverse()).unwrap().is_identity());
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Self {
        let mut inv = alloc::vec![0_u16; self.len()];
        for (i, &p) in self.0.iter().enumerate() {
            inv[usize::from(p)] = u16::try_from(i).unwrap_or(u16::MAX);
        }
        Self(inv)
    }

    /// Compose: **`self` first, then `next`**.
    ///
    /// `(a.then(b)).apply(x) == b.apply(a.apply(x))`. The order is easy to
    /// read backwards, so the example pins it.
    ///
    /// # Errors
    ///
    /// [`CardError::PermutationLength`] if the lengths differ.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let a = Permutation::try_from_vec(vec![1, 0, 2]).unwrap(); // swap 0 and 1
    /// let b = Permutation::try_from_vec(vec![0, 2, 1]).unwrap(); // swap 1 and 2
    /// let x = ['a', 'b', 'c'];
    ///
    /// // `a` runs first.
    /// assert_eq!(a.apply(&x).unwrap(), vec!['b', 'a', 'c']);
    /// assert_eq!(a.then(&b).unwrap().apply(&x).unwrap(), vec!['b', 'c', 'a']);
    /// assert_eq!(
    ///     a.then(&b).unwrap().apply(&x).unwrap(),
    ///     b.apply(&a.apply(&x).unwrap()).unwrap()
    /// );
    ///
    /// // Order matters: composition does not commute.
    /// assert_ne!(a.then(&b).unwrap(), b.then(&a).unwrap());
    /// ```
    pub fn then(&self, next: &Self) -> Result<Self, CardError> {
        if self.len() != next.len() {
            return Err(CardError::PermutationLength {
                expected: self.len(),
                actual: next.len(),
            });
        }
        Ok(Self(
            next.0.iter().map(|&j| self.0[usize::from(j)]).collect(),
        ))
    }

    /// `[u16 BE len][u16 BE]*`. Frozen.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let len = u16::try_from(self.len()).unwrap_or(u16::MAX);
        let mut out = Vec::with_capacity(2 + self.len() * 2);
        out.extend_from_slice(&len.to_be_bytes());
        for &i in &self.0 {
            out.extend_from_slice(&i.to_be_bytes());
        }
        out
    }

    /// Inverse of [`canonical_bytes`](Self::canonical_bytes). Strict.
    ///
    /// # Errors
    ///
    /// [`CardError::CanonicalMalformed`] on truncation or trailing bytes;
    /// [`CardError::InvalidPermutation`] if the decoded list is not a bijection.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, CardError> {
        let malformed = |what: &str| CardError::CanonicalMalformed(what.to_string());
        let (len, mut rest) = take_u16(bytes).ok_or_else(|| malformed("truncated"))?;
        let mut v = Vec::with_capacity(usize::from(len));
        for _ in 0..len {
            let (i, tail) = take_u16(rest).ok_or_else(|| malformed("truncated"))?;
            v.push(i);
            rest = tail;
        }
        if !rest.is_empty() {
            return Err(malformed("trailing bytes"));
        }
        Self::try_from_vec(v)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__types__permutation_tests {
    use super::*;
    use crate::prelude::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn permutation__identity_and_len() {
        let p = Permutation::identity(3).unwrap();
        assert_eq!(p.as_slice(), &[0, 1, 2]);
        assert!(p.is_identity());
        assert_eq!(p.len(), 3);
        assert!(!p.is_empty());
        assert!(Permutation::identity(0).unwrap().is_empty());
    }

    #[test]
    fn permutation__identity_too_large_errors() {
        assert!(matches!(
            Permutation::identity(70_000),
            Err(CardError::InvalidPermutation(_))
        ));
    }

    #[test]
    fn permutation__rejects_duplicate() {
        assert!(matches!(
            Permutation::try_from_vec(vec![0, 1, 1]),
            Err(CardError::InvalidPermutation(_))
        ));
    }

    #[test]
    fn permutation__rejects_out_of_range() {
        assert!(matches!(
            Permutation::try_from_vec(vec![0, 2]),
            Err(CardError::InvalidPermutation(_))
        ));
    }

    /// The one convention, stated once: `out[i] = in[p[i]]`.
    #[test]
    fn permutation__apply_follows_convention() {
        let p = Permutation::try_from_vec(vec![2, 0, 1]).unwrap();
        assert_eq!(p.apply(&["a", "b", "c"]).unwrap(), vec!["c", "a", "b"]);
        assert!(!p.is_identity());
    }

    #[test]
    fn permutation__apply_length_mismatch_errors() {
        let p = Permutation::identity(3).unwrap();
        assert_eq!(
            p.apply(&[1, 2]),
            Err(CardError::PermutationLength {
                expected: 3,
                actual: 2
            })
        );
    }

    #[test]
    fn permutation__inverse_roundtrip() {
        let x: Vec<u32> = (0..52).collect();
        for seed in 0..8_u64 {
            let p = Permutation::from_seed(52, seed).unwrap();
            assert_eq!(p.inverse().apply(&p.apply(&x).unwrap()).unwrap(), x);
            assert!(p.then(&p.inverse()).unwrap().is_identity());
        }
    }

    #[test]
    fn permutation__compose_law() {
        let x: Vec<u32> = (0..20).collect();
        let a = Permutation::from_seed(20, 1).unwrap();
        let b = Permutation::from_seed(20, 2).unwrap();
        assert_eq!(
            a.then(&b).unwrap().apply(&x).unwrap(),
            b.apply(&a.apply(&x).unwrap()).unwrap()
        );
        let short = Permutation::identity(3).unwrap();
        assert_eq!(
            a.then(&short),
            Err(CardError::PermutationLength {
                expected: 20,
                actual: 3
            })
        );
    }

    /// `from_rng` is *defined* as the same Fisher–Yates `Pile::shuffle_with_rng`
    /// runs, so the two agree for the same RNG state.
    #[test]
    fn permutation__from_rng_matches_pile_shuffle() {
        let deck = Standard52::deck();
        for seed in [0_u64, 1, 7, 42, 1_000_003] {
            let p = Permutation::from_rng(52, &mut StdRng::seed_from_u64(seed)).unwrap();
            assert_eq!(
                p.apply(deck.cards()).unwrap(),
                *deck.shuffled_with_seed(seed).cards(),
                "seed {seed}"
            );
            assert_eq!(Permutation::from_seed(52, seed).unwrap(), p);
        }
    }

    #[test]
    fn permutation__rotation_is_cut() {
        let x = [0, 1, 2, 3, 4];
        assert_eq!(
            Permutation::rotation(5, 2).unwrap().apply(&x).unwrap(),
            vec![2, 3, 4, 0, 1]
        );
        assert!(Permutation::rotation(5, 0).unwrap().is_identity());
        assert!(Permutation::rotation(5, 5).unwrap().is_identity());
        assert_eq!(Permutation::rotation(5, 6), Err(CardError::InvalidCut(6)));
    }

    #[test]
    fn permutation__canonical_roundtrip() {
        let p = Permutation::try_from_vec(vec![2, 0, 1]).unwrap();
        let bytes = p.canonical_bytes();
        assert_eq!(bytes, vec![0x00, 0x03, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01]);
        assert_eq!(Permutation::from_canonical_bytes(&bytes).unwrap(), p);
        let p = Permutation::from_seed(216, 9).unwrap();
        assert_eq!(
            Permutation::from_canonical_bytes(&p.canonical_bytes()).unwrap(),
            p
        );
    }

    #[test]
    fn permutation__canonical_rejects_invalid() {
        assert!(matches!(
            Permutation::from_canonical_bytes(&[0x00]),
            Err(CardError::CanonicalMalformed(_))
        ));
        assert!(matches!(
            Permutation::from_canonical_bytes(&[0x00, 0x02, 0x00, 0x00]),
            Err(CardError::CanonicalMalformed(_))
        ));
        assert!(matches!(
            Permutation::from_canonical_bytes(&[0x00, 0x01, 0x00, 0x00, 0xFF]),
            Err(CardError::CanonicalMalformed(_))
        ));
        assert!(matches!(
            Permutation::from_canonical_bytes(&[0x00, 0x02, 0x00, 0x01, 0x00, 0x01]),
            Err(CardError::InvalidPermutation(_))
        ));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn permutation__serde_roundtrip() {
        let p = Permutation::from_seed(10, 3).unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<Permutation>(&json).unwrap(), p);
    }
}
