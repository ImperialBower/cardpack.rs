use crate::basic::decks::cards::french::FLUENT_KEY_BASE_NAME_FRENCH;
use crate::basic::types::basic_card::BasicCard;
pub use crate::basic::types::card::Card;
pub use crate::basic::types::deck::Deck;
pub use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use std::collections::HashMap;
use std::hash::Hash;

pub trait DeckedBase {
    fn base_vec() -> Vec<BasicCard>;

    fn colors() -> HashMap<Pip, colored::Color>;

    fn deck_name() -> String;

    /// Use this to override the fluent keys iside the ftl files
    #[must_use]
    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }

    fn fluent_deck_key() -> String;

    /// And just like that we have a `Pile`.
    #[must_use]
    fn pile() -> Pile {
        Pile::from(Self::base_vec())
    }
}

pub trait Decked<DeckType>: DeckedBase
where
    DeckType: Copy + Default + Ord + DeckedBase + Hash,
{
    #[must_use]
    fn deck() -> Deck<DeckType> {
        Deck::<DeckType>::from(Self::deckvec())
    }

    /// This way of doing it from `CoPilot` is an interesting alternative to
    /// my old `pile_up` method:
    ///
    /// ```txt
    /// fn decks(n: usize) -> Pile<RankType, SuitType> {
    ///     Pile::<RankType, SuitType>::pile_up(n, <Self as Decked<RankType, SuitType>>::deck)
    /// }
    /// ```
    #[must_use]
    fn decks(count: usize) -> Deck<DeckType> {
        Deck::<DeckType>::from(Self::deckvec().repeat(count))
    }

    #[must_use]
    fn deckvec() -> Vec<Card<DeckType>> {
        let v = Card::<DeckType>::base_vec();
        v.iter().map(|card| Card::<DeckType>::from(*card)).collect()
    }

    // The [capitalize crate](https://crates.io/crates/capitalize) does handle this, but it's
    // funnier just doing it this way.
    //
    // When I was coding it there were three ways that it played out.
    //
    // ## 1. My way aka the lazy way
    //
    // Use the [capitalize crate](https://crates.io/crates/capitalize) crate and be done with it.
    //
    // ## 2. The `CoPilot` way
    //
    // ```
    // fn capitalize_first_letter(s: &str) -> String {
    //     let mut c = "foo".chars();
    //     match c.next() {
    //         None => String::new(),
    //         Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    //     }
    // }
    //
    // fn main() {
    //     let s = "hello";
    //     let capitalized = capitalize_first_letter(s);
    //     println!("{}", capitalized); // Output: "Hello"
    // }
    // ```
    // All that and it's a complete waste of time because it references the `Fluent` name
    // and not the `Deck` name. We ended up creating the method at the `DeckedBase` trait.
    // #[must_use]
    // fn deck_name() -> String {
    //     let binding = Self::fluent_deck_key();
    //     let mut letters = binding.chars();
    //     match letters.next() {
    //         None => String::new(),
    //         Some(first) => first.to_uppercase().collect::<String>() + letters.as_str(),
    //     }
    // }

    fn demo(verbose: bool) {
        let deck = Self::deck();
        let shuffled = deck.shuffled();
        let name = Self::deck_name();

        println!();
        println!("{name} Deck:          {}", deck.to_color_symbol_string());
        println!("{name} Deck Index:    {}", deck.index());
        println!(
            "{name} Deck Shuffled: {}",
            shuffled.to_color_symbol_string()
        );

        if verbose {
            println!();
            println!("Long in English and German:");

            for card in deck {
                let name = card.fluent_name_default();
                println!("  {name} ");
            }
        }
    }

    /// Warning on original version of the code.
    /// TODO: I need to understand slices better.
    ///
    /// ```txt
    /// warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    ///   --> src/rev6/traits.rs:22:31
    ///    |
    /// 22 |     fn into_cards(base_cards: &Vec<BaseCard>) -> Vec<Card<DeckType>> {
    ///    |                               ^^^^^^^^^^^^^^ help: change this to: `&[BaseCard]`
    /// ```
    #[must_use]
    fn into_cards(base_cards: &[BasicCard]) -> Vec<Card<DeckType>> {
        base_cards
            .iter()
            .map(|card| Card::<DeckType>::from(*card))
            .collect()
    }

    /// This is the urtest for a deck. If you can do this, you are golden.
    #[must_use]
    fn validate() -> bool {
        let deck = Self::deck();

        deck == deck.clone().shuffled().sort()
    }
}
