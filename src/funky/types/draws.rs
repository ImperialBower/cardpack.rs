use serde::{Deserialize, Serialize};

/// # Diary
///
/// 1. Start with the struct.
/// 2. Add the `#[derive]` attributes.
/// 3. Implement the `Display` trait.
///
/// I am obsessed with having a simple way to print out the values of data structures. Perhaps
/// this is a callback to my early days when I didn't know how to do logging, or testing and
/// debugging for that matter. When things went wrong, I would dump everything out as a print
/// statement. That's true for everybody I guess. The faster you get past it the better.
///
/// Still, `Display` gives me a way to quickly write unit tests that act as sanity checks, and I
/// use them all the time for my interactive example programs where I can pluy with the libraries.
/// Unit tests are one thing, but they don't replace being able to play with the code. 
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct Draws {
    pub hands_to_play: usize,
    pub discards: usize,
}

impl std::fmt::Display for Draws {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Draws {{ hands_to_play: {}, discards: {} }}",
            self.hands_to_play, self.discards
        )
    }
}

impl Draws {
    #[must_use]
    pub fn new(hands_to_play: usize, discards: usize) -> Self {
        Self {
            hands_to_play,
            discards,
        }
    }
}