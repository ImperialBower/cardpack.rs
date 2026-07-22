# Decks

* [Shipped deck catalog](deck-catalog.md) - The 12 deck kinds the crate ships, their sizes, and their quirks; DeckKind is the runtime registry over all of them.
* [Creating a custom deck](extending-decks.md) - Consumers author new deck vocabularies with the same machinery as shipped decks — implement DeckedBase on a marker struct, then Decked for free methods.
