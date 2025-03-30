use crate::preludes::funky::{BuffoonCard, BuffoonPile};

pub struct Board {
    pub deck: BuffoonPile,
    pub in_hand: BuffoonPile,
    pub played: BuffoonPile,
    pub consummables: BuffoonCard,
    pub jokers: BuffoonCard,
}
