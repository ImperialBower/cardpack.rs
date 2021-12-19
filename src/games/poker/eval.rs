use crate::{Card, Suit};
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
