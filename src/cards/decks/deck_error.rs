#[derive(Debug, PartialEq)]
pub enum DeckError {
    InvalidIndex,
    Incomplete,
    PilePackMismatch,
}
