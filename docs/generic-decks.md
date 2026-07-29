# Generic Decks: the phantom-type deck pattern

How cardpack uses Rust generics so that one set of card, pile, parsing, and
serialization machinery serves fourteen shipped decks — and any deck a consumer
invents — with zero runtime cost. This document explains how the pattern works,
why each piece is shaped the way it is, and how to set it up cleanly in a new
library.

The shipped decks range from a 4-card teaching deck (`Tiny`) to the 120-card
`Dashavatara` Ganjifa deck, through Pinochle (duplicated ranks), Skat (a
completely different suit/rank vocabulary), and Tarot (mixed Major/Minor
Arcana). None of them required touching `Card`, `Pile`, parsing, shuffling, or
YAML serialization.

## The problem the pattern solves

A playing-card library has two forces pulling in opposite directions:

1. **The machinery is universal.** Drawing, shuffling, sorting, parsing
   `"A♠ K♠"`, combinatorics, serialization — none of this cares which deck it
   operates on.
2. **The vocabulary is not.** A Skat deck has no Queen of Hearts. A Pinochle
   deck has *two* of each card. Mixing a Tarot card into a French pile is a
   bug, and Ace is high in Standard 52 but low in Razz.

Dynamic solutions (a `deck_id` field checked at runtime, trait objects,
registries of `Box<dyn Deck>`) push vocabulary errors to runtime and put a
tax on every operation. The phantom-type pattern instead makes the deck a
**compile-time brand**: the machinery is written once, generically, and the
type system guarantees cards from different vocabularies never mix.

```text
                 type-driven (identity)                 data-driven (behavior)
  ┌────────────────────────────────────────┐   ┌──────────────────────────────────┐
  │  French   Skat   Pinochle   Tiny  ...  │   │  Pip { weight, index, symbol, …} │
  │  (empty marker structs, zero-sized)    │   │  BasicCard { suit: Pip, rank: Pip}│
  └───────────────────┬────────────────────┘   └────────────────┬─────────────────┘
                      │  impl DeckedBase                        │  plain data,
                      ▼  (the vocabulary contract)              ▼  no generics
  ┌─────────────────────────────────────────────────────────────────────────────┐
  │   Card<DeckType>  =  BasicCard + PhantomData<DeckType>       (the brand)    │
  │   Pile<DeckType>  =  Vec<Card<DeckType>>                     (the machine)  │
  └─────────────────────────────────────────────────────────────────────────────┘
                      ▲
                      │  provided methods & blanket impls
  ┌───────────────────┴─────────────────────────────────────────────────────────┐
  │   Decked<T> (deck(), decks(n), validate())   Ranged (combos)   YamlDecked   │
  └─────────────────────────────────────────────────────────────────────────────┘
```

Four layers, two axes. The left axis (types) decides *which cards exist and
what they're called*. The right axis (data) decides *how cards behave* —
ordering, display, value. Keeping those axes separate is what makes the
pattern portable.

## Layer 1: plain data, no generics

The bottom layer is deliberately generic-free. `Pip`
([`src/basic/types/pips.rs`](../src/basic/types/pips.rs)) is the atomic facet
of a card:

```rust
pub struct Pip {
    pub weight: usize,   // sorting/precedence — this is where "Ace is high" lives
    pub pip_type: PipType, // Suit | Rank | Joker | Special | Blank
    pub index: char,     // machine identity: 'A', 'S'
    pub symbol: char,    // display: 'A', '♠'
    pub value: usize,    // game value when it differs from weight
}
```

`BasicCard` ([`src/basic/types/basic_card.rs`](../src/basic/types/basic_card.rs))
is just two `Pip`s:

```rust
pub struct BasicCard {
    pub suit: Pip,
    pub rank: Pip,
}
```

Two design decisions here are load-bearing:

