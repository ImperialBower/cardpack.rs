fn main() {
    println!("Let's shuffle a Standard 52 deck of cards:\n");
    let standard52 = cardpack::Standard52::new_shuffled();

    println!("{}", standard52);
}
