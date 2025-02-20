use cardpack::prelude_old::*;

fn main() {
    let canasta = Pack::canasta_deck();

    canasta.cards().demo();
}
