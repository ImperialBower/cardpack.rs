use crate::basic::types::card::Card;
use crate::basic::types::pips::{Pip, PipType};
use crate::common::traits::{CKCRevised, DeckedBase};
use crate::common::utils::Bit;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

/// I've created this intermediary struct to make it easier to mix and match cards for related decks.
/// Whilst [`Card`] needs to be generic so that we can easily share processing code in collections,
/// the raw data in the [`BasicCard`] should be simple and flexible.
///
/// The [`BasicCard`] struct is organized so that the suit [`Pip`] is first, followed by the rank
/// [`Pip`] so that the default sorting for a collection is done suit first.
///
/// **NOTE:**  The [`Ord`] and [`PartialOrd`] are customize so that the sorts are done in reverse
/// order. This may be a mistake, since vectors are suboptimal taking from the beginning.
///
/// TODO RF: Structure the code so that the end of the vector is treated as the top of the deck
/// in terms of how it is interacted with. So when you call `draw()` on a deck you are taking from
/// the bottom of the vector.
///
/// [Playing cards in Unicode](https://en.wikipedia.org/wiki/Playing_cards_in_Unicode)
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BasicCard {
    pub suit: Pip,
    pub rank: Pip,
}

impl BasicCard {
    /// The original version of this code just passes the error down to the caller:
    ///
    /// ```
    /// use std::error::Error;
    /// use std::fs::File;
    /// use std::io::Read;
    /// use cardpack::prelude::*;
    ///
    /// fn cards_from_file(file_path: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
    /// let mut file = File::open(file_path)?;
    ///     let mut contents = String::new();
    ///
    ///     file.read_to_string(&mut contents)?;
    ///
    ///     BasicCard::cards_from_str(&contents)
    /// }
    /// ```
    ///
    /// In the revised version, we're going to log an error before continuing.
    ///
    /// # Errors
    ///
    /// Throws an error for an invalid path of invalid data.
    pub fn cards_from_yaml_file(file_path: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        BasicCard::cards_from_str(&contents)
    }

    ///
    ///
    /// # Errors
    ///
    /// Throws an error for an invalid path of invalid data.
    pub fn cards_from_str(yaml_str: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
        let cards: Vec<BasicCard> = serde_yml::from_str(yaml_str)?;

        Ok(cards)
    }

    /// The index is the most basic way to represent a `Card` as a `String` using
    /// only basic characters. It is made up of the rank [`Pip`] index followed by the
    /// suit [`Pip`] index.
    ///
    /// For example, the Jack of Diamonds index value is `JD`, while it's
    /// display value is `J♦`:
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(FrenchBasicCard::JACK_DIAMONDS.index(), "JD");
    /// assert_eq!(FrenchBasicCard::JACK_DIAMONDS.to_string(), "J♦");
    /// ```
    #[must_use]
    pub fn index(&self) -> String {
        format!("{}{}", self.rank.index, self.suit.index)
    }

    /// Returns true if either the rank [`Pip`] or the suit [`Pip`] has a value of `_`,
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rank.index == Pip::BLANK_INDEX || self.suit.index == Pip::BLANK_INDEX
    }
}

impl CKCRevised for BasicCard {
    #[must_use]
    fn get_ckc_number(&self) -> u32 {
        if self.is_blank() {
            return 0;
        }
        self.ckc_rank_number() + self.ckc_suit_number()
    }

    fn ckc_rank_number(&self) -> u32 {
        self.ckc_rank_bits() | self.ckc_rank_shift8() | self.ckc_get_prime()
    }

    // TODO: This needs to be moved out of Basic. Maybe a trait? Maybe just move it out altogether?
    fn ckc_suit_number(&self) -> u32 {
        if self.suit.pip_type == PipType::Joker {
            return 0;
        }
        match self.suit.value {
            1..=4 => 1 << (Bit::SUIT_FLAG_SHIFT + self.suit.value),
            _ => 0,
        }
    }

    fn ckc_rank_bits(&self) -> u32 {
        1 << (Bit::RANK_FLAG_SHIFT + self.rank.weight)
    }

    fn ckc_get_prime(&self) -> u32 {
        if self.rank.weight as usize >= Pip::PRIMES.len() {
            0
        } else {
            Pip::PRIMES[(self.rank.weight) as usize]
        }
    }

    #[must_use]
    fn ckc_rank_shift8(&self) -> u32 {
        self.rank.weight << 8
    }
}

impl Display for BasicCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank.symbol, self.suit.symbol)
    }
}

impl<DeckType: DeckedBase> From<Card<DeckType>> for BasicCard {
    fn from(card: Card<DeckType>) -> Self {
        Self {
            suit: card.base_card.suit,
            rank: card.base_card.rank,
        }
    }
}

