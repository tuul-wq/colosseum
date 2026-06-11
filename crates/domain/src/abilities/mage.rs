use crate::abilities::general::{main_attack, offhand_attack};
use crate::abilities::{Ability, AbilityEffect, AbilityId, AbilityTarget};
use crate::position::Position;

pub fn mage_main_attack(damage: u8) -> Ability {
    Ability {
        name: "Mage wand attack".into(),
        ..main_attack(damage, Position::back(), Position::all())
    }
}

pub fn mage_offhand_attack(damage: u8) -> Ability {
    Ability {
        name: "Mage knife attack".into(),
        ..offhand_attack(damage, Position::front(), Position::front())
    }
}

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
        effect_type: AbilityEffect::Damage(6),
        positions_from: Position::back(),
        positions_to: Position::front(),
    }
}
