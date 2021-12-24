use crate::Card;
use bitvec::field::BitField;
use bitvec::prelude::{BitArray, Msb0};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitCard(BitArray<Msb0, [u8; 4]>);

impl BitCard {
    #[must_use]
    pub fn new(b: BitArray<Msb0, [u8; 4]>) -> BitCard {
        BitCard(b)
    }

    #[must_use]
    #[allow(clippy::needless_borrow)]
    pub fn new_from_card(card: &Card) -> BitCard {
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_rank(&card);
        bit_card.set_rank_flag(&card);
        bit_card.set_rank_prime(&card);
        bit_card.set_suit(&card);
        bit_card
    }

    fn set_rank(&mut self, card: &Card) {
        self.0[20..24].store_be(card.rank.weight);
    }

    fn set_rank_flag(&mut self, card: &Card) {
        match card.rank.weight {
            12 => self.0.set(3, true), // Ace
            11 => self.0.set(4, true), // King
            10 => self.0.set(5, true), // Queen
            9 => self.0.set(6, true),  // Jack
            8 => self.0.set(7, true),  // Ten
            7 => self.0.set(8, true),  // Nine
            6 => self.0.set(9, true),  // Eight
            5 => self.0.set(10, true), // Seven
            4 => self.0.set(11, true), // Six
            3 => self.0.set(12, true), // Five
            2 => self.0.set(13, true), // Four
            1 => self.0.set(14, true), // Three
            0 => self.0.set(15, true), // Two
            _ => (),
        }
    }

    fn set_rank_prime(&mut self, card: &Card) {
        self.0[26..32].store_be(card.rank.prime);
    }

    fn set_suit(&mut self, card: &Card) {
        match card.suit.weight {
            4 => self.0.set(19, true), // Spades
            3 => self.0.set(18, true), // Hearts
            2 => self.0.set(17, true), // Diamonds
            1 => self.0.set(16, true), // Clubs
            _ => (),
        }
    }

    /// Takes the `BitArray` representation of the Card and returns a `String`
    /// representation of the bits. If split is set to true, it will put a space
    /// between each bite. For instance, `00001000000000000100101100100101`
    /// becomes `00001000 00000000 01001011 00100101`.
    #[must_use]
    pub fn display(&self, split: bool) -> String {
        let mut word_string = String::with_capacity(35);
        let start_bit: usize = 0;
        let bits = start_bit..start_bit + 32;
        for (bit, idx) in self.0.as_bitslice().iter().by_val().zip(bits) {
            word_string.push_str(if bit { "1" } else { "0" });
            if split && idx % 8 == 7 && idx % 32 != 31 {
                word_string.push(' ');
            }
        }
        word_string
    }
}

impl Default for BitCard {
    fn default() -> BitCard {
        BitCard::new(BitArray::zeroed())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_card_tests {
    use super::*;
    use crate::Standard52;

    #[test]
    fn set_rank() {
        let mut bit_card: BitCard = BitCard::default();
        let card = Standard52::card_from_index("K♦");

        bit_card.set_rank(&card);
        assert_eq!(
            "00000000 00000000 00001011 00000000",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_rank_flag() {
        let mut bit_card: BitCard = BitCard::default();
        let card = Standard52::card_from_index("K♦");

        bit_card.set_rank_flag(&card);
        assert_eq!(
            "00001000 00000000 00000000 00000000",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_rank_prime() {
        let mut bit_card: BitCard = BitCard::default();
        let card = Standard52::card_from_index("K♦");

        bit_card.set_rank_prime(&card);
        assert_eq!(
            "00000000 00000000 00000000 00100101",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_suit() {
        let mut bit_card: BitCard = BitCard::default();

        let card = Standard52::card_from_index("KS");
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 00010000 00000000",
            bit_card.display(true)
        );

        let card = Standard52::card_from_index("KH");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 00100000 00000000",
            bit_card.display(true)
        );

        let card = Standard52::card_from_index("K♦");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 01000000 00000000",
            bit_card.display(true)
        );

        let card = Standard52::card_from_index("KC");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 10000000 00000000",
            bit_card.display(true)
        );
    }
}
