use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum VSupSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl VSupSuit {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn short_string(&self) -> &'static str {
        match *self {
            VSupSuit::Spades => "s",
            VSupSuit::Hearts => "h",
            VSupSuit::Diamonds => "d",
            VSupSuit::Clubs => "c",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum VSupValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    // no jokers
}

impl VSupValue {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn short_string(&self) -> &'static str {
        match *self {
            VSupValue::Two => "2",
            VSupValue::Three => "3",
            VSupValue::Four => "4",
            VSupValue::Five => "5",
            VSupValue::Six => "6",
            VSupValue::Seven => "7",
            VSupValue::Eight => "8",
            VSupValue::Nine => "9",
            VSupValue::Ten => "T",
            VSupValue::Jack => "J",
            VSupValue::Queen => "Q",
            VSupValue::King => "K",
            VSupValue::Ace => "A",
        }
    }
}

//TODO: debug still relevant? It was used to print a vec of cards.
/// An unnamed tuple with Value and Suit.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct VSupCard {
    pub value: VSupValue,
    pub suit: VSupSuit,
}

impl VSupCard {
    #[allow(dead_code)]
    pub fn new(value: VSupValue, suit: VSupSuit) -> VSupCard {
        VSupCard { value, suit }
    }
}

// so cards can be printed using fmt method
impl fmt::Display for VSupCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.value.short_string(),
            self.suit.short_string()
        )
    }
}
