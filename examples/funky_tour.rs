//! # Features
//!
//! Uses `funky` (the Balatro-style engine, `cardpack::preludes::funky`).
//! cardpack is pure by default (`default = []`), so to use these APIs in your
//! own crate enable it explicitly:
//! `cardpack = { version = "0.9", features = ["funky"] }`
//! — `funky` implies `std` + `serde`, since every funky type derives both.
//!
//! Run it from this repo with `cargo ex funky_tour` — the alias in `.cargo/config.toml`
//! supplies the features, so no `--features` flag is needed.

use cardpack::preludes::funky::*;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// A guided tour of the funky (Balatro-style) engine — the features closed out
/// by EPIC-01 and its children, one act per subsystem:
///
///   * **Act 1** — the round loop: deal, play, cash out (EPIC-01a).
///   * **Act 2** — four-phase scoring, then editions folding in (EPIC-01d).
///   * **Act 3** — the shop: stock, buying, rerolls, the voucher slot
///     (EPIC-01b, EPIC-01c).
///   * **Act 4** — spectral cards: Black Hole and Hex (EPIC-01e).
///
/// Every random draw is seeded, so the tour prints the same run every time —
/// the same reproducibility a solver relies on.
fn main() {
    act_1_round_loop();
    act_2_scoring_and_editions();
    act_3_shop_and_vouchers();
    act_4_spectrals();
}

fn banner(title: &str) {
    println!("\n=== {title} ===");
}

/// Deal a seeded deck, play hands until the round resolves, then cash out.
fn act_1_round_loop() {
    banner("Act 1 — the round loop (EPIC-01a)");

    let deck = Deck::basic_buffoon_pile().shuffled_with_seed(42);
    let mut board = BuffoonBoard::new(Draws::new(4, 3), deck);
    board.blind_target = 1; // any scoring hand beats the blind

    board.on_blind_selected();
    let dealt = board.deal_to_hand_size();
    println!("dealt {dealt} cards: {}", board.in_hand);

    if let Some(score) = board.play_hand(&[0, 1, 2, 3, 4]) {
        println!("played the first five: {score}");
    }
    println!(
        "round_score {} vs target {} — won: {}",
        board.round_score,
        board.blind_target,
        board.round_is_won()
    );

    let before = board.money;
    board.on_round_end();
    println!(
        "cash-out: ${before} -> ${} (blind reward + $1 per unused hand + interest)",
        board.money
    );
}

/// The fixed royal-flush board from `examples/buffoon.rs`, then the same board
/// with an edition stamped on the joker.
fn act_2_scoring_and_editions() {
    banner("Act 2 — four-phase scoring + editions (EPIC-01d)");

    let plain = score_royal_flush_with(bcard!(DROLL));
    println!("royal flush + plain Droll (+10 mult on flush): {plain}");

    for edition in [Edition::Foil, Edition::Holographic, Edition::Polychrome] {
        let scored = score_royal_flush_with(bcard!(DROLL).with_edition(edition));
        println!("same board, {edition} Droll: {scored}");
    }
}

fn score_royal_flush_with(joker: BuffoonCard) -> Score {
    let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
    board.played = bcards!("AS KS QS JS TS");
    board.jokers.push(joker);
    board.score()
}

/// Open a seeded shop, buy the first slot, price a reroll, redeem the voucher.
fn act_3_shop_and_vouchers() {
    banner("Act 3 — the shop & vouchers (EPIC-01b, EPIC-01c)");

    let mut rng = StdRng::seed_from_u64(7);
    let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
    board.money = 25;

    board.open_shop_with_rng(&mut rng);
    if let Some(shop) = &board.shop {
        for (index, card) in shop.stock.iter().enumerate() {
            println!("stock slot {index}: {card}");
        }
        for pack in &shop.packs {
            println!("pack on offer: {:?} (${})", pack.kind, pack.cost);
        }
        if let Some(voucher) = shop.voucher {
            println!("voucher on offer: {voucher:?} ($10)");
        }
    }

    let bought = board.buy_stock(0);
    println!("buy slot 0 -> {bought}; money now ${}", board.money);
    println!("next reroll costs ${}", board.reroll_cost());
    let rerolled = board.reroll_with_rng(&mut rng);
    println!("rerolled -> {rerolled}; money now ${}", board.money);
    let redeemed = board.redeem_shop_voucher();
    println!(
        "redeem voucher -> {redeemed}; money ${}; vouchers held: {:?}",
        board.money, board.vouchers
    );
}

/// Black Hole levels every poker hand; Hex leaves one Polychrome joker standing.
fn act_4_spectrals() {
    banner("Act 4 — spectral cards (EPIC-01e)");

    let mut rng = StdRng::seed_from_u64(99);
    let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
    board.jokers.push(bcard!(CRAZY));
    board.jokers.push(bcard!(DROLL));
    board.jokers.push(bcard!(DEVIOUS));

    let before = board.poker_hands.get(&HandType::Flush).copied();
    let created = board.create_consumable(BLACK_HOLE);
    let used = board.use_consumable(0, &[]).is_some();
    let after = board.poker_hands.get(&HandType::Flush).copied();
    if let (Some(before), Some(after)) = (before, after) {
        println!(
            "Black Hole (created {created}, used {used}): Flush level {} ({}x{}) -> level {} ({}x{})",
            before.level, before.chips, before.mult, after.level, after.chips, after.mult
        );
    }

    println!("jokers before Hex: {}", board.jokers);
    let created = board.create_consumable(HEX);
    let used = board.use_consumable_with_rng(0, &[], &mut rng).is_some();
    println!(
        "Hex (created {created}, used {used}): jokers after: {}",
        board.jokers
    );
    for joker in &board.jokers {
        println!("  survivor edition: {}", joker.edition);
    }
}
