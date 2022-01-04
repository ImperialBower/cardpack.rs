use crate::cards::card_error::CardError;
use crate::games::poker::cactus_kev_card::{ckc, CKC};
use crate::games::poker::cactus_kev_cards::CactusKevCards;
use crate::games::poker::hand_rank::{HandRank, HandRankName};
use crate::games::poker::vsupalov::lookups;
use crate::Pile;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

pub const POSSIBLE_COMBINATIONS: usize = 7937;
// 00000000 00000000 11110000 00000000
pub const SUITS_FILTER: u32 = 0xf000;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CactusKevHand([CKC; 5]);

impl CactusKevHand {
    #[must_use]
    pub fn new(cards: [CKC; 5]) -> CactusKevHand {
        CactusKevHand(cards)
    }

    /// Returns a valid `CactusKevHand` if the entered index string splits out into exactly
    /// five valid `Card` index strings.
    ///
    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid `Card` in the index.
    ///
    /// Will return `CardError::InvalidIndex` if not exactly 5 `Card`s.
    ///
    /// # Panics
    ///
    /// Shouldn't be able to panic. (fingers crossed)
    ///
    pub fn from_index(index: &'static str) -> Result<CactusKevHand, CardError> {
        let cards = CactusKevCards::from_index(index);

        if let Err(CardError::InvalidCard) = cards {
            return Err(CardError::InvalidCard);
        }

        let cards = cards.unwrap();
        if !cards.is_complete_hand() {
            return Err(CardError::InvalidIndex);
        }
        Ok(CactusKevHand::new(cards.to_five_array().unwrap()))
    }

    #[must_use]
    pub fn eval(&self) -> HandRank {
        let i = self.or_rank_bits();

        if self.is_flush() {
            return HandRank::new(lookups::FLUSHES[i]);
        }

        let unique = CactusKevHand::unique(i);
        if unique.value != 0 {
            return unique;
        }

        // It's not a flush and the cards aren't unique (straight or high card).
        self.last_pass()
    }

    /// Based on [this](https://github.com/vsupalov/pokereval-rs/blob/d244030715560dbae38c68dbcd09244d5285b518/src/original.rs#L6)
    /// which is in turn based on [find fast method](http://suffe.cool/poker/code/pokerlib.c) from Cactus Kev's original C code.
    ///
    /// TODO: Refactor to [Rust-PHF](https://github.com/rust-phf/rust-phf)
    #[must_use]
    pub fn find_it(key: usize) -> usize {
        let mut low = 0;
        let mut high = 4887;
        let mut mid;

        while low <= high {
            mid = (high + low) >> 1; // divide by two

            let product = lookups::PRODUCTS[mid] as usize;
            match key.cmp(&product) {
                Ordering::Less => high = mid - 1,
                Ordering::Greater => low = mid + 1,
                Ordering::Equal => return mid,
            }
        }
        0
    }

    #[must_use]
    pub fn is_flush(&self) -> bool {
        (self.0[0] & self.0[1] & self.0[2] & self.0[3] & self.0[4] & SUITS_FILTER) != 0
    }

    fn last_pass(&self) -> HandRank {
        let i = CactusKevHand::find_it(self.multiply_primes());
        HandRank::new(lookups::VALUES[i])
    }

    #[must_use]
    pub fn multiply_primes(&self) -> usize {
        ((self.0[0] & 0xff)
            * (self.0[1] & 0xff)
            * (self.0[2] & 0xff)
            * (self.0[3] & 0xff)
            * (self.0[4] & 0xff)) as usize
    }

    /// Returns a value that is made up of performing an or operation on all of the
    /// rank bit flags of the `CactusKevCard`.
    #[must_use]
    pub fn or_rank_bits(&self) -> usize {
        ((self.0[0] | self.0[1] | self.0[2] | self.0[3] | self.0[4]) as usize) >> 16
    }

    #[must_use]
    pub fn to_cactus_kev_cards(&self) -> CactusKevCards {
        CactusKevCards::new(self.0.to_vec())
    }

    #[must_use]
    pub fn to_pile(&self) -> Pile {
        CactusKevCards::new(self.0.to_vec()).to_pile()
    }

    #[must_use]
    pub fn unique(index: usize) -> HandRank {
        if index > POSSIBLE_COMBINATIONS {
            return HandRank::default();
        }
        HandRank::new(lookups::UNIQUE_5[index])
    }

