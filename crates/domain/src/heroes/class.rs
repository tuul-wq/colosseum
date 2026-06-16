use std::collections::HashMap;

use crate::abilities::{Ability, AbilityId, mage_abilities, warrior_abilities};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroClass {
    Mage,
    Warrior,
}

impl HeroClass {
    pub fn abilities(&self) -> HashMap<AbilityId, Ability> {
        match self {
            HeroClass::Mage => mage_abilities(),
            HeroClass::Warrior => warrior_abilities(),
        }
    }
}