**Behavior lives in the data, not in code.** There is no `impl Ord` per deck,
no comparator functions, no `match` on rank names anywhere in the sorting
path. `BasicCard`'s `Ord` simply compares the embedded weights (reversed, so
that a plain `sort()` yields the canonical high-to-low, suit-major deck
order). The consequence: an Ace-low deck is *not* a sorting option — it is a
different deck whose Ace `Pip` carries a low weight. The shipped `Razz` deck
is exactly this. If you find yourself wanting a `SortOrder` parameter, the
pattern's answer is "no — make a new vocabulary with different weights."

**The data layer is the interchange format.** Because `BasicCard` has no type
parameter and derives `Serialize`/`Deserialize` (under the `serde` feature),
everything that crosses a boundary — YAML files, the deck registry, cross-deck
analysis — speaks `BasicCard`. Generics stop at the API surface; data at rest
is plain.

## Layer 2: the brand — `Card<DeckType>`

`Card` ([`src/basic/types/card.rs`](../src/basic/types/card.rs)) wraps a
`BasicCard` and adds *nothing at runtime*:

```rust
pub struct Card<DeckType>
where
    DeckType: DeckedBase,
{
    pub base_card: BasicCard,
    pub deck: PhantomData<DeckType>,
}
```

`PhantomData<DeckType>` is a zero-sized marker: `Card<French>` and
`Card<Skat>` have identical memory layouts, but the compiler treats them as
unrelated types. That single line buys:

- **Compile-time vocabulary safety.** `Pile<French>` is
  `Vec<Card<French>>`; pushing a `Card<Skat>` into it is a type error. No
  runtime check, no error variant, no test needed.
- **Deck-aware behavior with no stored state.** `card.is_valid()` needs to
  know its deck's card list. It reaches through the *type*, not through a
  field: `<DeckType as DeckedBase>::basic_pile().contains(&self.base_card)`.
- **Deck-scoped parsing.** `FromStr` is implemented against the brand, so
  `Card::<French>::from_str("QH")` succeeds while
  `Card::<Skat>::from_str("QH")` fails — same string, different vocabulary,
  and the failure is data-driven (the parser searches `DeckType::base_vec()`)
  rather than hand-coded per deck.

One subtlety worth copying: `Card` implements `From<BasicCard>` without
validating. You *can* build a `Card::<French>` from a Skat card; `is_valid()`
exists to check. This keeps the data layer fluid (cross-deck experimentation
stays easy) while the branded layer makes intent explicit. The library's
`validate()` gate (below) is where correctness is enforced.

## Layer 3: the vocabulary contract — `DeckedBase`

`DeckedBase` ([`src/basic/types/traits.rs`](../src/basic/types/traits.rs)) is
the single point where a type becomes a deck:

```rust
pub trait DeckedBase {
    fn base_vec() -> Vec<BasicCard>;          // the canonical card list
    fn deck_name() -> String;                 // identity for display & YAML headers
    fn fluent_deck_key() -> String;           // localization namespace
    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color>;       // presentation hints

    // provided:
    fn basic_pile() -> BasicPile { BasicPile::from(Self::base_vec()) }
    fn fluent_name_base() -> String { /* French default */ }
}
```

Note what's *absent*: every method is an associated function — no `&self`
anywhere. Deck types are **empty structs that are never meaningfully
instantiated**. `French`, `Skat`, `Tiny` are namespaces and brands, not
values. All deck knowledge is static, which is what lets `Card<T>` and
`Pile<T>` summon it from nothing but their type parameter.

The implementing struct is one line plus a card list
([`src/basic/decks/tiny.rs`](../src/basic/decks/tiny.rs) is the in-tree
minimal example):

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tiny {}

impl Tiny {
    pub const DECK_SIZE: usize = 4;
    pub const DECK: [BasicCard; Self::DECK_SIZE] = [
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
    ];
}

