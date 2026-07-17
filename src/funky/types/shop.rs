use crate::funky::types::buffoon_card::BuffoonCard;
use serde::{Deserialize, Serialize};

/// Which family of cards a [`BoosterPack`] opens into.
///
/// The three base packs whose contents this engine can already draw: a Buffoon
/// pack offers jokers, an Arcana pack tarots, a Celestial pack planets. The
/// Standard (playing-card) and Spectral packs are out of scope — playing-card
/// shop stock is voucher-gated, and spectral cards do not exist (EPIC-01
/// Story 3).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum PackKind {
    /// Choose one of two jokers, rarity-rolled.
    Buffoon,
    /// Choose one of three tarots.
    Arcana,
    /// Choose one of three planets.
    Celestial,
}

/// A booster pack on offer in the [`Shop`] — a `kind` and what it costs to open.
///
/// `cost` is a plain field rather than a constant so the Jumbo ($6) and Mega
/// ($8) tiers are data, not new mechanics; this engine stocks the base tier at
/// $4.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BoosterPack {
    pub kind: PackKind,
    pub cost: usize,
}

/// The between-rounds **shop** — the seam money is spent through.
///
/// Held on the board as an `Option<Shop>`: `None` is a closed shop (the state
/// every board is in until [`open_shop_with_rng`] draws one), so a run that
/// never shops is byte-identical to a pre-shop board. Opening it draws the
/// **card slots** and **pack slots** from the rarity piles and decks at
/// Balatro's weights; buying and pack-choosing route through the same
/// joker/consumable seams selling already uses.
///
/// [`open_shop_with_rng`]: crate::funky::types::board::BuffoonBoard::open_shop_with_rng
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Shop {
    /// The card slots on offer this shop — jokers, tarots, and planets drawn at
    /// the wiki weights. A bought slot is removed, so the `Vec` shrinks as the
    /// player spends; it never refills except by a reroll.
    pub stock: Vec<BuffoonCard>,
    /// The booster-pack slots on offer this shop. Each is bought-and-opened or
    /// skipped, then removed; a reroll leaves them alone (it redraws only
    /// `stock`).
    pub packs: Vec<BoosterPack>,
    /// How many times the card slots have been rerolled this shop. Base 0, reset
    /// each time a shop opens; read by the escalating reroll cost.
    pub rerolls_used: usize,
}

impl Shop {
    /// A shop stocked with exactly `stock`, no packs, and no rerolls spent — the
    /// deterministic constructor tests and callers use to skip the random draw.
    #[must_use]
    pub fn with_stock(stock: Vec<BuffoonCard>) -> Self {
        Self {
            stock,
            packs: Vec::new(),
            rerolls_used: 0,
        }
    }
}
