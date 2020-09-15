fn main() {
    let deck = cardpack::Pack::tarot_deck();

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
}

fn display(deck: &cardpack::Pack) {
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
