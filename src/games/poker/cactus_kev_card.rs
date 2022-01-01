/// A card encoded using the bit pattern described in Cactus Kev's
/// [article](http://www.suffecool.net/poker/evaluator.html).
pub type CKC = u32;
pub type HandRank = u16;

// 00000000 00000000 11110000 00000000
#[allow(dead_code)]
pub const SUITS_FILTER: u32 = 0xf000;

pub mod ckc {
    use crate::games::poker::bit_card::BitCard;
    use crate::games::poker::cactus_kev_card::CKC;
    use crate::Card;

    /// Creates [Cactus Kev's Hand Evaluator](http://suffe.cool/poker/evaluator.html) value.
    /// ```txt
    /// +--------+--------+--------+--------+
    /// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
    /// +--------+--------+--------+--------+
    ///
    /// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
    /// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
    /// cdhs = suit of card (bit turned on based on suit of card)
    /// b = bit turned on depending on rank of card
    /// ```
    /// This is used for Poker hand evaluation.
    #[must_use]
    pub fn from_card(card: &Card) -> CKC {
        let suit: u32 = card.suit.binary_signature();
        let bits = 1 << (16 + card.rank.weight);
        let rank_eight = card.rank.weight << 8;

        // println!("{} | {} | {} | {}", bits, self.rank.prime, rank_eight, suit);

        bits | card.rank.prime | rank_eight | suit
    }

    #[must_use]
    pub fn to_card(ckc: &CKC) -> Card {
        BitCard::from_cactus_kev_card(ckc).to_card()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod ckc_tests {
    use super::*;
    use crate::{Card, DIAMONDS, KING};

    #[test]
    fn from_card() {
        let card = Card::from_index_strings(KING, DIAMONDS);

        assert_eq!(ckc::from_card(&card), 134236965);
    }
}
