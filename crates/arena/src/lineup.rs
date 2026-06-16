use std::collections::HashMap;

use domain::{Hero, HeroClass, HeroId, Position};
use world::Lineup as FormationLineup;

use crate::setup::ArenaSetup;

pub struct ArenaLineup {
    left: TeamLineup,
    right: TeamLineup,
}

impl ArenaLineup {
    pub fn new(setup: ArenaSetup) -> Self {
        Self {
            left: TeamLineup::new(setup.left),
            right: TeamLineup::new(setup.right),
        }
    }

    pub fn to_world_lineups(&self) -> (FormationLineup, FormationLineup) {
        (self.left.to_world_lineup(), self.right.to_world_lineup())
    }

    pub fn all_heroes(self) -> HashMap<HeroId, Hero> {
        self.left
            .into_heroes()
            .chain(self.right.into_heroes())
            .map(|hero| (hero.id.clone(), hero))
            .collect()
    }
}

struct TeamLineup {
    slots: Vec<HeroSlot>,
}

impl TeamLineup {
    fn new(classes: [HeroClass; Position::COUNT]) -> Self {
        Self {
            slots: Position::ordered()
                .into_iter()
                .zip(classes)
                .map(|(position, class)| HeroSlot::new(position, class))
                .collect(),
        }
    }

    fn to_world_lineup(&self) -> FormationLineup {
        let mut slots = self.slots.iter().collect::<Vec<_>>();
        slots.sort_by_key(|slot| slot.position.index());

        let heroes = slots
            .into_iter()
            .map(|slot| slot.hero.id.clone())
            .collect::<Vec<_>>();

        FormationLineup::new(heroes[0].clone(), heroes[1].clone(), heroes[2].clone())
    }

    fn into_heroes(self) -> impl Iterator<Item = Hero> {
        self.slots.into_iter().map(|slot| slot.hero)
    }
}

struct HeroSlot {
    position: Position,
    hero: Hero,
}

impl HeroSlot {
    fn new(position: Position, class: HeroClass) -> Self {
        Self {
            position,
            hero: Hero::new(class),
        }
    }
}
