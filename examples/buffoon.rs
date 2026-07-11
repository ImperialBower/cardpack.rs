use cardpack::preludes::funky::*;

/// End-to-end demonstration of funky (Balatro-style) card scoring.
///
/// It builds a board, plays a hand, detects the poker-hand type, and then runs
/// the implemented **phase-4 joker-scoring** pass, printing the resulting
/// chips × mult.
///
/// Note: board scoring phases 1–3 (`scoring_phase1_pre_scoring` …
/// `scoring_phase3_effects_in_hand`) are not implemented yet and will `panic!`
/// via `todo!()`, so this example deliberately exercises only the working
/// phase-4 joker contribution.
fn main() {
    env_logger::init();

    // Deal a fresh, shuffled Buffoon deck onto a board with 4 hands / 3 discards.
    let draws = Draws::new(4, 3);
    let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile().shuffled());

    // Play a royal flush — it satisfies both the "straight" and "flush"
    // conditions the two jokers below care about.
    board.played = bcards!("AS KS QS JS TS");

    // Four jokers whose effects are wired and scored today. A royal flush is
    // both a straight and a flush, so all four fire:
    //   CRAZY   = MPip::MultPlusOnStraight(12)
    //   DROLL   = MPip::MultPlusOnFlush(10)
    //   DEVIOUS = MPip::ChipsOnStraight(100)
    //   CRAFTY  = MPip::ChipsOnFlush(80)
    board.jokers.push(bcard!(CRAZY));
    board.jokers.push(bcard!(DROLL));
    board.jokers.push(bcard!(DEVIOUS));
    board.jokers.push(bcard!(CRAFTY));

    let hand_type = board.played.determine_hand_type();
    let score = board.scoring_phase4_joker_scoring();

    println!("Played hand : {}", board.played);
    println!("Hand type   : {hand_type:?}");
    println!("Jokers      : {}", board.jokers);
    println!("Phase-4 {score}");
    println!("Total joker chips × mult = {}", score.score());
}
