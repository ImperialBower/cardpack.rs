use std::collections::HashMap;
use std::fmt;

use crate::cards::decks::deck_error::DeckError;
use crate::cards::decks::standard52_set::Standard52Set;
use crate::cards::rank::{Rank, BLANK_RANK};
use crate::cards::suit::Suit;
use crate::{Card, Pack, Pile};

/// `Standard52` is a representation of a deck of cards used to play
/// most versions of poker. It is useful to determine if a `Card` belongs
/// in the deck and to deserialize Cards, Piles and decks from index strings.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Standard52 {
    pub pack: Pack,
    pub deck: Pile,
}

impl Standard52 {
    /// # Errors
    ///
    /// Will return `DeckError::PilePackMismatch` error if `Pile` passed in isn't a `FrenchDeck`.
    pub fn new_from_pile(pile: Pile) -> Result<Standard52, DeckError> {
        let standard52 = Standard52 {
            pack: Pack::french_deck(),
            deck: pile,
        };

        if standard52.is_complete() {
            Ok(standard52)
        } else {
            Err(DeckError::PilePackMismatch)
        }
    }

    #[must_use]
    pub fn new_shuffled() -> Standard52 {
        Standard52 {
            pack: Pack::french_deck(),
            deck: Pile::french_deck().shuffle(),
        }
    }

    /// # Errors
    ///
    /// Will return `DeckError::InvalidIndex` if passed in index is incomplete.
    pub fn from_index(card_str: &'static str) -> Result<Standard52, DeckError> {
        let standard52 = Standard52 {
            pack: Pack::french_deck(),
            deck: Standard52::pile_from_index(card_str)?,
        };

        if standard52.is_complete() {
            Ok(standard52)
        } else {
            Err(DeckError::InvalidIndex)
        }
    }

    /// # Errors
    ///
    /// Will return `DeckError::InvalidIndex` if passed in index is invalid.
    pub fn pile_from_index(card_str: &'static str) -> Result<Pile, DeckError> {
        let mut pile = Pile::default();
        for index in card_str.split_whitespace() {
            let card = Standard52::card_from_index(index);

            if card.is_valid() {
                pile.push(card);
            } else {
                return Err(DeckError::InvalidIndex);
            }
        }
        Ok(pile)
    }

    /// Validating method that takes a `Standard52` index string and returns a `Pile`,
    /// making sure that there are no duplicate valid cards in the string.
    ///
    /// This method is doing a lot :-P
    ///
    /// # Errors
    ///
    /// Will return `DeckError::InvalidIndex` if passed in index is invalid.
    /// Will return `DeckError::DuplicateCard` if the index has the same `Card` more
    /// than once.
    ///
    /// # Panics
    ///
    /// Should not be possible.
    #[allow(clippy::question_mark)]
    pub fn pile_from_index_validated(card_str: &'static str) -> Result<Pile, DeckError> {
        let mut set = Standard52Set::default();
        let pile = Standard52::pile_from_index(card_str);
        if pile.is_err() {
            return pile;
        }

        for card in pile.unwrap() {
            let inserted = set.insert(card);
            if !inserted {
                return Err(DeckError::DuplicateCard);
            }
        }

        Ok(set.to_pile())
    }

    /// # Errors
    ///
    /// Will return `DeckError::PilePackMismatch` if `Pile` passed contains a card that isn't
    /// in the `Standard52` deck.
    pub fn pile_from_pile(&self, pile: Pile) -> Result<Pile, DeckError> {
        let mut r = Pile::default();
        for card in pile {
            if self.is_valid_card(&card) {
                r.push(card);
            } else {
                return Err(DeckError::PilePackMismatch);
            }
        }
        Ok(r)
    }

    pub fn draw(&mut self, x: usize) -> Option<Pile> {
        if x > self.deck.len() || x < 1 {
            None
        } else {
            let mut cards = Pile::default();
            for _ in 0..x {
                cards.push(self.deck.draw_first()?);
            }
            Some(cards)
        }
    }

    #[must_use]
    pub fn to_index(&self) -> String {
        self.deck.to_index()
    }

