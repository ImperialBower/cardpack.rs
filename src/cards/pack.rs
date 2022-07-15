use crate::cards::pile::Pile;
use crate::Card;

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
    fn new(cards: Pile) -> Self {
        Self { cards }
    }

    /// Returns true of the combined Cards from the passed in Vector match the Cards in the Pack.
    #[must_use]
    pub fn is_complete(&self, piles: &[Pile]) -> bool {
        let mut pile = Pile::pile_on(piles.to_vec());
        pile.sort_in_place();
        pile == self.cards
    }

    /// Returns a reference to the cards in the Pack.
    #[must_use]
    pub fn cards(&self) -> &Pile {
        &self.cards
    }

    /// Returns true if the passed in `Card` is a part of the `Pack`.
    #[must_use]
    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn canasta_deck() -> Self {
        let pile = Pile::pile_up(2, Pile::canasta_single_deck);
        let pile = pile.sort();
        Self::new(pile)
    }

    #[must_use]
    pub fn euchre_deck() -> Self {
        Self::new(Pile::euchre_deck())
    }

    pub fn hand_and_foot_deck() -> Self {
        let pile = Pile::pile_up(5, Pile::canasta_base_single_deck);
        let pile = pile.sort();
        Self::new(pile)
    }

    #[must_use]
    pub fn french_deck() -> Self {
        Self::new(Pile::french_deck())
    }

    #[must_use]
    pub fn french_deck_with_jokers() -> Self {
        Self::new(Pile::french_deck_with_jokers())
    }

    #[must_use]
    pub fn pinochle_deck() -> Self {
        Self::new(Pile::pinochle_deck())
    }

    #[must_use]
    pub fn short_deck() -> Self {
        Self::new(Pile::short_deck())
    }

    #[must_use]
    pub fn skat_deck() -> Self {
        Self::new(Pile::skat_deck())
    }

    #[must_use]
    pub fn spades_deck() -> Self {
        Self::new(Pile::spades_deck())
    }

    #[must_use]
    pub fn tarot_deck() -> Self {
        Self::new(Pile::tarot_deck())
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