impl DeckedBase for Tiny {
    fn base_vec() -> Vec<BasicCard> { Self::DECK.to_vec() }
    fn deck_name() -> String { "Tiny".to_string() }
    fn fluent_deck_key() -> String { FLUENT_KEY_BASE_NAME_FRENCH.to_string() }
    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> { Standard52::colors() }
}
```

Decks compose by *reusing constants from the data layer* — `Tiny` borrows
French cards, `Spades` is Standard 52 minus two cards, `Pinochle` repeats
cards. Because the vocabulary is a plain `Vec<BasicCard>`, subsetting,
extending, duplicating, and re-weighting are ordinary vector operations, not
type-level gymnastics.

### The derive line is part of the contract

That long derive on the marker struct is not decoration. The generic
machinery's bounds are `DeckedBase + Default + Ord + Copy + Hash`
(see `Pile`'s declaration), and Rust's `derive` on `Card<T>`/`Pile<T>`
generates impls conditional on `T` having the same traits. An empty struct
derives all of them trivially — but omit one and the compile errors surface
far from the marker struct, in whatever generic code first needs the bound.
When you port this pattern, document the required derive list right next to
the trait, exactly because the compiler won't point at the right place.

## Layer 4: derived behavior — `Decked`, blanket impls, and one-line opt-in

`Decked<DeckType>` supplies everything a deck gets for free, entirely as
provided methods:

```rust
pub trait Decked<DeckType>: DeckedBase
where
    DeckType: Copy + Default + Ord + DeckedBase + Hash,
{
    fn deck() -> Pile<DeckType> { ... }        // the full canonical pile
    fn decks(count: usize) -> Pile<DeckType> { ... } // multi-deck games
    fn deckvec() -> Vec<Card<DeckType>> { ... }
    fn into_cards(base_cards: &[BasicCard]) -> Vec<Card<DeckType>> { ... }
    fn validate() -> bool { ... }
}
```

A deck opts in with a single empty impl:

```rust
impl Decked<Self> for Tiny {}
```

Why does `Decked` have a type parameter instead of just using `Self`?
Because **two different types implement it for the same deck**: the marker
struct (`impl Decked<Tiny> for Tiny`) and the pile
(`impl Decked<DeckType> for Pile<DeckType>` in
[`src/basic/types/pile.rs`](../src/basic/types/pile.rs)). That's what makes
both spellings work, and the doctests pin their equivalence:

```rust
let a: Pile<Pinochle> = Pinochle::deck();  // deck-first
let b = Pile::<Pinochle>::deck();          // pile-first
assert_eq!(a, b);
```

### Pass-through impls keep the layers connected

`Card<T>` and `Pile<T>` both implement `DeckedBase` themselves by delegating
every method to `T`. This is the quiet trick that makes the whole stack
composable: generic code can ask *any* layer for the vocabulary
(`Card::<French>::base_vec()`, `Pile::<French>::deck_name()`) without caring
whether it holds a card, a pile, or nothing at all. When you set the pattern
up elsewhere, write the delegation impls immediately — they're boilerplate,
but their absence forces downstream code to thread an extra type parameter
around.

### Blanket impls: implement one trait, get features for free

Cross-cutting features hang off `DeckedBase` via blanket impls, so a consumer
deck gets them with **zero** additional code:

```rust
#[cfg(feature = "yaml")]
impl<T: DeckedBase> YamlDecked for T {}
```

`YamlDecked` gives every deck `to_yaml()`, `deck_from_yaml()`, and
`validate_yaml()`; `Ranged` (combinatorics, poker-hand analysis) is
implemented for `Pile<T>` wholesale. There is deliberately **no override
hook** on the YAML path: one shared format is what keeps golden fixtures and
the runtime registry meaningful. When porting the pattern, resist making
blanket-provided behavior customizable until a real deck needs it — every
override point you expose is a format fork you must support.

This is the pattern's economic core: the *n*-th deck costs a card list and
two one-liners, while every feature added to the trait stack multiplies
across all decks at once — shipped and consumer-authored alike. There is no
registration step, no macro to invoke, no list to append to (the optional
runtime registry below is the one deliberate exception).

## The escape hatch: dropping the brand

The brand is for *construction and correctness*, not for analysis. Once you
hold a legitimate hand of cards, deck identity often stops mattering —
combinatorics, rank counting, and hand evaluation work on raw pips. The
library models this explicitly with `BasicPile` (a plain
`Vec<BasicCard>` wrapper) and the `Ranged` trait's single required method:

```rust
fn my_basic_pile(&self) -> BasicPile;
```

Everything else in `Ranged` — `combos(k)`, `map_by_rank()`,
`is_connector()` — is a provided method over that unbranded view. The
comment on `my_basic_pile` states the philosophy directly: the generic
`Pile` is "very useful for getting us to where we want to be," but once
there, the analysis "can rid ourselves of its confinements and focus on
data."

This two-way door — brand on the way in, plain data for analysis — is worth
designing deliberately in any port. A pattern that *only* brands becomes a
prison: every analysis function grows a type parameter it never uses.

## The runtime companion: a registry enum

Generics resolve at compile time, but some callers only know the deck at
runtime — a CLI picking a deck by name, a YAML file declaring what it
contains. The library's answer is `DeckKind`
([`src/basic/decks/registry.rs`](../src/basic/decks/registry.rs)): a
`#[non_exhaustive]` enum with one variant per *shipped* deck, whose methods
match on the variant and call into the generic machinery.

