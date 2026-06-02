use uuid::Uuid;

use crate::{
    Health, Hero, Position, Stats,
    class::HeroClass,
    weapon::{MainHandWeapon, OffHandWeapon, WeaponRangeType},
};

#[derive(Debug)]
pub struct WarriorClass {
    /// Rage used for future special melee abilities.
    pub rage: Rage,
}

#[derive(Debug)]
pub struct Rage {
    pub max: u8,
    pub current: u8,
}

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
            class: HeroClass::Warrior(WarriorClass {
                rage: Rage {
                    max: 100,
                    current: 0,
                },
            }),
            position: Position::new(0, 0),
            main_weapon: MainHandWeapon::new(10, WeaponRangeType::Melee),
            offhand_weapon: OffHandWeapon::new(3, WeaponRangeType::Melee),
        }
    }
}
