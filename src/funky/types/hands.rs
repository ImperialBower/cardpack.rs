#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

pub struct PokerHands {
    pub hands: Vec<PokerHand>,
}