use cardpack::prelude::*;

fn main() {
    let deck = Standard52::deck();
    let mut shuffled = deck.shuffled();

    let small_blind = shuffled.draw(2).unwrap().sort_by_rank();
    let big_blind = shuffled.draw(2).unwrap().sort_by_rank();

    println!("small blind: {}", small_blind.to_string());
    println!("big blind:   {}", big_blind.to_string());

    println!();
    println!("flop : {}", shuffled.draw(3).unwrap().to_string());
    println!("turn : {}", shuffled.draw(1).unwrap().to_string());
    println!("river: {}", shuffled.draw(1).unwrap().to_string());
}
