use crate::{Ability, AbilityEffect, AbilityId, AbilityTarget, Position};

pub fn fireball() -> Ability {
    Ability {
        id: AbilityId::Fireball,
        name: "Fireball".into(),
        target_type: AbilityTarget::Enemy,
        effect_type: AbilityEffect::Damage(15),
        positions_from: Position::back(),
        positions_to: Position::all(),
    }
}

pub fn arcane_explosion() -> Ability {
    Ability {
        id: AbilityId::ArcaneExplosion,
        name: "Arcane Explosion".into(),
        target_type: AbilityTarget::AreaOfEffect,
        effect_type: AbilityEffect::Damage(8),
        positions_from: Position::back(),
        positions_to: Position::front(),
    }
}
