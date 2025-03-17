use crate::funky::decks::basic;
use crate::funky::types::mpip::{MPip, MPipType};
use crate::prelude::{CardError, Pip};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;
// region BCardType

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum BCardType {
    #[default]
    Basic,
    Joker,
    Planet,
    Spectral,
    Tarot,
    Voucher,
}

impl Display for BCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BCardType::Joker => 'j',
            BCardType::Planet => 'p',
            BCardType::Spectral => 's',
            BCardType::Tarot => 't',
            BCardType::Voucher => 'v',
            BCardType::Basic => '_',
        };
        write!(f, "{s}")
    }
}

impl From<char> for BCardType {
    fn from(c: char) -> Self {
        match c {
            'j' | 'J' => BCardType::Joker,
            'p' | 'P' => BCardType::Planet,
            's' | 'S' => BCardType::Spectral,
            't' | 'T' => BCardType::Tarot,
            'v' | 'V' => BCardType::Voucher,
            _ => BCardType::Basic,
        }
    }
}

impl FromStr for BCardType {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(CardError::InvalidIndex(s.to_string()));
        }
        Ok(s.chars().next().unwrap().into())
    }
}

// endregion BCardType

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BuffoonCard {
    pub suit: Pip,
    pub rank: Pip,
    pub card_type: BCardType,
    pub enhancement: MPip,
}

impl BuffoonCard {
    /// Adds to the underlying value of the rank of the card.
    ///
    /// There are two ways to increase the total chips for a `BuffoonCard`. The first is to
    /// add to the value of the underlying rank of the card. The other is to add the `MPipType::Chips`
    /// enhancement to the card.
    ///
    /// DIARY: I'm settling on a fluid style for the changes to the card. That's where
    /// instead of adjusting internal state, it returns a new card with the changes.
    ///
    /// I love `Self { rank, ..*self }` over my original hacky way:
    ///
    /// ```txt
    /// BuffoonCard {
    ///     suit: self.suit,
    ///     rank,
    ///     card_type: self.card_type,
    ///     enhancement: self.enhancement,
    /// }
    /// ```
    #[must_use]
    pub fn add_base_chips(&self, chips: usize) -> Self {
        let mut value = self.rank.value;
        value += chips;

        let rank = self.rank.update_value(value);

        Self { rank, ..*self }
    }

    fn get_enhanced_chips(&self) -> usize {
        let mut chips = 0;
        if let MPipType::Chips(c) = self.enhancement.pip_type {
            chips = c;
        };
        chips
    }

    #[must_use]
    pub fn get_chips(&self) -> usize {
        let mut chips = 0;
        print!("   chips: {}", self.rank.value);
        if let MPipType::Chips(c) = self.enhancement.pip_type {
            print!(" + {}", self.enhancement);
            chips += c;
        };
        println!();
        chips + self.rank.value
    }

    #[must_use]
    pub fn enhance(&self, enhancer: BuffoonCard) -> Self {
        println!("Enhancing {} with: {}", self, enhancer.enhancement);
        let bc = match enhancer.enhancement.pip_type {
            MPipType::Death(_)
            | MPipType::DoubleMoney(_)
            | MPipType::Hanged(_)
            | MPipType::Planet(_)
            | MPipType::RandomTarot(_)
            | MPipType::JokersValue(_)
            | MPipType::Diamonds
            | MPipType::Clubs
            | MPipType::Hearts
            | MPipType::Spades
            | MPipType::RandomJoker(_)
            | MPipType::Wheel(_) => *self,
            MPipType::Strength => basic::card::plus_rank(*self),
            _ => self.enhance_swap(enhancer.enhancement),
        };
        println!("Enhanced {bc}");
        bc
    }

    /// Function to implement mods where they are just straight up replacements.
    fn enhance_swap(&self, enhancement: MPip) -> Self {
        println!("Enhance swap: {enhancement}");
        Self {
            enhancement,
            ..*self
        }
    }
}

impl Display for BuffoonCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}-{}",
            self.rank, self.suit, self.card_type, self.enhancement
        )
    }
}

impl FromStr for BuffoonCard {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Inverts the order so that the highest card comes first.
impl Ord for BuffoonCard {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .suit
            .cmp(&self.suit)
            .then_with(|| other.rank.cmp(&self.rank))
    }
}

impl PartialOrd for BuffoonCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__buffoon_card_tests {
    use super::*;
    use crate::funky::decks::basic::card::*;
    use crate::funky::decks::tarot::card::*;
    use crate::funky::types::mpip::MPipType;
    use rstest::rstest;

    #[test]
    fn get_chips() {
        let ks = KING_SPADES.add_base_chips(11).add_base_chips(15);

        assert_eq!(ks.get_chips(), 36);
        assert_eq!(DEATH.get_chips(), 10);
    }

    #[test]
    fn enhance__magician() {
        assert_eq!(
            TEN_DIAMONDS.enhance_swap(MPip::LUCKY).enhancement,
            MPip::LUCKY
        );
        assert_eq!(TEN_DIAMONDS.enhance(MAGICIAN).enhancement, MPip::LUCKY);
        assert_eq!(TEN_DIAMONDS.enhance(MAGICIAN).get_chips(), 10);
    }

    /// The High Priestess creates up to 2 random Planet cards, so is a pass-through with no effect
    /// to the underlying card.
    #[test]
    fn enhance__high_priestess() {
        let card = QUEEN_SPADES.enhance(MAGICIAN);
        let original_enhancement = card.enhancement;

        assert_eq!(
            card.enhance(HIGH_PRIESTESS).enhancement,
            original_enhancement
        );
    }

