use cardpack::prelude::*;

/// [WikiHow: Hand and Foot](https://www.wikihow.com/Play-Hand-and-Foot)
fn main() {
    let headfootdeck = FrenchDeck::decks(5);

    headfootdeck.demo_cards(true);
}