    /// Performs the verification of
    /// [Cactus Kev's Hand Rank breakdown](https://suffe.cool/poker/evaluator.html).
    ///
    #[must_use]
    pub fn all_possible_combos() -> (HashMap<HandRankName, usize>, HashMap<HandRank, bool>) {
        let mut rank_class_count: HashMap<HandRankName, usize> = HashMap::new();
        let mut rank_count: HashMap<HandRank, bool> = HashMap::new();

        let dummy_kev_value: CKC = 0;
        let mut current_hand: [CKC; 5] = [dummy_kev_value; 5];

        // 2,598,960 unique poker hands
        for i1 in 0..52 {
            for i2 in (i1 + 1)..52 {
                for i3 in (i2 + 1)..52 {
                    for i4 in (i3 + 1)..52 {
                        for i5 in (i4 + 1)..52 {
                            current_hand[0] = ckc::DECK[i1];
                            current_hand[1] = ckc::DECK[i2];
                            current_hand[2] = ckc::DECK[i3];
                            current_hand[3] = ckc::DECK[i4];
                            current_hand[4] = ckc::DECK[i5];

                            let ckc_hand = CactusKevHand::new(current_hand);

                            let rank = ckc_hand.eval();

                            // mark the rank in the map
                            rank_count.entry(rank).or_insert(true);
                        }
                    }
                }
            }
        }

        for key in rank_count.keys() {
            let rank_class = key.name.clone();

            let count = rank_class_count.entry(rank_class).or_insert(0);
            *count += 1;
        }

        (rank_class_count, rank_count)
    }
}

