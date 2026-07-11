use crate::funky::types::draws::Draws;
use crate::preludes::funky::{BuffoonPile, HandType, MPip, PokerHands, Score};
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
            consumables: BuffoonPile::new_with_capacity(2),
            jokers: BuffoonPile::new_with_capacity(5),
            poker_hands: PokerHands::default(),
        }
    }

    /// Phase 1 — pre-scoring: establishes the base chips and mult for the
    /// played hand from its [`HandType`] at its current level, as tracked in
    /// [`PokerHands`]. This is the starting [`Score`] before any played-card
    /// (phase 2), held-card (phase 3), or joker (phase 4) contributions.
    ///
    /// A Royal Flush shares the Straight Flush's base and level, matching
    /// Balatro (there is no separate Royal Flush entry to level up).
    ///
    /// From [Detailed Break down of Balatro Scoring System and some tips to optimise your hand scoring.](https://www.reddit.com/r/balatro/comments/1blbexa/detailed_break_down_of_balatro_scoring_system_and/)
    #[must_use]
    pub fn scoring_phase1_pre_scoring(&self) -> Score {
        let hand_type = match self.played.determine_hand_type() {
            HandType::RoyalFlush => HandType::StraightFlush,
            other => other,
        };

        self.poker_hands
            .get(&hand_type)
            .map_or_else(Score::default, |hand| Score::new(hand.chips, hand.mult))
    }

    /// Phase 2 — played-hand scoring: each played card contributes its chip
    /// value (base rank + flat `Chips` enhancement, via
    /// [`BuffoonCard::get_chips`]) plus any per-card plus-effects driven by its
    /// own enhancement (conditional chips / mult, via
    /// [`BuffoonCard::calculate_plus`]). Those two paths handle disjoint `MPip`
    /// variants, so nothing is counted twice.
    ///
    /// [`BuffoonCard::get_chips`]: crate::funky::types::buffoon_card::BuffoonCard::get_chips
    /// [`BuffoonCard::calculate_plus`]: crate::funky::types::buffoon_card::BuffoonCard::calculate_plus
    #[must_use]
    pub fn scoring_phase2_dealt_hand_scoring(&self) -> Score {
        let mut score = Score::default();

        for card in &self.played {
            score.chips += card.get_chips();
            score += card.calculate_plus(card);
        }

        score
    }

    /// Phase 3 — held-card effects: applies the ×mult contributions of cards
    /// held **in hand** (not played) to the running `score`. The canonical case
    /// is a Steel card (`MPip::STEEL` = `MultTimes1Dot(15)` = ×1.5 mult while
    /// held); `MultTimes(n)` gives a flat ×n.
    ///
    /// Unlike the additive phases, held effects multiply, so this takes the
    /// score accumulated so far (phases 1 + 2) and returns it transformed. With
    /// no held cards it is the identity.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn scoring_phase3_effects_in_hand(&self, running: Score) -> Score {
        let mut score = running;

        for card in &self.in_hand {
            match card.enhancement {
                MPip::MultTimes1Dot(n) => score = score.multi_mult(n as f32 / 10.0),
                MPip::MultTimes(n) => score = score.multi_mult(n as f32),
                _ => {}
            }
        }

        score
    }

    #[must_use]
    pub fn scoring_phase4_joker_scoring(&self) -> Score {
        let mut score = Score::default();

        for joker in &self.jokers {
            score += self.played.calculate_plus(joker);
        }

        score
    }

    /// Combined score for the currently played hand — the full four-phase
    /// pipeline, in Balatro order:
    ///
    /// 1. base hand chips/mult,
    /// 2. played-card chips,
    /// 3. held-card ×mult (Steel, …),
    /// 4. joker contributions.
    ///
    /// The final chips × mult is `score().score()`. This never panics, so a
    /// solver can call it for any board.
    ///
    /// NOTE: joker scoring (phase 4) is still additive-only; multiplicative
    /// jokers (`MultTimes`) are not applied yet, and the ~54 unhandled `MPip`
    /// variants still fall through to zero.
    #[must_use]
    pub fn score(&self) -> Score {
        let base_and_cards =
            self.scoring_phase1_pre_scoring() + self.scoring_phase2_dealt_hand_scoring();

        self.scoring_phase3_effects_in_hand(base_and_cards) + self.scoring_phase4_joker_scoring()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__board__buffoon_board_tests {
    use super::*;
    use crate::bcards;
    use crate::funky::decks::basic::card as basic;
    use crate::funky::decks::joker::card;
    use crate::funky::decks::planet;
    use crate::funky::types::mpip::MPip;
    use crate::preludes::funky::{BuffoonCard, Deck};

    #[test]
    fn phase_4_joker_scoring_basic1_5() {
        let draws = Draws::new(4, 3);
        let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile());
        board.played = bcards!("AS KD QC JS TH");
        board.jokers.push(card::JOKER);
        board.jokers.push(card::GREEDY_JOKER);
        board.jokers.push(card::LUSTY_JOKER);
        board.jokers.push(card::WRATHFUL_JOKER);
        board.jokers.push(card::GLUTTONOUS_JOKER);
        board.jokers.push(card::GLUTTONOUS_JOKER);
        board.jokers.push(card::JOLLY_JOKER);

        let score = board.scoring_phase4_joker_scoring();
        assert_eq!(score, Score { chips: 0, mult: 22 });
    }

    #[test]
    fn phase_4_joker_scoring_basic6_8_11_13() {
        let draws = Draws::new(4, 3);
        let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile());
        board.played = bcards!("AS AD AC JS JH");
        board.jokers.push(card::GLUTTONOUS_JOKER); // Does nothing
        board.jokers.push(card::JOLLY_JOKER);
        board.jokers.push(card::ZANY_JOKER);
        board.jokers.push(card::MAD_JOKER);
        board.jokers.push(card::SLY_JOKER);
        board.jokers.push(card::WILY_JOKER);
        board.jokers.push(card::CLEVER_JOKER);

        let score = board.scoring_phase4_joker_scoring();
        assert_eq!(
            score,
            Score {
                chips: 230,
                mult: 33
            }
        );
    }

    #[test]
    fn phase_4_joker_scoring_basic9_10_14_15() {
        let draws = Draws::new(4, 3);
        let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile());
        board.played = bcards!("AH KH QH JH TH");
        board.jokers.push(card::GLUTTONOUS_JOKER); // Does nothing
        board.jokers.push(card::CRAZY_JOKER);
        board.jokers.push(card::DROLL_JOKER);
        board.jokers.push(card::DEVIOUS_JOKER);
        board.jokers.push(card::CRAFTY_JOKER);

        let score = board.scoring_phase4_joker_scoring();
        assert_eq!(
            score,
            Score {
                chips: 180,
                mult: 22
            }
        );
    }

    #[test]
    fn phase_4_joker_scoring_basic16() {
        let draws = Draws::new(4, 3);
        let mut board = BuffoonBoard::new(draws, Deck::basic_buffoon_pile());
        board.played = bcards!("AH KH QH");
        board.jokers.push(card::HALF_JOKER);

        let score = board.scoring_phase4_joker_scoring();
        assert_eq!(score, Score { chips: 0, mult: 20 });
    }

    fn board_playing(index: &str) -> BuffoonBoard {
        let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        board.played = bcards!(index);
        board
    }

    #[test]
    fn phase_1_pre_scoring__high_card() {
        // 2,5,8,T,K — no pair, straight, or flush.
        let board = board_playing("2S 5D 8C TS KH");
        assert_eq!(
            board.played.determine_hand_type(),
            HandType::HighCard,
            "test fixture must be a high card"
        );
        assert_eq!(
            board.scoring_phase1_pre_scoring(),
            Score { chips: 5, mult: 1 }
        );
    }

    #[test]
    fn phase_1_pre_scoring__pair() {
        let board = board_playing("AS AD QC JS TH");
        assert_eq!(board.played.determine_hand_type(), HandType::Pair);
        assert_eq!(
            board.scoring_phase1_pre_scoring(),
            Score { chips: 10, mult: 2 }
        );
    }

    #[test]
    fn phase_1_pre_scoring__royal_flush_uses_straight_flush_base() {
        let board = board_playing("AS KS QS JS TS");
        assert_eq!(board.played.determine_hand_type(), HandType::RoyalFlush);
        // Royal Flush has no table entry of its own; it borrows Straight
        // Flush's base (100 chips, 8 mult).
        assert_eq!(
            board.scoring_phase1_pre_scoring(),
            Score {
                chips: 100,
                mult: 8
            }
        );
    }

    #[test]
    fn phase_1_pre_scoring__reflects_planet_leveling() {
        let mut board = board_playing("2S 5D 8C TS KH");
        // Pluto levels High Card: +10 chips, +1 mult (5/1 -> 15/2).
        board.poker_hands.increment(planet::card::PLUTO);
        assert_eq!(
            board.scoring_phase1_pre_scoring(),
            Score { chips: 15, mult: 2 }
        );
    }

    #[test]
    fn phase_2_dealt_hand__plain_cards_sum_rank_chips() {
        // A=11, K/Q/J/T=10 -> 51 chips, no enhancements so no mult.
        let board = board_playing("AS KS QS JS TS");
        assert_eq!(
            board.scoring_phase2_dealt_hand_scoring(),
            Score { chips: 51, mult: 0 }
        );
    }

    #[test]
    fn phase_2_dealt_hand__pair_of_aces() {
        // 11 + 11 + 10 + 10 + 10 = 52.
        let board = board_playing("AS AD QC JS TH");
        assert_eq!(
            board.scoring_phase2_dealt_hand_scoring(),
            Score { chips: 52, mult: 0 }
        );
    }

    fn enhanced(card: BuffoonCard, enhancement: MPip) -> BuffoonCard {
        BuffoonCard {
            enhancement,
            ..card
        }
    }

    #[test]
    fn phase_2_dealt_hand__chips_enhancement_adds_flat_chips() {
        // A "Bonus"-style card: +30 flat chips on top of the ace's 11.
        let mut board = board_playing("KS");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::Chips(30))]);
        assert_eq!(
            board.scoring_phase2_dealt_hand_scoring(),
            Score { chips: 41, mult: 0 }
        );
    }

    #[test]
    fn phase_2_dealt_hand__mult_enhancement_adds_mult() {
        // A "Mult"-style card: +4 mult on top of the ace's 11 chips.
        let mut board = board_playing("KS");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::MultPlus(4))]);
        assert_eq!(
            board.scoring_phase2_dealt_hand_scoring(),
            Score { chips: 11, mult: 4 }
        );
    }

    #[test]
    fn phase_3_effects_in_hand__no_held_cards_is_identity() {
        let board = board_playing("AS KS QS JS TS");
        let running = Score::new(151, 8);
        assert_eq!(board.scoring_phase3_effects_in_hand(running), running);
    }

    #[test]
    fn phase_3_effects_in_hand__one_steel_card_times_1_5() {
        let mut board = board_playing("AS KS QS JS TS");
        board.in_hand = BuffoonPile::from(vec![enhanced(basic::KING_HEARTS, MPip::STEEL)]);
        // 8 mult x 1.5 = 12; chips untouched.
        assert_eq!(
            board.scoring_phase3_effects_in_hand(Score::new(151, 8)),
            Score {
                chips: 151,
                mult: 12
            }
        );
    }

    #[test]
    fn phase_3_effects_in_hand__two_steel_cards_compound() {
        let mut board = board_playing("AS KS QS JS TS");
        board.in_hand = BuffoonPile::from(vec![
            enhanced(basic::KING_HEARTS, MPip::STEEL),
            enhanced(basic::QUEEN_HEARTS, MPip::STEEL),
        ]);
        // 8 -> 12 -> 18.
        assert_eq!(
            board.scoring_phase3_effects_in_hand(Score::new(151, 8)),
            Score {
                chips: 151,
                mult: 18
            }
        );
    }

    #[test]
    fn score__combines_base_cards_and_jokers_end_to_end() {
        let mut board = board_playing("AH KH QH JH TH");
        board.jokers.push(card::CRAZY_JOKER); // +12 mult on straight
        board.jokers.push(card::DROLL_JOKER); // +10 mult on flush
        board.jokers.push(card::DEVIOUS_JOKER); // +100 chips on straight
        board.jokers.push(card::CRAFTY_JOKER); // +80 chips on flush

        // Phase 1 base (Royal -> Straight Flush): 100 chips, 8 mult.
        // Phase 2 played cards: 51 chips, 0 mult.
        // Phase 3 held cards: none -> identity.
        // Phase 4 jokers: +180 chips, +22 mult.
        // Combined: 331 chips x 30 mult = 9930.
        let score = board.score();
        assert_eq!(
            score,
            Score {
                chips: 331,
                mult: 30
            }
        );
        assert_eq!(score.score(), 9930);
    }

    #[test]
    fn score__held_steel_multiplies_mult_end_to_end() {
        let mut board = board_playing("AS KS QS JS TS");
        board.in_hand = BuffoonPile::from(vec![enhanced(basic::KING_HEARTS, MPip::STEEL)]);

        // Phase 1 + 2: 151 chips, 8 mult. Phase 3: steel x1.5 -> 12 mult.
        // No jokers. Final: 151 x 12 = 1812.
        let score = board.score();
        assert_eq!(
            score,
            Score {
                chips: 151,
                mult: 12
            }
        );
        assert_eq!(score.score(), 1812);
    }
}
