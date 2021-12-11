use crate::cards::pile::Pile;

/// A Pack is an immutable pile of cards. Packs are designed to be a flexible representation of
/// a deck, stack, discard pile, or hand.
///
/// Packs available are for a traditional 52 card French Deck, pinochle, spades, skat and tarot.
///
/// # Usage:
/// ```
/// let pack = cardpack::Pack::french_deck();
///
/// let mut shuffled = pack.cards().shuffle();
/// let sb = shuffled.draw(2).unwrap();
/// let bb = shuffled.draw(2).unwrap();
///
/// println!("small blind: {}", sb.to_symbol_index());
/// println!("big blind:   {}", bb);
///
/// println!();
/// println!("flop : {}", shuffled.draw(3).unwrap());
/// println!("turn : {}", shuffled.draw(1).unwrap());
/// println!("river: {}", shuffled.draw(1).unwrap());
///
/// ```
///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Pack {
    cards: Pile,
}

impl Pack {
    fn new(cards: Pile) -> Pack {
        Pack { cards }
    }

    /// Returns true of the combined Cards from the passed in Vector match the Cards in the Pack.
    pub fn is_complete(&self, piles: &[Pile]) -> bool {
        let mut pile = Pile::pile_on(piles.to_vec());
        pile.sort_in_place();
        pile == self.cards
    }

    /// Returns a reference to the cards in the Pack.
    pub fn cards(&self) -> &Pile {
        &self.cards
    }

    pub fn canasta_deck() -> Pack {
        let pile = Pile::pile_up(2, Pile::canasta_single_deck);
        let pile = pile.sort();
        Pack::new(pile)
    }

    pub fn euchre_deck() -> Pack {
        Pack::new(Pile::euchre_deck())
    }

    pub fn hand_and_foot_deck() -> Pack {
        let pile = Pile::pile_up(5, Pile::canasta_base_single_deck);
        let pile = pile.sort();
        Pack::new(pile)
    }

    pub fn french_deck() -> Pack {
        Pack::new(Pile::french_deck())
    }

    pub fn french_deck_with_jokers() -> Pack {
        Pack::new(Pile::french_deck_with_jokers())
    }

    pub fn pinochle_deck() -> Pack {
        Pack::new(Pile::pinochle_deck())
    }

    pub fn skat_deck() -> Pack {
        Pack::new(Pile::skat_deck())
    }

    pub fn spades_deck() -> Pack {
        Pack::new(Pile::spades_deck())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_deck_tests {
    use super::*;

    #[test]
    fn is_complete() {
        let deck = Pack::french_deck();
        let mut cards = deck.cards.shuffle();
        let south = cards.draw(13).unwrap();
        let west = cards.draw(13).unwrap();
        let north = cards.draw(13).unwrap();
        let east = cards.draw(13).unwrap();
        let v = vec![south, west, north, east];

        let pile = Pile::pile_on(v.to_vec());

        assert_eq!(52, pile.len());
        assert!(deck.is_complete(&v));
    }

    #[test]
    fn is_complete_ne() {
        let deck = Pack::french_deck();
        let mut cards = deck.cards.shuffle();
        let south = cards.draw(13).unwrap();
        let west = cards.draw(13).unwrap();
        let north = cards.draw(13).unwrap();
        let east = cards.draw(12).unwrap();
        let v = vec![south, west, north, east];

        let pile = Pile::pile_on(v.clone());

        assert_eq!(51, pile.len());
        assert!(!deck.is_complete(&v));
    }
}
