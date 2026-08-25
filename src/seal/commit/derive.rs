//! The frozen seed → [`Permutation`] derivation.
//!
//! **This is the verifier's contract.** Every step below is part of the `v1`
//! format; change any of it and every recorded transcript breaks. A new
//! algorithm is a `v2` tag.
//!
//! 1. **Stream.** Block `k` is `SHA-256(seed || u32 BE k)`, `k` from 0. Each
//!    block yields eight `u32` words, big-endian, in order. Blocks are
//!    consumed lazily and in sequence.
//! 2. **Uniform draw in `0..range`.** Take the next word `x`. Let
//!    `m = 2^32 mod range`. If `x < 2^32 − m`, the result is `x mod range`;
//!    otherwise discard `x` and take the next word. This is exact rejection
//!    sampling: a plain `x mod range` would favour small indices.
//! 3. **Fisher–Yates.** Start from the identity `p = [0, 1, …, n−1]`. For
//!    `i` from `n−1` down to `1`, draw `j` uniformly in `0..=i` and swap
//!    `p[i]` with `p[j]`.
//!
//! The result is a [`Permutation`] with the crate convention `out[i] =
//! in[p[i]]` — exactly the value `Pile::shuffle_with_rng` would have produced
//! had its RNG emitted this word sequence.

use sha2::{Digest, Sha256};

use crate::basic::types::permutation::Permutation;
use crate::common::errors::CardError;

/// SHA-256 counter-mode word stream over a 32-byte seed.
pub(crate) struct Stream<'a> {
    seed: &'a [u8; 32],
    counter: u32,
    block: [u8; 32],
    /// Next word index within `block`; `8` means "exhausted".
    word: usize,
}

impl<'a> Stream<'a> {
    pub(crate) fn new(seed: &'a [u8; 32]) -> Self {
        Self {
            seed,
            counter: 0,
            block: [0; 32],
            word: 8,
        }
    }

    pub(crate) fn next_u32(&mut self) -> u32 {
        if self.word == 8 {
            let mut h = Sha256::new();
            h.update(self.seed);
            h.update(self.counter.to_be_bytes());
            self.block = h.finalize().into();
            self.counter = self.counter.wrapping_add(1);
            self.word = 0;
        }
        let at = self.word * 4;
        self.word += 1;
        u32::from_be_bytes([
            self.block[at],
            self.block[at + 1],
            self.block[at + 2],
            self.block[at + 3],
        ])
    }

    /// Uniform in `0..range` by exact rejection sampling. `range >= 1`.
    fn below(&mut self, range: u32) -> u32 {
        // 2^32 mod range, computed without overflow.
        let m = ((u32::MAX % range) + 1) % range;
        // Accept x iff x < 2^32 - m, i.e. x <= u32::MAX - m.
        let max_ok = u32::MAX - m;
        loop {
            let x = self.next_u32();
            if x <= max_ok {
                return x % range;
            }
        }
    }
}

