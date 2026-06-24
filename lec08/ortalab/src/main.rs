use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};

use clap::Parser;
use ortalib::{Chips, Mult, Round};

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round, opts.explain);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn score(round: Round, explain: bool) -> (Chips, Mult) {
    if explain {
        println!("I am explaining!");
    }
    
    let mut chips = 0.0;
    for card in round.cards_played {
        chips += card.rank.rank_value();
    }

    println!("{:?}", round.jokers);
    for joker_card in round.jokers {
        let joker = joker_card.joker;
        
    }

    (chips, 1.0)
}
