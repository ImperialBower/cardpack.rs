use cardpack::preludes::funky::*;
use clap::Parser;

/// Demo for all Balatro (funky) decks:
///
/// `cargo run --example funky_demo -- -bjpstu -v`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'v', long)]
    verbose: bool,

    #[clap(short = 'b', long)]
    basic: bool,

    #[clap(short = 'j', long)]
    joker: bool,

    #[clap(short = 'p', long)]
    planet: bool,

    #[clap(short = 's', long)]
    spectral: bool,

    #[clap(short = 't', long)]
    tarot: bool,

    #[clap(short = 'u', long)]
    voucher: bool,
}

fn demo_pile(name: &str, pile: &BuffoonPile, verbose: bool) {
    println!();
    println!("{name} ({} cards):", pile.len());
    println!("  {pile}");
    if verbose {
        for card in pile.iter() {
            println!("    {card}");
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.basic {
        demo_pile(
            "Basic Deck",
            &BuffoonPile::from(&Deck::DECK[..]),
            args.verbose,
        );
        demo_pile(
            "Abandoned Deck",
            &BuffoonPile::from(&Deck::ABANDONED_DECK[..]),
            args.verbose,
        );
        demo_pile(
            "Checkered Deck",
            &BuffoonPile::from(&Deck::CHECKERED_DECK[..]),
            args.verbose,
        );
    }

    if args.joker {
        demo_pile("Common Jokers", &Joker::pile_common(), args.verbose);
        demo_pile("Uncommon Jokers", &Joker::pile_uncommon(), args.verbose);
        demo_pile("Rare Jokers", &Joker::pile_rare(), args.verbose);
        demo_pile("Legendary Jokers", &Joker::pile_legendary(), args.verbose);
    }

    if args.planet {
        demo_pile(
            "Planet Deck",
            &BuffoonPile::from(&Planet::DECK[..]),
            args.verbose,
        );
        demo_pile(
            "Secret Planet Deck",
            &BuffoonPile::from(&Planet::SECRET_DECK[..]),
            args.verbose,
        );
    }

    if args.spectral {
        demo_pile(
            "Spectral Deck",
            &BuffoonPile::from(&Spectral::DECK[..]),
            args.verbose,
        );
    }

    if args.tarot {
        demo_pile(
            "Tarot (Major Arcana)",
            &BuffoonPile::from(&MajorArcana::DECK[..]),
            args.verbose,
        );
    }

    if args.voucher {
        demo_pile(
            "Base Vouchers",
            &BuffoonPile::from(&Voucher::BASE_VOUCHERS[..]),
            args.verbose,
        );
        demo_pile(
            "Upgraded Vouchers",
            &BuffoonPile::from(&Voucher::UPGRADED_VOUCHERS[..]),
            args.verbose,
        );
    }
}
