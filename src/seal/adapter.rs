//! `Seal<D>` — the adapter through which a reveal can be *verified*.
//!
//! Lives in `adapter.rs` (not `seal.rs`) to avoid `seal::seal`.

use crate::basic::types::card::Card;
use crate::basic::types::traits::DeckedBase;
use crate::seal::slot::SlotId;
use rand::Rng;

/// A card-sealing scheme: the caller's lock and key.
///
/// cardpack defines the shape; the *caller* provides the implementation, the
/// keys, and the tokens. cardpack never constructs an `S`, never stores one,
/// and **no cardpack type is generic over one** — the only kernel caller is
/// [`Revealed::reveal_with`](crate::seal::revealed::Revealed::reveal_with),
/// which is generic at the method.
///
/// Five items: three associated types and two methods. The slot is passed
/// to both methods so a backend *may* bind payload to slot (an AEAD does; an
/// `ElGamal` scheme ignores it); `seal` takes an RNG because every real
/// backend is randomized.
///
/// The round-trip law every implementation must satisfy —
/// `unseal(seal(card, slot, rng), slot, token) == card` — is one generic
/// test, `seal_roundtrip` (behind `seal-test-double`), exported
/// under the `seal-test-double` feature so backends in other crates can run it.
///
/// # Implementing one
///
/// ```
/// use cardpack::prelude::*;
///
/// // A toy scheme: `Sealed` is the card itself, so the "secret" is a lie.
/// // Real backends do real crypto — see `HolderKeySeal` (`seal-aead`).
/// struct Toy;
///
/// impl Seal<Standard52> for Toy {
///     type Sealed = Card<Standard52>;
///     type Token = u16;
///     type Error = CardError;
///
///     fn seal(&self, card: Card<Standard52>, _slot: SlotId, _rng: &mut dyn rand::Rng)
///         -> Result<Card<Standard52>, CardError> { Ok(card) }
///
///     fn unseal(&self, sealed: &Card<Standard52>, _slot: SlotId, token: &u16)
///         -> Result<Card<Standard52>, CardError>
///     {
///         if *token == 42 { Ok(*sealed) } else { Err(CardError::Fubar) }
///     }
/// }
///
/// let ace = Standard52::deck().cards()[0];
/// use rand::SeedableRng;
/// let mut rng = rand::rngs::StdRng::seed_from_u64(1);
///
/// let sealed = Toy.seal(ace, SlotId::new(3), &mut rng)?;
///
/// assert_eq!(Toy.unseal(&sealed, SlotId::new(3), &42)?, ace);
/// assert!(Toy.unseal(&sealed, SlotId::new(3), &0).is_err());
/// # Ok::<(), CardError>(())
/// ```
pub trait Seal<D: DeckedBase> {
    /// The opaque payload. The backend picks the representation: 42 bytes of
    /// AEAD output, an `ElGamal` ciphertext, or (in tests) a `Card<D>`.
    ///
    /// `Eq` is for containers only. Under any randomized scheme, two seals of
    /// the same card are unequal — never use it for distinctness.
    type Sealed: Clone + Eq + core::fmt::Debug;

    /// What a caller presents to open exactly one sealed card. May be a
    /// collection (a threshold scheme needs one share per player).
    type Token;

    /// Scheme-specific failure. Associated, so cardpack never names a crypto
    /// error type and `CardError` stays crypto-free.
    type Error: core::error::Error + Send + Sync + 'static;

    /// Lock a plaintext card into `slot`. Called by whoever *has* the key —
    /// never by cardpack itself.
    ///
    /// # Errors
    ///
    /// Scheme-specific; for example a card that is not in the deck.
    fn seal(
        &self,
        card: Card<D>,
        slot: SlotId,
        rng: &mut dyn Rng,
    ) -> Result<Self::Sealed, Self::Error>;

    /// Open one sealed payload with a token.
    ///
    /// # Errors
    ///
    /// A wrong token, wrong slot, or wrong context must be `Err` — **never**
    /// a different card.
    fn unseal(
        &self,
        sealed: &Self::Sealed,
        slot: SlotId,
        token: &Self::Token,
    ) -> Result<Card<D>, Self::Error>;
}

#[cfg(test)]
#[allow(non_snake_case, dead_code)]
mod seal__adapter_tests {
    use super::*;
    use crate::prelude::*;
    use crate::seal::plaintext::{PlainSealError, PlainToken};

    /// Compile-time check: the trait is object-safe.
    fn _assert_object_safe(
        _: &dyn Seal<French, Sealed = Card<French>, Token = PlainToken, Error = PlainSealError>,
    ) {
    }

    #[test]
    fn seal__trait_is_object_safe() {
        // The function above must compile; nothing to run.
    }
}
