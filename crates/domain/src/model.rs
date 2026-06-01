use uuid::Uuid;

use crate::class::HeroClass;

pub type HeroID = Uuid;

#[derive(Debug)]
pub struct Hero {
    pub id: HeroID,
    pub stats: Stats,
    pub class: HeroClass,
    pub weapon: Weapon,
    pub position: Position,
}

#[derive(Debug)]
pub struct Stats {
    /// The name of the hero.
    pub name: String,
    /// The health of the hero, in hit points.
    pub health: Health,
    /// The initiative of the hero, determines turn order.
    pub initiative: u8,
    /// The speed of the hero, in cells per turn.
    pub speed: u8,
}

#[derive(Debug)]
pub struct Health {
    /// The maximum health of the hero, in hit points.
    pub max: u8,
    /// The current health of the hero, in hit points.
    pub current: u8,
}

#[derive(Debug)]
pub struct Weapon {
    /// The damage of the weapon, in hit points.
    pub damage: u8,
    /// The range of the weapon, in cells.
    pub range: u8,
}

#[derive(Debug)]
pub struct Position {
    /// The x-coordinate of the position.
    pub x: u8,
    /// The y-coordinate of the position.
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
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
        let mut hero = Hero::warrior("Warrior".to_string());

        hero.move_to(Position::new(2, 3));

        assert_eq!(hero.position.x, 2);
        assert_eq!(hero.position.y, 3);
    }

    #[test]
    fn hero_takes_damage() {
        let mut hero = Hero::warrior("Warrior".to_string());

        hero.take_damage(25);
        assert_eq!(hero.stats.health.current, 75);

        hero.take_damage(95);
        assert_eq!(hero.stats.health.current, 0);
    }

    #[test]
    fn hero_is_alive_when_health_is_above_zero() {
        let hero = Hero::warrior("Warrior".to_string());

        assert!(hero.is_alive());
    }

    #[test]
    fn hero_is_not_alive_when_health_is_zero() {
        let mut hero = Hero::warrior("Warrior".to_string());

        hero.take_damage(100);
        assert!(!hero.is_alive());
    }
}
