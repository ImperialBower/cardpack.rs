fn main() {
    println!("Let's deal out a heads up hand of Texas Holdem:\n");
    let pack = cardpack::Pack::french_deck();

    let mut shuffled = pack.cards().shuffle();
    let sb = shuffled.draw(2).unwrap();
    let bb = shuffled.draw(2).unwrap();

    println!("small blind: {}", sb.to_symbol_index());
    println!("big blind:   {}", bb.to_symbol_index());

    println!();
    println!("flop : {}", shuffled.draw(3).unwrap().to_symbol_index());
    println!("turn : {}", shuffled.draw(1).unwrap().to_symbol_index());
    println!("river: {}", shuffled.draw(1).unwrap().to_symbol_index());
}
