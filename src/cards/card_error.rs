#[derive(Debug, PartialEq)]
pub enum CardError {
    InvalidCard,
    NotEnoughCards,
    TooManyCards,
}
