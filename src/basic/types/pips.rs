use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `PipType` is used to handle control flows for special, conditional processing of pips.
///
/// Here's a simple hypothetical example:
/// B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣
/// ```
/// use cardpack::prelude::*;
///
/// let hand = french_cards!("A♠ B🃟 Q♠ J♠ T♠");
///
/// let optimal_hand = match hand.cards_of_suit_pip_type(PipType::Joker).len() {
///   0 => hand,
///   _ => find_optimal_hand(hand),
/// };
///
/// fn find_optimal_hand(hand: Pile<French>) -> Pile<French> {
///     // Logic that returns the best scoring version of the hand with the joker.
///     hand
/// }
/// ```
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum PipType {
    #[default]
    Blank,
    Suit,
    Rank,
    Joker,
    Special,
}

/// `Pip` is the smallest unit of a [`BasicCard`](crate::basic::types::basic_card::BasicCard).
///
/// Originally, I had different structs for `Rank` and `Suit`. The, I came to the realization that
/// I could get the same results with a single struct. Eventually, I could see creating a card type
/// that has an unlimited type of different pips stored in a vector. That's a TODO for after this
/// version is done.
///
/// Each Pip is made up of the following fields:
///
/// - `weight`: A `u32` that is used for sorting.
/// - `pip_type`: Used to classify the type of pip it is.
/// - `index`: A `char` that is the key identifier for the `Pip`, such as 'A' for Ace.
/// - `symbol`: A `char` that is the visual representation of the `Pip`, such as '♠' for Spades.
/// - `value`: A `u32` that is used when a numerical valus is needed that is different than the `weight`.
///
/// Each [`BasicCard`](crate::basic::types::basic_card::BasicCard) struct is made up of two `Pips`, one
/// representing the suit of the card and another for the rank.
///
/// Here's a basic example of `Pips` in action:
///
/// ```
/// use cardpack::prelude::*;
///
/// let trey_of_hearts = BasicCard {
///    suit: Pip {
///         weight: 2,
///         pip_type: PipType::Suit,
///         index: 'H',
///         symbol: '♥',
///         value: 3,
///     },
///     rank: Pip {
///         weight: 1,
///         pip_type: PipType::Rank,
///         index: '3',
///         symbol: '3',
///         value: 3,
///     },
/// };
///
/// assert_eq!(trey_of_hearts, FrenchBasicCard::TREY_HEARTS);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pip {
    pub weight: usize,
    pub pip_type: PipType,
    pub index: char,
    pub symbol: char,
    pub value: usize,
}

impl Pip {
    /// The universal index for a blank `Pip` in a [`Card`](crate::basic::types::card::Card). Blank
    /// is the default value for all cards.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert!(Card::<French>::default().is_blank());
    /// ```
    pub const BLANK_INDEX: char = '_';

    /// TODO: HACK
    pub const PRIMES: [usize; 60] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    ];

    #[must_use]
    pub fn new(pip_type: PipType, weight: usize, index: char, symbol: char) -> Self {
        Self {
            weight,
            pip_type,
            index,
            symbol,
            ..Default::default()
        }
    }

    /// Returns the absolute difference between the weights of two `Pips`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(FrenchRank::ACE.distance(&FrenchRank::DEUCE), 12); // Calling 2s deuces really throws the AI off.
    /// assert_eq!(FrenchRank::DEUCE.distance(&FrenchRank::ACE), 12);
    /// assert_eq!(FrenchRank::DEUCE.distance(&FrenchRank::DEUCE), 0);
    /// ```
    ///
    /// **DIARY:** Hey! I learned a new function. `asb_diff()` is very cool.
    #[must_use]
    pub fn distance(&self, other: &Self) -> usize {
        self.weight.abs_diff(other.weight)
    }

    /// Factory method to update values as needed.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let expected = Pip {
    ///     pip_type: PipType::Rank,
    ///     weight: 12,
    ///     index: 'A',
    ///     symbol: 'A',
    ///     value: 22,
    /// };
    ///
    /// let updated_as: Pip = FrenchRank::ACE.update_value(22);
    ///
    /// assert_eq!(updated_as, expected);
    /// ```
    #[must_use]
    pub fn update_value(&self, value: usize) -> Self {
        Self { value, ..*self }
    }
}

impl Default for Pip {
    fn default() -> Self {
        Self {
            pip_type: PipType::Blank,
            weight: 0,
            index: Self::BLANK_INDEX,
            symbol: Self::BLANK_INDEX,
            value: 0,
        }
    }
}

impl Display for Pip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__pips_tests {
    use super::*;
    use crate::basic::types::basic_card::BasicCard;

    #[test]
    fn pip__default() {
        let pip = Pip::default();
        assert_eq!(pip.pip_type, PipType::Blank);
        assert_eq!(pip.weight, 0);
        assert_eq!(pip.index, Pip::BLANK_INDEX);
        assert_eq!(pip.symbol, Pip::BLANK_INDEX);
        assert_eq!(pip.value, 0);
    }
}
