use crate::funky::types::effect::ScoreOp;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A Balatro **edition** — the foil/holographic/polychrome/negative overlay a
/// card or joker can wear, *orthogonal* to its enhancement (a card can be Steel
/// **and** Foil at once).
///
/// A dedicated field on [`BuffoonCard`], not an [`MPip`] variant, precisely
/// because it is orthogonal: an edition and an enhancement coexist, so they
/// cannot share the one `enhancement` slot. Three of the four are pure scoring
/// and resolve to a [`ScoreOp`] the fold already applies; Negative is a slot
/// rule with no score.
///
/// [`BuffoonCard`]: crate::funky::types::buffoon_card::BuffoonCard
/// [`MPip`]: crate::funky::types::mpip::MPip
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Edition {
    /// The unedited default.
    #[default]
    None,
    /// +50 chips when the bearer scores.
    Foil,
    /// +10 mult when the bearer scores.
    Holographic,
    /// ×1.5 mult when the bearer scores.
    Polychrome,
    /// The bearer takes **no slot** (joker or consumable). Not a scoring effect,
    /// and never on a playing card.
    Negative,
}

impl Edition {
    /// The scoring contribution of a **bearer that is scoring** — a played card
    /// or a joker. `None` and `Negative` contribute nothing (`Negative` is a
    /// slot rule, read elsewhere).
    ///
    /// Polychrome's `×1.5` rides [`ScoreOp::TimesMult`], which routes to
    /// `Score::multi_mult` — the same ceil-based path `MultTimes1Dot` uses — so
    /// the factor composes at the bearer's position in the fold, not as a global
    /// post-multiply.
    #[must_use]
    pub fn score_op(self) -> ScoreOp {
        match self {
            Self::Foil => ScoreOp::AddChips(50),
            Self::Holographic => ScoreOp::AddMult(10),
            Self::Polychrome => ScoreOp::TimesMult(1.5),
            Self::None | Self::Negative => ScoreOp::Nothing,
        }
    }

    /// Whether this edition exempts its bearer from a slot limit.
    #[must_use]
    pub fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }
}

impl Display for Edition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::None => "None",
            Self::Foil => "Foil",
            Self::Holographic => "Holographic",
            Self::Polychrome => "Polychrome",
            Self::Negative => "Negative",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__edition_tests {
    use super::*;
    use crate::funky::types::score::Score;

    #[test]
    fn default__is_none() {
        assert_eq!(Edition::default(), Edition::None);
    }

    #[test]
    fn score_op__foil_adds_fifty_chips() {
        assert_eq!(
            Edition::Foil.score_op().apply(Score::new(10, 2)),
            Score::new(60, 2)
        );
    }

    #[test]
    fn score_op__holographic_adds_ten_mult() {
        assert_eq!(
            Edition::Holographic.score_op().apply(Score::new(10, 2)),
            Score::new(10, 12)
        );
    }

    #[test]
    fn score_op__polychrome_times_one_and_a_half_mult() {
        // multi_mult ceils: 10 mult × 1.5 = 15.
        assert_eq!(
            Edition::Polychrome.score_op().apply(Score::new(100, 10)),
            Score::new(100, 15)
        );
    }

    #[test]
    fn score_op__none_and_negative_are_inert() {
        let base = Score::new(100, 8);
        assert_eq!(Edition::None.score_op().apply(base), base);
        assert_eq!(Edition::Negative.score_op().apply(base), base);
    }

    #[test]
    fn is_negative__only_negative() {
        assert!(Edition::Negative.is_negative());
        for edition in [
            Edition::None,
            Edition::Foil,
            Edition::Holographic,
            Edition::Polychrome,
        ] {
            assert!(!edition.is_negative());
        }
    }
}
