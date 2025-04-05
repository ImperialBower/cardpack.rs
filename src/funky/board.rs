use crate::preludes::funky::BuffoonPile;

pub struct Board {
    pub hands: usize,
    pub discards: usize,
    pub deck: BuffoonPile,
    pub in_hand: BuffoonPile,
    pub played: BuffoonPile,
    pub consumables: BuffoonPile,
    pub jokers: BuffoonPile,
}