The division of labor:

| Concern | Generic path (`Pile<T>`) | Registry path (`DeckKind`) |
|---|---|---|
| Deck known at | compile time | runtime |
| Consumer decks | yes — implement `DeckedBase` | no — shipped decks only |
| Cost per operation | zero (monomorphized) | one `match` |

The registry is a thin façade over the generic core, never a second
implementation. Keep that direction of dependency when porting: the enum
calls the generics; the generics never know the enum exists.

## Setting the pattern up in a new library

The order matters — each layer is testable before the next exists.

1. **Design the plain data layer first, generics-free.** Decide what a card
   *is* (for cardpack: two pips), and put every behavior-determining fact —
   sort precedence, display glyphs, game values — *into the data*. Test
   ordering and display with plain vectors before any generic code exists.
   If two decks would need the same cards to behave differently, that's two
   sets of data, not a code path.
2. **Define the vocabulary trait.** One trait, all associated functions
   (no `&self`), with the canonical card list (`base_vec()`) as its heart
   plus whatever identity your domain needs (name, localization key).
   Keep it minimal — every method here is a promise all decks must keep.
3. **Add the brand.** A generic wrapper holding `data + PhantomData<T>`,
   `T` bounded by the vocabulary trait. Implement `From<Data>` (unvalidated),
   `FromStr` (searches `T::base_vec()`), `Display` (delegates to data), and
   the **pass-through impl** of the vocabulary trait itself.
4. **Add the collection.** A newtype over `Vec<Brand<T>>` with the
   machinery (draw, shuffle, sort, parse). Give it the same pass-through
   impl. Accept that the bound list (`Default + Ord + Copy + Hash + ...`)
   repeats on every `impl` block — it's the pattern's one genuine
   ergonomic tax (Rust has no way to alias bound sets today; the author's
   in-code diary in `pile.rs` is honest about how it feels).
5. **Add the convenience trait with provided methods only**, parameterized
   like `Decked<T>` so both the marker and the collection can implement it.
   A new deck's opt-in should be one empty line.
6. **Blanket-implement cross-cutting features** (serialization, analysis)
   off the vocabulary trait, so consumer decks can never forget to wire
   them.
7. **Ship `validate()` and make it every deck's first test.** cardpack's is
   a strong template — it composes three properties into one boolean:
   *round-trip* (`Pile::from_str(deck.to_string())` reproduces the deck),
   *canonical order* (a seeded shuffle followed by `sorted()` restores it),
   and implicitly *display uniqueness* (round-trip fails if two cards print
   alike). Every new vocabulary — shipped or consumer — starts with
   `assert!(MyDeck::validate())`.
