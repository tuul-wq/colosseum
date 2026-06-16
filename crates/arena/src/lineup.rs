use std::collections::HashMap;

use domain::{Hero, HeroClass, HeroId, Position};
use world::Lineup as FormationLineup;

use crate::setup::{ArenaSetup, Layout, TeamSetup};

pub struct ArenaLineup {
    left: TeamLineup,
    right: TeamLineup,
}

impl ArenaLineup {
    pub fn from_setup(setup: ArenaSetup) -> Self {
        match setup {
            ArenaSetup::TwoVsTwo { left, right } => Self::new(
                TeamSetup::new(Layout::Two, left),
                TeamSetup::new(Layout::Two, right),
            ),
            ArenaSetup::ThreeVsThree { left, right } => Self::new(
                TeamSetup::new(Layout::Three, left),
                TeamSetup::new(Layout::Three, right),
            ),
        }
    }

    pub fn new<const L: usize, const R: usize>(left: TeamSetup<L>, right: TeamSetup<R>) -> Self {
        assert_eq!(left.layout, right.layout);

        Self {
            left: TeamLineup::from_setup(left),
            right: TeamLineup::from_setup(right),
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
    fn from_setup<const N: usize>(setup: TeamSetup<N>) -> Self {
        assert_eq!(setup.layout.position_count(), setup.classes.len());

        Self {
            slots: Position::range(setup.layout.position_count())
                .zip(setup.classes)
                .map(|(position, class)| HeroSlot::new(position, class))
                .collect(),
        }
    }

    fn to_world_lineup(&self) -> FormationLineup {
        let mut slots = self.slots.iter().collect::<Vec<_>>();
        slots.sort_by_key(|slot| slot.position.index());

        FormationLineup::new(slots.into_iter().map(|slot| slot.hero.id.clone()).collect())
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
