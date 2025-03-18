use crate::funky::decks::{basic, tarot};
use crate::funky::types::mpip::{MPip, MPipType};
use crate::prelude::{CardError, FrenchSuit, Pip};
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
    CommonJoker,
    UncommonJoker,
    RareJoker,
    LegendaryJoker,
    Planet,
    Spectral,
    Tarot,
    Voucher,
}

impl Display for BCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BCardType::CommonJoker => 'j',
            BCardType::UncommonJoker => 'u',
            BCardType::RareJoker => 'r',
            BCardType::LegendaryJoker => 'l',
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
            'j' | 'J' => BCardType::CommonJoker,
            'u' | 'U' => BCardType::UncommonJoker,
            'r' | 'R' => BCardType::RareJoker,
            'l' | 'L' => BCardType::LegendaryJoker,
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
    pub debuffed: bool,
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
            | MPipType::RandomJoker(_)
            | MPipType::Wheel(_) => *self,
            MPipType::Strength => basic::card::plus_rank(*self),
            MPipType::Diamonds(_) => basic::card::set_suit(*self, FrenchSuit::DIAMONDS),
            MPipType::Clubs(_) => basic::card::set_suit(*self, FrenchSuit::CLUBS),
            MPipType::Hearts(_) => basic::card::set_suit(*self, FrenchSuit::HEARTS),
            MPipType::Spades(_) => basic::card::set_suit(*self, FrenchSuit::SPADES),
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

    #[must_use]
    pub fn is_basic(&self) -> bool {
        (self.card_type == BCardType::Basic) && (self.enhancement == MPip::BLANK)
    }
}

impl Display for BuffoonCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_basic() {
            write!(f, "{}{}", self.rank.index, self.suit.index)
        } else {
            write!(
                f,
                "{}{}{}-{}",
                self.rank.index, self.suit.index, self.card_type, self.enhancement
            )
        }
    }
}

impl FromStr for BuffoonCard {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();

        if s.len() < 2 {
            return Err(CardError::InvalidIndex(s.to_string()));
        }

        let index: String = s.chars().take(2).collect();

