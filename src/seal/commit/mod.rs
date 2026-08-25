//! Commit–reveal shuffle: provably-fair shuffles and blind order commitments.
//!
//! Behind the `commit-reveal` feature (one dependency, `sha2`; `no_std`).
//! Design: `docs/EPIC-04a_Commit_Reveal_Shuffle.md`.
//!
//! # What it gives you
//!
//! * **Nobody can bias the shuffle.** Every participant commits to secret
//!   entropy ([`Contribution::commit`]) before anyone reveals
//!   ([`ShuffleRound`] enforces it). The seed is a hash over *all*
//!   contributions, so the last to reveal learns nothing they can use.
//! * **Anybody can verify.** The public transcript — participant ids,
//!   commitments, revealed contributions — determines [`CombinedSeed`] and,
//!   through a frozen derivation, the exact [`Permutation`](crate::basic::types::permutation::Permutation). A verifier
//!   needs SHA-256 and nothing else.
//! * **Dealers can be audited.** [`commit_pile`] / [`commit_permutation`]
//!   let a dealer publish a blind commitment to a concrete order before
//!   dealing and open it after.
//!
//! # What it does not give you
//!
//! * **Hidden cards.** This hides the *shuffle*, not the *cards*. Anyone who
//!   sees the pile sees every card. Hiding cards is a sealing backend (see
//!   `docs/EPIC-04b_Holder_Key_Seal.md`).
//! * **Transport, signatures, liveness.** [`ShuffleRound`] is a pure state
//!   machine. Who delivers messages, who signs them, and what happens when a
//!   participant never reveals are yours. A participant who commits and then
//!   refuses to reveal **aborts** the round; they cannot bias it.
//!
//! # The `v1` format — a verifier's contract
//!
//! All preimages are domain-separated with a versioned ASCII tag. SHA-256 is
//! part of the format, not a parameter. Integers are big-endian.
//!
//! | Value | Preimage |
//! |---|---|
//! | [`Commitment`] of a contribution | `SHA-256(b"cardpack/commit-reveal/v1/contribution" ‖ 32 bytes)` |
//! | [`CombinedSeed`] | `SHA-256(b"cardpack/commit-reveal/v1/seed" ‖ u16 n ‖ (u16 id ‖ 32 bytes)*)`, pairs sorted by id |
//! | [`Permutation`](crate::basic::types::permutation::Permutation) from a seed | Fisher–Yates over the identity, drawing from `SHA-256(seed ‖ u32 counter)` with exact rejection sampling — see [`derive`](mod@derive) |
//! | [`commit_permutation`] | `SHA-256(b"cardpack/commit-reveal/v1/permutation" ‖ blind ‖ Permutation::canonical_bytes)` |
//! | [`commit_pile`] | `SHA-256(b"cardpack/commit-reveal/v1/pile" ‖ blind ‖ Codebook::encode_pile)` (`CANON_V1`) |
//!
//! Changing any row is a `v2` tag, never an edit. Golden vectors in
//! `tests/commit_reveal.rs` pin every row, and that file carries a
//! stand-alone Python reference that produced them.
//!
//! # Verifying a transcript (pseudo-code)
//!
//! ```text
//! for (id, commitment, contribution) in transcript:
//!     assert SHA-256(TAG_CONTRIBUTION ‖ contribution) == commitment
//! seed = SHA-256(TAG_SEED ‖ n ‖ sorted-by-id (id ‖ contribution)*)
//! perm = fisher_yates(identity(n_cards), stream = SHA-256(seed ‖ counter))
//! assert dealt_order == committed_order[perm]        # out[i] = in[perm[i]]
//! ```
//!
//! In Rust that is: rebuild a [`ShuffleRound`] from the transcript, then
//! `pile.shuffled_by_round(&round)` (see `examples/provably_fair.rs`).
//!
//! # Example
//!
//! ```
//! use cardpack::prelude::*;
//!
//! let (dealer, player) = (ParticipantId(1), ParticipantId(2));
//! let a = Contribution::from_bytes([0x11; 32]); // use Contribution::random in real code
//! let b = Contribution::from_bytes([0x22; 32]);
//!
//! let mut round = ShuffleRound::new([dealer, player])?;
//! round.commit(dealer, a.commit())?;
//! assert_eq!(round.reveal(dealer, a), Err(CardError::RevealBeforeAllCommitted));
//! round.commit(player, b.commit())?;
//! round.reveal(dealer, a)?;
//! round.reveal(player, b)?;
//!
//! let shuffled = Standard52::deck().shuffled_by_round(&round)?;
//! assert!(Standard52::deck().same(&shuffled));
//! # Ok::<(), CardError>(())
//! ```

pub mod commitment;
pub mod derive;
mod hex;
pub mod pile;
pub mod round;

pub use commitment::{Commitment, Contribution, TAG_CONTRIBUTION};
pub use pile::{
    TAG_PERMUTATION, TAG_PILE, commit_permutation, commit_pile, verify_permutation, verify_pile,
};
pub use round::{CombinedSeed, ParticipantId, ShuffleRound, TAG_SEED};
