use arena::{Arena, ArenaLineup};
use arguments::{Cli, HeroArg, LineupArg, TeamArg};
use clap::Parser;
use domain::Hero;

mod arguments;

fn main() {
    let cli = Cli::parse();
    let lineup_name = cli.lineup.as_str();

    match cli.command {
        Commands::Fight => println!("Fight!"),
    }
}

    let mage = Hero::mage();
    let warrior = Hero::warrior();

    let heroes = vec![mage, warrior];

    println!("mage - {:?}", heroes[0]);
    println!("warrior - {:?}", heroes[1]);
}
