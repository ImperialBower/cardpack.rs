fn main() {
    let pack = cardpack::Pack::french_deck();

    pack.cards().demo();

    let pack = cardpack::Pack::french_deck_with_jokers();
    println!();
    pack.cards().demo();
}
