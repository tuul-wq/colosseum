use domain::{Hero, HeroClass};

use crate::{DesicionContext, HeroAI, MainAction, SecondaryAction, TurnPlan};

pub struct MageAI;

impl HeroAI for MageAI {
    fn supports(&self, hero: &Hero) -> bool {
        matches!(hero.class, HeroClass::Mage(_))
    }

    fn decide_turn(&self, ctx: &DesicionContext) -> TurnPlan {
        TurnPlan {
            main: MainAction::Skip,
            secondary: SecondaryAction::Skip,
        }
    }
}
