//! `PlaintextSeal` — the test double. **NO SECURITY.**
//!
//! Reachable only under `cfg(test)` or the `seal-test-double` feature. It
//! exists to test the *plumbing* — `Revealed::reveal_with`, the conformance
//! law — never secrecy.

use crate::basic::types::card::Card;
use crate::basic::types::traits::{Decked, DeckedBase};
use crate::seal::adapter::Seal;
use crate::seal::slot::SlotId;
use core::fmt::Debug;
use core::hash::Hash;
use rand::Rng;

/// **NO SECURITY WHATSOEVER.**
///
/// `Sealed = Card<D>`: "sealing" is the identity function and `unseal` checks
/// that the token matches a shared secret. The payload *is* the card and its
/// derived `Debug` prints it.
#[derive(Clone, Debug)]
pub struct PlaintextSeal {
    secret: u64,
}

/// The double's token: the shared secret, in the clear.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainToken(pub u64);

/// The double's only failure mode. Its own enum, like every backend's.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PlainSealError {
    #[error("wrong token")]
    WrongToken,
}

impl PlaintextSeal {
    #[must_use]
    pub const fn new(secret: u64) -> Self {
        Self { secret }
    }
}

impl<D: DeckedBase + Default + Ord + Copy + Hash + Debug> Seal<D> for PlaintextSeal {
    type Sealed = Card<D>;
    type Token = PlainToken;
    type Error = PlainSealError;

    fn seal(
        &self,
        card: Card<D>,
        _slot: SlotId,
        _rng: &mut dyn Rng,
    ) -> Result<Card<D>, PlainSealError> {
        Ok(card)
    }

    fn unseal(
        &self,
        sealed: &Card<D>,
        _slot: SlotId,
        token: &PlainToken,
    ) -> Result<Card<D>, PlainSealError> {
        if token.0 == self.secret {
            Ok(*sealed)
        } else {
            Err(PlainSealError::WrongToken)
        }
    }
}

/// The conformance law every [`Seal`] implementation must satisfy, as one
/// generic check over a whole deck:
/// `unseal(seal(card, slot, rng), slot, token_for(slot)) == card`.
///
/// Exported under `seal-test-double` so a backend in another crate can run
/// exactly this against its own scheme.
///
/// # Panics
///
/// On the first card that does not round-trip, naming the slot and the
/// backend's error.
///
/// # Example — a foreign backend, checked from outside the crate
///
/// A hand-rolled scheme (a per-slot XOR pad — **no security**, it exists to
/// show the bounds are satisfiable by another crate) run through the helper:
///
/// ```
/// use cardpack::prelude::*;
/// use cardpack::seal::seal_roundtrip;
/// use rand::SeedableRng;
///
/// struct XorSeal { pad: u16 }
///
/// #[derive(Debug)]
/// struct BadToken;
/// impl core::fmt::Display for BadToken {
///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str("bad token") }
/// }
/// impl core::error::Error for BadToken {}
///
/// impl Seal<Standard52> for XorSeal {
///     type Sealed = u16;
///     type Token = u16;
///     type Error = BadToken;
///
///     fn seal(&self, card: Card<Standard52>, slot: SlotId, _rng: &mut dyn rand::Rng)
///         -> Result<u16, BadToken>
///     {
///         let ord = Standard52::codebook().ordinal(&card).ok_or(BadToken)?;
///         Ok(ord.get() ^ self.pad ^ slot.get())
///     }
///
///     fn unseal(&self, sealed: &u16, slot: SlotId, token: &u16)
///         -> Result<Card<Standard52>, BadToken>
///     {
///         if *token != self.pad { return Err(BadToken); }
///         Standard52::codebook()
///             .card(Ordinal::new(sealed ^ self.pad ^ slot.get()))
///             .ok_or(BadToken)
///     }
/// }
///
/// let scheme = XorSeal { pad: 0x5a5a };
/// seal_roundtrip::<Standard52, _>(&scheme, |_| 0x5a5a, &mut rand::rngs::StdRng::seed_from_u64(0));
/// ```
pub fn seal_roundtrip<D, S>(scheme: &S, token_for: impl Fn(SlotId) -> S::Token, rng: &mut dyn Rng)
where
    D: Decked<D> + Default + Ord + Copy + Hash + Debug,
    S: Seal<D>,
{
    for (i, card) in D::deck().cards().iter().enumerate() {
        let slot = SlotId::new(u16::try_from(i).unwrap_or(u16::MAX));
        let sealed = match scheme.seal(*card, slot, rng) {
            Ok(sealed) => sealed,
            Err(e) => panic!("seal failed at slot {slot}: {e}"),
        };
        match scheme.unseal(&sealed, slot, &token_for(slot)) {
            Ok(back) => assert_eq!(back, *card, "round-trip mismatch at slot {slot}"),
            Err(e) => panic!("unseal failed at slot {slot}: {e}"),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__plaintext_tests {
    use super::*;
    use crate::prelude::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn plaintext_seal__roundtrip_law() {
        let scheme = PlaintextSeal::new(42);
        seal_roundtrip::<French, _>(&scheme, |_| PlainToken(42), &mut StdRng::seed_from_u64(0));
        seal_roundtrip::<Tarot, _>(&scheme, |_| PlainToken(42), &mut StdRng::seed_from_u64(1));
    }

    #[test]
    fn plaintext_seal__wrong_token_errors() {
        let scheme = PlaintextSeal::new(42);
        let card = Card::<French>::from(FrenchBasicCard::ACE_SPADES);
        let slot = SlotId::new(3);
        let sealed = scheme
            .seal(card, slot, &mut StdRng::seed_from_u64(0))
            .unwrap();
        assert_eq!(
            scheme.unseal(&sealed, slot, &PlainToken(41)),
            Err(PlainSealError::WrongToken)
        );
    }

    /// Documents the double's total lack of secrecy: the payload *is* the card.
    #[test]
    fn plaintext_seal__sealed_is_the_card() {
        let scheme = PlaintextSeal::new(1);
        let card = Card::<French>::from(FrenchBasicCard::TREY_DIAMONDS);
        let sealed = scheme
            .seal(card, SlotId::new(0), &mut StdRng::seed_from_u64(0))
            .unwrap();
        assert_eq!(sealed, card);
    }
}
