use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

/// Named [`BasicCard`] constants for tests and docs — a sampler, not all 216.
pub struct GanjifaBasicCard;
/// Rank [`Pip`]s shared by every Ganjifa deck: two courts plus a strong
/// (`10 > … > A`) and a weak (`A > … > 10`) pip ladder.
pub struct GanjifaRank;
/// The eight Mughal Ganjifa suit [`Pip`]s.
///
/// CKC suit flags only exist for suit `value` 1–4, so higher-value suits
/// share CKC numbers at equal rank weight — CKC is a poker-evaluator
/// concept and not meaningful for Ganjifa.
pub struct MughalSuit;
/// The ten Dashavatara Ganjifa suit [`Pip`]s (Vishnu's avatars).
///
/// CKC suit flags only exist for suit `value` 1–4, so higher-value suits
/// share CKC numbers at equal rank weight — CKC is a poker-evaluator
/// concept and not meaningful for Ganjifa.
pub struct DashavataraSuit;

pub const FLUENT_KEY_BASE_NAME_MUGHAL: &str = "mughal";
pub const FLUENT_KEY_BASE_NAME_DASHAVATARA: &str = "dashavatara";

impl GanjifaRank {
    // Courts — shared by both ladders. Weight 11 max keeps the CKC shift at
    // 16 + 11 = 27 < 32, safe on wasm32.
    pub const KING: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 11,
        index: 'K',
        symbol: 'K',
        value: 12,
    };
    pub const VIZIER: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 10,
        index: 'V',
        symbol: 'V',
        value: 11,
    };

    // Strong-ladder pips: 10 > 9 > … > 2 > A.
    pub const TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'T',
        symbol: 'T',
        value: 10,
    };
    pub const NINE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: '9',
        symbol: '9',
        value: 9,
    };
    pub const EIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: '8',
        symbol: '8',
        value: 8,
    };
    pub const SEVEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: '7',
        symbol: '7',
        value: 7,
    };
    pub const SIX: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: '6',
        symbol: '6',
        value: 6,
    };
    pub const FIVE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: '5',
        symbol: '5',
        value: 5,
    };
    pub const FOUR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: '4',
        symbol: '4',
        value: 4,
    };
    pub const TREY: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const DEUCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '2',
        symbol: '2',
        value: 2,
    };
    pub const ACE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: 'A',
        symbol: 'A',
        value: 1,
    };

    // Weak-ladder pips: A > 2 > … > 9 > 10. Same index/symbol/value as their
    // strong twins; only the weights invert.
    pub const WEAK_ACE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'A',
        symbol: 'A',
        value: 1,
    };
    pub const WEAK_DEUCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: '2',
        symbol: '2',
        value: 2,
    };
    pub const WEAK_TREY: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const WEAK_FOUR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: '4',
        symbol: '4',
        value: 4,
    };
    pub const WEAK_FIVE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: '5',
        symbol: '5',
        value: 5,
    };
    pub const WEAK_SIX: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: '6',
        symbol: '6',
        value: 6,
    };
    pub const WEAK_SEVEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: '7',
        symbol: '7',
        value: 7,
    };
    pub const WEAK_EIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '8',
        symbol: '8',
        value: 8,
    };
    pub const WEAK_NINE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '9',
        symbol: '9',
        value: 9,
    };
    pub const WEAK_TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: 'T',
        symbol: 'T',
        value: 10,
    };

    /// Strong-suit rank ladder in descending-weight (sorted) order.
    pub const STRONG: [Pip; 12] = [
        Self::KING,
        Self::VIZIER,
        Self::TEN,
        Self::NINE,
        Self::EIGHT,
        Self::SEVEN,
        Self::SIX,
        Self::FIVE,
        Self::FOUR,
        Self::TREY,
        Self::DEUCE,
        Self::ACE,
    ];

    /// Weak-suit rank ladder in descending-weight (sorted) order — pips
    /// inverted: Ace high, Ten low.
    pub const WEAK: [Pip; 12] = [
        Self::KING,
        Self::VIZIER,
        Self::WEAK_ACE,
        Self::WEAK_DEUCE,
        Self::WEAK_TREY,
        Self::WEAK_FOUR,
        Self::WEAK_FIVE,
        Self::WEAK_SIX,
        Self::WEAK_SEVEN,
        Self::WEAK_EIGHT,
        Self::WEAK_NINE,
        Self::WEAK_TEN,
    ];
}

