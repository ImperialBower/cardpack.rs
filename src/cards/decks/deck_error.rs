#[derive(Debug)]
pub enum DeckError {
    InvalidIndex,
    Incomplete,
    PilePackMismatch,
}
