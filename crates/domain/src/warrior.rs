use uuid::Uuid;

use crate::{Health, Hero, Position, Stats, Weapon};

pub type WarriorHero = Hero<Warrior>;

#[derive(Debug)]
pub struct Warrior {
    /// Rage to cast spells.
    pub rage: u8,
}

impl WarriorHero {
    pub fn new(name: String, position: Position) -> Self {
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
            class: Warrior { rage: 100 },
            weapon: Weapon {
                damage: 10,
                range: 1,
            },
            position,
        }
    }
}
