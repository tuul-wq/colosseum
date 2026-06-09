use std::collections::HashMap;

use domain::{Hero, HeroId};
use world::{Side, World};

pub struct DecisionContext<'a> {
    pub actor: &'a Hero,
    pub world: &'a World,
    pub side: Side,
    pub targets: HashMap<HeroId, &'a Hero>,
}

impl DecisionContext<'_> {
    pub fn get_allies(&self, side: Side) -> HashMap<HeroId, &Hero> {
        self.world
            .all_heroes(side)
            .iter()
            .filter_map(|id| self.targets.get(id).map(|&hero| (*id, hero)))
            .collect::<HashMap<_, _>>()
    }

    pub fn get_enemies(&self, side: Side) -> HashMap<HeroId, &Hero> {
        self.world
            .all_heroes(Side::other_side(side))
            .iter()
            .filter_map(|id| self.targets.get(id).map(|&hero| (*id, hero)))
            .collect::<HashMap<_, _>>()
    }
}

// TODO: think about params of each enum variant
pub enum TurnAction {
    /// Use special ability.
    Ability,
    /// Move inside formation
    Move,
    /// Swap with ally hero
    Swap,
    /// Tiny healing action
    Bandage,
}

pub trait HeroAi {
    fn supports(&self, hero: &Hero) -> bool;

    fn decide_turn(&self, ctx: &DecisionContext) -> TurnAction;
}
