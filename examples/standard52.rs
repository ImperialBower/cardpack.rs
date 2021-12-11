fn main() {
    println!("Let's shuffle a Standard 52 deck of cards:\n");
    let pack = cardpack::Pack::french_deck();

    let shuffled = pack.cards().shuffle();
    let index_str = shuffled.to_index();
    println!("{}", index_str);
}
