#[derive(Debug, PartialEq)]
pub enum CardError {
    InvalidCard,
    InvalidCardCount,
    InvalidIndex,
    NotEnoughCards,
    TooManyCards,
}
