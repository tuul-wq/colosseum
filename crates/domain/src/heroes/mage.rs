use uuid::Uuid;

use crate::heroes::class::HeroClass;
use crate::{Health, Hero, Stats};

impl Hero {
    pub fn mage(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            class: HeroClass::Mage,
            stats: Stats {
                name,
                health: Health {
                    max: 60,
                    current: 60,
                },
                initiative: 12,
                speed: 2,
            },
        }
    }
}
