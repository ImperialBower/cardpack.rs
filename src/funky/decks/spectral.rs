use crate::funky::types::buffoon_card::BuffoonCard;

/// The 18 **Spectral** cards — Balatro's high-risk consumables.
///
/// Mirrors [`MajorArcana`](crate::funky::decks::tarot::MajorArcana): a marker
/// struct holding the `DECK`, with the card literals in the [`card`] submodule.
/// Every card is `card_type: BCardType::Spectral`; the four seal-adding spectrals
/// (Talisman, Deja Vu, Trance, Medium) carry `MPip::Blank` because no seal
/// subsystem exists — their effects wait on a future Seals EPIC. The rest carry a
/// descriptive `MPip` wired across EPIC-01e Phases 1–3.
pub struct Spectral {}

impl Spectral {
    pub const DECK_SIZE: usize = 18;

    pub const DECK: [BuffoonCard; Self::DECK_SIZE] = [
        card::FAMILIAR,
        card::GRIM,
        card::INCANTATION,
        card::TALISMAN,
        card::AURA,
        card::WRAITH,
        card::SIGIL,
        card::OUIJA,
        card::ECTOPLASM,
        card::IMMOLATE,
        card::ANKH,
        card::DEJA_VU,
        card::HEX,
        card::TRANCE,
        card::MEDIUM,
        card::CRYPTID,
        card::THE_SOUL,
        card::BLACK_HOLE,
    ];
}

pub mod card {
    use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
    use crate::funky::types::edition::Edition;
    use crate::funky::types::mpip::MPip;
    use crate::prelude::{Pip, PipType};

    /// The shared suit every spectral card wears — the spectral analogue of
    /// `TarotSuit::MAJOR_ARCANA`.
    const SPECTRAL_SUIT: Pip = Pip {
        weight: 100,
        pip_type: PipType::Suit,
        index: 's',
        symbol: '👻',
        value: 0,
    };

    /// Build a spectral rank `Pip`. Each card's `weight` is distinct, which is
    /// what makes the 18 cards distinct even while every effect is still
    /// `MPip::Blank` (a `BuffoonCard` compares by all fields, and `Pip` by all of
    /// its own — so distinct weights alone guarantee 18 unique cards).
    const fn rank(weight: usize, index: char, symbol: char) -> Pip {
        Pip {
            weight,
            pip_type: PipType::Rank,
            index,
            symbol,
            value: 0,
        }
    }

    macro_rules! spectral {
        ($name:ident, $weight:expr, $index:expr, $symbol:expr, $mpip:expr) => {
            pub const $name: BuffoonCard = BuffoonCard {
                suit: SPECTRAL_SUIT,
                rank: rank($weight, $index, $symbol),
                card_type: BCardType::Spectral,
                enhancement: $mpip,
                resell_value: 1,
                edition: Edition::None,
                debuffed: false,
            };
        };
    }

    // Effects are wired across EPIC-01e Phases 1–3; every card is `Blank` here.
    // The four seal spectrals (Talisman, Deja Vu, Trance, Medium) stay `Blank`
    // permanently — seals do not exist.
    spectral!(FAMILIAR, 18, 'F', '👪', MPip::Blank);
    spectral!(GRIM, 17, 'G', '💀', MPip::Blank);
    spectral!(INCANTATION, 16, 'I', '📜', MPip::Blank);
    spectral!(TALISMAN, 15, 'T', '🧿', MPip::Blank);
    spectral!(AURA, 14, 'A', '✨', MPip::Blank);
    spectral!(WRAITH, 13, 'W', '👻', MPip::Blank);
    spectral!(SIGIL, 12, 'S', '🔯', MPip::Blank);
    spectral!(OUIJA, 11, 'O', '🪧', MPip::Blank);
    spectral!(ECTOPLASM, 10, 'E', '🫧', MPip::Blank);
    spectral!(IMMOLATE, 9, 'M', '🔥', MPip::Blank);
    spectral!(ANKH, 8, 'K', '☥', MPip::Blank);
    spectral!(DEJA_VU, 7, 'D', '🔁', MPip::Blank);
    spectral!(HEX, 6, 'H', '⬡', MPip::Blank);
    spectral!(TRANCE, 5, 'R', '🌀', MPip::Blank);
    spectral!(MEDIUM, 4, 'U', '🔮', MPip::Blank);
    spectral!(CRYPTID, 3, 'C', '🦎', MPip::Blank);
    spectral!(THE_SOUL, 2, 'L', '🌟', MPip::Blank);
    spectral!(BLACK_HOLE, 1, 'B', '🕳', MPip::Blank);
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__decks__spectral_tests {
    use super::*;
    use crate::funky::types::buffoon_card::BCardType;
    use std::collections::HashSet;

    #[test]
    fn deck__has_eighteen_cards() {
        assert_eq!(Spectral::DECK.len(), Spectral::DECK_SIZE);
        assert_eq!(Spectral::DECK_SIZE, 18);
    }

    #[test]
    fn deck__is_all_spectral() {
        for card in Spectral::DECK {
            assert_eq!(
                card.card_type,
                BCardType::Spectral,
                "{card} is not a Spectral"
            );
        }
    }

    #[test]
    fn deck__cards_are_distinct() {
        // Distinct rank weights make the 18 cards unique even while every effect
        // is still `Blank` — the invariant the deck's identity rests on.
        let unique: HashSet<_> = Spectral::DECK.iter().collect();
        assert_eq!(
            unique.len(),
            Spectral::DECK_SIZE,
            "duplicate spectral cards"
        );
    }
}
