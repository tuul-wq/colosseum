use std::str::FromStr;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[arg(value_enum, value_name = "lineup")]
    pub lineup: LineupArg,
    #[arg(value_name = "team-1")]
    pub team_1: TeamArg,
    #[arg(value_name = "team-2")]
    pub team_2: TeamArg,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "fight")]
    Fight,
}
