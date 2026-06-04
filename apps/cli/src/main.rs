use arguments::{Cli, Commands};
use clap::Parser;
use domain::{Hero, Position};

mod arguments;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fight => println!("Fight!"),
    }

    let mage = Hero::mage("Mage_1".into(), Position::Backline);
    let warrior = Hero::warrior("Warrior_1".into(), Position::Frontline);

    let heroes = vec![mage, warrior];

    println!("mage - {:?}", heroes[0]);
    println!("warrior - {:?}", heroes[1]);
}
