use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
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
    FlushHouse,
    FiveOfAKind,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
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
        Self { hands }
    }

    #[must_use]
    pub fn get(&self, hand_type: &HandType) -> Option<&PokerHand> {
        self.hands.get(hand_type)
    }

    pub fn get_mut(&mut self, hand_type: &HandType) -> Option<&mut PokerHand> {
        self.hands.get_mut(hand_type)
    }

    pub fn insert(&mut self, hand_type: HandType, poker_hand: PokerHand) {
        self.hands.insert(hand_type, poker_hand);
    }
}

impl Default for PokerHands {
    fn default() -> Self {
        Self::new()
    }
}