impl MughalSuit {
    pub const SLAVES: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 7,
        index: 'G',
        symbol: '👤',
        value: 8,
    };
    pub const CROWNS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 6,
        index: 'T',
        symbol: '👑',
        value: 7,
    };
    pub const SWORDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 5,
        index: 'S',
        symbol: '⚔',
        value: 6,
    };
    pub const RED_COINS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 4,
        index: 'R',
        symbol: '🔴',
        value: 5,
    };
    pub const HARPS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'H',
        symbol: '🎵',
        value: 4,
    };
    pub const BILLS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'B',
        symbol: '📜',
        value: 3,
    };
    pub const WHITE_COINS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'W',
        symbol: '⚪',
        value: 2,
    };
    pub const CLOTH: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'Q',
        symbol: '🧵',
        value: 1,
    };
}

impl DashavataraSuit {
    pub const MATSYA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 9,
        index: 'M',
        symbol: '🐟',
        value: 10,
    };
    pub const KURMA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 8,
        index: 'U',
        symbol: '🐢',
        value: 9,
    };
    pub const VARAHA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 7,
        index: 'B',
        symbol: '🐗',
        value: 8,
    };
    pub const NARASIMHA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 6,
        index: 'N',
        symbol: '🦁',
        value: 7,
    };
    pub const VAMANA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 5,
        index: 'D',
        symbol: '☂',
        value: 6,
    };
    pub const PARASHURAMA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 4,
        index: 'P',
        symbol: '🪓',
        value: 5,
    };
    pub const RAMA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'R',
        symbol: '🏹',
        value: 4,
    };
    pub const KRISHNA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'K',
        symbol: '🐄',
        value: 3,
    };
    pub const JAGANNATH: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'J',
        symbol: '☸',
        value: 2,
    };
    pub const KALKI: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'C',
        symbol: '🐎',
        value: 1,
    };
}

impl GanjifaBasicCard {
    pub const KING_SLAVES: BasicCard = BasicCard::new(MughalSuit::SLAVES, GanjifaRank::KING);
    pub const WEAK_ACE_RED_COINS: BasicCard =
        BasicCard::new(MughalSuit::RED_COINS, GanjifaRank::WEAK_ACE);
    pub const KING_MATSYA: BasicCard = BasicCard::new(DashavataraSuit::MATSYA, GanjifaRank::KING);
    pub const TEN_KALKI: BasicCard = BasicCard::new(DashavataraSuit::KALKI, GanjifaRank::TEN);
}

