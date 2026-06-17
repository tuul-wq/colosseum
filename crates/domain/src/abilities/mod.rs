use std::collections::{HashMap, HashSet};

mod general;
mod mage;
mod warrior;

use crate::position::Position;

pub fn mage_abilities() -> HashMap<AbilityId, Ability> {
    HashMap::from([
        (AbilityId::MainAttack, mage::mage_main_attack(5)),
        (AbilityId::OffhandAttack, mage::mage_offhand_attack(3)),
        (AbilityId::Fireball, mage::fireball()),
        (AbilityId::ArcaneExplosion, mage::arcane_explosion()),
    ])
}

pub fn warrior_abilities() -> HashMap<AbilityId, Ability> {
    HashMap::from([
        (AbilityId::MainAttack, warrior::warrior_main_attack(9)),
        (AbilityId::OffhandAttack, warrior::warrior_offhand_attack(5)),
        (AbilityId::Slam, warrior::slam()),
        (AbilityId::Whirlwind, warrior::whirlwind()),
    ])
}

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
    // DamageReduction { amount: u8, duration_turns: u8 },
    // Buff,
    // Debuff,
}
