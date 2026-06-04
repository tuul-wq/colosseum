pub mod mage;
pub mod warrior;

use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::{Ability, AbilityId};

pub type HeroId = Uuid;

#[derive(Debug)]
pub struct Hero {
    pub id: HeroId,
    pub stats: Stats,
    pub abilities: HashMap<AbilityId, Ability>,
    pub position: Position,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Frontline,
    Midline,
    Backline,
    Rearguard,
}

impl Position {
    pub fn all() -> HashSet<Self> {
        HashSet::from([
            Self::Frontline,
            Self::Midline,
            Self::Backline,
            Self::Rearguard,
        ])
    }

    pub fn front() -> HashSet<Self> {
        HashSet::from([Self::Frontline, Self::Midline])
    }

    pub fn mid() -> HashSet<Self> {
        HashSet::from([Self::Midline, Self::Backline])
    }

    pub fn back() -> HashSet<Self> {
        HashSet::from([Self::Backline, Self::Rearguard])
    }
}

impl Hero {
    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

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
    fn hero_moves_to_position() {
        let mut hero = Hero::warrior("Warrior".into(), Position::Frontline);

        hero.move_to(Position::Frontline);

        assert_eq!(hero.position, Position::Frontline);
    }

    #[test]
    fn hero_takes_damage() {
        let mut hero = Hero::warrior("Warrior".into(), Position::Frontline);

        hero.take_damage(25);
        assert_eq!(hero.stats.health.current, 75);

        hero.take_damage(95);
        assert_eq!(hero.stats.health.current, 0);
    }

    #[test]
    fn hero_is_alive_when_health_is_above_zero() {
        let hero = Hero::warrior("Warrior".into(), Position::Frontline);

        assert!(hero.is_alive());
    }

    #[test]
    fn hero_is_not_alive_when_health_is_zero() {
        let mut hero = Hero::warrior("Warrior".into(), Position::Frontline);

        hero.take_damage(100);
        assert!(!hero.is_alive());
    }
}
