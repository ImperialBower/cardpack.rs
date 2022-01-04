/// A card encoded using the bit pattern described in Cactus Kev's
/// [article](http://www.suffecool.net/poker/evaluator.html).
pub type CKC = u32;

// 00000000 00000000 11110000 00000000
#[allow(dead_code)]
pub const SUITS_FILTER: u32 = 0xf000;

pub mod ckc {
    use crate::games::poker::bit_card::BitCard;
    use crate::games::poker::cactus_kev_card::CKC;
    use crate::{Card, Standard52};

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
    pub fn from_index(index: &'static str) -> Option<CKC> {
        let card = Standard52::card_from_index(index);
        if card.is_valid() {
            Some(from_card(&card))
        } else {
            None
        }
    }

    #[must_use]
    pub fn to_card(ckc: &CKC) -> Card {
        BitCard::from_cactus_kev_card(ckc).to_card()
    }

    /// Returns an Standard52 deck as an array of Cactus Kev Cards (`CKC`).
    pub const DECK: [CKC; 52] = [
        268_442_665,
        134_224_677,
        67_115_551,
        33_560_861,
        16_783_383,
        8_394_515,
        4_199_953,
        2_102_541,
        1_053_707,
        529_159,
        266_757,
        135_427,
        69_634,
        268_446_761,
        134_228_773,
        67_119_647,
        33_564_957,
        16_787_479,
        8_398_611,
        4_204_049,
        2_106_637,
        1_057_803,
        533_255,
        270_853,
        139_523,
        73_730,
        268_454_953,
        134_236_965,
        67_127_839,
        33_573_149,
        16_795_671,
        8_406_803,
        4_212_241,
        2_114_829,
        1_065_995,
        541_447,
        279_045,
        147_715,
        81_922,
        268_471_337,
        134_253_349,
        67_144_223,
        33_589_533,
        16_812_055,
        8_423_187,
        4_228_625,
        2_131_213,
        1_082_379,
        557_831,
        295_429,
        164_099,
        98_306,
    ];
}

#[cfg(test)]
#[allow(non_snake_case)]
mod ckc_tests {
    use super::*;
    use crate::{Card, Standard52, DIAMONDS, KING};

    #[test]
    fn from_card() {
        let card = Card::from_index_strings(KING, DIAMONDS);

        assert_eq!(ckc::from_card(&card), 134236965);
    }

    #[test]
    fn deck() {
        let standard52 = Standard52::default();

        for (i, card) in standard52.deck.into_iter().enumerate() {
            assert_eq!(ckc::DECK[i], ckc::from_card(&card));
            assert_eq!(ckc::to_card(&ckc::DECK[i]), card);
        }
    }
}
