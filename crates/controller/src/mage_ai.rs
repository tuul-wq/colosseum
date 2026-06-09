use domain::{Hero, heroes::class::HeroClass};

use crate::hero_ai::{DecisionContext, HeroAi, TurnAction};

pub struct MageAI;

impl HeroAi for MageAI {
    fn supports(&self, hero: &Hero) -> bool {
        matches!(hero.class, HeroClass::Mage)
    }

    fn decide_turn(&self, ctx: &DecisionContext) -> TurnAction {}
}
