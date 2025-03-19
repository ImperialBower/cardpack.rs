#[macro_export]
macro_rules! bcard {
    (AS) => {
        crate::funky::decks::basic::card::ACE_SPADES
    };
    (2S) => {
        crate::funky::decks::basic::card::DEUCE_SPADES
    };
    (3S) => {
        crate::funky::decks::basic::card::TREY_SPADES
    };
    (4S) => {
        crate::funky::decks::basic::card::FOUR_SPADES
    };
    (5S) => {
        crate::funky::decks::basic::card::FIVE_SPADES
    };
    (6S) => {
        crate::funky::decks::basic::card::SIX_SPADES
    };
    (7S) => {
        crate::funky::decks::basic::card::SEVEN_SPADES
    };
    (8S) => {
        crate::funky::decks::basic::card::EIGHT_SPADES
    };
    (9S) => {
        crate::funky::decks::basic::card::NINE_SPADES
    };
    (TS) => {
        crate::funky::decks::basic::card::TEN_SPADES
    };
    (JS) => {
        crate::funky::decks::basic::card::JACK_SPADES
    };
    (QS) => {
        crate::funky::decks::basic::card::QUEEN_SPADES
    };
    (KS) => {
        crate::funky::decks::basic::card::KING_SPADES
    };
    (AD) => {
        crate::funky::decks::basic::card::ACE_DIAMONDS
    };
    (2D) => {
        crate::funky::decks::basic::card::DEUCE_DIAMONDS
    };
    (3D) => {
        crate::funky::decks::basic::card::TREY_DIAMONDS
    };
    (4D) => {
        crate::funky::decks::basic::card::FOUR_DIAMONDS
    };
    (5D) => {
        crate::funky::decks::basic::card::FIVE_DIAMONDS
    };
    (6D) => {
        crate::funky::decks::basic::card::SIX_DIAMONDS
    };
    (7D) => {
        crate::funky::decks::basic::card::SEVEN_DIAMONDS
    };
    (8D) => {
        crate::funky::decks::basic::card::EIGHT_DIAMONDS
    };
    (9D) => {
        crate::funky::decks::basic::card::NINE_DIAMONDS
    };
    (TD) => {
        crate::funky::decks::basic::card::TEN_DIAMONDS
    };
    (JD) => {
        crate::funky::decks::basic::card::JACK_DIAMONDS
    };
    (QD) => {
        crate::funky::decks::basic::card::QUEEN_DIAMONDS
    };
    (KD) => {
        crate::funky::decks::basic::card::KING_DIAMONDS
    };
    (AH) => {
        crate::funky::decks::basic::card::ACE_HEARTS
    };
    (2H) => {
        crate::funky::decks::basic::card::DEUCE_HEARTS
    };
    (3H) => {
        crate::funky::decks::basic::card::TREY_HEARTS
    };
    (4H) => {
        crate::funky::decks::basic::card::FOUR_HEARTS
    };
    (5H) => {
        crate::funky::decks::basic::card::FIVE_HEARTS
    };
    (6H) => {
        crate::funky::decks::basic::card::SIX_HEARTS
    };
    (7H) => {
        crate::funky::decks::basic::card::SEVEN_HEARTS
    };
    (8H) => {
        crate::funky::decks::basic::card::EIGHT_HEARTS
    };
    (9H) => {
        crate::funky::decks::basic::card::NINE_HEARTS
    };
    (TH) => {
        crate::funky::decks::basic::card::TEN_HEARTS
    };
    (JH) => {
        crate::funky::decks::basic::card::JACK_HEARTS
    };
    (QH) => {
        crate::funky::decks::basic::card::QUEEN_HEARTS
    };
    (KH) => {
        crate::funky::decks::basic::card::KING_HEARTS
    };
    (AC) => {
        crate::funky::decks::basic::card::ACE_CLUBS
    };
    (2C) => {
        crate::funky::decks::basic::card::DEUCE_CLUBS
    };
    (3C) => {
        crate::funky::decks::basic::card::TREY_CLUBS
    };
    (4C) => {
        crate::funky::decks::basic::card::FOUR_CLUBS
    };
    (5C) => {
        crate::funky::decks::basic::card::FIVE_CLUBS
    };
    (6C) => {
        crate::funky::decks::basic::card::SIX_CLUBS
    };
    (7C) => {
        crate::funky::decks::basic::card::SEVEN_CLUBS
    };
    (8C) => {
        crate::funky::decks::basic::card::EIGHT_CLUBS
    };
    (9C) => {
        crate::funky::decks::basic::card::NINE_CLUBS
    };
    (TC) => {
        crate::funky::decks::basic::card::TEN_CLUBS
    };
    (JC) => {
        crate::funky::decks::basic::card::JACK_CLUBS
    };
    (QC) => {
        crate::funky::decks::basic::card::QUEEN_CLUBS
    };
    (KC) => {
        crate::funky::decks::basic::card::KING_CLUBS
    };
    (0M) => {
        crate::funky::decks::tarot::card::FOOL
    };
    (1M) => {
        crate::funky::decks::tarot::card::MAGICIAN
    };
    (2M) => {
        crate::funky::decks::tarot::card::HIGH_PRIESTESS
    };
    (3M) => {
        crate::funky::decks::tarot::card::EMPRESS
    };
    (4M) => {
        crate::funky::decks::tarot::card::EMPEROR
    };
    (5M) => {
        crate::funky::decks::tarot::card::HIEROPHANT
    };
    (6M) => {
        crate::funky::decks::tarot::card::LOVERS
    };
    (7M) => {
        crate::funky::decks::tarot::card::THE_CHARIOT
    };
    (8M) => {
        crate::funky::decks::tarot::card::STRENGTH
    };
    (9M) => {
        crate::funky::decks::tarot::card::HERMIT
    };
    (AM) => {
        crate::funky::decks::tarot::card::WHEEL_OF_FORTUNE
    };
    (BM) => {
        crate::funky::decks::tarot::card::JUSTICE
    };
    (CM) => {
        crate::funky::decks::tarot::card::HANGED_MAN
    };
    (DM) => {
        crate::funky::decks::tarot::card::DEATH
    };
    (EM) => {
        crate::funky::decks::tarot::card::TEMPERANCE
    };
    (FM) => {
        crate::funky::decks::tarot::card::DEVIL
    };
    (GM) => {
        crate::funky::decks::tarot::card::TOWER
    };
    (HM) => {
        crate::funky::decks::tarot::card::STAR
    };
    (IM) => {
        crate::funky::decks::tarot::card::MOON
    };
    (JM) => {
        crate::funky::decks::tarot::card::SUN
    };
    (KM) => {
        crate::funky::decks::tarot::card::JUDGEMENT
    };
    (LM) => {
        crate::funky::decks::tarot::card::WORLD
    };
    (JOKER) => {
        crate::funky::decks::joker::card::JOKER
    };
    (GREEDY) => {
        crate::funky::decks::joker::card::GREEDY_JOKER
    };
    (LUSTY) => {
        crate::funky::decks::joker::card::LUSTY_JOKER
    };
}

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! bcards {
    ($card_str:expr) => {
        BuffoonPile::forgiving_from_str($card_str)
    };
}