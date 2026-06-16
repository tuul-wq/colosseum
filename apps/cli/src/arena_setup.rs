use arena::ArenaSetup;
use clap::{CommandFactory, error::ErrorKind};
use domain::HeroClass;

use crate::arguments::{Cli, HeroArg, TeamArg};

pub fn from_cli(cli: Cli) -> Result<ArenaSetup, clap::Error> {
    Ok(ArenaSetup {
        left: into_classes(cli.team_1, "team-1")?,
        right: into_classes(cli.team_2, "team-2")?,
    })
}

fn into_classes(team: TeamArg, team_name: &'static str) -> Result<[HeroClass; 3], clap::Error> {
    let heroes: [HeroArg; 3] = team.heroes.try_into().map_err(|heroes: Vec<HeroArg>| {
        Cli::command().error(
            ErrorKind::ValueValidation,
            format!("3v3 expects 3 heroes in {team_name}, got {}", heroes.len()),
        )
    })?;

    Ok(heroes.map(hero_class))
}

fn hero_class(hero: HeroArg) -> HeroClass {
    match hero {
        HeroArg::Warrior => HeroClass::Warrior,
        HeroArg::Mage => HeroClass::Mage,
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[test]
    fn builds_setup_from_cli() {
        let cli =
            Cli::try_parse_from(["cli", "mage,warrior,mage", "warrior,mage,warrior"]).unwrap();

        let setup = from_cli(cli).unwrap();

        assert_eq!(
            setup.left,
            [HeroClass::Mage, HeroClass::Warrior, HeroClass::Mage]
        );
        assert_eq!(
            setup.right,
            [HeroClass::Warrior, HeroClass::Mage, HeroClass::Warrior]
        );
    }

    #[test]
    fn validates_team_size_against_lineup() {
        let cli = Cli::try_parse_from(["cli", "mage,warrior", "warrior,mage,mage"]).unwrap();

        let Err(error) = from_cli(cli) else {
            panic!("expected invalid team size");
        };

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
        assert!(
            error
                .to_string()
                .contains("3v3 expects 3 heroes in team-1, got 2")
        );
    }
}
