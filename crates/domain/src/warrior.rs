use uuid::Uuid;

use crate::{Health, Hero, Position, Stats, Weapon, class::HeroClass};

#[derive(Debug)]
pub struct WarriorClass {
    /// Rage to cast abilities.
    pub rage: Rage,
}

#[derive(Debug)]
pub struct Rage {
    pub max: u8,
    pub current: u8,
}

impl Hero {
    pub fn warrior(name: String, position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            stats: Stats {
                name,
                health: Health {
                    max: 100,
                    current: 100,
                },
                alive: true,
                initiative: 10,
                speed: 2,
            },
            class: HeroClass::Warrior(WarriorClass {
                rage: Rage {
                    max: 100,
                    current: 0,
                },
            }),
            weapon: Weapon {
                damage: 10,
                range: 1,
            },
            position,
        }
    }
}
