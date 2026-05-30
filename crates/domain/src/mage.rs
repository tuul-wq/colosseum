use uuid::Uuid;

use crate::{Hero, Position, Stats, Weapon};

pub type MageHero = Hero<Mage>;

#[derive(Debug)]
pub struct Mage {
    /// Mana points to cast spells.
    pub mana: u8,
}

impl MageHero {
    pub fn new(name: String, position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            stats: Stats {
                name,
                health: 60,
                alive: true,
                initiative: 12,
                speed: 2,
            },
            class: Mage { mana: 100 },
            weapon: Weapon {
                damage: 5,
                range: 3,
            },
            position,
        }
    }
}
