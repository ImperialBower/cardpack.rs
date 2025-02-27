use cardpack::prelude::*;
use clap::Parser;

/// Run all of the decks with 1 for each:
///
/// `cargo run --example cli2 -- -temfkpsac -v`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'v', long)]
    verbose: bool,

    #[clap(short = 'c', long)]
    canasta: bool,

    #[clap(short = 'e', long)]
    euchre: bool,

    #[clap(short = 'm', long)]
    manila: bool,

    #[clap(short = 'f', long)]
    french: bool,

    #[clap(short = 'p', long)]
    pinochle: bool,

    #[clap(short = 'k', long)]
    skat: bool,

    #[clap(short = 'a', long)]
    spades: bool,

    #[clap(short = 's', long)]
    standard: bool,

    #[clap(short = 't', long)]
    tarot: bool,
}

fn main() -> Result<(), CardError> {
    let args = Args::parse();

    if args.tarot {
        TarotDeck::demo(args.verbose);
    }

    if args.canasta {
        CanastaDeck::demo(args.verbose);
    }

    if args.euchre {
        Euchre24Deck::demo(args.verbose);
    }

    if args.manila {
        ShortDeck::demo(args.verbose);
    }

    if args.french {
        FrenchDeck::demo(args.verbose);
    }

    if args.spades {
        SpadesDeck::demo(args.verbose);
    }

    if args.pinochle {
        PinochleDeck::demo(args.verbose);
    }

    if args.skat {
        SkatDeck::demo(args.verbose);
    }

    if args.standard {
        Standard52Deck::demo(args.verbose);
    }

    Ok(())
}
