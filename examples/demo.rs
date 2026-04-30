use cardpack::prelude::*;
use clap::Parser;

/// Run all of the decks with one of each:
///
/// `cargo run --example demo -- --all -v`
///
/// Or pick specific ones:
///
/// `cargo run --example demo -- -temfkpsac -v`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'v', long)]
    verbose: bool,

    /// Iterate every deck the crate ships (via `DeckKind::all()`).
    #[clap(long)]
    all: bool,

    #[clap(short = 'c', long)]
    canasta: bool,

    #[clap(short = 'e', long)]
    euchre: bool,

    #[clap(short = 'm', long)]
    short: bool,

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

    if args.all {
        for kind in DeckKind::all() {
            kind.demo(args.verbose);
        }
        return Ok(());
    }

    if args.tarot {
        DeckKind::Tarot.demo(args.verbose);
    }

    if args.canasta {
        DeckKind::Canasta.demo(args.verbose);
    }

    if args.euchre {
        DeckKind::Euchre24.demo(args.verbose);
    }

    if args.short {
        DeckKind::Short.demo(args.verbose);
    }

    if args.french {
        DeckKind::French.demo(args.verbose);
    }

    if args.spades {
        DeckKind::Spades.demo(args.verbose);
    }

    if args.pinochle {
        DeckKind::Pinochle.demo(args.verbose);
    }

    if args.skat {
        DeckKind::Skat.demo(args.verbose);
    }

    if args.standard {
        DeckKind::Standard52.demo(args.verbose);
    }

    Ok(())
}
