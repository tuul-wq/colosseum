use std::collections::HashMap;

use uuid::Uuid;

use crate::abilities::warrior;
use crate::{AbilityId, Health, Hero, Stats};

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
            abilities: HashMap::from([
                (AbilityId::MainAttack, warrior::warrior_main_attack(8)),
                (AbilityId::OffhandAttack, warrior::warrior_offhand_attack(4)),
                (AbilityId::Slam, warrior::slam()),
                (AbilityId::Whirlwind, warrior::whirlwind()),
            ]),
        }
    }
}
