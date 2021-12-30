use bitvec::field::BitField;
use bitvec::prelude::{BitVec, Msb0};
use cardpack::games::poker::bit_card::{AnnotatedBitCard, BitCard};
use cardpack::games::poker::cactus_kev_card::ckc;
use cardpack::Standard52;

fn main() {
    let king_spades: BitCard = BitCard::from_index("KS").unwrap();

    let r = king_spades.as_bitarray().load::<u64>();

    let bv: BitVec<Msb0, u64> = king_spades.as_bitarray().iter().collect();

    println!(">> {}", bv);
    let ks = Standard52::card_from_index("KS");

    let bvv = bv.load::<u64>();
    println!(">> {}", bvv);

    println!("{}", king_spades);
    println!("{}", r);
    println!("{}", ckc::from_card(&ks));

    let asd = king_spades.get_rank_bitslice().load::<u64>();
    println!(">>> {}", asd);

    println!("{:#}", AnnotatedBitCard::new(king_spades));

    let standard52 = Standard52::default();
    for card in standard52.deck {
        let bs: BitCard = BitCard::from_card(&card);
        let bss = bs.as_bitarray().load::<u64>();
        let rbs = bs.get_rank_bitslice().load::<u64>();
        println!("{} {} {}", card, rbs, bss);
    }

    // let standard52 = Standard52::default();
    // for card in standard52.deck {
    //     card.debug();
    // }
}
