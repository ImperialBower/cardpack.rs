use crate::funky::types::draws::Draws;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A **Boss Blind** — the ones this engine models.
///
/// Deliberately a short list rather than Balatro's full roster of ~20. Every
/// entry here is a boss whose ability is a pure [`Draws`] mutation, which means
/// it lands on the round configuration [`BuffoonBoard::on_blind_selected`]
/// already recomputes and needs no new machinery at all.
///
/// The rest of the roster is *not* modelled, and the omission is the point: a
/// boss like The Wall (a larger score requirement) needs a blind score target,
/// and the suit-debuff bosses (The Club, The Goad, …) need debuffs threaded into
/// scoring. Neither exists, and inventing a half of either would be the
/// silently-wrong scoring EPIC-01a is built to avoid. Add a boss here only once
/// its ability is genuinely expressible.
///
/// [`BuffoonBoard::on_blind_selected`]: crate::funky::types::board::BuffoonBoard::on_blind_selected
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum BossBlind {
    /// **The Needle** — play only 1 hand.
    ///
    /// Sets the round's hands to exactly 1, which is why Dusk ("retrigger all
    /// played cards in the final hand of round") always fires under it: the only
    /// hand is the final one.
    #[default]
    TheNeedle,
    /// **The Water** — start with 0 discards.
    ///
    /// Switches Mystic Summit (+15 mult at zero discards) on, and zeroes Banner
    /// (+30 chips per remaining discard).
    TheWater,
    /// **The Manacle** — −1 hand size.
    TheManacle,
}

impl BossBlind {
    /// Apply this boss's ability to a round's [`Draws`].
    ///
    /// Takes and returns `Draws` rather than mutating the board, so it composes
    /// into the recompute-from-baseline pass the draw-modifier jokers already
    /// use and cannot accidentally stack across blinds.
    #[must_use]
    pub fn apply(self, mut draws: Draws) -> Draws {
        match self {
            Self::TheNeedle => draws.hands_to_play = 1,
            Self::TheWater => draws.discards = 0,
            Self::TheManacle => draws.hand_size = draws.hand_size.saturating_sub(1),
        }
        draws
    }
}

impl Display for BossBlind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TheNeedle => write!(f, "The Needle"),
            Self::TheWater => write!(f, "The Water"),
            Self::TheManacle => write!(f, "The Manacle"),
        }
    }
}

/// Which blind a round is being played against.
///
/// The minimum Balatro's blind-reading jokers need: Madness cares only that a
/// blind is *not* a boss, Rocket only that one *is*, and Luchador and Chicot act
/// on the boss's ability.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum Blind {
    #[default]
    Small,
    Big,
    Boss(BossBlind),
}

impl Blind {
    /// Whether this is a Boss Blind — an **identity** question, not an
    /// effectiveness one.
    ///
    /// A boss disabled by Luchador or Chicot is still a Boss Blind: Madness
    /// still refuses to trigger on it, and Rocket still counts it as one
    /// defeated. Whether its *ability* is in force is a separate question, asked
    /// through `BuffoonBoard::boss_ability_active`.
    #[must_use]
    pub fn is_boss(self) -> bool {
        matches!(self, Self::Boss(_))
    }

    /// The boss behind this blind, if it is one.
    #[must_use]
    pub fn boss(self) -> Option<BossBlind> {
        match self {
            Self::Boss(boss) => Some(boss),
            _ => None,
        }
    }
}

impl Display for Blind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "Small Blind"),
            Self::Big => write!(f, "Big Blind"),
            Self::Boss(boss) => write!(f, "{boss}"),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__blind_tests {
    use super::*;

    #[test]
    fn default__is_the_small_blind() {
        assert_eq!(Blind::default(), Blind::Small);
        assert!(!Blind::default().is_boss());
    }

    #[test]
    fn is_boss__separates_identity_from_the_ability() {
        assert!(Blind::Boss(BossBlind::TheNeedle).is_boss());
        assert!(!Blind::Small.is_boss());
        assert!(!Blind::Big.is_boss());
        assert_eq!(
            Blind::Boss(BossBlind::TheWater).boss(),
            Some(BossBlind::TheWater)
        );
        assert_eq!(Blind::Small.boss(), None);
    }

    #[test]
    fn apply__the_needle_leaves_exactly_one_hand() {
        let draws = BossBlind::TheNeedle.apply(Draws::new(4, 3));
        assert_eq!(draws.hands_to_play, 1);
        assert_eq!(draws.discards, 3, "it does not touch discards");
    }

    #[test]
    fn apply__the_water_leaves_no_discards() {
        let draws = BossBlind::TheWater.apply(Draws::new(4, 3));
        assert_eq!(draws.discards, 0);
        assert_eq!(draws.hands_to_play, 4, "it does not touch hands");
    }

    #[test]
    fn apply__the_manacle_shrinks_the_hand_by_one() {
        let draws = BossBlind::TheManacle.apply(Draws::new(4, 3));
        assert_eq!(draws.hand_size, Draws::DEFAULT_HAND_SIZE - 1);

        // A hand size already at 0 floors rather than wrapping.
        let mut tiny = Draws::new(4, 3);
        tiny.hand_size = 0;
        assert_eq!(BossBlind::TheManacle.apply(tiny).hand_size, 0);
    }

    #[test]
    fn display() {
        assert_eq!(Blind::Small.to_string(), "Small Blind");
        assert_eq!(Blind::Big.to_string(), "Big Blind");
        assert_eq!(Blind::Boss(BossBlind::TheNeedle).to_string(), "The Needle");
        assert_eq!(
            Blind::Boss(BossBlind::TheManacle).to_string(),
            "The Manacle"
        );
    }
}
