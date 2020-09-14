fn main() {
    let deck = spielkartenlib::karten::Karten::tarot_deck();

    println!("Tarot Deck");
    display(&deck);

    println!();
    println!("Tarot Deck Shuffled");
    display(&deck.mischen());
}

fn display(deck: &spielkartenlib::karten::Karten) {
    for karte in deck.values() {
        let suitname = karte.anzug.name.to_string();

        let rankname = karte.rang.name.to_string();
        if suitname == "Major Arcana".to_string() {
            println!("      {}", rankname);
        } else {
            println!("      {} of {} ", rankname, suitname);
        }
    }
}
