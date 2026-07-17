use crate::funky::types::buffoon_card::BuffoonCard;
use serde::{Deserialize, Serialize};

/// The between-rounds **shop** — the seam money is spent through.
///
/// Held on the board as an `Option<Shop>`: `None` is a closed shop (the state
/// every board is in until [`open_shop_with_rng`] draws one), so a run that
/// never shops is byte-identical to a pre-shop board. Opening it draws the
/// **card slots** from the rarity piles at Balatro's weights; buying routes each
/// slot through the same joker/consumable seams selling already uses.
///
/// Booster **packs** are a later phase and are deliberately absent here — this
/// is the smallest shop a round loop can spend money in.
///
/// [`open_shop_with_rng`]: crate::funky::types::board::BuffoonBoard::open_shop_with_rng
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Shop {
    /// The card slots on offer this shop — jokers, tarots, and planets drawn at
    /// the wiki weights. A bought slot is removed, so the `Vec` shrinks as the
    /// player spends; it never refills except by a reroll.
    pub stock: Vec<BuffoonCard>,
    /// How many times the card slots have been rerolled this shop. Base 0, reset
    /// each time a shop opens; read by the escalating reroll cost (Phase 3).
    pub rerolls_used: usize,
}

impl Shop {
    /// A shop stocked with exactly `stock` and no rerolls spent — the deterministic
    /// constructor tests and callers use to skip the random draw.
    #[must_use]
    pub fn with_stock(stock: Vec<BuffoonCard>) -> Self {
        Self {
            stock,
            rerolls_used: 0,
        }
    }
}
