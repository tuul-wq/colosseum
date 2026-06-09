use std::collections::HashMap;

use domain::{Hero, HeroId, Position};
use world::{Side, World};

pub struct DesicionContext<'a> {
    pub actor: &'a Hero,
    pub world: &'a World, // already contains formations
    pub side: Side,
    pub targets: HashMap<HeroId, &'a Hero>,
}

impl DesicionContext<'_> {
    pub fn get_allies(&self) -> HashMap<HeroId, &Hero> {
        let mut allies = HashMap::new();

        for id in self.world.all_heroes(self.side) {
            if let Some(&hero) = self.targets.get(&id) {
                allies.insert(id, hero);
            }
        }

        allies
    }

    pub fn get_enemies(&self, side: Side) -> HashMap<HeroId, &Hero> {
        self.world
            .all_heroes(side)
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

pub trait HeroAI {
    fn supports(&self, hero: &Hero) -> bool;

    fn decide_turn(&self, ctx: &DesicionContext) -> TurnAction;
}