    #[must_use]
    pub fn to_index_str(&self) -> &'static str {
        self.deck.to_index_str()
    }

    #[must_use]
    pub fn to_symbol_index(&self) -> String {
        self.deck.to_symbol_index()
    }

    /// A Standard52 deck is complete if a sorted Pile of the deck is equal to it's Pack.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        let pile = self.deck.sort();
        &pile == self.pack.cards()
    }

    #[must_use]
    pub fn card_from_index(index: &'static str) -> Card {
        let rank = Rank::from_french_deck_index(Standard52::rank_str_from_index(index));
        let suit = Suit::from_french_deck_index(Standard52::suit_char_from_index(index));

        if rank.is_blank() || suit.is_blank() {
            Card::blank_card()
        } else {
            Card::new(rank, suit)
        }
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn card_from_string(index: String) -> Card {
        let char_vec: Vec<char> = index.chars().collect();

        let mut rank = Rank::default();
        let mut suit = Suit::default();

        if let Some(r) = char_vec.get(0) {
            rank = Rank::from_french_deck_char(*r);
        }

        if let Some(s) = char_vec.get(1) {
            suit = Suit::from_french_deck_index(*s);
        }

        if rank.is_blank() || suit.is_blank() {
            Card::blank_card()
        } else {
            Card::new(rank, suit)
        }
    }

    #[must_use]
    pub fn is_valid_card(&self, card: &Card) -> bool {
        self.pack.contains(card)
    }

    fn rank_str_from_index(card_str: &'static str) -> &'static str {
        if card_str.len() < 2 {
            return BLANK_RANK;
        }
        &card_str[..1]
    }

    fn suit_char_from_index(card_str: &'static str) -> char {
        if card_str.len() < 2 {
            return '_';
        }
        card_str.char_indices().nth(1).unwrap().1
    }

    // Suit HashMap Functions

    /// Returns `HashMap` of Piles of Cards sorted by the Standard52 Suits.
    ///
    /// <https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry//>
    /// <https://www.reddit.com/r/rust/comments/9xho3i/i_have_a_hashmap_that_pairs_strings_with_vectors//>
    #[must_use]
    pub fn sort_by_suit(pile: &Pile) -> HashMap<Suit, Pile> {
        let mut sorted: HashMap<Suit, Pile> = HashMap::new();

        for suit in Suit::generate_french_suits() {
            let cards_by_suit = pile.cards_by_suit(suit);
            if !cards_by_suit.is_empty() {
                sorted.insert(suit, Pile::from_vector(cards_by_suit));
            }
        }

        sorted
    }

    /// Returns true if five or more cards in a `Pile` are of the same `Suit`.
    ///
    /// NOTE: This method is non optimal and is primarily for verification purposes.
    #[must_use]
    pub fn is_flush(pile: &Pile) -> bool {
        let hash_map = Standard52::sort_by_suit(pile);
        for c in hash_map.values() {
            if c.len() > 4 {
                return true;
            }
        }
        false
    }
}

impl Default for Standard52 {
    fn default() -> Standard52 {
        Standard52 {
            pack: Pack::french_deck(),
            deck: Pile::french_deck(),
        }
    }
}

impl fmt::Display for Standard52 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = self.to_index();
        write!(f, "{}", sig)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod standard52_tests {
    use super::*;
    use crate::{CLUBS, DIAMONDS, FIVE, FOUR, HEARTS, KING, QUEEN, SPADES, TEN, THREE, TWO};
    use rstest::rstest;

    #[test]
    fn from_index() {
        let index_string = "2S 3D QS KH 3C 3S TC 9H 3H 6H QD 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";

        let mut standard52 = Standard52::from_index(index_string).unwrap();

        assert_eq!(
            Card::from_index_strings(TWO, SPADES),
            standard52.deck.draw_first().unwrap()
        );
        assert_eq!(
            Card::from_index_strings(THREE, DIAMONDS),
            standard52.deck.draw_first().unwrap()
        );
    }

    #[test]
    fn from_index__invalid() {
        let index_string = "2S 3D QS K 3C 3S TC 9H 3H 6H QD 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";

        let standard52 = Standard52::from_index(index_string);

        assert!(standard52.is_err())
    }

    #[test]
    fn to_index() {
        let expected = "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C".to_string();
        assert_eq!(expected, Standard52::default().to_index())
    }

    #[test]
    fn to_index_str() {
        let expected = "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C";
        assert_eq!(expected, Standard52::default().to_index_str())
    }

    #[test]
    fn to_symbol_index() {
        let expected = "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣".to_string();
        assert_eq!(expected, Standard52::default().to_symbol_index())
    }

    #[test]
    fn from_index_shuffled() {
        let starter = Standard52::new_shuffled();

        let standard52 = Standard52::from_index(starter.to_index_str()).unwrap();

        assert!(standard52.is_complete());
        assert_eq!(starter, standard52);
    }

    #[test]
    fn from_index_shuffled__symbol_index() {
        let index = "8♣ 4♣ K♥ Q♦ K♦ 8♥ 5♦ T♣ 9♦ J♣ T♠ 2♠ 4♥ 2♦ 3♠ 5♥ 3♦ A♣ T♥ 7♠ 4♠ K♠ 5♠ 7♣ A♥ K♣ J♠ A♠ Q♥ 2♣ 6♦ J♦ 6♠ 8♠ T♦ 9♠ 7♦ 8♦ 7♥ Q♣ 4♦ 9♣ J♥ 3♣ 6♥ 5♣ A♦ 3♥ 6♣ Q♠ 2♥ 9♥";

        let standard52 = Standard52::from_index(index).unwrap();

        assert!(standard52.is_complete());
    }

    #[test]
    fn from_index__invalid_index__invalid_index_error() {
        let index = "8 4♣ K♥ Q♦ K♦ 8♥ 5♦ T♣ 9♦ J♣ T♠ 2♠ 4♥ 2♦ 3♠ 5♥ 3♦ A♣ T♥ 7♠ 4♠ K♠ 5♠ 7♣ A♥ K♣ J♠ A♠ Q♥ 2♣ 6♦ J♦ 6♠ 8♠ T♦ 9♠ 7♦ 8♦ 7♥ Q♣ 4♦ 9♣ J♥ 3♣ 6♥ 5♣ A♦ 3♥ 6♣ Q♠ 2♥ 9♥";

        let actual_error = Standard52::from_index(index).unwrap_err();

        assert_eq!(actual_error, DeckError::InvalidIndex);
    }

    #[test]
    fn pile_from_index() {
        let index_string = "2S 3D QS KH 3C 3S TC";

        let pile = Standard52::pile_from_index(index_string);

        assert!(pile.is_ok());
        let pile = pile.unwrap();
        assert_eq!(pile.cards().len(), 7);
        assert!(pile.contains(&Card::from_index_strings(TWO, SPADES)));
        assert!(pile.contains(&Card::from_index_strings(THREE, DIAMONDS)));
        assert!(pile.contains(&Card::from_index_strings(QUEEN, SPADES)));
        assert!(pile.contains(&Card::from_index_strings(KING, HEARTS)));
        assert!(pile.contains(&Card::from_index_strings(THREE, CLUBS)));
        assert!(pile.contains(&Card::from_index_strings(THREE, SPADES)));
        assert!(pile.contains(&Card::from_index_strings(TEN, CLUBS)));
    }

    /// <https://zhauniarovich.com/post/2021/2021-01-testing-errors-in-rust//>
    #[test]
    fn pile_from_index__invalid_index__invalid_index_error() {
        let index = "2S 3D QS K 3C 3S TC";

        let actual_error = Standard52::pile_from_index(index).unwrap_err();

        assert_eq!(actual_error, DeckError::InvalidIndex);
    }

    #[test]
    fn is_complete() {
        assert!(Standard52::default().is_complete());
        assert!(Standard52::new_shuffled().is_complete());
        assert!(
            Standard52::new_from_pile(Pile::french_deck().draw(52).unwrap())
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn is_complete__french_deck_with_jokers__false() {
        let standard52 = Standard52 {
            pack: Pack::french_deck(),
            deck: Pile::french_deck_with_jokers(),
        };

        assert!(!standard52.is_complete());
    }

    #[test]
    fn is_complete__missing_cards__false() {
        let standard52 = Standard52 {
            pack: Pack::french_deck(),
            deck: Pile::french_deck().shuffle().draw(50).unwrap(),
        };

        assert!(!standard52.is_complete());
    }

    #[test]
    fn rank_str_from_index() {
        assert_eq!("2", Standard52::rank_str_from_index("2S"));
    }

    #[test]
    fn suit_char_from_index() {
        assert_eq!('S', Standard52::suit_char_from_index("2S"));
        assert_eq!('♠', Standard52::suit_char_from_index("2♠"));
    }

    #[rstest]
    #[case("2S", Card::from_index_strings(TWO, SPADES))]
    #[case("2s", Card::from_index_strings(TWO, SPADES))]
    #[case("2♠", Card::from_index_strings(TWO, SPADES))]
    #[case("3S", Card::from_index_strings(THREE, SPADES))]
    #[case("3♠", Card::from_index_strings(THREE, SPADES))]
    #[case("4♠", Card::from_index_strings(FOUR, SPADES))]
    #[case("4S", Card::from_index_strings(FOUR, SPADES))]
    #[case("5♠", Card::from_index_strings(FIVE, SPADES))]
    #[case("5S", Card::from_index_strings(FIVE, SPADES))]
    fn card_from_index(#[case] input: &'static str, #[case] expected: Card) {
        assert_eq!(expected, Standard52::card_from_index(input));
    }

    #[rstest]
    #[case("XX")]
    #[case("2X")]
    #[case("XS")]
    #[case("  ")]
    #[case(" ")]
    #[case("")]
    fn card_from_index__invalid_index(#[case] input: &'static str) {
        assert_eq!(Card::blank_card(), Standard52::card_from_index(input));
    }

    #[rstest]
    #[case(String::from("2S"), Card::from_index_strings(TWO, SPADES))]
    #[case(String::from("2s"), Card::from_index_strings(TWO, SPADES))]
    #[case(String::from("2♠"), Card::from_index_strings(TWO, SPADES))]
    #[case(String::from("3S"), Card::from_index_strings(THREE, SPADES))]
    #[case(String::from("3♠"), Card::from_index_strings(THREE, SPADES))]
    #[case(String::from("4♠"), Card::from_index_strings(FOUR, SPADES))]
    #[case(String::from("4S"), Card::from_index_strings(FOUR, SPADES))]
    #[case(String::from("5♠"), Card::from_index_strings(FIVE, SPADES))]
    #[case(String::from("5S"), Card::from_index_strings(FIVE, SPADES))]
    fn card_from_string(#[case] input: String, #[case] expected: Card) {
        assert_eq!(expected, Standard52::card_from_string(input));
    }

    #[rstest]
    #[case(String::from("XX"))]
    #[case(String::from("2X"))]
    #[case(String::from("XS"))]
    #[case(String::from("  "))]
    #[case(String::from(" "))]
    #[case(String::from(""))]
    fn card_from_string__invalid_index(#[case] input: String) {
        assert_eq!(Card::blank_card(), Standard52::card_from_string(input));
    }

    #[test]
    fn sort_by_suit() {
        let pile = Standard52::pile_from_index("2S 3S 9S TS QS JH Ac").unwrap();

        let sorted = Standard52::sort_by_suit(&pile);

        assert!(sorted.contains_key(&Suit::new(SPADES)));
        assert!(sorted.contains_key(&Suit::new(HEARTS)));
        assert!(sorted.contains_key(&Suit::new(CLUBS)));
        assert!(!sorted.contains_key(&Suit::new(DIAMONDS)));
    }

    #[rstest]
    #[case("2S 3S 9S TS QS")]
    fn to_a_flush(#[case] input: &'static str) {
        let pile = Standard52::pile_from_index(input).unwrap();

        let _sorted = Standard52::sort_by_suit(&pile);
    }

    #[rstest]
    #[case("2S 3S 9S TS QS")]
    #[case("2S 3S 9S TS QS AH QD")]
    fn is_flush(#[case] input: &'static str) {
        assert!(Standard52::is_flush(
            &Standard52::pile_from_index(input).unwrap()
        ));
    }

    #[rstest]
    #[case("2S 3S 9D TS QS")]
    #[case("2S 3S 9S TD QS AH QD")]
    fn is_flush__false(#[case] input: &'static str) {
        assert!(!Standard52::is_flush(
            &Standard52::pile_from_index(input).unwrap()
        ));
    }
}
