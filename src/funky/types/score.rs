use crate::preludes::funky::BuffoonPile;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct Score {
    pub chips: usize,
    pub mult: usize,
}

impl Score {
    #[must_use]
    pub fn new(chips: usize, mult: usize) -> Self {
        Self { chips, mult }
    }

    #[must_use]
    pub fn add_chips(&self, chips: usize) -> Self {
        let chips = self.chips + chips;
        Self {
            chips,
            mult: self.mult,
        }
    }

    #[must_use]
    pub fn add_mult(&mut self, mult: usize) -> Self {
        let mult = self.mult + mult;
        Self {
            chips: self.chips,
            mult,
        }
    }

    #[must_use]
    pub fn current(&self) -> usize {
        self.chips * self.mult
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__score_tests {
    use super::*;

    /// This is an example of a test that I would normally skip.
    #[test]
    fn new() {
        assert_eq!(Score::new(1, 2).current(), 2);
        assert_eq!(Score::new(2, 3).current(), 6);
    }

    #[test]
    fn add_chips() {
        assert_eq!(Score::new(1, 2).add_chips(1).current(), 4);
        assert_eq!(Score::new(2, 3).add_chips(1).current(), 9);
    }

    /// I love it when the AI suggests something that is basically wrong. The original
    /// second test had the result as 12.
    #[test]
    fn add_mult() {
        assert_eq!(Score::new(1, 2).add_mult(1).current(), 3);
        assert_eq!(Score::new(2, 3).add_mult(1).current(), 8);
    }

    #[test]
    fn default() {
        assert_eq!(Score::default().current(), 0);
    }
}