        match index.as_str() {
            "AS" => Ok(basic::card::ACE_SPADES),
            "2S" => Ok(basic::card::DEUCE_SPADES),
            "3S" => Ok(basic::card::TREY_SPADES),
            "4S" => Ok(basic::card::FOUR_SPADES),
            "5S" => Ok(basic::card::FIVE_SPADES),
            "6S" => Ok(basic::card::SIX_SPADES),
            "7S" => Ok(basic::card::SEVEN_SPADES),
            "8S" => Ok(basic::card::EIGHT_SPADES),
            "9S" => Ok(basic::card::NINE_SPADES),
            "TS" => Ok(basic::card::TEN_SPADES),
            "JS" => Ok(basic::card::JACK_SPADES),
            "QS" => Ok(basic::card::QUEEN_SPADES),
            "KS" => Ok(basic::card::KING_SPADES),
            "AD" => Ok(basic::card::ACE_DIAMONDS),
            "2D" => Ok(basic::card::DEUCE_DIAMONDS),
            "3D" => Ok(basic::card::TREY_DIAMONDS),
            "4D" => Ok(basic::card::FOUR_DIAMONDS),
            "5D" => Ok(basic::card::FIVE_DIAMONDS),
            "6D" => Ok(basic::card::SIX_DIAMONDS),
            "7D" => Ok(basic::card::SEVEN_DIAMONDS),
            "8D" => Ok(basic::card::EIGHT_DIAMONDS),
            "9D" => Ok(basic::card::NINE_DIAMONDS),
            "TD" => Ok(basic::card::TEN_DIAMONDS),
            "JD" => Ok(basic::card::JACK_DIAMONDS),
            "QD" => Ok(basic::card::QUEEN_DIAMONDS),
            "KD" => Ok(basic::card::KING_DIAMONDS),
            "AH" => Ok(basic::card::ACE_HEARTS),
            "2H" => Ok(basic::card::DEUCE_HEARTS),
            "3H" => Ok(basic::card::TREY_HEARTS),
            "4H" => Ok(basic::card::FOUR_HEARTS),
            "5H" => Ok(basic::card::FIVE_HEARTS),
            "6H" => Ok(basic::card::SIX_HEARTS),
            "7H" => Ok(basic::card::SEVEN_HEARTS),
            "8H" => Ok(basic::card::EIGHT_HEARTS),
            "9H" => Ok(basic::card::NINE_HEARTS),
            "TH" => Ok(basic::card::TEN_HEARTS),
            "JH" => Ok(basic::card::JACK_HEARTS),
            "QH" => Ok(basic::card::QUEEN_HEARTS),
            "KH" => Ok(basic::card::KING_HEARTS),
            "AC" => Ok(basic::card::ACE_CLUBS),
            "2C" => Ok(basic::card::DEUCE_CLUBS),
            "3C" => Ok(basic::card::TREY_CLUBS),
            "4C" => Ok(basic::card::FOUR_CLUBS),
            "5C" => Ok(basic::card::FIVE_CLUBS),
            "6C" => Ok(basic::card::SIX_CLUBS),
            "7C" => Ok(basic::card::SEVEN_CLUBS),
            "8C" => Ok(basic::card::EIGHT_CLUBS),
            "9C" => Ok(basic::card::NINE_CLUBS),
            "TC" => Ok(basic::card::TEN_CLUBS),
            "JC" => Ok(basic::card::JACK_CLUBS),
            "QC" => Ok(basic::card::QUEEN_CLUBS),
            "KC" => Ok(basic::card::KING_CLUBS),

            // Tarot
            "0M" => Ok(tarot::card::FOOL),
            "1M" => Ok(tarot::card::MAGICIAN),
            "2M" => Ok(tarot::card::HIGH_PRIESTESS),
            "3M" => Ok(tarot::card::EMPRESS),
            "4M" => Ok(tarot::card::EMPEROR),
            "5M" => Ok(tarot::card::HIEROPHANT),
            "6M" => Ok(tarot::card::LOVERS),
            "7M" => Ok(tarot::card::THE_CHARIOT),
            "8M" => Ok(tarot::card::STRENGTH),
            "9M" => Ok(tarot::card::HERMIT),
            "AM" => Ok(tarot::card::WHEEL_OF_FORTUNE),
            "BM" => Ok(tarot::card::JUSTICE),
            "CM" => Ok(tarot::card::HANGED_MAN),
            "DM" => Ok(tarot::card::DEATH),
            "EM" => Ok(tarot::card::TEMPERANCE),
            "FM" => Ok(tarot::card::DEVIL),
            "GM" => Ok(tarot::card::TOWER),
            "HM" => Ok(tarot::card::STAR),
            "IM" => Ok(tarot::card::MOON),
            "JM" => Ok(tarot::card::SUN),
            "KM" => Ok(tarot::card::JUDGEMENT),
            "LM" => Ok(tarot::card::WORLD),

            _ => Ok(BuffoonCard::default()),
        }
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

#[macro_export]
macro_rules! bcard {
    (AS) => {
        basic::card::ACE_SPADES
    };
    (2S) => {
        basic::card::DEUCE_SPADES
    };
    (3S) => {
        basic::card::TREY_SPADES
    };
    (4S) => {
        basic::card::FOUR_SPADES
    };
    (5S) => {
        basic::card::FIVE_SPADES
    };
    (6S) => {
        basic::card::SIX_SPADES
    };
    (7S) => {
        basic::card::SEVEN_SPADES
    };
    (8S) => {
        basic::card::EIGHT_SPADES
    };
    (9S) => {
        basic::card::NINE_SPADES
    };
    (TS) => {
        basic::card::TEN_SPADES
    };
    (JS) => {
        basic::card::JACK_SPADES
    };
    (QS) => {
        basic::card::QUEEN_SPADES
    };
    (KS) => {
        basic::card::KING_SPADES
    };
    (AD) => {
        basic::card::ACE_DIAMONDS
    };
    (2D) => {
        basic::card::DEUCE_DIAMONDS
    };
    (3D) => {
        basic::card::TREY_DIAMONDS
    };
    (4D) => {
        basic::card::FOUR_DIAMONDS
    };
    (5D) => {
        basic::card::FIVE_DIAMONDS
    };
    (6D) => {
        basic::card::SIX_DIAMONDS
    };
    (7D) => {
        basic::card::SEVEN_DIAMONDS
    };
    (8D) => {
        basic::card::EIGHT_DIAMONDS
    };
    (9D) => {
        basic::card::NINE_DIAMONDS
    };
    (TD) => {
        basic::card::TEN_DIAMONDS
    };
    (JD) => {
        basic::card::JACK_DIAMONDS
    };
    (QD) => {
        basic::card::QUEEN_DIAMONDS
    };
    (KD) => {
        basic::card::KING_DIAMONDS
    };
    (AH) => {
        basic::card::ACE_HEARTS
    };
    (2H) => {
        basic::card::DEUCE_HEARTS
    };
    (3H) => {
        basic::card::TREY_HEARTS
    };
    (4H) => {
        basic::card::FOUR_HEARTS
    };
    (5H) => {
        basic::card::FIVE_HEARTS
    };
    (6H) => {
        basic::card::SIX_HEARTS
    };
    (7H) => {
        basic::card::SEVEN_HEARTS
    };
    (8H) => {
        basic::card::EIGHT_HEARTS
    };
    (9H) => {
        basic::card::NINE_HEARTS
    };
    (TH) => {
        basic::card::TEN_HEARTS
    };
    (JH) => {
        basic::card::JACK_HEARTS
    };
    (QH) => {
        basic::card::QUEEN_HEARTS
    };
    (KH) => {
        basic::card::KING_HEARTS
    };
    (AC) => {
        basic::card::ACE_CLUBS
    };
    (2C) => {
        basic::card::DEUCE_CLUBS
    };
    (3C) => {
        basic::card::TREY_CLUBS
    };
    (4C) => {
        basic::card::FOUR_CLUBS
    };
    (5C) => {
        basic::card::FIVE_CLUBS
    };
    (6C) => {
        basic::card::SIX_CLUBS
    };
    (7C) => {
        basic::card::SEVEN_CLUBS
    };
    (8C) => {
        basic::card::EIGHT_CLUBS
    };
    (9C) => {
        basic::card::NINE_CLUBS
    };
    (TC) => {
        basic::card::TEN_CLUBS
    };
    (JC) => {
        basic::card::JACK_CLUBS
    };
    (QC) => {
        basic::card::QUEEN_CLUBS
    };
    (KC) => {
        basic::card::KING_CLUBS
    };
    (0M) => {
        tarot::card::FOOL
    };
    (1M) => {
        tarot::card::MAGICIAN
    };
    (2M) => {
        tarot::card::HIGH_PRIESTESS
    };
    (3M) => {
        tarot::card::EMPRESS
    };
    (4M) => {
        tarot::card::EMPEROR
    };
    (5M) => {
        tarot::card::HIEROPHANT
    };
    (6M) => {
        tarot::card::LOVERS
    };
    (7M) => {
        tarot::card::THE_CHARIOT
    };
    (8M) => {
        tarot::card::STRENGTH
    };
    (9M) => {
        tarot::card::HERMIT
    };
    (AM) => {
        tarot::card::WHEEL_OF_FORTUNE
    };
    (BM) => {
        tarot::card::JUSTICE
    };
    (CM) => {
        tarot::card::HANGED_MAN
    };
    (DM) => {
        tarot::card::DEATH
    };
    (EM) => {
        tarot::card::TEMPERANCE
    };
    (FM) => {
        tarot::card::DEVIL
    };
    (GM) => {
        tarot::card::TOWER
    };
    (HM) => {
        tarot::card::STAR
    };
    (IM) => {
        tarot::card::MOON
    };
    (JM) => {
        tarot::card::SUN
    };
    (KM) => {
        tarot::card::JUDGEMENT
    };
    (LM) => {
        tarot::card::WORLD
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__buffoon_card_tests {
    use super::*;
    use crate::funky::decks::basic::card::*;
    use crate::funky::decks::tarot::card::*;

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

        assert_eq!(card.enhance(EMPRESS).enhancement, MPip::MULT_PLUS4);
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

    #[test]
    fn enhance_suits() {
        assert_eq!(SIX_CLUBS.enhance(STAR).suit, FrenchSuit::DIAMONDS);
        assert_eq!(SIX_CLUBS.enhance(MOON).suit, FrenchSuit::CLUBS);
        assert_eq!(SIX_CLUBS.enhance(SUN).suit, FrenchSuit::HEARTS);
        assert_eq!(SIX_CLUBS.enhance(WORLD).suit, FrenchSuit::SPADES);
    }

    #[test]
    fn from_str() {
        assert_eq!(BuffoonCard::from_str("__").unwrap(), BuffoonCard::default());
        assert_eq!(BuffoonCard::from_str("AS").unwrap(), ACE_SPADES);
        assert_eq!(BuffoonCard::from_str("2S").unwrap(), DEUCE_SPADES);
        assert_eq!(BuffoonCard::from_str("3S").unwrap(), TREY_SPADES);
        assert_eq!(BuffoonCard::from_str("4S").unwrap(), FOUR_SPADES);
        assert_eq!(BuffoonCard::from_str("5S").unwrap(), FIVE_SPADES);
        assert_eq!(BuffoonCard::from_str("6S").unwrap(), SIX_SPADES);
        assert_eq!(BuffoonCard::from_str("7S").unwrap(), SEVEN_SPADES);
        assert_eq!(BuffoonCard::from_str("8S").unwrap(), EIGHT_SPADES);
        assert_eq!(BuffoonCard::from_str("9S").unwrap(), NINE_SPADES);
        assert_eq!(BuffoonCard::from_str("TS").unwrap(), TEN_SPADES);
        assert_eq!(BuffoonCard::from_str("JS").unwrap(), JACK_SPADES);
        assert_eq!(BuffoonCard::from_str("QS").unwrap(), QUEEN_SPADES);
        assert_eq!(BuffoonCard::from_str("KS").unwrap(), KING_SPADES);
        assert_eq!(BuffoonCard::from_str("AD").unwrap(), ACE_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("2D").unwrap(), DEUCE_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("3D").unwrap(), TREY_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("4D").unwrap(), FOUR_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("5D").unwrap(), FIVE_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("6D").unwrap(), SIX_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("7D").unwrap(), SEVEN_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("8D").unwrap(), EIGHT_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("9D").unwrap(), NINE_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("TD").unwrap(), TEN_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("JD").unwrap(), JACK_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("QD").unwrap(), QUEEN_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("KD").unwrap(), KING_DIAMONDS);
        assert_eq!(BuffoonCard::from_str("AH").unwrap(), ACE_HEARTS);
        assert_eq!(BuffoonCard::from_str("2H").unwrap(), DEUCE_HEARTS);
        assert_eq!(BuffoonCard::from_str("3H").unwrap(), TREY_HEARTS);
        assert_eq!(BuffoonCard::from_str("4H").unwrap(), FOUR_HEARTS);
        assert_eq!(BuffoonCard::from_str("5H").unwrap(), FIVE_HEARTS);
        assert_eq!(BuffoonCard::from_str("6H").unwrap(), SIX_HEARTS);
        assert_eq!(BuffoonCard::from_str("7H").unwrap(), SEVEN_HEARTS);
        assert_eq!(BuffoonCard::from_str("8H").unwrap(), EIGHT_HEARTS);
        assert_eq!(BuffoonCard::from_str("9H").unwrap(), NINE_HEARTS);
        assert_eq!(BuffoonCard::from_str("TH").unwrap(), TEN_HEARTS);
        assert_eq!(BuffoonCard::from_str("JH").unwrap(), JACK_HEARTS);
        assert_eq!(BuffoonCard::from_str("QH").unwrap(), QUEEN_HEARTS);
        assert_eq!(BuffoonCard::from_str("KH").unwrap(), KING_HEARTS);
        assert_eq!(BuffoonCard::from_str("AC").unwrap(), ACE_CLUBS);
        assert_eq!(BuffoonCard::from_str("2C").unwrap(), DEUCE_CLUBS);
        assert_eq!(BuffoonCard::from_str("3C").unwrap(), TREY_CLUBS);
        assert_eq!(BuffoonCard::from_str("4C").unwrap(), FOUR_CLUBS);
        assert_eq!(BuffoonCard::from_str("5C").unwrap(), FIVE_CLUBS);
        assert_eq!(BuffoonCard::from_str("6C").unwrap(), SIX_CLUBS);
        assert_eq!(BuffoonCard::from_str("7C").unwrap(), SEVEN_CLUBS);
        assert_eq!(BuffoonCard::from_str("8C").unwrap(), EIGHT_CLUBS);
        assert_eq!(BuffoonCard::from_str("9C").unwrap(), NINE_CLUBS);
        assert_eq!(BuffoonCard::from_str("TC").unwrap(), TEN_CLUBS);
        assert_eq!(BuffoonCard::from_str("JC").unwrap(), JACK_CLUBS);
        assert_eq!(BuffoonCard::from_str("QC").unwrap(), QUEEN_CLUBS);
        assert_eq!(BuffoonCard::from_str("KC").unwrap(), KING_CLUBS);

        assert_eq!(BuffoonCard::from_str("0M").unwrap(), FOOL);
        assert_eq!(BuffoonCard::from_str("1M").unwrap(), MAGICIAN);
        assert_eq!(BuffoonCard::from_str("2M").unwrap(), HIGH_PRIESTESS);
        assert_eq!(BuffoonCard::from_str("3M").unwrap(), EMPRESS);
        assert_eq!(BuffoonCard::from_str("4M").unwrap(), EMPEROR);
        assert_eq!(BuffoonCard::from_str("5M").unwrap(), HIEROPHANT);
        assert_eq!(BuffoonCard::from_str("6M").unwrap(), LOVERS);
        assert_eq!(BuffoonCard::from_str("7M").unwrap(), THE_CHARIOT);
        assert_eq!(BuffoonCard::from_str("8M").unwrap(), STRENGTH);
        assert_eq!(BuffoonCard::from_str("9M").unwrap(), HERMIT);
        assert_eq!(BuffoonCard::from_str("AM").unwrap(), WHEEL_OF_FORTUNE);
        assert_eq!(BuffoonCard::from_str("BM").unwrap(), JUSTICE);
        assert_eq!(BuffoonCard::from_str("CM").unwrap(), HANGED_MAN);
        assert_eq!(BuffoonCard::from_str("DM").unwrap(), DEATH);
        assert_eq!(BuffoonCard::from_str("EM").unwrap(), TEMPERANCE);
        assert_eq!(BuffoonCard::from_str("FM").unwrap(), DEVIL);
        assert_eq!(BuffoonCard::from_str("GM").unwrap(), TOWER);
        assert_eq!(BuffoonCard::from_str("HM").unwrap(), STAR);
        assert_eq!(BuffoonCard::from_str("IM").unwrap(), MOON);
        assert_eq!(BuffoonCard::from_str("JM").unwrap(), SUN);
        assert_eq!(BuffoonCard::from_str("KM").unwrap(), JUDGEMENT);
        assert_eq!(BuffoonCard::from_str("LM").unwrap(), WORLD);
    }

    #[test]
    fn bcard() {
        assert_eq!(bcard!(AS), ACE_SPADES);
        assert_eq!(bcard!(2S), DEUCE_SPADES);
        assert_eq!(bcard!(3S), TREY_SPADES);
        assert_eq!(bcard!(4S), FOUR_SPADES);
        assert_eq!(bcard!(5S), FIVE_SPADES);
        assert_eq!(bcard!(6S), SIX_SPADES);
        assert_eq!(bcard!(7S), SEVEN_SPADES);
        assert_eq!(bcard!(8S), EIGHT_SPADES);
        assert_eq!(bcard!(9S), NINE_SPADES);
        assert_eq!(bcard!(TS), TEN_SPADES);
        assert_eq!(bcard!(JS), JACK_SPADES);
        assert_eq!(bcard!(QS), QUEEN_SPADES);
        assert_eq!(bcard!(KS), KING_SPADES);
        assert_eq!(bcard!(AD), ACE_DIAMONDS);
        assert_eq!(bcard!(2D), DEUCE_DIAMONDS);
        assert_eq!(bcard!(3D), TREY_DIAMONDS);
        assert_eq!(bcard!(4D), FOUR_DIAMONDS);
        assert_eq!(bcard!(5D), FIVE_DIAMONDS);
        assert_eq!(bcard!(6D), SIX_DIAMONDS);
        assert_eq!(bcard!(7D), SEVEN_DIAMONDS);
        assert_eq!(bcard!(8D), EIGHT_DIAMONDS);
        assert_eq!(bcard!(9D), NINE_DIAMONDS);
        assert_eq!(bcard!(TD), TEN_DIAMONDS);
        assert_eq!(bcard!(JD), JACK_DIAMONDS);
        assert_eq!(bcard!(QD), QUEEN_DIAMONDS);
        assert_eq!(bcard!(KD), KING_DIAMONDS);
        assert_eq!(bcard!(AH), ACE_HEARTS);
        assert_eq!(bcard!(2H), DEUCE_HEARTS);
        assert_eq!(bcard!(3H), TREY_HEARTS);
        assert_eq!(bcard!(4H), FOUR_HEARTS);
        assert_eq!(bcard!(5H), FIVE_HEARTS);
        assert_eq!(bcard!(6H), SIX_HEARTS);
        assert_eq!(bcard!(7H), SEVEN_HEARTS);
        assert_eq!(bcard!(8H), EIGHT_HEARTS);
        assert_eq!(bcard!(9H), NINE_HEARTS);
        assert_eq!(bcard!(TH), TEN_HEARTS);
        assert_eq!(bcard!(JH), JACK_HEARTS);
        assert_eq!(bcard!(QH), QUEEN_HEARTS);
        assert_eq!(bcard!(KH), KING_HEARTS);
        assert_eq!(bcard!(AC), ACE_CLUBS);
        assert_eq!(bcard!(2C), DEUCE_CLUBS);
        assert_eq!(bcard!(3C), TREY_CLUBS);
        assert_eq!(bcard!(4C), FOUR_CLUBS);
        assert_eq!(bcard!(5C), FIVE_CLUBS);
        assert_eq!(bcard!(6C), SIX_CLUBS);
        assert_eq!(bcard!(7C), SEVEN_CLUBS);
        assert_eq!(bcard!(8C), EIGHT_CLUBS);
        assert_eq!(bcard!(9C), NINE_CLUBS);
        assert_eq!(bcard!(TC), TEN_CLUBS);
        assert_eq!(bcard!(JC), JACK_CLUBS);
        assert_eq!(bcard!(QC), QUEEN_CLUBS);
        assert_eq!(bcard!(KC), KING_CLUBS);
        assert_eq!(bcard!(0M), FOOL);
        assert_eq!(bcard!(1M), MAGICIAN);
        assert_eq!(bcard!(2M), HIGH_PRIESTESS);
        assert_eq!(bcard!(3M), EMPRESS);
        assert_eq!(bcard!(4M), EMPEROR);
        assert_eq!(bcard!(5M), HIEROPHANT);
        assert_eq!(bcard!(6M), LOVERS);
        assert_eq!(bcard!(7M), THE_CHARIOT);
        assert_eq!(bcard!(8M), STRENGTH);
        assert_eq!(bcard!(9M), HERMIT);
        assert_eq!(bcard!(AM), WHEEL_OF_FORTUNE);
        assert_eq!(bcard!(BM), JUSTICE);
        assert_eq!(bcard!(CM), HANGED_MAN);
        assert_eq!(bcard!(DM), DEATH);
        assert_eq!(bcard!(EM), TEMPERANCE);
        assert_eq!(bcard!(FM), DEVIL);
        assert_eq!(bcard!(GM), TOWER);
        assert_eq!(bcard!(HM), STAR);
        assert_eq!(bcard!(IM), MOON);
        assert_eq!(bcard!(JM), SUN);
        assert_eq!(bcard!(KM), JUDGEMENT);
        assert_eq!(bcard!(LM), WORLD);
    }
}
