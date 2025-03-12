use serde::{Deserialize, Serialize};

/// DIARY: I created this out of habit, but I am feeling that it is needless. Removing it for now
/// from the `FIntPip` struct.
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
    pub index: char,
    pub symbol: char,
    pub f: fn(usize) -> usize,
}

impl FIntPip {
    /// DIARY: This is where having a tool like `CoPilot` is helpful. This is how I would set it up
    /// and instead of typing it, it just generates the suggestion for me and with a simple press
    /// of a button, I have thc code. They problem is, if you don't know what you want, you won't
    /// be able to judge the quality of the suggestion. AI isn't there to replace you, it's there
    /// to assist you. The fact that it is being weaponized with hype is a serious problem.
    pub fn new(index: char, symbol: char, f: fn(usize) -> usize) -> Self {
        Self {
            // pip_type, Not seeing the need for this right now.
            index,
            symbol,
            f,
        }
    }

    /// DIARY: There's something really nice about having a vision for something and feeling like
    /// you are finally getting it, and then you create the code, and it just works. One of the
    /// hardest things about working on teams for places that are on very tight deadlines is that
    /// you don't really have the time for aesthetic beauty, and giving your mind a chance to
    /// reflect on the system.
    ///
    /// DIARY: The problem with the way I like to name functions, aka silly, they usually end up
    /// being called something else.
    ///
    /// **STORY TIME:** When I Director of Architecture for the startup Comedy World (oooooooo....
    /// ahhhhhhh.... marvel at my magnificent achievements... (aka Fucked Company Magazine's 2nd
    /// worst "dot bomb" of all time)) I had this funny name of starting functions that changed
    /// system state with the phrase "goGoGadget", such as `goGoGadgetPublishArticle()`. This is a
    /// reference to the cartoon Inspector Gadget, of which I am a big fan. I really
    /// feel sorry for Frank, the only guy would could figure out my home grown publishing system.
    /// In hindsight, RJ was right in giving me shit for shit like that. You want people like RJ
    /// around you... people who say it to your face. I treasure people like that. The fact that
    /// they are so rare in business shows you how fucked it truly is.
    #[must_use]
    pub fn f_it(&self, x: usize) -> usize {
        (self.f)(x)
    }
}

impl Default for FIntPip {
    /// DIARY: But right after that, here's a suggestion that while it is useful, isn't correct.
    /// In `Rust`, we implement the Default trait for this. This is a great example of how LLMs
    /// bubble up shit. Just because more people do it one way, doesn't mean that it's the best way.
    /// If AI companies have their way, the
    /// [Berenstain Bears](https://en.wikipedia.org/wiki/Berenstain_Bears#Name_confusion) will
    /// always be known as the Berenstein Bears.
    ///
    /// When I started out writing `Rust`, I had no idea about traits. `Rust` was one of the first
    /// languages I ever coded in where I didn't start by reading the book. It's an OK boomer habit
    /// of mine to read the book written by the creator of a programming language if I am insterested
    /// in using it. You have no excuse not to with `Rust`. [It's free.](https://doc.rust-lang.org/stable/book/)
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
            index: '_',
            symbol: '_',
            f: |x| x,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__fpips_tests {
    use super::*;

    fn plus_pip() -> FIntPip {
        FIntPip::new('+', '+', |x| x + 1)
    }

    fn double_pip() -> FIntPip {
        FIntPip::new('+', '+', |x| x * 2)
    }

    #[test]
    fn f_it() {
        assert_eq!(plus_pip().f_it(1), 2);
        assert_eq!(plus_pip().f_it(2), 3);
        assert_eq!(double_pip().f_it(1), 2);
        assert_eq!(double_pip().f_it(2), 4);
    }

    #[test]
    fn default() {
        let f_int_pip = FIntPip::default();
        assert_eq!(f_int_pip.f_it(5), 5);
    }
}