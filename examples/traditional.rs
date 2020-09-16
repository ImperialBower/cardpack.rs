fn main() {
    let deck = cardpack::Pack::french_deck();

    let cards = deck.cards().clone();
    cards.demo();
}
