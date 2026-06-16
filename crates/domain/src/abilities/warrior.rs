use std::collections::HashSet;

use crate::abilities::general::{main_attack, offhand_attack};
use crate::abilities::{Ability, AbilityEffect, AbilityId, AbilityTarget};
use crate::position::Position;

pub fn warrior_main_attack(damage: u8) -> Ability {
    Ability {
        name: "Warrior club attack".into(),
        ..main_attack(
            damage,
            HashSet::from([Position::FRONTLINE]),
            Position::front(),
        )
    }
}

pub fn warrior_offhand_attack(damage: u8) -> Ability {
    Ability {
        name: "Warrior gun shot".into(),
        ..offhand_attack(damage, Position::mid(), Position::all())
    }
}

pub fn slam() -> Ability {
    Ability {
        id: AbilityId::Slam,
        name: "Slam".into(),
        target_type: AbilityTarget::Enemy,
        effect_type: AbilityEffect::Damage(13),
        positions_from: HashSet::from([Position::FRONTLINE]),
        positions_to: Position::front(),
    }
}

pub fn whirlwind() -> Ability {
    Ability {
        id: AbilityId::Whirlwind,
        name: "Whirlwind".into(),
        target_type: AbilityTarget::AreaOfEffect,
        effect_type: AbilityEffect::Damage(6),
        positions_from: HashSet::from([Position::FRONTLINE]),
        positions_to: Position::front(),
    }
}
