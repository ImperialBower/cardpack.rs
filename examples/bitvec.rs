use bitvec::field::BitField;
use bitvec::prelude::{BitVec, Lsb0};
use cardpack::games::poker::bit_card::{AnnotatedBitCard, BitCard};
use cardpack::Standard52;

fn main() {
    let king_spades: BitCard = BitCard::from_index("KS").unwrap();

    let r = king_spades.as_bitarray().load::<u64>();

    let bv: BitVec<Lsb0, u64> = king_spades.as_bitarray().iter().collect();

    println!(">> {}", bv);
    let ks = Standard52::card_from_index("KS");

    let bvv = bv.load::<u64>();
    println!(">> {}", bvv);

    println!("{}", king_spades);
    println!("{}", r);
    println!("{}", ks.binary_signature());

    let asd = king_spades.get_rank_bitslice().load::<u64>();
    println!(">>> {}", asd);

    println!("{:#}", AnnotatedBitCard::new(king_spades));
}
