use crate::cards::pile::Pile;

/// A Pack is an immutable pile of cards. Packs are designed to be a flexible representation of
/// a deck, a stack, a discard pile, or a hand.
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
/// println!("small blind: {}", sb.by_symbol_index());
/// println!("big blind:   {}", bb);
///
/// println!();
/// println!("flop : {}", shuffled.draw(3).unwrap());
/// println!("turn : {}", shuffled.draw(1).unwrap());
/// println!("river: {}", shuffled.draw(1).unwrap());
///
/// ```
///

pub struct Pack {
    cards: Pile,
}

impl Pack {
    fn new(cards: Pile) -> Pack {
        Pack { cards }
    }

    /// Returns a reference to the cards in the Pack.
    pub fn cards(&self) -> &Pile {
        &self.cards
    }

    ///
    pub fn french_deck() -> Pack {
        Pack::new(Pile::french_deck())
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
