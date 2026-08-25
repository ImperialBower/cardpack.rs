//! Commit–reveal shuffle (EPIC-04a): golden vectors and properties.
//!
//! # Golden vectors
//!
//! Every golden value in this file and in the `src/seal/commit/*` unit tests
//! was produced by an **independent Python implementation** of the `v1`
//! format, reproduced here so a future reader can regenerate it:
//!
//! ```python
//! import hashlib
//! TAG_CONTRIB = b"cardpack/commit-reveal/v1/contribution"
//! TAG_SEED    = b"cardpack/commit-reveal/v1/seed"
//! TAG_PERM    = b"cardpack/commit-reveal/v1/permutation"
//! TAG_PILE    = b"cardpack/commit-reveal/v1/pile"
//!
//! def commit(contrib):            # Contribution::commit
//!     return hashlib.sha256(TAG_CONTRIB + contrib).digest()
//!
//! def combine(parts):             # CombinedSeed::combine, parts = [(id, 32 bytes)]
//!     parts = sorted(parts, key=lambda p: p[0])
//!     h = hashlib.sha256(TAG_SEED + len(parts).to_bytes(2, 'big'))
//!     for pid, c in parts:
//!         h.update(pid.to_bytes(2, 'big') + c)
//!     return h.digest()
//!
//! def stream(seed):               # SHA-256 counter mode, u32 BE words
//!     ctr = 0
//!     while True:
//!         blk = hashlib.sha256(seed + ctr.to_bytes(4, 'big')).digest()
//!         for k in range(0, 32, 4):
//!             yield int.from_bytes(blk[k:k+4], 'big')
//!         ctr += 1
//!
//! def permutation(seed, n):       # CombinedSeed::permutation
//!     p = list(range(n)); g = stream(seed)
//!     for i in range(n - 1, 0, -1):
//!         r = i + 1; m = (1 << 32) % r
//!         while True:
//!             x = next(g)
//!             if x < (1 << 32) - m:
//!                 j = x % r; break
//!         p[i], p[j] = p[j], p[i]
//!     return p
//!
//! def perm_canonical(p):          # Permutation::canonical_bytes
//!     return len(p).to_bytes(2, 'big') + b''.join(i.to_bytes(2, 'big') for i in p)
//!
//! def commit_permutation(p, blind):
//!     return hashlib.sha256(TAG_PERM + blind + perm_canonical(p)).digest()
//!
//! def commit_pile(canon_v1_bytes, blind):
//!     return hashlib.sha256(TAG_PILE + blind + canon_v1_bytes).digest()
//!
//! a, b = bytes([0x11]) * 32, bytes([0x22]) * 32
//! seed = combine([(1, a), (2, b)])
//! print(commit(a).hex())                 # commitment__golden_vector
//! print(seed.hex())                      # combine__golden_vector
//! print(permutation(seed, 52))           # derive__golden_permutation_52
//! print(commit_permutation([0, 1, 2], a).hex())
//! ```
//!
//! Skipped on `wasm32-unknown-unknown` because proptest's transitive
//! `wait-timeout` crate is unix-only.

#![cfg(all(feature = "commit-reveal", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

use cardpack::prelude::*;
use proptest::prelude::*;

fn contribution() -> impl Strategy<Value = Contribution> {
    any::<[u8; 32]>().prop_map(Contribution::from_bytes)
}

fn complete_round(parts: &[(u16, Contribution)]) -> ShuffleRound {
    let mut r = ShuffleRound::new(parts.iter().map(|(id, _)| ParticipantId(*id))).unwrap();
    for (id, c) in parts {
        r.commit(ParticipantId(*id), c.commit()).unwrap();
    }
    for (id, c) in parts {
        r.reveal(ParticipantId(*id), *c).unwrap();
    }
    r
}

#[test]
fn golden__transcript_end_to_end() {
    let a = Contribution::from_bytes([0x11; 32]);
    let b = Contribution::from_bytes([0x22; 32]);
    let round = complete_round(&[(1, a), (2, b)]);
    let seed = round.seed().unwrap();

    assert_eq!(
        a.commit().to_hex(),
        "a070e19c3356bc6b0c8c012c303815182a16674d480faf5169f5381f14f6942e"
    );
    assert_eq!(
        seed.to_hex(),
        "600d8d3d6e4f300530a2ebd4301b32f1afc512237d98947703260d7577287f78"
    );
    assert_eq!(seed.permutation(5).unwrap().as_slice(), &[4, 1, 3, 0, 2]);
    assert_eq!(
        commit_permutation(&Permutation::identity(3).unwrap(), &a).to_hex(),
        "15ecd3ef7ec6ff499db58b7289582c1d2fd8d899517e7f02987757892d4c068c"
    );
}

proptest! {
    /// Binding: only the committed contribution opens the commitment.
    #[test]
    fn commitment__verify_roundtrip(c in contribution()) {
        prop_assert!(c.commit().verify(&c));
    }

    #[test]
    fn commitment__verify_rejects_other(a in contribution(), b in contribution()) {
        prop_assume!(a != b);
        prop_assert!(!a.commit().verify(&b));
    }

    /// Every derived permutation is a bijection, for every pile size a
    /// shipped deck can reach (four French decks = 216 cards).
    #[test]
    fn derive__is_valid_permutation(c in contribution(), n in 0usize..=216) {
        let seed = CombinedSeed::combine(&[(ParticipantId(0), c)]).unwrap();
        let p = seed.permutation(n).unwrap();
        prop_assert_eq!(p.len(), n);
        prop_assert!(Permutation::try_from_vec(p.as_slice().to_vec()).is_ok());
    }

    /// Reproducibility: two rounds built from the same public transcript
    /// reach the same seed, and so the same shuffle.
    #[test]
    fn round__any_verifier_reproduces_seed(a in contribution(), b in contribution(), c in contribution()) {
        let parts = [(3u16, a), (1, b), (2, c)];
        let dealer = complete_round(&parts);
        let mut reversed = parts;
        reversed.reverse();
        let verifier = complete_round(&reversed);
        prop_assert_eq!(dealer.seed().unwrap(), verifier.seed().unwrap());
    }

    /// The pile API integration is a shuffle, not a transform.
    #[test]
    fn shuffled_by_round__preserves_multiset(a in contribution(), b in contribution()) {
        let round = complete_round(&[(1, a), (2, b)]);
        let deck = Standard52::deck();
        let shuffled = deck.shuffled_by_round(&round).unwrap();
        prop_assert!(deck.same(&shuffled));
    }

    /// Order commitments are order-sensitive and blind-sensitive.
    #[test]
    fn commit_pile__opens_only_with_same_order_and_blind(
        blind in contribution(), other in contribution(), seed in any::<u64>(), i in 0usize..52, j in 0usize..52
    ) {
        prop_assume!(blind != other);
        prop_assume!(i != j);
        let cb = Standard52::codebook();
        let deck = Standard52::deck().shuffled_with_seed(seed);
        let c = commit_pile(&cb, &deck, &blind).unwrap();
        prop_assert!(verify_pile(&c, &cb, &deck, &blind).unwrap());
        prop_assert!(!verify_pile(&c, &cb, &deck, &other).unwrap());

        let mut cards = deck.cards().clone();
        cards.swap(i, j);
        let swapped = Pile::<Standard52>::from(cards);
        prop_assert!(!verify_pile(&c, &cb, &swapped, &blind).unwrap());
    }
}
