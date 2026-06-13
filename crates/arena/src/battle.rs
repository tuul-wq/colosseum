use std::collections::HashMap;

use domain::{Hero, HeroId};
use world::World;
use world::formation::Lineup as FormationLineup;

pub struct Arena {
    world: World,
    heroes: HashMap<HeroId, Hero>,
}

pub enum ArenaLineup {
    TwoVsTwo { left: [Hero; 2], right: [Hero; 2] },
    ThreeVsThree { left: [Hero; 3], right: [Hero; 3] },
}

impl ArenaLineup {
    fn to_world_lineups(&self) -> (FormationLineup, FormationLineup) {
        match self {
            ArenaLineup::TwoVsTwo {
                left: [l_frontline, l_midline],
                right: [r_frontline, r_midline],
            } => (
                FormationLineup::Two {
                    frontline: l_frontline.id.clone(),
                    midline: l_midline.id.clone(),
                },
                FormationLineup::Two {
                    frontline: r_frontline.id.clone(),
                    midline: r_midline.id.clone(),
                },
            ),
            ArenaLineup::ThreeVsThree {
                left: [l_frontline, l_midline, l_backline],
                right: [r_frontline, r_midline, r_backline],
            } => (
                FormationLineup::Three {
                    frontline: l_frontline.id.clone(),
                    midline: l_midline.id.clone(),
                    backline: l_backline.id.clone(),
                },
                FormationLineup::Three {
                    frontline: r_frontline.id.clone(),
                    midline: r_midline.id.clone(),
                    backline: r_backline.id.clone(),
                },
            ),
        }
    }

    fn all_heroes(self) -> HashMap<HeroId, Hero> {
        fn heroes_map<const N: usize>(left: [Hero; N], right: [Hero; N]) -> HashMap<HeroId, Hero> {
            left.into_iter()
                .chain(right)
                .map(|h| (h.id.clone(), h))
                .collect()
        }

        match self {
            ArenaLineup::TwoVsTwo { left, right } => heroes_map(left, right),
            ArenaLineup::ThreeVsThree { left, right } => heroes_map(left, right),
        }
    }
}

impl Arena {
    pub fn new(arena_lineup: ArenaLineup) -> Self {
        let (left_team, right_team) = arena_lineup.to_world_lineups();

        Self {
            world: World::new(left_team, right_team),
            heroes: arena_lineup.all_heroes(),
        }
    }
}
