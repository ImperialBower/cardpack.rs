use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum CardError {
    #[error("Fubar should not be possible.")]
    Fubar,

    #[error("Invalid Card: `{0}`")]
    InvalidCard(String),

    #[error("Invalid Card Count: `{0}`")]
    InvalidCardCount(usize),

    #[error("Invalid File Path: `{0}`")]
    InvalidFilePath(String),

    #[error(
        "Invalid FluentName: `{0}`. Must be alphanumeric with hyphens, en-dashes, or em-dashes."
    )]
    InvalidFluentName(String),

    #[error("Invalid Fluent Rank: `{0}`. Must be single char.")]
    InvalidFluentRank(String),

    #[error("Invalid Index: `{0}`")]
    InvalidIndex(String),

    #[error("Not enough cards: `{0}` missing")]
    NotEnoughCards(usize),

    #[error("Too many cards: `{0}` extra")]
    TooManyCards(usize),
}
