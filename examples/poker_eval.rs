//! Wires `cardpack`'s `Pile<Standard52>` to the `ckc-rs` poker hand evaluator.
//!
//! cardpack ships the *primitives* for Cactus Kev encoding (`CKCRevised` trait
//! on `BasicCard`) but not the evaluator itself — that lives in `ckc-rs`.
//! This example shows the integration pattern: deal a Texas Hold'em board,
//! enumerate each player's 21 possible 5-card hands from their 7 cards
//! (2 hole + 5 community), score each via `ckc_rs::evaluate::five_cards`,
//! and pick the lowest score (best hand) per player.
//!
//! Run with: `cargo run --example poker_eval`

use cardpack::basic::types::traits::Ranged;
use cardpack::prelude::*;
use ckc_rs::hand_rank::HandRank;
use ckc_rs::{CKCNumber, evaluate};

fn best_hand_for(seven: &Pile<Standard52>) -> HandRank {
    let mut best = HandRank::from(0);
    let mut best_value = u32::MAX;

    for five in seven.combos(5) {
        let mut numbers = [0 as CKCNumber; 5];
        for (i, card) in five.iter().enumerate() {
            numbers[i] = card.get_ckc_number() as CKCNumber;
        }

        let value = evaluate::five_cards(numbers);
        if value > 0 && (value as u32) < best_value {
            best_value = value as u32;
            best = HandRank::from(value);
        }
    }

    best
}

fn main() {
    let mut deck = Standard52::deck();
    deck.shuffle();

    let alice_hole = deck.draw(2).unwrap().sorted_by_rank();
    let bob_hole = deck.draw(2).unwrap().sorted_by_rank();
    let flop = deck.draw(3).unwrap();
    let turn = deck.draw(1).unwrap();
    let river = deck.draw(1).unwrap();

    println!("Alice hole: {alice_hole}");
    println!("Bob hole:   {bob_hole}");
    println!();
    println!("Flop:  {flop}");
    println!("Turn:  {turn}");
    println!("River: {river}");
    println!();

    let board = Pile::<Standard52>::pile_on(&[flop, turn, river]);

    let alice_seven = Pile::<Standard52>::pile_on(&[alice_hole, board.clone()]);
    let bob_seven = Pile::<Standard52>::pile_on(&[bob_hole, board]);

    let alice = best_hand_for(&alice_seven);
    let bob = best_hand_for(&bob_seven);

    println!(
        "Alice best: {:?} ({:?}, value {})",
        alice.name, alice.class, alice.value
    );
    println!(
        "Bob   best: {:?} ({:?}, value {})",
        bob.name, bob.class, bob.value
    );
    println!();

    match alice.value.cmp(&bob.value) {
        std::cmp::Ordering::Less => println!("Alice wins."),
        std::cmp::Ordering::Greater => println!("Bob wins."),
        std::cmp::Ordering::Equal => println!("Split pot."),
    }
}
