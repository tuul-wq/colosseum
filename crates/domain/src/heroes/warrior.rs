use uuid::Uuid;

use crate::heroes::class::HeroClass;
use crate::{Health, Hero, Stats};

impl Hero {
    pub fn warrior(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            class: HeroClass::Warrior,
            stats: Stats {
                name,
                health: Health {
                    max: 100,
                    current: 100,
                },
                initiative: 10,
                speed: 2,
            },
        }
    }
}
