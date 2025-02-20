#[derive(Debug, PartialEq)]
pub enum DeckError {
    DuplicateCard,
    InvalidIndex,
    Incomplete,
    PilePackMismatch,
}
