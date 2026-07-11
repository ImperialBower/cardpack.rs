use cardpack::preludes::funky::*;

/// End-to-end demonstration of funky (Balatro-style) card scoring.
///
/// It builds a board, plays a hand, detects the poker-hand type, then runs the
/// implemented scoring phases and prints the resulting chips × mult:
///   * **phase 1** — base chips/mult from the hand type at its current level;
///   * **phase 4** — joker contributions;
///   * `board.score()` — the combined (partial) pipeline a solver would call.
///
/// Note: phases 2 (played-card chips) and 3 (held-card effects) are not
/// implemented yet, so `board.score()` currently sums phases 1 and 4 only.
fn main() {
    env_logger::init();

    // Deal a fresh, shuffled Buffoon deck onto a board with 4 hands / 3 discards.
    let draws = Draws::new(4, 3);
    let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile().shuffled());

    // Play a royal flush — it satisfies both the "straight" and "flush"
    // conditions the jokers below care about.
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
    let base = board.scoring_phase1_pre_scoring();
    let jokers = board.scoring_phase4_joker_scoring();
    let total = board.score();

    println!("Played hand : {}", board.played);
    println!("Hand type   : {hand_type:?}");
    println!("Jokers      : {}", board.jokers);
    println!("Phase 1 (base)   {base}");
    println!("Phase 4 (jokers) {jokers}");
    println!("Combined         {total}");
    println!("Final score = {}", total.score());
}
