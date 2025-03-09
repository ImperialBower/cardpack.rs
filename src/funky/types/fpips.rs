use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum FPipType {
    #[default]
    Blank,
    Integer,
    OneDecimalPlace,
}

/// NOTE
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FIntPip {
    pub pip_type: FPipType,
    pub index: char,
    pub symbol: char,
    pub f: fn(usize) -> usize,
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__fpips_tests {
    use super::*;
}