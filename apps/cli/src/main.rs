use arena::Arena;
use clap::Parser;

use crate::arguments::Cli;

mod arena_setup;
mod arguments;

fn main() {
    let setup =
        arena_setup::from_cli(Cli::parse()).unwrap_or_else(|error: clap::Error| error.exit());

    Arena::new(setup);
}
