use bitvec::prelude::*;

use cardpack::games::poker::cactus_kev::CactusKevCard;

fn main() {
    let mut c: CactusKevCard = CactusKevCard::blank();
    c.card[..8].store(1u8);

    println!("{:#}", c.card);
    println!("{:#}", c);
    println!("{}", c.dump());

    assert_eq!(c.card.len(), 32);
    println!("{}", c.card.get(7).unwrap());
}
