pub mod mage;
pub mod warrior;

use std::collections::HashMap;

use uuid::Uuid;

use crate::{Ability, AbilityId};

pub type HeroId = Uuid;

#[derive(Debug)]
pub struct Hero {
    pub id: HeroId,
    pub stats: Stats,
    pub abilities: HashMap<AbilityId, Ability>,
}

#[derive(Debug)]
pub struct Stats {
    pub name: String,
    pub health: Health,
    pub initiative: u8,
    pub speed: u8,
}

#[derive(Debug)]
pub struct Health {
    pub max: u8,
    pub current: u8,
}

impl Hero {
    pub fn take_damage(&mut self, damage: u8) {
        self.stats.health.current = self.stats.health.current.saturating_sub(damage);
    }

    pub fn is_alive(&self) -> bool {
        self.stats.health.current > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_takes_damage() {
        let mut hero = Hero::warrior("Warrior".into());

        hero.take_damage(25);
        assert_eq!(hero.stats.health.current, 75);

        hero.take_damage(95);
        assert_eq!(hero.stats.health.current, 0);
    }

    #[test]
    fn hero_is_alive_when_health_is_above_zero() {
        let hero = Hero::warrior("Warrior".into());

        assert!(hero.is_alive());
    }

    #[test]
    fn hero_is_not_alive_when_health_is_zero() {
        let mut hero = Hero::warrior("Warrior".into());

        hero.take_damage(100);
        assert!(!hero.is_alive());
    }
}
