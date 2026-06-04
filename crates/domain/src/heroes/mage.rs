use std::collections::HashMap;

use uuid::Uuid;

use crate::{Health, Hero, Position, Stats};

impl Hero {
    pub fn mage(name: String, position: Position) -> Self {
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
            abilities: HashMap::new(),
            position,
        }
    }
}
