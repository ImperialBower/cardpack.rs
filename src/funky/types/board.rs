use crate::preludes::funky::{BuffoonPile, PokerHands};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
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

impl BuffoonBoard {
    pub fn new(deck: BuffoonPile) -> Self {
        Self {
            hands_to_play: 0,
            discards: 0,
            deck,
            in_hand: BuffoonPile::default(),
            played: BuffoonPile::default(),
            consumables: BuffoonPile::default(),
            jokers: BuffoonPile::default(),
            poker_hands: PokerHands::default(),
        }
    }
}
