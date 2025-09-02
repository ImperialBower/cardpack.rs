use crate::funky::types::draws::Draws;
use crate::preludes::funky::{BuffoonPile, PokerHands, Score};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuffoonBoard {
    pub draws: Draws,
    pub deck: BuffoonPile,
    pub in_hand: BuffoonPile,
    pub played: BuffoonPile,
    pub consumables: BuffoonPile,
    pub jokers: BuffoonPile,
    pub poker_hands: PokerHands,
}

impl BuffoonBoard {
    #[must_use]
    pub fn new(draws: Draws, deck: BuffoonPile) -> Self {
        Self {
            draws,
            deck,
            in_hand: BuffoonPile::default(),
            played: BuffoonPile::default(),
            consumables: BuffoonPile::default(),
            jokers: BuffoonPile::default(),
            poker_hands: PokerHands::default(),
        }
    }

    /// From [Detailed Break down of Balatro Scoring System and some tips to optimise your hand scoring.](https://www.reddit.com/r/balatro/comments/1blbexa/detailed_break_down_of_balatro_scoring_system_and/)
    pub fn scoring_phase1_pre_scoring(&self) {
        todo!()
    }

    pub fn scoring_phase2_dealt_hand_scoring(&self) {
        todo!()
    }

    pub fn scoring_phase3_effects_in_hand(&self) {
        todo!()
    }

    #[must_use]
    pub fn scoring_phase4_joker_scoring(&self) -> Score {
        let mut score = Score::default();

        for joker in &self.jokers {
            score += self.played.calculate_plus(joker);
        }

        score
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__board__buffoon_board_tests {
    use super::*;
    use crate::bcards;
    use crate::funky::decks::joker::card;
    use crate::preludes::funky::Deck;

    #[test]
    fn phase_4_joker_scoring() {
        let draws = Draws::new(4, 3);
        let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile());
        board.played = bcards!("AS KD QC JS TH");
        board.jokers.push(card::JOKER);
        board.jokers.push(card::GREEDY_JOKER);
        board.jokers.push(card::LUSTY_JOKER);
        board.jokers.push(card::WRATHFUL_JOKER);
        board.jokers.push(card::GLUTTONOUS_JOKER);

        let score = board.scoring_phase4_joker_scoring();
        assert_eq!(score, Score { chips: 0, mult: 19 });
    }
}
