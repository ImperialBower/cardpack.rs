//! The backend's own error. `CardError` stays crypto-free (EPIC-04 decision 8).

use alloc::string::String;
use thiserror::Error;

/// What went wrong — without saying which of the wrong things it was.
///
/// Every unseal failure (bad token, wrong slot, wrong context, wrong deck,
/// tampered bytes) is the one [`Unseal`](Self::Unseal) variant. Telling them
/// apart would be an oracle.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
#[non_exhaustive]
pub enum AeadSealError {
    /// `seal` or `token_for` on a verifier-mode scheme.
    #[error("verifier-mode seal has no master key")]
    NoMasterKey,
    /// The card has no ordinal in this deck (e.g. `Card::default()`).
    #[error("card `{0}` is not in the deck")]
    CardNotInDeck(String),
    /// Authentication failed. Deliberately one variant.
    #[error("unseal failed: bad token, wrong slot, or wrong context")]
    Unseal,
    /// Cannot occur for bytes this scheme produced (the tag covers the
    /// plaintext); recorded because the decoder is total.
    #[error("authentic payload decoded to ordinal {0}, which is out of range")]
    InvalidOrdinal(u16),
    /// `D::deck_name()` does not fit the `u16` length field in the associated
    /// data. Unreachable for every shipped deck; only a consumer's own
    /// `DeckedBase` can hit it. `Codebook::encode_pile` refuses the same
    /// input, and this backend agrees with it rather than truncating.
    #[error("deck name of {0} bytes exceeds the 65535 the associated data can describe")]
    DeckNameTooLong(usize),
    /// `deal` on a pile longer than `u16::MAX` slots.
    #[error("pile of {0} cards exceeds the 65535 slots a SlotPile can name")]
    PileTooLong(usize),
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__aead__error_tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn display__all_variants() {
        assert_eq!(
            AeadSealError::NoMasterKey.to_string(),
            "verifier-mode seal has no master key"
        );
        assert_eq!(
            AeadSealError::CardNotInDeck("A♠".to_string()).to_string(),
            "card `A♠` is not in the deck"
        );
        assert_eq!(
            AeadSealError::Unseal.to_string(),
            "unseal failed: bad token, wrong slot, or wrong context"
        );
        assert_eq!(
            AeadSealError::InvalidOrdinal(99).to_string(),
            "authentic payload decoded to ordinal 99, which is out of range"
        );
        assert_eq!(
            AeadSealError::PileTooLong(70_000).to_string(),
            "pile of 70000 cards exceeds the 65535 slots a SlotPile can name"
        );
        assert_eq!(
            AeadSealError::DeckNameTooLong(70_000).to_string(),
            "deck name of 70000 bytes exceeds the 65535 the associated data can describe"
        );
    }

    #[test]
    fn error__is_std_error_send_sync() {
        fn assert_bounds<E: core::error::Error + Send + Sync + 'static>() {}
        assert_bounds::<AeadSealError>();
    }
}
