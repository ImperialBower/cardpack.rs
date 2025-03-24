use crate::funky::decks::{basic, tarot};
use crate::funky::types::mpip::MPip;
use crate::prelude::{BasicCard, CardError, FrenchSuit, Pip, PipType};
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

    #[must_use]
    pub fn basic_card(&self) -> BasicCard {
        BasicCard {
            suit: self.suit,
            rank: self.rank,
        }
    }

    #[must_use]
    pub fn calculate_mult_plus(&self, enhancer: BuffoonCard) -> usize {
        match enhancer.enhancement {
            MPip::MultPlus(value) => value,
            MPip::MultPlusOnSuit(value, suit) => {
                if self.suit.index == suit {
                    value
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    #[must_use]
    pub fn distance(&self, other: &BuffoonCard) -> usize {
        todo!()
    }

    fn get_enhanced_chips(&self) -> usize {
        let mut chips = 0;
        if let MPip::Chips(c) = self.enhancement {
            chips = c;
        };
        chips
    }

    #[must_use]
    pub fn get_chips(&self) -> usize {
        let mut chips = 0;
        print!("   chips: {}", self.rank.value);
        if let MPip::Chips(c) = self.enhancement {
            print!(" + {}", self.enhancement);
            chips += c;
        };
        println!();
        chips + self.rank.value
    }

    #[must_use]
    pub fn enhance(&self, enhancer: BuffoonCard) -> Self {
        println!("Enhancing {} with: {}", self, enhancer.enhancement);
        let bc = match enhancer.enhancement {
            MPip::Death(_)
            | MPip::DoubleMoney(_)
            | MPip::Hanged(_)
            | MPip::Planet(_)
            | MPip::RandomTarot(_)
            | MPip::JokersValue(_)
            | MPip::RandomJoker(_)
            | MPip::Odds1in(_) => *self,
            MPip::Strength => basic::card::plus_rank(*self),
            MPip::Diamonds(_) => basic::card::set_suit(*self, FrenchSuit::DIAMONDS),
            MPip::Clubs(_) => basic::card::set_suit(*self, FrenchSuit::CLUBS),
            MPip::Hearts(_) => basic::card::set_suit(*self, FrenchSuit::HEARTS),
            MPip::Spades(_) => basic::card::set_suit(*self, FrenchSuit::SPADES),
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
        (self.card_type == BCardType::Basic) && (self.enhancement == MPip::Blank)
    }

    #[must_use]
    pub fn is_joker(&self) -> bool {
        self.suit.pip_type == PipType::Joker
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

        // let index: String = s.chars().take(2).collect();

        match s.as_str() {
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
            "0M" | "FOOL" => Ok(tarot::card::FOOL),
            "1M" | "MAGICIAN" => Ok(tarot::card::MAGICIAN),
            "2M" | "HIGH_PRIESTESS" => Ok(tarot::card::HIGH_PRIESTESS),
            "3M" | "EMPRESS" => Ok(tarot::card::EMPRESS),
            "4M" | "EMPEROR" => Ok(tarot::card::EMPEROR),
            "5M" | "HIEROPHANT" => Ok(tarot::card::HIEROPHANT),
            "6M" | "LOVERS" => Ok(tarot::card::LOVERS),
            "7M" | "THE_CHARIOT" => Ok(tarot::card::THE_CHARIOT),
            "8M" | "STRENGTH" => Ok(tarot::card::STRENGTH),
            "9M" | "HERMIT" => Ok(tarot::card::HERMIT),
            "AM" | "WHEEL_OF_FORTUNE" => Ok(tarot::card::WHEEL_OF_FORTUNE),
            "BM" | "JUSTICE" => Ok(tarot::card::JUSTICE),
            "CM" | "HANGED_MAN" => Ok(tarot::card::HANGED_MAN),
            "DM" | "DEATH" => Ok(tarot::card::DEATH),
            "EM" | "TEMPERANCE" => Ok(tarot::card::TEMPERANCE),
            "FM" | "DEVIL" => Ok(tarot::card::DEVIL),
            "GM" | "TOWER" => Ok(tarot::card::TOWER),
            "HM" | "STAR" => Ok(tarot::card::STAR),
            "IM" | "MOON" => Ok(tarot::card::MOON),
            "JM" | "SUN" => Ok(tarot::card::SUN),
            "KM" | "JUDGEMENT" => Ok(tarot::card::JUDGEMENT),
            "LM" | "WORLD" => Ok(tarot::card::WORLD),

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

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__buffoon_card_tests {
    use super::*;
    use crate::bcard;
    use crate::funky::decks::basic::card::*;
    use crate::funky::decks::joker;
    use crate::funky::decks::tarot::card::*;

    #[test]
    fn get_chips() {
        let ks = KING_SPADES.add_base_chips(11).add_base_chips(15);

        assert_eq!(ks.get_chips(), 36);
        assert_eq!(DEATH.get_chips(), 10);
    }

    #[test]
    fn enhance__tarot__magician() {
        assert_eq!(
            TEN_DIAMONDS.enhance_swap(MPip::Lucky(5, 15)).enhancement,
            MPip::Lucky(5, 15)
        );
        assert_eq!(
            TEN_DIAMONDS.enhance(MAGICIAN).enhancement,
            MPip::Lucky(5, 15)
        );
        assert_eq!(TEN_DIAMONDS.enhance(MAGICIAN).get_chips(), 10);
    }

    /// The High Priestess creates up to 2 random Planet cards, so is a pass-through with no effect
    /// to the underlying card.
    #[test]
    fn enhance__tarot__high_priestess() {
        let card = QUEEN_SPADES.enhance(MAGICIAN);
        let original_enhancement = card.enhancement;

        assert_eq!(
            card.enhance(HIGH_PRIESTESS).enhancement,
            original_enhancement
        );
    }

    /// The Empress replaces the existing enhancement.
    #[test]
    fn enhance__tarot__empress() {
        let card = JACK_SPADES.enhance(MAGICIAN);

        assert_eq!(card.enhance(EMPRESS).enhancement, MPip::MultPlus(4));
    }

    #[test]
    fn enhance__tarot__emperor() {
        let card = TEN_DIAMONDS.enhance(MAGICIAN);
        let original_enhancement = card.enhancement;

        assert_eq!(card.enhance(EMPEROR).enhancement, original_enhancement);
    }

    #[test]
    fn enhance__tarot__hierophant() {
        let card = JACK_CLUBS;

        assert_eq!(card.get_chips(), 10);
        assert_eq!(card.enhance(HIEROPHANT).enhancement, MPip::BONUS);
        assert_eq!(card.enhance(HIEROPHANT).get_chips(), 40);
        assert_eq!(card.enhance(HIEROPHANT).add_base_chips(9).get_chips(), 49);
    }

    #[test]
    fn enhance__tarot__lovers() {
        let card = TEN_CLUBS;

        assert_eq!(card.get_chips(), 10);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(LOVERS).get_chips(), 10);
        assert_eq!(card.enhance(LOVERS).enhancement, MPip::Wild(PipType::Suit));
    }

    #[test]
    fn enhance__tarot__chariot() {
        let card = NINE_CLUBS;

        assert_eq!(card.get_chips(), 9);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(THE_CHARIOT).get_chips(), 9);
        assert_eq!(card.enhance(THE_CHARIOT).enhancement, MPip::STEEL);
    }

    #[test]
    fn enhance__tarot__justice() {
        let card = EIGHT_CLUBS;

        assert_eq!(card.get_chips(), 8);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(JUSTICE).get_chips(), 8);
        assert_eq!(card.enhance(JUSTICE).enhancement, MPip::Glass(2, 4));
    }

    #[test]
    fn enhance__tarot__hermit() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(HERMIT).get_chips(), 7);
        assert_eq!(card.enhance(HERMIT).enhancement, MPip::Blank);
    }

    #[test]
    fn enhance__tarot__wheel() {
        let card = SIX_CLUBS;

        assert_eq!(card.get_chips(), 6);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(WHEEL_OF_FORTUNE).get_chips(), 6);
        assert_eq!(card.enhance(WHEEL_OF_FORTUNE).enhancement, MPip::Blank);
    }

    #[test]
    fn enhance__tarot__strength() {
        let card = FIVE_CLUBS;

        assert_eq!(card.get_chips(), 5);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(STRENGTH).get_chips(), 6);
        assert_eq!(card.enhance(STRENGTH).enhancement, MPip::Blank);
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
    fn enhance__tarot__hanged() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(HANGED_MAN).get_chips(), 7);
        assert_eq!(card.enhance(HANGED_MAN).enhancement, MPip::Blank);
    }

    #[test]
    fn enhance__tarot__death() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(DEATH).get_chips(), 7);
        assert_eq!(card.enhance(DEATH).enhancement, MPip::Blank);
    }

    #[test]
    fn enhance__tarot__temperance() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(TEMPERANCE).get_chips(), 7);
        assert_eq!(card.enhance(TEMPERANCE).enhancement, MPip::Blank);
    }

    #[test]
    fn enhance__tarot__devil() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(DEVIL).get_chips(), 7);
        assert_eq!(card.enhance(DEVIL).enhancement, MPip::DEVIL);
    }

    #[test]
    fn enhance__tarot__tower() {
        let card = SEVEN_CLUBS;

        assert_eq!(card.get_chips(), 7);
        assert_eq!(card.enhancement, MPip::Blank);
        assert_eq!(card.enhance(TOWER).get_chips(), 7);
        assert_eq!(card.enhance(TOWER).enhancement, MPip::TOWER);
    }

    #[test]
    fn enhance__tarot__suits() {
        assert_eq!(SIX_CLUBS.enhance(STAR).enhancement, MPip::Blank);
        assert_eq!(SIX_CLUBS.enhance(STAR).suit, FrenchSuit::DIAMONDS);
        assert_eq!(SIX_CLUBS.enhance(MOON).enhancement, MPip::Blank);
        assert_eq!(SIX_CLUBS.enhance(MOON).suit, FrenchSuit::CLUBS);
        assert_eq!(SIX_CLUBS.enhance(SUN).enhancement, MPip::Blank);
        assert_eq!(SIX_CLUBS.enhance(SUN).suit, FrenchSuit::HEARTS);
        assert_eq!(SIX_CLUBS.enhance(WORLD).enhancement, MPip::Blank);
        assert_eq!(SIX_CLUBS.enhance(WORLD).suit, FrenchSuit::SPADES);
    }

    #[test]
    fn enhance__tarot__judgement() {
        assert_eq!(SIX_CLUBS.enhance(JUDGEMENT).enhancement, MPip::Blank);
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
    fn calculate_mult_plus__joker() {
        assert_eq!(bcard!(JD).calculate_mult_plus(joker::card::JOKER), 4);
        assert_eq!(bcard!(JD).calculate_mult_plus(bcard!(JOKER)), 4);
    }

    #[test]
    fn calculate_mult_plus__greedy_joker() {
        assert_eq!(bcard!(JD).calculate_mult_plus(bcard!(GREEDY)), 3);
        assert_eq!(bcard!(JC).calculate_mult_plus(joker::card::GREEDY_JOKER), 0);
        assert_eq!(bcard!(JS).calculate_mult_plus(bcard!(GREEDY)), 0);
        assert_eq!(bcard!(JH).calculate_mult_plus(joker::card::GREEDY_JOKER), 0);
    }
    #[test]
    fn calculate_mult_plus__lusty_joker() {
        assert_eq!(bcard!(JH).calculate_mult_plus(joker::card::LUSTY_JOKER), 3);
        assert_eq!(bcard!(JD).calculate_mult_plus(bcard!(LUSTY)), 0);
        assert_eq!(bcard!(JC).calculate_mult_plus(joker::card::LUSTY_JOKER), 0);
        assert_eq!(bcard!(JS).calculate_mult_plus(bcard!(LUSTY)), 0);
    }
}
