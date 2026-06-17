use std::collections::HashMap;

use domain::{Hero, HeroId};
use world::World;

use crate::lineup::ArenaLineup;
use crate::setup::ArenaSetup;

pub struct Arena {
    world: World,
    heroes: HashMap<HeroId, Hero>,
}

impl Arena {
    pub fn new(setup: ArenaSetup) -> Self {
        let arena_lineup = ArenaLineup::new(setup);
        let (left_team, right_team) = arena_lineup.to_world_lineups();

        Self {
            world: World::new(left_team, right_team),
            heroes: arena_lineup.all_heroes(),
        }
    }

    pub fn start() {}
}

#[cfg(test)]
mod tests {
    use domain::{HeroClass, Position};
    use world::Side;

    use super::*;

    #[test]
    fn new_creates_heroes_from_setup_classes() {
        let arena = Arena::new(ArenaSetup {
            left: [HeroClass::Mage, HeroClass::Warrior, HeroClass::Warrior],
            right: [HeroClass::Warrior, HeroClass::Mage, HeroClass::Warrior],
        });

        let left_frontline = arena
            .world
            .hero_at(Side::Left, Position::Frontline)
            .expect("left frontline hero should exist");
        let right_midline = arena
            .world
            .hero_at(Side::Right, Position::Midline)
            .expect("right midline hero should exist");

        assert!(matches!(
            arena.heroes.get(left_frontline).map(|hero| hero.class),
            Some(HeroClass::Mage)
        ));
        assert!(matches!(
            arena.heroes.get(right_midline).map(|hero| hero.class),
            Some(HeroClass::Mage)
        ));
        assert_eq!(arena.heroes.len(), 6);
    }

    #[test]
    fn new_places_three_vs_three_backline_heroes() {
        let arena = Arena::new(ArenaSetup {
            left: [HeroClass::Warrior, HeroClass::Warrior, HeroClass::Mage],
            right: [HeroClass::Mage, HeroClass::Warrior, HeroClass::Warrior],
        });

        let left_backline = arena
            .world
            .hero_at(Side::Left, Position::Backline)
            .expect("left backline hero should exist");

        assert!(matches!(
            arena.heroes.get(left_backline).map(|hero| hero.class),
            Some(HeroClass::Mage)
        ));
        assert_eq!(arena.heroes.len(), 6);
    }
}
