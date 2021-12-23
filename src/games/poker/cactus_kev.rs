use crate::Card;
use bitvec::prelude::*;
use std::fmt::{Display, Formatter};
use wyz::fmt::FmtForward;

/// A `CactusKev` Card representation is made up of four bites.
pub type CactusKev = BitArray<Msb0, [u8; 4]>;

#[allow(clippy::module_name_repetitions)]
pub struct CactusKevCard {
    pub card: CactusKev,
}

impl CactusKevCard {
    #[must_use]
    pub fn new(card: CactusKev) -> Self {
        Self { card }
    }

    #[must_use]
    pub fn blank() -> CactusKevCard {
        let card: CactusKev = BitArray::zeroed();
        CactusKevCard::new(card)
    }

    #[must_use]
    pub fn dump(&self) -> String {
        let mut word_string = String::with_capacity(35);
        for (idx, word) in self.card.as_bitslice().chunks(32).enumerate() {
            let start_bit = idx * 32;
            let bits = start_bit..start_bit + 32;
            for (bit, idx) in word.iter().by_val().zip(bits) {
                word_string.push_str(if bit { "1" } else { "0" });
                if idx % 8 == 7 && idx % 32 != 31 {
                    word_string.push(' ');
                }
            }
        }
        word_string
    }

    pub fn set_rank_prime(&mut self, card: &Card) {
        self.card[26..32].store_be(card.rank.prime);
    }

    pub fn set_rank_flag(&mut self, card: &Card) {
        match card.rank.weight {
            12 => self.card[..4].store(1u8), // Ace
            11 => self.card[..5].store(1u8), // King
            10 => self.card[..6].store(1u8), // Queen
            9 => self.card[..7].store(1u8),  // Jack
            8 => self.card[..8].store(1u8),  // Ten
            7 => self.card[..9].store(1u8),  // Nine
            6 => self.card[..10].store(1u8), // Eight
            5 => self.card[..11].store(1u8), // Seven
            4 => self.card[..12].store(1u8), // Six
            3 => self.card[..13].store(1u8), // Five
            2 => self.card[..14].store(1u8), // Four
            1 => self.card[..15].store(1u8), // Three
            0 => self.card[..16].store(1u8), // Two
            _ => (),
        }
    }

    pub fn set_rank(&mut self, card: &Card) {
        self.card[20..24].store_be(card.rank.weight);
    }

    pub fn set_suit(&mut self, card: &Card) {
        match card.suit.weight {
            4 => self.card[..20].store(1u8),   // Spades
            3 => self.card[..19].store(1u8),   // Hearts
            2 => self.card[17..18].store(1u8), // Diamonds
            1 => self.card[16..17].store(1u8), // Clubs
            _ => (),
        }
    }
}

/// [Module ``std::fmt``](https://doc.rust-lang.org/std/fmt/)
impl Display for CactusKevCard {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        let mut mark_string = String::with_capacity(35);
        mark_string.push_str("xxxAKQJT 98765432 CDHSrrrr xxpppppp");

        out.entry(&(self.dump()).fmt_display());
        out.entry(&(&mark_string).fmt_display());
        out.finish()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_tests {
    use super::*;
    use crate::{Card, DIAMONDS, KING};

    #[test]
    fn len() {
        let card: CactusKev = BitArray::zeroed();
        assert_eq!(card.len(), 32);
    }

    //  00001000 00000000 01000000 00100101
    //  00001000 00000000 01001011 00100101
    #[test]
    fn set_rank_prime() {
        let mut cactus: CactusKevCard = CactusKevCard::blank();
        let card = Card::from_index_strings(KING, DIAMONDS);

        cactus.set_rank_prime(&card);
        assert_eq!("00000000 00000000 00000000 00100101", cactus.dump());

        cactus.set_rank(&card);
        cactus.set_rank_flag(&card);
        assert_eq!("00001000 00000000 00001011 00100101", cactus.dump());

        cactus.set_suit(&card);
        assert_eq!("00001000 00000000 01001011 00100101", cactus.dump());

        println!("{}", cactus.dump());
        println!("{:032b}", card.binary_signature());
        println!("{:#}", cactus);
    }
}
