use super::lookups;
use crate::games::poker::alt::holdem::{CactusKevCard, HandRank};

#[allow(clippy::comparison_chain, dead_code)]
fn findit(key: usize) -> usize {
    let mut low = 0;
    let mut high = 4887;
    let mut mid;

    while low <= high {
        mid = (high + low) >> 1; // divide by two

        if key < lookups::PRODUCTS[mid] as usize {
            high = mid - 1;
        } else if key > lookups::PRODUCTS[mid] as usize {
            low = mid + 1;
        } else {
            return mid;
        }
    }

    panic!("No match found")
}

/// Returns a value between (1 to 7462 inclusive), where 1 is the best. It is called original because of the old handRank convention.
//TODO: make this line less fugly?
#[allow(clippy::trivially_copy_pass_by_ref, dead_code)]
fn eval_5cards_kev_original(
    c1: &CactusKevCard,
    c2: &CactusKevCard,
    c3: &CactusKevCard,
    c4: &CactusKevCard,
    c5: &CactusKevCard,
) -> HandRank {
    let q: usize = ((c1 | c2 | c3 | c4 | c5) as usize) >> 16;

    if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        return lookups::FLUSHES[q] as HandRank;
    }
    let s = lookups::UNIQUE_5[q] as HandRank;
    if s != 0 {
        return s;
    }

    let q = ((c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff)) as usize;
    let lookup = findit(q);
    lookups::VALUES[lookup] as HandRank
}

// no array used -> for bench purposes

#[allow(clippy::trivially_copy_pass_by_ref, dead_code)]
pub fn eval_5cards_kev(
    c1: &CactusKevCard,
    c2: &CactusKevCard,
    c3: &CactusKevCard,
    c4: &CactusKevCard,
    c5: &CactusKevCard,
) -> HandRank {
    let kev_rank = eval_5cards_kev_original(c1, c2, c3, c4, c5);
    7461 - (kev_rank - 1) as HandRank //let's change this to be (0 to 7461 inclusive), with 7461 being the best
}

#[allow(dead_code)]
pub fn eval_5cards_kev_array(cards: &[&CactusKevCard; 5]) -> HandRank {
    let kev_rank = eval_5cards_kev_original(cards[0], cards[1], cards[2], cards[3], cards[4]);
    7461 - (kev_rank - 1) as HandRank //let's change this to be (0 to 7461 inclusive), with 7461 being the best
}

#[allow(dead_code)]
pub fn eval_6cards_kev_array(cards: &[&CactusKevCard; 6]) -> HandRank {
    let mut tmp;
    let mut best = 0;

    let dummy_kev_value = 0;
    let mut subhand: [&CactusKevCard; 5] = [&dummy_kev_value; 5];

    for ids in &lookups::PERM_6 {
        for i in 0..5 {
            subhand[i] = cards[ids[i] as usize];
        }

        tmp = eval_5cards_kev_array(&subhand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

#[allow(dead_code)]
pub fn eval_7cards_kev_array(cards: &[&CactusKevCard; 7]) -> HandRank {
    let mut tmp;
    let mut best = 0;

    let dummy_kev_value = 0;
    let mut subhand: [&CactusKevCard; 5] = [&dummy_kev_value; 5];

    for ids in &lookups::PERM_7 {
        for i in 0..5 {
            subhand[i] = cards[ids[i] as usize];
        }

        tmp = eval_5cards_kev_array(&subhand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use crate::games::poker::alt::holdem::{
        hand_rank_to_class, CactusKevCard, HandRank, HandRankClass,
    };
    use crate::games::poker::alt::vsup_deck::VSupDeck;
    use std::collections::HashMap;

    use super::eval_5cards_kev_array;

    // TODO: this is not really specific to this evaluation method. It could as well live in the library tests folder
    // the reason it is here, is due to the internal representation hackage which got ditched
    #[test]
    fn evaluate_all_possible_5_card_combinations() {
        let mut deck = VSupDeck::new_unshuffled();
        let mut cards: [CactusKevCard; 52] = [0; 52];

        // this could be made faster, by creating a function that works on raw-card-representations and translating
        // the deck cards into it
        for i in 0..52 {
            let card = deck.draw().ok().unwrap();
            cards[i] = card.card_to_deck_number();
        }

        let mut rank_class_count: HashMap<HandRankClass, usize> = HashMap::new();
        let mut rank_count: HashMap<HandRank, bool> = HashMap::new();

        let dummy_kev_value = 0;
        let mut current_hand: [&CactusKevCard; 5] = [&dummy_kev_value; 5];

        // 2,598,960 unique poker hands
        for i1 in 0..52 {
            for i2 in (i1 + 1)..52 {
                for i3 in (i2 + 1)..52 {
                    for i4 in (i3 + 1)..52 {
                        for i5 in (i4 + 1)..52 {
                            current_hand[0] = &cards[i1];
                            current_hand[1] = &cards[i2];
                            current_hand[2] = &cards[i3];
                            current_hand[3] = &cards[i4];
                            current_hand[4] = &cards[i5];

                            let rank = eval_5cards_kev_array(&current_hand);

                            // mark the rank in the map
                            rank_count.entry(rank).or_insert(true);
                        }
                    }
                }
            }
        }

        // count distinct ranks for each rank class
        for key in rank_count.keys() {
            let rank_class = hand_rank_to_class(key);

            let count = rank_class_count.entry(rank_class).or_insert(0);
            *count += 1;
        }

        // this is a bit redundant
        // there should be 7462 unique ranks, in accordance with the hand_rank_to_class function
        assert_eq!(rank_count.len(), 7462);

        assert_eq!(
            *rank_class_count.get(&HandRankClass::HighCard).unwrap(),
            1277
        );
        assert_eq!(
            *rank_class_count.get(&HandRankClass::OnePair).unwrap(),
            2860
        );
        assert_eq!(*rank_class_count.get(&HandRankClass::TwoPair).unwrap(), 858);
        assert_eq!(
            *rank_class_count.get(&HandRankClass::ThreeOfAKind).unwrap(),
            858
        );
        assert_eq!(*rank_class_count.get(&HandRankClass::Straight).unwrap(), 10);
        assert_eq!(*rank_class_count.get(&HandRankClass::Flush).unwrap(), 1277);
        assert_eq!(
            *rank_class_count.get(&HandRankClass::FullHouse).unwrap(),
            156
        );
        assert_eq!(
            *rank_class_count.get(&HandRankClass::FourOfAKind).unwrap(),
            156
        );
        assert_eq!(
            *rank_class_count.get(&HandRankClass::StraightFlush).unwrap(),
            10
        );
    }
}
