fn main() {
    println!("Let's show a Short Deck");
    let pack = cardpack::Pack::short_deck();

    pack.cards().demo();

    println!();
    println!("Now, let's deal out a heads up hand of Short Deck Poker:\n");
    let pack = cardpack::Pack::short_deck();

    let mut shuffled = pack.cards().shuffle();
    // TODO: Make it so that each player is dealt one card at a time.
    let first = shuffled.draw(2).unwrap();
    let second = shuffled.draw(2).unwrap();
    let button = shuffled.draw(2).unwrap();
    let sb = shuffled.draw(2).unwrap();
    let bb = shuffled.draw(2).unwrap();

    println!("first seat: {}", first.to_symbol_index());
    println!("second seat: {}", second.to_symbol_index());
    println!("button: {}", button.to_symbol_index());
    println!("small blind: {}", sb.to_symbol_index());
    println!("big blind:   {}", bb.to_symbol_index());

    println!();
    println!("flop : {}", shuffled.draw(3).unwrap().to_symbol_index());
    println!("turn : {}", shuffled.draw(1).unwrap().to_symbol_index());
    println!("river: {}", shuffled.draw(1).unwrap().to_symbol_index());
}
