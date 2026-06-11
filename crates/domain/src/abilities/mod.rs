use std::collections::HashSet;

pub mod general;
pub mod mage;
pub mod warrior;

use crate::position::Position;

#[derive(Debug)]
pub struct Ability {
    pub id: AbilityId,
    pub name: String,
    pub target_type: AbilityTarget,
    pub effect_type: AbilityEffect,
    pub positions_from: HashSet<Position>,
    pub positions_to: HashSet<Position>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityId {
    // General
    MainAttack,
    OffhandAttack,

    // Mage
    Fireball,
    ArcaneExplosion,

    // Warrior
    Slam,
    Whirlwind,
}

#[derive(Debug)]
pub enum AbilityTarget {
    SelfTarget,
    Enemy,
    Ally,
    AreaOfEffect,
}

#[derive(Debug)]
pub enum AbilityEffect {
    Damage(u8),
    Heal(u8),
    DamageReduction { amount: u8, duration_turns: u8 },
    // Buff,
    // Debuff,
}
