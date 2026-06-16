mod class;
mod mage;
mod warrior;

pub use crate::heroes::class::HeroClass;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeroId(String);

impl HeroId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub struct Hero {
    pub id: HeroId,
    pub name: String,
    pub health: Health,
    pub initiative: u8,
    pub class: HeroClass,
}

#[derive(Debug)]
pub struct Health {
    pub max: u8,
    pub current: u8,
}

impl Hero {
    pub fn new(class: HeroClass) -> Self {
        match class {
            HeroClass::Mage => Self::mage(),
            HeroClass::Warrior => Self::warrior(),
        }
    }

    pub fn take_damage(&mut self, damage: u8) {
        self.health.current = self.health.current.saturating_sub(damage);
    }

    pub fn is_alive(&self) -> bool {
        self.health.current > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_takes_damage() {
        let mut hero = Hero::warrior();

        hero.take_damage(25);
        assert_eq!(hero.health.current, hero.health.max - 25);

        hero.take_damage(95);
        assert_eq!(hero.health.current, 0);
    }

    #[test]
    fn hero_is_alive_when_health_is_above_zero() {
        let hero = Hero::warrior();

        assert!(hero.is_alive());
    }

    #[test]
    fn hero_is_not_alive_when_health_is_zero() {
        let mut hero = Hero::warrior();

        hero.take_damage(hero.health.max);
        assert!(!hero.is_alive());
    }
}
