use uuid::Uuid;

use crate::{Health, Hero, Position, Stats, Weapon, class::HeroClass};

#[derive(Debug)]
pub struct MageClass {
    /// Mana points to cast abilities.
    pub mana: Mana,
}

#[derive(Debug)]
pub struct Mana {
    pub max: u8,
    pub current: u8,
}

impl Hero {
    pub fn mage(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            stats: Stats {
                name,
                health: Health {
                    max: 60,
                    current: 60,
                },
                initiative: 12,
                speed: 2,
            },
            class: HeroClass::Mage(MageClass {
                mana: Mana {
                    max: 100,
                    current: 100,
                },
            }),
            weapon: Weapon {
                damage: 5,
                range: 3,
            },
            position: Position::new(0, 0),
        }
    }
}
