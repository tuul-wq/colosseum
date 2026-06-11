use std::collections::HashSet;

use crate::abilities::{Ability, AbilityEffect, AbilityId, AbilityTarget};
use crate::position::Position;

pub fn main_attack(
    damage: u8,
    positions_from: HashSet<Position>,
    positions_to: HashSet<Position>,
) -> Ability {
    Ability {
        id: AbilityId::MainAttack,
        name: "Main Attack".into(),
        target_type: AbilityTarget::Enemy,
        effect_type: AbilityEffect::Damage(damage),
        positions_from,
        positions_to,
    }
}

pub fn offhand_attack(
    damage: u8,
    positions_from: HashSet<Position>,
    positions_to: HashSet<Position>,
) -> Ability {
    Ability {
        id: AbilityId::OffhandAttack,
        name: "Offhand Attack".into(),
        target_type: AbilityTarget::Enemy,
        effect_type: AbilityEffect::Damage(damage),
        positions_from,
        positions_to,
    }
}
