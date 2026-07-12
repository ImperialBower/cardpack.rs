use crate::funky::types::draws::Draws;
use crate::funky::types::effect::{EffectRegistry, ScoreOp, ScoringContext};
use crate::preludes::funky::{
    BCardType, BuffoonCard, BuffoonPile, HandType, MPip, PokerHands, Score,
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
        let hand_type = match self.played.determine_hand_type() {
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
    /// a Lucky card rolls (if `rng`), a `MPip::Custom` card is looked up (if
    /// `registry`). Built-in cards are unaffected by the options.
    fn fold_played_cards<R: Rng + ?Sized>(
        &self,
        running: Score,
        mut rng: Option<&mut R>,
        registry: Option<&EffectRegistry>,
    ) -> Score {
        let mut score = running;

        for card in &self.played {
            // Every played card contributes its built-in chips/mult first, then
            // its special (probabilistic / custom) effect resolves in card order.
            score = Self::builtin_played_op(card).apply(score);

            let special = match card.enhancement {
                MPip::Lucky(mult_odds, _) if mult_odds > 0 => {
                    rng.as_deref_mut().map_or(ScoreOp::Nothing, |rng| {
                        if rng.random_range(0..mult_odds) == 0 {
                            ScoreOp::AddMult(LUCKY_MULT)
                        } else {
                            ScoreOp::Nothing
                        }
                    })
                }
                MPip::Custom(id) => self.custom_op(*card, id, registry),
                _ => ScoreOp::Nothing,
            };
            score = special.apply(score);
        }

        score
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

        for card in &self.in_hand {
            let op = match card.enhancement {
                MPip::Custom(id) => self.custom_op(*card, id, registry),
                _ => Self::builtin_held_op(card),
            };
            score = op.apply(score);
        }

        score
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
            MPip::ChipsPlusPerScoredFace(n) => {
                let faces = self
                    .played
                    .iter()
                    .filter(|card| matches!(card.rank.index, 'K' | 'Q' | 'J'))
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

    /// The ×mult factor a joker applies to the running score given the played
    /// hand, or `None` if it is not a (satisfied) multiplicative joker — in
    /// which case it is handled additively. Hand-conditional jokers use the
    /// "contains" predicates (e.g. `has_pair` is true for two pair / trips /
    /// full house / quads), matching Balatro.
    #[allow(clippy::cast_precision_loss)]
    fn joker_x_mult(&self, joker: &BuffoonCard) -> Option<f32> {
        let played = &self.played;
        let factor = match joker.enhancement {
            MPip::MultTimes(n) => n as f32,
            MPip::MultTimes1Dot(n) => n as f32 / 10.0,
            MPip::MultTimesOnPair(n) if played.has_pair() => n as f32,
            MPip::MultTimesOnTrips(n) if played.has_trips() => n as f32,
            MPip::MultTimesOn4OfAKind(n) if played.has_4_of_a_kind() => n as f32,
            MPip::MultTimesOnStraight(n) if played.has_straight() => n as f32,
            MPip::MultTimesOnFlush(n) if played.has_flush() => n as f32,
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
    fn growth_delta(enhancement: MPip, event: &GrowthEvent) -> i32 {
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
            (MPip::GainChipsPerStraightHand(_), GrowthEvent::HandPlayed(p)) if p.has_straight() => {
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

    fn apply_growth(&mut self, event: &GrowthEvent) {
        self.ensure_state_len();
        let deltas: Vec<i32> = self
            .jokers
            .iter()
            .map(|j| Self::growth_delta(j.enhancement, event))
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

        // One Uncommon joker (Steel Joker, inert here) -> ×1.5 -> ceil(1×1.5)=2.
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
}
