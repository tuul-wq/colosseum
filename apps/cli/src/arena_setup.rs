use arena::ArenaSetup;
use clap::{CommandFactory, error::ErrorKind};
use domain::HeroClass;

use crate::arguments::{Cli, HeroArg, LineupArg, TeamArg};

pub fn from_cli(cli: Cli) -> Result<ArenaSetup, clap::Error> {
    match cli.lineup {
        LineupArg::TwoVsTwo => Ok(ArenaSetup::TwoVsTwo {
            left: into_classes(cli.team_1, "team-1", cli.lineup)?,
            right: into_classes(cli.team_2, "team-2", cli.lineup)?,
        }),
        LineupArg::ThreeVsThree => Ok(ArenaSetup::ThreeVsThree {
            left: into_classes(cli.team_1, "team-1", cli.lineup)?,
            right: into_classes(cli.team_2, "team-2", cli.lineup)?,
        }),
    }
}

fn into_classes<const N: usize>(
    team: TeamArg,
    team_name: &'static str,
    lineup: LineupArg,
) -> Result<[HeroClass; N], clap::Error> {
    let heroes: [HeroArg; N] = team.heroes.try_into().map_err(|heroes: Vec<HeroArg>| {
        Cli::command().error(
            ErrorKind::ValueValidation,
            format!(
                "{lineup} expects {N} heroes in {team_name}, got {}",
                heroes.len()
            ),
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
    use arena::ArenaSetup;
    use clap::Parser;

    use super::*;

    #[test]
    fn builds_two_vs_two_setup_from_cli() {
        let cli = Cli::try_parse_from(["cli", "2v2", "mage,warrior", "warrior,mage"]).unwrap();

        let setup = from_cli(cli).unwrap();

        let ArenaSetup::TwoVsTwo { left, right } = setup else {
            panic!("expected 2v2 setup");
        };

        assert_eq!(left, [HeroClass::Mage, HeroClass::Warrior]);
        assert_eq!(right, [HeroClass::Warrior, HeroClass::Mage]);
    }

    #[test]
    fn validates_team_size_against_lineup() {
        let cli = Cli::try_parse_from(["cli", "3v3", "mage,warrior", "warrior,mage,mage"]).unwrap();

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
