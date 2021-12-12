fn main() {
    println!("Let's create a Standard 52 deck of cards:");
    let standard52 = cardpack::Standard52::default();
    println!("{}\n", standard52.to_index());

    println!("Let's display it with its symbol index:");
    println!("{}\n", standard52.to_symbol_index());

    println!("Let's create a shuffled Standard 52 deck of cards:");
    let standard52 = cardpack::Standard52::new_shuffled();
    println!("{}\n", standard52);

    println!("Let's display it with its symbol index:");
    println!("{}\n", standard52.to_symbol_index());
}
