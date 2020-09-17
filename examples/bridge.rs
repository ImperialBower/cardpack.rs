fn main() {
    let pack = cardpack::Pack::french_deck();

    println!("Take a PBN Deal String and convert it into Bridge hands (packs):");

    // This is a deal string from a Portable Bridge Notation document.
    let deal = "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

    println!("[Deal \"{}\"]\n", deal);

    parse_pbn_deal(pack.cards(), deal);
}

// "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982"
pub fn parse_pbn_deal(deck: &cardpack::Pile, s: &str) -> Vec<cardpack::Pile> {
    let declarer = s.chars().next().unwrap();
    let directions = compass(declarer);

    let cards = &s[2..];
    println!("Declarer is: {:?}\n", directions.get(0).unwrap());

    let mut diriter = directions.iter();

    for hand in cards.split_whitespace() {
        let dir = diriter.next().unwrap();
        let pile = to_pile(deck, hand);

        println!("{:<5}: {}", dir.to_string(), pile.by_symbol_index());
    }

    vec![]
}

fn to_pile(deck: &cardpack::Pile, s: &str) -> cardpack::Pile {
    let rawsuits: Vec<&str> = s.split(".").collect();

    let mut v: Vec<String> = Vec::new();
    v.append(&mut splice_suit_in(&rawsuits.get(0).unwrap(), 'S'));
    v.append(&mut splice_suit_in(&rawsuits.get(1).unwrap(), 'H'));
    v.append(&mut splice_suit_in(&rawsuits.get(2).unwrap(), 'D'));
    v.append(&mut splice_suit_in(&rawsuits.get(3).unwrap(), 'C'));

    let coll: Vec<cardpack::Card> = v.iter().map(|s| deck.card_by_index(s.as_str()).unwrap().clone()).collect();

    cardpack::Pile::new_from_vector(coll)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    South,
    West,
    North,
    East,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn compass(c: char) -> Vec<Direction> {
    // let points = ['S', 'W', 'N', 'E'];
    match c {
        'S' => vec![Direction::South, Direction::West, Direction::North, Direction::East],
        'W' => vec![Direction::West, Direction::North, Direction::East, Direction::South],
        'N' => vec![Direction::North, Direction::East, Direction::South, Direction::West],
        _ => vec![Direction::East, Direction::South, Direction::West, Direction::North],
    }
}

fn splice_suit_in(s: &str, suit: char) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    for c in s.chars() {
        v.push(format!("{}{}", c, suit));
    }
    v
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_deck_tests {
    use super::*;

    #[test]
    fn test_compass() {
        assert_eq!(vec![Direction::North, Direction::East, Direction::South, Direction::West], compass('N'))
    }

    #[test]
    fn test_splice_suit_in() {
        let raw = "Q42";

        let expected = vec!["QS".to_string(), "4S".to_string(), "2S".to_string()];

        assert_eq!(expected, splice_suit_in(raw, 'S'));
    }
}