use std::collections::HashMap;

use crate::abilities::{Ability, AbilityId, mage as mage_abilities, warrior as warrior_abilities};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroClass {
    Mage,
    Warrior,
}

impl HeroClass {
    pub fn abilities(&self) -> HashMap<AbilityId, Ability> {
        match self {
            HeroClass::Mage => HashMap::from([
                (AbilityId::MainAttack, mage_abilities::mage_main_attack(5)),
                (
                    AbilityId::OffhandAttack,
                    mage_abilities::mage_offhand_attack(3),
                ),
                (AbilityId::Fireball, mage_abilities::fireball()),
                (
                    AbilityId::ArcaneExplosion,
                    mage_abilities::arcane_explosion(),
                ),
            ]),
            HeroClass::Warrior => HashMap::from([
                (
                    AbilityId::MainAttack,
                    warrior_abilities::warrior_main_attack(9),
                ),
                (
                    AbilityId::OffhandAttack,
                    warrior_abilities::warrior_offhand_attack(5),
                ),
                (AbilityId::Slam, warrior_abilities::slam()),
                (AbilityId::Whirlwind, warrior_abilities::whirlwind()),
            ]),
        }
    }
}