    /// The Empress replaces the existing enhancement.
    #[test]
    fn enhance__empress() {
        let card = JACK_SPADES.enhance(MAGICIAN);

        assert_eq!(card.enhance(EMPRESS).enhancement, MPip::MOD_MULT_PLUS4);
    }

    #[test]
    fn enhance__emperor() {
        let card = TEN_DIAMONDS.enhance(MAGICIAN);
        let original_enhancement = card.enhancement;

        assert_eq!(card.enhance(EMPEROR).enhancement, original_enhancement);
    }

    #[test]
    fn enhance__hierophant() {
        let card = JACK_CLUBS;

        assert_eq!(card.get_chips(), 10);
        assert_eq!(card.enhance(HIEROPHANT).enhancement, MPip::BONUS);
        assert_eq!(card.enhance(HIEROPHANT).get_chips(), 40);
        assert_eq!(card.enhance(HIEROPHANT).add_base_chips(9).get_chips(), 49);
    }

    #[test]
    fn enhance__lovers() {
        let card = TEN_CLUBS;

        assert_eq!(card.get_chips(), 10);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(LOVERS).get_chips(), 10);
        assert_eq!(card.enhance(LOVERS).enhancement, MPip::WILD_SUIT);
    }

    #[test]
    fn enhance__chariot() {
        let card = NINE_CLUBS;

        assert_eq!(card.get_chips(), 9);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(THE_CHARIOT).get_chips(), 9);
        assert_eq!(card.enhance(THE_CHARIOT).enhancement, MPip::STEEL);
    }

    #[test]
    fn enhance__justice() {
        let card = EIGHT_CLUBS;

        assert_eq!(card.get_chips(), 8);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(JUSTICE).get_chips(), 8);
        assert_eq!(card.enhance(JUSTICE).enhancement, MPip::GLASS);
    }

    #[test]
    fn enhance__hermit() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(HERMIT).get_chips(), 7);
        assert_eq!(card.enhance(HERMIT).enhancement, MPip::BLANK);
    }

    #[test]
    fn enhance__wheel() {
        let card = SIX_CLUBS;

        assert_eq!(card.get_chips(), 6);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(WHEEL_OF_FORTUNE).get_chips(), 6);
        assert_eq!(card.enhance(WHEEL_OF_FORTUNE).enhancement, MPip::BLANK);
    }

    #[test]
    fn enhance__strength() {
        let card = FIVE_CLUBS;

        assert_eq!(card.get_chips(), 5);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(STRENGTH).get_chips(), 6);
        assert_eq!(card.enhance(STRENGTH).enhancement, MPip::BLANK);
        assert_eq!(card.enhance(STRENGTH), SIX_CLUBS);
        assert_eq!(SIX_CLUBS.enhance(STRENGTH), SEVEN_CLUBS);
        assert_eq!(SEVEN_CLUBS.enhance(STRENGTH), EIGHT_CLUBS);
        assert_eq!(EIGHT_CLUBS.enhance(STRENGTH), NINE_CLUBS);
        assert_eq!(NINE_CLUBS.enhance(STRENGTH), TEN_CLUBS);
        assert_eq!(TEN_CLUBS.enhance(STRENGTH), JACK_CLUBS);
        assert_eq!(JACK_CLUBS.enhance(STRENGTH), QUEEN_CLUBS);
        assert_eq!(QUEEN_CLUBS.enhance(STRENGTH), KING_CLUBS);
        assert_eq!(KING_CLUBS.enhance(STRENGTH), ACE_CLUBS);
        assert_eq!(ACE_CLUBS.enhance(STRENGTH), DEUCE_CLUBS);
        assert_eq!(DEUCE_CLUBS.enhance(STRENGTH), TREY_CLUBS);
        assert_eq!(TREY_CLUBS.enhance(STRENGTH), FOUR_CLUBS);
        assert_eq!(FOUR_CLUBS.enhance(STRENGTH), FIVE_CLUBS);
    }

    #[test]
    fn enhance__hanged() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(HANGED_MAN).get_chips(), 7);
        assert_eq!(card.enhance(HANGED_MAN).enhancement, MPip::BLANK);
    }

    #[test]
    fn enhance__death() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(DEATH).get_chips(), 7);
        assert_eq!(card.enhance(DEATH).enhancement, MPip::BLANK);
    }

    #[test]
    fn enhance__temperance() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(TEMPERANCE).get_chips(), 7);
        assert_eq!(card.enhance(TEMPERANCE).enhancement, MPip::BLANK);
    }

    #[test]
    fn enhance__devil() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(DEVIL).get_chips(), 7);
        assert_eq!(card.enhance(DEVIL).enhancement, MPip::DEVIL);
    }

    #[test]
    fn enhance__tower() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::BLANK);
        assert_eq!(card.enhance(TOWER).get_chips(), 7);
        assert_eq!(card.enhance(TOWER).enhancement, MPip::TOWER);
    }

    #[test]
    fn enhance__suits() {
        assert_eq!(SIX_CLUBS.enhance(STAR).enhancement, MPip::BLANK);
        assert_eq!(SIX_CLUBS.enhance(MOON).enhancement, MPip::BLANK);
        assert_eq!(SIX_CLUBS.enhance(SUN).enhancement, MPip::BLANK);
        assert_eq!(SIX_CLUBS.enhance(WORLD).enhancement, MPip::BLANK);
    }

    #[test]
    fn enhance__judgement() {
        assert_eq!(SIX_CLUBS.enhance(JUDGEMENT).enhancement, MPip::BLANK);
    }
}
