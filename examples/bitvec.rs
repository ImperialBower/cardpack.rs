use bitvec::prelude::*;

use cardpack::games::poker::cactus_kev::CactusKevCard;

fn main() {
    let mut c: CactusKevCard = CactusKevCard::blank();
    c.bites[..8].store(1u8);

    println!("{:#}", c.bites);
    println!("{:#}", c);
    println!("{}", c.display(true));

    assert_eq!(c.bites.len(), 32);
    println!("{}", c.bites.get(7).unwrap());
}
