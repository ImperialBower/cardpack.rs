use bitvec::prelude::*;

use cardpack::games::poker::cactus_kev::{CactusKev, CactusKevCard};

fn main() {
    let mut card: CactusKev = BitArray::zeroed();
    card[..8].store(1u8);

    let c = CactusKevCard::new(&card);

    println!("{:#}", card);
    println!("{:#}", c);
    println!("{}", c.dump());

    assert_eq!(card.len(), 32);
    println!("{}", card.get(7).unwrap());
}
