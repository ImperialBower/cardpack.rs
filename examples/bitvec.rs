use cardpack::games::poker::bit_card::{AnnotatedBitCard, BitCard};

fn main() {
    let king_spades: BitCard = BitCard::from_index("KS").unwrap();
    println!("{}", king_spades);

    println!("{:#}", AnnotatedBitCard::new(king_spades));
}
