use crate::basic::types::card::Card;
use crate::basic::types::pips::{Pip, PipType};
use crate::basic::types::traits::{CKCRevised, DeckedBase};
use crate::common::utils::Bit;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

/// Intermediary struct to help mix and match cards for related decks.
///
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
    /// Reads in a YAML file version of `BasicCard` data at the passed in location and returns a vector of `BasicCards`. See the
    /// [`Razz`](crate::basic::decks::razz::Razz) deck for an example of how to use this method.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let cards = BasicCard::cards_from_yaml_file("src/basic/decks/yaml/french.yaml").unwrap();
    ///
    /// assert_eq!(cards.len(), 54);
    /// assert_eq!(cards, Pile::<French>::base_vec());
    /// ```
    ///
    /// # Errors
    ///
    /// Throws an error for an invalid path or invalid data.
    pub fn cards_from_yaml_file(file_path: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Self::cards_from_yaml_str(&contents)
    }

    /// Takes in a YAML string and returns a vector of `BasicCards`.
    ///
    /// # Errors
    ///
    /// Throws an error for an invalid path or invalid data.
    pub fn cards_from_yaml_str(yaml_str: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let cards: Vec<Self> = serde_yml::from_str(yaml_str)?;

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
    fn get_ckc_number(&self) -> usize {
        if self.is_blank() {
            return 0;
        }
        self.ckc_rank_number() + self.ckc_suit_number()
    }

    fn ckc_rank_number(&self) -> usize {
        self.ckc_rank_bits() | self.ckc_rank_shift8() | self.ckc_get_prime()
    }

    // TODO: This needs to be moved out of Basic. Maybe a trait? Maybe just move it out altogether?
    fn ckc_suit_number(&self) -> usize {
        if self.suit.pip_type == PipType::Joker {
            return 0;
        }
        match self.suit.value {
            1..=4 => 1 << (Bit::SUIT_FLAG_SHIFT + self.suit.value),
            _ => 0,
        }
    }

    fn ckc_rank_bits(&self) -> usize {
        1 << (Bit::RANK_FLAG_SHIFT + self.rank.weight)
    }

    fn ckc_get_prime(&self) -> usize {
        if self.rank.weight >= Pip::PRIMES.len() {
            0
        } else {
            Pip::PRIMES[self.rank.weight]
        }
    }

    fn ckc_rank_shift8(&self) -> usize {
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
    use crate::prelude::{Decked, French, Pile};
    use ckc_rs::CardNumber;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn cards_from_yaml_file() {
        let cards = BasicCard::cards_from_yaml_file("src/basic/decks/yaml/french.yaml").unwrap();

        assert_eq!(cards.len(), 54);
        assert_eq!(cards, Pile::<French>::base_vec())
    }

    #[test]
    fn is_blank() {
        let base_card = BasicCard::default();
        assert!(base_card.is_blank());
    }

    #[rstest]
    #[case("A♠", CardNumber::ACE_SPADES as usize)]
    #[case("ks", CardNumber::KING_SPADES as usize)]
    #[case("QS", CardNumber::QUEEN_SPADES as usize)]
    #[case("J♠", CardNumber::JACK_SPADES as usize)]
    #[case("TS", CardNumber::TEN_SPADES as usize)]
    #[case("9s", CardNumber::NINE_SPADES as usize)]
    #[case("8♠", CardNumber::EIGHT_SPADES as usize)]
    #[case("7S", CardNumber::SEVEN_SPADES as usize)]
    #[case("6♠", CardNumber::SIX_SPADES as usize)]
    #[case("5S", CardNumber::FIVE_SPADES as usize)]
    #[case("4♠", CardNumber::FOUR_SPADES as usize)]
    #[case("3s", CardNumber::TREY_SPADES as usize)]
    #[case("2S", CardNumber::DEUCE_SPADES as usize)]
    #[case("A♥", CardNumber::ACE_HEARTS as usize)]
    #[case("k♥", CardNumber::KING_HEARTS as usize)]
    #[case("QH", CardNumber::QUEEN_HEARTS as usize)]
    #[case("jh", CardNumber::JACK_HEARTS as usize)]
    #[case("T♥", CardNumber::TEN_HEARTS as usize)]
    #[case("9♥", CardNumber::NINE_HEARTS as usize)]
    #[case("8h", CardNumber::EIGHT_HEARTS as usize)]
    #[case("7H", CardNumber::SEVEN_HEARTS as usize)]
    #[case("6h", CardNumber::SIX_HEARTS as usize)]
    #[case("5H", CardNumber::FIVE_HEARTS as usize)]
    #[case("4♥", CardNumber::FOUR_HEARTS as usize)]
    #[case("3♥", CardNumber::TREY_HEARTS as usize)]
    #[case("2h", CardNumber::DEUCE_HEARTS as usize)]
    #[case("A♦", CardNumber::ACE_DIAMONDS as usize)]
    #[case("k♦", CardNumber::KING_DIAMONDS as usize)]
    #[case("Q♦", CardNumber::QUEEN_DIAMONDS as usize)]
    #[case("Jd", CardNumber::JACK_DIAMONDS as usize)]
    #[case("tD", CardNumber::TEN_DIAMONDS as usize)]
    #[case("9♦", CardNumber::NINE_DIAMONDS as usize)]
    #[case("8D", CardNumber::EIGHT_DIAMONDS as usize)]
    #[case("7♦", CardNumber::SEVEN_DIAMONDS as usize)]
    #[case("6D", CardNumber::SIX_DIAMONDS as usize)]
    #[case("5D", CardNumber::FIVE_DIAMONDS as usize)]
    #[case("4♦", CardNumber::FOUR_DIAMONDS as usize)]
    #[case("3♦", CardNumber::TREY_DIAMONDS as usize)]
    #[case("2d", CardNumber::DEUCE_DIAMONDS as usize)]
    #[case("a♣", CardNumber::ACE_CLUBS as usize)]
    #[case("k♣", CardNumber::KING_CLUBS as usize)]
    #[case("QC", CardNumber::QUEEN_CLUBS as usize)]
    #[case("jc", CardNumber::JACK_CLUBS as usize)]
    #[case("tC", CardNumber::TEN_CLUBS as usize)]
    #[case("9♣", CardNumber::NINE_CLUBS as usize)]
    #[case("8♣", CardNumber::EIGHT_CLUBS as usize)]
    #[case("7c", CardNumber::SEVEN_CLUBS as usize)]
    #[case("6♣", CardNumber::SIX_CLUBS as usize)]
    #[case("5C", CardNumber::FIVE_CLUBS as usize)]
    #[case("4c", CardNumber::FOUR_CLUBS as usize)]
    #[case("3C", CardNumber::TREY_CLUBS as usize)]
    #[case("2C", CardNumber::DEUCE_CLUBS as usize)]
    #[case("__", 0)]
    fn card__get_ckc_number(#[case] input: &str, #[case] expected_ckc: usize) {
        let card = Card::<Standard52>::from_str(input).unwrap();

        let base_card: BasicCard = card.into();

        assert_eq!(base_card.get_ckc_number(), expected_ckc);
    }
}
