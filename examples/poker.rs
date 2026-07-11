use cardpack::prelude::*;

fn main() {
    let mut pack = Standard52::deck();

    pack.shuffle();

    let small_blind = pack.draw(2).unwrap().sorted_by_rank();
    let big_blind = pack.draw(2).unwrap().sorted_by_rank();

    println!("small blind: {small_blind}");
    println!("big blind:   {big_blind}");

    let flop = pack.draw(3).unwrap();
    let turn = pack.draw(1).unwrap();
    let river = pack.draw(1).unwrap();

    println!();
    println!("flop : {flop}");
    println!("turn : {turn}");
    println!("river: {river}");

    // Now, let's validate that the cards when collected back together are a valid Standard52
    // deck of cards.
    let reconstituted_pile =
        Pile::<Standard52>::pile_on(&[pack, small_blind, big_blind, flop, turn, river]);
    assert!(Standard52::deck().same(&reconstituted_pile));
}
