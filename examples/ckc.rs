use cardpack::games::poker::cactus_kev_hand::CactusKevHand;

/// Prints out the number of possible distinct combinations for each `HandRank` type in a
/// Poker Standard 52 French Deck.
fn main() {
    let (rank_class_count, _) = CactusKevHand::all_possible_combos();

    for v in rank_class_count.keys() {
        println!(
            "{} possible {:?} combinations.",
            rank_class_count.get(v).unwrap(),
            v
        );
    }
}
