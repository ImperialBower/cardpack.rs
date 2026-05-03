# cardpack.rs

[![Build and Test](https://github.com/ImperialBower/cardpack.rs/actions/workflows/CI.yaml/badge.svg)](https://github.com/ImperialBower/cardpack.rs/actions/workflows/CI.yaml)
[![codecov](https://codecov.io/gh/ImperialBower/cardpack.rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ImperialBower/cardpack.rs)
[![Crates.io Version](https://img.shields.io/crates/v/cardpack.svg)](https://crates.io/crates/cardpack)
[![Rustdocs](https://docs.rs/cardpack/badge.svg)](https://docs.rs/cardpack/)

Generic pack of cards library written in Rust. The goals of the library include:

* Various types of decks of cards.
* Internationalization support.
* Ability to create custom sorts for a specific pack of cards.

**UPDATE:** This is a complete rewrite of the library taking advantage of generics
in order to make the code cleaner, and easier to extend. 

## Setup

Build and run common tasks with [GNU make](https://www.gnu.org/software/make/):

```shell
make
```

Run `make help` to see all available targets.

## Usage

```rust
use cardpack::prelude::*;

fn main() {
  let mut pack = Standard52::deck();

  pack.shuffle();

  // Deal no-limit hold'em hands for two players:
  let small_blind = pack.draw(2).unwrap().sorted_by_rank();
  let big_blind = pack.draw(2).unwrap().sorted_by_rank();

  println!("small blind: {}", small_blind.to_string());
  println!("big blind:   {}", big_blind.to_string());

  let flop = pack.draw(3).unwrap();
  let turn = pack.draw(1).unwrap();
  let river = pack.draw(1).unwrap();

  println!();
  println!("flop : {}", flop.to_string());
  println!("turn : {}", turn.to_string());
  println!("river: {}", river.to_string());

  // Now, let's validate that the cards when collected back together are a valid Standard52
  // deck of cards.
  let reconstituted_pile =
          Pile::<Standard52>::pile_on(&*vec![pack, small_blind, big_blind, flop, turn, river]);
  assert!(Standard52::deck().same(&reconstituted_pile));
}
```

## Details

The goal of this library is to be able to support the creation of card
decks of various sizes and suits. Out of the box, the library supports:

* [French Deck](https://en.wikipedia.org/wiki/French_playing_cards)
  * [Pinochle](https://en.wikipedia.org/wiki/Pinochle#Deck)
  * [Spades](https://en.wikipedia.org/wiki/Spades_(card_game)#General_overview) with [Jokers](https://en.wikipedia.org/wiki/Joker_(playing_card))
  * [Standard 52](https://en.wikipedia.org/wiki/Standard_52-card_deck)
  * [Canasta](https://en.wikipedia.org/wiki/Canasta#Cards_and_deal)
    * [Hand and Foot](https://www.pagat.com/rummy/handfoot.html)
  * [Euchre](https://en.wikipedia.org/wiki/Euchre)
* [Short Deck](https://en.wikipedia.org/wiki/Six-plus_hold_'em)
* [Skat](https://en.wikipedia.org/wiki/Skat_(card_game)#Deck)
* [Tarot](https://en.wikipedia.org/wiki/Tarot#Tarot_gaming_decks) with [Major](https://en.wikipedia.org/wiki/Major_Arcana) and [Minor](https://en.wikipedia.org/wiki/Minor_Arcana) Arcana

The project takes advantage of [Project Fluent](https://www.projectfluent.org/)'s
[Rust](https://github.com/projectfluent/fluent-rs) support to offer
internationalization. Current languages supported are
[English](src/localization/locales/en-US/french.ftl),
[German](src/localization/locales/de/french.ftl),
[French](src/localization/locales/fr/french.ftl),
[Latin](src/localization/locales/la/french.ftl), and
[Klingon](src/localization/locales/tlh/french.ftl).

## Cargo features

Every dependency-bearing capability is gated behind a Cargo feature so
consumers can trim what they don't need:

| Feature           | Default | Pulls in           | What turns off without it                                     |
|-------------------|---------|--------------------|---------------------------------------------------------------|
| `i18n`            | yes     | `fluent-templates` | `FluentName`, `Named`, `Card::fluent_name*`, `localization`   |
| `colored-display` | yes     | `colored`          | `Color`, `Colorize`, `Card::color*`, `Pile::to_color_*`       |
| `yaml`            | yes     | `serde_norway`     | `BasicCard::cards_from_yaml_*`, the `Razz` deck (YAML-loaded) |
| `serde`           | yes     | `serde`            | `Serialize`/`Deserialize` derives on `Pip`/`Card`/`Pile` etc. |

Default-features builds behave exactly like prior versions. To trim:

```toml
cardpack = { version = "0.6", default-features = false, features = ["serde"] }
```

`yaml` implies `serde` (it deserializes into the serde-derived structs).

## WebAssembly

cardpack compiles cleanly to `wasm32-unknown-unknown` (browser WASM)
with every feature combination. See [`docs/wasm.md`](docs/wasm.md) for
the consumer-side `getrandom` backend setup, recommended feature
combos, and runtime gotchas. A working example lives at
[`examples/wasm.rs`](examples/wasm.rs).

## Responsibilities

* Represent a specific type of card deck.
* Validate that a collection of cards is valid for that type of deck.
* Create a textual representation of a deck that can be serialized and deserialized.
* Shuffle a deck

## Examples

The library has several examples programs, including `demo` which shows you the different decks
available.

For the traditional 54 card French Deck with Jokers:

```shell
❯ cargo run --example demo -- --french -v
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/examples/demo --french -v`

French Deck:          B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣
French Deck Index:    BJ LJ AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C
French Deck Shuffled: K♣ 7♦ 8♣ Q♥ 6♠ J♦ 4♦ J♥ K♠ 9♥ 6♥ T♥ 2♦ 3♦ 3♣ J♣ 3♥ Q♣ 5♥ Q♦ 3♠ T♣ 7♥ 4♥ K♦ 5♦ 2♠ 6♦ T♠ 8♥ T♦ 7♠ 8♠ 2♣ Q♠ 7♣ A♣ 5♠ A♥ 9♣ 2♥ 9♦ 9♠ 4♠ K♥ 8♦ 5♣ A♦ L🃟 B🃟 A♠ 6♣ 4♣ J♠

  English                  | German                   | French                   | Latin                    | Klingon
  ------------------------ | ------------------------ | ------------------------ | ------------------------ | ------------------------
  Joker Full-Color         | Joker Großer             | Joker Grand              | Joker Magnus             | Joker qoH'a'
  Joker One-Color          | Joker Kleiner            | Joker Petit              | Joker Parvus             | Joker qoHHom
  Ace of Spades            | Ass Spaten               | As de Pique              | As Spathae               | wa'DIch yan
  King of Spades           | König Spaten             | Roi de Pique             | Rex Spathae              | ta' yan
  Queen of Spades          | Dame Spaten              | Dame de Pique            | Regina Spathae           | ta'be' yan
  Jack of Spades           | Bube Spaten              | Valet de Pique           | Famulus Spathae          | toy'wI' yan
  Ten of Spades            | Zhen Spaten              | Dix de Pique             | Decem Spathae            | wa'maH yan
  ...
```

Display a hand of [Bridge](https://en.wikipedia.org/wiki/Contract_bridge):

```shell
❯ cargo run --example bridge                                                          
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/examples/bridge`
First, let's deal out a random bridge hand.

Here it is in Portable Bridge Notation:
    W:KJT.JT63.K8.QJT9 A75.KQ9874.65.AK Q6432.5.AJ74.853 98.A2.QT932.7642

How does it look as a traditional compass?
               NORTH
            ♠ A 7 5
            ♥ K Q 9 8 7 4
            ♦ 6 5
            ♣ A K

       WEST              EAST
    ♠ K J T           ♠ Q 6 4 3 2
    ♥ J T 6 3         ♥ 5
    ♦ K 8             ♦ A J 7 4
    ♣ Q J T 9         ♣ 8 5 3

                SOUTH
             ♠ 9 8
             ♥ A 2
             ♦ Q T 9 3 2
             ♣ 7 6 4 2

Now, let's take a PBN Deal String and convert it into a bridge hand.
Here's the original' Portable Bridge Notation:
    S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982

As a bridge compass:

                NORTH
             ♠ A J T 8 5
             ♥ J 7 6
             ♦ K J
             ♣ A 6 5

       WEST              EAST
    ♠ 9 7             ♠ K 6 3
    ♥ A T 9 3         ♥ K 8 4
    ♦ 6 5 2           ♦ 8 7
    ♣ T 7 4 3         ♣ K J 9 8 2

               SOUTH
            ♠ Q 4 2
            ♥ Q 5 2
            ♦ A Q T 9 4 3
            ♣ Q

```

Other decks in the demo program are `canasta`, `euchre`, `short`, `pinochle`, `skat`, `spades`,
`standard`, and `tarot`.

Other examples are:

- `cargo run --example handandfoot` - Shows how to support more than one decks like in the game [Hand and Foot](https://www.wikihow.com/Play-Hand-and-Foot).
- `cargo run --example poker` - A random heads up [no-limit Poker](https://en.wikipedia.org/wiki/Texas_hold_%27em) deal.

## References

* [Card games in Germany](https://www.pagat.com/national/germany.html)
* [Playing cards in Unicode](https://en.wikipedia.org/wiki/Playing_cards_in_Unicode)

### Other Deck of Cards Libraries

* [ascclemens/cards](https://github.com/ascclemens/cards)
* [locka99/deckofcards-rs](https://github.com/locka99/deckofcards-rs)
* [vsupalov/cards-rs](https://github.com/vsupalov/cards-rs)
* [droundy/bridge-cards](https://github.com/droundy/bridge-cards)
* Tarot Libraries
  * [lawreka/ascii-tarot](https://github.com/lawreka/ascii-tarot)
  * [pietdaniel/tarot](https://github.com/pietdaniel/tarot)
  * [jeremytarling/ruby-tarot](https://github.com/jeremytarling/ruby-tarot)

## Dependencies

* [Colored](https://github.com/colored-rs/colored)
* [Fluent Templates](https://github.com/XAMPPRocky/fluent-templates)
  * [Project Fluent](https://www.projectfluent.org/)
* [itertools](https://github.com/rust-itertools/itertools)
* [log](https://github.com/rust-lang/log)
* [rand](https://github.com/rust-random/rand)
* [serde](https://github.com/serde-rs/serde)
  * [serde_norway](https://crates.io/crates/serde_norway)
* [thiserror](https://github.com/dtolnay/thiserror)

## Dev Dependencies

* [term-table](https://github.com/RyanBluth/term-table-rs)
* [rstest](https://github.com/la10736/rstest) - Fixture-based test framework for Rust

## TODO

* [Hanafuda](https://en.wikipedia.org/wiki/Hanafuda)
  * [고스톱 (Go-Stop)](https://en.wikipedia.org/wiki/Go-Stop)
    * [Go-Stop - The Cards](https://www.sloperama.com/gostop/cards.html)
    * [nbry/go-stop-rust](https://github.com/nbry/go-stop-rust)
  * [Sakura](https://en.wikipedia.org/wiki/Sakura_(card_game))
* [Cinch](https://en.wikipedia.org/wiki/Cinch_(card_game))
* [Zwickern](https://en.wikipedia.org/wiki/Zwickern)
