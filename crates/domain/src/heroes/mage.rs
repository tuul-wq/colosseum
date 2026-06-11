use uuid::Uuid;

use crate::heroes::class::HeroClass;
use crate::{Health, Hero};

impl Hero {
    pub fn mage(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            class: HeroClass::Mage,
            health: Health {
                max: 55,
                current: 55,
            },
            initiative: 13,
        }
    }
}