/// Inverts the order so that the highest card comes first.
impl Ord for BasicCard {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .suit
            .cmp(&self.suit)
            .then_with(|| other.rank.cmp(&self.rank))
    }
}

impl PartialOrd for BasicCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__basic_card_tests {
    use super::*;
    use crate::basic::decks::standard52::Standard52;
    use crate::basic::types::basic_card::BasicCard;
    use crate::prelude::{Deck, Decked, French};
    use ckc_rs::CardNumber;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn cards_from_yaml_file() {
        let cards = BasicCard::cards_from_yaml_file("src/basic/decks/yaml/french.yaml").unwrap();

        assert_eq!(cards.len(), 54);
        assert_eq!(cards, Deck::<French>::base_vec())
    }

    #[test]
    fn is_blank() {
        let base_card = BasicCard::default();
        assert!(base_card.is_blank());
    }

    #[rstest]
    #[case("A♠", CardNumber::ACE_SPADES)]
    #[case("ks", CardNumber::KING_SPADES)]
    #[case("QS", CardNumber::QUEEN_SPADES)]
    #[case("J♠", CardNumber::JACK_SPADES)]
    #[case("TS", CardNumber::TEN_SPADES)]
    #[case("9s", CardNumber::NINE_SPADES)]
    #[case("8♠", CardNumber::EIGHT_SPADES)]
    #[case("7S", CardNumber::SEVEN_SPADES)]
    #[case("6♠", CardNumber::SIX_SPADES)]
    #[case("5S", CardNumber::FIVE_SPADES)]
    #[case("4♠", CardNumber::FOUR_SPADES)]
    #[case("3s", CardNumber::TREY_SPADES)]
    #[case("2S", CardNumber::DEUCE_SPADES)]
    #[case("A♥", CardNumber::ACE_HEARTS)]
    #[case("k♥", CardNumber::KING_HEARTS)]
    #[case("QH", CardNumber::QUEEN_HEARTS)]
    #[case("jh", CardNumber::JACK_HEARTS)]
    #[case("T♥", CardNumber::TEN_HEARTS)]
    #[case("9♥", CardNumber::NINE_HEARTS)]
    #[case("8h", CardNumber::EIGHT_HEARTS)]
    #[case("7H", CardNumber::SEVEN_HEARTS)]
    #[case("6h", CardNumber::SIX_HEARTS)]
    #[case("5H", CardNumber::FIVE_HEARTS)]
    #[case("4♥", CardNumber::FOUR_HEARTS)]
    #[case("3♥", CardNumber::TREY_HEARTS)]
    #[case("2h", CardNumber::DEUCE_HEARTS)]
    #[case("A♦", CardNumber::ACE_DIAMONDS)]
    #[case("k♦", CardNumber::KING_DIAMONDS)]
    #[case("Q♦", CardNumber::QUEEN_DIAMONDS)]
    #[case("Jd", CardNumber::JACK_DIAMONDS)]
    #[case("tD", CardNumber::TEN_DIAMONDS)]
    #[case("9♦", CardNumber::NINE_DIAMONDS)]
    #[case("8D", CardNumber::EIGHT_DIAMONDS)]
    #[case("7♦", CardNumber::SEVEN_DIAMONDS)]
    #[case("6D", CardNumber::SIX_DIAMONDS)]
    #[case("5D", CardNumber::FIVE_DIAMONDS)]
    #[case("4♦", CardNumber::FOUR_DIAMONDS)]
    #[case("3♦", CardNumber::TREY_DIAMONDS)]
    #[case("2d", CardNumber::DEUCE_DIAMONDS)]
    #[case("a♣", CardNumber::ACE_CLUBS)]
    #[case("k♣", CardNumber::KING_CLUBS)]
    #[case("QC", CardNumber::QUEEN_CLUBS)]
    #[case("jc", CardNumber::JACK_CLUBS)]
    #[case("tC", CardNumber::TEN_CLUBS)]
    #[case("9♣", CardNumber::NINE_CLUBS)]
    #[case("8♣", CardNumber::EIGHT_CLUBS)]
    #[case("7c", CardNumber::SEVEN_CLUBS)]
    #[case("6♣", CardNumber::SIX_CLUBS)]
    #[case("5C", CardNumber::FIVE_CLUBS)]
    #[case("4c", CardNumber::FOUR_CLUBS)]
    #[case("3C", CardNumber::TREY_CLUBS)]
    #[case("2C", CardNumber::DEUCE_CLUBS)]
    #[case("__", 0u32)]
    fn card__get_ckc_number(#[case] input: &str, #[case] expected_ckc: u32) {
        let card = Card::<Standard52>::from_str(input).unwrap();

        let base_card: BasicCard = card.into();

        assert_eq!(base_card.get_ckc_number(), expected_ckc);
    }
}
