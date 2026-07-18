use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A **voucher** — a redeemed-once, run-permanent modifier the shop sells from
/// its $10 slot.
///
/// A dedicated enum rather than a [`BuffoonCard`] + [`MPip`], because a voucher
/// is not a card: it is never held in a pile, never sold, and never scores. It
/// modifies *board configuration* — the round's draws, the slot limits, the
/// shop's prices and weights — which is the [`BossBlind`] shape (its own enum),
/// not the joker shape (a card carrying a scoring effect).
///
/// The variants come in **base → upgrade** pairs: an upgrade
/// ([`requires`](Self::requires) returns its base) is only offered and only
/// redeemable once its base is already held. The per-voucher *effects* land in
/// later phases of EPIC-01c; this type and the redeem path are Phase 1.
///
/// [`BuffoonCard`]: crate::funky::types::buffoon_card::BuffoonCard
/// [`MPip`]: crate::funky::types::mpip::MPip
/// [`BossBlind`]: crate::funky::types::blind::BossBlind
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Voucher {
    /// +1 hand per round.
    Grabber,
    /// +1 hand per round, on top of Grabber.
    NachoTong,
    /// +1 discard per round.
    Wasteful,
    /// +1 discard per round, on top of Wasteful.
    Recyclomancy,
    /// +1 hand size.
    PaintBrush,
    /// +1 hand size, on top of Paint Brush.
    Palette,
    /// Shop card slots 2 → 3.
    Overstock,
    /// Shop card slots 3 → 4.
    OverstockPlus,
    /// +1 consumable slot.
    CrystalBall,
    /// +1 joker slot.
    ///
    /// Balatro gates Antimatter behind the Blank voucher (which does nothing);
    /// Blank is out of scope here, so Antimatter is a base voucher with no
    /// prerequisite. The elision is invisible — Blank had no effect to lose.
    Antimatter,
    /// Rerolls cost $2 less.
    RerollSurplus,
    /// Rerolls cost $4 less (on top of Surplus's $2, so $2 more).
    RerollGlut,
    /// All shop cards and packs 25% off.
    ClearanceSale,
    /// All shop cards and packs 50% off.
    Liquidation,
    /// Interest cap $5 → $10.
    SeedMoney,
    /// Interest cap $10 → $20.
    MoneyTree,
    /// Tarots appear twice as often in the shop.
    TarotMerchant,
    /// Tarots appear four times as often in the shop.
    TarotTycoon,
    /// Planets appear twice as often in the shop.
    PlanetMerchant,
    /// Planets appear four times as often in the shop.
    PlanetTycoon,
}

impl Voucher {
    /// Every voucher this engine models, in tier order — the pool the shop draws
    /// its slot from.
    pub const ALL: [Self; 20] = [
        Self::Grabber,
        Self::NachoTong,
        Self::Wasteful,
        Self::Recyclomancy,
        Self::PaintBrush,
        Self::Palette,
        Self::Overstock,
        Self::OverstockPlus,
        Self::CrystalBall,
        Self::Antimatter,
        Self::RerollSurplus,
        Self::RerollGlut,
        Self::ClearanceSale,
        Self::Liquidation,
        Self::SeedMoney,
        Self::MoneyTree,
        Self::TarotMerchant,
        Self::TarotTycoon,
        Self::PlanetMerchant,
        Self::PlanetTycoon,
    ];

    /// The base voucher this one requires, or `None` if it is itself a base.
    ///
    /// This is the whole base → upgrade prerequisite, as data: the shop only
    /// offers an upgrade once its base is held, and redeeming enforces the same.
    #[must_use]
    pub fn requires(self) -> Option<Self> {
        match self {
            Self::NachoTong => Some(Self::Grabber),
            Self::Recyclomancy => Some(Self::Wasteful),
            Self::Palette => Some(Self::PaintBrush),
            Self::OverstockPlus => Some(Self::Overstock),
            Self::RerollGlut => Some(Self::RerollSurplus),
            Self::Liquidation => Some(Self::ClearanceSale),
            Self::MoneyTree => Some(Self::SeedMoney),
            Self::TarotTycoon => Some(Self::TarotMerchant),
            Self::PlanetTycoon => Some(Self::PlanetMerchant),
            _ => None,
        }
    }
}

impl Display for Voucher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Grabber => "Grabber",
            Self::NachoTong => "Nacho Tong",
            Self::Wasteful => "Wasteful",
            Self::Recyclomancy => "Recyclomancy",
            Self::PaintBrush => "Paint Brush",
            Self::Palette => "Palette",
            Self::Overstock => "Overstock",
            Self::OverstockPlus => "Overstock Plus",
            Self::CrystalBall => "Crystal Ball",
            Self::Antimatter => "Antimatter",
            Self::RerollSurplus => "Reroll Surplus",
            Self::RerollGlut => "Reroll Glut",
            Self::ClearanceSale => "Clearance Sale",
            Self::Liquidation => "Liquidation",
            Self::SeedMoney => "Seed Money",
            Self::MoneyTree => "Money Tree",
            Self::TarotMerchant => "Tarot Merchant",
            Self::TarotTycoon => "Tarot Tycoon",
            Self::PlanetMerchant => "Planet Merchant",
            Self::PlanetTycoon => "Planet Tycoon",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__voucher_tests {
    use super::*;

    #[test]
    fn all__lists_every_variant_once() {
        // ALL is the draw pool; a variant missing from it could never be
        // offered. Uniqueness guards against a copy-paste duplicate.
        let mut seen = Voucher::ALL.to_vec();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), Voucher::ALL.len(), "no duplicates in ALL");
        assert_eq!(Voucher::ALL.len(), 20);
    }

    #[test]
    fn requires__pairs_each_upgrade_with_its_base() {
        assert_eq!(Voucher::NachoTong.requires(), Some(Voucher::Grabber));
        assert_eq!(Voucher::MoneyTree.requires(), Some(Voucher::SeedMoney));
        assert_eq!(
            Voucher::PlanetTycoon.requires(),
            Some(Voucher::PlanetMerchant)
        );
        // Bases have no prerequisite.
        assert_eq!(Voucher::Grabber.requires(), None);
        assert_eq!(Voucher::CrystalBall.requires(), None);
        assert_eq!(Voucher::Antimatter.requires(), None);
    }

    #[test]
    fn requires__every_required_base_is_itself_a_base() {
        // A prerequisite chain is exactly one deep — an upgrade's base is never
        // itself an upgrade, so holding the base is always reachable.
        for voucher in Voucher::ALL {
            if let Some(base) = voucher.requires() {
                assert_eq!(base.requires(), None, "{voucher}'s base {base} is a base");
            }
        }
    }

    #[test]
    fn display__reads_as_the_wiki_name() {
        assert_eq!(Voucher::NachoTong.to_string(), "Nacho Tong");
        assert_eq!(Voucher::OverstockPlus.to_string(), "Overstock Plus");
        assert_eq!(Voucher::CrystalBall.to_string(), "Crystal Ball");
    }
}
