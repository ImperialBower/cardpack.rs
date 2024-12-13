use cardpack::US_ENGLISH;

fn main() {
    let standard52 = cardpack::Standard52::default();

    let mut pile = standard52.deck.shuffle();

    let card = pile.draw_first().unwrap();

    println!("{} {}", card.symbol(&US_ENGLISH), card.symbol_colorized(&US_ENGLISH));

}