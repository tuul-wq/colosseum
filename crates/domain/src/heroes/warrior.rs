use uuid::Uuid;

use crate::heroes::class::HeroClass;
use crate::{Health, Hero};

impl Hero {
    pub fn warrior(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            class: HeroClass::Warrior,
            health: Health {
                max: 110,
                current: 110,
            },
            initiative: 8,
        }
    }
}
