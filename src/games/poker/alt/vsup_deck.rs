use crate::games::poker::alt::vsup_card::{VSupCard, VSupSuit, VSupValue};

pub struct VSupDeck {
    count_dealt: usize,
    // TODO: consider turning this into a Vec<Card>, for iterator
    // goodness. deck.next() producing Option<Card>?
    cards: [u8; 52],
}

pub enum DeckError {
    NotEnoughCards,
}

/// translates a value between 0 and 51 to a Card. Used internally.
fn create_card_for_value(value: u8) -> VSupCard {
    let suit = match value / 13 {
        0 => VSupSuit::Spades,
        1 => VSupSuit::Hearts,
        2 => VSupSuit::Diamonds,
        3 => VSupSuit::Clubs,
        _ => panic!("Unexpected suit conversion number"),
    };

    let value = match value % 13 {
        0 => VSupValue::Two,
        1 => VSupValue::Three,
        2 => VSupValue::Four,
        3 => VSupValue::Five,
        4 => VSupValue::Six,
        5 => VSupValue::Seven,
        6 => VSupValue::Eight,
        7 => VSupValue::Nine,
        8 => VSupValue::Ten,
        9 => VSupValue::Jack,
        10 => VSupValue::Queen,
        11 => VSupValue::King,
        12 => VSupValue::Ace,
        _ => panic!("Unexpected value conversion number"),
    };

    VSupCard::new(value, suit)
}

/// A deck can be dealt from and shuffled.
impl VSupDeck {
    //TODO: a deck containing multiple sets of cards? When 52*3 is needed.

    /// Returns a deck where all cards are sorted by Suit, then by Value.
    pub fn new_unshuffled() -> VSupDeck {
        let mut d = VSupDeck {
            count_dealt: 0,
            cards: [0; 52],
        };

        let mut value = 0;
        #[allow(clippy::explicit_counter_loop)]
        for x in d.cards.iter_mut() {
            *x = value;
            value += 1;
        }
        d
    }

    /// An attempt to get a card from the deck. There might not be enough.
    pub fn draw(&mut self) -> Result<VSupCard, DeckError> {
        if self.count_dealt + 1 > 52 {
            Err(DeckError::NotEnoughCards)
        } else {
            let value = self.cards[self.count_dealt];
            self.count_dealt += 1;

            let card = create_card_for_value(value);
            Ok(card)
        }
    }

    /// An attempt to get n cards from the deck wrapped in a Vec. There might not be enough.
    pub fn draw_n(&mut self, n: usize) -> Result<Vec<VSupCard>, DeckError> {
        if self.count_dealt + n > 52 {
            Err(DeckError::NotEnoughCards)
        } else {
            let mut cards = Vec::new();

            for _ in 0..n {
                cards.push(self.draw().ok().unwrap());
            }

            Ok(cards)
        }
    }
}
