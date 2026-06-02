use uuid::Uuid;

use crate::{
    Health, Hero, Position, Stats,
    class::HeroClass,
    weapon::{MainHandWeapon, OffHandWeapon, WeaponRangeType},
};

#[derive(Debug)]
pub struct MageClass {
    /// Mana points used for spell casting.
    ///
    /// Mage spells should be modeled as class abilities that spend mana, not as
    /// the hero's basic weapon attack.
    pub mana: Mana,
}

#[derive(Debug)]
pub struct Mana {
    pub max: u8,
    pub current: u8,
}

impl Hero {
    pub fn mage(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            stats: Stats {
                name,
                health: Health {
                    max: 60,
                    current: 60,
                },
                initiative: 12,
                speed: 2,
            },
            class: HeroClass::Mage(MageClass {
                mana: Mana {
                    max: 100,
                    current: 100,
                },
            }),
            position: Position::new(0, 0),
            main_weapon: MainHandWeapon::new(5, WeaponRangeType::Ranged(2)),
            offhand_weapon: OffHandWeapon::new(1, WeaponRangeType::Melee),
        }
    }
}
