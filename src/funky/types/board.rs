use crate::funky::decks::basic;
use crate::funky::decks::joker::Joker;
use crate::funky::decks::planet::Planet;
use crate::funky::decks::tarot::MajorArcana;
use crate::funky::types::blind::Blind;
use crate::funky::types::draws::Draws;
use crate::funky::types::effect::{EffectRegistry, ScoreOp, ScoringContext};
use crate::funky::types::shop::{BoosterPack, PackKind, Shop};
use crate::funky::types::voucher::Voucher;
use crate::preludes::funky::{
    BCardType, BuffoonCard, BuffoonPile, HandRules, HandType, MPip, PokerHands, Score,
};
use std::collections::BTreeMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Balatro's Lucky card grants a flat +20 mult on a successful (1-in-N) roll.
const LUCKY_MULT: usize = 20;

/// A lifecycle event that can grow a joker's counter or pay a joker's cash.
enum GrowthEvent<'a> {
    HandPlayed(&'a BuffoonPile),
    Discard(&'a BuffoonPile),
    RoundEnd,
    /// A playing card joined the run's deck through
    /// [`BuffoonBoard::add_card_to_deck`] — Hologram's trigger. Carries the card
    /// so a future "gains only on <kind>" joker can discriminate; Hologram
    /// counts them all alike.
    CardAdded(BuffoonCard),
    /// A playing card left the run through [`BuffoonBoard::destroy_deck_card`] —
    /// Canio's trigger, which counts only the faces among them.
    CardDestroyed(BuffoonCard),
    /// The played hand is about to score, fired by [`BuffoonBoard::on_scored`]
    /// with the hand — Vampire's trigger.
    ///
    /// Distinct from [`HandPlayed`](Self::HandPlayed), which fires *after* the
    /// hand has scored and records it. A counter growing here is read by the
    /// very hand that grew it; one growing on `HandPlayed` is not.
    Scored(&'a BuffoonPile),
    /// A consumable was spent through [`BuffoonBoard::use_consumable`] — the
    /// trigger for Constellation (Planets) and Fortune Teller (Tarots). Carries
    /// the card so the two can tell each other's kind apart.
    ConsumableUsed(BuffoonCard),
    /// A blind was selected, fired by [`BuffoonBoard::on_blind_selected`] with
    /// the blind — Madness's trigger, which fires on everything *except* a boss.
    BlindSelected(Blind),
    /// The shop's card slots were rerolled, fired by
    /// [`BuffoonBoard::reroll_with_rng`] — Flash Card's trigger, which gains
    /// `+n` mult on each one.
    ShopRerolled,
    /// A booster pack was skipped, fired by [`BuffoonBoard::skip_pack`] — Red
    /// Card's trigger, which gains `+n` mult on each one.
    ///
    /// There is deliberately **no `PackOpened` event**: Hallucination, the only
    /// joker that reads a pack opening, is a probabilistic *creation* rolled
    /// immediately (the Riff-Raff shape), not a counter that grows — so it is
    /// handled inline in [`BuffoonBoard::open_pack_with_rng`] rather than through
    /// this growth seam, which only carries counter deltas.
    PackSkipped,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuffoonBoard {
    pub draws: Draws,
    pub deck: BuffoonPile,
    pub in_hand: BuffoonPile,
    pub played: BuffoonPile,
    /// Cards spent this round — played out or discarded.
    ///
    /// The pile the board never had: until the round loop landed there was
    /// nowhere for a spent card to go, which is half of why
    /// [`full_deck`](Self::full_deck) had to be a stored roster rather than a
    /// union of the location piles.
    pub discarded: BuffoonPile,
    /// Chips × mult accumulated across every hand played this round — what a
    /// blind is beaten with. Reset at blind select and round end.
    pub round_score: usize,
    /// The score this round must reach to be won, or `0` for an untargeted
    /// round that simply runs until its hands are spent.
    ///
    /// Set by the caller. Balatro derives it from the ante and the blind (Small
    /// ×1, Big ×1.5, Boss ×2 of an ante base), and **ante progression is not
    /// modelled here** — so the mechanism lives on the board while the table
    /// does not. Inventing the table would be a number this engine cannot check.
    pub blind_target: usize,
    pub consumables: BuffoonPile,
    pub jokers: BuffoonPile,
    pub poker_hands: PokerHands,
    /// Money the run currently holds. Signed so Credit Card can carry debt to
    /// -$20. Read by scoring jokers (Bull); written by the `+$` jokers through
    /// [`on_round_end`](Self::on_round_end) / [`on_discard`](Self::on_discard),
    /// and later by the shop and base interest. Inert by default (0).
    pub money: isize,
    /// How many discards have been used this round. Incremented by
    /// [`on_discard`](Self::on_discard), reset by
    /// [`on_round_end`](Self::on_round_end). Delayed Gratification forfeits its
    /// payout the moment this is non-zero. Kept separate from
    /// [`draws`](Self::draws), which counts what the round *grants*, not what
    /// it has consumed.
    pub discards_used: usize,
    /// How many hands have been **completed** this round. Incremented by
    /// [`on_hand_played`](Self::on_hand_played), reset by
    /// [`on_blind_selected`](Self::on_blind_selected) and
    /// [`on_round_end`](Self::on_round_end).
    ///
    /// Counts hands *behind* the board, not the one in front of it: a hand is
    /// scored and then recorded, which is the convention every counter joker
    /// already follows (Ice Cream scores its full +100 on the first hand and
    /// only then decays). So while the round's Nth hand is being scored this
    /// reads `N − 1` — see [`is_final_hand`](Self::is_final_hand), which is what
    /// Dusk turns on.
    ///
    /// Per **round**, unlike the per-run [`joker_state`](Self::joker_state)
    /// accumulators, because "final hand of round" resets with the round.
    pub hands_played: usize,
    /// How many times each poker hand type has been played **this round** —
    /// Card Sharp's condition ("already been played this round").
    ///
    /// The per-type twin of [`hands_played`](Self::hands_played), and per round
    /// like it. Distinct from [`PokerHands`]'s `times_played`, which counts the
    /// whole **run** and never resets, so it cannot answer this question.
    ///
    /// Board state rather than a per-joker counter, deliberately: two Card
    /// Sharps read one shared tally, which falls out of this being a property of
    /// the round rather than of a joker.
    pub hands_by_type_this_round: BTreeMap<HandType, usize>,
    /// The suit Ancient Joker currently pays for, re-rolled at the end of each
    /// round by [`on_round_end_with_rng`](Self::on_round_end_with_rng).
    ///
    /// Run state, not per-joker state — which is *why* two Ancient Jokers agree
    /// on the suit in Balatro: it is one shared field, not a synchronisation
    /// rule. `None` until the first roll, which is what makes that roll a
    /// 1-in-4 across all four suits while every later one is a 1-in-3 excluding
    /// the current — so the suit can never repeat back to back.
    pub ancient_suit: Option<char>,
    /// The round configuration the run started with — the baseline
    /// [`on_blind_selected`](Self::on_blind_selected) recomputes
    /// [`draws`](Self::draws) from. Recorded (like
    /// [`starting_deck_size`](Self::starting_deck_size)) so joker draw
    /// modifiers never stack across blinds, and a sold modifier joker takes
    /// its bonus with it at the next blind.
    pub starting_draws: Draws,
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
    /// How many jokers the board has **room** for (Balatro's base 5).
    ///
    /// A real limit, unlike the `Vec` capacity `jokers` is built with — capacity
    /// is a reallocation hint that neither bounds pushes nor can be read back
    /// meaningfully. The "must have room" jokers (Riff-Raff) check against this.
    pub joker_slots: usize,
    /// How many consumables the board has **room** for (Balatro's base 2). The
    /// joker-slot rule on the consumable side: [`create_consumable`](Self::create_consumable)
    /// refuses to exceed it, which is the "(Must have room)" every creator card
    /// carries.
    pub consumable_slots: usize,
    /// Which blind this round is played against. Read by Madness (which refuses
    /// to trigger on a boss) and Rocket (which counts them), and applied as a
    /// [`Draws`] modifier when its ability is in force.
    pub blind: Blind,
    /// Whether the current Boss Blind's ability has been switched off by selling
    /// Luchador. Reset by [`on_blind_selected`](Self::on_blind_selected) — the
    /// next blind is a fresh boss.
    ///
    /// Chicot is deliberately **not** modelled here: it disables bosses by being
    /// on the board, so it is read live from `jokers` and needs no flag. Selling
    /// it therefore restores the boss automatically.
    pub boss_disabled: bool,
    /// How many Tarot cards the run has used, ever.
    ///
    /// Deliberately **not** a [`joker_state`](Self::joker_state) accumulator:
    /// Fortune Teller is retroactive in Balatro — it reads a run-wide statistic,
    /// so a Fortune Teller bought after ten Tarots have been used is immediately
    /// worth +10. A per-joker counter would start it at zero and be wrong.
    /// Contrast Constellation, which is a plain counter and does *not* scale
    /// retroactively.
    pub tarots_used: usize,
    /// How big [`full_deck`](Self::full_deck) was when the run started.
    ///
    /// Recorded rather than assumed to be 52, since alternate decks start at
    /// other sizes. Erosion scores the shortfall against this.
    pub starting_deck_size: usize,
    /// The open [`Shop`], or `None` while it is closed.
    ///
    /// `None` is the default and the state every board is in until
    /// [`open_shop_with_rng`](Self::open_shop_with_rng) draws one — so a run
    /// that never shops behaves exactly as it did before the shop existed.
    pub shop: Option<Shop>,
    /// The **vouchers** redeemed this run — run-permanent, redeemed once each.
    ///
    /// Defaults empty, and an empty set is inert: it contributes nothing to the
    /// draw recompute, the slot limits, or the shop's prices and weights, so a
    /// run that redeems nothing behaves exactly as it did before vouchers
    /// existed. Read live by the draw recompute and the shop's cost/weight
    /// readers (EPIC-01c Phases 2–5).
    pub vouchers: Vec<Voucher>,
}

/// An empty board with Balatro's base slot counts.
///
/// Hand-written rather than derived because the slot fields must not default to
/// `0` — a derived `Default` would produce a board with no room for a joker or a
/// consumable, which is a trap rather than a neutral starting point. Delegating
/// to [`BuffoonBoard::new`] keeps the two constructors from drifting apart.
impl Default for BuffoonBoard {
    fn default() -> Self {
        Self::new(Draws::default(), BuffoonPile::default())
    }
}

impl BuffoonBoard {
    /// Balatro's base joker slot count.
    pub const DEFAULT_JOKER_SLOTS: usize = 5;
    /// Balatro's base consumable slot count.
    pub const DEFAULT_CONSUMABLE_SLOTS: usize = 2;

    #[must_use]
    pub fn new(draws: Draws, deck: BuffoonPile) -> Self {
        // At construction the run owns exactly the deck it was handed, so the
        // roster and the undealt remainder start out equal.
        let full_deck = deck.clone();
        let starting_deck_size = full_deck.len();
        Self {
            draws,
            starting_draws: draws,
            deck,
            full_deck,
            starting_deck_size,
            in_hand: BuffoonPile::default(),
            played: BuffoonPile::default(),
            discarded: BuffoonPile::default(),
            round_score: 0,
            blind_target: 0,
            consumables: BuffoonPile::new_with_capacity(Self::DEFAULT_CONSUMABLE_SLOTS),
            jokers: BuffoonPile::new_with_capacity(Self::DEFAULT_JOKER_SLOTS),
            joker_slots: Self::DEFAULT_JOKER_SLOTS,
            consumable_slots: Self::DEFAULT_CONSUMABLE_SLOTS,
            poker_hands: PokerHands::default(),
            money: 0,
            discards_used: 0,
            hands_played: 0,
            hands_by_type_this_round: BTreeMap::new(),
            ancient_suit: None,
            tarots_used: 0,
            blind: Blind::default(),
            boss_disabled: false,
            joker_state: Vec::new(),
            shop: None,
            vouchers: Vec::new(),
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
        self.poker_hands
            .get(&self.scoring_hand_type())
            .map_or_else(Score::default, |hand| Score::new(hand.chips, hand.mult))
    }

    /// The hand type the played cards score as, under the board's
    /// [`HandRules`] — so Four Fingers and Shortcut are already accounted for.
    ///
    /// A Royal Flush is normalised to Straight Flush, matching Balatro (there is
    /// no separate Royal Flush entry to level up). Shared by phase 1 and by the
    /// hand-type readers (Card Sharp, `on_hand_played`'s per-round tally) so
    /// there is one answer to "what hand is this" for the whole board — a
    /// second, subtly different copy is exactly how a Royal Flush would come to
    /// count as its own type in one place and not another.
    #[must_use]
    pub fn scoring_hand_type(&self) -> HandType {
        Self::normalise_hand_type(self.played.determine_hand_type_with(self.hand_rules()))
    }

    fn normalise_hand_type(hand_type: HandType) -> HandType {
        match hand_type {
            HandType::RoyalFlush => HandType::StraightFlush,
            other => other,
        }
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
                    // (1-in-`_odds` after the hand) is still data only:
                    // `on_round_end_with_rng` rolls *joker* destruction, and
                    // destroying a played card needs the deck-mutation seam
                    // wired into a round loop that does not exist yet.
                    MPip::Glass(mult, _odds) => ScoreOp::TimesMult(mult as f32),
                    MPip::Custom(id) => self.custom_op(*card, id, registry),
                    _ => ScoreOp::Nothing,
                };
                score = special.apply(score);

                // The card's edition scores at its own position, after its
                // chips/mult — so a Polychrome ×1.5 multiplies the running score
                // here (the Glass shape), and a retriggered card re-applies its
                // edition each pass, matching Balatro.
                score = card.edition.score_op().apply(score);
            }
        }

        score
    }

    /// How many *additional* times the played card at `index` is scored, summed
    /// over the board's retrigger jokers. 0 for a board with none (the common
    /// case), so the played-card fold is byte-identical when no retrigger joker
    /// is held. `index` is the card's position in `self.played`, used by
    /// position-based retriggers (Hanging Chad fires only on the first card).
    ///
    /// Jokers are walked with their `joker_state` slot, since the round-state
    /// retriggers read a counter: Seltzer retriggers only while its 10 hands
    /// are unspent.
    fn played_retriggers(&self, index: usize, card: &BuffoonCard) -> usize {
        self.jokers
            .iter()
            .enumerate()
            .map(|(slot, joker)| {
                let counter = self.joker_state.get(slot).copied().unwrap_or(0);
                match joker.enhancement {
                    MPip::RetriggerPlayedRanks(n, ranks) if ranks.contains(&card.rank.index) => n,
                    MPip::RetriggerPlayedFaces(n) if self.is_face_card(card) => n,
                    MPip::RetriggerFirstPlayed(n) if index == 0 => n,
                    // Dusk: every played card, but only on the round's last hand.
                    MPip::RetriggerPlayedCardsInFinalRound if self.is_final_hand() => 1,
                    // Seltzer: every played card, for its first `hands` hands.
                    // `counter` is hands *completed*, so the hand that spends the
                    // last one still retriggers and `melt_emptied_jokers` removes
                    // the joker immediately after it.
                    MPip::RetriggerAllPlayedForHands(n, hands)
                        if usize::try_from(counter.max(0)).unwrap_or(usize::MAX) < hands =>
                    {
                        n
                    }
                    _ => 0,
                }
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

            // The joker's own edition scores at its position, after its effect —
            // so a Polychrome joker ×1.5s the running score once its +mult/×mult
            // has landed, matching Balatro's left-to-right joker order.
            score = joker.edition.score_op().apply(score);
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
                return ScoreOp::AddChips(n * self.discards_remaining());
            }
            // Mystic Summit: +n mult only when no discards remain, else inert.
            MPip::MultPlusOnZeroDiscards(n) => {
                return if self.discards_remaining() == 0 {
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
            // Fortune Teller: +n mult per Tarot used this run. A board reader,
            // not a counter — that is what makes it retroactive.
            MPip::MultPlusPerTarotUsedThisRun(n) => {
                return ScoreOp::AddMult(n * self.tarots_used);
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

    /// Whether a card of suit `card_suit` counts as the `target` suit.
    ///
    /// Exact, except under **Smeared Joker**, which makes Hearts ≡ Diamonds and
    /// Spades ≡ Clubs — the same merge it already applies to flush sizing, so
    /// Smeared widens what Ancient Joker pays for exactly as it widens a flush.
    fn suit_matches(card_suit: char, target: char, rules: HandRules) -> bool {
        if card_suit == target {
            return true;
        }
        rules.smeared
            && matches!(
                (card_suit, target),
                ('H', 'D') | ('D', 'H') | ('S', 'C') | ('C', 'S')
            )
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
            // Cavendish (`MultTimesChanceDestroyed`) rides the plain ×n arm:
            // its destruction half rolls at end of round
            // (`on_round_end_with_rng`), never gating the mult — the Gros
            // Michel compound shape, on the ×mult side.
            MPip::MultTimes(n) | MPip::MultTimesChanceDestroyed(n, _, _) => n as f32,
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
            // Card Sharp: ×n if this hand type has already been played this
            // round. `hands_by_type_this_round` is bumped by `on_hand_played`,
            // which fires *after* a hand scores — so during the round's second
            // Pair the tally reads 1, and `>= 1` is the test. (Balatro bumps
            // before its joker pass and so tests `> 1`; same semantics, and
            // getting it backwards makes Card Sharp fire on the first play.)
            MPip::MultTimesOnRepeatedHandThisRound(n)
                if self
                    .hands_by_type_this_round
                    .get(&self.scoring_hand_type())
                    .copied()
                    .unwrap_or(0)
                    >= 1 =>
            {
                n as f32
            }
            // Ancient Joker: ×(tenths/10) per played card of the run's current
            // ancient suit; compounds, like its per-card ×mult neighbours. No
            // suit rolled yet means no matches, i.e. ×1 — inert rather than
            // zeroing.
            MPip::MultTimesPerScoredAncientSuit(tenths) => {
                let matches = self.ancient_suit.map_or(0, |suit| {
                    played
                        .iter()
                        .filter(|card| Self::suit_matches(card.suit.index, suit, rules))
                        .count()
                });
                let per = tenths as f32 / 10.0;
                (0..matches).fold(1.0, |acc, _| acc * per)
            }
            // Joker Stencil: ×n per empty joker slot, "Joker Stencil included" —
            // i.e. it counts its own occupied slot as if it were empty. Every
            // Stencil on the board adds that +1, so the rule reduces to
            // `slots − (jokers that are not Stencils)`.
            //
            // The `> 0` gate is on **literally** empty slots, not the inclusive
            // count: a full board applies nothing at all. With one Stencil that
            // is unobservable (the inclusive count is exactly ×1 — identity —
            // when the gate closes), but with two on a full board the two
            // disagree, and the gate wins.
            MPip::MultTimesOnEmptyJokerSlots(n) => {
                let empty = self.joker_slots.saturating_sub(self.jokers.len());
                if empty == 0 {
                    return None;
                }
                let stencils = self
                    .jokers
                    .iter()
                    .filter(|j| matches!(j.enhancement, MPip::MultTimesOnEmptyJokerSlots(_)))
                    .count();
                (n * (empty + stencils)) as f32
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
    ///
    /// Fires the `CardAdded` growth event, so Hologram gains its ×0.25 for every
    /// card that arrives here — including the Stone card Marble Joker adds at
    /// blind select, which is the interaction Balatro players build on.
    pub fn add_card_to_deck(&mut self, card: BuffoonCard) {
        self.full_deck.push(card);
        self.deck.push(card);
        self.apply_growth(&GrowthEvent::CardAdded(card));
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
    ///
    /// Fires the `CardDestroyed` growth event, so Canio gains its ×1 when the
    /// card was a face. The event fires **after** the card is gone, so a joker
    /// reading the board sees the post-destruction deck — consistent with
    /// Erosion, which scores the shortfall this call just widened.
    pub fn destroy_deck_card(&mut self, index: usize) -> Option<BuffoonCard> {
        if index >= self.full_deck.len() {
            return None;
        }
        let card = self.full_deck.remove(index);
        if let Some(undealt) = self.deck.iter().position(|c| *c == card) {
            self.deck.remove(undealt);
        }
        self.apply_growth(&GrowthEvent::CardDestroyed(card));
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

    /// Whether the board has room for another consumable — the "(Must have
    /// room)" clause the creator cards carry.
    #[must_use]
    pub fn has_consumable_room(&self) -> bool {
        self.consumables.len() < self.consumable_slots
    }

    /// Whether the board has room for another joker.
    #[must_use]
    pub fn has_joker_room(&self) -> bool {
        self.jokers.len() < self.joker_slots
    }

    /// Put `card` in a consumable slot, or refuse if there is no room. Returns
    /// whether it landed.
    ///
    /// Refusing rather than growing past [`consumable_slots`](Self::consumable_slots)
    /// is Balatro's rule: a creator card with a full inventory simply creates
    /// nothing — it does not queue, and it does not evict.
    pub fn create_consumable(&mut self, card: BuffoonCard) -> bool {
        if !self.has_consumable_room() {
            return false;
        }
        self.consumables.push(card);
        true
    }

    /// Spend the consumable at `index`, apply its effect, and record the use.
    /// Returns the card spent, or `None` if `index` is out of bounds.
    ///
    /// What "apply" means, by kind:
    ///
    /// * **Planet** — levels its hand type, through the existing
    ///   [`PokerHands::increment`] (chips, mult, and level together).
    /// * **Tarot** — enhances each roster card named by `targets` (indices into
    ///   [`full_deck`](Self::full_deck)), through
    ///   [`BuffoonCard::enhance`] and the [`replace_deck_card`](Self::replace_deck_card)
    ///   seam, so the change persists on the run's own copy. Pass an empty
    ///   `targets` for a tarot that takes none.
    ///
    /// Either way the card leaves `consumables` and fires the `ConsumableUsed`
    /// growth event, which is what Constellation and Fortune Teller read.
    ///
    /// # Known gap: run-level tarots
    ///
    /// The **card-enhancing** tarots are applied here. The ones that act on the
    /// *run* rather than on a card — Death, Judgement, The Hermit, The Wheel of
    /// Fortune — pass through [`BuffoonCard::enhance`] unchanged, so this counts
    /// them as used (correctly, for Fortune Teller) while their real effects stay
    /// out of scope, exactly as EPIC-01a item 5e leaves them. Their systems
    /// (spectral cards, the shop, run-level RNG) are EPIC-01 Story 3's, not this
    /// seam's — using one here is a no-op rather than a wrong effect.
    ///
    /// [`PokerHands::increment`]: crate::funky::types::hands::PokerHands::increment
    pub fn use_consumable(&mut self, index: usize, targets: &[usize]) -> Option<BuffoonCard> {
        if index >= self.consumables.len() {
            return None;
        }
        let card = self.consumables.remove(index);

        match card.card_type {
            BCardType::Planet => self.poker_hands.increment(card),
            BCardType::Tarot => {
                for &slot in targets {
                    let Some(target) = self.full_deck.get(slot).copied() else {
                        continue;
                    };
                    self.replace_deck_card(slot, target.enhance(card));
                }
                self.tarots_used += 1;
            }
            _ => {}
        }

        self.apply_growth(&GrowthEvent::ConsumableUsed(card));
        Some(card)
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
    ///
    /// Takes `&self` for the same reason [`payout_delta`](Self::payout_delta)
    /// does: some growth reads the board rather than just the event. Canio
    /// classifies the destroyed card through [`is_face_card`](Self::is_face_card),
    /// so Pareidolia widens what feeds it.
    // Arms are kept one-per-variant (rather than merged where bodies coincide)
    // to mirror `counter_joker_op`'s per-joker arms one-for-one.
    #[allow(clippy::match_same_arms)]
    fn growth_delta(&self, enhancement: MPip, event: &GrowthEvent, rules: HandRules) -> i32 {
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
            // Popcorn: one tick per round ended.
            (MPip::LoseMultPerRound(_, _), GrowthEvent::RoundEnd) => 1,
            // Seltzer: one tick per hand played — its 10 hands are a per-run
            // allowance, so unlike Dusk's "final hand" it does not reset with
            // the round.
            (MPip::RetriggerAllPlayedForHands(_, _), GrowthEvent::HandPlayed(_)) => 1,
            // Yorick counts *cards* discarded, not discard actions, so the
            // accumulator takes the whole pile — the read side does the
            // per-23 division.
            (MPip::GainMultTimesPerDiscardedCards(_, _), GrowthEvent::Discard(d)) => {
                i32::try_from(d.len()).unwrap_or(i32::MAX)
            }
            // Hologram: one tick per playing card added to the deck.
            (MPip::GainMultTimesPerCardAdded(_), GrowthEvent::CardAdded(_)) => 1,
            // Canio: one tick per destroyed *face* card.
            (MPip::GainMultTimesPerFaceDestroyed(_), GrowthEvent::CardDestroyed(card))
                if self.is_face_card(card) =>
            {
                1
            }
            // Vampire: one tick per enhanced card in the hand about to score.
            // Growing on `Scored` rather than `HandPlayed` is what lets the
            // ×mult apply to that same hand.
            (MPip::GainMultTimesPerEnhancedPlayed(_), GrowthEvent::Scored(played)) => {
                i32::try_from(Self::enhanced_count(played)).unwrap_or(i32::MAX)
            }
            // Constellation: one tick per Planet used. Fortune Teller has no arm
            // here on purpose — it is retroactive and reads `tarots_used` from
            // the board instead.
            (MPip::GainMultTimesPerPlanetUsed(_), GrowthEvent::ConsumableUsed(card))
                if card.card_type == BCardType::Planet =>
            {
                1
            }
            // Madness: one tick per Small or Big Blind — never a Boss. The gain
            // is independent of whether its destruction pass finds a victim, so
            // it is counted here rather than beside the removal.
            (
                MPip::GainMultTimesOnNonBossBlindDestroyingJoker(_),
                GrowthEvent::BlindSelected(blind),
            ) if !blind.is_boss() => 1,
            // Rocket: one tick per Boss Blind defeated. Reaching the end of a
            // round on a Boss Blind is what "defeated" means here; a *disabled*
            // boss still counts, since it is still a boss.
            (MPip::CashOnRoundEndGrowingOnBossDefeat(_, _), GrowthEvent::RoundEnd)
                if self.blind.is_boss() =>
            {
                1
            }
            // Flash Card: one tick per shop reroll. Green Joker's shape on a
            // different event — the counter is read as +mult at scoring time.
            (MPip::MultPlusPerReroll(_), GrowthEvent::ShopRerolled) => 1,
            // Red Card: one tick per booster pack skipped, the same counter
            // shape on the skip event.
            (MPip::MultPlusPerPackSkipped(_), GrowthEvent::PackSkipped) => 1,
            _ => 0,
        }
    }

    /// How many cards in `pile` carry an enhancement — what Vampire counts and
    /// eats. A played card's enhancement only ever arrives from a tarot, so
    /// "not [`MPip::Blank`]" is the whole of "Enhanced" here.
    fn enhanced_count(pile: &BuffoonPile) -> usize {
        pile.iter()
            .filter(|card| card.enhancement != MPip::Blank)
            .count()
    }

    /// Grow every joker's counter for a played hand, then melt any decaying
    /// joker that has reached zero: Ice Cream is destroyed **by the hand that
    /// empties it**, not at end of round — exact Balatro timing, which is why
    /// the check rides this hook rather than [`on_round_end`](Self::on_round_end).
    pub fn on_hand_played(&mut self, played: &BuffoonPile) {
        self.apply_growth(&GrowthEvent::HandPlayed(played));
        self.hands_played += 1;
        // Record the hand's type for this round (Card Sharp). Keyed off the pile
        // handed in rather than `self.played`, since that is what was played.
        let hand_type =
            Self::normalise_hand_type(played.determine_hand_type_with(self.hand_rules()));
        *self.hands_by_type_this_round.entry(hand_type).or_insert(0) += 1;
        self.melt_emptied_jokers();
    }

    /// Draw from the [`deck`](Self::deck) until the hand is full, or the deck
    /// runs out. Returns how many cards were drawn.
    ///
    /// "Full" is `draws.hand_size`, so Juggler's +1 and The Manacle's −1 both
    /// reach it through the round's recomputed [`Draws`]. Drawing takes from the
    /// **end** of the deck (the top), which is `pop` rather than `remove(0)` —
    /// the direction the deck is meant to be dealt from.
    ///
    /// Deliberately **not** [`BuffoonPile::draw`]: that helper drains the deck
    /// and then returns `None` if it could not supply the full count, which
    /// loses the cards it already popped. Balatro simply deals as many as it
    /// has, which is what this does.
    pub fn deal_to_hand_size(&mut self) -> usize {
        let mut drawn = 0;
        while self.in_hand.len() < self.draws.hand_size {
            let Some(card) = self.deck.pop() else {
                break;
            };
            self.in_hand.push(card);
            drawn += 1;
        }
        drawn
    }

    /// How many hands the round has **left**: what it granted, minus what has
    /// been played. Floors at 0.
    #[must_use]
    pub fn hands_remaining(&self) -> usize {
        self.draws.hands_to_play.saturating_sub(self.hands_played)
    }

    /// Whether the round is finished — its target is met, or its hands are gone.
    ///
    /// An untargeted round ([`blind_target`](Self::blind_target) of 0) runs
    /// until its hands are spent, which is what makes a plain board behave as it
    /// always has.
    #[must_use]
    pub fn round_is_over(&self) -> bool {
        self.round_is_won() || self.hands_remaining() == 0
    }

    /// Whether the round's target has been reached. Always false for an
    /// untargeted round.
    #[must_use]
    pub fn round_is_won(&self) -> bool {
        self.blind_target > 0 && self.round_score >= self.blind_target
    }

    /// Take the cards at `indices` out of [`in_hand`](Self::in_hand), keeping
    /// them in hand order. `None` if any index is out of bounds, in which case
    /// the hand is left untouched — a partial move would be worse than a
    /// refusal.
    fn take_from_hand(&mut self, indices: &[usize]) -> Option<BuffoonPile> {
        let mut slots: Vec<usize> = indices.to_vec();
        slots.sort_unstable();
        slots.dedup();
        if slots.iter().any(|slot| *slot >= self.in_hand.len()) {
            return None;
        }
        let mut taken = BuffoonPile::default();
        for slot in &slots {
            taken.push(*self.in_hand.get(*slot)?);
        }
        // Remove back to front so the earlier slots stay valid.
        for slot in slots.iter().rev() {
            self.in_hand.remove(*slot);
        }
        Some(taken)
    }

    /// Play the cards at `indices` from the hand: score them, record the hand,
    /// spend them, and refill. Returns the hand's [`Score`], or `None` if the
    /// round has no hands left or an index is out of bounds.
    ///
    /// This is the sequence the lifecycle hooks were built for, and the order is
    /// the whole point:
    ///
    /// 1. the cards move from `in_hand` to `played`;
    /// 2. [`on_scored`](Self::on_scored) — pre-scoring mutations (Hiker fattens,
    ///    Vampire eats), which the hand about to score must see;
    /// 3. [`score`](Self::score) — the pure four-phase fold, and the result is
    ///    added to [`round_score`](Self::round_score);
    /// 4. [`on_hand_played`](Self::on_hand_played) — the hand is *recorded*,
    ///    which is why the counters that read it (Ice Cream, Card Sharp) see the
    ///    hand behind them rather than the one they just scored;
    /// 5. the played cards go to [`discarded`](Self::discarded), and the hand
    ///    refills from the deck.
    ///
    /// The pure variant leaves the probabilistic effects inert — a Lucky card
    /// never procs, Superposition never creates — exactly as [`score`](Self::score)
    /// does. Use [`play_hand_with_rng`](Self::play_hand_with_rng) to drive those.
    pub fn play_hand(&mut self, indices: &[usize]) -> Option<Score> {
        self.play_hand_inner::<StdRng>(indices, None)
    }

    /// [`play_hand`](Self::play_hand), with the probabilistic effects live: Lucky
    /// cards roll, and the Tarot creators (Superposition, Vagabond) fire.
    pub fn play_hand_with_rng<R: Rng + ?Sized>(
        &mut self,
        indices: &[usize],
        rng: &mut R,
    ) -> Option<Score> {
        self.play_hand_inner(indices, Some(rng))
    }

    fn play_hand_inner<R: Rng + ?Sized>(
        &mut self,
        indices: &[usize],
        mut rng: Option<&mut R>,
    ) -> Option<Score> {
        if self.hands_remaining() == 0 {
            return None;
        }
        self.played = self.take_from_hand(indices)?;

        match rng.as_deref_mut() {
            Some(rng) => self.on_scored_with_rng(rng),
            None => self.on_scored(),
        }
        // `on_scored` may have mutated the played cards (Hiker, Vampire), so the
        // hand that scores and is recorded is the board's, not the one taken.
        let scored = self.played.clone();
        let score = rng.map_or_else(|| self.score(), |rng| self.score_with_rng(rng));
        self.round_score = self.round_score.saturating_add(score.score());
        self.on_hand_played(&scored);

        self.discarded.extend(&self.played);
        self.played.clear();
        self.deal_to_hand_size();
        Some(score)
    }

    /// Discard the cards at `indices` from the hand and refill. Returns whether
    /// the discard happened — `false` if the round has no discards left or an
    /// index is out of bounds.
    ///
    /// Fires [`on_discard`](Self::on_discard), so the discard-triggered jokers
    /// see it (Faceless Joker pays, Ramen and Yorick grow) and the round's
    /// remaining-discard count drops — which Banner and Mystic Summit read.
    pub fn discard_cards(&mut self, indices: &[usize]) -> bool {
        if self.discards_remaining() == 0 {
            return false;
        }
        let Some(discarded) = self.take_from_hand(indices) else {
            return false;
        };
        self.on_discard(&discarded);
        self.discarded.extend(&discarded);
        self.deal_to_hand_size();
        true
    }

    /// How many discards the round has **left**: what it granted, minus what has
    /// been used. Floors at 0.
    ///
    /// The board splits these deliberately — [`draws`](Self::draws) is the
    /// round's *allowance* and [`discards_used`](Self::discards_used) its
    /// *consumption* — so "remaining" is neither field on its own, and any joker
    /// that says "remaining" (Banner, Mystic Summit) has to ask here. Reading
    /// `draws.discards` directly is the bug this exists to prevent: it silently
    /// means "granted", which is only the same number until the first discard.
    #[must_use]
    pub fn discards_remaining(&self) -> usize {
        self.draws.discards.saturating_sub(self.discards_used)
    }

    /// Whether the hand currently in [`played`](Self::played) is the round's
    /// **last** one — Dusk's condition.
    ///
    /// [`hands_played`](Self::hands_played) counts *completed* hands, so the
    /// hand being scored is the `hands_played + 1`-th of the
    /// `draws.hands_to_play` the round grants. `>=` rather than `==` so a board
    /// driven past its allowance (or one whose hand allowance shrank mid-round)
    /// stays final rather than silently falling back off the end.
    ///
    /// A board that never drives [`on_hand_played`](Self::on_hand_played) reads
    /// `hands_played == 0`, so this is false for any round granting more than
    /// one hand — Dusk stays inert on the untouched boards, which is what keeps
    /// the pure `score()` unchanged.
    #[must_use]
    pub fn is_final_hand(&self) -> bool {
        self.hands_played + 1 >= self.draws.hands_to_play
    }

    /// Grow every joker's counter for a discard, pay the discard-triggered
    /// jokers (Faceless Joker), and record that a discard was used this round
    /// (Delayed Gratification's forfeit signal).
    pub fn on_discard(&mut self, discarded: &BuffoonPile) {
        self.apply_growth(&GrowthEvent::Discard(discarded));
        self.apply_payouts(&GrowthEvent::Discard(discarded));
        self.discards_used += 1;
    }

    /// Start-of-blind lifecycle: recompute the round's [`draws`](Self::draws)
    /// from [`starting_draws`](Self::starting_draws) plus the board's draw
    /// modifiers — Juggler (+hand size), Drunkard (+discards), Burglar
    /// (+hands, then lose **all** discards, wiping Drunkard's bonus too, as in
    /// Balatro; the wipe lands after every increment so joker order cannot
    /// matter).
    ///
    /// Recomputing from the recorded baseline rather than mutating in place
    /// makes the hook idempotent: selecting the next blind never stacks a
    /// bonus twice, and a sold joker takes its bonus with it. On a board with
    /// no draw modifiers this is the identity.
    ///
    /// Also resets [`discards_used`](Self::discards_used): a new blind is a
    /// new round for Delayed Gratification's forfeit signal, whether or not
    /// [`on_round_end`](Self::on_round_end) was driven in between.
    ///
    /// Finally, the deterministic **creators** fire: Marble Joker adds a Stone
    /// card to the deck. The random one (Riff-Raff, which draws jokers from a
    /// rarity pool) lives in
    /// [`on_blind_selected_with_rng`](Self::on_blind_selected_with_rng),
    /// mirroring the `score`/`score_with_rng` split.
    pub fn on_blind_selected(&mut self) {
        // A new blind is a fresh boss: whatever Luchador switched off last round
        // is back on.
        self.boss_disabled = false;
        self.recompute_draws();
        self.apply_growth(&GrowthEvent::BlindSelected(self.blind));
        self.discards_used = 0;
        self.hands_played = 0;
        self.hands_by_type_this_round.clear();
        self.round_score = 0;

        // Marble Joker: one Stone card into the deck per copy. Collected first
        // so the deck can be mutated without holding a borrow on `jokers`.
        let additions: Vec<BCardType> = self
            .jokers
            .iter()
            .filter_map(|joker| match joker.enhancement {
                MPip::AddCardTypeWhenBlindSelected(card_type) => Some(card_type),
                _ => None,
            })
            .collect();
        for card_type in additions {
            if let Some(card) = Self::mint_card(card_type) {
                self.add_card_to_deck(card);
            }
        }
    }

    /// Whether the current Boss Blind's **ability** is in force.
    ///
    /// Three ways it is not: the blind is not a boss at all; Luchador was sold
    /// this round ([`boss_disabled`](Self::boss_disabled)); or a Chicot is on
    /// the board, which disables every boss just by being held.
    ///
    /// Distinct from [`Blind::is_boss`], the identity question. A disabled boss
    /// is still a boss — Madness still will not trigger on it, and Rocket still
    /// counts it defeated — it just has no ability. Keeping the two apart is
    /// what makes Chicot mean something without changing what Madness sees.
    #[must_use]
    pub fn boss_ability_active(&self) -> bool {
        self.blind.is_boss() && !self.boss_disabled && !self.has_boss_disabling_joker()
    }

    fn has_boss_disabling_joker(&self) -> bool {
        self.jokers
            .iter()
            .any(|joker| matches!(joker.enhancement, MPip::DisablesAllBossBlinds))
    }

    /// Recompute the round's [`draws`](Self::draws) from
    /// [`starting_draws`](Self::starting_draws), the board's draw-modifier
    /// jokers, and the Boss Blind's ability (if it is in force).
    ///
    /// Recomputing from the baseline rather than mutating in place is what makes
    /// this idempotent and self-cleaning — a second call never stacks a bonus,
    /// and a sold joker takes its bonus with it. That is also why selling a
    /// joker can just call this again: the board simply describes itself afresh.
    ///
    /// The boss's ability lands **last**, after every joker modifier including
    /// Burglar's discard wipe. A Boss Blind is a constraint on the round rather
    /// than another bonus in the pile, so The Needle leaves exactly one hand
    /// whatever Burglar had to say about it.
    fn recompute_draws(&mut self) {
        let mut draws = self.starting_draws;
        let mut lose_discards = false;
        for joker in &self.jokers {
            match joker.enhancement {
                MPip::HandSizeIncrement(n) => draws.hand_size += n,
                MPip::DiscardIncrement(n) => draws.discards += n,
                MPip::GainHandsLoseDiscardsWhenBlindSelected(n) => {
                    draws.hands_to_play += n;
                    lose_discards = true;
                }
                _ => {}
            }
        }
        // The Draws vouchers (EPIC-01c Phase 2), read live like the jokers so
        // they stack with them and never accumulate across blinds. Added before
        // the discard-wipe so Burglar still zeroes a Wasteful discard, and before
        // the boss ability so The Needle still overrides Grabber — the ordering
        // both is deliberate.
        for voucher in &self.vouchers {
            match voucher {
                Voucher::Grabber | Voucher::NachoTong => draws.hands_to_play += 1,
                Voucher::Wasteful | Voucher::Recyclomancy => draws.discards += 1,
                Voucher::PaintBrush | Voucher::Palette => draws.hand_size += 1,
                _ => {}
            }
        }
        if lose_discards {
            draws.discards = 0;
        }
        if self.boss_ability_active() {
            if let Some(boss) = self.blind.boss() {
                draws = boss.apply(draws);
            }
        }
        self.draws = draws;
    }

    /// Sell the joker at `index`: it leaves the board, its
    /// [`resell_value`](BuffoonCard::resell_value) is paid into
    /// [`money`](Self::money), and the round's draws are recomputed. Returns the
    /// joker sold, or `None` if `index` is out of bounds.
    ///
    /// Selling **Luchador** disables the current Boss Blind, which is its whole
    /// effect. The recompute is what makes that observable: the boss's grip on
    /// the round's draws lifts immediately. The same recompute means selling any
    /// draw-modifier joker (Juggler, Drunkard) correctly takes its bonus with it,
    /// and selling a Chicot hands the boss back its ability.
    ///
    /// The round's own counters ([`hands_played`](Self::hands_played),
    /// [`discards_used`](Self::discards_used)) are deliberately left alone — a
    /// sale happens *mid*-round and must not reset it.
    pub fn sell_joker(&mut self, index: usize) -> Option<BuffoonCard> {
        if index >= self.jokers.len() {
            return None;
        }
        let joker = self.remove_joker(index);
        self.money = self
            .money
            .saturating_add(isize::try_from(joker.resell_value).unwrap_or(0));
        if matches!(joker.enhancement, MPip::DisableBossBlindOnSell) {
            self.boss_disabled = true;
        }
        self.recompute_draws();
        Some(joker)
    }

    // ---- Shop (EPIC-01b Phase 2) -----------------------------------------

    /// What a stock card costs to buy.
    ///
    /// Tarots and Planets are a flat **$3** (Balatro's base consumable price);
    /// every joker is priced by its [`rank.value`](crate::prelude::Pip::value),
    /// the same number [`sell_joker`](Self::sell_joker) halves for the resale.
    #[must_use]
    fn stock_price(card: BuffoonCard) -> usize {
        match card.card_type {
            BCardType::Tarot | BCardType::Planet => 3,
            _ => card.rank.value,
        }
    }

    /// Draw one joker at the shop's rarity odds — **70% Common / 25% Uncommon /
    /// 5% Rare**, Legendary never. Every pick comes from the rarity piles the
    /// 2026-07-16 sweep made a trustworthy partition, so a drawn joker is always
    /// a piled one — never a parallel catalog. Shared by the card slots and by a
    /// Buffoon pack's choices.
    fn draw_shop_joker<R: Rng + ?Sized>(rng: &mut R) -> BuffoonCard {
        let rarity = rng.random_range(0..100);
        let pool: &[BuffoonCard] = if rarity < 70 {
            &Joker::COMMON_JOKERS
        } else if rarity < 95 {
            &Joker::UNCOMMON_JOKERS
        } else {
            &Joker::RARE_JOKERS
        };
        pool[rng.random_range(0..pool.len())]
    }

    /// The shop's card-slot weights `(joker, tarot, planet)`, read **live**.
    ///
    /// Base **20 / 4 / 4**; a Tarot Merchant doubles the tarot band and a Tarot
    /// Tycoon quadruples it (the Tycoon requires the Merchant and supersedes it,
    /// so the multiplier is 1/2/4, not stacked), and the same for planets. The
    /// joker band and the rarity partition inside it are untouched — only the
    /// consumable bands move.
    fn stock_weights(&self) -> (usize, usize, usize) {
        let mult = |merchant, tycoon| {
            if self.vouchers.contains(&tycoon) {
                4
            } else if self.vouchers.contains(&merchant) {
                2
            } else {
                1
            }
        };
        let tarot = 4 * mult(Voucher::TarotMerchant, Voucher::TarotTycoon);
        let planet = 4 * mult(Voucher::PlanetMerchant, Voucher::PlanetTycoon);
        (20, tarot, planet)
    }

    /// Draw one card slot at the shop's [`stock_weights`](Self::stock_weights): a
    /// joker (then rolled through [`draw_shop_joker`](Self::draw_shop_joker)), a
    /// tarot, or a planet. With no Merchant/Tycoon voucher the weights are the
    /// base 20/4/4 out of 28, so an un-vouchered draw is byte-identical to before.
    fn draw_stock_card<R: Rng + ?Sized>(&self, rng: &mut R) -> BuffoonCard {
        let (joker, tarot, planet) = self.stock_weights();
        let total = joker + tarot + planet;
        let roll = rng.random_range(0..total);
        if roll < joker {
            Self::draw_shop_joker(rng)
        } else if roll < joker + tarot {
            MajorArcana::DECK[rng.random_range(0..MajorArcana::DECK.len())]
        } else {
            Planet::DECK[rng.random_range(0..Planet::DECK.len())]
        }
    }

    /// Draw one booster-pack slot: a uniformly-chosen [`PackKind`] at the base
    /// **$4** tier.
    ///
    /// Balatro's real pack-appearance weights differ by kind and tier; this
    /// engine draws the three base packs it can fill (Buffoon / Arcana /
    /// Celestial) with equal odds, which is enough for a run loop to spend in.
    fn draw_pack<R: Rng + ?Sized>(rng: &mut R) -> BoosterPack {
        let kind = match rng.random_range(0..3) {
            0 => PackKind::Buffoon,
            1 => PackKind::Arcana,
            _ => PackKind::Celestial,
        };
        BoosterPack { kind, cost: 4 }
    }

    /// Open the [`Shop`], drawing its two card slots and two pack slots at the
    /// wiki weights.
    ///
    /// There is deliberately **no pure `open_shop`** — a shop without RNG has no
    /// stock to draw, exactly as [`on_blind_selected_with_rng`](Self::on_blind_selected_with_rng)
    /// exists for Riff-Raff. A fresh shop has rerolled nothing.
    pub fn open_shop_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let slots = 2 + self.overstock_bonus();
        let stock = (0..slots).map(|_| self.draw_stock_card(rng)).collect();
        let packs = vec![Self::draw_pack(rng), Self::draw_pack(rng)];
        let eligible = self.eligible_vouchers();
        let voucher = if eligible.is_empty() {
            None
        } else {
            Some(eligible[rng.random_range(0..eligible.len())])
        };
        self.shop = Some(Shop {
            stock,
            packs,
            voucher,
            rerolls_used: 0,
        });
    }

    /// Extra shop card slots from the Overstock vouchers, read **live** at open:
    /// +1 for Overstock, +1 more for Overstock Plus (which requires Overstock, so
    /// holding it means holding both). Unlike the board slots, there is no field
    /// to bump — the shop's card-slot count is computed fresh each open.
    fn overstock_bonus(&self) -> usize {
        self.vouchers
            .iter()
            .filter(|voucher| matches!(voucher, Voucher::Overstock | Voucher::OverstockPlus))
            .count()
    }

    /// The cap on interest earned at cash-out: **$5** base, **$10** with Seed
    /// Money, **$20** with Money Tree (which requires Seed Money).
    ///
    /// The single reader both interest sites share — the base cash-out interest
    /// ([`cash_out`](Self::cash_out)) and To the Moon's `ExtraInterest` payout —
    /// so the two can never disagree on the ceiling. Unifying them is what let
    /// Seed Money raise the cap in one place rather than two.
    fn interest_cap(&self) -> isize {
        if self.vouchers.contains(&Voucher::MoneyTree) {
            20
        } else if self.vouchers.contains(&Voucher::SeedMoney) {
            10
        } else {
            5
        }
    }

    /// How many dollars the Reroll vouchers take off a reroll: **$2** per
    /// Reroll Surplus / Reroll Glut held (Glut requires Surplus, so both = $4),
    /// read live. The caller floors the cost at $0.
    fn reroll_discount(&self) -> usize {
        2 * self
            .vouchers
            .iter()
            .filter(|voucher| matches!(voucher, Voucher::RerollSurplus | Voucher::RerollGlut))
            .count()
    }

    /// `price` after the shop-discount vouchers, floored at **$1** (never free):
    /// **25% off** with Clearance Sale, **50% off** with Liquidation (which
    /// requires Clearance Sale and supersedes it — the discounts do not stack).
    /// Applies to cards and packs; the $10 voucher price is not discounted.
    fn discounted(&self, price: usize) -> usize {
        let pct = if self.vouchers.contains(&Voucher::Liquidation) {
            50
        } else if self.vouchers.contains(&Voucher::ClearanceSale) {
            25
        } else {
            0
        };
        ((price * (100 - pct)) / 100).max(1)
    }

    /// The vouchers the shop may offer: those **not yet redeemed** whose
    /// **base-tier prerequisite** (if any) is already held. Empty once every
    /// modelled voucher is redeemed — the shop then offers no voucher.
    fn eligible_vouchers(&self) -> Vec<Voucher> {
        Voucher::ALL
            .into_iter()
            .filter(|voucher| !self.vouchers.contains(voucher))
            .filter(|voucher| {
                voucher
                    .requires()
                    .is_none_or(|base| self.vouchers.contains(&base))
            })
            .collect()
    }

    /// Redeem the shop's offered voucher into [`vouchers`](Self::vouchers) for
    /// **$10**. Returns whether it happened.
    ///
    /// Refused — leaving the board untouched — when no voucher is offered, when
    /// it is already held, when its base-tier prerequisite is unmet, or when the
    /// $10 would drop [`money`](Self::money) below the debt floor (`buy_stock`'s
    /// floor, so a Credit Card lets a voucher go into debt too). On success the
    /// voucher joins the run and the slot is cleared — a voucher is redeemed once
    /// and never returns to the pool.
    ///
    /// The board-slot vouchers apply their **permanent** bump here: Crystal Ball
    /// grows [`consumable_slots`](Self::consumable_slots), Antimatter
    /// [`joker_slots`](Self::joker_slots). Unlike the Draws vouchers (recomputed
    /// live each blind), the slot fields have no recompute pass, and a redeem
    /// happens once and is guarded — so a one-time bump cannot stack. (Overstock
    /// is not here: it sizes the *shop's* card slots, read live at open, with no
    /// board field to bump.)
    pub fn redeem_shop_voucher(&mut self) -> bool {
        let Some(voucher) = self.shop.as_ref().and_then(|shop| shop.voucher) else {
            return false;
        };
        if self.vouchers.contains(&voucher) {
            return false;
        }
        if let Some(base) = voucher.requires() {
            if !self.vouchers.contains(&base) {
                return false;
            }
        }
        let price = 10;
        if self.money.saturating_sub(price) < self.debt_floor() {
            return false;
        }
        self.money = self.money.saturating_sub(price);
        self.vouchers.push(voucher);
        match voucher {
            Voucher::CrystalBall => self.consumable_slots += 1,
            Voucher::Antimatter => self.joker_slots += 1,
            _ => {}
        }
        if let Some(shop) = self.shop.as_mut() {
            shop.voucher = None;
        }
        true
    }

    /// The lowest [`money`](Self::money) a purchase may leave the board at.
    ///
    /// **$0** normally; each **Credit Card** held lowers it by its
    /// `MPip::Credit(n)` allowance (base $20), read **live** from `jokers` — the
    /// Chicot pattern, so selling the card restores the floor with no stored
    /// flag. Two Credit Cards stack, as in Balatro.
    #[must_use]
    fn debt_floor(&self) -> isize {
        let credit: usize = self
            .jokers
            .iter()
            .filter_map(|joker| match joker.enhancement {
                MPip::Credit(n) => Some(n),
                _ => None,
            })
            .sum();
        -isize::try_from(credit).unwrap_or(0)
    }

    /// Buy the stock at `index`, routing it onto the board. Returns whether the
    /// purchase happened.
    ///
    /// Refused — leaving the board untouched — when there is no such slot, when
    /// the price would drop [`money`](Self::money) below the debt floor (`$0`,
    /// or lower while a Credit Card is held), or when the destination is full: a
    /// joker needs [`has_joker_room`](Self::has_joker_room), a consumable a slot
    /// from [`create_consumable`](Self::create_consumable). The card is placed
    /// first and charged only once it lands, so a refusal for room never spends
    /// money.
    ///
    /// A bought joker goes through [`push_joker`](Self::push_joker), **not**
    /// [`add_card_to_deck`](Self::add_card_to_deck): it is not a playing card
    /// joining the deck, so no `CardAdded` fires and Hologram stays still.
    pub fn buy_stock(&mut self, index: usize) -> bool {
        let Some(card) = self
            .shop
            .as_ref()
            .and_then(|shop| shop.stock.get(index).copied())
        else {
            return false;
        };
        let price = isize::try_from(self.discounted(Self::stock_price(card))).unwrap_or(isize::MAX);
        if self.money.saturating_sub(price) < self.debt_floor() {
            return false;
        }
        let placed = if card.is_joker() {
            if self.has_joker_room() {
                self.push_joker(card);
                true
            } else {
                false
            }
        } else {
            self.create_consumable(card)
        };
        if !placed {
            return false;
        }
        self.money = self.money.saturating_sub(price);
        if let Some(shop) = self.shop.as_mut() {
            shop.stock.remove(index);
        }
        true
    }

    /// How many free rerolls the board is granted this shop — the sum of every
    /// held `MPip::FreeReroll(n)` (Chaos the Clown's `1`), read **live** so two
    /// Chaos grant two, and selling one gives its free reroll back.
    #[must_use]
    fn free_rerolls(&self) -> usize {
        self.jokers
            .iter()
            .filter_map(|joker| match joker.enhancement {
                MPip::FreeReroll(n) => Some(n),
                _ => None,
            })
            .sum()
    }

    /// What the next reroll of the shop's card slots costs.
    ///
    /// The shop's free rerolls (one per held `MPip::FreeReroll`, i.e. Chaos the
    /// Clown) cost **$0**; each paid reroll after them starts at **$5** and
    /// climbs **$1** apiece. The count resets every time
    /// [`open_shop_with_rng`](Self::open_shop_with_rng) draws a fresh shop, since
    /// a new `Shop` starts at `rerolls_used == 0`.
    #[must_use]
    pub fn reroll_cost(&self) -> usize {
        let free = self.free_rerolls();
        let used = self.shop.as_ref().map_or(0, |shop| shop.rerolls_used);
        let base = if used < free { 0 } else { 5 + (used - free) };
        base.saturating_sub(self.reroll_discount())
    }

    /// Reroll the shop's card slots, paying [`reroll_cost`](Self::reroll_cost)
    /// and redrawing the two card slots. Returns whether it happened.
    ///
    /// Refused — untouched — with no shop open, or when the cost would drop
    /// [`money`](Self::money) below the debt floor (a free reroll is always
    /// affordable). Only the card slots are redrawn; a future pack slot is left
    /// alone. Fires the `ShopRerolled` growth event, which is where **Flash
    /// Card** gains its `+2` mult.
    pub fn reroll_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) -> bool {
        if self.shop.is_none() {
            return false;
        }
        let cost = isize::try_from(self.reroll_cost()).unwrap_or(isize::MAX);
        if self.money.saturating_sub(cost) < self.debt_floor() {
            return false;
        }
        self.money = self.money.saturating_sub(cost);
        // Redraw the same number of card slots the shop offers — Overstock
        // widens the reroll too, matching `open_shop_with_rng`.
        let slots = 2 + self.overstock_bonus();
        let stock = (0..slots).map(|_| self.draw_stock_card(rng)).collect();
        if let Some(shop) = self.shop.as_mut() {
            shop.stock = stock;
            shop.rerolls_used += 1;
        }
        self.apply_growth(&GrowthEvent::ShopRerolled);
        true
    }

    /// Skip the booster pack at `index`, taking it off the shop for free.
    /// Returns whether there was a pack to skip.
    ///
    /// Fires the `PackSkipped` growth event — where **Red Card** gains its `+3`
    /// mult. Skipping costs nothing (unlike opening); it is the free way to
    /// clear a pack slot.
    pub fn skip_pack(&mut self, index: usize) -> bool {
        let present = self
            .shop
            .as_ref()
            .is_some_and(|shop| index < shop.packs.len());
        if !present {
            return false;
        }
        if let Some(shop) = self.shop.as_mut() {
            shop.packs.remove(index);
        }
        self.apply_growth(&GrowthEvent::PackSkipped);
        true
    }

    /// Open the booster pack at `index`, paying its cost and returning the
    /// choices it offers — jokers for a Buffoon pack, tarots for Arcana, planets
    /// for Celestial. `None` if there is no such pack or the cost would drop
    /// [`money`](Self::money) below the debt floor.
    ///
    /// The returned cards are the pack's *offer*; placing the player's pick is
    /// the caller's, through the same [`push_joker`](Self::push_joker) /
    /// [`create_consumable`](Self::create_consumable) seams buying uses — a full
    /// choose-and-place flow is a feature of its own and is not built here.
    ///
    /// **Hallucination** fires as a side effect: for each one held, a rolled
    /// `1-in-2` (scaled by the board's shared odds seam, so Oops! All 6s doubles
    /// it) creates a Tarot when there is consumable room.
    pub fn open_pack_with_rng<R: Rng + ?Sized>(
        &mut self,
        index: usize,
        rng: &mut R,
    ) -> Option<Vec<BuffoonCard>> {
        let pack = self
            .shop
            .as_ref()
            .and_then(|shop| shop.packs.get(index).copied())?;
        let cost = isize::try_from(self.discounted(pack.cost)).unwrap_or(isize::MAX);
        if self.money.saturating_sub(cost) < self.debt_floor() {
            return None;
        }
        self.money = self.money.saturating_sub(cost);
        if let Some(shop) = self.shop.as_mut() {
            shop.packs.remove(index);
        }
        let choices = Self::draw_pack_choices(pack.kind, rng);
        self.hallucinate(rng);
        Some(choices)
    }

    /// The cards a pack of `kind` offers: two jokers for a Buffoon pack, three
    /// tarots for Arcana, three planets for Celestial — the base-tier choice
    /// counts, drawn from the same piles and decks the shop stocks.
    fn draw_pack_choices<R: Rng + ?Sized>(kind: PackKind, rng: &mut R) -> Vec<BuffoonCard> {
        match kind {
            PackKind::Buffoon => (0..2).map(|_| Self::draw_shop_joker(rng)).collect(),
            PackKind::Arcana => (0..3)
                .map(|_| MajorArcana::DECK[rng.random_range(0..MajorArcana::DECK.len())])
                .collect(),
            PackKind::Celestial => (0..3)
                .map(|_| Planet::DECK[rng.random_range(0..Planet::DECK.len())])
                .collect(),
        }
    }

    /// Roll every held Hallucination's tarot chance for one pack opening.
    ///
    /// Each `MPip::CreateTarotOnPackOpen(num, den)` rolls `num`-in-`den`, scaled
    /// by [`probability_numerator`](Self::probability_numerator) so Oops! All 6s
    /// doubles it (capped at certainty); a win creates a random Tarot when there
    /// is consumable room, refusing silently when there is not — the "(Must have
    /// room)" clause [`create_consumable`](Self::create_consumable) already
    /// enforces. Handled inline rather than through the growth seam because it is
    /// an immediate creation, not a counter (the Riff-Raff pattern).
    fn hallucinate<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let scale = self.probability_numerator();
        let rolls: Vec<(usize, usize)> = self
            .jokers
            .iter()
            .filter_map(|joker| match joker.enhancement {
                MPip::CreateTarotOnPackOpen(num, den) => Some((num, den)),
                _ => None,
            })
            .collect();
        for (num, den) in rolls {
            if den == 0 {
                continue;
            }
            let wins = num.saturating_mul(scale).min(den);
            if rng.random_range(0..den) < wins && self.has_consumable_room() {
                let tarot = MajorArcana::DECK[rng.random_range(0..MajorArcana::DECK.len())];
                self.create_consumable(tarot);
            }
        }
    }

    /// The playing card a `BCardType` names, or `None` if this engine has no
    /// canonical one for it.
    ///
    /// Only Stone is mintable today, which is all Marble Joker needs. Returning
    /// `None` for the rest is deliberate: minting an arbitrary stand-in would be
    /// a wrong card, which is worse than adding nothing.
    fn mint_card(card_type: BCardType) -> Option<BuffoonCard> {
        match card_type {
            BCardType::Stone => Some(basic::card::STONE_CARD),
            _ => None,
        }
    }

    /// The joker pool a rarity draws from, or `None` if the `BCardType` is not a
    /// rarity.
    fn joker_pool(rarity: BCardType) -> Option<&'static [BuffoonCard]> {
        match rarity {
            BCardType::CommonJoker => Some(&Joker::COMMON_JOKERS),
            BCardType::UncommonJoker => Some(&Joker::UNCOMMON_JOKERS),
            BCardType::RareJoker => Some(&Joker::RARE_JOKERS),
            BCardType::LegendaryJoker => Some(&Joker::LEGENDARY_JOKERS),
            _ => None,
        }
    }

    /// Everything [`on_blind_selected`](Self::on_blind_selected) does, then the
    /// random blind-select effects:
    ///
    /// * **Madness** destroys one random *other* joker, on a Small or Big Blind
    ///   only. Its ×0.5 gain is not here — that is deterministic and already
    ///   applied by the pure hook, because Balatro grants it whether or not
    ///   anything was destroyed.
    /// * **Riff-Raff** draws 2 Common Jokers from the rarity pool, stopping at
    ///   [`joker_slots`](Self::joker_slots) — checked per joker, so a board with
    ///   one free slot gets one of the two and a full board gets none, which is
    ///   Balatro's "(Must have room)".
    ///
    /// Madness runs first: it frees a slot, and Riff-Raff can then fill it.
    pub fn on_blind_selected_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.on_blind_selected();
        self.madness_destroys_a_joker(rng);

        let creations: Vec<(usize, BCardType)> = self
            .jokers
            .iter()
            .filter_map(|joker| match joker.enhancement {
                MPip::CreateJokersWhenBlindSelected(n, rarity) => Some((n, rarity)),
                _ => None,
            })
            .collect();
        for (count, rarity) in creations {
            let Some(pool) = Self::joker_pool(rarity) else {
                continue;
            };
            if pool.is_empty() {
                continue;
            }
            for _ in 0..count {
                if !self.has_joker_room() {
                    break;
                }
                let pick = pool[rng.random_range(0..pool.len())];
                self.push_joker(pick);
            }
        }
    }

    /// Each Madness on the board destroys one random joker — never itself, and
    /// never on a Boss Blind.
    ///
    /// Victims are picked one Madness at a time, re-reading the board each pass,
    /// so two Madnesses cannot both target the same slot and a Madness can eat
    /// another Madness (as in Balatro). If the board holds nothing else, nothing
    /// is destroyed and the ×0.5 the pure hook already granted still stands.
    fn madness_destroys_a_joker<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        if self.blind.is_boss() {
            return;
        }
        let is_madness = |joker: &BuffoonCard| {
            matches!(
                joker.enhancement,
                MPip::GainMultTimesOnNonBossBlindDestroyingJoker(_)
            )
        };
        let sources: Vec<usize> = self
            .jokers
            .iter()
            .enumerate()
            .filter(|(_, joker)| is_madness(joker))
            .map(|(slot, _)| slot)
            .collect();
        if sources.is_empty() {
            return;
        }

        // Resolve against the board's **original** slots and apply the removals
        // once at the end. Destroying as we go would shift the indices under the
        // later sources, and a Madness eaten by an earlier one would still get a
        // turn it should not have.
        let mut alive: Vec<usize> = (0..self.jokers.len()).collect();
        for source in sources {
            if !alive.contains(&source) {
                continue; // an earlier Madness already ate this one
            }
            let victims: Vec<usize> = alive.iter().copied().filter(|s| *s != source).collect();
            if victims.is_empty() {
                continue; // it is alone, and cannot destroy itself
            }
            let victim = victims[rng.random_range(0..victims.len())];
            alive.retain(|slot| *slot != victim);
        }

        for slot in (0..self.jokers.len()).rev() {
            if !alive.contains(&slot) {
                self.remove_joker(slot);
            }
        }
    }

    /// The round's **Cash Out** income, or `0` for a round that was not won.
    ///
    /// Balatro's three cash-out lines, at their wiki values:
    ///
    /// * **blind reward** — $3 / $4 / $5 for Small / Big / Boss;
    /// * **$1 per unused hand** — [`hands_remaining`](Self::hands_remaining);
    /// * **interest** — $1 per full $5 held, capped at $5, so money above $25
    ///   earns nothing and debt earns nothing.
    ///
    /// **Gated on [`round_is_won`](Self::round_is_won)** — all three lines or
    /// none. In Balatro you cash out by *beating* a blind; a round that merely
    /// runs out of hands is a loss, and losses pay nothing. Since `round_is_won`
    /// is false whenever [`blind_target`](Self::blind_target) is 0, an
    /// untargeted round — the mode every board ran in before the shop existed —
    /// is unaffected, and cash-out is opt-in through the target that ante
    /// progression will one day set.
    ///
    /// Takes `&self` and returns the delta rather than paying itself, so
    /// [`on_round_end`](Self::on_round_end) can apply it against the **same
    /// pre-cash-out balance** the `+$` jokers are paid from — see the ordering
    /// note there.
    fn cash_out(&self) -> isize {
        if !self.round_is_won() {
            return 0;
        }
        let reward: isize = match self.blind {
            Blind::Small => 3,
            Blind::Big => 4,
            Blind::Boss(_) => 5,
        };
        let per_hand = isize::try_from(self.hands_remaining()).unwrap_or(isize::MAX);
        // The same shape as To the Moon's `ExtraInterest` steps in
        // `payout_delta`, deliberately: they are the same rule, and the clamp's
        // lower bound is what keeps debt from charging negative interest; the
        // upper bound is the voucher-raised cap (Seed Money / Money Tree).
        let interest = (self.money / 5).clamp(0, self.interest_cap());
        reward.saturating_add(per_hand).saturating_add(interest)
    }

    /// End-of-round lifecycle, the deterministic half: tick the round counters
    /// (Popcorn's decay, Rocket's boss tally), pay the round-end `+$` jokers and
    /// the round's cash-out into [`money`](Self::money), grow each Egg's resell
    /// value, destroy anything the decay emptied, and reset the round's
    /// counters. Inert on a board without those jokers.
    ///
    /// The order is load-bearing at three points:
    ///
    /// * **Growth before payouts** — Rocket's increment for defeating a Boss
    ///   Blind lands *before* the payout of the round that defeated it, so the
    ///   boss round pays the already-raised amount. Nothing else reads a counter
    ///   to pay, so nothing else notices.
    /// * **Cash-out from the pre-event balance** — the round's cash-out (blind
    ///   reward, $1 per unused hand, interest — gated on
    ///   [`round_is_won`](Self::round_is_won)) is computed at the top and
    ///   applied after the payouts, so its interest line and To the Moon's
    ///   `ExtraInterest` read the *same* money the round was walked into with.
    ///   Balatro's cash-out screen computes every line from that one balance;
    ///   paying either first would let the two compound off each other. This is
    ///   the same rule the `+$` payouts already follow internally — cash-out is
    ///   a third reader of it, not a new one.
    /// * **Payouts before destruction** — the cash-out-then-cleanup order
    ///   [`on_round_end_with_rng`](Self::on_round_end_with_rng) also uses for its
    ///   rolls: a joker that both pays and dies this round still pays.
    ///
    /// The probabilistic half — the joker destruction rolls (Gros Michel,
    /// Cavendish) — lives in
    /// [`on_round_end_with_rng`](Self::on_round_end_with_rng), mirroring the
    /// `score`/`score_with_rng` split: with no RNG the rolls are simply
    /// skipped, the way a Lucky card stays inert in the pure [`score`](Self::score).
    pub fn on_round_end(&mut self) {
        // Read before anything mutates `money`; applied below, after the payouts
        // have read that same balance.
        let cash_out = self.cash_out();
        self.apply_growth(&GrowthEvent::RoundEnd);
        self.apply_payouts(&GrowthEvent::RoundEnd);
        self.money = self.money.saturating_add(cash_out);
        self.melt_emptied_jokers();
        // Egg: its own resell value grows in place, every round.
        for index in 0..self.jokers.len() {
            let Some(joker) = self.jokers.get(index).copied() else {
                continue;
            };
            if let MPip::SellValueIncrement(n) = joker.enhancement {
                let mut grown = joker;
                grown.resell_value = grown.resell_value.saturating_add(n);
                self.jokers.remove(index);
                self.jokers.insert(index, grown);
            }
        }
        self.discards_used = 0;
        self.hands_played = 0;
        self.hands_by_type_this_round.clear();
        self.round_score = 0;
    }

    /// Everything [`on_round_end`](Self::on_round_end) does, then the
    /// destruction pass: each joker carrying a destruction chance
    /// ([`MPip::MultPlusChanceDestroyed`], [`MPip::MultTimesChanceDestroyed`],
    /// or a bare [`MPip::ChanceDestroyed`]) rolls its
    /// `numerator`-in-`denominator`, scaled through the board's shared odds
    /// seam (`probability_numerator`) so Oops! All 6s doubles it, capped at
    /// certainty.
    ///
    /// Payouts land before destruction — Balatro's cash-out-then-cleanup
    /// order, so a hypothetical paying self-destroyer would still pay the
    /// round it dies. Destroyed indices are collected first and removed in
    /// reverse via [`remove_joker`](Self::remove_joker), which keeps
    /// `joker_state` aligned.
    pub fn on_round_end_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.on_round_end();
        let scale = self.probability_numerator();
        let destroyed: Vec<usize> = self
            .jokers
            .iter()
            .enumerate()
            .filter_map(|(index, joker)| {
                let (MPip::ChanceDestroyed(numerator, denominator)
                | MPip::MultPlusChanceDestroyed(_, numerator, denominator)
                | MPip::MultTimesChanceDestroyed(_, numerator, denominator)) = joker.enhancement
                else {
                    return None;
                };
                if denominator == 0 {
                    return None;
                }
                let wins = numerator.saturating_mul(scale).min(denominator);
                (rng.random_range(0..denominator) < wins).then_some(index)
            })
            .collect();
        for index in destroyed.into_iter().rev() {
            self.remove_joker(index);
        }
        self.reroll_ancient_suit(rng);
    }

    /// Re-roll [`ancient_suit`](Self::ancient_suit) — Ancient Joker's "suit
    /// changes at end of round".
    ///
    /// The new suit is drawn from the three that are **not** the current one, so
    /// it can never repeat back to back. The first roll is the exception: with
    /// no suit yet the pool is all four, which is exactly how Balatro seeds it at
    /// run start.
    ///
    /// **Gated on holding an Ancient Joker**, which is a deliberate deviation.
    /// Balatro rolls the suit every round whether or not you hold one. Rolling
    /// unconditionally here would consume RNG on every board and shift every
    /// other seeded roll downstream (Gros Michel's 1-in-6, Cavendish's
    /// 1-in-1000), changing results for boards that have nothing to do with this
    /// joker. Nothing but Ancient Joker reads the suit, so gating is
    /// unobservable — the only difference is *which* suit a joker acquired
    /// mid-run starts on, and that is a fresh draw either way.
    fn reroll_ancient_suit<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        const SUITS: [char; 4] = ['S', 'H', 'C', 'D'];
        let holds_ancient = self
            .jokers
            .iter()
            .any(|joker| matches!(joker.enhancement, MPip::MultTimesPerScoredAncientSuit(_)));
        if !holds_ancient {
            return;
        }
        let pool: Vec<char> = SUITS
            .into_iter()
            .filter(|suit| Some(*suit) != self.ancient_suit)
            .collect();
        self.ancient_suit = Some(pool[rng.random_range(0..pool.len())]);
    }

    /// Money a joker pays for one lifecycle event — the cash mirror of
    /// [`growth_delta`](Self::growth_delta); returns 0 for every non-cash
    /// joker. Takes `&self` because payouts read board state: Cloud 9 counts
    /// the full deck, To the Moon reads `money`, Delayed Gratification reads
    /// the round's discard usage, and Faceless Joker classifies the discarded
    /// cards through [`is_face_card`](Self::is_face_card) — so Pareidolia
    /// amplifies it, as in Balatro.
    fn payout_delta(&self, enhancement: MPip, event: &GrowthEvent, counter: i32) -> isize {
        let cash = |n: usize| isize::try_from(n).unwrap_or(isize::MAX);
        match (enhancement, event) {
            // Golden Joker: a flat $n every round.
            (MPip::CashOnRoundEnd(n), GrowthEvent::RoundEnd) => cash(n),
            // Rocket: $base, raised by $increase per Boss Blind defeated. The
            // counter is grown before payouts run, so the boss round itself pays
            // the already-raised amount — Balatro's order.
            (MPip::CashOnRoundEndGrowingOnBossDefeat(base, increase), GrowthEvent::RoundEnd) => {
                let bosses = usize::try_from(counter.max(0)).unwrap_or(0);
                cash(base.saturating_add(increase.saturating_mul(bosses)))
            }
            // Delayed Gratification: $n per remaining discard, forfeited the
            // moment any discard is used this round. Reads `discards_remaining`
            // like its siblings — equivalent here (the payout only survives when
            // nothing has been used, where granted and remaining coincide), but
            // it keeps `draws.discards` from having any "remaining" readers left
            // to imitate.
            (MPip::CashPerDiscardIfNoneUsed(n), GrowthEvent::RoundEnd) => {
                if self.discards_used == 0 {
                    cash(n * self.discards_remaining())
                } else {
                    0
                }
            }
            // Cloud 9: $n per matching rank in the full deck — the roster, so
            // destroyed 9s stop paying and added ones start.
            (MPip::CashPerFullDeckRank(n, rank), GrowthEvent::RoundEnd) => {
                let count = self
                    .full_deck
                    .iter()
                    .filter(|card| card.rank.index == rank)
                    .count();
                cash(n * count)
            }
            // To the Moon: $n extra interest per full $5 held, capped at the
            // same [`interest_cap`](Self::interest_cap) the base interest reads
            // (so Seed Money raises both together); debt earns nothing.
            (MPip::ExtraInterest(n), GrowthEvent::RoundEnd) => {
                let steps = (self.money / 5).clamp(0, self.interest_cap());
                cash(n).saturating_mul(steps)
            }
            // Faceless Joker: $cash when enough faces go in a single discard.
            (MPip::CashOnFacesDiscarded(payout, min_faces), GrowthEvent::Discard(discarded)) => {
                let faces = discarded
                    .iter()
                    .filter(|card| self.is_face_card(card))
                    .count();
                if faces >= min_faces { cash(payout) } else { 0 }
            }
            _ => 0,
        }
    }

    /// Pay every joker's cash for one lifecycle event into
    /// [`money`](Self::money). Every delta is computed against the **same**
    /// board and applied as one sum, so joker order cannot matter — in
    /// particular, To the Moon's interest reads the money held *before* this
    /// round's payouts land, matching Balatro's cash-out screen, where every
    /// line is computed from the same starting balance.
    ///
    /// Each joker's counter is passed alongside its enhancement, since Rocket
    /// pays out of one. Those counters have already been grown for this event by
    /// the time this runs — see [`on_round_end`](Self::on_round_end), where that
    /// order is deliberate and is what makes a boss round pay Rocket's raised
    /// amount rather than its previous one.
    fn apply_payouts(&mut self, event: &GrowthEvent) {
        let total: isize = self
            .jokers
            .iter()
            .enumerate()
            .map(|(slot, joker)| {
                let counter = self.joker_state.get(slot).copied().unwrap_or(0);
                self.payout_delta(joker.enhancement, event, counter)
            })
            .sum();
        self.money = self.money.saturating_add(total);
    }

    /// Whether a decaying joker's resource has been fully consumed, given its
    /// accumulator — Ice Cream's chips (`base − per × hands`), Popcorn's mult
    /// (`base − per × rounds`), or Seltzer's retriggers (`hands − 1 × hands
    /// played`). `None` for every joker that does not decay.
    ///
    /// All three are one shape — a resource spent at a fixed rate per event —
    /// and all three are destroyed at 0 in Balatro, so the rule lives here once
    /// and each hook calls [`melt_emptied_jokers`](Self::melt_emptied_jokers)
    /// after growing its own event's counter.
    fn is_decayed_to_nothing(enhancement: MPip, counter: i32) -> Option<bool> {
        let (base, per) = match enhancement {
            MPip::LoseChipsPerHand(base, per) | MPip::LoseMultPerRound(base, per) => (base, per),
            // Seltzer spends one of its `hands` per hand played.
            MPip::RetriggerAllPlayedForHands(_, hands) => (hands, 1),
            _ => return None,
        };
        let ticks = usize::try_from(counter).unwrap_or(0);
        Some(base.saturating_sub(per.saturating_mul(ticks)) == 0)
    }

    /// Remove every decaying joker whose decay has consumed its base: Ice Cream
    /// (`LoseChipsPerHand`, emptied by hands played) and Popcorn
    /// (`LoseMultPerRound`, emptied by rounds ended). Each is destroyed **by the
    /// event that empties it** — Balatro's exact timing, which is why this runs
    /// from both [`on_hand_played`](Self::on_hand_played) and
    /// [`on_round_end`](Self::on_round_end) rather than from one of them: a
    /// joker is only ever emptied by its own event, so the other hook's call is
    /// a no-op for it.
    ///
    /// Slots are walked in reverse so a removal cannot shift an unprocessed
    /// index; [`remove_joker`](Self::remove_joker) keeps `joker_state` aligned.
    fn melt_emptied_jokers(&mut self) {
        for index in (0..self.jokers.len()).rev() {
            let Some(joker) = self.jokers.get(index) else {
                continue;
            };
            let counter = self.joker_state.get(index).copied().unwrap_or(0);
            if Self::is_decayed_to_nothing(joker.enhancement, counter) == Some(true) {
                self.remove_joker(index);
            }
        }
    }

    /// The pre-scoring pass: apply the card mutations that fire as the played
    /// hand scores, and grow the counters that the same hand then reads.
    ///
    /// Two jokers live here, and both need this to run *before* scoring:
    ///
    /// * **Hiker** (`MPip::GainChipsOnScored`) — every card in
    ///   [`played`](Self::played) permanently gains chips.
    /// * **Vampire** (`MPip::GainMultTimesPerEnhancedPlayed`) — gains ×0.1 per
    ///   enhanced played card and **strips** the enhancement off each. Because
    ///   the strip lands before the fold, the eaten enhancement does not score
    ///   on this hand (a Glass card gives neither its ×2 nor its break chance),
    ///   while the ×mult Vampire just gained does — which is exactly Balatro's
    ///   "removes Enhancements before their effect occurs".
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
        // Vampire counts the enhancements *before* anything eats them.
        let played = self.played.clone();
        self.apply_growth(&GrowthEvent::Scored(&played));

        let bump: usize = self
            .jokers
            .iter()
            .map(|joker| match joker.enhancement {
                MPip::GainChipsOnScored(n) => n,
                _ => 0,
            })
            .sum();
        let eats_enhancements = self
            .jokers
            .iter()
            .any(|joker| matches!(joker.enhancement, MPip::GainMultTimesPerEnhancedPlayed(_)));
        if bump == 0 && !eats_enhancements {
            return;
        }

        for index in 0..self.played.len() {
            let Some(card) = self.played.get(index).copied() else {
                continue;
            };
            let mut mutated = card.add_base_chips(bump);
            if eats_enhancements {
                mutated.enhancement = MPip::Blank;
            }
            if mutated == card {
                continue;
            }
            self.played.remove(index);
            self.played.insert(index, mutated);
            if let Some(slot) = self.full_deck_index_of(card) {
                self.replace_deck_card(slot, mutated);
            }
        }
    }

    /// The random half of [`on_scored`](Self::on_scored): the **Tarot creators**
    /// that fire on the hand being played.
    ///
    /// * **Superposition** — the hand is a Straight *and* holds an Ace.
    /// * **Vagabond** — the hand is played holding `$n` or less.
    ///
    /// Both draw a random Tarot from [`MajorArcana::DECK`], and both are subject
    /// to the free-slot rule through [`create_consumable`](Self::create_consumable),
    /// so a full inventory silently creates nothing — Balatro's "(Must have
    /// room)". Random, hence the `_with_rng` split: the pure
    /// [`on_scored`](Self::on_scored) leaves them inert, the way it leaves Lucky.
    ///
    /// Superposition reads the straight through the board's [`HandRules`], so
    /// Four Fingers and Shortcut widen what qualifies, as they do everywhere
    /// else.
    pub fn on_scored_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.on_scored();

        let rules = self.hand_rules();
        let is_ace_straight = self.played.has_straight_with(rules)
            && self.played.iter().any(|card| card.rank.index == 'A');
        let money = self.money;

        let creators: Vec<MPip> = self
            .jokers
            .iter()
            .filter(|joker| {
                matches!(
                    joker.enhancement,
                    MPip::CreateTarotOnAceStraight | MPip::CreateTarotOnLowMoney(_)
                )
            })
            .map(|joker| joker.enhancement)
            .collect();

        for enhancement in creators {
            let fires = match enhancement {
                MPip::CreateTarotOnAceStraight => is_ace_straight,
                MPip::CreateTarotOnLowMoney(limit) => {
                    money <= isize::try_from(limit).unwrap_or(isize::MAX)
                }
                _ => false,
            };
            if !fires || !self.has_consumable_room() {
                continue;
            }
            let pick = MajorArcana::DECK[rng.random_range(0..MajorArcana::DECK_SIZE)];
            self.create_consumable(pick);
        }
    }

    fn apply_growth(&mut self, event: &GrowthEvent) {
        self.ensure_state_len();
        let rules = self.hand_rules();
        let deltas: Vec<i32> = self
            .jokers
            .iter()
            .map(|j| self.growth_delta(j.enhancement, event, rules))
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
            // Spare Trousers gains +rate mult per two-pair hand; Flash Card per
            // reroll; Red Card per pack skipped. Different events grow the
            // counter (`growth_delta` keeps them apart), but the read is one
            // additive rule, so it is written once.
            MPip::GainMultPerTwoPairHand(rate)
            | MPip::MultPlusPerReroll(rate)
            | MPip::MultPlusPerPackSkipped(rate) =>
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
                let raw = (per as f32).mul_add(-(counter.max(0) as f32), base as f32) / 100.0;
                Some(ScoreOp::TimesMult(raw.max(1.0)))
            }
            MPip::LoseChipsPerHand(base, per) => {
                #[allow(clippy::cast_sign_loss)]
                let hands = counter.max(0) as usize;
                Some(ScoreOp::AddChips(base.saturating_sub(per * hands)))
            }
            // Popcorn: Ice Cream's decay on the mult side, per round rather
            // than per hand. Floors at 0 — the round that empties it also
            // destroys it (`melt_emptied_jokers`).
            MPip::LoseMultPerRound(base, per) => {
                #[allow(clippy::cast_sign_loss)]
                let rounds = counter.max(0) as usize;
                Some(ScoreOp::AddMult(base.saturating_sub(per * rounds)))
            }
            // Yorick: ×1 per 23 cards discarded. The accumulator counts cards,
            // so the factor steps only on each completed block of `per`.
            MPip::GainMultTimesPerDiscardedCards(rate, per) if per > 0 => {
                #[allow(clippy::cast_sign_loss)]
                let blocks = counter.max(0) as usize / per;
                Some(ScoreOp::TimesMult(Self::gain_x_mult(rate, blocks)))
            }
            // Hologram (×0.25 per card added to the deck), Canio (×1 per face
            // destroyed) and Vampire (×0.1 per enhanced card played) differ only
            // in which event grows them — `growth_delta` keeps them apart. The
            // *read* is one rule, so it is written once.
            MPip::GainMultTimesPerCardAdded(rate)
            | MPip::GainMultTimesPerFaceDestroyed(rate)
            | MPip::GainMultTimesPerEnhancedPlayed(rate)
            | MPip::GainMultTimesPerPlanetUsed(rate)
            | MPip::GainMultTimesOnNonBossBlindDestroyingJoker(rate) => {
                #[allow(clippy::cast_sign_loss)]
                let ticks = counter.max(0) as usize;
                Some(ScoreOp::TimesMult(Self::gain_x_mult(rate, ticks)))
            }
            _ => None,
        }
    }

    /// The ×mult factor of a "this joker gains ×`rate`/100 mult per event"
    /// counter that has ticked `count` times: `1 + (rate/100) × count`.
    ///
    /// **Additive, not compounding** — the Steel Joker rule
    /// ([`MPip::MultTimesPlusPerFullDeckSteel`]), which is what Balatro's
    /// "gains ×N Mult" jokers do: Hologram at four cards added is ×2, not
    /// ×0.25⁴. Base ×1 falls out of `count == 0`, so an ungrown counter joker
    /// is inert rather than zeroing the mult.
    #[allow(clippy::cast_precision_loss)]
    fn gain_x_mult(rate: usize, count: usize) -> f32 {
        (rate as f32 / 100.0).mul_add(count as f32, 1.0)
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
    use crate::funky::decks::planet::card as planet_card;
    use crate::funky::decks::tarot::card as tarot_card;
    use crate::funky::types::effect::{Effect, ScoreOp};
    use crate::funky::types::mpip::MPip;
    use crate::preludes::funky::{Blind, BossBlind};
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

        // 20+ hands -> the decay empties it, and the emptying hand melts the
        // joker away entirely (see on_hand_played__ice_cream_melts_at_zero_chips).
        for _ in 0..30 {
            board.on_hand_played(&hand);
        }
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn score__popcorn_loses_mult_per_round_played() {
        // Popcorn: +20 Mult, −4 for each round played. Ice Cream's decay on the
        // mult side, ticking per round rather than per hand.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::POPCORN);

        // No rounds played yet -> the full +20 mult.
        assert_eq!(board.score(), Score::new(40, 21));

        // One round -> +16.
        board.on_round_end();
        assert_eq!(board.score(), Score::new(40, 17));

        // Four rounds -> +4, its last scoring round.
        board.on_round_end();
        board.on_round_end();
        board.on_round_end();
        assert_eq!(board.score(), Score::new(40, 5));
    }

    #[test]
    fn on_round_end__popcorn_is_destroyed_by_the_round_that_empties_it() {
        // The Ice Cream rule on Popcorn's clock: 20 mult at −4 a round is spent
        // after exactly five rounds, and the round that spends it takes the
        // joker with it rather than leaving a +0 stub on the board.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::POPCORN);

        for _ in 0..4 {
            board.on_round_end();
        }
        assert_eq!(board.jokers.len(), 1, "still worth +4 after four rounds");

        board.on_round_end();
        assert!(
            board.jokers.is_empty(),
            "the fifth round empties it, so it is destroyed"
        );
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn score__yorick_gains_x_mult_every_twenty_three_cards_discarded() {
        // Yorick: gains ×1 Mult every 23 cards discarded; base ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::YORICK);

        // Ungrown -> ×1, i.e. inert rather than zeroing the mult.
        assert_eq!(board.score(), Score::new(40, 1));

        // 22 cards is short of the first block -> still ×1.
        for _ in 0..22 {
            board.on_discard(&bcards!("2C"));
        }
        assert_eq!(
            board.score(),
            Score::new(40, 1),
            "22 cards is short a block"
        );

        // The 23rd completes it -> ×2.
        board.on_discard(&bcards!("2C"));
        assert_eq!(board.score(), Score::new(40, 2));

        // 46 -> ×3: the factor is additive (1 + 1×blocks), not compounding.
        for _ in 0..23 {
            board.on_discard(&bcards!("2C"));
        }
        assert_eq!(board.score(), Score::new(40, 3));
    }

    #[test]
    fn score__yorick_counts_discarded_cards_not_discard_actions() {
        // The distinguishing case: one discard action carrying 23 cards is a
        // whole block on its own. Counting *actions* would leave this at ×1.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::YORICK);

        board.on_discard(&bcards!(
            "2C 3C 4C 5C 6C 7C 8C 9C TC JC QC KC AC 2D 3D 4D 5D 6D 7D 8D 9D TD JD"
        ));
        assert_eq!(board.score(), Score::new(40, 2));
    }

    #[test]
    fn score__hologram_gains_x_mult_per_card_added_to_the_deck() {
        // Hologram: gains ×0.25 Mult per playing card added to the deck; base ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::HOLOGRAM);

        // Ungrown -> ×1.
        assert_eq!(board.score(), Score::new(40, 1));

        // Four cards added -> ×2. The factor is **additive** (1 + 0.25×4), the
        // Steel Joker rule — compounding (×0.25⁴) would collapse the mult to 0
        // instead, so this value is what keeps the two apart.
        let seven = bcards!("7D").iter().next().copied().unwrap();
        for _ in 0..4 {
            board.add_card_to_deck(seven);
        }
        assert_eq!(board.score(), Score::new(40, 2));
    }

    #[test]
    fn score__hologram_grows_only_on_cards_added_not_replaced() {
        // `replace_deck_card` is a mutation, not an addition — the run owns the
        // same number of cards afterwards, so Hologram must not tick. This is
        // what stops Hiker's per-card bump silently feeding it.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::HOLOGRAM);

        let replacement = bcards!("7D").iter().next().copied().unwrap();
        assert!(board.replace_deck_card(0, replacement));
        assert_eq!(
            board.score(),
            Score::new(40, 1),
            "a replacement is not an add"
        );
    }

    #[test]
    fn score__canio_gains_x_mult_per_face_card_destroyed() {
        // Canio: gains ×1 Mult when a face card is destroyed; base ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::CANIO);

        // Ungrown -> ×1.
        assert_eq!(board.score(), Score::new(40, 1));

        // Destroying a *non*-face card leaves it alone.
        let seven = board
            .full_deck_index_of(bcards!("7D").iter().next().copied().unwrap())
            .expect("the basic deck holds a 7D");
        board.destroy_deck_card(seven);
        assert_eq!(board.score(), Score::new(40, 1), "a 7 is not a face");

        // Destroying a King -> ×2.
        let king = board
            .full_deck_index_of(bcards!("KS").iter().next().copied().unwrap())
            .expect("the basic deck holds a KS");
        board.destroy_deck_card(king);
        assert_eq!(board.score(), Score::new(40, 2));

        // A second face -> ×3, additive like its siblings.
        let queen = board
            .full_deck_index_of(bcards!("QS").iter().next().copied().unwrap())
            .expect("the basic deck holds a QS");
        board.destroy_deck_card(queen);
        assert_eq!(board.score(), Score::new(40, 3));
    }

    #[test]
    fn score__pareidolia_makes_every_destroyed_card_feed_canio() {
        // Canio classifies through the board's face predicate, so Pareidolia
        // ("all cards are face cards") makes even a destroyed 7 grow it — the
        // same amplification Pareidolia already gives Faceless Joker.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::CANIO);
        board.push_joker(card::PAREIDOLIA);

        let seven = board
            .full_deck_index_of(bcards!("7D").iter().next().copied().unwrap())
            .expect("the basic deck holds a 7D");
        board.destroy_deck_card(seven);
        assert_eq!(
            board.score(),
            Score::new(40, 2),
            "under Pareidolia a 7 is a face"
        );
    }

    #[test]
    fn score__vampire_gains_x_mult_per_enhanced_card_played_and_eats_it() {
        // Vampire: gains ×0.1 Mult per enhanced card played, removing the
        // enhancement. Base ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::VAMPIRE);

        // Two Bonus cards (+30 chips each) in the played hand.
        for index in 0..2 {
            let card = board.played.remove(index);
            board.played.insert(index, enhanced(card, MPip::BONUS));
        }

        // Ungrown and un-eaten, the Bonus chips are worth +60: 40 -> 100.
        assert_eq!(board.score(), Score::new(100, 1));

        // After Vampire eats them the +60 is gone — the enhancement is removed
        // *before* it can score. The chips are the observable half here; the
        // ×1.2 it gained rounds up to mult 2 off a base of 1.
        board.on_scored();
        assert_eq!(board.score(), Score::new(40, 2));
        assert_eq!(board.joker_state[0], 2, "it counted both enhanced cards");
        assert!(
            board.played.iter().all(|c| c.enhancement == MPip::Blank),
            "both enhancements are eaten off the cards themselves"
        );
    }

    #[test]
    fn score__vampire_x_mult_applies_to_the_hand_it_ate() {
        // The gain lands on the same hand it feeds on — that ordering is the
        // joker, and it is why Vampire grows on `Scored` rather than
        // `HandPlayed`. Base Joker (+4 mult) first, so the ×mult has something
        // big enough to scale visibly.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER); // +4 mult
        board.push_joker(card::VAMPIRE);

        for index in 0..5 {
            let card = board.played.remove(index);
            board.played.insert(index, enhanced(card, MPip::BONUS));
        }

        // Five enhanced cards eaten -> ×1.5, applied to this very hand:
        // mult 1 + 4 = 5, then ×1.5 -> ceil(7.5) = 8. Growing on `HandPlayed`
        // instead would leave this at mult 5.
        board.on_scored();
        assert_eq!(board.score(), Score::new(40, 8));
    }

    #[test]
    fn score__vampire_eats_glass_cards_before_they_can_multiply() {
        // The wiki's headline interaction: a Glass card Vampire eats gives
        // neither its ×2 mult nor (being enhancement-less) its chance to break.
        let mut board = board_playing("KH QD 2S 5D 8C");
        board.push_joker(card::VAMPIRE);

        for index in 0..2 {
            let card = board.played.remove(index);
            board
                .played
                .insert(index, enhanced(card, MPip::Glass(2, 4)));
        }

        // Two Glass cards each double the mult as they score: 1 × 2 × 2 = 4.
        assert_eq!(board.score(), Score::new(40, 4));

        // Vampire eats both before either multiplies, leaving only its own ×1.2
        // (ceil 2). Strip-after-scoring would leave this at 4.
        board.on_scored();
        assert_eq!(board.score(), Score::new(40, 2));
        assert!(
            board.played.iter().all(|c| c.enhancement == MPip::Blank),
            "both Glass enhancements are eaten"
        );
    }

    #[test]
    fn on_scored__vampire_leaves_plain_cards_and_a_plain_board_alone() {
        // A hand of unenhanced cards feeds it nothing, and a board without it
        // never strips anything — exit criterion 2 for the strip pass.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::VAMPIRE);
        board.on_scored();
        assert_eq!(board.joker_state[0], 0);
        assert_eq!(board.score(), Score::new(40, 1));

        let mut plain = board_playing("2S 5D 8C TS KH");
        let king = plain.played.remove(4);
        plain.played.insert(4, enhanced(king, MPip::BONUS));
        let before = plain.score();
        plain.on_scored();
        assert_eq!(plain.score(), before, "no Vampire, no strip");
    }

    #[test]
    fn create_consumable__refuses_once_both_slots_are_full() {
        // "(Must have room)" — a creator with a full inventory creates nothing;
        // it does not queue, and it does not evict.
        let mut board = board_playing("2S 5D 8C TS KH");
        assert!(board.create_consumable(tarot_card::JUSTICE));
        assert!(board.create_consumable(tarot_card::JUSTICE));
        assert!(
            !board.create_consumable(tarot_card::JUSTICE),
            "the base cap is two"
        );
        assert_eq!(board.consumables.len(), 2);

        // Spending one makes room again.
        board.use_consumable(0, &[]);
        assert!(board.create_consumable(tarot_card::JUSTICE));
    }

    #[test]
    fn use_consumable__planet_levels_its_hand_type() {
        let mut board = board_playing("KH KS 8C 5D 2S"); // a Pair
        let before = board.score();

        board.create_consumable(planet_card::MERCURY); // Pair: +15 chips, +1 mult
        assert_eq!(board.use_consumable(0, &[]), Some(planet_card::MERCURY));

        assert_eq!(
            board.score(),
            Score::new(before.chips + 15, before.mult + 1)
        );
        assert!(board.consumables.is_empty(), "it is spent, not kept");
    }

    #[test]
    fn use_consumable__tarot_enhances_its_targets_on_the_run_roster() {
        // A Tarot's enhancement lands through `replace_deck_card`, so it sticks
        // to the run's own copy rather than a temporary.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.create_consumable(tarot_card::JUSTICE); // Glass

        board.use_consumable(0, &[0, 1]);

        assert_eq!(
            board.full_deck.get(0).unwrap().enhancement,
            MPip::Glass(2, 4)
        );
        assert_eq!(
            board.full_deck.get(1).unwrap().enhancement,
            MPip::Glass(2, 4)
        );
        assert_eq!(board.tarots_used, 1, "one Tarot, however many targets");
    }

    #[test]
    fn score__constellation_gains_x_mult_per_planet_used() {
        // Constellation: gains ×0.1 Mult per Planet card used; base ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::JOKER); // +4 mult, so the ×mult scales visibly
        board.push_joker(card::CONSTELLATION);

        // Ungrown -> ×1: mult 1 + 4 = 5.
        assert_eq!(board.score(), Score::new(40, 5));

        // Five Planets used -> ×1.5. Mercury levels Pair, a hand this High Card
        // board never scores, so only Constellation's growth moves the number.
        for _ in 0..5 {
            board.create_consumable(planet_card::MERCURY);
            board.use_consumable(0, &[]);
        }
        assert_eq!(board.score(), Score::new(40, 8)); // ceil(5 × 1.5)
    }

    #[test]
    fn score__constellation_does_not_scale_retroactively() {
        // The counter/board-reader distinction, from Constellation's side: it is
        // a plain accumulator, so Planets spent before it arrived are worth
        // nothing to it. This is the exact opposite of Fortune Teller.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER);
        for _ in 0..5 {
            board.create_consumable(planet_card::MERCURY);
            board.use_consumable(0, &[]);
        }

        board.push_joker(card::CONSTELLATION);
        assert_eq!(
            board.score(),
            Score::new(40, 5),
            "still ×1 — it missed those five"
        );
    }

    #[test]
    fn score__fortune_teller_adds_mult_per_tarot_used_this_run() {
        // Fortune Teller: +1 Mult per Tarot card used this run.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::FORTUNE_TELLER);
        assert_eq!(board.score(), Score::new(40, 1), "no Tarots used yet");

        for _ in 0..3 {
            board.create_consumable(tarot_card::JUSTICE);
            board.use_consumable(0, &[]);
        }
        assert_eq!(board.score(), Score::new(40, 4)); // 1 + 3

        // Planets are not Tarots.
        board.create_consumable(planet_card::MERCURY);
        board.use_consumable(0, &[]);
        assert_eq!(board.score(), Score::new(40, 4));
    }

    #[test]
    fn score__fortune_teller_is_retroactive() {
        // The distinguishing case: it reads the run's Tarot tally off the board,
        // so one acquired after three Tarots is worth +3 the moment it lands. A
        // per-joker counter would start it at zero and read 40/1 here.
        let mut board = board_playing("2S 5D 8C TS KH");
        for _ in 0..3 {
            board.create_consumable(tarot_card::JUSTICE);
            board.use_consumable(0, &[]);
        }

        board.push_joker(card::FORTUNE_TELLER);
        assert_eq!(board.score(), Score::new(40, 4), "+3 immediately");
    }

    #[test]
    fn on_blind_selected__marble_joker_adds_a_stone_card_to_the_deck() {
        // Marble Joker: adds one Stone card to the deck when a Blind is
        // selected. It lands in the *deck* (undealt), not the hand.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MARBLE_JOKER);
        let deck_before = board.deck.len();
        let roster_before = board.full_deck.len();

        board.on_blind_selected();

        assert_eq!(board.deck.len(), deck_before + 1);
        assert_eq!(board.full_deck.len(), roster_before + 1);
        let added = board.full_deck.iter().last().copied().unwrap();
        assert_eq!(added.enhancement, MPip::TOWER, "it is a Stone card");
        assert!(added.is_stone());
        // "No rank or suit" is the *enhancement's* doing, not erased pips: the
        // base survives underneath and is masked. So the card still carries one
        // — what matters is that detection cannot see it, and that its chips are
        // the flat 50 rather than the base's value.
        assert_eq!(added.get_chips(), 50, "flat, not base + 50");
        let mut probe = bcards!("2C 3D");
        probe.push(added);
        assert_eq!(probe.detectable().len(), 2, "detection cannot see it");

        // A second blind adds a second, unlike the draw modifiers, which
        // recompute from a baseline rather than stacking.
        board.on_blind_selected();
        assert_eq!(board.full_deck.len(), roster_before + 2);
    }

    #[test]
    fn on_blind_selected__marble_joker_feeds_stone_joker_and_hologram() {
        // The reason adding a Stone card is worth anything today: the roster
        // count behind Stone Joker (+25 chips per Stone in the full deck) is
        // wired, and every added card feeds Hologram.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::MARBLE_JOKER);
        board.push_joker(card::STONE_JOKER);
        board.push_joker(card::HOLOGRAM);

        assert_eq!(board.score(), Score::new(40, 1), "no Stones yet");

        board.on_blind_selected();
        // Stone Joker: +25 chips for the one Stone. Hologram: ×1.25 -> ceil 2.
        assert_eq!(board.score(), Score::new(65, 2));
    }

    #[test]
    fn on_blind_selected_with_rng__riff_raff_creates_two_common_jokers() {
        // Riff-Raff: when a Blind is selected, create 2 Common Jokers.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::RIFF_RAFF);

        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(42));

        assert_eq!(board.jokers.len(), 3, "Riff-Raff plus the two it made");
        for created in board.jokers.iter().skip(1) {
            assert_eq!(
                created.card_type,
                BCardType::CommonJoker,
                "it creates Common Jokers"
            );
        }
        assert_eq!(
            board.joker_state.len(),
            board.jokers.len(),
            "created jokers get their own counter slot"
        );
    }

    #[test]
    fn on_blind_selected_with_rng__riff_raff_only_fills_the_room_it_has() {
        // "(Must have room)", checked per joker rather than all-or-nothing: with
        // one slot free it makes one, and with none it makes nothing.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::RIFF_RAFF);
        for _ in 0..3 {
            board.push_joker(card::JOKER);
        }
        assert_eq!(board.jokers.len(), 4, "one of the five slots is free");

        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.jokers.len(), 5, "it filled the one free slot");

        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(board.jokers.len(), 5, "a full board gets nothing");
    }

    #[test]
    fn on_blind_selected__is_inert_without_a_creator() {
        // Exit criterion 2 for the blind-select creators.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER);
        let before = board.clone();
        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(7));
        assert_eq!(board.deck.len(), before.deck.len());
        assert_eq!(board.full_deck.len(), before.full_deck.len());
        assert_eq!(board.jokers.len(), before.jokers.len());
        assert_eq!(board.score(), before.score());
    }

    #[test]
    fn on_scored_with_rng__superposition_needs_both_an_ace_and_a_straight() {
        // Superposition: create a Tarot if the hand contains an Ace *and* a
        // Straight — so only A-K-Q-J-T or A-2-3-4-5 qualify.
        let mut ace_straight = board_playing("AH KH QD JC TS");
        ace_straight.push_joker(card::SUPERPOSITION);
        ace_straight.on_scored_with_rng(&mut StdRng::seed_from_u64(3));
        assert_eq!(ace_straight.consumables.len(), 1, "Ace + Straight");
        assert_eq!(
            ace_straight.consumables.iter().next().unwrap().card_type,
            BCardType::Tarot
        );

        // A straight with no Ace: nothing.
        let mut no_ace = board_playing("9H KH QD JC TS");
        no_ace.push_joker(card::SUPERPOSITION);
        no_ace.on_scored_with_rng(&mut StdRng::seed_from_u64(3));
        assert!(no_ace.consumables.is_empty(), "a Straight but no Ace");

        // An Ace with no straight: nothing.
        let mut no_straight = board_playing("AH KH 8D 5C 2S");
        no_straight.push_joker(card::SUPERPOSITION);
        no_straight.on_scored_with_rng(&mut StdRng::seed_from_u64(3));
        assert!(no_straight.consumables.is_empty(), "an Ace but no Straight");
    }

    #[test]
    fn on_scored_with_rng__vagabond_creates_a_tarot_at_four_dollars_or_less() {
        // Vagabond: create a Tarot if a hand is played with $4 or less.
        for (money, expected) in [(0, 1), (4, 1), (5, 0)] {
            let mut board = board_playing("2S 5D 8C TS KH");
            board.money = money;
            board.push_joker(card::VAGABOND);
            board.on_scored_with_rng(&mut StdRng::seed_from_u64(9));
            assert_eq!(
                board.consumables.len(),
                expected,
                "${money} should {} a Tarot",
                if expected == 1 { "make" } else { "not make" }
            );
        }
    }

    #[test]
    fn on_scored_with_rng__a_tarot_creator_needs_a_free_consumable_slot() {
        // "(Must have room)" on the consumable side.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::VAGABOND);
        board.create_consumable(tarot_card::JUSTICE);
        board.create_consumable(tarot_card::JUSTICE);

        board.on_scored_with_rng(&mut StdRng::seed_from_u64(9));
        assert_eq!(board.consumables.len(), 2, "no room, so nothing is created");
    }

    #[test]
    fn on_scored__leaves_the_tarot_creators_inert_without_rng() {
        // The `score`/`score_with_rng` split, applied to creation: the pure hook
        // does not roll, the way a Lucky card stays inert in `score()`.
        let mut board = board_playing("AH KH QD JC TS");
        board.push_joker(card::SUPERPOSITION);
        board.on_scored();
        assert!(board.consumables.is_empty());
    }

    #[test]
    fn on_blind_selected__the_needle_leaves_one_hand_and_switches_dusk_on() {
        // The Needle plays only 1 hand, so the first hand *is* the final hand —
        // which is why Dusk always fires under it. The wiki calls this pairing
        // out explicitly, and it is the cheapest proof the boss ability is real.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::DUSK);
        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.on_blind_selected();

        assert_eq!(board.draws.hands_to_play, 1);
        // Every card retriggers on the very first hand: 40 -> 75.
        assert_eq!(board.score(), Score::new(75, 1));
    }

    #[test]
    fn on_blind_selected__the_water_leaves_no_discards_and_switches_mystic_summit_on() {
        // The Water starts the round with 0 discards, which Mystic Summit reads.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MYSTIC_SUMMIT);
        board.blind = Blind::Boss(BossBlind::TheWater);
        board.on_blind_selected();

        assert_eq!(board.draws.discards, 0);
        assert_eq!(board.score(), Score::new(40, 16)); // 1 + 15
    }

    #[test]
    fn on_blind_selected__the_boss_ability_lands_after_every_joker_modifier() {
        // A Boss Blind constrains the round rather than joining the pile of
        // bonuses, so The Needle leaves one hand whatever Burglar had to say.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::BURGLAR); // +3 hands
        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 1, "the boss wins the tie");

        // And on a non-boss blind Burglar gets its way as usual.
        board.blind = Blind::Small;
        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 7);
    }

    #[test]
    fn sell_joker__luchador_disables_the_current_boss_blind() {
        // Luchador: sell it to disable the current Boss Blind. The disable is
        // observable because the boss's grip on the round's draws lifts.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::LUCHADOR);
        board.blind = Blind::Boss(BossBlind::TheWater);
        board.on_blind_selected();
        assert_eq!(board.draws.discards, 0, "The Water is in force");

        let sold = board.sell_joker(0).expect("Luchador is in slot 0");
        assert_eq!(sold, card::LUCHADOR);
        assert!(board.boss_disabled);
        assert_eq!(board.draws.discards, 3, "the boss is off; discards return");
        assert_eq!(board.money, 2, "it paid its resell value");
    }

    #[test]
    fn sell_joker__only_luchador_disables_the_boss() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER);
        board.blind = Blind::Boss(BossBlind::TheWater);
        board.on_blind_selected();

        board.sell_joker(0);
        assert!(!board.boss_disabled);
        assert_eq!(board.draws.discards, 0, "The Water still holds");
    }

    #[test]
    fn on_blind_selected__luchadors_disable_lasts_only_the_current_blind() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::LUCHADOR);
        board.blind = Blind::Boss(BossBlind::TheWater);
        board.on_blind_selected();
        board.sell_joker(0);
        assert!(board.boss_disabled);

        // The next blind is a fresh boss.
        board.on_blind_selected();
        assert!(!board.boss_disabled);
        assert_eq!(board.draws.discards, 0, "The Water is back");
    }

    #[test]
    fn score__chicot_disables_every_boss_blind_while_it_is_held() {
        // Chicot: passive — it disables bosses by being on the board, so unlike
        // Luchador it needs no flag and selling it hands the boss back.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::CHICOT);
        board.blind = Blind::Boss(BossBlind::TheWater);
        board.on_blind_selected();

        assert!(!board.boss_ability_active());
        assert_eq!(board.draws.discards, 3, "The Water is disabled");

        // Selling it restores the boss.
        board.sell_joker(0);
        assert_eq!(board.draws.discards, 0, "The Water is back");
    }

    #[test]
    fn boss_ability_active__separates_the_ability_from_the_identity() {
        // A disabled boss is still a boss: Madness still refuses to grow on it
        // and Rocket still counts it. Only the *ability* is off.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::CHICOT);
        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.on_blind_selected();

        assert!(board.blind.is_boss(), "identity is unchanged");
        assert!(!board.boss_ability_active(), "but the ability is off");
        assert_eq!(board.draws.hands_to_play, 4, "The Needle does not bite");
    }

    #[test]
    fn score__madness_gains_x_mult_on_non_boss_blinds_only() {
        // Madness: gain ×0.5 Mult when a Small or Big Blind is selected — never
        // a Boss. Base ×1.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::JOKER); // +4 mult, so the ×mult scales visibly
        board.push_joker(card::MADNESS);
        assert_eq!(board.score(), Score::new(40, 5), "ungrown is ×1");

        board.blind = Blind::Small;
        board.on_blind_selected();
        board.blind = Blind::Big;
        board.on_blind_selected();
        // Two blinds -> ×2: mult 5 × 2 = 10.
        assert_eq!(board.score(), Score::new(40, 10));

        // A Boss Blind grows it not at all.
        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.on_blind_selected();
        assert_eq!(board.score(), Score::new(40, 10), "bosses do not feed it");
    }

    #[test]
    fn on_blind_selected_with_rng__madness_destroys_another_joker_but_never_itself() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MADNESS);
        board.push_joker(card::JOKER);
        board.push_joker(card::BANNER);
        board.blind = Blind::Small;

        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(5));

        assert_eq!(board.jokers.len(), 2, "one of the other two is gone");
        assert!(
            board.jokers.iter().any(|j| *j == card::MADNESS),
            "it cannot destroy itself"
        );
    }

    #[test]
    fn on_blind_selected_with_rng__madness_gains_even_with_nothing_to_destroy() {
        // The two halves are independent: a lone Madness still gains its ×0.5.
        // Coupling the gain to a successful destruction is the easy bug here.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MADNESS);
        board.blind = Blind::Small;

        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(5));

        assert_eq!(board.jokers.len(), 1, "it is alone and survives");
        assert_eq!(board.joker_state[0], 1, "and it still gained");
    }

    #[test]
    fn on_blind_selected_with_rng__madness_destroys_nothing_on_a_boss_blind() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MADNESS);
        board.push_joker(card::JOKER);
        board.blind = Blind::Boss(BossBlind::TheNeedle);

        board.on_blind_selected_with_rng(&mut StdRng::seed_from_u64(5));

        assert_eq!(board.jokers.len(), 2, "no destruction on a boss");
        assert_eq!(board.joker_state[0], 0, "and no gain either");
    }

    #[test]
    fn on_round_end__rocket_pays_one_and_grows_two_per_boss_defeated() {
        // Rocket: $1 at end of round, +$2 more per Boss Blind defeated.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::ROCKET);

        // A non-boss round: the base $1, and no growth.
        board.blind = Blind::Small;
        board.on_round_end();
        assert_eq!(board.money, 1);

        // A boss round pays the *already raised* amount — the increment lands
        // before the payout of the round that earned it. $1 + $2 = $3.
        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.on_round_end();
        assert_eq!(board.money, 4, "1 + 3, not 1 + 1");

        // A second boss: $1 + $4 = $5.
        board.on_round_end();
        assert_eq!(board.money, 9);

        // Back to a small blind: it keeps the raised payout.
        board.blind = Blind::Small;
        board.on_round_end();
        assert_eq!(board.money, 14);
    }

    #[test]
    fn on_round_end__rocket_counts_a_disabled_boss_as_defeated() {
        // Chicot switches the ability off, but the blind is still a boss — so
        // beating it is still beating a boss.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::ROCKET);
        board.push_joker(card::CHICOT);
        board.blind = Blind::Boss(BossBlind::TheNeedle);

        board.on_round_end();
        assert_eq!(board.money, 3, "1 + 2");
    }

    #[test]
    fn on_blind_selected__is_inert_without_a_blind_reader() {
        // Exit criterion 2 for Phase 8: a plain board on a Boss Blind scores
        // exactly what it scores anywhere else.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER);
        let plain = board.score();

        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.on_blind_selected();
        assert_eq!(board.score(), plain);
        assert_eq!(board.money, 0);
    }

    #[test]
    fn score__joker_stencil_counts_its_own_slot_as_empty() {
        // Joker Stencil: ×1 Mult per empty Joker slot, "Joker Stencil included".
        // Alone on a 5-slot board that is ×5, not ×4: four slots are literally
        // empty, and it counts its own as if it were too.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::JOKER_STENCIL);

        assert_eq!(board.draws.hands_to_play, 4); // untouched; just orienting
        assert_eq!(board.score(), Score::new(40, 5), "(5 − 1) + 1 = 5");

        // Each ordinary joker dilutes it by one.
        board.push_joker(card::JOKER); // +4 mult, and one slot fuller
        // Stencil is folded first: 1 × ((5 − 2) + 1) = 4, then Joker's +4 = 8.
        assert_eq!(board.score(), Score::new(40, 8));
    }

    #[test]
    fn score__joker_stencil_counts_every_stencil_not_only_itself() {
        // The "included" clause is +1 per Stencil *on the board*, not +1 for
        // self — so two Stencils are ×5 each rather than ×4, and compound to
        // ×25. The clean restatement: slots − (jokers that are not Stencils).
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER_STENCIL);
        board.push_joker(card::JOKER_STENCIL);

        // (5 − 2) + 2 = 5 each; 1 × 5 × 5 = 25.
        assert_eq!(board.score(), Score::new(40, 25));
    }

    #[test]
    fn score__joker_stencil_is_inert_on_a_full_board() {
        // The gate is on **literally** empty slots, so a full board applies
        // nothing at all — never ×0.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER_STENCIL);
        for _ in 0..4 {
            board.push_joker(card::BANNER); // inert here: 3 discards × 30 chips
        }
        assert_eq!(board.jokers.len(), board.joker_slots, "the board is full");

        // Only Banner's chips (4 × 90) land; the Stencil contributes nothing.
        assert_eq!(board.score(), Score::new(400, 1));
    }

    #[test]
    fn score__joker_stencil_reads_the_current_slot_limit() {
        // It reads `joker_slots` live rather than a hardcoded 5, so a run that
        // gains a slot gains a ×1 with it.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER_STENCIL);
        assert_eq!(board.score(), Score::new(40, 5));

        board.joker_slots += 1;
        assert_eq!(board.score(), Score::new(40, 6), "(6 − 1) + 1 = 6");
    }

    #[test]
    fn score__card_sharp_x3_when_the_hand_type_repeats_this_round() {
        // Card Sharp: ×3 Mult if the played poker hand has already been played
        // this round.
        let mut board = board_playing("KH KS 8C 5D 2S"); // a Pair
        board.push_joker(card::CARD_SHARP);
        let pair = bcards!("KH KS 8C 5D 2S");

        // The round's *first* Pair does not fire. `on_hand_played` records a
        // hand after it scores, so the tally is still empty here — reading `> 0`
        // off a tally bumped *before* scoring would wrongly fire on this hand.
        let plain = board.score();
        assert_eq!(board.score(), plain);

        // Having played one Pair, the next Pair fires.
        board.on_hand_played(&pair);
        assert_eq!(board.score(), plain.multi_mult(3.0));
    }

    #[test]
    fn score__card_sharp_keys_on_the_hand_type_not_the_hand_count() {
        // It is per *type*: three hands of other types leave a Pair unfired.
        let mut board = board_playing("KH KS 8C 5D 2S"); // a Pair
        board.push_joker(card::CARD_SHARP);
        let plain = board.score();

        board.on_hand_played(&bcards!("2S 5D 8C TS KH")); // High Card
        board.on_hand_played(&bcards!("AH KH QH JH TH")); // Straight Flush
        assert_eq!(board.score(), plain, "no Pair played yet");

        board.on_hand_played(&bcards!("QD QC 7S 4H 2C")); // a Pair
        assert_eq!(board.score(), plain.multi_mult(3.0));
    }

    #[test]
    fn score__card_sharp_resets_with_the_round() {
        // "…this round": a new blind wipes the tally.
        let mut board = board_playing("KH KS 8C 5D 2S");
        board.push_joker(card::CARD_SHARP);
        let plain = board.score();

        board.on_hand_played(&bcards!("KH KS 8C 5D 2S"));
        assert_eq!(board.score(), plain.multi_mult(3.0));

        board.on_blind_selected();
        assert_eq!(board.score(), plain, "a new round, a fresh tally");
    }

    #[test]
    fn score__ancient_joker_x_mult_per_played_card_of_the_ancient_suit() {
        // Ancient Joker: ×1.5 Mult per played card of the current suit; the
        // factor compounds, so three Hearts is ×3.375.
        let mut board = board_playing("AH KH QH 5D 2S"); // three Hearts
        board.push_joker(card::JOKER); // +4 mult, so the ×mult scales visibly
        board.push_joker(card::ANCIENT_JOKER);

        // No suit rolled yet -> no matches -> ×1, inert rather than zeroing.
        assert_eq!(board.score(), Score::new(43, 5));

        board.ancient_suit = Some('H');
        // mult 1 + 4 = 5, then ×1.5³ = ×3.375 -> ceil(16.875) = 17.
        assert_eq!(board.score(), Score::new(43, 17));

        // A suit the hand does not hold pays nothing.
        board.ancient_suit = Some('C');
        assert_eq!(board.score(), Score::new(43, 5));
    }

    #[test]
    fn score__smeared_widens_what_ancient_joker_pays_for() {
        // Smeared merges Hearts≡Diamonds, exactly as it does for flush sizing.
        let mut board = board_playing("AH KH QH 5D 2S"); // three Hearts, one Diamond
        board.push_joker(card::JOKER);
        board.push_joker(card::ANCIENT_JOKER);
        board.ancient_suit = Some('H');
        assert_eq!(board.score(), Score::new(43, 17), "three Hearts: ×1.5³");

        // With Smeared the Diamond counts too: ×1.5⁴ = ×5.0625 -> ceil(25.3) = 26.
        board.push_joker(card::SMEARED_JOKER);
        assert_eq!(board.score(), Score::new(43, 26));
    }

    #[test]
    fn on_round_end_with_rng__the_ancient_suit_never_repeats_back_to_back() {
        // "suit changes at end of round" — the new suit is drawn from the three
        // that are *not* current, so a repeat is impossible.
        let mut board = board_playing("AH KH QH 5D 2S");
        board.push_joker(card::ANCIENT_JOKER);

        board.on_round_end_with_rng(&mut StdRng::seed_from_u64(1));
        let first = board.ancient_suit.expect("the first round end rolls one");

        let mut previous = first;
        for seed in 0..40 {
            board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
            let next = board.ancient_suit.unwrap();
            assert_ne!(next, previous, "a suit must never re-roll to itself");
            previous = next;
        }
    }

    #[test]
    fn on_round_end_with_rng__the_first_ancient_roll_can_reach_all_four_suits() {
        // The first roll has no current suit to exclude, so its pool is all
        // four — Balatro's run-start seeding. Later rolls can only reach three.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..60 {
            let mut board = board_playing("AH KH QH 5D 2S");
            board.push_joker(card::ANCIENT_JOKER);
            board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
            seen.insert(board.ancient_suit.unwrap());
        }
        assert_eq!(seen.len(), 4, "the first roll reaches every suit: {seen:?}");
    }

    #[test]
    fn on_round_end_with_rng__the_ancient_suit_is_left_alone_without_the_joker() {
        // Gated on holding one, so a board that has nothing to do with Ancient
        // Joker neither gains a suit nor consumes RNG that its neighbours' rolls
        // depend on.
        let mut board = board_playing("AH KH QH 5D 2S");
        board.push_joker(card::JOKER);
        board.on_round_end_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.ancient_suit, None);
    }

    #[test]
    fn score__banner_counts_the_discards_that_remain_not_the_ones_granted() {
        // Banner is "+30 chips for each **remaining** discard", so spending one
        // must cost it 30. Reading `draws.discards` — what the round *granted* —
        // leaves it paying for discards that are already gone.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::BANNER);
        assert_eq!(board.score(), Score::new(130, 1), "3 remaining -> +90");

        board.on_discard(&bcards!("2C"));
        assert_eq!(board.score(), Score::new(100, 1), "2 remaining -> +60");

        board.on_discard(&bcards!("3C"));
        board.on_discard(&bcards!("4C"));
        assert_eq!(board.score(), Score::new(40, 1), "all spent -> nothing");
    }

    #[test]
    fn score__mystic_summit_fires_once_the_discards_are_actually_spent() {
        // The mirror image: "+15 mult when 0 discards remaining" must turn *on*
        // when the last discard is used, not only when the round granted none.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::MYSTIC_SUMMIT);
        assert_eq!(board.score(), Score::new(40, 1), "3 remaining -> inert");

        board.on_discard(&bcards!("2C"));
        board.on_discard(&bcards!("3C"));
        assert_eq!(board.score(), Score::new(40, 1), "1 still remains");

        board.on_discard(&bcards!("4C"));
        assert_eq!(board.score(), Score::new(40, 16), "spent -> +15");
    }

    #[test]
    fn discards_remaining__is_granted_minus_used_and_floors_at_zero() {
        let mut board = board_playing("2S 5D 8C TS KH");
        assert_eq!(board.discards_remaining(), 3);

        board.on_discard(&bcards!("2C"));
        assert_eq!(board.discards_remaining(), 2);

        // A round that grants none, driven anyway, floors rather than wrapping.
        board.draws.discards = 0;
        assert_eq!(board.discards_remaining(), 0);
    }

    /// A board with a real deck and an empty hand — the round loop's starting
    /// point, as opposed to `board_playing`, which pokes `played` directly.
    fn board_for_a_round() -> BuffoonBoard {
        BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile())
    }

    #[test]
    fn deal_to_hand_size__fills_the_hand_from_the_deck() {
        let mut board = board_for_a_round();
        assert_eq!(board.in_hand.len(), 0);
        let deck_before = board.deck.len();

        assert_eq!(board.deal_to_hand_size(), 8, "the base hand size");
        assert_eq!(board.in_hand.len(), 8);
        assert_eq!(board.deck.len(), deck_before - 8, "the deck really shrank");
        assert_eq!(
            board.full_deck.len(),
            52,
            "the roster is unchanged by a deal"
        );

        // Idempotent once full.
        assert_eq!(board.deal_to_hand_size(), 0);
        assert_eq!(board.in_hand.len(), 8);
    }

    #[test]
    fn deal_to_hand_size__deals_what_it_has_when_the_deck_runs_dry() {
        // Balatro deals as many as it can rather than failing. (`BuffoonPile::draw`
        // would drain the deck and return None here, losing the cards.)
        let mut board = board_for_a_round();
        board.deck = bcards!("2C 3C 4C");

        assert_eq!(board.deal_to_hand_size(), 3);
        assert_eq!(board.in_hand.len(), 3);
        assert!(board.deck.is_empty(), "and it is not left half-drained");
    }

    #[test]
    fn deal_to_hand_size__honours_the_rounds_hand_size() {
        // Juggler (+1) and The Manacle (−1) both reach the deal through the
        // round's recomputed Draws rather than through any code here.
        let mut board = board_for_a_round();
        board.push_joker(card::JUGGLER);
        board.on_blind_selected();
        assert_eq!(board.deal_to_hand_size(), 9);

        let mut manacled = board_for_a_round();
        manacled.blind = Blind::Boss(BossBlind::TheManacle);
        manacled.on_blind_selected();
        assert_eq!(manacled.deal_to_hand_size(), 7);
    }

    #[test]
    fn play_hand__moves_cards_through_the_deal_and_spends_a_hand() {
        let mut board = board_for_a_round();
        board.on_blind_selected();
        board.deal_to_hand_size();
        assert_eq!(board.hands_remaining(), 4);

        let score = board.play_hand(&[0, 1, 2, 3, 4]).expect("a legal hand");

        assert!(score.score() > 0);
        assert_eq!(board.hands_remaining(), 3, "a hand was spent");
        assert_eq!(board.round_score, score.score(), "and it accumulated");
        assert_eq!(board.discarded.len(), 5, "the played cards are spent");
        assert_eq!(board.in_hand.len(), 8, "and the hand refilled");
        assert!(board.played.is_empty(), "played is cleared after the hand");
        assert_eq!(board.full_deck.len(), 52, "the run still owns every card");
    }

    #[test]
    fn play_hand__refuses_once_the_hands_are_spent() {
        let mut board = board_for_a_round();
        board.deal_to_hand_size();
        for _ in 0..4 {
            assert!(board.play_hand(&[0]).is_some());
        }
        assert_eq!(board.hands_remaining(), 0);
        assert!(board.round_is_over());
        assert!(board.play_hand(&[0]).is_none(), "no hands left");
    }

    #[test]
    fn play_hand__refuses_an_out_of_bounds_index_without_touching_the_hand() {
        let mut board = board_for_a_round();
        board.deal_to_hand_size();
        let before = board.in_hand.clone();

        assert!(board.play_hand(&[0, 99]).is_none());
        assert_eq!(board.in_hand, before, "a refusal leaves the hand alone");
        assert_eq!(board.hands_remaining(), 4, "and spends nothing");
    }

    #[test]
    fn discard_cards__spends_a_discard_and_refills() {
        let mut board = board_for_a_round();
        board.deal_to_hand_size();
        assert_eq!(board.discards_remaining(), 3);

        assert!(board.discard_cards(&[0, 1]));
        assert_eq!(board.discards_remaining(), 2);
        assert_eq!(board.discarded.len(), 2);
        assert_eq!(board.in_hand.len(), 8, "the hand refilled");
        assert_eq!(board.hands_remaining(), 4, "a discard is not a hand");
    }

    #[test]
    fn discard_cards__refuses_once_the_discards_are_spent() {
        let mut board = board_for_a_round();
        board.deal_to_hand_size();
        for _ in 0..3 {
            assert!(board.discard_cards(&[0]));
        }
        assert!(!board.discard_cards(&[0]), "no discards left");
    }

    #[test]
    fn round_loop__conserves_every_card_the_run_owns() {
        // The invariant the board never had: through a whole round of playing
        // and discarding, every card is in exactly one place, and the roster is
        // the sum of them. This is what `full_deck` had to be a stored roster
        // *for* — see its docs — and the loop is the first thing that can hold
        // the line.
        let mut board = board_for_a_round();
        board.on_blind_selected();
        board.deal_to_hand_size();

        board.play_hand(&[0, 1, 2]);
        board.discard_cards(&[0, 1]);
        board.play_hand(&[0, 1, 2, 3, 4]);
        board.discard_cards(&[0]);

        let located = board.deck.len() + board.in_hand.len() + board.discarded.len();
        assert_eq!(
            located,
            board.full_deck.len(),
            "deck {} + hand {} + spent {} should be the roster's {}",
            board.deck.len(),
            board.in_hand.len(),
            board.discarded.len(),
            board.full_deck.len()
        );
        assert_eq!(board.full_deck.len(), 52);
    }

    #[test]
    fn round_loop__is_won_when_the_target_is_reached() {
        let mut board = board_for_a_round();
        board.blind_target = 1;
        board.deal_to_hand_size();
        assert!(!board.round_is_won());
        assert!(!board.round_is_over());

        board.play_hand(&[0]);
        assert!(board.round_is_won(), "any score clears a target of 1");
        assert!(board.round_is_over(), "a won round is over");
        assert!(board.hands_remaining() > 0, "with hands to spare");
    }

    #[test]
    fn round_loop__an_untargeted_round_runs_until_its_hands_are_spent() {
        let mut board = board_for_a_round();
        assert_eq!(board.blind_target, 0);
        board.deal_to_hand_size();

        board.play_hand(&[0]);
        assert!(!board.round_is_won(), "no target, never won");
        assert!(!board.round_is_over(), "and it runs on");
    }

    #[test]
    fn on_blind_selected__resets_the_rounds_score() {
        let mut board = board_for_a_round();
        board.deal_to_hand_size();
        board.play_hand(&[0]);
        assert!(board.round_score > 0);

        board.on_blind_selected();
        assert_eq!(board.round_score, 0);
        assert_eq!(board.hands_remaining(), 4, "and its hands are back");
    }

    #[test]
    fn round_loop__the_lifecycle_hooks_compose_in_order() {
        // The point of the loop. Four ordering rules were each found separately
        // and pinned separately; nothing had ever run them together. One round,
        // one board, all four:
        //
        //  * Vampire grows on `Scored` (before the fold) so its ×mult lands on
        //    the hand it ate;
        //  * Ice Cream decays on `HandPlayed` (after the fold) so its first hand
        //    scores the full +100;
        //  * Rocket's boss increment lands before the round's payout;
        //  * Popcorn decays at round end and is destroyed by the round that
        //    empties it.
        let mut board = board_for_a_round();
        board.blind = Blind::Boss(BossBlind::TheWater); // 0 discards
        board.push_joker(card::ICE_CREAM);
        board.push_joker(card::ROCKET);
        board.push_joker(card::POPCORN);
        board.on_blind_selected();
        board.deal_to_hand_size();

        // The Water is in force: no discards, so Mystic Summit's condition holds
        // from the first hand — and `discards_remaining` agrees.
        assert_eq!(board.discards_remaining(), 0);
        assert!(!board.discard_cards(&[0]), "The Water left none to spend");

        // Hand one: Ice Cream still worth its full +100 (it decays *after*),
        // Popcorn its full +20.
        let first = board.play_hand(&[0, 1, 2, 3, 4]).expect("a legal hand");
        assert_eq!(board.hands_played, 1);

        // Hand two: Ice Cream has decayed by 5, so the same-sized hand is worth
        // less. (Both hands are five cards; the cards differ, so compare the
        // joker state rather than the raw score.)
        board.play_hand(&[0, 1, 2, 3, 4]);
        assert_eq!(board.joker_state[0], 2, "Ice Cream counted both hands");
        assert!(first.score() > 0);

        // Round end on a Boss Blind: Rocket's increment lands *before* the
        // payout, so this round pays $1 + $2 = $3. Popcorn decays a step.
        board.on_round_end();
        assert_eq!(board.money, 3, "Rocket paid its raised amount");
        assert_eq!(board.joker_state[2], 1, "Popcorn lost a round");
        assert_eq!(board.hands_played, 0, "and the round reset");
        assert_eq!(board.round_score, 0);
    }

    #[test]
    fn round_loop__a_won_round_cashes_out() {
        // EPIC-01b 1b: the economy cycles. A real round — blind selected, hand
        // dealt, hand played, target cleared — pays all three cash-out lines
        // *and* a joker payout, every one of them read off the balance the round
        // was walked into with.
        let mut board = board_for_a_round();
        board.blind = Blind::Small;
        board.blind_target = 1; // any score clears it
        board.money = 10;
        board.push_joker(card::GOLDEN_JOKER); // a flat $4 every round

        board.on_blind_selected();
        board.deal_to_hand_size();
        assert!(!board.round_is_won(), "nothing played yet");

        board.play_hand(&[0, 1, 2, 3, 4]).expect("a legal hand");
        assert!(board.round_is_won(), "the target is cleared");
        assert_eq!(board.hands_remaining(), 3, "of 4 granted, 1 spent");

        board.on_round_end();

        // $10 walked in with:
        //   + $3  Small Blind reward
        //   + $3  one per unused hand (3 left)
        //   + $2  interest, two full $5 steps on the pre-cash-out $10
        //   + $4  Golden Joker
        //   = $22
        // Interest computed after the payouts would read $14 and pay $2 still,
        // but after the reward too it would read $16 and pay $3 — which is why
        // the delta is taken before either lands.
        assert_eq!(board.money, 22);
        assert_eq!(board.hands_played, 0, "and the round reset behind it");
        assert_eq!(board.round_score, 0);
    }

    #[test]
    fn round_loop__a_lost_round_ends_with_nothing() {
        // The mirror of the above, and the reason the gate is on `round_is_won`
        // rather than "the round ended": a round whose hands run out short of
        // its target pays no reward, no per-hand, and no interest. The joker
        // payout is not cash-out and still lands — Golden Joker pays every
        // round, won or lost.
        let mut board = board_for_a_round();
        board.blind = Blind::Small;
        board.blind_target = usize::MAX; // unreachable
        board.money = 10;
        board.push_joker(card::GOLDEN_JOKER);

        board.on_blind_selected();
        board.deal_to_hand_size();
        for _ in 0..4 {
            board.play_hand(&[0, 1, 2, 3, 4]);
        }
        assert_eq!(board.hands_remaining(), 0, "the hands are spent");
        assert!(!board.round_is_won(), "and the target was never met");

        board.on_round_end();
        assert_eq!(board.money, 14, "$10 + Golden Joker's $4, and nothing else");
    }

    #[test]
    fn round_loop__the_economy_cycles_from_cash_out_into_a_buy() {
        // EPIC-01b's headline: the economy closes. A won round cashes out, the
        // shop opens, and that cash-out money buys a joker onto the board — earn
        // then spend, in one loop.
        let mut board = board_for_a_round();
        board.blind = Blind::Small;
        board.blind_target = 1;
        board.money = 0; // start broke — everything spent is earned this round

        board.on_blind_selected();
        board.deal_to_hand_size();
        board.play_hand(&[0, 1, 2, 3, 4]).expect("a legal hand");
        assert!(board.round_is_won());

        board.on_round_end();
        // $3 Small reward + $3 for three unused hands (4 granted, 1 played) +
        // $0 interest on a $0 balance = $6 earned from nothing.
        assert_eq!(board.money, 6, "the round paid for the shopping");

        // Open a shop and put a known joker in a slot to buy deterministically.
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(9));
        board.shop.as_mut().unwrap().stock = vec![card::BLUE_JOKER]; // $5
        assert!(board.buy_stock(0), "the $6 covers the $5 joker");
        assert_eq!(board.money, 1, "and $1 change is left");
        assert_eq!(board.jokers.get(0).copied(), Some(card::BLUE_JOKER));
    }

    #[test]
    fn round_loop__vampire_eats_across_a_real_round() {
        // Vampire's ordering, driven through the loop rather than by hand: the
        // enhancement is gone from the *roster*, so the card stays eaten when it
        // comes round again.
        let mut board = board_for_a_round();
        board.push_joker(card::VAMPIRE);
        // Enhance the whole roster, so whatever is dealt is food.
        for slot in 0..board.full_deck.len() {
            let card = board.full_deck.get(slot).copied().unwrap();
            board.replace_deck_card(slot, enhanced(card, MPip::BONUS));
        }
        board.on_blind_selected();
        board.deal_to_hand_size();

        board.play_hand(&[0, 1, 2, 3, 4]);

        assert_eq!(board.joker_state[0], 5, "it ate all five");
        assert!(
            board.discarded.iter().all(|c| c.enhancement == MPip::Blank),
            "and the spent cards are stripped for good"
        );
    }

    #[test]
    fn round_loop__a_marble_jokers_stone_card_cannot_fake_a_straight() {
        // The whole path, end to end, and the reason the blank-rank filter in
        // `connectors` exists: Marble Joker adds a Stone card to the *deck*, the
        // round loop deals from the deck, so the Stone card is playable — and a
        // Stone card has no rank, so it must not connect a straight.
        //
        // Before the loop existed nothing could draw it, which is exactly why
        // this went unnoticed: a blank rank pip weighs 0, and so does a Deuce.
        let mut board = board_for_a_round();
        board.push_joker(card::MARBLE_JOKER);
        board.on_blind_selected(); // adds one Stone card to the deck

        let stone = *board.full_deck.iter().last().unwrap();
        assert_eq!(stone.enhancement, MPip::TOWER);

        // K-Q-J-T plus the Stone, whose masked base is an Ace — so if its rank
        // leaked at all, this would read as a Straight.
        board.in_hand = bcards!("KH QD JC TS");
        board.in_hand.push(stone);
        board.draws.hand_size = 5; // stop the deal topping the hand back up

        assert_eq!(
            board.in_hand.determine_hand_type(),
            HandType::HighCard,
            "the Stone's masked Ace must not complete A-K-Q-J-T"
        );
        board.play_hand(&[0, 1, 2, 3, 4]);
    }

    #[test]
    fn score__stone_card_adds_its_flat_fifty_chips() {
        // The Stone card, both halves at once. Chips: a flat +50, replacing the
        // rank's value rather than adding to it.
        let mut board = board_playing("KH 2S 5D 8C TS"); // High Card 40/1
        assert_eq!(board.score(), Score::new(40, 1));

        // The King (10 chips) becomes a Stone card: 40 − 10 + 50 = 80.
        let king = board.played.remove(0);
        board.played.insert(0, enhanced(king, MPip::TOWER));
        assert_eq!(board.score(), Score::new(80, 1));
    }

    #[test]
    fn score__stone_card_takes_part_in_no_hand_type() {
        // The other half, and the reason the chips waited for it: a Stone card
        // must not pair, connect, or flush. Each case is one a rank/suit-blind
        // implementation would get wrong.
        let stone = |index: &str| {
            let card = bcards!(index).iter().next().copied().unwrap();
            enhanced(card, MPip::TOWER)
        };

        // It does not connect a straight — it would be a 2 if rank leaked.
        let mut straight = bcards!("3C 4D 5S 6H");
        straight.push(stone("2C"));
        assert_eq!(straight.determine_hand_type(), HandType::HighCard);

        // It does not pair its own former rank.
        let mut pair = bcards!("KH 7D 9S 3C");
        pair.push(stone("KS"));
        assert_eq!(pair.determine_hand_type(), HandType::HighCard);

        // Two Stones do not pair *each other* — the trap of any model that
        // blanks the rank instead of masking it.
        let mut two = bcards!("7D 9S 3C");
        two.push(stone("KS"));
        two.push(stone("QH"));
        assert_eq!(two.determine_hand_type(), HandType::HighCard);

        // And it does not size a flush.
        let mut flush = bcards!("3H 4H 5H 6H");
        flush.push(stone("9H"));
        assert_eq!(flush.determine_hand_type(), HandType::HighCard);
    }

    #[test]
    fn score__four_fingers_rescues_a_hand_a_stone_card_shortened() {
        // A Stone card costs the hand a slot, so a four-card straight is all
        // that is left — which is exactly what Four Fingers asks for. Falls out
        // of `detectable` for free rather than needing an arm.
        let mut board = board_playing("3C 4D 5S 6H 2C");
        let last = board.played.remove(4);
        board.played.insert(4, enhanced(last, MPip::TOWER));
        assert_eq!(board.scoring_hand_type(), HandType::HighCard);

        board.push_joker(card::FOUR_FINGERS);
        assert_eq!(board.scoring_hand_type(), HandType::Straight);
    }

    #[test]
    fn on_round_end__golden_joker_pays_4() {
        // Golden Joker: earn $4 at end of round — money, not hand score.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::GOLDEN_JOKER);

        board.on_round_end();
        assert_eq!(board.money, 4);

        // It pays every round, and never touches the hand score.
        board.on_round_end();
        assert_eq!(board.money, 8);
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn on_round_end__delayed_gratification_pays_2_per_remaining_discard() {
        // Delayed Gratification: $2 per remaining discard when none was used.
        // board_playing gives Draws::new(4, 3) -> 3 discards -> $6.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::DELAYED_GRATIFICATION);

        board.on_round_end();
        assert_eq!(board.money, 6);
    }

    #[test]
    fn on_round_end__delayed_gratification_pays_0_after_a_discard() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::DELAYED_GRATIFICATION);

        board.on_discard(&bcards!("9C"));
        board.on_round_end();
        assert_eq!(board.money, 0, "any discard this round forfeits the payout");

        // on_round_end resets the round, so the next clean round pays again.
        board.on_round_end();
        assert_eq!(board.money, 6);
    }

    #[test]
    fn on_round_end__cloud_9_pays_1_per_nine_in_full_deck() {
        // Cloud 9: $1 for each 9 in the full deck. The basic deck owns four.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::CLOUD_9);

        board.on_round_end();
        assert_eq!(board.money, 4);

        // Destroy a 9 from the run: it stops paying. Cloud 9 reads the
        // roster, not the undealt remainder, so real destruction is what
        // moves it.
        let nine = board
            .full_deck
            .iter()
            .position(|card| card.rank.index == '9')
            .unwrap();
        board.destroy_deck_card(nine);
        board.on_round_end();
        assert_eq!(board.money, 4 + 3);
    }

    #[test]
    fn on_round_end__to_the_moon_pays_1_per_5_dollars_capped_at_5() {
        // To the Moon: $1 extra interest per $5 held, capped at $5.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::TO_THE_MOON);

        // $12 held -> two full $5 steps -> $2.
        board.money = 12;
        board.on_round_end();
        assert_eq!(board.money, 14);

        // $50 held -> ten steps, capped at five -> $5.
        board.money = 50;
        board.on_round_end();
        assert_eq!(board.money, 55);

        // Debt earns nothing (and costs nothing).
        board.money = -20;
        board.on_round_end();
        assert_eq!(board.money, -20);
    }

    #[test]
    fn on_discard__faceless_joker_pays_5_on_three_faces() {
        // Faceless Joker: $5 when 3 or more face cards are discarded at once.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::FACELESS_JOKER);

        board.on_discard(&bcards!("KH QD JC 2S"));
        assert_eq!(board.money, 5);
    }

    #[test]
    fn on_discard__faceless_joker_pays_0_on_two_faces() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::FACELESS_JOKER);

        board.on_discard(&bcards!("KH QD 2S 3S"));
        assert_eq!(board.money, 0, "two faces are not enough");
    }

    #[test]
    fn on_discard__faceless_joker_pareidolia_makes_any_three_cards_faces() {
        // Pareidolia makes every card a face, so any 3-card discard pays —
        // the payout goes through the same is_face_card hook as scoring.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::FACELESS_JOKER);
        board.push_joker(card::PAREIDOLIA);

        board.on_discard(&bcards!("2C 3D 4H"));
        assert_eq!(board.money, 5);
    }

    #[test]
    fn on_round_end__egg_grows_resell_value() {
        // Egg: its own sell value grows $3 every round, in place.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::EGG);
        assert_eq!(board.jokers.get(0).unwrap().resell_value, 2);

        board.on_round_end();
        assert_eq!(board.jokers.get(0).unwrap().resell_value, 5);

        board.on_round_end();
        assert_eq!(board.jokers.get(0).unwrap().resell_value, 8);
        assert_eq!(board.money, 0, "Egg is value growth, not a payout");
    }

    #[test]
    fn on_round_end__is_inert_on_a_plain_board() {
        // Exit criterion 2: with no cash / decay / destruction jokers, the
        // round-end hooks change nothing at all.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER);
        let before = board.clone();

        board.on_round_end();
        board.on_round_end_with_rng(&mut StdRng::seed_from_u64(7));
        assert_eq!(board, before);
    }

    /// A board that has **won** its round: a target of 1, cleared by a played
    /// hand, with `hands` of the 4 granted still unspent.
    ///
    /// Cash-out is gated on [`round_is_won`](BuffoonBoard::round_is_won), so
    /// every test below must go through a real win rather than poking `money` —
    /// that gate is half of what these tests exist to pin.
    fn board_that_won_a_round(hands_left: usize) -> BuffoonBoard {
        let mut board = board_for_a_round();
        board.blind_target = 1;
        board.round_score = 1;
        board.hands_played = board.draws.hands_to_play - hands_left;
        assert!(board.round_is_won(), "the fixture must be a won round");
        assert_eq!(board.hands_remaining(), hands_left);
        board
    }

    #[test]
    fn cash_out__pays_the_blind_reward_for_each_blind() {
        // Wiki: Small $3, Big $4, Boss $5. Isolated from the other two
        // components: 0 hands left, $0 held -> no per-hand pay, no interest.
        for (blind, reward) in [
            (Blind::Small, 3),
            (Blind::Big, 4),
            (Blind::Boss(BossBlind::TheNeedle), 5),
        ] {
            let mut board = board_that_won_a_round(0);
            board.blind = blind;
            board.on_round_end();
            assert_eq!(board.money, reward, "{blind} pays ${reward}");
        }
    }

    #[test]
    fn cash_out__pays_one_per_unused_hand() {
        // $1 per hand left unplayed. Small blind's $3 is the constant beneath.
        for hands_left in 0..=4 {
            let mut board = board_that_won_a_round(hands_left);
            board.on_round_end();
            assert_eq!(
                board.money,
                3 + isize::try_from(hands_left).unwrap(),
                "{hands_left} unused hands pay ${hands_left} over the $3 reward"
            );
        }
    }

    #[test]
    fn cash_out__pays_interest_of_one_per_five_held_capped_at_five() {
        // $1 per full $5 held, capped at $5 — money above $25 earns nothing.
        for (held, interest) in [(0, 0), (4, 0), (5, 1), (9, 1), (23, 4), (25, 5), (60, 5)] {
            let mut board = board_that_won_a_round(0);
            board.money = held;
            board.on_round_end();
            assert_eq!(
                board.money,
                held + 3 + interest,
                "${held} held earns ${interest} interest"
            );
        }
    }

    #[test]
    fn cash_out__debt_earns_no_interest() {
        // A negative balance must not charge negative interest.
        let mut board = board_that_won_a_round(0);
        board.money = -20;
        board.on_round_end();
        assert_eq!(board.money, -20 + 3, "the reward lands, interest does not");
    }

    #[test]
    fn cash_out__interest_and_to_the_moon_read_the_same_pre_cash_out_balance() {
        // The ordering trap (EPIC-01b Phase 1): every cash-out line is computed
        // from the balance walked in with. $23 held -> base interest $4 (four
        // full $5 steps) and To the Moon's ExtraInterest(1) -> $4, never
        // compounding off each other's payout.
        let mut board = board_that_won_a_round(0);
        board.push_joker(card::TO_THE_MOON);
        board.money = 23;

        board.on_round_end();

        // $23 + $3 reward + $4 interest + $4 To the Moon = $34.
        // Compounding would pay To the Moon on $27 ($5) or interest on $27 ($5).
        assert_eq!(board.money, 34);
    }

    // ---- Shop, Phase 2 ----------------------------------------------------

    /// A board holding a shop whose card slots are exactly `stock` — the
    /// deterministic fixture the buying tests use, skipping the random draw so
    /// the assertions do not depend on a seed.
    fn board_with_stock(stock: Vec<BuffoonCard>) -> BuffoonBoard {
        let mut board = board_for_a_round();
        board.shop = Some(crate::funky::types::shop::Shop::with_stock(stock));
        board
    }

    #[test]
    fn open_shop_with_rng__fills_two_card_slots() {
        let mut board = board_for_a_round();
        assert!(board.shop.is_none(), "closed until opened");

        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        let shop = board.shop.as_ref().expect("the shop is open");
        assert_eq!(shop.stock.len(), 2, "two card slots");
        assert_eq!(shop.rerolls_used, 0, "a fresh shop has rerolled nothing");
    }

    #[test]
    fn open_shop_with_rng__draws_only_shoppable_cards() {
        // The distribution *shape*, not exact draws: every stock card is a shop
        // joker (Common/Uncommon/Rare, never Legendary), a Tarot, or a Planet;
        // and across enough seeds all three categories and all three joker
        // rarities appear, while Legendary never does.
        use crate::funky::types::buffoon_card::BCardType;
        let mut saw_common = false;
        let mut saw_uncommon = false;
        let mut saw_rare = false;
        let mut saw_tarot = false;
        let mut saw_planet = false;
        let mut jokers = 0;
        let mut total = 0;

        for seed in 0..400 {
            let mut board = board_for_a_round();
            board.open_shop_with_rng(&mut StdRng::seed_from_u64(seed));
            for card in &board.shop.as_ref().unwrap().stock {
                total += 1;
                match card.card_type {
                    BCardType::CommonJoker => {
                        jokers += 1;
                        saw_common = true;
                        assert!(Joker::COMMON_JOKERS.contains(card), "{card} not piled");
                    }
                    BCardType::UncommonJoker => {
                        jokers += 1;
                        saw_uncommon = true;
                        assert!(Joker::UNCOMMON_JOKERS.contains(card), "{card} not piled");
                    }
                    BCardType::RareJoker => {
                        jokers += 1;
                        saw_rare = true;
                        assert!(Joker::RARE_JOKERS.contains(card), "{card} not piled");
                    }
                    BCardType::LegendaryJoker => panic!("Legendary never appears in the shop"),
                    BCardType::Tarot => saw_tarot = true,
                    BCardType::Planet => saw_planet = true,
                    other => panic!("{other:?} is not shop stock"),
                }
            }
        }

        assert!(
            saw_common && saw_uncommon && saw_rare,
            "all rarities reachable"
        );
        assert!(saw_tarot && saw_planet, "consumables reachable");
        // Jokers are the 20-of-28 weight: a clear majority over a large sample.
        assert!(
            jokers * 2 > total,
            "jokers should dominate stock ({jokers}/{total})"
        );
    }

    #[test]
    fn buy_stock__puts_a_joker_on_the_board_for_its_rank_value() {
        // Blue Joker costs its rank value; buying it deducts exactly that and
        // routes it into a joker slot.
        let price = isize::try_from(card::BLUE_JOKER.rank.value).unwrap();
        let mut board = board_with_stock(vec![card::BLUE_JOKER]);
        board.money = 10;

        assert!(board.buy_stock(0), "affordable, room to spare");
        assert_eq!(board.money, 10 - price);
        assert_eq!(board.jokers.len(), 1);
        assert_eq!(board.jokers.get(0).copied(), Some(card::BLUE_JOKER));
        assert_eq!(board.joker_state.len(), 1, "its counter came with it");
        assert!(
            board.shop.as_ref().unwrap().stock.is_empty(),
            "slot consumed"
        );
    }

    #[test]
    fn buy_stock__routes_a_consumable_through_the_consumable_slots_for_three() {
        // A Tarot or Planet costs a flat $3 and lands in a consumable slot, not
        // a joker slot.
        let mut board = board_with_stock(vec![tarot_card::FOOL]);
        board.money = 10;

        assert!(board.buy_stock(0));
        assert_eq!(board.money, 7, "flat $3, not the card's rank value");
        assert_eq!(board.consumables.len(), 1);
        assert_eq!(board.jokers.len(), 0, "a consumable is not a joker");
    }

    #[test]
    fn buy_stock__refuses_without_the_money() {
        let mut board = board_with_stock(vec![card::BLUE_JOKER]);
        board.money = 1; // Blue Joker costs $5

        assert!(!board.buy_stock(0), "cannot afford it");
        assert_eq!(board.money, 1, "no charge on a refused buy");
        assert_eq!(board.jokers.len(), 0);
        assert_eq!(
            board.shop.as_ref().unwrap().stock.len(),
            1,
            "still on offer"
        );
    }

    #[test]
    fn buy_stock__refuses_without_a_joker_slot() {
        let mut board = board_with_stock(vec![card::BLUE_JOKER]);
        board.money = 100;
        for _ in 0..board.joker_slots {
            board.push_joker(card::JOKER);
        }
        assert!(!board.has_joker_room(), "the fixture fills every slot");

        assert!(!board.buy_stock(0), "no room refuses the buy");
        assert_eq!(board.money, 100, "and charges nothing");
        assert_eq!(board.shop.as_ref().unwrap().stock.len(), 1);
    }

    #[test]
    fn buy_stock__refuses_an_index_off_the_end() {
        let mut board = board_with_stock(vec![card::BLUE_JOKER]);
        board.money = 100;
        assert!(!board.buy_stock(5), "no such slot");
        assert_eq!(board.money, 100);
    }

    #[test]
    fn buy_stock__credit_card_lets_a_buy_go_into_debt() {
        // Credit Card carries MPip::Credit(20): the buy floor drops to -$20, so
        // a purchase that ends at -$19 succeeds — and refuses without it.
        let price = isize::try_from(card::BLUE_JOKER.rank.value).unwrap();

        let mut without = board_with_stock(vec![card::BLUE_JOKER]);
        without.money = price - 19; // ends at -$19, below the $0 floor
        assert!(!without.buy_stock(0), "no Credit Card, no debt");
        assert_eq!(without.money, price - 19);

        let mut with = board_with_stock(vec![card::BLUE_JOKER]);
        with.push_joker(card::CREDIT_CARD);
        with.money = price - 19;
        assert!(with.buy_stock(0), "Credit Card allows the debt");
        assert_eq!(with.money, -19);
        assert_eq!(with.jokers.len(), 2, "Credit Card plus the bought joker");
    }

    #[test]
    fn shop__a_board_that_never_opens_one_is_unchanged() {
        // Exit criterion 2: the shop field defaults to None and nothing new
        // fires on a board that never opens it — a full round is byte-identical
        // to before the shop existed.
        let mut board = board_for_a_round();
        board.blind_target = 1;
        board.push_joker(card::GOLDEN_JOKER);
        board.on_blind_selected();
        board.deal_to_hand_size();
        board.play_hand(&[0, 1, 2, 3, 4]);
        board.on_round_end();

        assert!(board.shop.is_none(), "no shop was ever opened");
    }

    #[test]
    fn buy_stock__buying_a_joker_fires_no_card_added() {
        // Hologram counts playing cards added to the deck; a bought joker is not
        // one, so its ×0.25 counter must not tick.
        let mut board = board_with_stock(vec![card::BLUE_JOKER]);
        board.money = 10;
        board.push_joker(card::HOLOGRAM);
        let hologram_slot = board.jokers.len() - 1;
        assert_eq!(board.joker_state[hologram_slot], 0);

        assert!(board.buy_stock(0));
        assert_eq!(
            board.joker_state[hologram_slot], 0,
            "Hologram did not see a card added"
        );
        assert_eq!(board.full_deck.len(), 52, "the deck did not grow");
    }

    // ---- Reroll, Phase 3 --------------------------------------------------

    #[test]
    fn reroll_cost__starts_at_five_and_climbs_by_one() {
        let mut board = board_for_a_round();
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        board.money = 100;

        assert_eq!(board.reroll_cost(), 5, "the first paid reroll is $5");
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(board.reroll_cost(), 6, "then $6");
        board.reroll_with_rng(&mut StdRng::seed_from_u64(3));
        assert_eq!(board.reroll_cost(), 7, "then $7");
    }

    #[test]
    fn reroll_with_rng__charges_and_counts_the_reroll() {
        let mut board = board_for_a_round();
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        board.money = 20;

        assert!(board.reroll_with_rng(&mut StdRng::seed_from_u64(2)));
        assert_eq!(board.money, 15, "charged the $5 base");
        let shop = board.shop.as_ref().unwrap();
        assert_eq!(shop.rerolls_used, 1);
        assert_eq!(shop.stock.len(), 2, "the two card slots were redrawn");
    }

    #[test]
    fn reroll_cost__resets_when_a_new_shop_opens() {
        let mut board = board_for_a_round();
        board.money = 100;
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        board.reroll_with_rng(&mut StdRng::seed_from_u64(3));
        assert_eq!(board.reroll_cost(), 7, "climbed to $7");

        board.open_shop_with_rng(&mut StdRng::seed_from_u64(4));
        assert_eq!(board.reroll_cost(), 5, "a fresh shop is back to $5");
    }

    #[test]
    fn reroll_with_rng__refuses_without_the_money() {
        let mut board = board_for_a_round();
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        board.money = 3; // a reroll is $5

        assert!(!board.reroll_with_rng(&mut StdRng::seed_from_u64(2)));
        assert_eq!(board.money, 3, "no charge on a refused reroll");
        assert_eq!(board.shop.as_ref().unwrap().rerolls_used, 0);
    }

    #[test]
    fn reroll_cost__chaos_the_clown_grants_one_free_reroll_per_shop() {
        let mut board = board_for_a_round();
        board.push_joker(card::CHAOS_THE_CLOWN); // MPip::FreeReroll(1)
        board.money = 100;
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));

        assert_eq!(board.reroll_cost(), 0, "the first reroll is free");
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(board.money, 100, "and cost nothing");
        assert_eq!(board.reroll_cost(), 5, "the second is the $5 base");

        board.reroll_with_rng(&mut StdRng::seed_from_u64(3));
        assert_eq!(board.money, 95);
        assert_eq!(board.reroll_cost(), 6);
    }

    #[test]
    fn reroll_cost__two_chaos_the_clowns_grant_two_free_rerolls() {
        let mut board = board_for_a_round();
        board.push_joker(card::CHAOS_THE_CLOWN);
        board.push_joker(card::CHAOS_THE_CLOWN);
        board.money = 100;
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));

        assert_eq!(board.reroll_cost(), 0);
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(board.reroll_cost(), 0, "the second is also free");
        board.reroll_with_rng(&mut StdRng::seed_from_u64(3));
        assert_eq!(board.reroll_cost(), 5, "the third is the $5 base");
        assert_eq!(board.money, 100, "nothing spent on the two free rerolls");
    }

    #[test]
    fn score__flash_card_adds_two_mult_per_reroll() {
        // Flash Card: MPip::MultPlusPerReroll(2). It grows on each reroll and
        // adds +2 mult per reroll at score time — the Green Joker shape.
        let mut board = board_playing("2S 5D 8C TS KH"); // a high card
        board.push_joker(card::FLASH_CARD);
        board.shop = Some(crate::funky::types::shop::Shop::with_stock(vec![]));
        board.money = 100;

        let base = board.score();

        board.reroll_with_rng(&mut StdRng::seed_from_u64(1));
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        let after = board.score();

        assert_eq!(after.mult, base.mult + 4, "two rerolls add +4 mult (2 x 2)");
    }

    // ---- Vouchers, EPIC-01c Phase 1 ---------------------------------------

    use crate::funky::types::edition::Edition;
    use crate::funky::types::voucher::Voucher;

    /// A board whose shop offers exactly `voucher` and nothing else.
    fn board_offering_voucher(voucher: Voucher) -> BuffoonBoard {
        let mut board = board_for_a_round();
        let mut shop = crate::funky::types::shop::Shop::with_stock(vec![]);
        shop.voucher = Some(voucher);
        board.shop = Some(shop);
        board
    }

    #[test]
    fn vouchers__an_empty_set_is_inert_in_the_recompute() {
        // Phase 0a, the guard every later phase keeps green: with no vouchers
        // (and no jokers), a blind's recompute leaves the draws at the baseline.
        let mut board = board_for_a_round();
        assert!(board.vouchers.is_empty());
        board.on_blind_selected();
        assert_eq!(
            board.draws, board.starting_draws,
            "an empty set adds nothing"
        );
        assert_eq!(board.joker_slots, BuffoonBoard::DEFAULT_JOKER_SLOTS);
        assert_eq!(
            board.consumable_slots,
            BuffoonBoard::DEFAULT_CONSUMABLE_SLOTS
        );
    }

    #[test]
    fn redeem_shop_voucher__adds_it_and_charges_ten() {
        let mut board = board_offering_voucher(Voucher::Grabber);
        board.money = 15;

        assert!(board.redeem_shop_voucher(), "affordable at $10");
        assert_eq!(board.money, 5);
        assert_eq!(board.vouchers, vec![Voucher::Grabber]);
        assert_eq!(
            board.shop.as_ref().unwrap().voucher,
            None,
            "the slot is cleared once redeemed"
        );
    }

    #[test]
    fn redeem_shop_voucher__refuses_without_the_money() {
        let mut board = board_offering_voucher(Voucher::Grabber);
        board.money = 9; // a voucher is $10

        assert!(!board.redeem_shop_voucher());
        assert_eq!(board.money, 9, "no charge on a refused redeem");
        assert!(board.vouchers.is_empty());
        assert!(
            board.shop.as_ref().unwrap().voucher.is_some(),
            "still offered"
        );
    }

    #[test]
    fn redeem_shop_voucher__an_upgrade_needs_its_base() {
        // Overstock Plus requires Overstock. Refused without it, even with money.
        let mut without = board_offering_voucher(Voucher::OverstockPlus);
        without.money = 100;
        assert!(!without.redeem_shop_voucher(), "no base, no upgrade");
        assert_eq!(without.money, 100, "and no charge");

        let mut with = board_offering_voucher(Voucher::OverstockPlus);
        with.money = 100;
        with.vouchers.push(Voucher::Overstock);
        assert!(with.redeem_shop_voucher(), "the base is held");
        assert!(with.vouchers.contains(&Voucher::OverstockPlus));
    }

    #[test]
    fn redeem_shop_voucher__refuses_an_empty_slot() {
        let mut board = board_for_a_round();
        board.shop = Some(crate::funky::types::shop::Shop::with_stock(vec![]));
        assert!(!board.redeem_shop_voucher(), "no voucher on offer");
    }

    #[test]
    fn open_shop_with_rng__offers_an_eligible_voucher() {
        // A fresh shop offers a voucher, and it is always an eligible one — a
        // base, or an upgrade whose base is held. With nothing held, only bases
        // are eligible, so the offer's `requires()` is None.
        let mut board = board_for_a_round();
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        let offered = board
            .shop
            .as_ref()
            .unwrap()
            .voucher
            .expect("a voucher offered");
        assert_eq!(offered.requires(), None, "an upgrade cannot be offered yet");
    }

    #[test]
    fn open_shop_with_rng__never_offers_a_redeemed_voucher() {
        // Once Grabber is held, no shop offers it again.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Grabber);
        for seed in 0..64 {
            board.open_shop_with_rng(&mut StdRng::seed_from_u64(seed));
            assert_ne!(
                board.shop.as_ref().unwrap().voucher,
                Some(Voucher::Grabber),
                "a redeemed voucher never re-offers (seed {seed})"
            );
        }
    }

    #[test]
    fn open_shop_with_rng__offers_an_upgrade_once_its_base_is_held() {
        // With Grabber held, Nacho Tong becomes eligible; across seeds it does
        // get offered, and no other upgrade whose base is unheld ever does.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Grabber);
        let mut saw_nacho = false;
        for seed in 0..256 {
            board.open_shop_with_rng(&mut StdRng::seed_from_u64(seed));
            let offered = board.shop.as_ref().unwrap().voucher.unwrap();
            if let Some(base) = offered.requires() {
                assert!(
                    board.vouchers.contains(&base),
                    "{offered} offered without its base {base} (seed {seed})"
                );
            }
            if offered == Voucher::NachoTong {
                saw_nacho = true;
            }
        }
        assert!(saw_nacho, "Nacho Tong is reachable once Grabber is held");
    }

    // ---- Draws vouchers, EPIC-01c Phase 2 ---------------------------------

    #[test]
    fn recompute_draws__grabber_adds_a_hand() {
        let mut board = board_for_a_round(); // 4 hands, 3 discards, hand size 8
        board.vouchers.push(Voucher::Grabber);
        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 5);
        assert_eq!(board.draws.discards, 3, "only hands move");
    }

    #[test]
    fn recompute_draws__nacho_tong_adds_a_second_hand() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Grabber);
        board.vouchers.push(Voucher::NachoTong);
        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 6, "Grabber + Nacho Tong = +2");
    }

    #[test]
    fn recompute_draws__wasteful_adds_a_discard() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Wasteful);
        board.on_blind_selected();
        assert_eq!(board.draws.discards, 4);
        assert_eq!(board.draws.hands_to_play, 4, "only discards move");
    }

    #[test]
    fn recompute_draws__recyclomancy_adds_a_second_discard() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Wasteful);
        board.vouchers.push(Voucher::Recyclomancy);
        board.on_blind_selected();
        assert_eq!(board.draws.discards, 5);
    }

    #[test]
    fn recompute_draws__paint_brush_adds_hand_size() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::PaintBrush);
        board.on_blind_selected();
        assert_eq!(board.draws.hand_size, Draws::DEFAULT_HAND_SIZE + 1);
    }

    #[test]
    fn recompute_draws__palette_adds_a_second_hand_size() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::PaintBrush);
        board.vouchers.push(Voucher::Palette);
        board.on_blind_selected();
        assert_eq!(board.draws.hand_size, Draws::DEFAULT_HAND_SIZE + 2);
    }

    #[test]
    fn recompute_draws__the_boss_ability_still_overrides_grabber() {
        // The Needle leaves exactly 1 hand, applied last — after every bonus.
        // Grabber's +1 is computed and then overridden, matching Balatro.
        let mut board = board_for_a_round();
        board.blind = Blind::Boss(BossBlind::TheNeedle);
        board.vouchers.push(Voucher::Grabber);
        board.on_blind_selected();
        assert_eq!(
            board.draws.hands_to_play, 1,
            "the boss constrains after the bonus"
        );
    }

    #[test]
    fn recompute_draws__burglar_still_zeroes_a_wasteful_discard() {
        // Burglar loses all discards; Wasteful's +1 is added before that zeroing,
        // so Burglar still wins — the voucher discard does not survive it.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Wasteful);
        board.push_joker(card::BURGLAR);
        board.on_blind_selected();
        assert_eq!(
            board.draws.discards, 0,
            "Burglar zeroes even a Wasteful discard"
        );
    }

    #[test]
    fn recompute_draws__vouchers_do_not_stack_across_blinds() {
        // The recompute rebuilds from `starting_draws` each blind, so a permanent
        // voucher adds its bonus once, not once per blind.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Grabber);
        board.on_blind_selected();
        board.on_blind_selected();
        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 5, "still +1, never +3");
    }

    // ---- Slot vouchers, EPIC-01c Phase 3 ----------------------------------

    #[test]
    fn redeem_shop_voucher__crystal_ball_adds_a_consumable_slot() {
        let mut board = board_offering_voucher(Voucher::CrystalBall);
        board.money = 20;
        assert_eq!(
            board.consumable_slots,
            BuffoonBoard::DEFAULT_CONSUMABLE_SLOTS
        );

        assert!(board.redeem_shop_voucher());
        assert_eq!(
            board.consumable_slots,
            BuffoonBoard::DEFAULT_CONSUMABLE_SLOTS + 1,
            "Crystal Ball adds a consumable slot"
        );
        assert_eq!(
            board.joker_slots,
            BuffoonBoard::DEFAULT_JOKER_SLOTS,
            "and leaves joker slots alone"
        );
    }

    #[test]
    fn redeem_shop_voucher__antimatter_adds_a_joker_slot() {
        let mut board = board_offering_voucher(Voucher::Antimatter);
        board.money = 20;

        assert!(board.redeem_shop_voucher());
        assert_eq!(board.joker_slots, BuffoonBoard::DEFAULT_JOKER_SLOTS + 1);
        assert_eq!(
            board.consumable_slots,
            BuffoonBoard::DEFAULT_CONSUMABLE_SLOTS
        );
    }

    #[test]
    fn crystal_ball__opens_room_for_a_third_consumable() {
        // The end-to-end: a full inventory refuses a third consumable, and Crystal
        // Ball redeemed opens exactly the room for it.
        let mut board = board_for_a_round();
        board.money = 20;
        assert!(board.create_consumable(tarot_card::FOOL));
        assert!(board.create_consumable(tarot_card::FOOL));
        assert!(!board.has_consumable_room(), "the two slots are full");

        let mut shop = crate::funky::types::shop::Shop::with_stock(vec![tarot_card::FOOL]);
        shop.voucher = Some(Voucher::CrystalBall);
        board.shop = Some(shop);

        assert!(!board.buy_stock(0), "no room for a third consumable yet");
        assert!(board.redeem_shop_voucher(), "redeem Crystal Ball");
        assert!(board.buy_stock(0), "now the third fits");
        assert_eq!(board.consumables.len(), 3);
    }

    #[test]
    fn open_shop_with_rng__overstock_offers_three_card_slots() {
        // Overstock sizes the shop's card slots live at open, not a board field.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Overstock);
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.shop.as_ref().unwrap().stock.len(), 3);
    }

    #[test]
    fn open_shop_with_rng__overstock_plus_offers_four_card_slots() {
        // Overstock Plus requires Overstock, so holding it means holding both —
        // the bonus is +2.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::Overstock);
        board.vouchers.push(Voucher::OverstockPlus);
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.shop.as_ref().unwrap().stock.len(), 4);
    }

    // ---- Economy vouchers, EPIC-01c Phase 4 -------------------------------

    #[test]
    fn cash_out__seed_money_raises_the_interest_cap_to_ten() {
        // Base cap is $5 (money above $25 earns nothing). Seed Money raises it to
        // $10, so $60 held now earns the full 12 steps capped at 10.
        let mut board = board_that_won_a_round(0); // Small blind: $3 reward
        board.money = 60;
        board.vouchers.push(Voucher::SeedMoney);
        board.on_round_end();
        // $60 + $3 reward + $10 interest = $73 (was $68 at the $5 cap).
        assert_eq!(board.money, 73);
    }

    #[test]
    fn cash_out__money_tree_raises_the_cap_to_twenty() {
        let mut board = board_that_won_a_round(0);
        board.money = 200;
        board.vouchers.push(Voucher::SeedMoney);
        board.vouchers.push(Voucher::MoneyTree);
        board.on_round_end();
        // (200/5=40).clamp(0,20) = $20 interest + $3 reward.
        assert_eq!(board.money, 223);
    }

    #[test]
    fn cash_out__to_the_moon_reads_the_same_raised_cap() {
        // The keystone: both interest readers see one cap. With Seed Money, base
        // interest AND To the Moon's ExtraInterest both cap at $10, not $5.
        let mut board = board_that_won_a_round(0);
        board.money = 60;
        board.push_joker(card::TO_THE_MOON);
        board.vouchers.push(Voucher::SeedMoney);
        board.on_round_end();
        // $60 + $3 reward + $10 base interest + $10 To the Moon = $83.
        assert_eq!(board.money, 83);
    }

    #[test]
    fn reroll_cost__reroll_surplus_takes_two_dollars_off() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::RerollSurplus);
        board.money = 100;
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.reroll_cost(), 3, "$5 base − $2");
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(board.reroll_cost(), 4, "$6 − $2");
    }

    #[test]
    fn reroll_cost__reroll_glut_takes_four_off() {
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::RerollSurplus);
        board.vouchers.push(Voucher::RerollGlut);
        board.money = 100;
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.reroll_cost(), 1, "$5 base − $4");
    }

    #[test]
    fn buy_stock__clearance_sale_discounts_a_card_and_liquidation_more() {
        // Blue Joker is $5. Clearance Sale (25% off) → $3; Liquidation (50%) → $2.
        let mut clearance = board_with_stock(vec![card::BLUE_JOKER]);
        clearance.money = 20;
        clearance.vouchers.push(Voucher::ClearanceSale);
        assert!(clearance.buy_stock(0));
        assert_eq!(clearance.money, 20 - 3, "$5 → $3 at 25% off");

        let mut liquidation = board_with_stock(vec![card::BLUE_JOKER]);
        liquidation.money = 20;
        liquidation.vouchers.push(Voucher::ClearanceSale);
        liquidation.vouchers.push(Voucher::Liquidation);
        assert!(liquidation.buy_stock(0));
        assert_eq!(liquidation.money, 20 - 2, "$5 → $2 at 50% off");
    }

    #[test]
    fn open_pack_with_rng__clearance_discounts_the_pack() {
        // A $4 pack: Clearance Sale → $3, Liquidation → $2.
        let mut board = board_with_packs(vec![buffoon_pack()]);
        board.money = 20;
        board.vouchers.push(Voucher::ClearanceSale);
        board.open_pack_with_rng(0, &mut StdRng::seed_from_u64(1));
        assert_eq!(board.money, 20 - 3, "$4 pack → $3 at 25% off");
    }

    // ---- Shop-weight vouchers, EPIC-01c Phase 5 ---------------------------

    fn count_kind(vouchers: &[Voucher], kind: BCardType, draws: usize) -> usize {
        let mut board = board_for_a_round();
        for voucher in vouchers {
            board.vouchers.push(*voucher);
        }
        let mut rng = StdRng::seed_from_u64(42);
        (0..draws)
            .filter(|_| board.draw_stock_card(&mut rng).card_type == kind)
            .count()
    }

    #[test]
    fn draw_stock_card__tarot_tycoon_biases_toward_tarots() {
        // Base tarot weight is 4/28; Tarot Tycoon lifts it to 16/40. Not a 4×
        // *share* (the denominator grows), but well over double the count.
        let base = count_kind(&[], BCardType::Tarot, 4000);
        let tycoon = count_kind(
            &[Voucher::TarotMerchant, Voucher::TarotTycoon],
            BCardType::Tarot,
            4000,
        );
        assert!(
            tycoon > 2 * base,
            "Tarot Tycoon biases toward tarots ({tycoon} vs {base})"
        );
    }

    #[test]
    fn draw_stock_card__planet_merchant_biases_toward_planets() {
        // Planet Merchant doubles the planet band (4 → 8).
        let base = count_kind(&[], BCardType::Planet, 4000);
        let merchant = count_kind(&[Voucher::PlanetMerchant], BCardType::Planet, 4000);
        assert!(
            merchant > base,
            "Planet Merchant biases toward planets ({merchant} vs {base})"
        );
    }

    #[test]
    fn draw_stock_card__jokers_stay_a_piled_partition_under_bias() {
        // The consumable bands moving must not corrupt the joker partition — a
        // drawn joker is still always a piled one.
        let mut board = board_for_a_round();
        board.vouchers.push(Voucher::TarotMerchant);
        board.vouchers.push(Voucher::TarotTycoon);
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..2000 {
            let card = board.draw_stock_card(&mut rng);
            if card.is_joker() {
                assert!(
                    Joker::COMMON_JOKERS.contains(&card)
                        || Joker::UNCOMMON_JOKERS.contains(&card)
                        || Joker::RARE_JOKERS.contains(&card),
                    "{card} drawn but not piled"
                );
            }
        }
    }

    #[test]
    fn reroll_with_rng__overstock_widens_the_reroll_too() {
        // A reroll redraws the same number of card slots the shop offers, so
        // Overstock's wider stock survives a reroll.
        let mut board = board_for_a_round();
        board.money = 100;
        board.vouchers.push(Voucher::Overstock);
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        assert_eq!(board.shop.as_ref().unwrap().stock.len(), 3);
        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(
            board.shop.as_ref().unwrap().stock.len(),
            3,
            "the reroll kept all three slots"
        );
    }

    // ---- Played-card editions, EPIC-01d Phase 1 ---------------------------

    /// `board_playing(index)` with `edition` stamped on the first played card.
    fn board_playing_edition(index: &str, edition: Edition) -> BuffoonBoard {
        let mut board = board_playing(index);
        let first = board.played.get(0).copied().unwrap().with_edition(edition);
        board.played.remove(0);
        board.played.insert(0, first);
        board
    }

    #[test]
    fn score__a_foil_played_card_adds_fifty_chips() {
        let base = board_playing("2S 5D 8C TS KH").score();
        let foil = board_playing_edition("2S 5D 8C TS KH", Edition::Foil).score();
        assert_eq!(foil.chips, base.chips + 50, "Foil is +50 chips");
        assert_eq!(foil.mult, base.mult, "and no mult");
    }

    #[test]
    fn score__a_holographic_played_card_adds_ten_mult() {
        let base = board_playing("2S 5D 8C TS KH").score();
        let holo = board_playing_edition("2S 5D 8C TS KH", Edition::Holographic).score();
        assert_eq!(holo.mult, base.mult + 10, "Holo is +10 mult");
        assert_eq!(holo.chips, base.chips, "and no chips");
    }

    #[test]
    fn score__a_polychrome_played_card_multiplies_mult_by_one_and_a_half() {
        // A pair enters phase 2 at 2 mult; Polychrome on a played card ×1.5s the
        // running mult at that card's position → ceil(2 × 1.5) = 3.
        let base = board_playing("AS AD QC JS TH").score();
        assert_eq!(base.mult, 2, "a pair's base mult");
        let poly = board_playing_edition("AS AD QC JS TH", Edition::Polychrome).score();
        assert_eq!(poly.mult, 3, "×1.5 ceils 2 → 3");
        assert_eq!(poly.chips, base.chips, "Polychrome moves mult, not chips");
    }

    #[test]
    fn score__an_unedited_played_hand_is_unchanged() {
        // The inertness anchor: Edition::None everywhere scores byte-identical.
        let base = board_playing("AS AD QC JS TH");
        let none = board_playing_edition("AS AD QC JS TH", Edition::None);
        assert_eq!(none.score(), base.score());
    }

    // ---- Joker editions, EPIC-01d Phase 2 ---------------------------------

    /// `board_playing(index)` with a single joker pushed on.
    fn board_playing_joker(index: &str, joker: BuffoonCard) -> BuffoonBoard {
        let mut board = board_playing(index);
        board.push_joker(joker);
        board
    }

    #[test]
    fn score__a_foil_joker_adds_fifty_chips() {
        let base = board_playing_joker("2S 5D 8C TS KH", card::JOKER).score();
        let foil =
            board_playing_joker("2S 5D 8C TS KH", card::JOKER.with_edition(Edition::Foil)).score();
        assert_eq!(foil.chips, base.chips + 50, "Foil is +50 chips");
        assert_eq!(
            foil.mult, base.mult,
            "and the joker's own +4 mult, unchanged"
        );
    }

    #[test]
    fn score__a_holographic_joker_adds_ten_mult() {
        let base = board_playing_joker("2S 5D 8C TS KH", card::JOKER).score();
        let holo = board_playing_joker(
            "2S 5D 8C TS KH",
            card::JOKER.with_edition(Edition::Holographic),
        )
        .score();
        assert_eq!(holo.mult, base.mult + 10, "Holo is +10 mult");
        assert_eq!(holo.chips, base.chips, "and no chips");
    }

    #[test]
    fn score__a_polychrome_joker_multiplies_the_running_mult() {
        // High card enters phase 4 at 1 mult; the Joker adds +4 → 5, then the
        // joker's Polychrome ×1.5s at its position → ceil(5 × 1.5) = 8.
        let base = board_playing_joker("2S 5D 8C TS KH", card::JOKER).score();
        assert_eq!(base.mult, 5, "1 base + the Joker's 4");
        let poly = board_playing_joker(
            "2S 5D 8C TS KH",
            card::JOKER.with_edition(Edition::Polychrome),
        )
        .score();
        assert_eq!(poly.mult, 8, "×1.5 after the joker's effect, ceil(7.5)");
    }

    #[test]
    fn score__an_unedited_joker_is_unchanged() {
        let base = board_playing_joker("2S 5D 8C TS KH", card::JOKER);
        let none = board_playing_joker("2S 5D 8C TS KH", card::JOKER.with_edition(Edition::None));
        assert_eq!(none.score(), base.score());
    }

    // ---- Booster packs, Phase 4 -------------------------------------------

    /// A board whose shop offers exactly `packs`, and nothing else.
    fn board_with_packs(packs: Vec<BoosterPack>) -> BuffoonBoard {
        let mut board = board_for_a_round();
        let mut shop = crate::funky::types::shop::Shop::with_stock(vec![]);
        shop.packs = packs;
        board.shop = Some(shop);
        board
    }

    fn buffoon_pack() -> BoosterPack {
        BoosterPack {
            kind: crate::funky::types::shop::PackKind::Buffoon,
            cost: 4,
        }
    }

    #[test]
    fn open_shop_with_rng__offers_two_booster_packs() {
        let mut board = board_for_a_round();
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        let packs = &board.shop.as_ref().unwrap().packs;
        assert_eq!(packs.len(), 2, "two pack slots");
        assert!(packs.iter().all(|p| p.cost == 4), "base tier is $4");
    }

    #[test]
    fn skip_pack__removes_the_pack() {
        let mut board = board_with_packs(vec![buffoon_pack(), buffoon_pack()]);
        assert!(board.skip_pack(0));
        assert_eq!(board.shop.as_ref().unwrap().packs.len(), 1, "one skipped");
    }

    #[test]
    fn skip_pack__refuses_a_bad_index() {
        let mut board = board_with_packs(vec![buffoon_pack()]);
        assert!(!board.skip_pack(3), "no such pack");
        assert_eq!(board.shop.as_ref().unwrap().packs.len(), 1);
    }

    #[test]
    fn score__red_card_adds_three_mult_per_pack_skipped() {
        // Red Card: MPip::MultPlusPerPackSkipped(3), +3 mult per skip.
        let mut board = board_playing("2S 5D 8C TS KH"); // high card
        board.push_joker(card::RED_CARD);
        let mut shop = crate::funky::types::shop::Shop::with_stock(vec![]);
        shop.packs = vec![buffoon_pack(), buffoon_pack()];
        board.shop = Some(shop);

        let base = board.score();
        board.skip_pack(0);
        board.skip_pack(0);
        let after = board.score();

        assert_eq!(after.mult, base.mult + 6, "two skips add +6 mult (2 x 3)");
    }

    #[test]
    fn open_pack_with_rng__pays_and_returns_the_choices() {
        // A Buffoon pack costs $4 and offers two jokers to choose from.
        let mut board = board_with_packs(vec![buffoon_pack()]);
        board.money = 10;

        let choices = board
            .open_pack_with_rng(0, &mut StdRng::seed_from_u64(1))
            .expect("an affordable pack");
        assert_eq!(board.money, 6, "charged the $4");
        assert_eq!(choices.len(), 2, "choose 1 of 2 jokers");
        assert!(choices.iter().all(BuffoonCard::is_joker), "a Buffoon pack");
        assert!(
            board.shop.as_ref().unwrap().packs.is_empty(),
            "pack consumed"
        );
    }

    #[test]
    fn open_pack_with_rng__refuses_without_the_money() {
        let mut board = board_with_packs(vec![buffoon_pack()]);
        board.money = 2; // a pack is $4

        assert!(
            board
                .open_pack_with_rng(0, &mut StdRng::seed_from_u64(1))
                .is_none()
        );
        assert_eq!(board.money, 2, "no charge on a refused open");
        assert_eq!(
            board.shop.as_ref().unwrap().packs.len(),
            1,
            "still on offer"
        );
    }

    #[test]
    fn open_pack_with_rng__hallucination_creates_tarots_about_half_the_time() {
        // Hallucination: 1-in-2 to create a Tarot on any pack opened. Across
        // seeds both outcomes occur, deterministically per seed.
        let mut made = 0;
        let mut skipped = 0;
        for seed in 0..64 {
            let mut board = board_with_packs(vec![buffoon_pack()]);
            board.money = 10;
            board.push_joker(card::HALLUCINATION);
            board.open_pack_with_rng(0, &mut StdRng::seed_from_u64(seed));
            if board.consumables.is_empty() {
                skipped += 1;
            } else {
                assert_eq!(board.consumables.len(), 1);
                assert_eq!(
                    board.consumables.get(0).unwrap().card_type,
                    BCardType::Tarot,
                    "it makes a Tarot"
                );
                made += 1;
            }
        }
        assert!(
            made > 0 && skipped > 0,
            "both outcomes occur ({made} made, {skipped} not)"
        );
    }

    #[test]
    fn open_pack_with_rng__three_oops_all_6s_make_hallucination_certain() {
        // The Gros Michel pin: enough Oops! All 6s caps the 1-in-2 at certainty,
        // so a Tarot is made on every seed.
        for seed in 0..32 {
            let mut board = board_with_packs(vec![buffoon_pack()]);
            board.money = 10;
            board.push_joker(card::HALLUCINATION);
            board.push_joker(card::OOPS_ALL_6S);
            board.push_joker(card::OOPS_ALL_6S);
            board.push_joker(card::OOPS_ALL_6S);
            board.open_pack_with_rng(0, &mut StdRng::seed_from_u64(seed));
            assert_eq!(board.consumables.len(), 1, "certain on seed {seed}");
        }
    }

    #[test]
    fn reroll_with_rng__leaves_the_packs_alone() {
        // A reroll redraws only the card slots; the pack slots are untouched.
        let mut board = board_for_a_round();
        board.money = 100;
        board.open_shop_with_rng(&mut StdRng::seed_from_u64(1));
        let packs_before = board.shop.as_ref().unwrap().packs.clone();

        board.reroll_with_rng(&mut StdRng::seed_from_u64(2));
        assert_eq!(
            board.shop.as_ref().unwrap().packs,
            packs_before,
            "the reroll left the packs alone"
        );
    }

    #[test]
    fn cash_out__an_unwon_round_pays_nothing() {
        // The gate: cash-out fires only on a won round. An untargeted round
        // (blind_target 0 — every pre-EPIC-01b board) is never won, so it keeps
        // paying exactly what it paid before the shop existed.
        let mut board = board_for_a_round();
        board.money = 23;
        assert!(!board.round_is_won());

        board.on_round_end();
        assert_eq!(board.money, 23, "no reward, no per-hand, no interest");

        // And a targeted round that fell short pays nothing either.
        let mut lost = board_for_a_round();
        lost.blind_target = 100;
        lost.round_score = 99;
        lost.money = 23;
        lost.on_round_end();
        assert_eq!(lost.money, 23);
    }

    #[test]
    fn on_round_end_with_rng__gros_michel_survives_and_dies() {
        // Gros Michel: a 1-in-6 destruction roll at end of round. Across
        // seeds both outcomes must occur, deterministically per seed — the
        // same contract as score_with_seed.
        let mut survived = false;
        let mut destroyed = false;
        for seed in 0..64 {
            let mut board = board_playing("2S 5D 8C TS KH");
            board.push_joker(card::GROS_MICHEL);
            board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
            if board.jokers.is_empty() {
                destroyed = true;
            } else {
                survived = true;
            }
        }
        assert!(destroyed, "a 1-in-6 roll should destroy over 64 seeds");
        assert!(survived, "a 1-in-6 roll should survive over 64 seeds");
    }

    #[test]
    fn on_round_end_with_rng__cavendish_1_in_1000() {
        // Cavendish: 1-in-1000 — rare but real. Deterministic for a given
        // rand version; expected ~4 destructions over 4000 seeds.
        let mut destructions = 0;
        for seed in 0..4000 {
            let mut board = board_playing("2S 5D 8C TS KH");
            board.push_joker(card::CAVENDISH);
            board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
            if board.jokers.is_empty() {
                destructions += 1;
            }
        }
        assert!(destructions >= 1, "1-in-1000 must be able to fire");
        assert!(
            destructions < 40,
            "1-in-1000 must stay rare (got {destructions} in 4000)"
        );
    }

    #[test]
    fn on_round_end_with_rng__oops_doubles_gros_michel_odds() {
        // Three Oops! All 6s scale the 1-in-6 by 2^3 = 8, capped at
        // certainty: Gros Michel dies on every seed, through the same
        // probability_numerator seam the Lucky roll uses.
        for seed in 0..16 {
            let mut board = board_playing("2S 5D 8C TS KH");
            board.push_joker(card::GROS_MICHEL);
            board.push_joker(card::OOPS_ALL_6S);
            board.push_joker(card::OOPS_ALL_6S);
            board.push_joker(card::OOPS_ALL_6S);
            board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
            assert_eq!(
                board.jokers.len(),
                3,
                "seed {seed}: certainty must destroy Gros Michel (and only it)"
            );
        }
    }

    #[test]
    fn on_hand_played__ice_cream_melts_at_zero_chips() {
        // Ice Cream: 100 chips decaying −5 per hand. The 19th hand leaves +5;
        // the 20th empties it, and the emptying hand melts the joker — on
        // that hand, not at round end (exact Balatro timing).
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::ICE_CREAM);

        let hand = bcards!("2S 5D 8C TS KH");
        for _ in 0..19 {
            board.on_hand_played(&hand);
        }
        assert_eq!(board.jokers.len(), 1, "19 hands leave +5 chips");
        assert_eq!(board.score(), Score::new(45, 1));

        board.on_hand_played(&hand);
        assert!(board.jokers.is_empty(), "the 20th hand melts it");
        assert_eq!(board.score(), Score::new(40, 1));
    }

    #[test]
    fn on_blind_selected__juggler_increases_hand_size() {
        // Juggler: +1 hand size while held. board_playing starts at the base
        // hand size of 8.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JUGGLER);

        board.on_blind_selected();
        assert_eq!(board.draws.hand_size, 9);

        // Selling the joker takes its bonus with it at the next blind.
        board.remove_joker(0);
        board.on_blind_selected();
        assert_eq!(board.draws.hand_size, 8);
    }

    #[test]
    fn on_blind_selected__drunkard_adds_a_discard() {
        // Drunkard: +1 discard each round. board_playing starts at 3.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::DRUNKARD);

        board.on_blind_selected();
        assert_eq!(board.draws.discards, 4);

        // Banner reads the recomputed round state: +30 chips per remaining
        // discard is now 4 discards' worth.
        board.push_joker(card::BANNER);
        assert_eq!(board.score(), Score::new(40 + 120, 1));
    }

    #[test]
    fn on_blind_selected__burglar_gains_hands_and_wipes_discards() {
        // Burglar: +3 hands and lose ALL discards when the Blind is selected —
        // including another joker's discard bonus, so it lands after every
        // increment regardless of joker order (Drunkard sits to its right).
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::BURGLAR);
        board.push_joker(card::DRUNKARD);

        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 7);
        assert_eq!(board.draws.discards, 0, "Drunkard's +1 is wiped too");
    }

    #[test]
    fn on_blind_selected__burglar_enables_mystic_summit() {
        // Mystic Summit (+15 mult on zero discards) reads what Burglar wipes:
        // selecting a blind with both aboard turns it on.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::MYSTIC_SUMMIT);
        assert_eq!(board.score(), Score::new(40, 1), "3 discards -> inert");

        board.push_joker(card::BURGLAR);
        board.on_blind_selected();
        assert_eq!(board.score(), Score::new(40, 16));
    }

    #[test]
    fn on_blind_selected__is_idempotent() {
        // Recomputed from starting_draws, never accumulated: a second blind
        // select must not stack the bonuses.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JUGGLER);
        board.push_joker(card::DRUNKARD);
        board.push_joker(card::BURGLAR);

        board.on_blind_selected();
        board.on_blind_selected();
        assert_eq!(board.draws.hands_to_play, 7);
        assert_eq!(board.draws.discards, 0);
        assert_eq!(board.draws.hand_size, 9);
    }

    #[test]
    fn on_blind_selected__is_inert_on_a_plain_board() {
        // Exit criterion 2: with no draw-modifier jokers the hook changes
        // nothing.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER);
        let before = board.clone();

        board.on_blind_selected();
        assert_eq!(board, before);
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
    fn score__dusk_retriggers_every_played_card_on_the_rounds_final_hand() {
        // Dusk: retrigger all played cards in the final hand of the round.
        // `board_playing` grants four hands (Draws::new(4, 3)).
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::DUSK);

        // An untouched board reads `hands_played == 0`, so the first of four
        // hands is not the last -> no retrigger.
        assert_eq!(board.score(), Score::new(40, 1));

        let hand = bcards!("2S 5D 8C TS KH");
        board.on_hand_played(&hand);
        board.on_hand_played(&hand);
        assert_eq!(
            board.score(),
            Score::new(40, 1),
            "the third of four hands is not the last"
        );

        // Three hands done -> the fourth is final: all five cards score twice,
        // so the 35 chips of card pips land again (the 5-chip High Card base is
        // phase 1 and does not re-run). 40 -> 75.
        board.on_hand_played(&hand);
        assert_eq!(board.score(), Score::new(75, 1));
    }

    #[test]
    fn score__dusk_follows_the_hand_allowance_rather_than_a_fixed_count() {
        // Burglar's +3 hands pushes the final hand out; Dusk must track the
        // round's *granted* allowance, not a hardcoded four. With 7 hands
        // granted, the fourth is no longer final.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::DUSK);
        board.push_joker(card::BURGLAR);
        board.on_blind_selected(); // 4 + 3 = 7 hands

        let hand = bcards!("2S 5D 8C TS KH");
        for _ in 0..3 {
            board.on_hand_played(&hand);
        }
        assert_eq!(
            board.score(),
            Score::new(40, 1),
            "the fourth of seven hands is not the last"
        );

        for _ in 0..3 {
            board.on_hand_played(&hand);
        }
        assert_eq!(board.score(), Score::new(75, 1), "the seventh is");
    }

    #[test]
    fn score__seltzer_retriggers_every_played_card_for_ten_hands() {
        // Seltzer: retrigger all cards played for the next 10 hands.
        let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
        board.push_joker(card::SELTZER);

        // Fresh: all five cards score twice. 40 -> 75.
        assert_eq!(board.score(), Score::new(75, 1));

        // Nine hands spent, one left: still retriggering.
        let hand = bcards!("2S 5D 8C TS KH");
        for _ in 0..9 {
            board.on_hand_played(&hand);
        }
        assert_eq!(
            board.score(),
            Score::new(75, 1),
            "the tenth hand still retriggers"
        );
    }

    #[test]
    fn on_hand_played__seltzer_is_destroyed_after_its_tenth_hand() {
        // The counter is hands *completed*, so the tenth hand retriggers and the
        // joker is destroyed straight after it — not before it, which would give
        // only nine.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::SELTZER);

        let hand = bcards!("2S 5D 8C TS KH");
        for _ in 0..9 {
            board.on_hand_played(&hand);
        }
        assert_eq!(board.jokers.len(), 1, "nine hands spent, one left");

        board.on_hand_played(&hand);
        assert!(
            board.jokers.is_empty(),
            "the tenth hand spends the last one, so it is destroyed"
        );
        assert_eq!(board.score(), Score::new(40, 1));
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
