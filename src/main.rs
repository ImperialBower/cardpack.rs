use spielkartenlib::ToLocaleString;

fn main() {
    println!("spielkarten.rs demo\n");

    let french_deck = spielkartenlib::Kartendeck::french_deck();
    let pinochle_deck = spielkartenlib::Kartendeck::pinochle_deck();
    let spades_deck = spielkartenlib::Kartendeck::spades_deck();

    println!("French Deck:");
    demo(french_deck);

    println!("Pinochle Deck:");
    demo(pinochle_deck);

    println!("Spades Deck:");
    demo(spades_deck);

    demo_tarot_deck();
}

fn demo(deck: spielkartenlib::Kartendeck) {
    print!("   Short With Symbols:           ");
    for karte in deck.karten.values() {
        print!("{} ", karte);
    }

    println!();
    print!("   Short With Symbols in German: ");
    for karte in deck.karten.values() {
        print!(
            "{} ",
            karte.to_locale_string(&spielkartenlib::fluent::GERMAN)
        );
    }

    println!();
    print!("   Short With Letters:           ");
    for karte in deck.karten.values() {
        print!(
            "{} ",
            karte.to_txt_string(&spielkartenlib::fluent::US_ENGLISH)
        );
    }

    println!();
    print!("   Short With Letters in German: ");
    for karte in deck.karten.values() {
        print!("{} ", karte.to_txt_string(&spielkartenlib::fluent::GERMAN));
    }

    println!();
    print!("   Shuffle Deck:                 ");
    for karte in deck.shuffle().values() {
        print!(
            "{} ",
            karte.to_locale_string(&spielkartenlib::fluent::US_ENGLISH)
        );
    }

    println!();
    print!("   Long in English and German:\n");
    for karte in deck.karten.values() {
        let anzugname = karte
            .anzug
            .name
            .to_locale_string(&spielkartenlib::fluent::GERMAN);
        let suitname = karte
            .anzug
            .name
            .to_locale_string(&spielkartenlib::fluent::US_ENGLISH);
        let rangname = karte
            .rang
            .name
            .to_locale_string(&spielkartenlib::fluent::GERMAN);
        let rankname = karte
            .rang
            .name
            .to_locale_string(&spielkartenlib::fluent::US_ENGLISH);
        println!("      {} of {} ", rankname, suitname);
        println!("      {} von {} ", rangname, anzugname);
    }

    println!();
}

fn demo_tarot_deck() {
    println!("Tarot Deck");
    let deck = spielkartenlib::Kartendeck::tarot_deck();
    display_tarot(&deck.karten);

    println!();
    println!("Tarot Deck Shuffled");
    let cards = deck.shuffle();
    display_tarot(&cards);
}

fn display_tarot(cards: &spielkartenlib::karten::Karten) {
    for karte in cards.values() {
        let suitname = karte
            .anzug
            .name
            .to_locale_string(&spielkartenlib::fluent::US_ENGLISH);
        let rankname = karte
            .rang
            .name
            .to_locale_string(&spielkartenlib::fluent::US_ENGLISH);
        if suitname == "Major Arcana".to_string() {
            println!("      {}", rankname);
        } else {
            println!("      {} of {} ", rankname, suitname);
        }
    }
}
