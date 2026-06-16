use std::fmt;
use std::str::FromStr;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(value_enum, value_name = "lineup")]
    pub lineup: LineupArg,
    #[arg(
        value_name = "team-1",
        help = "Comma-separated heroes, e.g. mage,warrior"
    )]
    pub team_1: TeamArg,
    #[arg(
        value_name = "team-2",
        help = "Comma-separated heroes, e.g. warrior,mage"
    )]
    pub team_2: TeamArg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LineupArg {
    #[value(name = "2v2")]
    TwoVsTwo,
    #[value(name = "3v3")]
    ThreeVsThree,
}

impl fmt::Display for LineupArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwoVsTwo => f.write_str("2v2"),
            Self::ThreeVsThree => f.write_str("3v3"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamArg {
    pub heroes: Vec<HeroArg>,
}

impl FromStr for TeamArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err("team must contain at least one hero".to_owned());
        }

        let heroes = s
            .split(',')
            .map(|hero| HeroArg::from_str(hero.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { heroes })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroArg {
    Warrior,
    Mage,
}

impl FromStr for HeroArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "warrior" => Ok(Self::Warrior),
            "mage" => Ok(Self::Mage),
            value => Err(format!(
                "unknown hero `{value}`; expected one of: warrior, mage"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::error::ErrorKind;

    use super::*;

    #[test]
    fn parses_cli_arguments() {
        let cli = Cli::try_parse_from(["cli", "2v2", "mage,warrior", "warrior,mage"]).unwrap();

        assert_eq!(cli.lineup, LineupArg::TwoVsTwo);
        assert_eq!(cli.team_1.heroes, [HeroArg::Mage, HeroArg::Warrior]);
        assert_eq!(cli.team_2.heroes, [HeroArg::Warrior, HeroArg::Mage]);
    }

    #[test]
    fn rejects_unknown_hero_values() {
        let error = Cli::try_parse_from(["cli", "2v2", "rogue,mage", "warrior,mage"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
    }
}
