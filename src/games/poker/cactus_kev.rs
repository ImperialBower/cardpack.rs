use crate::Card;
use bitvec::prelude::*;
use std::fmt::{Display, Formatter};
use wyz::fmt::FmtForward;

/// A `CactusKev` Card representation is made up of four bites.
pub type CactusKev = BitArray<Msb0, [u8; 4]>;

#[allow(clippy::module_name_repetitions)]
pub struct CactusKevCard {
    pub bites: CactusKev,
}

impl CactusKevCard {
    #[must_use]
    pub fn new(bites: CactusKev) -> Self {
        Self { bites }
    }

    #[must_use]
    #[allow(clippy::needless_borrow)]
    pub fn new_from_card(card: &Card) -> CactusKevCard {
        let mut cactus: CactusKevCard = CactusKevCard::blank();
        cactus.set_rank(&card);
        cactus.set_rank_flag(&card);
        cactus.set_rank_prime(&card);
        cactus.set_suit(&card);
        cactus
    }

    #[must_use]
    pub fn blank() -> CactusKevCard {
        let bites: CactusKev = BitArray::zeroed();
        CactusKevCard::new(bites)
    }

    /// Takes the `BitArray` representation of the Card and returns a `String`
    /// separating each eight bits by a 0. For instance, `00001000000000000100101100100101`
    /// becomes `00001000 00000000 01001011 00100101`.
    #[must_use]
    pub fn display(&self, split: bool) -> String {
        let mut word_string = String::with_capacity(35);
        let start_bit: usize = 0;
        let bits = start_bit..start_bit + 32;
        for (bit, idx) in self.bites.as_bitslice().iter().by_val().zip(bits) {
            word_string.push_str(if bit { "1" } else { "0" });
            if split && idx % 8 == 7 && idx % 32 != 31 {
                word_string.push(' ');
            }
        }
        word_string
    }

    /// Returns a `BitSlice` of the `Suit` section of the `CactusKev` `BitArray`.
    #[must_use]
    pub fn get_suit(&self) -> &BitSlice<Msb0, u8> {
        &self.bites[16..20]
    }

    fn set_rank_prime(&mut self, card: &Card) {
        self.bites[26..32].store_be(card.rank.prime);
    }

    fn set_rank_flag(&mut self, card: &Card) {
        match card.rank.weight {
            12 => self.bites[3..4].store(1u8),  // Ace
            11 => self.bites[4..5].store(1u8),  // King
            10 => self.bites[5..6].store(1u8),  // Queen
            9 => self.bites[6..7].store(1u8),   // Jack
            8 => self.bites[7..8].store(1u8),   // Ten
            7 => self.bites[8..9].store(1u8),   // Nine
            6 => self.bites[9..10].store(1u8),  // Eight
            5 => self.bites[10..11].store(1u8), // Seven
            4 => self.bites[11..12].store(1u8), // Six
            3 => self.bites[12..13].store(1u8), // Five
            2 => self.bites[13..14].store(1u8), // Four
            1 => self.bites[14..15].store(1u8), // Three
            0 => self.bites[15..16].store(1u8), // Two
            _ => (),
        }
    }

    fn set_rank(&mut self, card: &Card) {
        self.bites[20..24].store_be(card.rank.weight);
    }

    fn set_suit(&mut self, card: &Card) {
        match card.suit.weight {
            4 => self.bites[19..20].store(1u8), // Spades
            3 => self.bites[18..19].store(1u8), // Hearts
            2 => self.bites[17..18].store(1u8), // Diamonds
            1 => self.bites[16..17].store(1u8), // Clubs
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

        out.entry(&(self.display(true)).fmt_display());
        out.entry(&(&mark_string).fmt_display());
        out.finish()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_tests {
    use super::*;
    use crate::Standard52;

    #[test]
    fn len() {
        let card: CactusKev = BitArray::zeroed();
        assert_eq!(card.len(), 32);
        assert_eq!(CactusKevCard::blank().bites.len(), 32);
    }

    #[test]
    fn new_from_card() {
        let card = Standard52::card_from_index("K♦");
        let cactusKevCard: CactusKevCard = CactusKevCard::new_from_card(&card);

        assert_eq!(
            "00001000 00000000 01001011 00100101",
            cactusKevCard.display(true)
        );
    }

    /// This test goes through all 52 cards in a Standard52 deck and compares the
    /// `CactusKevCard` version of the bite signature with the `Card`'s version.
    #[test]
    fn new_from_card__complete() {
        let standard52 = Standard52::default();
        for card in standard52.deck {
            let cactusKevCard: CactusKevCard = CactusKevCard::new_from_card(&card);
            let s = format!("{:032b}", card).to_string();
            assert_eq!(s, cactusKevCard.display(false));
        }
    }

    #[test]
    fn set_rank_prime() {
        let mut cactus: CactusKevCard = CactusKevCard::blank();
        let card = Standard52::card_from_index("K♦");

        cactus.set_rank_prime(&card);
        assert_eq!("00000000 00000000 00000000 00100101", cactus.display(true));

        cactus.set_rank(&card);
        cactus.set_rank_flag(&card);
        assert_eq!("00001000 00000000 00001011 00100101", cactus.display(true));

        cactus.set_suit(&card);
        assert_eq!("00001000 00000000 01001011 00100101", cactus.display(true));

        println!("{}", cactus.display(true));
        println!("{:032b}", card.binary_signature());
        println!("{:#}", cactus);
    }

    #[test]
    fn get_suit() {
        let card = Standard52::card_from_index("KS");
        let cactusKevCard: CactusKevCard = CactusKevCard::new_from_card(&card);
        assert_eq!("[0001]", format!("{:04b}", cactusKevCard.get_suit()));

        let card = Standard52::card_from_index("KH");
        let cactusKevCard: CactusKevCard = CactusKevCard::new_from_card(&card);
        assert_eq!("[0010]", format!("{:04b}", cactusKevCard.get_suit()));

        let card = Standard52::card_from_index("K♦");
        let cactusKevCard: CactusKevCard = CactusKevCard::new_from_card(&card);
        assert_eq!("[0100]", format!("{:04b}", cactusKevCard.get_suit()));

        let card = Standard52::card_from_index("KC");
        let cactusKevCard: CactusKevCard = CactusKevCard::new_from_card(&card);
        assert_eq!("[1000]", format!("{:04b}", cactusKevCard.get_suit()));
    }
}
