use std::str::FromStr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(
        value_name = "team-1",
        help = "Comma-separated 3-hero team, e.g. mage,warrior,archer"
    )]
    pub team_1: TeamArg,
    #[arg(
        value_name = "team-2",
        help = "Comma-separated 3-hero team, e.g. warrior,mage,archer"
    )]
    pub team_2: TeamArg,
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
        let cli =
            Cli::try_parse_from(["cli", "mage,warrior,mage", "warrior,mage,warrior"]).unwrap();

        assert_eq!(
            cli.team_1.heroes,
            [HeroArg::Mage, HeroArg::Warrior, HeroArg::Mage]
        );
        assert_eq!(
            cli.team_2.heroes,
            [HeroArg::Warrior, HeroArg::Mage, HeroArg::Warrior]
        );
    }

    #[test]
    fn rejects_unknown_hero_values() {
        let error =
            Cli::try_parse_from(["cli", "rogue,mage,mage", "warrior,mage,mage"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
    }
}
