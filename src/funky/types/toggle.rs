use crate::preludes::funky::BuffoonCard;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ToggleCard {
    pub card: BuffoonCard,
    pub toggle: Toggle,
}

impl ToggleCard {
    #[must_use]
    pub fn new(card: BuffoonCard) -> Self {
        // Create a new ToggleCard with a default Toggle
        Self {
            card,
            toggle: Toggle::default(),
        }
    }

    pub fn is_selected(&self) -> bool {
        self.toggle.is_on()
    }

    pub fn toggle(&self) {
        self.toggle.toggle();
    }
}

impl Display for ToggleCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.toggle, self.card)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Toggle {
    is_on: RefCell<bool>,
}

impl Toggle {
    pub fn is_on(&self) -> bool {
        *self.is_on.borrow()
    }

    pub fn toggle(&self) {
        // Toggle the boolean value inside the RefCell
        let mut is_on = self.is_on.borrow_mut();
        *is_on = !*is_on;
        // The value is now toggled
        log::debug!("Toggle state is now: {}", *is_on);
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self {
            is_on: RefCell::new(false),
        }
    }
}

impl Display for Toggle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let state = if self.is_on() { "+" } else { "-" };
        write!(f, "{state}")
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__toggle_tests {
    use super::*;
    use crate::preludes::funky::ACE_SPADES;

    #[test]
    fn toggle_card() {
        let toggle_card = ToggleCard::new(ACE_SPADES);

        // Check initial state
        assert!(!toggle_card.is_selected());
        assert_eq!(toggle_card.to_string(), "-AS");

        // Toggle the state
        toggle_card.toggle();
        assert!(toggle_card.is_selected());
        assert_eq!(toggle_card.to_string(), "+AS");

        // Toggle again
        toggle_card.toggle();
        assert!(!toggle_card.is_selected());
        assert_eq!(toggle_card.to_string(), "-AS");
    }

    #[test]
    fn toggle() {
        let toggle = Toggle::default();
        assert!(!toggle.is_on());
        assert_eq!(toggle.to_string(), "-");

        toggle.toggle();
        assert!(toggle.is_on());
        assert_eq!(toggle.to_string(), "+");

        toggle.toggle();
        assert!(!toggle.is_on());
        assert_eq!(toggle.to_string(), "-");
    }
}
