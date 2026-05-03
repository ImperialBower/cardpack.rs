//! Criterion benchmarks for cardpack's hot deck/pile operations.
//!
//! Runs against `Pile<Canasta>` because Canasta is the largest shipped
//! deck (108 cards). Smaller decks are cheaper than these numbers; the
//! relative deltas across releases are what matters for regression
//! detection.
//!
//! ## Cases
//!
//! - `shuffled` — non-deterministic shuffle (calls `rand::rng()`)
//! - `shuffled_with_seed` — deterministic shuffle (no thread-RNG)
//! - `draw_1` — single card from a 108-card deck
//! - `draw_13` — bridge-hand-sized draw
//! - `pile_on_8` — concatenate eight 13-card piles (poker-table sized)
//! - `combos_5_of_7` — enumerate the 21 5-card combos from a 7-card
//!   pile (the workload `examples/poker_eval.rs` runs per player)
//!
//! Run with: `cargo bench --bench draw`

use cardpack::basic::types::traits::Ranged;
use cardpack::prelude::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_shuffled(c: &mut Criterion) {
    let deck = Canasta::deck();
    c.bench_function("shuffled (canasta, 108 cards)", |b| {
        b.iter(|| black_box(&deck).shuffled());
    });
}

fn bench_shuffled_with_seed(c: &mut Criterion) {
    let deck = Canasta::deck();
    c.bench_function("shuffled_with_seed (canasta, 108 cards)", |b| {
        b.iter(|| black_box(&deck).shuffled_with_seed(black_box(0xC0FFEE)));
    });
}

fn bench_draw_one(c: &mut Criterion) {
    let template = Canasta::deck();
    c.bench_function("draw(1) from 108-card deck", |b| {
        b.iter_batched(
            || template.clone(),
            |mut deck| {
                let _ = deck.draw(black_box(1));
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_draw_thirteen(c: &mut Criterion) {
    let template = Canasta::deck();
    c.bench_function("draw(13) from 108-card deck", |b| {
        b.iter_batched(
            || template.clone(),
            |mut deck| {
                let _ = deck.draw(black_box(13));
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_pile_on_eight(c: &mut Criterion) {
    let mut deck = Canasta::deck().shuffled_with_seed(0);
    let piles: Vec<_> = (0..8).map(|_| deck.draw(13).unwrap()).collect();
    c.bench_function("pile_on(8 piles of 13)", |b| {
        b.iter(|| Pile::<Canasta>::pile_on(black_box(&piles)));
    });
}

fn bench_combos_5_of_7(c: &mut Criterion) {
    // Mirror the per-player workload in examples/poker_eval.rs:
    // enumerate all 21 5-card combinations from a 7-card pile.
    let mut deck = Standard52::deck().shuffled_with_seed(0);
    let seven = deck.draw(7).unwrap();
    c.bench_function("combos(5) of 7-card pile", |b| {
        b.iter(|| black_box(&seven).combos(black_box(5)));
    });
}

criterion_group!(
    benches,
    bench_shuffled,
    bench_shuffled_with_seed,
    bench_draw_one,
    bench_draw_thirteen,
    bench_pile_on_eight,
    bench_combos_5_of_7,
);
criterion_main!(benches);
