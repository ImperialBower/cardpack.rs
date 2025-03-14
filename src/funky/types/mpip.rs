use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum MPipType {
    #[default]
    Blank,
    Chips(u8),
    Glass(u8, u8),
    Gold(u8),
    Lucky(u8, u8),
    MultPlus(u8),
    MultTimes(u8),
    MultTimes1Dot(u8),
    Stone(u8),
    Wild,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MPip {
    pub pip_type: MPipType,
    pub index: char,
}

impl MPip {
    pub const BLANK: Self = Self {
        pip_type: MPipType::Blank,
        index: '_',
    };
}

impl Default for MPip {
    fn default() -> Self {
        Self::BLANK
    }
}

pub mod enhancement {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__mpips_tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(MPipType::default(), MPipType::Blank);
    }
}
