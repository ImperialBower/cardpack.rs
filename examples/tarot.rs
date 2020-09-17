fn main() {
    let deck = cardpack::Pile::tarot_deck();

    println!("Tarot Deck");
    display(&deck);

    println!();
    println!("Tarot Deck Shuffled");
    let mut shuffled = deck.shuffle();
    display(&shuffled);

    println!();
    println!("Tarot Deck Sorted");
    shuffled.sort();
    display(&shuffled);

    deck.demo();
}

fn display(deck: &cardpack::Pile) {
    for card in deck.values() {
        let suitname = card.suit.get_default_long();

        let rankname = card.rank.get_default_long();
        if suitname == "Major Arcana".to_string() {
            println!("      {}", rankname);
        } else {
            println!("      {} of {} ", rankname, suitname);
        }
    }
}
