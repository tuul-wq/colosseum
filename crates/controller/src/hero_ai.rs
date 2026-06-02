use domain::{Hero, HeroID, Position};
use world::World;

pub struct DesicionContext<'a> {
    pub actor: &'a Hero,
    pub world: &'a World,
    pub targets: Vec<&'a Hero>,
}

pub struct TurnPlan {
    pub main: MainAction,
    pub secondary: SecondaryAction,
}

pub enum MainAction {
    Attack(HeroID),
    Spell(HeroID),
    Skip,
}

pub enum SecondaryAction {
    MoveTo(Position),
    Skip,
}

pub trait HeroAI {
    fn supports(&self, hero: &Hero) -> bool;

    fn decide_turn(&self, ctx: &DesicionContext) -> TurnPlan;
}
