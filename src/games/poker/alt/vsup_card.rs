use crate::games::poker::alt::lookups;
use std::fmt;
use crate::cards::card::CactusKevCard;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum VSupSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl VSupSuit {
    fn short_string(&self) -> &'static str {
        match *self {
            VSupSuit::Spades => "s",
            VSupSuit::Hearts => "h",
            VSupSuit::Diamonds => "d",
            VSupSuit::Clubs => "c",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum VSupValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    // no jokers
}

impl VSupValue {
    fn short_string(&self) -> &'static str {
        match *self {
            VSupValue::Two => "2",
            VSupValue::Three => "3",
            VSupValue::Four => "4",
            VSupValue::Five => "5",
            VSupValue::Six => "6",
            VSupValue::Seven => "7",
            VSupValue::Eight => "8",
            VSupValue::Nine => "9",
            VSupValue::Ten => "T",
            VSupValue::Jack => "J",
            VSupValue::Queen => "Q",
            VSupValue::King => "K",
            VSupValue::Ace => "A",
        }
    }
}

//TODO: debug still relevant? It was used to print a vec of cards.
/// An unnamed tuple with Value and Suit.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct VSupCard {
    pub value: VSupValue,
    pub suit: VSupSuit,
}

impl VSupCard {
    pub fn new(value: VSupValue, suit: VSupSuit) -> VSupCard {
        VSupCard { value, suit }
    }

    /// translates a value between 0 and 51 to a Card. Used internally.
    pub fn create_card_for_value(value: u8) -> VSupCard {
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

    /// Converts a card to a `CactusKevCard`, which is a convenient binary representation:
    ///
    ///  +--------+--------+--------+--------+
    ///  |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
    ///  +--------+--------+--------+--------+
    ///
    ///  p = prime number of value (deuce=2,trey=3,four=5,five=7,...,ace=41)
    ///  r = value of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
    ///  cdhs = suit of card
    ///  b = bit turned on depending on value of card
    #[must_use]
    pub fn card_to_deck_number(&self) -> CactusKevCard {
        let value: u32 = match self.value {
            VSupValue::Two => 0,
            VSupValue::Three => 1,
            VSupValue::Four => 2,
            VSupValue::Five => 3,
            VSupValue::Six => 4,
            VSupValue::Seven => 5,
            VSupValue::Eight => 6,
            VSupValue::Nine => 7,
            VSupValue::Ten => 8,
            VSupValue::Jack => 9,
            VSupValue::Queen => 10,
            VSupValue::King => 11,
            VSupValue::Ace => 12,
        };
        #[allow(clippy::cast_lossless)]
        let prime: u32 = lookups::PRIMES[value as usize] as u32;
        let suit: u32 = match self.suit {
            VSupSuit::Spades => 0x1000,
            VSupSuit::Hearts => 0x2000,
            VSupSuit::Diamonds => 0x4000,
            VSupSuit::Clubs => 0x8000,
        };
        let bits: u32 = 1 << (16 + value);

        prime | value << 8 | suit | bits
    }
}

// so cards can be printed using fmt method
impl fmt::Display for VSupCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.value.short_string(),
            self.suit.short_string()
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod vsup_card_tests {
    use super::*;

    #[test]
    fn card_to_deck_number() {
        let vsup_card = VSupCard::new(VSupValue::Ace, VSupSuit::Spades);
        let expected = 268442665;

        assert_eq!(vsup_card.card_to_deck_number(), expected);
    }

    #[test]
    fn create_card_for_value() {
        let expected = VSupCard::new(VSupValue::Ace, VSupSuit::Spades);

        let actual = VSupCard::create_card_for_value(12);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scratch() {
        let vsup_card = VSupCard::new(VSupValue::Ace, VSupSuit::Spades);

        println!("{}", vsup_card);
        println!("{}", vsup_card.card_to_deck_number());
        println!("{:032b}", vsup_card.card_to_deck_number());
    }
}
