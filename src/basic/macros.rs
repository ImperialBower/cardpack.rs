#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! french_cards {
    ($card_str:expr) => {
        Deck::<French>::forgiving_from_str($card_str)
    };
}

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! cards {
    ($card_str:expr) => {
        Deck::<Standard52>::forgiving_from_str($card_str)
    };
}

#[macro_export]
macro_rules! card {
    (AS) => {
        Card::<Standard52>::new(FrenchBasicCard::ACE_SPADES)
    };
    (KS) => {
        Card::<Standard52>::new(FrenchBasicCard::KING_SPADES)
    };
    (QS) => {
        Card::<Standard52>::new(FrenchBasicCard::QUEEN_SPADES)
    };
    (JS) => {
        Card::<Standard52>::new(FrenchBasicCard::JACK_SPADES)
    };
    (TS) => {
        Card::<Standard52>::new(FrenchBasicCard::TEN_SPADES)
    };
    (9S) => {
        Card::<Standard52>::new(FrenchBasicCard::NINE_SPADES)
    };
    (8S) => {
        Card::<Standard52>::new(FrenchBasicCard::EIGHT_SPADES)
    };
    (7S) => {
        Card::<Standard52>::new(FrenchBasicCard::SEVEN_SPADES)
    };
    (6S) => {
        Card::<Standard52>::new(FrenchBasicCard::SIX_SPADES)
    };
    (5S) => {
        Card::<Standard52>::new(FrenchBasicCard::FIVE_SPADES)
    };
    (4S) => {
        Card::<Standard52>::new(FrenchBasicCard::FOUR_SPADES)
    };
    (3S) => {
        Card::<Standard52>::new(FrenchBasicCard::TREY_SPADES)
    };
    (2S) => {
        Card::<Standard52>::new(FrenchBasicCard::DEUCE_SPADES)
    };
    (AH) => {
        Card::<Standard52>::new(FrenchBasicCard::ACE_HEARTS)
    };
    (KH) => {
        Card::<Standard52>::new(FrenchBasicCard::KING_HEARTS)
    };
    (QH) => {
        Card::<Standard52>::new(FrenchBasicCard::QUEEN_HEARTS)
    };
    (JH) => {
        Card::<Standard52>::new(FrenchBasicCard::JACK_HEARTS)
    };
    (TH) => {
        Card::<Standard52>::new(FrenchBasicCard::TEN_HEARTS)
    };
    (9H) => {
        Card::<Standard52>::new(FrenchBasicCard::NINE_HEARTS)
    };
    (8H) => {
        Card::<Standard52>::new(FrenchBasicCard::EIGHT_HEARTS)
    };
    (7H) => {
        Card::<Standard52>::new(FrenchBasicCard::SEVEN_HEARTS)
    };
    (6H) => {
        Card::<Standard52>::new(FrenchBasicCard::SIX_HEARTS)
    };
    (5H) => {
        Card::<Standard52>::new(FrenchBasicCard::FIVE_HEARTS)
    };
    (4H) => {
        Card::<Standard52>::new(FrenchBasicCard::FOUR_HEARTS)
    };
    (3H) => {
        Card::<Standard52>::new(FrenchBasicCard::TREY_HEARTS)
    };
    (2H) => {
        Card::<Standard52>::new(FrenchBasicCard::DEUCE_HEARTS)
    };
    (AD) => {
        Card::<Standard52>::new(FrenchBasicCard::ACE_DIAMONDS)
    };
    (KD) => {
        Card::<Standard52>::new(FrenchBasicCard::KING_DIAMONDS)
    };
    (QD) => {
        Card::<Standard52>::new(FrenchBasicCard::QUEEN_DIAMONDS)
    };
    (JD) => {
        Card::<Standard52>::new(FrenchBasicCard::JACK_DIAMONDS)
    };
    (TD) => {
        Card::<Standard52>::new(FrenchBasicCard::TEN_DIAMONDS)
    };
    (9D) => {
        Card::<Standard52>::new(FrenchBasicCard::NINE_DIAMONDS)
    };
    (8D) => {
        Card::<Standard52>::new(FrenchBasicCard::EIGHT_DIAMONDS)
    };
    (7D) => {
        Card::<Standard52>::new(FrenchBasicCard::SEVEN_DIAMONDS)
    };
    (6D) => {
        Card::<Standard52>::new(FrenchBasicCard::SIX_DIAMONDS)
    };
    (5D) => {
        Card::<Standard52>::new(FrenchBasicCard::FIVE_DIAMONDS)
    };
    (4D) => {
        Card::<Standard52>::new(FrenchBasicCard::FOUR_DIAMONDS)
    };
    (3D) => {
        Card::<Standard52>::new(FrenchBasicCard::TREY_DIAMONDS)
    };
    (2D) => {
        Card::<Standard52>::new(FrenchBasicCard::DEUCE_DIAMONDS)
    };
    (AC) => {
        Card::<Standard52>::new(FrenchBasicCard::ACE_CLUBS)
    };
    (KC) => {
        Card::<Standard52>::new(FrenchBasicCard::KING_CLUBS)
    };
    (QC) => {
        Card::<Standard52>::new(FrenchBasicCard::QUEEN_CLUBS)
    };
    (JC) => {
        Card::<Standard52>::new(FrenchBasicCard::JACK_CLUBS)
    };
    (TC) => {
        Card::<Standard52>::new(FrenchBasicCard::TEN_CLUBS)
    };
    (9C) => {
        Card::<Standard52>::new(FrenchBasicCard::NINE_CLUBS)
    };
    (8C) => {
        Card::<Standard52>::new(FrenchBasicCard::EIGHT_CLUBS)
    };
    (7C) => {
        Card::<Standard52>::new(FrenchBasicCard::SEVEN_CLUBS)
    };
    (6C) => {
        Card::<Standard52>::new(FrenchBasicCard::SIX_CLUBS)
    };
    (5C) => {
        Card::<Standard52>::new(FrenchBasicCard::FIVE_CLUBS)
    };
    (4C) => {
        Card::<Standard52>::new(FrenchBasicCard::FOUR_CLUBS)
    };
    (3C) => {
        Card::<Standard52>::new(FrenchBasicCard::TREY_CLUBS)
    };
    (2C) => {
        Card::<Standard52>::new(FrenchBasicCard::DEUCE_CLUBS)
    };
    (__) => {
        Card::<Standard52>::default()
    };
    ($card_str:expr) => {
        Card::<Standard52>::from_str($card_str).unwrap_or_else(|_| Card::<Standard52>::default())
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__macros_tests {
    use crate::prelude::*;

    #[test]
    fn card__from_str() {
        assert_eq!(card!("A♠"), card!(AS));
        assert_eq!(card!("K♠"), card!(KS));
        assert_eq!(card!("Q♠"), card!(QS));
        assert_eq!(card!("J♠"), card!(JS));
        assert_eq!(card!("T♠"), card!(TS));
        assert_eq!(card!("9♠"), card!(9S));
        assert_eq!(card!("8♠"), card!(8S));
        assert_eq!(card!("7♠"), card!(7S));
        assert_eq!(card!("6♠"), card!(6S));
        assert_eq!(card!("5♠"), card!(5S));
        assert_eq!(card!("4♠"), card!(4S));
        assert_eq!(card!("3♠"), card!(3S));
        assert_eq!(card!("2♠"), card!(2S));
        assert_eq!(card!("A♥"), card!(AH));
        assert_eq!(card!("K♥"), card!(KH));
        assert_eq!(card!("Q♥"), card!(QH));
        assert_eq!(card!("J♥"), card!(JH));
        assert_eq!(card!("T♥"), card!(TH));
        assert_eq!(card!("9♥"), card!(9H));
        assert_eq!(card!("8♥"), card!(8H));
        assert_eq!(card!("7♥"), card!(7H));
        assert_eq!(card!("6♥"), card!(6H));
        assert_eq!(card!("5♥"), card!(5H));
        assert_eq!(card!("4♥"), card!(4H));
        assert_eq!(card!("3♥"), card!(3H));
        assert_eq!(card!("2♥"), card!(2H));
        assert_eq!(card!("A♦"), card!(AD));
        assert_eq!(card!("K♦"), card!(KD));
        assert_eq!(card!("Q♦"), card!(QD));
        assert_eq!(card!("J♦"), card!(JD));
        assert_eq!(card!("T♦"), card!(TD));
        assert_eq!(card!("9♦"), card!(9D));
        assert_eq!(card!("8♦"), card!(8D));
        assert_eq!(card!("7♦"), card!(7D));
        assert_eq!(card!("6♦"), card!(6D));
        assert_eq!(card!("5♦"), card!(5D));
        assert_eq!(card!("4♦"), card!(4D));
        assert_eq!(card!("3♦"), card!(3D));
        assert_eq!(card!("2♦"), card!(2D));
        assert_eq!(card!("A♣"), card!(AC));
        assert_eq!(card!("K♣"), card!(KC));
        assert_eq!(card!("Q♣"), card!(QC));
        assert_eq!(card!("J♣"), card!(JC));
        assert_eq!(card!("T♣"), card!(TC));
        assert_eq!(card!("9♣"), card!(9C));
        assert_eq!(card!("8♣"), card!(8C));
        assert_eq!(card!("7♣"), card!(7C));
        assert_eq!(card!("6♣"), card!(6C));
        assert_eq!(card!("5♣"), card!(5C));
        assert_eq!(card!("4♣"), card!(4C));
        assert_eq!(card!("3♣"), card!(3C));
        assert_eq!(card!("2♣"), card!(2C));
        assert_eq!(card!("__"), card!(__));
    }
}
