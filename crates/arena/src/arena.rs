use std::collections::HashMap;

use domain::{Hero, HeroClass, HeroId};
use world::World;

use crate::lineup::ArenaLineup;
use crate::setup::ArenaSetup;

pub struct Arena {
    world: World,
    heroes: HashMap<HeroId, Hero>,
}

impl Arena {
    pub fn new(setup: ArenaSetup) -> Self {
        let arena_lineup = ArenaLineup::from_setup(setup);
        let (left_team, right_team) = arena_lineup.to_world_lineups();

        Self {
            world: World::new(left_team, right_team),
            heroes: arena_lineup.all_heroes(),
        }
    }

    pub fn two_vs_two(left: [HeroClass; 2], right: [HeroClass; 2]) -> Self {
        Self::new(ArenaSetup::TwoVsTwo { left, right })
    }

    pub fn three_vs_three(left: [HeroClass; 3], right: [HeroClass; 3]) -> Self {
        Self::new(ArenaSetup::ThreeVsThree { left, right })
    }
}

#[cfg(test)]
mod tests {
    use domain::Position;
    use world::Side;

    use super::*;

    #[test]
    fn new_creates_heroes_from_setup_classes() {
        let arena = Arena::new(ArenaSetup::TwoVsTwo {
            left: [HeroClass::Mage, HeroClass::Warrior],
            right: [HeroClass::Warrior, HeroClass::Mage],
        });

        let left_frontline = arena
            .world
            .hero_at(Side::Left, Position::FRONTLINE)
            .expect("left frontline hero should exist");
        let right_midline = arena
            .world
            .hero_at(Side::Right, Position::MIDLINE)
            .expect("right midline hero should exist");

        assert!(matches!(
            arena.heroes.get(left_frontline).map(|hero| hero.class),
            Some(HeroClass::Mage)
        ));
        assert!(matches!(
            arena.heroes.get(right_midline).map(|hero| hero.class),
            Some(HeroClass::Mage)
        ));
        assert_eq!(arena.heroes.len(), 4);
    }

    #[test]
    fn new_places_three_vs_three_backline_heroes() {
        let arena = Arena::new(ArenaSetup::ThreeVsThree {
            left: [HeroClass::Warrior, HeroClass::Warrior, HeroClass::Mage],
            right: [HeroClass::Mage, HeroClass::Warrior, HeroClass::Warrior],
        });

        let left_backline = arena
            .world
            .hero_at(Side::Left, Position::BACKLINE)
            .expect("left backline hero should exist");

        assert!(matches!(
            arena.heroes.get(left_backline).map(|hero| hero.class),
            Some(HeroClass::Mage)
        ));
        assert_eq!(arena.heroes.len(), 6);
    }
}
