use arguments::{Cli, Commands};
use clap::Parser;
use domain::{MageHero, Position, WarriorHero};

mod arguments;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fight => println!("Fight!"),
    }

    let mage = MageHero::new("Mage_111".to_owned(), Position { x: 0, y: 1 });
    let warrior = WarriorHero::new("Mage_111".to_owned(), Position { x: 0, y: 1 });

    println!("mage - {:?}", mage);
    println!("warrior - {:?}", warrior);
}
