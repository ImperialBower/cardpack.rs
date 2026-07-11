//! Open effect interpretation for the funky engine.
//!
//! Card effects are **data** (`MPip`), interpreted at scoring time. The built-in
//! effects are matched directly; a custom effect from a mod is identified by
//! `MPip::Custom(u32)` and resolved through an [`EffectRegistry`] the mod
//! populates at startup — so a mod can add scoring behaviour without editing
//! funky source.
//!
//! Because [`BuffoonCard`] is
//! `Copy`, `const`-constructible and `Serialize`, effects on cards cannot be
//! boxed trait objects. The indirection lives in the *registry* (keyed by a
//! plain, serializable `u32`) instead of on the card.

use crate::funky::types::board::BuffoonBoard;
use crate::funky::types::buffoon_card::BuffoonCard;
use crate::funky::types::score::Score;
use std::collections::HashMap;

/// A declarative description of an effect's contribution to a running [`Score`].
///
/// Effects return a `ScoreOp` rather than mutating a `Score` directly, so the
/// scoring pipeline stays in control of ordering (additive vs multiplicative)
/// and the operation is easy to test in isolation.
#[derive(Clone, Debug, PartialEq)]
pub enum ScoreOp {
    /// Contribute nothing.
    Nothing,
    /// Add flat chips.
    AddChips(usize),
    /// Add flat mult.
    AddMult(usize),
    /// Add both chips and mult (a whole [`Score`] contribution).
    Add(Score),
    /// Multiply the running mult by a factor (e.g. `2.0` for ×2).
    TimesMult(f32),
    /// Apply several operations in order.
    Seq(Vec<Self>),
}

impl ScoreOp {
    /// Folds this operation into `score`, returning the updated score.
    #[must_use]
    pub fn apply(&self, score: Score) -> Score {
        match self {
            Self::Nothing => score,
            Self::AddChips(chips) => score + Score::new(*chips, 0),
            Self::AddMult(mult) => score + Score::new(0, *mult),
            Self::Add(delta) => score + *delta,
            Self::TimesMult(factor) => score.multi_mult(*factor),
            Self::Seq(ops) => ops.iter().fold(score, |acc, op| op.apply(acc)),
        }
    }
}

/// Everything a custom [`Effect`] can read about the board while scoring: the
/// full board (played/held/joker piles, leveled hands) and the specific card or
/// joker that carries the effect.
pub struct ScoringContext<'a> {
    /// The board being scored.
    pub board: &'a BuffoonBoard,
    /// The card (or joker) whose effect is firing.
    pub source: BuffoonCard,
}

/// A custom, moddable scoring effect. Implement this in a mod crate and register
/// it under a `u32` id; attach the effect to a card via `MPip::Custom(id)`.
///
/// Kept object-safe so the registry can hold `Box<dyn Effect>`.
pub trait Effect {
    /// The contribution this effect makes given the current scoring context.
    fn score(&self, ctx: &ScoringContext<'_>) -> ScoreOp;
}

/// Maps `MPip::Custom` ids to their handlers. A mod builds one at startup and
/// passes it to [`BuffoonBoard::score_with_registry`].
///
/// [`BuffoonBoard::score_with_registry`]: crate::funky::types::board::BuffoonBoard::score_with_registry
#[derive(Default)]
pub struct EffectRegistry {
    handlers: HashMap<u32, Box<dyn Effect>>,
}

impl EffectRegistry {
    /// A registry with no effects.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers `effect` under `id`, replacing any existing handler for it.
    pub fn register(&mut self, id: u32, effect: impl Effect + 'static) {
        self.handlers.insert(id, Box::new(effect));
    }

    /// Looks up the handler for `id`, if any.
    #[must_use]
    pub fn get(&self, id: u32) -> Option<&dyn Effect> {
        self.handlers.get(&id).map(|effect| &**effect)
    }

    /// Whether an effect is registered for `id`.
    #[must_use]
    pub fn contains(&self, id: u32) -> bool {
        self.handlers.contains_key(&id)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__effect_tests {
    use super::*;

    #[test]
    fn score_op__apply() {
        let base = Score::new(10, 4);
        assert_eq!(ScoreOp::Nothing.apply(base), Score::new(10, 4));
        assert_eq!(ScoreOp::AddChips(5).apply(base), Score::new(15, 4));
        assert_eq!(ScoreOp::AddMult(3).apply(base), Score::new(10, 7));
        assert_eq!(
            ScoreOp::Add(Score::new(5, 3)).apply(base),
            Score::new(15, 7)
        );
        assert_eq!(ScoreOp::TimesMult(2.0).apply(base), Score::new(10, 8));
    }

    #[test]
    fn score_op__seq_applies_in_order() {
        // +2 mult then ×3 -> (4+2) * 3 = 18; the reverse would be 4*3 + 2 = 14.
        let op = ScoreOp::Seq(vec![ScoreOp::AddMult(2), ScoreOp::TimesMult(3.0)]);
        assert_eq!(op.apply(Score::new(0, 4)), Score::new(0, 18));
    }

    struct FlatMult(usize);
    impl Effect for FlatMult {
        fn score(&self, _ctx: &ScoringContext<'_>) -> ScoreOp {
            ScoreOp::AddMult(self.0)
        }
    }

    #[test]
    fn registry__register_and_get() {
        let mut registry = EffectRegistry::new();
        assert!(!registry.contains(1));
        registry.register(1, FlatMult(7));
        assert!(registry.contains(1));
        assert!(registry.get(2).is_none());

        // The handler resolves; exercising it needs a ScoringContext, which the
        // board-level tests cover end to end.
        assert!(registry.get(1).is_some());
    }
}
