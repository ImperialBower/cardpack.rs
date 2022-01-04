use cardpack::games::poker::cactus_kev_cards::CactusKevCards;
use cardpack::Standard52;

fn main() {
    let mut deck = Standard52::new_shuffled();

    let first_hand = CactusKevCards::deal5(&mut deck)
        .to_cactus_kev_hand()
        .unwrap();
    let second_hand = CactusKevCards::deal5(&mut deck)
        .to_cactus_kev_hand()
        .unwrap();

    if first_hand.eval() > second_hand.eval() {
        println!("{} beats {}", first_hand, second_hand);
    } else {
        println!("{} is beaten by {}", first_hand, second_hand);
    }
}
