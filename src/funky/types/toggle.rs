use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Toggle {
    is_on: RefCell<bool>,
}

impl Toggle {
    pub fn is_on(&self) -> bool {
        // Borrow the boolean value inside the RefCell
        // This will return a boolean indicating if the toggle is on or off
        let is_on = self.is_on.borrow();
        *is_on // Dereference to get the actual boolean value
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

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__toggle_tests {
    use super::*;

    #[test]
    fn toggle() {
        let toggle = Toggle::default();

        toggle.toggle();
        assert!(toggle.is_on());

        toggle.toggle();
        assert!(!toggle.is_on());
    }

    #[test]
    fn default() {
        assert!(!Toggle::default().is_on());
    }
}