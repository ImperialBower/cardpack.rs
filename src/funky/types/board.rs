use crate::funky::types::draws::Draws;
use crate::funky::types::effect::{EffectRegistry, ScoreOp, ScoringContext};
use crate::preludes::funky::{
    BCardType, BuffoonCard, BuffoonPile, HandRules, HandType, MPip, PokerHands, Score,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Balatro's Lucky card grants a flat +20 mult on a successful (1-in-N) roll.
const LUCKY_MULT: usize = 20;

/// An in-round event that can grow a joker's counter.
enum GrowthEvent<'a> {
    HandPlayed(&'a BuffoonPile),
    Discard(&'a BuffoonPile),
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuffoonBoard {
    pub draws: Draws,
    pub deck: BuffoonPile,
    pub in_hand: BuffoonPile,
    pub played: BuffoonPile,
    pub consumables: BuffoonPile,
    pub jokers: BuffoonPile,
    pub poker_hands: PokerHands,
    /// Money the run currently holds. Signed so Credit Card can carry debt to
    /// -$20. Read by scoring jokers (Bull); written by `+$` jokers, the shop,
    /// and interest once those lifecycle events land. Inert by default (0).
    pub money: isize,
    /// One accumulator per joker, index-aligned with `jokers`: `joker_state[i]`
    /// belongs to `jokers[i]`. Signed because Green Joker's net (hands −
    /// discards) can dip negative before the read floors it at 0. Grown by the
    /// event hooks, read (never written) during scoring.
    pub joker_state: Vec<i32>,
    /// Every card the run **owns** — Balatro's "full deck".
    ///
    /// This is a stable roster, not a location: a card stays in `full_deck`
    /// once drawn, played, discarded, or held. Contrast [`deck`](Self::deck),
    /// which is the *undealt remainder* and shrinks as cards are drawn (Blue
    /// Joker reads that one). Modelling the roster separately is what Balatro
    /// does, and it keeps the "in full deck" jokers (Steel Joker, Stone Joker,
    /// Erosion) correct without requiring the deal to be conserved across the
    /// location piles.
    ///
    /// Seeded from the deck at [`new`](Self::new); only deck **mutation**
    /// (adding or destroying cards) should change it.
    pub full_deck: BuffoonPile,
    /// How big [`full_deck`](Self::full_deck) was when the run started.
    ///
    /// Recorded rather than assumed to be 52, since alternate decks start at
    /// other sizes. Erosion scores the shortfall against this.
    pub starting_deck_size: usize,
}

impl BuffoonBoard {
    #[must_use]
    pub fn new(draws: Draws, deck: BuffoonPile) -> Self {
        // At construction the run owns exactly the deck it was handed, so the
        // roster and the undealt remainder start out equal.
        let full_deck = deck.clone();
        let starting_deck_size = full_deck.len();
        Self {
            draws,
            deck,
            full_deck,
            starting_deck_size,
            in_hand: BuffoonPile::default(),
            played: BuffoonPile::default(),
            consumables: BuffoonPile::new_with_capacity(2),
            jokers: BuffoonPile::new_with_capacity(5),
            poker_hands: PokerHands::default(),
            money: 0,
            joker_state: Vec::new(),
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
        let hand_type = match self.played.determine_hand_type_with(self.hand_rules()) {
            HandType::RoyalFlush => HandType::StraightFlush,
            other => other,
        };

        self.poker_hands
            .get(&hand_type)
            .map_or_else(Score::default, |hand| Score::new(hand.chips, hand.mult))
    }

    /// Phase 2 — played-hand scoring: folds each played card into the running
    /// `score`. Every card adds its chip value (base rank + flat `Chips`
    /// enhancement, via [`BuffoonCard::get_chips`]) plus any per-card plus-effects
    /// from its own enhancement (conditional chips / mult, via
    /// [`BuffoonCard::calculate_plus`]); those two paths handle disjoint `MPip`
    /// variants, so nothing is counted twice.
    ///
    /// Takes the running score (rather than returning an independent
    /// contribution) so a custom played card's ×mult multiplies the score
    /// accumulated so far — including the phase-1 base — in card order.
    ///
    /// [`BuffoonCard::get_chips`]: crate::funky::types::buffoon_card::BuffoonCard::get_chips
    /// [`BuffoonCard::calculate_plus`]: crate::funky::types::buffoon_card::BuffoonCard::calculate_plus
    #[must_use]
    pub fn scoring_phase2_dealt_hand_scoring(&self, running: Score) -> Score {
        self.fold_played_cards::<StdRng>(running, None, None)
    }

    /// The single played-card fold behind phase 2. Each card adds its chips and
    /// built-in additive effects, then its special enhancement resolves:
    /// a Lucky card rolls (if `rng`), a Glass card applies its ×mult, a
    /// `MPip::Custom` card is looked up (if `registry`). Built-in cards are
    /// unaffected by the options.
    // Glass's ×mult factor is a small literal from the card data; the same
    // allow the other ×mult seams carry (`builtin_held_op`, `joker_x_mult`).
    #[allow(clippy::cast_precision_loss)]
    fn fold_played_cards<R: Rng + ?Sized>(
        &self,
        running: Score,
        mut rng: Option<&mut R>,
        registry: Option<&EffectRegistry>,
    ) -> Score {
        let mut score = running;

        for (index, card) in self.played.iter().enumerate() {
            // A card is scored once, plus once more for each retrigger a joker
            // grants it (Hack: each played 2-5; Hanging Chad: the first card).
            // Retriggering re-runs the whole per-card contribution, so a
            // retriggered Lucky card rolls again — matching Balatro. With no
            // retrigger joker this is a single pass.
            for _ in 0..=self.played_retriggers(index, card) {
                // Every played card contributes its built-in chips/mult first,
                // then its special (probabilistic / custom) effect resolves.
                score = Self::builtin_played_op(card).apply(score);

                let special = match card.enhancement {
                    MPip::Lucky(mult_odds, _) if mult_odds > 0 => {
                        // A 1-in-`mult_odds` roll wins on outcomes `0..wins`;
                        // Oops! All 6s doubles `wins` (capped at certainty).
                        let wins = self.probability_numerator().min(mult_odds);
                        rng.as_deref_mut().map_or(ScoreOp::Nothing, |rng| {
                            if rng.random_range(0..mult_odds) < wins {
                                ScoreOp::AddMult(LUCKY_MULT)
                            } else {
                                ScoreOp::Nothing
                            }
                        })
                    }
                    // Glass card: ×n mult when scored. Multiplicative, so it
                    // cannot ride the additive `calculate_plus` path — it scales
                    // the running score at this card's position, like a held
                    // Steel card does in phase 3. The destruction half
                    // (1-in-`_odds` after the hand) is data only until a
                    // round-end hook exists, exactly as with Gros Michel.
                    MPip::Glass(mult, _odds) => ScoreOp::TimesMult(mult as f32),
                    MPip::Custom(id) => self.custom_op(*card, id, registry),
                    _ => ScoreOp::Nothing,
                };
                score = special.apply(score);
            }
        }

        score
    }

    /// How many *additional* times the played card at `index` is scored, summed
    /// over the board's retrigger jokers. 0 for a board with none (the common
    /// case), so the played-card fold is byte-identical when no retrigger joker
    /// is held. `index` is the card's position in `self.played`, used by
    /// position-based retriggers (Hanging Chad fires only on the first card).
    fn played_retriggers(&self, index: usize, card: &BuffoonCard) -> usize {
        self.jokers
            .iter()
            .map(|joker| match joker.enhancement {
                MPip::RetriggerPlayedRanks(n, ranks) if ranks.contains(&card.rank.index) => n,
                MPip::RetriggerPlayedFaces(n) if self.is_face_card(card) => n,
                MPip::RetriggerFirstPlayed(n) if index == 0 => n,
                _ => 0,
            })
            .sum()
    }

    /// Built-in played-card contribution: base rank chips (+ flat `Chips`) plus
    /// the card's own additive plus-effects, as one additive [`ScoreOp`].
    fn builtin_played_op(card: &BuffoonCard) -> ScoreOp {
        ScoreOp::Add(Score::new(card.get_chips(), 0) + card.calculate_plus(card))
    }

    /// Resolves a `MPip::Custom(id)` card/joker to the [`ScoreOp`] its registered
    /// [`Effect`](crate::funky::types::effect::Effect) returns, or
    /// [`ScoreOp::Nothing`] if there is no registry entry. Shared by all three
    /// phase folds.
    fn custom_op(
        &self,
        source: BuffoonCard,
        id: u32,
        registry: Option<&EffectRegistry>,
    ) -> ScoreOp {
        registry
            .and_then(|r| r.get(id))
            .map_or(ScoreOp::Nothing, |effect| {
                let ctx = ScoringContext {
                    board: self,
                    source,
                };
                effect.score(&ctx)
            })
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
    pub fn scoring_phase3_effects_in_hand(&self, running: Score) -> Score {
        self.fold_held_cards(running, None)
    }

    /// The single held-card fold behind phase 3. Built-in Steel/`MultTimes`
    /// cards apply their ×mult; a `MPip::Custom` held card is resolved through
    /// `registry` (if any).
    fn fold_held_cards(&self, running: Score, registry: Option<&EffectRegistry>) -> Score {
        let mut score = running;

        // Mime retriggers held-card abilities: each held card's op applies
        // `1 + held_retriggers` times, so a retriggered Steel card gives ×1.5
        // twice. 0 for a board with no held-retrigger joker (the common case),
        // leaving the held fold byte-identical.
        let retriggers = self.held_retriggers();
        for card in &self.in_hand {
            let op = match card.enhancement {
                MPip::Custom(id) => self.custom_op(*card, id, registry),
                _ => Self::builtin_held_op(card),
            };
            for _ in 0..=retriggers {
                score = op.apply(score);
            }
        }

        score
    }

    /// How many *additional* times every held card's ability fires, summed over
    /// the board's held-retrigger jokers (Mime). Unlike `played_retriggers` this
    /// is card-independent — Mime retriggers all held cards alike.
    fn held_retriggers(&self) -> usize {
        self.jokers
            .iter()
            .map(|joker| match joker.enhancement {
                MPip::RetriggerCardsInHand(n) => n,
                _ => 0,
            })
            .sum()
    }

    /// Built-in held-card contribution: Steel / `MultTimes` give a ×mult, as a
    /// [`ScoreOp`]; anything else is inert while held.
    #[allow(clippy::cast_precision_loss)]
    fn builtin_held_op(card: &BuffoonCard) -> ScoreOp {
        match card.enhancement {
            MPip::MultTimes1Dot(n) => ScoreOp::TimesMult(n as f32 / 10.0),
            MPip::MultTimes(n) => ScoreOp::TimesMult(n as f32),
            _ => ScoreOp::Nothing,
        }
    }

    /// Phase 4 — joker scoring: applies each joker to the running `score`, left
    /// to right. Order matters: a `+mult` joker followed by a `×mult` joker
    /// scores differently than the reverse, so jokers must be folded into one
    /// running score rather than summed independently.
    ///
    /// Additive jokers contribute via [`BuffoonPile::calculate_plus`] (chips /
    /// +mult); multiplicative jokers scale the running mult — unconditional
    /// (`MultTimes(n)` = ×n, `MultTimes1Dot(n)` = ×n/10) or hand-conditional
    /// (`MultTimesOn{Pair,Trips,4OfAKind,Straight,Flush}` — The Duo/Trio/Family/
    /// Order/Tribe — which fire when the played hand *contains* that category).
    ///
    /// Note: state-dependent ×mult jokers (`MultTimesOnEmptyJokerSlots`,
    /// `MultTimesEveryXHands`) are not applied yet — they need board/round
    /// state and their exact factors pinned down.
    ///
    /// [`BuffoonPile::calculate_plus`]: crate::funky::types::buffoon_pile::BuffoonPile::calculate_plus
    #[must_use]
    pub fn scoring_phase4_joker_scoring(&self, running: Score) -> Score {
        self.fold_jokers::<StdRng>(running, None, None)
    }

    /// The single joker-scoring fold that every phase-4 entry point delegates
    /// to. Applies each joker to the running score left-to-right:
    ///
    /// * `MPip::Custom(id)` — resolved through `registry` (if any), else inert;
    /// * `MPip::MultPlusRandomTo(n)` — rolled with `rng` (if any), else inert;
    /// * multiplicative built-ins ([`joker_x_mult`](Self::joker_x_mult)) — ×mult;
    /// * everything else — additive ([`BuffoonPile::calculate_plus`]).
    ///
    /// The `rng`/`registry` options are what distinguish the pure, seeded, and
    /// registry entry points — the fold itself lives here once.
    ///
    /// [`BuffoonPile::calculate_plus`]: crate::funky::types::buffoon_pile::BuffoonPile::calculate_plus
    fn fold_jokers<R: Rng + ?Sized>(
        &self,
        running: Score,
        mut rng: Option<&mut R>,
        registry: Option<&EffectRegistry>,
    ) -> Score {
        let mut score = running;

        for (index, joker) in self.jokers.iter().enumerate() {
            let op = match joker.enhancement {
                MPip::Custom(id) => self.custom_op(*joker, id, registry),
                MPip::MultPlusRandomTo(n) if n > 0 => {
                    rng.as_deref_mut().map_or(ScoreOp::Nothing, |rng| {
                        ScoreOp::AddMult(rng.random_range(0..n))
                    })
                }
                _ => {
                    let counter = self.joker_state.get(index).copied().unwrap_or(0);
                    self.counter_joker_op(joker, counter)
                        .unwrap_or_else(|| self.builtin_joker_op(joker))
                }
            };
            score = op.apply(score);
        }

        score
    }

    /// Built-in joker contribution as a [`ScoreOp`]: a ×mult for a (satisfied)
    /// multiplicative joker, otherwise its additive chips/mult.
    fn builtin_joker_op(&self, joker: &BuffoonCard) -> ScoreOp {
        // Board-reading additive jokers (a pure function of the current board).
        match joker.enhancement {
            MPip::MultPlusPerJoker(n) => return ScoreOp::AddMult(n * self.jokers.len()),
            MPip::ChipsPerDeckCard(n) => return ScoreOp::AddChips(n * self.deck.len()),
            // Stone Joker: +n chips per Stone card in the full deck.
            MPip::ChipsPerFullDeckStone(n) => {
                return ScoreOp::AddChips(n * self.full_deck_stone_count());
            }
            // Erosion: +n mult per card destroyed from the starting deck.
            MPip::MultPlusPerMissingDeckCard(n) => {
                return ScoreOp::AddMult(n * self.cards_missing_from_deck());
            }
            MPip::ChipsPlusPerScoredFace(n) => {
                let faces = self
                    .played
                    .iter()
                    .filter(|card| self.is_face_card(card))
                    .count();
                return ScoreOp::AddChips(n * faces);
            }
            MPip::ChipsMultPlusPerScoredRanks(chips, mult, ranks) => {
                let count = self
                    .played
                    .iter()
                    .filter(|card| ranks.contains(&card.rank.index))
                    .count();
                return ScoreOp::Add(Score::new(chips * count, mult * count));
            }
            // Banner: +n chips for each remaining discard (reads round state).
            MPip::ChipsPerRemainingDiscard(n) => {
                return ScoreOp::AddChips(n * self.draws.discards);
            }
            // Mystic Summit: +n mult only when no discards remain, else inert.
            MPip::MultPlusOnZeroDiscards(n) => {
                return if self.draws.discards == 0 {
                    ScoreOp::AddMult(n)
                } else {
                    ScoreOp::Nothing
                };
            }
            // Bull: +n chips per $1 held; debt (negative money) scores nothing.
            MPip::ChipsPerDollar(n) => {
                let dollars = usize::try_from(self.money).unwrap_or(0);
                return ScoreOp::AddChips(n * dollars);
            }
            // Gros Michel: +n mult unconditionally. The destruction half of the
            // variant is inert here — it rolls at end of round, which has no
            // hook yet — but the mult is not conditional on it and scores now.
            MPip::MultPlusChanceDestroyed(n, _, _) => return ScoreOp::AddMult(n),
            // Scholar: +chips and +mult for each played card of the given rank;
            // compounds with the count (+20 chips, +4 mult per played Ace).
            MPip::MultPlusChipsOnRank(mult, chips, rank) => {
                let count = self
                    .played
                    .iter()
                    .filter(|card| card.rank.index == rank)
                    .count();
                return ScoreOp::Add(Score::new(chips * count, mult * count));
            }
            // Raised Fist: +n × the lowest-ranked held card's value to mult
            // (nothing when the hand is empty).
            MPip::MultPlusXOnLowestRankInHand(n) => {
                let lowest = self
                    .in_hand
                    .iter()
                    .map(|card| card.rank.value)
                    .min()
                    .unwrap_or(0);
                return ScoreOp::AddMult(n * lowest);
            }
            _ => {}
        }

        self.joker_x_mult(joker).map_or_else(
            || ScoreOp::Add(self.played.calculate_plus(joker)),
            ScoreOp::TimesMult,
        )
    }

    /// The straight/flush detection rules in force for this board, loosened by
    /// its rule-modifier jokers: **Four Fingers** drops straights and flushes to
    /// four cards; **Shortcut** allows one-gap straights. Vanilla Balatro
    /// ([`HandRules::default`]) when neither is held, so hand typing is
    /// unchanged. Multiple modifiers stack (Four Fingers + Shortcut → a
    /// four-card gapped straight).
    fn hand_rules(&self) -> HandRules {
        let mut rules = HandRules::default();
        for joker in &self.jokers {
            match joker.enhancement {
                MPip::FourFlushAndStraight => {
                    rules.straight_connectors = 3;
                    rules.flush_len = 4;
                }
                MPip::GappedStraight => rules.straight_distance = 2,
                MPip::SmearedSuits => rules.smeared = true,
                _ => {}
            }
        }
        rules
    }

    /// Whether `card` counts as a face card for the face-reading jokers. Kings,
    /// Queens and Jacks always do; **Pareidolia** makes *every* card a face.
    fn is_face_card(&self, card: &BuffoonCard) -> bool {
        self.all_cards_are_faces() || matches!(card.rank.index, 'K' | 'Q' | 'J')
    }

    /// Whether Pareidolia is on the board (every card is treated as a face).
    fn all_cards_are_faces(&self) -> bool {
        self.jokers
            .iter()
            .any(|joker| matches!(joker.enhancement, MPip::AllCardsAreFaces))
    }

    /// How many cards in the run's full deck carry a Steel enhancement.
    ///
    /// Steel is modelled as `MultTimes1Dot`, matching what
    /// [`builtin_held_op`](Self::builtin_held_op) treats as Steel while held.
    fn full_deck_steel_count(&self) -> usize {
        self.full_deck
            .iter()
            .filter(|card| matches!(card.enhancement, MPip::MultTimes1Dot(_)))
            .count()
    }

    /// How many cards in the run's full deck are Stone cards.
    fn full_deck_stone_count(&self) -> usize {
        self.full_deck
            .iter()
            .filter(|card| matches!(card.enhancement, MPip::Stone(_)))
            .count()
    }

    /// How many cards the full deck is **below** its starting size — the count
    /// of cards destroyed over the run. Saturates at 0, so a deck grown past
    /// its starting size (DNA, Séance) scores nothing rather than wrapping.
    fn cards_missing_from_deck(&self) -> usize {
        self.starting_deck_size.saturating_sub(self.full_deck.len())
    }

    /// Winning outcomes for a 1-in-N probability roll.
    ///
    /// 1 normally, doubled per **Oops! All 6s** on the board (each doubles
    /// listed probabilities). The caller caps it at the roll's denominator so
    /// it never exceeds certainty.
    fn probability_numerator(&self) -> usize {
        let oops = self
            .jokers
            .iter()
            .filter(|joker| matches!(joker.enhancement, MPip::DoubleOdds))
            .count();
        // 2^oops, saturating so a pathological joker count can't overflow-shift.
        1usize
            .checked_shl(u32::try_from(oops).unwrap_or(u32::MAX))
            .unwrap_or(usize::MAX)
    }

    /// The ×mult factor a joker applies to the running score given the played
    /// hand, or `None` if it is not a (satisfied) multiplicative joker — in
    /// which case it is handled additively. Hand-conditional jokers use the
    /// "contains" predicates (e.g. `has_pair` is true for two pair / trips /
    /// full house / quads), matching Balatro.
    #[allow(clippy::cast_precision_loss)]
    fn joker_x_mult(&self, joker: &BuffoonCard) -> Option<f32> {
        let played = &self.played;
        // The straight/flush conditionals (The Order, The Tribe) honour the
        // board's rule modifiers, so Four Fingers / Shortcut let them fire on a
        // four-card or gapped hand just as they widen the base hand type.
        let rules = self.hand_rules();
        let factor = match joker.enhancement {
            MPip::MultTimes(n) => n as f32,
            MPip::MultTimes1Dot(n) => n as f32 / 10.0,
            MPip::MultTimesOnPair(n) if played.has_pair() => n as f32,
            MPip::MultTimesOnTrips(n) if played.has_trips() => n as f32,
            MPip::MultTimesOn4OfAKind(n) if played.has_4_of_a_kind() => n as f32,
            MPip::MultTimesOnStraight(n) if played.has_straight_with(rules) => n as f32,
            MPip::MultTimesOnFlush(n) if played.has_flush_with(rules) => n as f32,
            MPip::MultTimesPerScoredRank(n, ranks) => {
                // ×n for each played card of a matching rank; the factor
                // compounds, e.g. two Kings and a Queen with Triboulet = ×2³.
                let matches = played
                    .iter()
                    .filter(|card| ranks.contains(&card.rank.index))
                    .count();
                (0..matches).fold(1.0, |acc, _| acc * n as f32)
            }
            MPip::MultTimesPerHeldRank(tenths, rank) => {
                // ×(tenths/10) for each held card of `rank`; compounds. Baron.
                let held = self
                    .in_hand
                    .iter()
                    .filter(|card| card.rank.index == rank)
                    .count();
                let per = tenths as f32 / 10.0;
                (0..held).fold(1.0, |acc, _| acc * per)
            }
            MPip::MultTimesPlusPerFullDeckSteel(tenths) => {
                // Steel Joker: ×1 base, gaining ×(tenths/10) per Steel card in
                // the full deck. Additive in the factor, unlike the compounding
                // per-card jokers above: two Steel with ×0.2 is ×1.4, not ×1.44.
                let steel = self.full_deck_steel_count();
                1.0 + (tenths * steel) as f32 / 10.0
            }
            MPip::MultTimesPerUncommonJoker(tenths) => {
                // ×(tenths/10) per Uncommon joker on the board; compounds. Baseball Card.
                let uncommon = self
                    .jokers
                    .iter()
                    .filter(|j| j.card_type == BCardType::UncommonJoker)
                    .count();
                let per = tenths as f32 / 10.0;
                (0..uncommon).fold(1.0, |acc, _| acc * per)
            }
            MPip::MultTimesIfHeldAllSuits(n, suits)
                if self
                    .in_hand
                    .iter()
                    .all(|card| suits.contains(&card.suit.index)) =>
            {
                // Blackboard: vacuously true (×n) when the hand is empty.
                n as f32
            }
            _ => return None,
        };
        Some(factor)
    }

    /// Combined score for the currently played hand — the full four-phase
    /// pipeline, in Balatro order:
    ///
    /// 1. base hand chips/mult,
    /// 2. played-card chips,
    /// 3. held-card ×mult (Steel, …),
    /// 4. joker contributions.
    ///
    /// The final chips × mult is `score().score()`. Each phase folds into one
    /// running score, so the Balatro-significant ordering (held ×mult before
    /// jokers, and jokers left-to-right) is preserved. This never panics, so a
    /// solver can call it for any board.
    ///
    /// NOTE: this is deterministic — probabilistic effects (Lucky, Misprint)
    /// contribute their floor of zero here; use
    /// [`score_with_seed`](Self::score_with_seed) to roll them. State-dependent
    /// effects (economy, discards/hands remaining) still fall through to zero.
    #[must_use]
    pub fn score(&self) -> Score {
        let base = self.scoring_phase1_pre_scoring();
        let after_cards = self.scoring_phase2_dealt_hand_scoring(base);
        let held = self.scoring_phase3_effects_in_hand(after_cards);

        self.scoring_phase4_joker_scoring(held)
    }

    /// Like [`score`](Self::score), but realizes the crate's **probabilistic**
    /// effects with a `u64` seed — deterministic per seed, so a solver can
    /// reproduce a roll or sample the outcome distribution over many seeds.
    ///
    /// Currently rolled: Lucky cards (1-in-N → +20 mult, phase 2) and the
    /// Misprint joker (`MultPlusRandomTo(n)` → +random(0..n) mult, phase 4).
    /// Everything else scores identically to [`score`](Self::score); in
    /// particular the pure `score()` is the guaranteed floor (no procs).
    #[must_use]
    pub fn score_with_seed(&self, seed: u64) -> Score {
        self.score_with_rng(&mut StdRng::seed_from_u64(seed))
    }

    /// Like [`score_with_seed`](Self::score_with_seed), but drives the
    /// probabilistic effects from the caller's RNG.
    #[must_use]
    pub fn score_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Score {
        let base = self.scoring_phase1_pre_scoring();
        let after_cards = self.scoring_phase2_dealt_hand_scoring_with_rng(base, rng);
        let held = self.scoring_phase3_effects_in_hand(after_cards);

        self.scoring_phase4_joker_scoring_with_rng(held, rng)
    }

    /// Phase 2 with probabilistic played-card effects (Lucky cards) rolled from
    /// `rng`, threaded through the same played-card fold as
    /// [`scoring_phase2_dealt_hand_scoring`](Self::scoring_phase2_dealt_hand_scoring).
    #[must_use]
    pub fn scoring_phase2_dealt_hand_scoring_with_rng<R: Rng + ?Sized>(
        &self,
        running: Score,
        rng: &mut R,
    ) -> Score {
        self.fold_played_cards(running, Some(rng), None)
    }

    /// Phase 4 with probabilistic joker effects rolled, threaded through the
    /// same left-to-right fold as
    /// [`scoring_phase4_joker_scoring`](Self::scoring_phase4_joker_scoring) so
    /// a random `+mult` (Misprint) still lands in joker order and interacts
    /// correctly with later ×mult jokers.
    #[must_use]
    pub fn scoring_phase4_joker_scoring_with_rng<R: Rng + ?Sized>(
        &self,
        running: Score,
        rng: &mut R,
    ) -> Score {
        self.fold_jokers(running, Some(rng), None)
    }

    /// Like [`score`](Self::score), but resolves `MPip::Custom(id)` jokers
    /// through a mod-supplied [`EffectRegistry`] — the extension point that lets
    /// a mod add scoring behaviour without editing funky source.
    ///
    /// Built-in effects score exactly as in [`score`](Self::score); a custom
    /// card or joker is scored by looking up its id in `registry` and applying
    /// the [`ScoreOp`] its [`Effect`] returns. Unregistered ids contribute
    /// nothing.
    ///
    /// Custom effects are resolved in every phase they can occur — **played
    /// cards** (phase 2), **held cards** (phase 3) and **jokers** (phase 4) —
    /// via the same [`ScoringContext`]/[`ScoreOp`] pattern.
    ///
    /// [`Effect`]: crate::funky::types::effect::Effect
    /// [`ScoreOp`]: crate::funky::types::effect::ScoreOp
    #[must_use]
    pub fn score_with_registry(&self, registry: &EffectRegistry) -> Score {
        let base = self.scoring_phase1_pre_scoring();
        let after_cards = self.fold_played_cards::<StdRng>(base, None, Some(registry));
        let held = self.fold_held_cards(after_cards, Some(registry));

        self.scoring_phase4_joker_scoring_with_registry(held, registry)
    }

    /// Phase 4, resolving `MPip::Custom(id)` jokers through `registry`. Built-in
    /// jokers fold in exactly as [`scoring_phase4_joker_scoring`] does.
    ///
    /// [`scoring_phase4_joker_scoring`]: Self::scoring_phase4_joker_scoring
    #[must_use]
    pub fn scoring_phase4_joker_scoring_with_registry(
        &self,
        running: Score,
        registry: &EffectRegistry,
    ) -> Score {
        self.fold_jokers::<StdRng>(running, None, Some(registry))
    }

    /// Add a joker with a fresh (0) counter, keeping `joker_state` aligned.
    pub fn push_joker(&mut self, joker: BuffoonCard) {
        self.jokers.push(joker);
        self.joker_state.push(0);
    }

    /// Remove the joker at `index`, dropping its counter with it.
    pub fn remove_joker(&mut self, index: usize) -> BuffoonCard {
        if index < self.joker_state.len() {
            self.joker_state.remove(index);
        }
        self.jokers.remove(index)
    }

    /// Add a card to the run's deck: the run now **owns** it (it joins
    /// [`full_deck`](Self::full_deck)) and it is **undealt** (it joins
    /// [`deck`](Self::deck)). This is the only sanctioned way to grow the deck —
    /// writing either pile alone desynchronises the roster from the remainder.
    ///
    /// [`starting_deck_size`](Self::starting_deck_size) is deliberately *not*
    /// bumped: it records where the run started, so a deck grown past it leaves
    /// Erosion scoring nothing rather than going negative.
    pub fn add_card_to_deck(&mut self, card: BuffoonCard) {
        self.full_deck.push(card);
        self.deck.push(card);
    }

    /// Destroy the roster card at `index`: it leaves the run entirely, so it
    /// goes from [`full_deck`](Self::full_deck) and — if it had not been dealt
    /// yet — from [`deck`](Self::deck) too. Returns the destroyed card, or
    /// `None` if `index` is out of bounds.
    ///
    /// The undealt copy is located by **value**, since a [`BuffoonCard`] is a
    /// `Copy` value type with no identity. That is not a compromise: two
    /// value-equal cards are interchangeable, so removing either leaves the same
    /// multiset. A card the roster holds but the remainder does not (i.e. it is
    /// already dealt, played, or held) simply leaves the remainder untouched.
    pub fn destroy_deck_card(&mut self, index: usize) -> Option<BuffoonCard> {
        if index >= self.full_deck.len() {
            return None;
        }
        let card = self.full_deck.remove(index);
        if let Some(undealt) = self.deck.iter().position(|c| *c == card) {
            self.deck.remove(undealt);
        }
        Some(card)
    }

    /// Replace the roster card at `index` with `replacement`, keeping the
    /// undealt copy (if any) in step. Returns `false` if `index` is out of
    /// bounds.
    ///
    /// This is the seam every permanent card mutation goes through — enhancing a
    /// deck card to Steel or Stone, Hiker's `+4` chips, a tarot's rank/suit
    /// change. Same value-matching rule as [`destroy_deck_card`](Self::destroy_deck_card).
    pub fn replace_deck_card(&mut self, index: usize, replacement: BuffoonCard) -> bool {
        let Some(old) = self.full_deck.get(index).copied() else {
            return false;
        };
        self.full_deck.remove(index);
        self.full_deck.insert(index, replacement);
        if let Some(undealt) = self.deck.iter().position(|c| *c == old) {
            self.deck.remove(undealt);
            self.deck.insert(undealt, replacement);
        }
        true
    }

    /// Where `card` sits in the roster, or `None` if the run does not own it.
    /// First match wins — see [`destroy_deck_card`](Self::destroy_deck_card) for
    /// why that is exact rather than approximate.
    #[must_use]
    pub fn full_deck_index_of(&self, card: BuffoonCard) -> Option<usize> {
        self.full_deck.iter().position(|c| *c == card)
    }

    /// Pad `joker_state` with zeros up to `jokers.len()`, so a board built by
    /// setting `jokers` directly still has a counter slot per joker. Only grows —
    /// never truncates.
    fn ensure_state_len(&mut self) {
        if self.joker_state.len() < self.jokers.len() {
            self.joker_state.resize(self.jokers.len(), 0);
        }
    }

    /// How much a joker's counter changes for one growth event. The write-side
    /// mirror of `counter_joker_op`; both switch on the same enhancement. Returns
    /// 0 for every non-counter joker.
    // Arms are kept one-per-variant (rather than merged where bodies coincide)
    // to mirror `counter_joker_op`'s future per-joker arms one-for-one.
    #[allow(clippy::match_same_arms)]
    fn growth_delta(enhancement: MPip, event: &GrowthEvent, rules: HandRules) -> i32 {
        match (enhancement, event) {
            (MPip::GainMultPerHandLessDiscard(_), GrowthEvent::HandPlayed(_)) => 1,
            (MPip::GainMultPerHandLessDiscard(_), GrowthEvent::Discard(_)) => -1,
            (MPip::LoseMultTimesPerDiscard(_, _), GrowthEvent::Discard(d)) => {
                i32::try_from(d.len()).unwrap_or(i32::MAX)
            }
            (MPip::LoseChipsPerHand(_, _), GrowthEvent::HandPlayed(_)) => 1,
            (MPip::GainChipsPerCardCountHand(_, n), GrowthEvent::HandPlayed(p)) if p.len() == n => {
                1
            }
            (MPip::GainMultPerTwoPairHand(_), GrowthEvent::HandPlayed(p)) if p.has_2pair() => 1,
            // Runner counts a straight under the board's rules, so Four Fingers /
            // Shortcut grow it on a four-card or gapped straight too.
            (MPip::GainChipsPerStraightHand(_), GrowthEvent::HandPlayed(p))
                if p.has_straight_with(rules) =>
            {
                1
            }
            _ => 0,
        }
    }

    /// Grow every joker's counter for a played hand.
    pub fn on_hand_played(&mut self, played: &BuffoonPile) {
        self.apply_growth(&GrowthEvent::HandPlayed(played));
    }

    /// Grow every joker's counter for a discard.
    pub fn on_discard(&mut self, discarded: &BuffoonPile) {
        self.apply_growth(&GrowthEvent::Discard(discarded));
    }

    /// Apply the permanent card mutations that fire when the played hand scores,
    /// then score it. Today that is Hiker (`MPip::GainChipsOnScored`): every card
    /// in [`played`](Self::played) gains chips for the rest of the run.
    ///
    /// Call this **before** [`score`](Self::score): in Balatro a card gains
    /// Hiker's chips as it scores, so the boost lands on the very hand that
    /// triggers it and on every later hand the card appears in. This mirrors the
    /// order the counter jokers already use — events fire, then scoring reads.
    ///
    /// The bump is applied to the played card *and* to the run's roster copy of
    /// it, so it persists once the card cycles back into the deck. A played card
    /// the roster does not hold (the board conserves no deal invariant — see
    /// [`full_deck`](Self::full_deck)) still scores its boost; only the
    /// persistence is skipped.
    ///
    /// Chips are added to the card's **base rank value**, which is orthogonal to
    /// its `enhancement`, so a Steel or Stone card accumulates Hiker chips
    /// without either effect clobbering the other. Rank `weight` is untouched,
    /// so a fattened card still sorts and connects normally — Hiker cannot
    /// silently break straight or flush detection.
    ///
    /// # Known gap: retriggers
    ///
    /// Each played card is bumped **once per hand**, not once per scoring
    /// trigger. Balatro fires Hiker on every trigger, so a card retriggered by
    /// Hack would gain `+4` twice, with the second trigger scoring the
    /// already-fattened card. Getting that exact needs the bump interleaved into
    /// the played-card fold, which is a pure `&self` fold and cannot mutate — so
    /// it waits on scoring becoming mutating.
    /// Boards without a retrigger joker (every board today except Hack, Sock and
    /// Buskin, and Hanging Chad ones) are exact.
    pub fn on_scored(&mut self) {
        let bump: usize = self
            .jokers
            .iter()
            .map(|joker| match joker.enhancement {
                MPip::GainChipsOnScored(n) => n,
                _ => 0,
            })
            .sum();
        if bump == 0 {
            return;
        }

        for index in 0..self.played.len() {
            let Some(card) = self.played.get(index).copied() else {
                continue;
            };
            let fattened = card.add_base_chips(bump);
            self.played.remove(index);
            self.played.insert(index, fattened);
            if let Some(slot) = self.full_deck_index_of(card) {
                self.replace_deck_card(slot, fattened);
            }
        }
    }

    fn apply_growth(&mut self, event: &GrowthEvent) {
        self.ensure_state_len();
        let rules = self.hand_rules();
        let deltas: Vec<i32> = self
            .jokers
            .iter()
            .map(|j| Self::growth_delta(j.enhancement, event, rules))
            .collect();
        for (slot, delta) in self.joker_state.iter_mut().zip(deltas) {
            *slot += delta;
        }
    }

    /// The scoring contribution of a counter joker given its accumulator, or
    /// `None` if `joker` is not a counter joker (so scoring falls through to
    /// `builtin_joker_op`). Arms are added per joker in later tasks, at which
    /// point they will read board state through `&self`.
    #[allow(clippy::unused_self)]
    fn counter_joker_op(&self, joker: &BuffoonCard, counter: i32) -> Option<ScoreOp> {
        match joker.enhancement {
            MPip::GainMultPerHandLessDiscard(rate) => {
                let net = counter.max(0);
                #[allow(clippy::cast_sign_loss)]
                Some(ScoreOp::AddMult(rate * net as usize))
            }
            MPip::GainChipsPerCardCountHand(rate, _n) =>
            {
                #[allow(clippy::cast_sign_loss)]
                Some(ScoreOp::AddChips(rate * counter.max(0) as usize))
            }
            MPip::GainMultPerTwoPairHand(rate) =>
            {
                #[allow(clippy::cast_sign_loss)]
                Some(ScoreOp::AddMult(rate * counter.max(0) as usize))
            }
            MPip::GainChipsPerStraightHand(rate) =>
            {
                #[allow(clippy::cast_sign_loss)]
                Some(ScoreOp::AddChips(rate * counter.max(0) as usize))
            }
            MPip::LoseMultTimesPerDiscard(base, per) => {
                #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
                let raw = (base as f32 - per as f32 * counter.max(0) as f32) / 100.0;
                Some(ScoreOp::TimesMult(raw.max(1.0)))
            }
            MPip::LoseChipsPerHand(base, per) => {
                #[allow(clippy::cast_sign_loss)]
                let hands = counter.max(0) as usize;
                Some(ScoreOp::AddChips(base.saturating_sub(per * hands)))
            }
            _ => None,
        }
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
    use crate::funky::types::effect::{Effect, ScoreOp};
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

        let score = board.scoring_phase4_joker_scoring(Score::default());
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

        let score = board.scoring_phase4_joker_scoring(Score::default());
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

        let score = board.scoring_phase4_joker_scoring(Score::default());
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

        let score = board.scoring_phase4_joker_scoring(Score::default());
        assert_eq!(score, Score { chips: 0, mult: 20 });
    }

    #[test]
    fn phase_4_joker__mult_times_scales_running_mult() {
        let mut board = board_playing("2S");
        board.jokers.push(enhanced(card::JOKER, MPip::MultTimes(3)));
        // x3 mult; chips untouched.
        assert_eq!(
            board.scoring_phase4_joker_scoring(Score::new(10, 4)),
            Score {
                chips: 10,
                mult: 12
            }
        );
    }

    #[test]
    fn phase_4_joker__mult_times_1_dot_scales_running_mult() {
        let mut board = board_playing("2S");
        board
            .jokers
            .push(enhanced(card::JOKER, MPip::MultTimes1Dot(15))); // x1.5
        assert_eq!(
            board.scoring_phase4_joker_scoring(Score::new(10, 8)),
            Score {
                chips: 10,
                mult: 12
            }
        );
    }

    #[test]
    fn phase_4_joker__order_matters_add_then_multiply() {
        // JOKER = +4 mult (additive), then a x2 joker.
        let mut board = board_playing("2S");
        board.jokers.push(card::JOKER);
        board.jokers.push(enhanced(card::JOKER, MPip::MultTimes(2)));
        // (0, 10) -> +4 -> 14 -> x2 -> 28.
        assert_eq!(
            board.scoring_phase4_joker_scoring(Score::new(0, 10)),
            Score { chips: 0, mult: 28 }
        );
    }

    #[test]
    fn phase_4_joker__order_matters_multiply_then_add() {
        // Reverse order of the previous test: x2 first, then +4.
        let mut board = board_playing("2S");
        board.jokers.push(enhanced(card::JOKER, MPip::MultTimes(2)));
        board.jokers.push(card::JOKER);
        // (0, 10) -> x2 -> 20 -> +4 -> 24 (differs from 28 above).
        assert_eq!(
            board.scoring_phase4_joker_scoring(Score::new(0, 10)),
            Score { chips: 0, mult: 24 }
        );
    }

    /// Helper: phase-4 mult after applying one joker to a running mult of 10.
    fn joker_mult_10(index: &str, joker: BuffoonCard) -> usize {
        let mut board = board_playing(index);
        board.jokers.push(joker);
        board.scoring_phase4_joker_scoring(Score::new(0, 10)).mult
    }

    #[test]
    fn phase_4_joker__the_duo_x2_on_pair() {
        assert_eq!(joker_mult_10("AS AD QC JS TH", card::THE_DUO), 20);
        // "contains a pair" also fires on trips / full house.
        assert_eq!(joker_mult_10("AS AD AC JS TH", card::THE_DUO), 20);
        // No pair -> no effect (running mult unchanged).
        assert_eq!(joker_mult_10("2S 5D 8C TS KH", card::THE_DUO), 10);
    }

    #[test]
    fn phase_4_joker__the_trio_x3_on_trips() {
        assert_eq!(joker_mult_10("AS AD AC JS TH", card::THE_TRIO), 30);
        // A mere pair does not satisfy "three of a kind".
        assert_eq!(joker_mult_10("AS AD QC JS TH", card::THE_TRIO), 10);
    }

    #[test]
    fn phase_4_joker__the_family_x4_on_quads() {
        assert_eq!(joker_mult_10("AS AD AC AH TH", card::THE_FAMILY), 40);
        assert_eq!(joker_mult_10("AS AD AC JS TH", card::THE_FAMILY), 10);
    }

    #[test]
    fn phase_4_joker__the_order_x3_on_straight() {
        assert_eq!(joker_mult_10("AS KD QC JH TS", card::THE_ORDER), 30);
        assert_eq!(joker_mult_10("AS AD QC JS TH", card::THE_ORDER), 10);
    }

    #[test]
    fn phase_4_joker__the_tribe_x2_on_flush() {
        // Flush but not a straight.
        assert_eq!(joker_mult_10("AS KS QS JS 9S", card::THE_TRIBE), 20);
        assert_eq!(joker_mult_10("AS KD QC JS TH", card::THE_TRIBE), 10);
    }

    #[test]
    fn score__the_tribe_flush_end_to_end() {
        let mut board = board_playing("AS KS QS JS 9S"); // flush, not a straight
        board.jokers.push(card::THE_TRIBE); // x2 mult on flush

        // Phase 1 (Flush): 35 chips, 4 mult.
        // Phase 2 (cards): 11+10+10+10+9 = 50 chips.
        // Running: 85 chips, 4 mult. Phase 4: x2 -> 8 mult.
        // Final: 85 x 8 = 680.
        let score = board.score();
        assert_eq!(score, Score { chips: 85, mult: 8 });
        assert_eq!(score.score(), 680);
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
            board.scoring_phase2_dealt_hand_scoring(Score::default()),
            Score { chips: 51, mult: 0 }
        );
    }

    #[test]
    fn phase_2_dealt_hand__pair_of_aces() {
        // 11 + 11 + 10 + 10 + 10 = 52.
        let board = board_playing("AS AD QC JS TH");
        assert_eq!(
            board.scoring_phase2_dealt_hand_scoring(Score::default()),
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
            board.scoring_phase2_dealt_hand_scoring(Score::default()),
            Score { chips: 41, mult: 0 }
        );
    }

    #[test]
    fn phase_2_dealt_hand__mult_enhancement_adds_mult() {
        // A "Mult"-style card: +4 mult on top of the ace's 11 chips.
        let mut board = board_playing("KS");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::MultPlus(4))]);
        assert_eq!(
            board.scoring_phase2_dealt_hand_scoring(Score::default()),
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

    #[test]
    fn score__even_steven_joker_scores_end_to_end() {
        // Regression: `MultPlusOn5Ranks` jokers used to silently score 0.
        // Play five even cards (high card) with Even Steven.
        let mut board = board_playing("TS 8D 6C 4H 2S");
        board.jokers.push(card::EVEN_STEVEN); // +4 mult per even card

        // Phase 1 (High Card): 5 chips, 1 mult.
        // Phase 2 (cards): 10+8+6+4+2 = 30 chips.
        // Phase 4 (Even Steven): +4 mult x 5 even cards = +20 mult.
        // Combined: 35 chips x 21 mult = 735.
        let score = board.score();
        assert_eq!(
            score,
            Score {
                chips: 35,
                mult: 21
            }
        );
        assert_eq!(score.score(), 735);
    }

    #[test]
    fn score__abstract_joker_scales_with_joker_count() {
        // +3 mult per joker on the board (counting itself).
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card (5,1) + 35 chips = 40/1
        board.jokers.push(card::ABSTRACT_JOKER);
        assert_eq!(board.score(), Score::new(40, 4)); // 1 joker -> +3

        board.jokers.push(card::JOKER); // now 2 jokers; Abstract +3x2=6 then Joker +4
        assert_eq!(board.score(), Score::new(40, 11));
    }

    #[test]
    fn score__blue_joker_scales_with_deck_size() {
        // +2 chips per card remaining in the deck (a fresh 52-card deck -> +104).
        let mut board = board_playing("2S 5D 8C TS KH");
        assert_eq!(board.deck.len(), 52);
        board.jokers.push(card::BLUE_JOKER);
        // High Card (5,1) + 35 card chips = 40/1; +2x52 = +104 chips -> 144/1.
        assert_eq!(board.score(), Score::new(144, 1));
    }

    #[test]
    fn score__baron_compounds_per_held_king() {
        let mut board = board_playing("AS KS QS JS TS"); // royal flush: 100/8 + 51 = 151/8
        board.jokers.push(card::BARON);

        // No Kings held -> x1.
        assert_eq!(board.score(), Score::new(151, 8));

        // One held King -> x1.5 -> ceil(8*1.5)=12.
        board.in_hand = BuffoonPile::from(vec![basic::KING_HEARTS]);
        assert_eq!(board.score(), Score::new(151, 12));

        // Two held Kings -> x1.5^2 = x2.25 -> ceil(8*2.25)=18.
        board.in_hand = BuffoonPile::from(vec![basic::KING_HEARTS, basic::KING_DIAMONDS]);
        assert_eq!(board.score(), Score::new(151, 18));
    }

    #[test]
    fn score__scary_face_adds_chips_per_face() {
        // +30 chips per played face card (J/Q/K).
        let mut board = board_playing("KS QD JC 2S 3H"); // 3 faces
        board.jokers.push(card::SCARY_FACE);
        // High Card (5,1) + cards (10+10+10+2+3 = 35) = 40/1; +30×3 = +90 -> 130/1.
        assert_eq!(board.score(), Score::new(130, 1));

        // No face cards -> no contribution.
        let mut none = board_playing("2S 5D 8C TS 9H");
        none.jokers.push(card::SCARY_FACE);
        assert_eq!(none.score(), Score::new(39, 1));
    }

    #[test]
    fn score__walkie_talkie_per_ten_or_four() {
        // +10 chips and +4 mult per played 10 or 4.
        let mut board = board_playing("TS TD 4C 2S 3H"); // two 10s + one 4 = 3 matches
        board.jokers.push(card::WALKIE_TALKIE);
        // Pair (10,2) + cards (10+10+4+2+3 = 29) = 39/2; +30 chips, +12 mult -> 69/14.
        assert_eq!(board.score(), Score::new(69, 14));
    }

    #[test]
    fn score__blackboard_x3_when_held_all_spades_or_clubs() {
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.jokers.push(card::BLACKBOARD);

        // All held are Spades/Clubs -> ×3.
        board.in_hand = BuffoonPile::from(vec![basic::KING_SPADES, basic::QUEEN_CLUBS]);
        assert_eq!(board.score(), Score::new(40, 3));

        // A held Heart breaks the condition -> ×1 (inert).
        board.in_hand = BuffoonPile::from(vec![basic::KING_SPADES, basic::QUEEN_HEARTS]);
        assert_eq!(board.score(), Score::new(40, 1));

        // Empty hand is vacuously true -> ×3 (matches Balatro).
        board.in_hand = BuffoonPile::default();
        assert_eq!(board.score(), Score::new(40, 3));
    }

    #[test]
    fn score__baseball_card_scales_with_uncommon_jokers() {
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.jokers.push(card::BASEBALL_CARD); // Common itself -> not counted

        // No Uncommon jokers -> ×1.
        assert_eq!(board.score(), Score::new(40, 1));

        // One Uncommon joker -> ×1.5 -> ceil(1×1.5)=2. Steel Joker is the stand-in
        // and contributes ×1 of its own, the deck holding no Steel cards.
        board.jokers.push(card::STEEL_JOKER);
        assert_eq!(board.score(), Score::new(40, 2));
    }

    #[test]
    fn score__mystic_summit_adds_mult_only_when_no_discards() {
        // Mystic Summit: +15 mult when 0 discards remain, else inert.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.jokers.push(card::MYSTIC_SUMMIT);

        // Default draws (3 discards remaining) -> no bonus.
        assert_eq!(board.draws.discards, 3);
        assert_eq!(board.score(), Score::new(40, 1));

        // Zero discards remaining -> +15 mult.
        board.draws.discards = 0;
        assert_eq!(board.score(), Score::new(40, 16));
    }

    #[test]
    fn score__banner_adds_chips_per_remaining_discard() {
        // Banner: +30 chips for each remaining discard.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.jokers.push(card::BANNER);

        // 3 discards remaining -> +90 chips.
        assert_eq!(board.draws.discards, 3);
        assert_eq!(board.score(), Score::new(130, 1));

        // 0 discards remaining -> no bonus.
        board.draws.discards = 0;
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn score__bull_scales_with_money() {
        // Bull: +2 chips for each $1 you have.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.jokers.push(card::BULL);

        // No money -> no bonus.
        assert_eq!(board.money, 0);
        assert_eq!(board.score(), Score::new(40, 1));

        // $7 -> +14 chips.
        board.money = 7;
        assert_eq!(board.score(), Score::new(54, 1));

        // Debt (negative money) never subtracts chips -> floors at 0.
        board.money = -20;
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn score__scholar_adds_chips_and_mult_per_played_ace() {
        // Scholar: +20 chips and +4 mult for each played Ace, compounding with
        // the number of Aces played (independent of the Ace's own chip value).
        let mut board = board_playing("AS AD 8C TS KH"); // two Aces
        let base = board.score();

        board.jokers.push(card::SCHOLAR);
        let scored = board.score();

        assert_eq!(scored.chips, base.chips + 40, "+20 chips per Ace x2");
        assert_eq!(scored.mult, base.mult + 8, "+4 mult per Ace x2");
    }

    #[test]
    fn score__raised_fist_adds_double_lowest_held_rank_to_mult() {
        // Raised Fist: +Mult equal to double the lowest-ranked held card's value.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.in_hand = bcards!("7H 3S"); // lowest held rank value = 3

        board.jokers.push(card::RAISED_FIST);

        // +2 x 3 = +6 mult; chips unchanged.
        assert_eq!(board.score(), Score::new(40, 7));
    }

    #[test]
    fn score__cavendish_x3_end_to_end() {
        // Cavendish = MPip::MultTimes(3), an unconditional ×3.
        let mut board = board_playing("2S 5D 8C TS KH"); // high card
        board.jokers.push(card::CAVENDISH);
        // Phase 1 (High Card) 5/1 + cards (2+5+8+10+10 = 35) = 40/1; ×3 -> 40/3.
        assert_eq!(board.score(), Score::new(40, 3));
    }

    #[test]
    fn score__triboulet_compounds_per_king_and_queen() {
        // Triboulet: ×2 mult per played King or Queen (compounds).
        let mut board = board_playing("KS QS 2D 3C 4H"); // 1 King + 1 Queen
        board.jokers.push(card::TRIBOULET);
        // Phase 1 (High Card) 5/1 + cards (10+10+2+3+4 = 29) = 34/1.
        // Two matches -> ×2² = ×4 -> 34/4.
        assert_eq!(board.score(), Score::new(34, 4));

        // No Kings/Queens -> ×2⁰ = ×1 (inert).
        let mut none = board_playing("2S 5D 8C TS 9H");
        none.jokers.push(card::TRIBOULET);
        assert_eq!(none.score().mult, none.scoring_phase1_pre_scoring().mult);
    }

    #[test]
    fn score__multiplicative_joker_end_to_end() {
        // Pair of aces, one additive joker (+4) and one x3 joker.
        let mut board = board_playing("AS AD QC JS TH");
        board.jokers.push(card::JOKER); // +4 mult
        board.jokers.push(enhanced(card::JOKER, MPip::MultTimes(3))); // x3

        // Phase 1 (Pair): 10 chips, 2 mult.
        // Phase 2 (cards): 11+11+10+10+10 = 52 chips.
        // Running: 62 chips, 2 mult. Phase 3: no held cards.
        // Phase 4: +4 -> 6 mult, then x3 -> 18 mult.
        // Combined: 62 chips x 18 mult = 1116.
        let score = board.score();
        assert_eq!(
            score,
            Score {
                chips: 62,
                mult: 18
            }
        );
        assert_eq!(score.score(), 1116);
    }

    // A mod-defined effect that reads the board through the context: ×2 mult
    // when the played hand is a flush. Adding it required NO change to any core
    // `MPip` match arm — only registering it under an id.
    struct FlushDoubler;
    impl Effect for FlushDoubler {
        fn score(&self, ctx: &ScoringContext<'_>) -> ScoreOp {
            if ctx.board.played.has_flush() {
                ScoreOp::TimesMult(2.0)
            } else {
                ScoreOp::Nothing
            }
        }
    }

    struct AddChips(usize);
    impl Effect for AddChips {
        fn score(&self, _ctx: &ScoringContext<'_>) -> ScoreOp {
            ScoreOp::AddChips(self.0)
        }
    }

    struct DoubleMult;
    impl Effect for DoubleMult {
        fn score(&self, _ctx: &ScoringContext<'_>) -> ScoreOp {
            ScoreOp::TimesMult(2.0)
        }
    }

    #[test]
    fn score_with_registry__custom_played_card_scores() {
        const ID: u32 = 7001;
        let mut registry = EffectRegistry::new();
        registry.register(ID, AddChips(50));

        let mut board = board_playing("2S");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::Custom(ID))]);

        // High Card (5,1) + ace chips 11 = 16 chips; custom adds +50 -> 66.
        assert_eq!(board.score_with_registry(&registry), Score::new(66, 1));
        // Pure score() ignores customs.
        assert_eq!(board.score(), Score::new(16, 1));
    }

    #[test]
    fn score_with_registry__custom_played_xmult_multiplies_running_base() {
        // The reason phase 2 takes a running score: a played ×mult must scale
        // the score so far (including the phase-1 base), in card order.
        const ID: u32 = 7002;
        let mut registry = EffectRegistry::new();
        registry.register(ID, DoubleMult);

        let mut board = board_playing("2S");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::Custom(ID))]);

        // Base High Card (5,1) + ace 11 chips = (16,1); custom ×2 -> (16,2),
        // i.e. the base mult of 1 was doubled, not just a card subtotal.
        assert_eq!(board.score_with_registry(&registry), Score::new(16, 2));
    }

    #[test]
    fn score_with_registry__custom_held_card_scores() {
        const ID: u32 = 7003;
        let mut registry = EffectRegistry::new();
        registry.register(ID, DoubleMult);

        let mut board = board_playing("AS KS QS JS TS"); // royal flush
        board.in_hand = BuffoonPile::from(vec![enhanced(basic::KING_HEARTS, MPip::Custom(ID))]);

        // Phase 1+2: 151 chips, 8 mult. Custom held ×2 -> 16 mult.
        assert_eq!(board.score_with_registry(&registry), Score::new(151, 16));
        assert_eq!(board.score(), Score::new(151, 8));
    }

    #[test]
    fn score_with_registry__custom_joker_scores() {
        const FLUSH_DOUBLER: u32 = 9001;
        let mut registry = EffectRegistry::new();
        registry.register(FLUSH_DOUBLER, FlushDoubler);

        let mut board = board_playing("AS KS QS JS 9S"); // flush, not a straight
        board
            .jokers
            .push(enhanced(card::JOKER, MPip::Custom(FLUSH_DOUBLER)));

        // Phase 1 (Flush) 35/4 + phase 2 cards (11+10+10+10+9 = 50) = 85/4.
        // Custom joker sees the flush -> x2 mult -> 85 x 8 = 680.
        let score = board.score_with_registry(&registry);
        assert_eq!(score, Score::new(85, 8));
        assert_eq!(score.score(), 680);

        // Pure score() does not resolve customs, so the joker contributes 0.
        assert_eq!(board.score(), Score::new(85, 4));
    }

    #[test]
    fn score_with_registry__custom_effect_is_hand_aware() {
        const FLUSH_DOUBLER: u32 = 9001;
        let mut registry = EffectRegistry::new();
        registry.register(FLUSH_DOUBLER, FlushDoubler);

        // Same custom joker, but a non-flush hand -> the effect returns Nothing.
        let mut board = board_playing("AS KD QC JS 9H"); // high card
        board
            .jokers
            .push(enhanced(card::JOKER, MPip::Custom(FLUSH_DOUBLER)));
        // Phase 1 (High Card) 5/1 + cards (11+10+10+10+9=50) = 55/1, unmultiplied.
        assert_eq!(board.score_with_registry(&registry), Score::new(55, 1));
    }

    #[test]
    fn score_with_registry__unregistered_custom_is_inert() {
        let registry = EffectRegistry::new(); // empty
        let mut board = board_playing("AS KS QS JS 9S");
        board.jokers.push(enhanced(card::JOKER, MPip::Custom(404)));
        // Unknown id -> no contribution.
        assert_eq!(board.score_with_registry(&registry), Score::new(85, 4));
    }

    fn lucky_ace_board() -> BuffoonBoard {
        let mut board = board_playing("2S");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::Lucky(5, 15))]);
        board
    }

    #[test]
    fn score_with_seed__is_deterministic() {
        let mut board = board_playing("2S");
        board.jokers.push(card::MISPRINT); // MultPlusRandomTo(24)
        assert_eq!(board.score_with_seed(7), board.score_with_seed(7));
        assert_eq!(board.score_with_seed(123), board.score_with_seed(123));
    }

    #[test]
    fn score__is_the_probabilistic_floor() {
        // Pure score() rolls nothing: Lucky and Misprint contribute 0.
        // Lucky ace: High Card (5,1) + ace chips 11 = 16 chips, 1 mult.
        assert_eq!(lucky_ace_board().score(), Score::new(16, 1));

        let mut misprint = board_playing("2S"); // High Card (5,1) + 2 chips
        misprint.jokers.push(card::MISPRINT);
        assert_eq!(misprint.score(), Score::new(7, 1));
    }

    #[test]
    fn score_with_seed__lucky_procs_or_floors() {
        let board = lucky_ace_board();
        // Each roll is either the floor (16 x 1) or a proc (+20 mult -> 16 x 21).
        let mut saw_proc = false;
        let mut saw_floor = false;
        for seed in 0..64 {
            let score = board.score_with_seed(seed);
            assert_eq!(score.chips, 16);
            match score.mult {
                1 => saw_floor = true,
                21 => saw_proc = true,
                other => panic!("unexpected Lucky mult {other}"),
            }
        }
        assert!(saw_proc, "a 1-in-5 Lucky roll should hit over 64 seeds");
        assert!(saw_floor, "a 1-in-5 Lucky roll should miss over 64 seeds");
    }

    #[test]
    fn score_with_seed__misprint_varies_within_bounds() {
        let mut board = board_playing("2S"); // High Card floor: 7 chips, 1 mult
        board.jokers.push(card::MISPRINT); // +random(0..24) mult

        let first = board.score_with_seed(0).mult;
        let mut varied = false;
        for seed in 0..32 {
            let score = board.score_with_seed(seed);
            assert_eq!(score.chips, 7);
            // Floor mult 1 + a random 0..=23 bonus.
            assert!(
                (1..=24).contains(&score.mult),
                "mult {} out of range",
                score.mult
            );
            if score.mult != first {
                varied = true;
            }
        }
        assert!(
            varied,
            "Misprint should produce different mults across seeds"
        );
    }

    #[test]
    fn joker_state__push_and_remove_stay_aligned() {
        let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        board.push_joker(card::JOKER);
        board.push_joker(card::CAVENDISH);
        assert_eq!(board.jokers.len(), 2);
        assert_eq!(board.joker_state, vec![0, 0]);

        // Grow the second joker's counter, then remove the first.
        board.joker_state[1] = 7;
        let removed = board.remove_joker(0);
        assert_eq!(removed, card::JOKER);
        assert_eq!(board.jokers.len(), 1);
        // The survivor keeps its counter, now at index 0.
        assert_eq!(board.joker_state, vec![7]);
    }

    #[test]
    fn on_hand_played__is_inert_without_counter_jokers() {
        // A board with only a non-counter joker scores identically before and
        // after events fire; the plumbing exists but does nothing yet.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::CAVENDISH); // MultTimes(3): 40/1 -> 40/3
        let before = board.score();

        board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
        board.on_discard(&bcards!("2C 3C"));

        assert_eq!(
            board.score(),
            before,
            "no counter jokers -> events change nothing"
        );
        assert_eq!(
            board.joker_state,
            vec![0],
            "Cavendish is not a counter joker"
        );
    }

    #[test]
    fn score__green_joker_gains_mult_per_hand_less_discard() {
        // Green Joker: +1 Mult per hand played, −1 per discard; floors at 0.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::GREEN_JOKER);

        // 3 hands, 1 discard -> net +2 mult.
        let hand = bcards!("2S 5D 8C TS KH");
        board.on_hand_played(&hand);
        board.on_hand_played(&hand);
        board.on_hand_played(&hand);
        board.on_discard(&bcards!("9C"));
        assert_eq!(board.score(), Score::new(40, 3)); // 1 + 2

        // More discards than hands -> floored at +0 mult, not negative.
        board.on_discard(&bcards!("9C"));
        board.on_discard(&bcards!("9C"));
        board.on_discard(&bcards!("9C"));
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn score__square_joker_gains_chips_per_four_card_hand() {
        // Square Joker: +4 chips for each hand played with exactly 4 cards.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::SQUARE_JOKER);

        // Two 4-card hands -> +8 chips.
        board.on_hand_played(&bcards!("2S 5D 8C TC"));
        board.on_hand_played(&bcards!("3S 6D 9C JC"));
        assert_eq!(board.score(), Score::new(48, 1));

        // A 5-card hand does not qualify -> no further gain.
        board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
        assert_eq!(board.score(), Score::new(48, 1));
    }

    #[test]
    fn score__spare_trousers_gains_mult_per_two_pair_hand() {
        // Spare Trousers: +2 Mult for each hand played containing a Two Pair.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::SPARE_TROUSERS);

        // Two two-pair hands -> +4 mult.
        board.on_hand_played(&bcards!("2S 2D 5C 5H 8C"));
        board.on_hand_played(&bcards!("3S 3D 6C 6H 9C"));
        assert_eq!(board.score(), Score::new(40, 5));

        // A no-two-pair hand does not qualify.
        board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
        assert_eq!(board.score(), Score::new(40, 5));
    }

    #[test]
    fn score__runner_gains_chips_per_straight_hand() {
        // Runner: +15 chips for each hand played containing a Straight.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::RUNNER);

        // Two straight hands -> +30 chips.
        board.on_hand_played(&bcards!("2S 3D 4C 5H 6C"));
        board.on_hand_played(&bcards!("5S 6D 7C 8H 9C"));
        assert_eq!(board.score(), Score::new(70, 1));

        // A non-straight hand does not qualify.
        board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
        assert_eq!(board.score(), Score::new(70, 1));
    }

    #[test]
    fn score__ramen_loses_x_mult_per_card_discarded() {
        // Ramen: ×2 Mult, −×0.01 for each card discarded; floors at ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::RAMEN);

        // No discards -> ×2.00 -> mult 1 * 2.00 = 2.
        assert_eq!(board.score(), Score::new(40, 2));

        // 99 cards discarded -> ×1.01 -> ceil(1 * 1.01) = 2.
        for _ in 0..33 {
            board.on_discard(&bcards!("2C 3C 4C")); // 33 * 3 = 99 cards
        }
        assert_eq!(board.score(), Score::new(40, 2), "x1.01 ceils to mult 2");

        // 100th card discarded -> exactly ×1.00 -> mult 1 (the floor boundary).
        board.on_discard(&bcards!("2C"));
        assert_eq!(
            board.score(),
            Score::new(40, 1),
            "x1.00 at the floor boundary"
        );

        // Further discards stay floored at ×1.
        board.on_discard(&bcards!("2C 3C 4C 5C 6C"));
        assert_eq!(board.score(), Score::new(40, 1), "floored at x1");
    }

    #[test]
    fn score__ice_cream_loses_chips_per_hand_played() {
        // Ice Cream: +100 chips, −5 for each hand played; floors at 0.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::ICE_CREAM);

        // No hands played yet -> +100 chips.
        assert_eq!(board.score(), Score::new(140, 1));

        // Two hands played -> +90 chips.
        let hand = bcards!("2S 5D 8C TS KH");
        board.on_hand_played(&hand);
        board.on_hand_played(&hand);
        assert_eq!(board.score(), Score::new(130, 1));

        // 20+ hands -> floored at +0 chips.
        for _ in 0..30 {
            board.on_hand_played(&hand);
        }
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn score__hack_retriggers_played_two_through_five() {
        // Hack: retrigger each played 2, 3, 4, or 5 one additional time.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::HACK);

        // Only 2S (+2 chips) and 5D (+5 chips) qualify; each scores a second
        // time -> +7 chips. 8/T/K are untouched. 40 -> 47.
        assert_eq!(board.score(), Score::new(47, 1));
    }

    #[test]
    fn score__sock_and_buskin_retriggers_played_faces() {
        // Sock and Buskin: retrigger each played face card (K/Q/J) one more time.
        let mut board = board_playing("KH QD 8C 5S 2H"); // High Card 40/1
        board.push_joker(card::SOCK_AND_BUSKIN);

        // KH (+10) and QD (+10) each score a second time; the 8/5/2 pips are
        // untouched (T is not a face). 40 -> 60.
        assert_eq!(board.score(), Score::new(60, 1));
    }

    #[test]
    fn score__hanging_chad_retriggers_first_played_card_twice() {
        // Hanging Chad: the first played card is scored 3× total (+2 triggers).
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::HANGING_CHAD);

        // Only the first card 2S (+2 chips) retriggers, twice more -> +4 chips;
        // the other four cards are untouched. 40 -> 44.
        assert_eq!(board.score(), Score::new(44, 1));
    }

    #[test]
    fn score__stacked_retriggers_are_additive() {
        // Hack (+1 to the 2) and Hanging Chad (+2 to the first card, also the
        // 2S) stack: the 2 scores 1 + 1 + 2 = 4 times -> 3 extra -> +6 chips.
        let mut board = board_playing("2S 8D TC JS KH"); // only first card is low
        let base = board.score();

        board.push_joker(card::HACK);
        board.push_joker(card::HANGING_CHAD);
        let scored = board.score();

        assert_eq!(scored.chips, base.chips + 6, "3 extra triggers of the 2");
        assert_eq!(scored.mult, base.mult);
    }

    #[test]
    fn score__mime_retriggers_held_steel_card() {
        // Mime: retrigger held-card abilities. A held Steel King's ×1.5 fires
        // twice instead of once: 8 -> 12 -> 18.
        let mut board = board_playing("AS KS QS JS TS");
        board.in_hand = BuffoonPile::from(vec![enhanced(basic::KING_HEARTS, MPip::STEEL)]);
        board.push_joker(card::MIME);

        // Phase 1+2: 151 chips, 8 mult. Steel retriggered -> 18 mult. Mime adds
        // nothing itself in phase 4. Final 151 x 18.
        assert_eq!(board.score(), Score::new(151, 18));
    }

    #[test]
    fn score__four_fingers_makes_four_card_straight_flush() {
        // 9-T-J-Q of Hearts + an off card: a High Card normally, a four-card
        // Straight Flush with Four Fingers.
        let mut board = board_playing("9H TH JH QH 2S");
        // High Card: 5 base chips + 41 played pips (9+10+10+10+2).
        assert_eq!(board.score(), Score::new(46, 1));

        board.push_joker(card::FOUR_FINGERS);
        // Straight Flush base 100/8 + 41 pips; Four Fingers scores nothing
        // itself. 141 x 8.
        assert_eq!(board.score(), Score::new(141, 8));
    }

    #[test]
    fn score__shortcut_makes_one_gap_straight() {
        // 2-4-6-8-T: a High Card normally, a Straight with Shortcut (one-gap).
        let mut board = board_playing("2C 4D 6H 8S TC");
        // High Card: 5 base chips + 30 played pips (2+4+6+8+10).
        assert_eq!(board.score(), Score::new(35, 1));

        board.push_joker(card::SHORTCUT);
        // Straight base 30/4 + 30 pips; Shortcut scores nothing itself. 60 x 4.
        assert_eq!(board.score(), Score::new(60, 4));
    }

    #[test]
    fn score__four_fingers_enables_the_order_on_four_card_straight() {
        // A rule modifier doesn't just widen the base hand — it lets the
        // straight/flush jokers fire on the widened hand too.
        let mut board = board_playing("9H TH JH QH 2S");
        board.push_joker(card::THE_ORDER); // x3 mult on a straight

        // The four-card straight doesn't register under vanilla rules, so The
        // Order stays inert: High Card 46/1.
        assert_eq!(board.score(), Score::new(46, 1));

        board.push_joker(card::FOUR_FINGERS);
        // Now a Straight Flush (141/8), and The Order fires x3 -> 141 x 24.
        assert_eq!(board.score(), Score::new(141, 24));
    }

    #[test]
    fn score__pareidolia_makes_every_card_a_face_for_scary_face() {
        // Scary Face: +30 chips per face card. Pareidolia makes all five count.
        let mut board = board_playing("KS QD 2S 3H 4C"); // High Card 34/1, 2 faces
        board.push_joker(card::SCARY_FACE);
        // K, Q only -> +60 chips. 34 -> 94.
        assert_eq!(board.score(), Score::new(94, 1));

        board.push_joker(card::PAREIDOLIA);
        // All five cards are faces -> +150 chips. 34 -> 184.
        assert_eq!(board.score(), Score::new(184, 1));
    }

    #[test]
    fn score__pareidolia_retriggers_every_card_under_sock_and_buskin() {
        // Sock and Buskin retriggers face cards; Pareidolia makes every card one.
        let mut board = board_playing("KS QD 2S 3H 4C"); // High Card 34/1, 2 faces
        board.push_joker(card::SOCK_AND_BUSKIN);
        // Only K (+10) and Q (+10) retrigger -> +20 chips. 34 -> 54.
        assert_eq!(board.score(), Score::new(54, 1));

        board.push_joker(card::PAREIDOLIA);
        // Every card retriggers -> +(10+10+2+3+4) = +29 chips. 34 -> 63.
        assert_eq!(board.score(), Score::new(63, 1));
    }

    #[test]
    fn score__smeared_joker_merges_suits_for_flush() {
        // Five red cards over two suits (3 Hearts + 2 Diamonds): a High Card
        // normally, a Flush with Smeared.
        let mut board = board_playing("AH KH 9H QD JD");
        // High Card: 5 base + 50 played pips (11+10+9+10+10).
        assert_eq!(board.score(), Score::new(55, 1));

        board.push_joker(card::SMEARED_JOKER);
        // Flush base 35/4 + 50 pips; Smeared scores nothing itself. 85 x 4.
        assert_eq!(board.score(), Score::new(85, 4));

        // Only four cards of a merged colour is still not a flush.
        let mut four_red = board_playing("AH KH QD JD 9C");
        four_red.push_joker(card::SMEARED_JOKER);
        // Still High Card: 5 + (11+10+10+10+9) = 55/1.
        assert_eq!(four_red.score(), Score::new(55, 1));
    }

    #[test]
    fn score__smeared_enables_the_tribe_on_red_flush() {
        // The merged-suit flush also lets the flush jokers fire.
        let mut board = board_playing("AH KH 9H QD JD");
        board.push_joker(card::THE_TRIBE); // x2 mult on a flush

        // No flush under vanilla rules, so The Tribe stays inert: High Card 55/1.
        assert_eq!(board.score(), Score::new(55, 1));

        board.push_joker(card::SMEARED_JOKER);
        // Now a Flush (85/4), and The Tribe fires x2 -> 85 x 8.
        assert_eq!(board.score(), Score::new(85, 8));
    }

    #[test]
    fn score__splash_is_inert_because_all_played_cards_already_score() {
        // In Balatro only the paired Kings would score; the 2/3/4 kickers would
        // not. This engine has no scoring-vs-kicker split — every played card's
        // chips already count — so Splash's "all cards score" is a verified
        // no-op, not a silent-zero bug.
        let mut board = board_playing("KS KD 2S 3H 4C"); // Pair
        // Pair base 10/2 + all five card pips (10+10+2+3+4 = 29) = 39/2. The
        // kickers contributing is what proves cards already all score.
        assert_eq!(board.score(), Score::new(39, 2));

        board.push_joker(card::SPLASH);
        // Splash changes nothing — the score is identical.
        assert_eq!(board.score(), Score::new(39, 2));
    }

    fn lucky_two_board() -> BuffoonBoard {
        // A 1-in-2 Lucky ace: floor 16 x 1, proc (+20 mult) 16 x 21.
        let mut board = board_playing("2S");
        board.played = BuffoonPile::from(vec![enhanced(basic::ACE_SPADES, MPip::Lucky(2, 15))]);
        board
    }

    #[test]
    fn score__oops_all_6s_doubles_lucky_odds_to_certainty() {
        // Without Oops, a 1-in-2 roll misses on some seeds.
        let plain = lucky_two_board();
        assert!(
            (0..16).any(|seed| plain.score_with_seed(seed) == Score::new(16, 1)),
            "a 1-in-2 Lucky should floor on at least one seed without Oops"
        );

        // Oops! All 6s doubles 1-in-2 to 2-in-2 -> it procs on every seed.
        let mut oops = lucky_two_board();
        oops.push_joker(card::OOPS_ALL_6S);
        for seed in 0..16 {
            assert_eq!(
                oops.score_with_seed(seed),
                Score::new(16, 21),
                "seed {seed} must proc once odds are doubled to certainty"
            );
        }
    }

    /// Swap the first `n` cards of the board's full deck for `enhancement`-ed
    /// copies. Size-preserving, so it moves Steel/Stone counts without also
    /// tripping Erosion.
    /// Enhance the first `n` roster cards, through the real mutation seam.
    fn enhance_in_full_deck(board: &mut BuffoonBoard, n: usize, enhancement: MPip) {
        for index in 0..n {
            let card = board.full_deck.get(index).copied().unwrap();
            assert!(board.replace_deck_card(index, enhanced(card, enhancement)));
        }
    }

    #[test]
    fn full_deck__starts_as_the_whole_deck_and_records_its_size() {
        let board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        // The run owns everything it was dealt from, so the roster and the
        // undealt remainder start equal.
        assert_eq!(board.full_deck.len(), Deck::DECK_SIZE);
        assert_eq!(board.starting_deck_size, Deck::DECK_SIZE);
        assert_eq!(board.deck.len(), Deck::DECK_SIZE);
    }

    #[test]
    fn score__stone_joker_adds_chips_per_full_deck_stone() {
        // Stone Joker: +25 chips for each Stone card in the run's full deck.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.push_joker(card::STONE_JOKER);

        // A stock deck holds no Stone cards -> inert.
        assert_eq!(board.score(), Score::new(40, 1));

        // Two Stone cards in the deck -> +50 chips. They score from the roster;
        // the played hand is untouched, which is the whole point of the joker.
        enhance_in_full_deck(&mut board, 2, MPip::TOWER);
        assert_eq!(board.score(), Score::new(90, 1));
    }

    #[test]
    fn score__erosion_adds_mult_per_card_below_starting_deck_size() {
        // Erosion: +4 mult for each card the full deck is below its start size.
        let mut board = board_playing("2S 5D 8C TS KH"); // 40/1
        board.push_joker(card::EROSION);

        // A whole deck is not eroded -> inert.
        assert_eq!(board.full_deck.len(), board.starting_deck_size);
        assert_eq!(board.score(), Score::new(40, 1));

        // Destroy three cards -> +12 mult.
        for _ in 0..3 {
            board.full_deck.remove(0);
        }
        assert_eq!(board.score(), Score::new(40, 13));

        // A deck grown past its starting size scores nothing, rather than
        // wrapping around on the subtraction.
        let mut grown = board_playing("2S 5D 8C TS KH");
        grown.push_joker(card::EROSION);
        grown.full_deck.push(basic::ACE_SPADES);
        assert_eq!(grown.score(), Score::new(40, 1));
    }

    #[test]
    fn score__steel_joker_x_mult_grows_additively_per_full_deck_steel() {
        // Steel Joker: x1 base, +x0.2 per Steel card in the full deck.
        let mut board = board_playing("9H TH JH QH KH"); // Straight Flush 100/8 + 49 = 149/8
        assert_eq!(board.played.determine_hand_type(), HandType::StraightFlush);
        board.push_joker(card::STEEL_JOKER);

        // No Steel in the deck -> x1, not zero.
        assert_eq!(board.score(), Score::new(149, 8));

        // Four Steel -> x(1 + 0.2x4) = x1.8 -> ceil(8 x 1.8) = 15. Were the
        // factor compounding (1.2^4 = x2.07) this would be 17, so the exact
        // value pins the additive rule.
        enhance_in_full_deck(&mut board, 4, MPip::STEEL);
        assert_eq!(board.score(), Score::new(149, 15));
    }

    #[test]
    fn add_card_to_deck__grows_the_roster_and_the_undealt_remainder() {
        let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        let start = board.starting_deck_size;

        board.add_card_to_deck(enhanced(basic::ACE_SPADES, MPip::TOWER));

        // The run owns one more card, and it has not been dealt yet.
        assert_eq!(board.full_deck.len(), start + 1);
        assert_eq!(board.deck.len(), start + 1);
        // Where the run *started* is history and does not move, so Erosion keeps
        // measuring against the original size.
        assert_eq!(board.starting_deck_size, start);
    }

    #[test]
    fn destroy_deck_card__removes_the_undealt_copy_but_tolerates_a_dealt_one() {
        let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        let start = board.starting_deck_size;

        // An undealt card leaves both piles: the run no longer owns it, and it
        // can no longer be drawn.
        let card = board.full_deck.get(0).copied().unwrap();
        assert_eq!(board.destroy_deck_card(0), Some(card));
        assert_eq!(board.full_deck.len(), start - 1);
        assert_eq!(board.deck.len(), start - 1);
        assert!(!board.deck.contains(&card));

        // A card already dealt out of the remainder still leaves the roster; the
        // remainder simply has nothing to drop. This is the case the board's
        // lack of a deal invariant makes real.
        let dealt = board.full_deck.get(0).copied().unwrap();
        let removed = board.deck.iter().position(|c| *c == dealt).unwrap();
        board.deck.remove(removed);
        assert_eq!(board.destroy_deck_card(0), Some(dealt));
        assert_eq!(board.full_deck.len(), start - 2);
        assert_eq!(board.deck.len(), start - 2);

        // Out of bounds is None, not a panic.
        assert_eq!(board.destroy_deck_card(9_999), None);
    }

    #[test]
    fn replace_deck_card__swaps_the_card_in_both_piles_and_keeps_its_slot() {
        let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        let card = board.full_deck.get(3).copied().unwrap();
        let steel = enhanced(card, MPip::STEEL);

        assert!(board.replace_deck_card(3, steel));

        // Same size, same slot, new card -- in the roster and the remainder both.
        assert_eq!(board.full_deck.len(), board.starting_deck_size);
        assert_eq!(board.full_deck.get(3), Some(&steel));
        assert!(board.deck.contains(&steel));
        assert!(!board.deck.contains(&card));

        assert!(!board.replace_deck_card(9_999, steel));
    }

    #[test]
    fn score__erosion_moves_through_real_deck_mutation() {
        // Phase 7 could only pose Erosion by poking `full_deck` directly. The
        // mutation seam is what makes it move the way play would.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::EROSION);
        assert_eq!(board.score(), Score::new(40, 1));

        for _ in 0..3 {
            assert!(board.destroy_deck_card(0).is_some());
        }
        // Three cards below the starting size -> +12 mult.
        assert_eq!(board.score(), Score::new(40, 13));
    }

    #[test]
    fn score__steel_joker_counts_the_deck_not_the_hand() {
        // The Steel *card* scores x1.5 while held (phase 3); the Steel *Joker*
        // reads the roster (phase 4). A Steel card held but not in the full
        // deck must move only the former -- this is the distinction the
        // full-deck view exists to draw.
        let mut board = board_playing("2S 5D 8C TS KH"); // 40/1
        board.in_hand = BuffoonPile::from(vec![enhanced(basic::KING_HEARTS, MPip::STEEL)]);
        board.push_joker(card::STEEL_JOKER);

        // Held Steel: x1.5 -> ceil(1 x 1.5) = 2. Steel Joker still sees an
        // unenhanced deck -> x1.
        assert_eq!(board.score(), Score::new(40, 2));
    }

    #[test]
    fn score__glass_card_multiplies_mult_when_scored() {
        // Glass card: x2 Mult when scored, 1 in 4 chance to be destroyed after
        // the hand. Both halves were declared on the const and neither was
        // wired, so a Glass King scored exactly like a plain King.
        let mut board = board_playing("2S 5D 8C TS"); // High Card 5/1 + 25 = 30/1
        assert_eq!(board.score(), Score::new(30, 1));

        // A plain King is +10 chips and nothing else.
        board.played.push(basic::KING_HEARTS);
        assert_eq!(board.score(), Score::new(40, 1));

        // The same King in Glass keeps its chips and doubles the mult.
        board.played.remove(4);
        board
            .played
            .push(enhanced(basic::KING_HEARTS, MPip::Glass(2, 4)));
        assert_eq!(board.score(), Score::new(40, 2));
    }

    #[test]
    fn score__glass_card_multiplies_at_its_own_position_in_the_hand() {
        // x-mult is order-sensitive: Glass scales the score accumulated up to
        // *its* card, so a later +mult card is not doubled. Pinning this stops
        // the arm drifting into the additive `calculate_plus` path, where it
        // would silently lose its ordering.
        let glass = enhanced(basic::KING_HEARTS, MPip::Glass(2, 4));
        let mult_card = enhanced(basic::QUEEN_HEARTS, MPip::MultPlus(4));

        // Chips are order-independent: High Card base 5 + 2 + K10 + Q10 = 27.
        let chips = 5 + 2 + 10 + 10;

        // Glass first: mult 1 x2 = 2, then the Mult card's +4 = 6.
        let mut glass_first = board_playing("2S");
        glass_first.played.push(glass);
        glass_first.played.push(mult_card);
        assert_eq!(glass_first.score(), Score::new(chips, 6));

        // Mult card first: 1 + 4 = 5, then Glass doubles it = 10. Same cards,
        // same chips, different mult -- which is the whole point.
        let mut mult_first = board_playing("2S");
        mult_first.played.push(mult_card);
        mult_first.played.push(glass);
        assert_eq!(mult_first.score(), Score::new(chips, 10));
    }

    #[test]
    fn score__gros_michel_adds_mult_regardless_of_its_destruction_chance() {
        // Gros Michel: +15 Mult, 1 in 6 chance to be destroyed at end of round.
        // The const carried only the destruction, so the joker silently scored
        // nothing -- it is the whole reason to play the card.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        assert_eq!(board.score(), Score::new(40, 1));

        board.push_joker(card::GROS_MICHEL);

        // The mult is unconditional: nothing about the 1-in-6 roll gates it, and
        // the pure score() path never rolls at all.
        assert_eq!(board.score(), Score::new(40, 16));
    }

    #[test]
    fn score__gros_michel_mult_is_not_a_probabilistic_effect() {
        // Its sibling Lucky rolls on the seeded path and floors on the pure one.
        // Gros Michel must not: +15 is flat, so every seed agrees with score().
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::GROS_MICHEL);

        for seed in 0..16_u64 {
            assert_eq!(board.score_with_seed(seed), Score::new(40, 16));
        }
    }

    #[test]
    fn score__cavendish_still_scores_its_x3_beside_its_sibling() {
        // Gros Michel and Cavendish are a matched pair in Balatro, and were
        // mirror-image data bugs: Gros Michel kept the destruction and lost the
        // mult, Cavendish kept the mult and lost the destruction. Cavendish's
        // scoring half is the one that always worked -- pin it so the compound
        // variant landing next door cannot regress it.
        let mut board = board_playing("2S 5D 8C TS KH"); // 40/1
        board.push_joker(card::CAVENDISH);
        assert_eq!(board.score(), Score::new(40, 3));
    }

    #[test]
    fn score__hiker_permanently_adds_chips_to_every_scored_card() {
        // Hiker: every played card permanently gains +4 chips when scored. The
        // boost lands on the hand that triggers it, so five cards -> +20 chips.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 5/1 + 35 = 40/1
        board.push_joker(card::HIKER);

        // Inert until the hand actually scores.
        assert_eq!(board.score(), Score::new(40, 1));

        board.on_scored();
        assert_eq!(board.score(), Score::new(60, 1));

        // "Permanently" is the whole joker: scoring the same cards again stacks
        // another +4 each rather than re-applying a flat bonus.
        board.on_scored();
        assert_eq!(board.score(), Score::new(80, 1));
    }

    #[test]
    fn on_scored__persists_the_chips_onto_the_run_roster() {
        // The bump has to outlive the hand, or Hiker is just a +4/card scoring
        // arm. The roster copy is what the card carries back into the deck.
        let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
        board.played = BuffoonPile::from(vec![basic::KING_SPADES]);
        board.push_joker(card::HIKER);

        let slot = board.full_deck_index_of(basic::KING_SPADES).unwrap();
        board.on_scored();

        // A King is 10 chips; after one scoring it is 14, in the roster and in
        // the undealt remainder both.
        let fattened = board.full_deck.get(slot).copied().unwrap();
        assert_eq!(fattened.get_chips(), 14);
        assert!(board.deck.contains(&fattened));
        assert!(!board.deck.contains(&basic::KING_SPADES));
    }

    #[test]
    fn on_scored__stacks_with_an_enhancement_rather_than_clobbering_it() {
        // Chips ride on the base rank value; enhancements are a separate field.
        // A Steel card must keep its x1.5 *and* collect Hiker's chips -- this is
        // why the bump goes through `add_base_chips` and not `enhance`.
        let mut board = board_playing("2S 5D 8C TS"); // 4 cards
        let steel_king = enhanced(basic::KING_HEARTS, MPip::STEEL);
        board.in_hand = BuffoonPile::from(vec![steel_king]);
        board.played.push(steel_king);
        board.push_joker(card::HIKER);

        board.on_scored();

        let played_king = board.played.get(4).copied().unwrap();
        assert_eq!(played_king.get_chips(), 14);
        assert_eq!(played_king.enhancement, MPip::STEEL);
    }

    #[test]
    fn on_scored__leaves_rank_weight_alone_so_detection_is_unaffected() {
        // Hiker fattens `rank.value`; straights and flushes key off `weight`. If
        // the bump touched weight, a Hiker board would silently stop detecting
        // its own straight flush -- the exact silent-wrong class this EPIC
        // guards against.
        let mut board = board_playing("9H TH JH QH KH");
        board.push_joker(card::HIKER);
        assert_eq!(board.played.determine_hand_type(), HandType::StraightFlush);

        board.on_scored();
        board.on_scored();

        assert_eq!(board.played.determine_hand_type(), HandType::StraightFlush);
        // Straight Flush 100/8 + (49 pips + 5 cards x 8 chips) = 189/8.
        assert_eq!(board.score(), Score::new(189, 8));
    }

    #[test]
    fn on_scored__bumps_once_per_hand_even_when_a_card_is_retriggered() {
        // Characterization of a known gap, not an endorsement. Balatro fires
        // Hiker per scoring *trigger*, so Hack's retriggered 5 would gain +4
        // twice and score the second time already fattened. `on_scored` runs
        // before the pure fold, so it bumps once per hand instead. Pinned here
        // so the deviation is visible and this test fails the day scoring
        // becomes mutating and the gap can be closed properly.
        let mut board = board_playing("5H 5S 5D"); // Trips
        board.push_joker(card::HIKER);
        board.push_joker(card::HACK); // retriggers each played 2-5

        board.on_scored();

        // Each 5 is bumped once (5 -> 9) and scored twice by Hack: Trips base
        // 30/3 + 6 x 9 = 84/3. Balatro would give 30 + (9+13) x 3 = 96/3.
        assert_eq!(board.score(), Score::new(84, 3));
    }

    #[test]
    fn on_scored__is_inert_without_hiker() {
        // Every other board must be byte-identical across the new hook.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MYSTIC_SUMMIT);
        let before = board.clone();

        board.on_scored();

        assert_eq!(board, before);
    }
}