impl fmt::Display for CactusKevHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand = self.clone().to_cactus_kev_cards();
        write!(f, "{} {}", hand, self.eval())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_hand_tests {
    use super::*;
    use crate::games::poker::hand_rank::HandRankValue;
    use crate::Standard52;
    use rstest::rstest;

    #[test]
    fn from_index__invalid_index() {
        let hand = CactusKevHand::from_index("K♠ Q♠ J♠ T♠");

        assert!(hand.is_err());
        assert_eq!(hand.unwrap_err(), CardError::InvalidIndex);
    }

    #[test]
    fn from_index__invalid_card() {
        let hand = CactusKevHand::from_index("AX K♠ Q♠ J♠ T♠");

        assert!(hand.is_err());
        assert_eq!(hand.unwrap_err(), CardError::InvalidCard);
    }

    #[rstest]
    #[case("A♠ K♠ Q♠ J♠ T♠", 1, HandRankName::StraightFlush)]
    #[case("A♣ 2♣ 3♣ 4♣ 5♣", 10, HandRankName::StraightFlush)]
    #[case("A♠ A♥ A♦ A♣ K♠", 11, HandRankName::FourOfAKind)]
    #[case("2♠ 2♥ 2♦ 2♣ 3♠", 166, HandRankName::FourOfAKind)]
    #[case("A♠ A♥ A♦ K♠ K♦", 167, HandRankName::FullHouse)]
    #[case("2♠ 2♥ 2♦ 3♠ 3♦", 322, HandRankName::FullHouse)]
    #[case("A♠ K♠ Q♠ J♠ 9♠", 323, HandRankName::Flush)]
    #[case("2♣ 3♣ 4♣ 5♣ 7♣", 1599, HandRankName::Flush)]
    #[case("A♣ K♠ Q♠ J♠ T♠", 1600, HandRankName::Straight)]
    #[case("A♥ 2♣ 3♣ 4♣ 5♣", 1609, HandRankName::Straight)]
    #[case("A♠ A♥ A♦ K♠ Q♣", 1610, HandRankName::ThreeOfAKind)]
    #[case("2♠ 2♥ 2♦ 3♠ 4♣", 2467, HandRankName::ThreeOfAKind)]
    #[case("A♠ A♥ K♦ K♠ Q♣", 2468, HandRankName::TwoPair)]
    #[case("3♠ 3♥ 2♦ 2♠ 4♣", 3325, HandRankName::TwoPair)]
    #[case("A♠ A♥ K♠ Q♠ J♠", 3326, HandRankName::Pair)]
    #[case("2♠ 2♥ 3♠ 4♠ 5♠", 6185, HandRankName::Pair)]
    #[case("A♠ K♠ Q♠ J♠ 9♣", 6186, HandRankName::HighCard)]
    #[case("2♣ 3♣ 4♣ 5♥ 7♣", 7462, HandRankName::HighCard)]
    #[case("2♣ 3♦ 4♣ 5♥ 7♣", 7462, HandRankName::HighCard)]
    fn eval(
        #[case] index: &'static str,
        #[case] hand_rank_value: HandRankValue,
        #[case] hand_rank_name: HandRankName,
    ) {
        let hand = CactusKevHand::from_index(index).unwrap();

        let actual_hand_rank = hand.eval();

        assert_eq!(hand_rank_value, actual_hand_rank.value);
        assert_eq!(hand_rank_name, actual_hand_rank.name);
        assert_eq!(HandRank::new(hand_rank_value), actual_hand_rank);
    }

    #[test]
    fn eval__royal_flush() {
        assert_eq!(
            HandRank::new(1),
            CactusKevHand::from_index("AC KC QC JC TC").unwrap().eval()
        );
    }

    #[test]
    fn is_flush() {
        assert!(CactusKevHand::from_index("AC KC QC JC TC")
            .unwrap()
            .is_flush());
        assert!(CactusKevHand::from_index("KS QS JS TS 3S")
            .unwrap()
            .is_flush());
        assert!(CactusKevHand::from_index("KS QS JS TS 6S")
            .unwrap()
            .is_flush());
        assert!(!CactusKevHand::from_index("KS QS JS TS 6D")
            .unwrap()
            .is_flush());
    }

    #[test]
    fn or_rank_bits() {
        let hand = CactusKevHand::from_index("AS KS QS JS TS").unwrap();

        assert_eq!("0001111100000000", format!("{:016b}", hand.or_rank_bits()));
        assert_eq!(hand.or_rank_bits(), 7936);
    }

    #[test]
    fn to_cactus_kev_cards() {
        let cards = CactusKevCards::from_index("AS KS QS JS TS").unwrap();
        let hand = CactusKevHand::from_index("AS KS QS JS TS").unwrap();

        assert_eq!(cards, hand.to_cactus_kev_cards());
    }

    #[test]
    fn to_pile() {
        let index = "AS KS QS JS TS";
        let pile = Standard52::pile_from_index(index.clone()).unwrap();
        let hand = CactusKevHand::from_index(index).unwrap();

        assert_eq!(pile, hand.to_pile())
    }

    #[test]
    fn display() {
        assert_eq!(
            "A♠ K♠ Q♠ J♠ T♠ HandRank { value: 1, name: StraightFlush }",
            format!("{}", CactusKevHand::from_index("AS KS QS JS TS").unwrap())
        );
    }

    #[test]
    fn display__invalid() {
        assert_eq!(
            "A♠ A♠ Q♠ J♠ T♠ HandRank { value: 0, name: Invalid }",
            format!("{}", CactusKevHand::from_index("AS AS QS JS TS").unwrap())
        );
    }

    #[test]
    fn eval_on_all_possible_combinations() {
        let (rank_class_count, rank_count) = CactusKevHand::all_possible_combos();

        // There should be 7462 unique ranks
        assert_eq!(rank_count.len(), 7462);

        assert_eq!(
            *rank_class_count.get(&HandRankName::HighCard).unwrap(),
            1277
        );

        assert_eq!(*rank_class_count.get(&HandRankName::Pair).unwrap(), 2860);

        assert_eq!(*rank_class_count.get(&HandRankName::TwoPair).unwrap(), 858);

        assert_eq!(
            *rank_class_count.get(&HandRankName::ThreeOfAKind).unwrap(),
            858
        );

        assert_eq!(*rank_class_count.get(&HandRankName::Straight).unwrap(), 10);

        assert_eq!(*rank_class_count.get(&HandRankName::Flush).unwrap(), 1277);

        assert_eq!(
            *rank_class_count.get(&HandRankName::FullHouse).unwrap(),
            156
        );

        assert_eq!(
            *rank_class_count.get(&HandRankName::FourOfAKind).unwrap(),
            156
        );

        assert_eq!(
            *rank_class_count.get(&HandRankName::StraightFlush).unwrap(),
            10
        );
    }
}
