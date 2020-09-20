use cardpack::BridgeBoard;

/// This is a complex example, and gives an idea of some of the work planned around Bridge.
fn main() {
    println!("First, let's deal out a Bridge hand:");
    let board = BridgeBoard::deal();
    board.demo();
    println!();

    println!("We can also take the board and convert it to a Portable Bridge Notation Deal String:");
    println!("[Deal \"{}\"]\n", board.to_pbn_deal());

    println!();
    println!("Now, let's take a PBN Deal String and convert it into Bridge hands (packs):");

    // This is a deal string from a Portable Bridge Notation document.
    let deal = "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

    let board = BridgeBoard::from_pbn_deal(deal);

    println!("[Deal \"{}\"]\n", deal);

    board.demo();
}
