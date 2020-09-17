fn main() {
    let pack = cardpack::Pack::french_deck();

    let deal = "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

    parse_pbn_deal(pack.cards(), deal.to_string());
}

// "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982"
pub fn parse_pbn_deal(_deck: &cardpack::Pile, s: String) -> Vec<cardpack::Pile> {
    let declarer = s.chars().next().unwrap();
    let directions = compass(declarer);

    let cards = &s[2..];
    println!("Declarer is: {:?}\n", directions.get(0).unwrap());

    let mut diriter = directions.iter();

    for hand in cards.split_whitespace() {
        let dir = diriter.next().unwrap();

        println!("{:?}: {}", dir, hand);
    }

    vec![]
}

fn to_pile(s: &str, ) -> cardpack::Pile {
    let rawsuits = s.split(".");

    cardpack::Pile::default()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    South,
    West,
    North,
    East,
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
    }
}