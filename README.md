# spielkarten.rs
Generic Deck of Cards Library in Rust

## Demo

```
$> cargo run
spielkarten.rs demo

French Deck:
   Short With Symbols:           A♠ K♠ Q♠ J♠ 10♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ 10♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ 10♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ 10♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣ 
   Short With Symbols in German: A♠ K♠ D♠ B♠ 10♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ D♥ B♥ 10♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ D♦ B♦ 10♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ D♣ B♣ 10♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣ 
   Short With Letters:           AS KS QS JS 10S 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH 10H 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD 10D 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC 10C 9C 8C 7C 6C 5C 4C 3C 2C 
   Short With Letters in German: AS KS DS BS 10S 9S 8S 7S 6S 5S 4S 3S 2S AH KH DH BH 10H 9H 8H 7H 6H 5H 4H 3H 2H AD KD DD BD 10D 9D 8D 7D 6D 5D 4D 3D 2D AK KK DK BK 10K 9K 8K 7K 6K 5K 4K 3K 2K 
   Shuffle Deck:                 9♥ Q♠ 6♠ 6♥ 7♣ 5♦ A♦ 7♥ 10♦ Q♥ 10♥ J♦ 10♠ K♥ 7♦ 10♣ 4♠ 8♦ 8♣ 3♠ J♠ 9♠ 3♣ K♠ 6♣ 8♥ 4♣ 9♦ 6♦ Q♣ 9♣ Q♦ A♣ 4♦ 8♠ 5♣ 2♥ 5♠ A♥ 2♠ 2♣ J♣ 4♥ J♥ 3♥ 3♦ 5♥ K♣ A♠ 7♠ K♦ 2♦ 
   Long in English and German:
      Ace of Spades 
      Ass von Spaten 
      King of Spades 
      König von Spaten 
      Queen of Spades 
      Dame von Spaten 
...
Pinochle Deck:
   Short With Symbols:           A♠ A♠ 10♠ 10♠ K♠ K♠ Q♠ Q♠ J♠ J♠ 9♠ 9♠ A♥ A♥ 10♥ 10♥ K♥ K♥ Q♥ Q♥ J♥ J♥ 9♥ 9♥ A♦ A♦ 10♦ 10♦ K♦ K♦ Q♦ Q♦ J♦ J♦ 9♦ 9♦ A♣ A♣ 10♣ 10♣ K♣ K♣ Q♣ Q♣ J♣ J♣ 9♣ 9♣ 
   Short With Symbols in German: A♠ A♠ 10♠ 10♠ K♠ K♠ D♠ D♠ B♠ B♠ 9♠ 9♠ A♥ A♥ 10♥ 10♥ K♥ K♥ D♥ D♥ B♥ B♥ 9♥ 9♥ A♦ A♦ 10♦ 10♦ K♦ K♦ D♦ D♦ B♦ B♦ 9♦ 9♦ A♣ A♣ 10♣ 10♣ K♣ K♣ D♣ D♣ B♣ B♣ 9♣ 9♣ 
   Short With Letters:           AS AS 10S 10S KS KS QS QS JS JS 9S 9S AH AH 10H 10H KH KH QH QH JH JH 9H 9H AD AD 10D 10D KD KD QD QD JD JD 9D 9D AC AC 10C 10C KC KC QC QC JC JC 9C 9C 
   Short With Letters in German: AS AS 10S 10S KS KS DS DS BS BS 9S 9S AH AH 10H 10H KH KH DH DH BH BH 9H 9H AD AD 10D 10D KD KD DD DD BD BD 9D 9D AK AK 10K 10K KK KK DK DK BK BK 9K 9K 
   Shuffle Deck:                 K♠ K♦ J♣ Q♣ J♥ 10♣ K♥ Q♥ Q♦ 10♦ J♥ K♦ 9♦ J♠ J♣ 10♦ 9♠ A♣ K♥ J♦ A♥ K♣ Q♠ 10♥ A♥ 9♣ J♦ Q♠ K♣ A♦ 9♥ A♣ A♠ Q♦ Q♣ A♦ K♠ 10♣ 9♦ 10♠ Q♥ 10♥ 9♥ 10♠ 9♣ 9♠ A♠ J♠ 
   Long in English and German:
      Ace of Spades 
      Ass von Spaten 
      Ace of Spades 
      Ass von Spaten 
      Ten of Spades 
      Zhen von Spaten 
 ...
```

## Details

The goal of this library is to be able to support the creation of card
decks of various sizes and suits. Out of the box, the library supports:

* [French Deck](https://en.wikipedia.org/wiki/French_playing_cards)
  * [Pinochle](https://en.wikipedia.org/wiki/Pinochle#Deck)
  * [Spades](https://en.wikipedia.org/wiki/Spades_(card_game)#General_overview) with [Jokers](https://en.wikipedia.org/wiki/Joker_(playing_card))
  * [Standard 52](https://en.wikipedia.org/wiki/Standard_52-card_deck)
* [Tarot](https://en.wikipedia.org/wiki/Tarot#Tarot_gaming_decks)
  * [Major Arcana](https://en.wikipedia.org/wiki/Major_Arcana)
  * [Minor Arcana](https://en.wikipedia.org/wiki/Minor_Arcana)

The project takes advantage of [Project Fluent](https://www.projectfluent.org/)'s
[Rust](https://github.com/projectfluent/fluent-rs) support to offer
internationalization. Current languages supported are
[English](src/fluent/locales/en-US/french-deck.ftl) and
[German](src/fluent/locales/de/french-deck.ftl).

Other possibilities include:

* [French Deck](https://en.wikipedia.org/wiki/French_playing_cards)
  * [Canasta](https://en.wikipedia.org/wiki/Canasta#Cards_and_deal)
  * [Euchre](https://en.wikipedia.org/wiki/Euchre)
* [Skat](https://en.wikipedia.org/wiki/Skat_(card_game)#Deck)

## Responsibilities

* Represent a specific type of card deck.
* Validate that a collection of cards is valid for that type of deck.
* Create a textual representation of a deck that can be serialized and deserialized.
* Shuffle a deck
* Verify that a specific card is playable given a set of discards.
* Determine if two deck types are translatable.

## References

* [Card games in Germany](https://www.pagat.com/national/germany.html)
* [Playing cards in Unicode](https://en.wikipedia.org/wiki/Playing_cards_in_Unicode)

### Other Rust Deck of Cards Libraries

* [ascclemens/cards](https://github.com/ascclemens/cards)
* [locka99/deckofcards-rs](https://github.com/locka99/deckofcards-rs)
* [vsupalov/cards-rs](https://github.com/vsupalov/cards-rs)

## Things to Explore

* [Fluent Templates](https://github.com/XAMPPRocky/fluent-templates)
* Tarot Libraries
  * [lawreka/ascii-tarot](https://github.com/lawreka/ascii-tarot)
  * [pietdaniel/tarot](https://github.com/pietdaniel/tarot)
  * [jeremytarling/ruby-tarot](https://github.com/jeremytarling/ruby-tarot)

## Dependencies

* [Fluent Templates](https://github.com/XAMPPRocky/fluent-templates)
  * [Project Fluent](https://www.projectfluent.org/)
