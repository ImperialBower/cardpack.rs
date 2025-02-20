use cardpack::prelude_old::*;

fn main() {
    let pack = Pack::french_deck();

    pack.cards().demo();

    let pack = Pack::french_deck_with_jokers();
    println!();
    pack.cards().demo();
}
