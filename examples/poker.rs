use cardpack::prelude::*;

fn main() {
    let mut pack = Standard52::deck();

    pack.shuffle();

    let small_blind = pack.draw(2).unwrap().sort_by_rank();
    let big_blind = pack.draw(2).unwrap().sort_by_rank();

    println!("small blind: {}", small_blind.to_string());
    println!("big blind:   {}", big_blind.to_string());

    let flop = pack.draw(3).unwrap();
    let turn = pack.draw(1).unwrap();
    let river = pack.draw(1).unwrap();

    println!();
    println!("flop : {}", flop.to_string());
    println!("turn : {}", turn.to_string());
    println!("river: {}", river.to_string());

    // Now, let's validate that the cards when collected back together are a valid Standard52
    // deck of cards.
    let reconstituted_pile =
        Pile::<Standard52>::pile_on(&*vec![pack, small_blind, big_blind, flop, turn, river]);
    assert!(Standard52::deck().same(&reconstituted_pile));
}
