use domain::{Hero, HeroID, Position};
use world::World;

pub enum MainAction {
    Attack(HeroID),
    Spell(HeroID),
}

pub enum SecondaryAction {
    MoveTo(Position),
    Wait,
}

pub trait HeroAI {
    // TODO: enemies, allies (in future)
    fn next_main_action(&self, targets: Vec<&Hero>, world: World) -> MainAction;

    // TODO: enemies, allies (in future)
    fn next_secondary_action(&self, targets: Vec<&Hero>, world: World) -> SecondaryAction;
}
