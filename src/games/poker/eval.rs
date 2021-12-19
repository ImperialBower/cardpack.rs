use crate::{Card, Pile, Suit};
use std::collections::HashMap;
// use std::collections::HashMap;

pub struct CardsBySuit {
    pub suit: Suit,
    pub cards: Vec<Card>,
}
//
// struct Suits {
//     keys: Vec<Suit>,
// }

// fn foo(fud: &Fud) {
//     let mut results: HashMap<&String, Vec<f64>> = HashMap::new();
//
//     for k in fud.keys.iter() {
//         results.insert(k, Vec::new());
//     }
//
//     for v in fud.keys.iter() {
//         results.get_mut(v).unwrap().push(0.0);
//     }
// }

/// <https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry//>
/// <https://www.reddit.com/r/rust/comments/9xho3i/i_have_a_hashmap_that_pairs_strings_with_vectors//>
#[must_use]
pub fn sort_by_suit(_pile: &Pile) -> HashMap<Suit, Pile> {
    let sorted: HashMap<Suit, Pile> = HashMap::new();

    sorted
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_tests {
    use super::*;
    use crate::Standard52;
    use rstest::rstest;

    #[rstest]
    #[case("2S 3S 9S TS QS JH Ac")]
    fn to_a_flush(#[case] input: &'static str) {
        let pile = Standard52::pile_from_index(input).unwrap();

        let _sorted = sort_by_suit(&pile);
    }
}
