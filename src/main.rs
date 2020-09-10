use spielkartenlib::ToLocaleString;

fn main() {
    println!("spielkarten.rs demo\n");

    let french_deck = spielkartenlib::Kartendeck::french_deck();
    let pinochle_deck = spielkartenlib::Kartendeck::pinochle_deck();

    println!("French Deck:");
    demo(french_deck);

    println!("Pinochle Deck:");
    demo(pinochle_deck);

    println!("Tarot Deck");
    demo_tarot_deck();
}

fn demo(deck: spielkartenlib::Kartendeck) {
    print!("   Short With Symbols:           ");
    for (_, karte) in deck.karten.iter().enumerate() {
        print!("{} ", karte);
    }

    println!();
    print!("   Short With Symbols in German: ");
    for (_, karte) in deck.karten.iter().enumerate() {
        print!(
            "{} ",
            karte.to_locale_string(&spielkartenlib::fluent::GERMAN)
        );
    }

    println!();
    print!("   Short With Letters:           ");
    for (_, karte) in deck.karten.iter().enumerate() {
        print!(
            "{} ",
            karte.to_txt_string(&spielkartenlib::fluent::US_ENGLISH)
        );
    }

    println!();
    print!("   Short With Letters in German: ");
    for (_, karte) in deck.karten.iter().enumerate() {
        print!("{} ", karte.to_txt_string(&spielkartenlib::fluent::GERMAN));
    }

    println!();
    print!("   Shuffle Deck:                 ");
    for (_, karte) in deck.shuffle().iter().enumerate() {
        print!(
            "{} ",
            karte.to_locale_string(&spielkartenlib::fluent::US_ENGLISH)
        );
    }

    println!();
    print!("   Long in English and German:\n");
    for (_, karte) in deck.karten.iter().enumerate() {
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
    let deck = spielkartenlib::Kartendeck::tarot_deck();
    let cards = deck.shuffle();
    for (_, karte) in cards.iter().enumerate() {
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
