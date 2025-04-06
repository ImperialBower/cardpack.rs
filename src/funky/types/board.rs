use crate::preludes::funky::{BuffoonPile, PokerHands};

pub struct BuffoonBoard {
    pub hands_to_play: usize,
    pub discards: usize,
    pub deck: BuffoonPile,
    pub in_hand: BuffoonPile,
    pub played: BuffoonPile,
    pub consumables: BuffoonPile,
    pub jokers: BuffoonPile,
    pub poker_hands: PokerHands,
}
