//! `SlotId` — a card's public name, carrying no knowledge of its value.

use core::fmt::{Display, Formatter};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A stable, public handle for one card in a shoe.
///
/// Assigned when a shoe is set up and carried thereafter, so shuffling
/// permutes *order* while every card keeps its name. This is what lets a
/// ledger say "seat 3 revealed slot 17" without saying what slot 17 is.
///
/// Deliberately **not** the [`Ordinal`](crate::basic::types::ordinal::Ordinal)
/// — that would be the card. A slot's number carries no information about
/// rank or suit *provided the shoe was shuffled before slots were assigned*
/// (see `docs/EPIC-04_Sealed_Decks.md`, decision 7).
///
/// ```
/// use cardpack::prelude::*;
///
/// let s = SlotId::new(17);
/// assert_eq!(s.get(), 17);
/// assert_eq!(s.to_string(), "17");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlotId(u16);

impl SlotId {
    #[must_use]
    pub const fn new(n: u16) -> Self {
        Self(n)
    }

    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl Display for SlotId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__slot_tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn slot_id__new_get_index_display_order() {
        let s = SlotId::new(17);
        assert_eq!(s.get(), 17);
        assert_eq!(s.index(), 17_usize);
        assert_eq!(s.to_string(), "17");
        assert!(SlotId::new(1) < SlotId::new(2));
        assert_eq!(SlotId::default(), SlotId::new(0));
    }
}