/// Builds a Ganjifa deck: for each suit, courts + 10 pips from the strong or
/// weak ladder, in descending-weight (sorted) order.
///
/// Because the ladders and any descending-weight `suits` array are already in
/// sorted order, the output equals the deck's `sorted()` order — which
/// `Decked::validate()` requires.
///
/// ```
/// use cardpack::prelude::*;
///
/// const SLAVES_ONLY: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::SLAVES], &[true]);
/// assert_eq!(SLAVES_ONLY[0], GanjifaBasicCard::KING_SLAVES);
/// ```
///
/// # Panics
///
/// Panics (at compile time for const evaluation) if `N != S * 12`.
#[must_use]
pub const fn ganjifa_deck<const S: usize, const N: usize>(
    suits: &[Pip; S],
    strong: &[bool; S],
) -> [BasicCard; N] {
    assert!(N == S * 12, "N must equal S * 12");
    let mut deck = [BasicCard::new(
        Pip::new(PipType::Blank, 0, '_', '_'),
        Pip::new(PipType::Blank, 0, '_', '_'),
    ); N];
    let mut i = 0;
    while i < S {
        let ranks = if strong[i] {
            &GanjifaRank::STRONG
        } else {
            &GanjifaRank::WEAK
        };
        let mut j = 0;
        while j < 12 {
            deck[i * 12 + j] = BasicCard::new(suits[i], ranks[j]);
            j += 1;
        }
        i += 1;
    }
    deck
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__decks__cards__ganjifa_tests {
    use super::*;

    /// Guards `ganjifa_deck`'s `const fn` status: if the builder ever loses
    /// `const`, this declaration fails to compile.
    const _GUARD: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::SLAVES], &[true]);

    const MUGHAL_SUITS: [Pip; 8] = [
        MughalSuit::SLAVES,
        MughalSuit::CROWNS,
        MughalSuit::SWORDS,
        MughalSuit::RED_COINS,
        MughalSuit::HARPS,
        MughalSuit::BILLS,
        MughalSuit::WHITE_COINS,
        MughalSuit::CLOTH,
    ];

    const DASHAVATARA_SUITS: [Pip; 10] = [
        DashavataraSuit::MATSYA,
        DashavataraSuit::KURMA,
        DashavataraSuit::VARAHA,
        DashavataraSuit::NARASIMHA,
        DashavataraSuit::VAMANA,
        DashavataraSuit::PARASHURAMA,
        DashavataraSuit::RAMA,
        DashavataraSuit::KRISHNA,
        DashavataraSuit::JAGANNATH,
        DashavataraSuit::KALKI,
    ];

    fn assert_weights_strictly_descending(pips: &[Pip]) {
        for w in pips.windows(2) {
            assert!(
                w[0].weight > w[1].weight,
                "weights not strictly descending: {:?} then {:?}",
                w[0],
                w[1]
            );
        }
    }

    fn assert_indexes_unique(pips: &[Pip]) {
        for (i, a) in pips.iter().enumerate() {
            for b in &pips[i + 1..] {
                assert_ne!(a.index, b.index, "duplicate index char in {a:?} / {b:?}");
            }
        }
    }

    #[test]
    fn ladders__strictly_descending() {
        assert_weights_strictly_descending(&GanjifaRank::STRONG);
        assert_weights_strictly_descending(&GanjifaRank::WEAK);
        assert_weights_strictly_descending(&MUGHAL_SUITS);
        assert_weights_strictly_descending(&DASHAVATARA_SUITS);
    }

    #[test]
    fn ladders__index_uniqueness() {
        assert_indexes_unique(&GanjifaRank::STRONG);
        assert_indexes_unique(&GanjifaRank::WEAK);
        assert_indexes_unique(&MUGHAL_SUITS);
        assert_indexes_unique(&DASHAVATARA_SUITS);
    }

    #[test]
    fn ladders__courts_first() {
        assert_eq!(GanjifaRank::STRONG[0], GanjifaRank::KING);
        assert_eq!(GanjifaRank::STRONG[1], GanjifaRank::VIZIER);
        assert_eq!(GanjifaRank::WEAK[0], GanjifaRank::KING);
        assert_eq!(GanjifaRank::WEAK[1], GanjifaRank::VIZIER);
    }

    #[test]
    fn ladders__weak_mirrors_strong() {
        // Every strong pip has a weak twin sharing index, symbol, and value;
        // pip weights (ladder positions 2..12) mirror, so each strong/weak
        // pair's weights sum to 9.
        for strong in &GanjifaRank::STRONG[2..] {
            let weak = GanjifaRank::WEAK[2..]
                .iter()
                .find(|w| w.index == strong.index)
                .expect("every strong pip needs a weak twin");
            assert_eq!(strong.symbol, weak.symbol);
            assert_eq!(strong.value, weak.value);
            assert_eq!(
                strong.weight + weak.weight,
                9,
                "weights must mirror for index {:?}",
                strong.index
            );
        }
    }

    #[test]
    fn pips__types_and_values() {
        for suit in MUGHAL_SUITS.iter().chain(DASHAVATARA_SUITS.iter()) {
            assert_eq!(suit.pip_type, PipType::Suit);
            assert_eq!(suit.value, suit.weight + 1);
        }
        for rank in GanjifaRank::STRONG.iter().chain(GanjifaRank::WEAK.iter()) {
            assert_eq!(rank.pip_type, PipType::Rank);
        }
    }

    #[test]
    fn ganjifa_deck__strong_suit_uses_strong_ladder() {
        let deck: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::SLAVES], &[true]);
        for (card, rank) in deck.iter().zip(GanjifaRank::STRONG.iter()) {
            assert_eq!(card.suit, MughalSuit::SLAVES);
            assert_eq!(card.rank, *rank);
        }
    }

    #[test]
    fn ganjifa_deck__weak_suit_uses_weak_ladder() {
        let deck: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::RED_COINS], &[false]);
        for (card, rank) in deck.iter().zip(GanjifaRank::WEAK.iter()) {
            assert_eq!(card.suit, MughalSuit::RED_COINS);
            assert_eq!(card.rank, *rank);
        }
    }

    #[test]
    fn named_cards() {
        assert_eq!(GanjifaBasicCard::KING_SLAVES.index(), "KG");
        assert_eq!(GanjifaBasicCard::WEAK_ACE_RED_COINS.index(), "AR");
        assert_eq!(GanjifaBasicCard::KING_MATSYA.index(), "KM");
        assert_eq!(GanjifaBasicCard::TEN_KALKI.index(), "TC");
    }
}
