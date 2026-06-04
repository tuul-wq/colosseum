use std::collections::HashMap;

use uuid::Uuid;

use crate::{Health, Hero, Position, Stats};

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
                initiative: 10,
                speed: 2,
            },
            abilities: HashMap::new(),
            position,
        }
    }
}
