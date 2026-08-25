//! The holder flow, end to end: a trusted dealer seals a shuffled deck, a
//! player draws two slots and gets two tokens, turns one card up by publishing
//! its token, and a spectator with no secret verifies it.
//!
//! Run with `cargo ex holder_seal` or
//! `cargo run --features seal-aead,std --example holder_seal`.

use cardpack::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let context = b"table-7/hand-12";

    // ── Dealer ────────────────────────────────────────────────────────────
    let dealer = HolderKeySeal::<Standard52>::dealer(DealKey::random(&mut rng), context);
    let (mut shoe, custody) = dealer.deal(&Standard52::deck(), &mut rng)?;
    println!(
        "Dealer sealed {} cards. Custody is public ciphertext:",
        custody.len()
    );
    for (slot, sealed) in custody.iter().take(3) {
        println!("  slot {slot}: {sealed:?}");
    }
    println!("  …");

    // Player draws two slots. Nobody — not even the shoe — knows the values.
    let hole = shoe.draw(2).ok_or("shoe empty")?;
    let tokens = dealer.tokens_for(hole.slots().iter().copied())?;
    println!(
        "\nPlayer holds slots {:?} and two tokens (secret).",
        hole.slots()
    );

    // ── Holder turns one card up ──────────────────────────────────────────
    let (slot, token) = (tokens[0].0, tokens[0].1.clone());
    let published: [u8; 32] = token.to_bytes();
    println!(
        "\nPlayer publishes slot {slot} + token {}…",
        hex(&published[..4])
    );

    // ── Spectator verifies, holding no secret ─────────────────────────────
    let spectator = HolderKeySeal::<Standard52>::verifier(context);
    let mut revealed = Revealed::<Standard52>::new();
    let card = revealed.reveal_with(
        slot,
        custody.get(slot).ok_or("slot not in custody")?,
        &spectator,
        &CardKey::from_bytes(published),
    )?;
    println!("Spectator verified slot {slot} = {card}");
    println!(
        "Slot {} is still sealed: {}",
        hole.slots()[1],
        !revealed.is_revealed(hole.slots()[1])
    );

    // The published token opens nothing else.
    let other = hole.slots()[1];
    let attempt = spectator.unseal(
        custody.get(other).ok_or("slot")?,
        other,
        &CardKey::from_bytes(published),
    );
    println!("Same token on slot {other}: {attempt:?}");

    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}