8. **Keep a `Tiny` in-tree.** A 4-card deck is living documentation: small
   enough to assert full deck strings in doctests, real enough to exercise
   every layer. cardpack's `Tiny` doubles as the custom-deck tutorial in
   the crate docs.
9. **Add the registry enum last, if at all** — only when a runtime-choice
   use case actually appears, and only as a façade.

## Pitfalls and sharp edges

Learned in this codebase, likely to recur in any port:

- **Calling trait statics generically needs qualified syntax.** Inside
  `Card<DeckType>`, `DeckedBase::basic_pile()` won't compile (E0790 —
  ambiguous impl); you must write
  `<DeckType as DeckedBase>::basic_pile()`. The compiler's suggestion is
  good, but the first encounter is disorienting — the in-code comment in
  `card.rs` preserves the full error as a field guide.
- **Bound drift.** The bound set (`DeckedBase + Default + Ord + Copy +
  Hash`) appears on the struct, on every `impl` block, and inside
  `Decked`'s `where` clause. When you add a bound, you add it everywhere;
  keep the list short and stable. Resist adding bounds "while you're
  there" — each one propagates to every marker struct's derive line.
- **`Default` needs a "blank" story.** Deriving `Default` on the brand
  means `Card::<T>::default()` exists. cardpack leans in: the default
  `BasicCard` is an explicit *blank card* (`__`), `FromStr` accepts it, and
  `is_blank()` names it — useful for representing gaps in a spread. If you
  derive `Default`, decide what the default card *means*; don't leave it as
  an accidental zero-value.
- **Unvalidated construction is a feature, but gate it.** `From<BasicCard>`
  deliberately skips validation. The compensating controls are `is_valid()`
  on the card and `validate()`/`validate_yaml()` at the deck level. Copy
  the *pair* — permissive construction alone is a foot-gun; validation
  alone makes experimentation miserable.
- **Serde and `PhantomData` coexist fine.** The brand derives
  `Serialize`/`Deserialize` under a feature flag; `PhantomData` serializes
  to nothing. The wire format stays the plain data layer's — another reason
  to keep that layer generic-free.
- **Monomorphization is the cost you accept.** Every deck type stamps out
  its own `Pile<T>` code. With fourteen decks this is negligible, but a
  library expecting hundreds of vocabularies, or wanting piles of *mixed*
  vocabularies, needs a different pattern (enum-based cards, or trait
  objects). The phantom brand is the right tool when vocabularies are
  known per-context and mixing is a bug — which is exactly the
  playing-card situation.

## Where to look in the source

| Concept | File |
|---|---|
| `Pip`, `PipType` | [`src/basic/types/pips.rs`](../src/basic/types/pips.rs) |
| `BasicCard` (+ reversed `Ord`) | [`src/basic/types/basic_card.rs`](../src/basic/types/basic_card.rs) |
| `Card<DeckType>`, pass-through impl, `FromStr` | [`src/basic/types/card.rs`](../src/basic/types/card.rs) |
| `Pile<DeckType>`, bounds, `Decked` for piles | [`src/basic/types/pile.rs`](../src/basic/types/pile.rs) |
| `DeckedBase`, `Decked`, `YamlDecked` blanket, `Ranged` | [`src/basic/types/traits.rs`](../src/basic/types/traits.rs) |
| Minimal deck (`Tiny`) | [`src/basic/decks/tiny.rs`](../src/basic/decks/tiny.rs) |
| Data-driven Ace-low deck (`Razz`) | [`src/basic/decks/razz.rs`](../src/basic/decks/razz.rs) |
| Runtime registry (`DeckKind`) | [`src/basic/decks/registry.rs`](../src/basic/decks/registry.rs) |
| Custom-deck walkthrough | `src/lib.rs` module docs, "Custom Deck example" |

Related knowledge-bundle concepts: the distilled
[card model](../.okf/architecture/card-model.md) and the
[custom-deck playbook](../.okf/decks/extending-decks.md).
