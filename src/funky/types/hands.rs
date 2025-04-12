use crate::preludes::funky::{BCardType, BuffoonCard, MPip};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize,
)]
pub enum HandType {
    #[default]
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct PokerHand {
    pub hand_type: HandType,
    pub level: usize,
    pub chips: usize,
    pub mult: usize,
    pub times_played: usize,
}

impl PokerHand {
    #[must_use]
    pub fn new(hand_type: HandType, chips: usize, mult: usize) -> Self {
        Self {
            hand_type,
            level: 1,
            chips,
            mult,
            times_played: 0,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PokerHands {
    pub hands: HashMap<HandType, PokerHand>,
}

impl PokerHands {
    #[must_use]
    pub fn new() -> Self {
        let mut hands = HashMap::new();
        hands.insert(HandType::HighCard, PokerHand::new(HandType::HighCard, 5, 1));
        hands.insert(HandType::Pair, PokerHand::new(HandType::Pair, 10, 2));
        hands.insert(HandType::TwoPair, PokerHand::new(HandType::TwoPair, 20, 2));
        hands.insert(
            HandType::ThreeOfAKind,
            PokerHand::new(HandType::ThreeOfAKind, 30, 3),
        );
        hands.insert(
            HandType::Straight,
            PokerHand::new(HandType::Straight, 30, 4),
        );
        hands.insert(HandType::Flush, PokerHand::new(HandType::Flush, 35, 4));
        hands.insert(
            HandType::FullHouse,
            PokerHand::new(HandType::FullHouse, 40, 4),
        );
        hands.insert(
            HandType::FourOfAKind,
            PokerHand::new(HandType::FourOfAKind, 50, 7),
        );
        hands.insert(
            HandType::StraightFlush,
            PokerHand::new(HandType::StraightFlush, 100, 8),
        );
        hands.insert(
            HandType::FiveOfAKind,
            PokerHand::new(HandType::FiveOfAKind, 120, 12),
        );
        hands.insert(
            HandType::FlushHouse,
            PokerHand::new(HandType::FlushHouse, 140, 14),
        );
        hands.insert(
            HandType::FlushFive,
            PokerHand::new(HandType::FlushHouse, 160, 16),
        );
        Self { hands }
    }

    #[must_use]
    pub fn get(&self, hand_type: &HandType) -> Option<&PokerHand> {
        self.hands.get(hand_type)
    }

    pub fn get_mut(&mut self, hand_type: &HandType) -> Option<&mut PokerHand> {
        self.hands.get_mut(hand_type)
    }

    pub fn increment(&mut self, planet_card: BuffoonCard) {
        if planet_card.card_type == BCardType::Planet {
            if let MPip::ChipsMultPlusOnHand(chips, mult, hand_type) = planet_card.enhancement {
                if let Some(poker_hand) = self.get_mut(&hand_type) {
                    poker_hand.chips += chips;
                    poker_hand.mult += mult;
                    poker_hand.level += 1;
                }
            }
        }
    }

    pub fn play_hand(&mut self, hand_type: &HandType) {
        if let Some(poker_hand) = self.get_mut(hand_type) {
            poker_hand.times_played += 1;
        }
    }
}

impl Default for PokerHands {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__hands_tests {
    use super::*;
    use crate::funky::decks::planet;
    use crate::preludes::funky::*;
    use rstest::rstest;

    #[test]
    fn increment() {
        let mut hands = PokerHands::default();
        let expected = PokerHand {
            hand_type: HandType::HighCard,
            level: 2,
            chips: 15,
            mult: 2,
            times_played: 0,
        };

        hands.increment(planet::card::PLUTO);

        assert_eq!(hands.get(&HandType::HighCard).unwrap(), &expected);
    }

    #[test]
    fn play_hand() {
        let mut hands = PokerHands::default();
        let expected = PokerHand {
            hand_type: HandType::HighCard,
            level: 1,
            chips: 5,
            mult: 1,
            times_played: 1,
        };

        hands.play_hand(&HandType::HighCard);

        assert_eq!(hands.get(&HandType::HighCard).unwrap(), &expected);
    }
}
