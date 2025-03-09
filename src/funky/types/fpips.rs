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

impl FIntPip {
    /// DIARY: This is where having a tool like CoPilot is helpful. This is how I would set it up
    /// and instead of typing it, it just generates the suggestion for me and with a simple press
    /// of a button, I have thc code. They problem is, if you don't know what you want, you won't
    /// be able to judge the quality of the suggestion. AI isn't there to replace you, it's there
    /// to assist you. The fact that it is being weaponized with hype is a serious problem.
    pub fn new(pip_type: FPipType, index: char, symbol: char, f: fn(usize) -> usize) -> Self {
        Self {
            pip_type,
            index,
            symbol,
            f,
        }
    }
}

impl Default for FIntPip {
    /// DIARY: But right after that, here's a suggestion that while it is useful, isn't correct.
    /// In `Rust`, we implement the Default trait for this.
    ///
    /// ```txt
    /// impl FIntPip {
    ///     fn new_blank() -> Self {
    ///         Self {
    ///             pip_type: FPipType::Blank,
    ///             index: '0',
    ///             symbol: ' ',
    ///             f: |x| x,
    ///         }
    ///     }
    /// }
    /// ```
    fn default() -> Self {
        Self {
            pip_type: FPipType::Blank,
            index: '0',
            symbol: ' ',
            f: |x| x,
        }
    }
}


#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__fpips_tests {
    use super::*;
}