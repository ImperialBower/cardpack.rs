fn main() {
    let pack = cardpack::Pack::french_deck();
    let shuffled = pack.cards().shuffle();

    let card_str = shuffled.to_symbol_index();

    let _rawcards: Vec<&str> = card_str.split(' ').collect();

    // for _s in rawcards {
    //
    // }

    println!();

    print!("{}", card_str);
}
