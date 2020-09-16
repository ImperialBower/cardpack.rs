use crate::cards::pile::Pile;

pub struct Pack {
    cards: Pile,
}

/// A Pack is an immutable pile of cards. Packs available are for a traditional
/// 52 card French Deck, pinochle, spades, skat and tarot.
///
/// # Usage:
/// ```
/// let pack = cardpack::Pack::french_deck();
/// let shuffled = pack.cards().shuffle();
/// ```
///
impl Pack {
    fn new(cards: Pile) -> Pack {
        Pack {
            cards,
        }
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