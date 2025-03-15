use crate::funky::types::mpip::{MPip, MPipType};
use crate::prelude::{CardError, Pip};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;
// region BCardType

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum BCardType {
    #[default]
    Basic,
    Joker,
    Planet,
    Spectral,
    Tarot,
    Voucher,
}

impl Display for BCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BCardType::Joker => 'J',
            BCardType::Planet => 'P',
            BCardType::Spectral => 'S',
            BCardType::Tarot => 'T',
            BCardType::Voucher => 'V',
            BCardType::Basic => '_',
        };
        write!(f, "{s}")
    }
}

impl From<char> for BCardType {
    fn from(c: char) -> Self {
        match c {
            'J' => BCardType::Joker,
            'P' => BCardType::Planet,
            'S' => BCardType::Spectral,
            'T' => BCardType::Tarot,
            'V' => BCardType::Voucher,
            _ => BCardType::Basic,
        }
    }
}

impl FromStr for BCardType {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(CardError::InvalidIndex(s.to_string()));
        }
        Ok(s.chars().next().unwrap().into())
    }
}

// endregion BCardType

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BuffoonCard {
    pub suit: Pip,
    pub rank: Pip,
    pub card_type: BCardType,
    pub enhancement: MPip,
}

impl BuffoonCard {
    #[must_use]
    pub fn add_chips(&self, chips: usize) -> Self {
        let mut current_chips = 0;

        if let MPipType::Chips(c) = self.enhancement.pip_type {
            current_chips = c;
        }
        let new_chips = current_chips + chips;

        BuffoonCard {
            suit: self.suit,
            rank: self.rank,
            card_type: self.card_type,
            enhancement: MPip::new_chips(new_chips),
        }
    }

    #[must_use]
    pub fn get_chips(&self) -> usize {
        let mut chips = self.rank.value as usize;
        if let MPipType::Chips(c) = self.enhancement.pip_type {
            chips += c;
        };
        chips
    }
}

/// Inverts the order so that the highest card comes first.
impl Ord for BuffoonCard {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .suit
            .cmp(&self.suit)
            .then_with(|| other.rank.cmp(&self.rank))
    }
}

impl PartialOrd for BuffoonCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__buffoon_card_tests {
    use super::*;
    use crate::funky::decks::{basic, tarot};
    use crate::funky::types::mpip::MPipType;

    #[test]
    fn get_chips() {
        let mut ks = basic::card::KING_SPADES.add_chips(11).add_chips(15);

        assert_eq!(ks.get_chips(), 36);
        assert_eq!(tarot::card::DEATH.get_chips(), 10);
    }
}
