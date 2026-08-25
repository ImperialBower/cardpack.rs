//! A two-party provably-fair shuffle, end to end.
//!
//! Run with `cargo ex provably_fair` or
//! `cargo run --features commit-reveal,std --example provably_fair`.
//!
//! The dealer and one player each commit to secret entropy, then reveal.
//! The combined seed fixes the shuffle. Everything printed under
//! "Transcript" is what a third party needs to check it.

use cardpack::prelude::*;

fn main() -> Result<(), CardError> {
    let dealer = ParticipantId(1);
    let player = ParticipantId(2);
    let mut rng = rand::rng();

    // Phase A: commit. Contributions stay secret; only commitments travel.
    let dealer_secret = Contribution::random(&mut rng);
    let player_secret = Contribution::random(&mut rng);

    let mut round = ShuffleRound::new([dealer, player])?;
    round.commit(dealer, dealer_secret.commit())?;
    round.commit(player, player_secret.commit())?;

    // A reveal before every commitment is in would be rejected:
    // `round.reveal(...)` returns `RevealBeforeAllCommitted`.

    // Phase B: reveal. Order does not matter; each reveal is checked against
    // its commitment.
    round.reveal(player, player_secret)?;
    round.reveal(dealer, dealer_secret)?;

    let seed = round.seed()?;
    let deck = Standard52::deck();
    let shuffled = deck.shuffled_by_round(&round)?;

    println!("Transcript (public):");
    for &id in round.participants() {
        println!(
            "  participant {id}: commitment {}  contribution {}",
            round.commitment(id).unwrap(),
            hex(round.contribution(id).unwrap().as_bytes())
        );
    }
    println!("  combined seed: {seed}");
    println!();
    println!(
        "Derived permutation: {:?}",
        seed.permutation(deck.len())?.as_slice()
    );
    println!("Shuffled deck: {shuffled}");
    println!();

    // A verifier rebuilds the round from the transcript alone.
    let mut verifier = ShuffleRound::new([dealer, player])?;
    for &id in round.participants() {
        verifier.commit(id, round.commitment(id).unwrap())?;
    }
    for &id in round.participants() {
        verifier.reveal(id, round.contribution(id).unwrap())?;
    }
    let check = deck.shuffled_by_round(&verifier)?;
    println!("Verifier agrees: {}", check == shuffled);

    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}
