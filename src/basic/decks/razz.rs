use crate::basic::decks::cards;
use crate::prelude::{BasicCard, Decked, DeckedBase, Pip, Standard52};
use colored::Color;
use std::collections::HashMap;

/// [`Razz`](https://en.wikipedia.org/wiki/Razz_(poker)) deck where the cards are ordered from low
/// to high with the `Ace` counting as low.
///
/// This is an example of Deck generation using `BasicCard` configuration in `yaml` instead of
/// programmatically.
///
/// The yaml file was generated with the help of `CoPilot`, which created a version that didn't
/// actually work. You can see it in [`razz_bad.yaml`](yaml/razz_bad.yaml). This is why we test.
/// While the front line `Deck::<Razz>::validate()` didn't catch anything, this time,
/// the basic `from_str()` test did, after we had to debug. This is why it is always dangerous
/// to bury errors with just returning default. In a production system, I would add at least
/// logging in order to have some record of what's going on. In fact, let's add that to
/// `BasicCard::cards_from_file()` now.
///
/// This is an
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Razz {}

impl DeckedBase for Razz {
    fn base_vec() -> Vec<BasicCard> {
        BasicCard::cards_from_yaml_file("src/basic/decks/yaml/razz.yaml")
            .unwrap_or_else(|_| Vec::default())
    }

    fn colors() -> HashMap<Pip, Color> {
        Standard52::colors()
    }

    fn deck_name() -> String {
        "Razz".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

impl Decked<Razz> for Razz {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__decks__razz_tests {
    use super::*;
    use crate::prelude::{Deck, Decked, French};

    #[test]
    fn from_str() {
        let deck = Deck::<Razz>::deck();

        assert_eq!(deck.to_string(), "A♠ 2♠ 3♠ 4♠ 5♠ 6♠ 7♠ 8♠ 9♠ T♠ J♠ Q♠ K♠ A♥ 2♥ 3♥ 4♥ 5♥ 6♥ 7♥ 8♥ 9♥ T♥ J♥ Q♥ K♥ A♦ 2♦ 3♦ 4♦ 5♦ 6♦ 7♦ 8♦ 9♦ T♦ J♦ Q♦ K♦ A♣ 2♣ 3♣ 4♣ 5♣ 6♣ 7♣ 8♣ 9♣ T♣ J♣ Q♣ K♣");
    }

    #[test]
    fn deck__draw() {
        let mut deck = Deck::<Razz>::deck();
        assert_eq!(deck.draw(3).unwrap().to_string(), "A♠ 2♠ 3♠");
    }

    #[test]
    fn decked__validate() {
        assert!(Deck::<Razz>::validate());
    }
}
