#[macro_export]
macro_rules! bcard {
    (AS) => {
        $crate::funky::decks::basic::card::ACE_SPADES
    };
    (2S) => {
        $crate::funky::decks::basic::card::DEUCE_SPADES
    };
    (3S) => {
        $crate::funky::decks::basic::card::TREY_SPADES
    };
    (4S) => {
        $crate::funky::decks::basic::card::FOUR_SPADES
    };
    (5S) => {
        $crate::funky::decks::basic::card::FIVE_SPADES
    };
    (6S) => {
        $crate::funky::decks::basic::card::SIX_SPADES
    };
    (7S) => {
        $crate::funky::decks::basic::card::SEVEN_SPADES
    };
    (8S) => {
        $crate::funky::decks::basic::card::EIGHT_SPADES
    };
    (9S) => {
        $crate::funky::decks::basic::card::NINE_SPADES
    };
    (TS) => {
        $crate::funky::decks::basic::card::TEN_SPADES
    };
    (JS) => {
        $crate::funky::decks::basic::card::JACK_SPADES
    };
    (QS) => {
        $crate::funky::decks::basic::card::QUEEN_SPADES
    };
    (KS) => {
        $crate::funky::decks::basic::card::KING_SPADES
    };
    (AD) => {
        $crate::funky::decks::basic::card::ACE_DIAMONDS
    };
    (2D) => {
        $crate::funky::decks::basic::card::DEUCE_DIAMONDS
    };
    (3D) => {
        $crate::funky::decks::basic::card::TREY_DIAMONDS
    };
    (4D) => {
        $crate::funky::decks::basic::card::FOUR_DIAMONDS
    };
    (5D) => {
        $crate::funky::decks::basic::card::FIVE_DIAMONDS
    };
    (6D) => {
        $crate::funky::decks::basic::card::SIX_DIAMONDS
    };
    (7D) => {
        $crate::funky::decks::basic::card::SEVEN_DIAMONDS
    };
    (8D) => {
        $crate::funky::decks::basic::card::EIGHT_DIAMONDS
    };
    (9D) => {
        $crate::funky::decks::basic::card::NINE_DIAMONDS
    };
    (TD) => {
        $crate::funky::decks::basic::card::TEN_DIAMONDS
    };
    (JD) => {
        $crate::funky::decks::basic::card::JACK_DIAMONDS
    };
    (QD) => {
        $crate::funky::decks::basic::card::QUEEN_DIAMONDS
    };
    (KD) => {
        $crate::funky::decks::basic::card::KING_DIAMONDS
    };
    (AH) => {
        $crate::funky::decks::basic::card::ACE_HEARTS
    };
    (2H) => {
        $crate::funky::decks::basic::card::DEUCE_HEARTS
    };
    (3H) => {
        $crate::funky::decks::basic::card::TREY_HEARTS
    };
    (4H) => {
        $crate::funky::decks::basic::card::FOUR_HEARTS
    };
    (5H) => {
        $crate::funky::decks::basic::card::FIVE_HEARTS
    };
    (6H) => {
        $crate::funky::decks::basic::card::SIX_HEARTS
    };
    (7H) => {
        $crate::funky::decks::basic::card::SEVEN_HEARTS
    };
    (8H) => {
        $crate::funky::decks::basic::card::EIGHT_HEARTS
    };
    (9H) => {
        $crate::funky::decks::basic::card::NINE_HEARTS
    };
    (TH) => {
        $crate::funky::decks::basic::card::TEN_HEARTS
    };
    (JH) => {
        $crate::funky::decks::basic::card::JACK_HEARTS
    };
    (QH) => {
        $crate::funky::decks::basic::card::QUEEN_HEARTS
    };
    (KH) => {
        $crate::funky::decks::basic::card::KING_HEARTS
    };
    (AC) => {
        $crate::funky::decks::basic::card::ACE_CLUBS
    };
    (2C) => {
        $crate::funky::decks::basic::card::DEUCE_CLUBS
    };
    (3C) => {
        $crate::funky::decks::basic::card::TREY_CLUBS
    };
    (4C) => {
        $crate::funky::decks::basic::card::FOUR_CLUBS
    };
    (5C) => {
        $crate::funky::decks::basic::card::FIVE_CLUBS
    };
    (6C) => {
        $crate::funky::decks::basic::card::SIX_CLUBS
    };
    (7C) => {
        $crate::funky::decks::basic::card::SEVEN_CLUBS
    };
    (8C) => {
        $crate::funky::decks::basic::card::EIGHT_CLUBS
    };
    (9C) => {
        $crate::funky::decks::basic::card::NINE_CLUBS
    };
    (TC) => {
        $crate::funky::decks::basic::card::TEN_CLUBS
    };
    (JC) => {
        $crate::funky::decks::basic::card::JACK_CLUBS
    };
    (QC) => {
        $crate::funky::decks::basic::card::QUEEN_CLUBS
    };
    (KC) => {
        $crate::funky::decks::basic::card::KING_CLUBS
    };
    (FOOL) => {
        $crate::funky::decks::tarot::card::FOOL
    };
    (MAGICIAN) => {
        $crate::funky::decks::tarot::card::MAGICIAN
    };
    (HIGH_PRIESTESS) => {
        $crate::funky::decks::tarot::card::HIGH_PRIESTESS
    };
    (EMPRESS) => {
        $crate::funky::decks::tarot::card::EMPRESS
    };
    (EMPEROR) => {
        $crate::funky::decks::tarot::card::EMPEROR
    };
    (HIEROPHANT) => {
        $crate::funky::decks::tarot::card::HIEROPHANT
    };
    (LOVERS) => {
        $crate::funky::decks::tarot::card::LOVERS
    };
    (THE_CHARIOT) => {
        $crate::funky::decks::tarot::card::THE_CHARIOT
    };
    (STRENGTH) => {
        $crate::funky::decks::tarot::card::STRENGTH
    };
    (HERMIT) => {
        $crate::funky::decks::tarot::card::HERMIT
    };
    (WHEEL_OF_FORTUNE) => {
        $crate::funky::decks::tarot::card::WHEEL_OF_FORTUNE
    };
    (JUSTICE) => {
        $crate::funky::decks::tarot::card::JUSTICE
    };
    (HANGED_MAN) => {
        $crate::funky::decks::tarot::card::HANGED_MAN
    };
    (DEATH) => {
        $crate::funky::decks::tarot::card::DEATH
    };
    (TEMPERANCE) => {
        $crate::funky::decks::tarot::card::TEMPERANCE
    };
    (DEVIL) => {
        $crate::funky::decks::tarot::card::DEVIL
    };
    (TOWER) => {
        $crate::funky::decks::tarot::card::TOWER
    };
    (STAR) => {
        $crate::funky::decks::tarot::card::STAR
    };
    (MOON) => {
        $crate::funky::decks::tarot::card::MOON
    };
    (SUN) => {
        $crate::funky::decks::tarot::card::SUN
    };
    (JUDGEMENT) => {
        $crate::funky::decks::tarot::card::JUDGEMENT
    };
    (WORLD) => {
        $crate::funky::decks::tarot::card::WORLD
    };
    (JOKER) => {
        $crate::funky::decks::joker::card::JOKER
    };
    (GREEDY) => {
        $crate::funky::decks::joker::card::GREEDY_JOKER
    };
    (LUSTY) => {
        $crate::funky::decks::joker::card::LUSTY_JOKER
    };
    (WRATHFUL) => {
        $crate::funky::decks::joker::card::WRATHFUL_JOKER
    };
    (GLUTTONOUS) => {
        $crate::funky::decks::joker::card::GLUTTONOUS_JOKER
    };
    (JOLLY) => {
        $crate::funky::decks::joker::card::JOLLY_JOKER
    };
    (ZANY) => {
        $crate::funky::decks::joker::card::ZANY_JOKER
    };
    (MAD) => {
        $crate::funky::decks::joker::card::MAD_JOKER
    };
    (CRAZY) => {
        $crate::funky::decks::joker::card::CRAZY_JOKER
    };
    (DROLL) => {
        $crate::funky::decks::joker::card::DROLL_JOKER
    };
    (SLY) => {
        $crate::funky::decks::joker::card::SLY_JOKER
    };
    (WILY) => {
        $crate::funky::decks::joker::card::WILY_JOKER
    };
    (CLEVER) => {
        $crate::funky::decks::joker::card::CLEVER_JOKER
    };
    (DEVIOUS) => {
        $crate::funky::decks::joker::card::DEVIOUS_JOKER
    };
    (CRAFTY) => {
        $crate::funky::decks::joker::card::CRAFTY_JOKER
    };
    (HALF) => {
        $crate::funky::decks::joker::card::HALF_JOKER
    };
    (STENCIL) => {
        $crate::funky::decks::joker::card::JOKER_STENCIL
    };
    (FOUR_FINGERS) => {
        $crate::funky::decks::joker::card::FOUR_FINGERS
    };
    (MIME) => {
        $crate::funky::decks::joker::card::MIME
    };
    (CREDIT_CARD) => {
        $crate::funky::decks::joker::card::CREDIT_CARD
    };
    (CEREMONIAL_DAGGER) => {
        $crate::funky::decks::joker::card::CEREMONIAL_DAGGER
    };
    (BANNER) => {
        $crate::funky::decks::joker::card::BANNER
    };
    (MARBLE) => {
        $crate::funky::decks::joker::card::MARBLE_JOKER
    };
    (LOYALTY_CARD) => {
        $crate::funky::decks::joker::card::LOYALTY_CARD
    };
}

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! bcards {
    ($card_str:expr) => {
        BuffoonPile::forgiving_from_str($card_str)
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__macros_tests {
    use crate::preludes::funky::*;

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
        assert_eq!(bcard!(FOOL), FOOL);
        assert_eq!(bcard!(MAGICIAN), MAGICIAN);
        assert_eq!(bcard!(HIGH_PRIESTESS), HIGH_PRIESTESS);
        assert_eq!(bcard!(EMPRESS), EMPRESS);
        assert_eq!(bcard!(EMPEROR), EMPEROR);
        assert_eq!(bcard!(HIEROPHANT), HIEROPHANT);
        assert_eq!(bcard!(LOVERS), LOVERS);
        assert_eq!(bcard!(THE_CHARIOT), THE_CHARIOT);
        assert_eq!(bcard!(STRENGTH), STRENGTH);
        assert_eq!(bcard!(HERMIT), HERMIT);
        assert_eq!(bcard!(WHEEL_OF_FORTUNE), WHEEL_OF_FORTUNE);
        assert_eq!(bcard!(JUSTICE), JUSTICE);
        assert_eq!(bcard!(HANGED_MAN), HANGED_MAN);
        assert_eq!(bcard!(DEATH), DEATH);
        assert_eq!(bcard!(TEMPERANCE), TEMPERANCE);
        assert_eq!(bcard!(DEVIL), DEVIL);
        assert_eq!(bcard!(TOWER), TOWER);
        assert_eq!(bcard!(STAR), STAR);
        assert_eq!(bcard!(MOON), MOON);
        assert_eq!(bcard!(SUN), SUN);
        assert_eq!(bcard!(JUDGEMENT), JUDGEMENT);
        assert_eq!(bcard!(WORLD), WORLD);
        assert_eq!(bcard!(GREEDY), GREEDY_JOKER);
        assert_eq!(bcard!(LUSTY), LUSTY_JOKER);
        assert_eq!(bcard!(WRATHFUL), WRATHFUL_JOKER);
        assert_eq!(bcard!(GLUTTONOUS), GLUTTONOUS_JOKER);
        assert_eq!(bcard!(JOLLY), JOLLY_JOKER);
        assert_eq!(bcard!(ZANY), ZANY_JOKER);
        assert_eq!(bcard!(MAD), MAD_JOKER);
    }
}
