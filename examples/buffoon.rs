use cardpack::preludes::funky::*;

/// Interesting question on PirateSoftware's stream: [tts q](https://www.twitch.tv/videos/2442976611?t=08h31m48s)
///
/// > DexAMD
/// > Yarr Matey.  Game mechanic thoughts.  If you were to have a debuff that doubles damage from all sources, and you were to receive a DOT, would you rather it...  A: have each tic be double damage and maintain DOT length B: double the length of the DOT but keep the tic damage the same C: have the DOT tic more during the duration
///
/// I need to code ways to show game mechanics.
#[allow(unused_mut, unused_variables)]
fn main() {
    env_logger::init();

    let mut deck = Deck::basic_buffoon_pile().shuffled();

    let hand = bcards!("AS KS QS JS TS");
    let mut score = Score::default();

    // CRAZY = MPip::MultPlusOnStraight(12),
    // DROLL = MPip::MultPlusOnFlush(10)
    let jokers = BuffoonPile::from(vec![bcard!(CRAZY), bcard!(DROLL)]);

    println!("{hand}");
}
