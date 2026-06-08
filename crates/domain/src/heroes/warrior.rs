use std::collections::HashMap;

use uuid::Uuid;

use crate::{Health, Hero, Stats};

impl Hero {
    pub fn warrior(name: String) -> Self {
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
        }
    }
}
