fn main() {
    let deck = cardpack::Pile::tarot_deck();

    println!("Tarot Deck");
    display(&deck);

    println!();
    println!("Tarot Deck Shuffled");
    let shuffled = deck.shuffle();
    display(&shuffled);

    println!();

    deck.demo_short();
}

fn display(deck: &cardpack::Pile) {
    for card in deck.values() {
        let suitname = card.suit.name.long_default();

        let rankname = card.rank.name.long_default();
        if suitname == "Major Arcana".to_string() {
            println!("      {}", rankname);
        } else {
            println!("      {} of {} ", rankname, suitname);
        }
    }
}
