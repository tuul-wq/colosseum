use std::collections::HashMap;

use domain::{Hero, HeroId};
use world::World;

pub struct Arena {
    world: World,
    heroes: HashMap<HeroId, Hero>,
}
