fn main() {
    let pack = cardpack::Pack::french_deck();

    let deal = "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

    parse_pbn_deal(pack.cards(), deal.to_string());
}

// "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982"
pub fn parse_pbn_deal(_deck: &cardpack::Pile, s: String) -> Vec<cardpack::Pile> {
    for hand in s.split_whitespace() {
        println!("HAND: {}", hand);
    }

    vec![]
}