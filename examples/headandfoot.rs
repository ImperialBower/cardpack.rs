use cardpack::prelude::*;

/// [WikiHow: Hand and Foot](https://www.wikihow.com/Play-Hand-and-Foot)
fn main() {
    let headfootdeck = French::decks(5);

    headfootdeck.demo_cards(true);
}
