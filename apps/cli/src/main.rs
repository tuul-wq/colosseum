use arguments::{Cli, Commands};
use clap::Parser;
use domain::Hero;

mod arguments;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fight => println!("Fight!"),
    }

    let mage = Hero::mage("Mage_1".to_owned());
    let warrior = Hero::warrior("Warrior_1".to_owned());

    let heroes = vec![mage, warrior];

    println!("mage - {:?}", heroes[0]);
    println!("warrior - {:?}", heroes[1]);
}
