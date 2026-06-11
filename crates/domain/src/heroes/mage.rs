use rand;

use crate::{Health, Hero, HeroClass, HeroId};

const MAGE_NAMES: [&str; 6] = ["Elowen", "Seraphina", "Mirella", "Isolde", "Lyra", "Amara"];

impl Hero {
    pub fn mage() -> Self {
        let name = MAGE_NAMES[rand::random_range(0..MAGE_NAMES.len())].to_owned();
        let id = format!("{}_{}", name, rand::random_range(0..100));

        Self {
            id: HeroId::new(id),
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
