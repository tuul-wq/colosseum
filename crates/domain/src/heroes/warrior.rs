use crate::heroes::class::HeroClass;
use crate::heroes::{Health, Hero, HeroId};

const WARRIOR_NAMES: [&str; 6] = ["Aldric", "Garrick", "Brom", "Cedric", "Darian", "Ronan"];

impl Hero {
    pub fn warrior() -> Self {
        let name = WARRIOR_NAMES[rand::random_range(0..WARRIOR_NAMES.len())].to_owned();
        let id = format!("{}_{}", name, rand::random_range(0..100));

        Self {
            id: HeroId::new(id),
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
