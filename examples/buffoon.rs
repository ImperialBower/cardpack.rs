use cardpack::preludes::funky::*;

fn main() {
    let mut deck = Deck::basic_buffoon_pile().shuffled();

    let hand = bcards!("AS KS QS JS TS");
    let mut score = Score::default();

    let jokers = BuffoonPile::from(vec![bcard!(CRAZY), bcard!(DROLL)]);

    println!("{hand}");
}
