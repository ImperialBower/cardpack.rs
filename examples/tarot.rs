fn main() {
    let deck = spielkartenlib::karten::Deck::tarot_deck();

    println!("Tarot Deck");
    display(&deck);

    println!();
    println!("Tarot Deck Shuffled");
    display(&deck.mischen());
}

fn display(deck: &spielkartenlib::karten::Deck) {
    for karte in deck.values() {
        let suitname = karte.suit.name.to_string();

        let rankname = karte.rank.name.to_string();
        if suitname == "Major Arcana".to_string() {
            println!("      {}", rankname);
        } else {
            println!("      {} of {} ", rankname, suitname);
        }
    }
}
