use domain::{Hero, HeroClass};

use crate::{DesicionContext, HeroAI, MainAction, SecondaryAction, TurnPlan};

pub struct WarriorAI;

impl HeroAI for WarriorAI {
    fn supports(&self, hero: &Hero) -> bool {
        matches!(hero.class, HeroClass::Warrior(_))
    }

    fn decide_turn(&self, ctx: &DesicionContext) -> TurnPlan {
        TurnPlan {
            main: MainAction::Skip,
            secondary: SecondaryAction::Skip,
        }
    }
}
