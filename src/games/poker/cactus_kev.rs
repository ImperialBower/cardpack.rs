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
        let mut cactus: CactusKevCard = CactusKevCard::default();
        cactus.set_rank(&card);
        cactus.set_rank_flag(&card);
        cactus.set_rank_prime(&card);
        cactus.set_suit(&card);
        cactus
    }

    #[must_use]
    pub fn new_from_u64(integer: u64) -> CactusKevCard {
        let mut cactus: CactusKevCard = CactusKevCard::default();
        cactus.bites[..32].store_be(integer);
        cactus
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

    // #[must_use]
    // pub fn get_int(&self) -> usize {
    //     self.bites.
    // }

    fn set_rank_prime(&mut self, card: &Card) {
        self.bites[26..32].store_be(card.rank.prime);
    }

    fn set_rank_flag(&mut self, card: &Card) {
        match card.rank.weight {
            12 => self.bites.set(3, true), // Ace
            11 => self.bites.set(4, true), // King
            10 => self.bites.set(5, true), // Queen
            9 => self.bites.set(6, true),  // Jack
            8 => self.bites.set(7, true),  // Ten
            7 => self.bites.set(8, true),  // Nine
            6 => self.bites.set(9, true),  // Eight
            5 => self.bites.set(10, true), // Seven
            4 => self.bites.set(11, true), // Six
            3 => self.bites.set(12, true), // Five
            2 => self.bites.set(13, true), // Four
            1 => self.bites.set(14, true), // Three
            0 => self.bites.set(15, true), // Two
            _ => (),
        }
    }

    fn set_rank(&mut self, card: &Card) {
        self.bites[20..24].store_be(card.rank.weight);
    }

    fn set_suit(&mut self, card: &Card) {
        match card.suit.weight {
            4 => self.bites.set(19, true), // Spades
            3 => self.bites.set(18, true), // Hearts
            2 => self.bites.set(17, true), // Diamonds
            1 => self.bites.set(16, true), // Clubs
            _ => (),
        }
    }
}

impl Default for CactusKevCard {
    fn default() -> CactusKevCard {
        CactusKevCard::new(BitArray::zeroed())
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
        assert_eq!(CactusKevCard::default().bites.len(), 32);
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
    fn set_rank() {
        let mut cactus: CactusKevCard = CactusKevCard::default();
        let card = Standard52::card_from_index("K♦");

        cactus.set_rank(&card);
        assert_eq!("00000000 00000000 00001011 00000000", cactus.display(true));
    }

    #[test]
    fn set_rank_flag() {
        let mut cactus: CactusKevCard = CactusKevCard::default();
        let card = Standard52::card_from_index("K♦");

        cactus.set_rank_flag(&card);
        assert_eq!("00001000 00000000 00000000 00000000", cactus.display(true));
    }

    #[test]
    fn set() {
        let mut cactus: CactusKevCard = CactusKevCard::default();
        let card = Standard52::card_from_index("K♦");

        cactus.set_rank_prime(&card);
        assert_eq!("00000000 00000000 00000000 00100101", cactus.display(true));

        cactus.set_rank(&card);
        assert_eq!("00000000 00000000 00001011 00100101", cactus.display(true));

        cactus.set_rank_flag(&card);
        assert_eq!("00001000 00000000 00001011 00100101", cactus.display(true));

        cactus.set_suit(&card);
        assert_eq!("00001000 00000000 01001011 00100101", cactus.display(true));

        // println!("{}", cactus.display(true));
        // println!("{:032b}", card.binary_signature());
        // println!("{:#}", cactus);
    }

    #[test]
    fn set_suit() {
        let mut cactus: CactusKevCard = CactusKevCard::default();

        let card = Standard52::card_from_index("KS");
        cactus.set_suit(&card);
        assert_eq!("00000000 00000000 00010000 00000000", cactus.display(true));

        let card = Standard52::card_from_index("KH");
        let mut cactus: CactusKevCard = CactusKevCard::default();
        cactus.set_suit(&card);
        assert_eq!("00000000 00000000 00100000 00000000", cactus.display(true));

        let card = Standard52::card_from_index("K♦");
        let mut cactus: CactusKevCard = CactusKevCard::default();
        cactus.set_suit(&card);
        assert_eq!("00000000 00000000 01000000 00000000", cactus.display(true));

        let card = Standard52::card_from_index("KC");
        let mut cactus: CactusKevCard = CactusKevCard::default();
        cactus.set_suit(&card);
        assert_eq!("00000000 00000000 10000000 00000000", cactus.display(true));
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

    #[test]
    fn new_from_u64() {
        let standard52 = Standard52::default();
        for card in standard52.deck {
            let cactusKevCardFromCard: CactusKevCard = CactusKevCard::new_from_card(&card);
            let cactusKevCardFromU64 = CactusKevCard::new_from_u64(card.binary_signature());
            // println!("{} {}", card, card.binary_signature());
            assert_eq!(
                cactusKevCardFromU64.display(true),
                cactusKevCardFromCard.display(true)
            );
        }
    }

    // AS 268442665
    // KS 134224677
    // QS 67115551
    // JS 33560861
    // TS 16783383
    // 9S 8394515
    // 8S 4199953
    // 7S 2102541
    // 6S 1053707
    // 5S 529159
    // 4S 266757
    // 3S 135427
    // 2S 69634
    // AH 268446761
    // KH 134228773
    // QH 67119647
    // JH 33564957
    // TH 16787479
    // 9H 8398611
    // 8H 4204049
    // 7H 2106637
    // 6H 1057803
    // 5H 533255
    // 4H 270853
    // 3H 139523
    // 2H 73730
    // AD 268454953
    // KD 134236965
    // QD 67127839
    // JD 33573149
    // TD 16795671
    // 9D 8406803
    // 8D 4212241
    // 7D 2114829
    // 6D 1065995
    // 5D 541447
    // 4D 279045
    // 3D 147715
    // 2D 81922
    // AC 268471337
    // KC 134253349
    // QC 67144223
    // JC 33589533
    // TC 16812055
    // 9C 8423187
    // 8C 4228625
    // 7C 2131213
    // 6C 1082379
    // 5C 557831
    // 4C 295429
    // 3C 164099
    // 2C 98306
    #[test]
    fn from_int() {
        let acespades: u64 = 268442665;
        let s = "00010000 00000000 00011100 00101001".to_string();
        let cactusKevCardFromU64 = CactusKevCard::new_from_u64(acespades);

        assert_eq!(cactusKevCardFromU64.display(true), s);
        // println!("{:#}", cactusKevCardFromU64);

        // this counts how many bit flags are set to true
        // let i = cactusKevCardFromU64.bites.iter().filter(|b| **b).count();
        // println!("{}", i);
        //
        // for j in cactusKevCardFromU64.bites.iter() {
        //     println!("{}", j);
        // }
    }
}