/// The derivation described in the module doc.
///
/// # Errors
///
/// [`CardError::InvalidPermutation`] if `n > u16::MAX`.
pub(crate) fn permutation(seed: &[u8; 32], n: usize) -> Result<Permutation, CardError> {
    let mut p: alloc::vec::Vec<u16> = Permutation::identity(n)?.as_slice().to_vec();
    let mut stream = Stream::new(seed);
    for i in (1..n).rev() {
        // i <= u16::MAX here, so i + 1 fits a u32.
        let range = u32::try_from(i + 1).map_err(|_| {
            CardError::InvalidPermutation(alloc::format!("length {n} exceeds u16::MAX"))
        })?;
        let j = stream.below(range) as usize;
        p.swap(i, j);
    }
    Permutation::try_from_vec(p)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__commit__derive_tests {
    use super::*;
    use crate::seal::commit::{CombinedSeed, Contribution, ParticipantId};
    use alloc::vec::Vec;

    fn golden_seed() -> CombinedSeed {
        CombinedSeed::combine(&[
            (ParticipantId(1), Contribution::from_bytes([0x11; 32])),
            (ParticipantId(2), Contribution::from_bytes([0x22; 32])),
        ])
        .unwrap()
    }

    /// Python reference: `permutation(seed, 52)` for the golden seed.
    const GOLDEN_52: [u16; 52] = [
        16, 30, 46, 47, 27, 18, 25, 50, 2, 26, 32, 23, 21, 6, 14, 7, 38, 43, 0, 12, 28, 5, 13, 3,
        34, 51, 8, 20, 15, 45, 42, 48, 11, 17, 40, 22, 29, 35, 1, 33, 4, 31, 10, 19, 49, 9, 39, 24,
        41, 37, 36, 44,
    ];

    #[test]
    fn derive__golden_permutation_52() {
        let p = golden_seed().permutation(52).unwrap();
        assert_eq!(p.as_slice(), &GOLDEN_52);
    }

    #[test]
    fn derive__golden_permutation_5() {
        let p = golden_seed().permutation(5).unwrap();
        assert_eq!(p.as_slice(), &[4, 1, 3, 0, 2]);
    }

    #[test]
    fn derive__is_valid_permutation_for_every_n_up_to_216() {
        let seed = golden_seed();
        for n in 0..=216 {
            let p = seed.permutation(n).unwrap();
            assert_eq!(p.len(), n);
            let v: Vec<u16> = p.as_slice().to_vec();
            assert!(Permutation::try_from_vec(v).is_ok(), "n = {n}");
        }
    }

    #[test]
    fn derive__zero_and_one_are_identity() {
        let seed = golden_seed();
        assert!(seed.permutation(0).unwrap().is_identity());
        assert!(seed.permutation(1).unwrap().is_identity());
    }

    #[test]
    fn derive__too_large_errors() {
        let n = usize::from(u16::MAX) + 1;
        assert!(matches!(
            golden_seed().permutation(n),
            Err(CardError::InvalidPermutation(_))
        ));
    }

    #[test]
    fn derive__differs_per_seed() {
        let other = CombinedSeed::combine(&[
            (ParticipantId(1), Contribution::from_bytes([0x11; 32])),
            (ParticipantId(2), Contribution::from_bytes([0x23; 32])),
        ])
        .unwrap();
        assert_ne!(
            golden_seed().permutation(52).unwrap(),
            other.permutation(52).unwrap()
        );
    }

    #[test]
    fn derive__is_deterministic() {
        let a = golden_seed().permutation(52).unwrap();
        let b = golden_seed().permutation(52).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn stream__words_are_big_endian_from_counter_blocks() {
        // Block 0 = SHA-256(seed || 0u32 BE); first word = its first 4 bytes.
        use sha2::{Digest, Sha256};
        let seed = golden_seed();
        let mut h = Sha256::new();
        h.update(seed.as_bytes());
        h.update(0u32.to_be_bytes());
        let block: [u8; 32] = h.finalize().into();
        let expected = u32::from_be_bytes([block[0], block[1], block[2], block[3]]);

        let mut s = Stream::new(seed.as_bytes());
        assert_eq!(s.next_u32(), expected);
    }

    /// Pins the rejection rule (module doc, step 2). With `range = 2^31 + 1`,
    /// `m = 2^31 − 1` and everything above `2^31` is rejected. For the golden
    /// seed the first stream word is 3211300292 (rejected) and the second is
    /// 904556844 (accepted). A plain `x % range` would return 1063816643.
    /// Fisher–Yates never reaches such a range (`n ≤ u16::MAX`), which is why
    /// the 52-card golden vector alone cannot catch this mutation.
    #[test]
    fn below__skips_words_in_the_biased_zone() {
        let seed = golden_seed();
        let mut s = Stream::new(seed.as_bytes());
        assert_eq!(s.below((1u32 << 31) + 1), 904_556_844);
    }

    /// Loose sanity check that the first output position is not skewed.
    /// Ignored by default: 20k derivations. `cargo test --features
    /// commit-reveal -- --ignored derive__unbiased_smoke`.
    #[test]
    #[ignore = "20k derivations; run on demand"]
    fn derive__unbiased_smoke() {
        const N: usize = 5;
        const TRIALS: u32 = 20_000;
        let mut counts = [0u32; N];
        for t in 0..TRIALS {
            let mut b = [0u8; 32];
            b[..4].copy_from_slice(&t.to_be_bytes());
            let seed =
                CombinedSeed::combine(&[(ParticipantId(0), Contribution::from_bytes(b))]).unwrap();
            let p = seed.permutation(N).unwrap();
            counts[usize::from(p.as_slice()[0])] += 1;
        }
        let expected = f64::from(TRIALS) / 5.0;
        let chi2: f64 = counts
            .iter()
            .map(|&c| {
                let d = f64::from(c) - expected;
                d * d / expected
            })
            .sum();
        // 4 degrees of freedom; 18.47 is the 0.1% critical value.
        assert!(chi2 < 18.47, "chi2 = {chi2}, counts = {counts:?}");
    }
}
