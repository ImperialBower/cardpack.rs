use crate::funky::types::mpip::MPip;
use crate::prelude::{CardError, Pip};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum CardType {
    #[default]
    Basic,
    Joker,
    Planet,
    Spectral,
    Tarot,
    Voucher,
}

impl Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardType::Joker => 'J',
            CardType::Planet => 'P',
            CardType::Spectral => 'S',
            CardType::Tarot => 'T',
            CardType::Voucher => 'V',
            CardType::Basic => '_',
        };
        write!(f, "{s}")
    }
}

impl From<char> for CardType {
    fn from(c: char) -> Self {
        match c {
            'J' => CardType::Joker,
            'P' => CardType::Planet,
            'S' => CardType::Spectral,
            'T' => CardType::Tarot,
            'V' => CardType::Voucher,
            _ => CardType::Basic,
        }
    }
}

impl FromStr for CardType {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(CardError::InvalidIndex(s.to_string()));
        }
        Ok(s.chars().next().unwrap().into())
    }
}

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize, Deserialize,
)]
pub struct BuffoonCard {
    pub suit: Pip,
    pub rank: Pip,
    pub card_type: CardType,
    pub enhancement: MPip,
}
