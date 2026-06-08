use std::collections::HashMap;

use uuid::Uuid;

use crate::{AbilityId, Health, Hero, Stats};

use crate::abilities::mage;

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
            abilities: HashMap::from([
                (AbilityId::Fireball, mage::fireball()),
                (AbilityId::ArcaneExplosion, mage::arcane_explosion()),
            ]),
        }
    }
}
