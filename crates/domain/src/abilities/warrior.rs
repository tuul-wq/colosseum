use crate::{Ability, AbilityEffect, AbilityId, AbilityTarget, Position};

pub fn slam() -> Ability {
    Ability {
        id: AbilityId::Slam,
        name: "Slam".into(),
        target_type: AbilityTarget::Enemy,
        effect_type: AbilityEffect::Damage(15),
        positions_from: Position::back(),
        positions_to: Position::all(),
    }
}

pub fn whirlwind() -> Ability {
    Ability {
        id: AbilityId::Whirlwind,
        name: "Whirlwind".into(),
        target_type: AbilityTarget::AreaOfEffect,
        effect_type: AbilityEffect::Damage(8),
        positions_from: Position::back(),
        positions_to: Position::front(),
    }
}
