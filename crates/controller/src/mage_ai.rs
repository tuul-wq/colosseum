use domain::Hero;

use crate::{DesicionContext, HeroAI, MainAction, SecondaryAction, TurnPlan};

pub struct MageAI;

impl HeroAI for MageAI {
    fn supports(&self, hero: &Hero) -> bool {
        matches!(hero.class, HeroClass::Mage(_))
    }

    // - try avoid melee contact
    // - if low hp, avoid melee contact even more
    //
    // - use spell when enough mana
    // - if any target will die from spell, then move and cast
    // - use attack for nearest target
    fn decide_turn(&self, ctx: &DesicionContext) -> TurnPlan {
        // if !self.supports(ctx.actor) {
        //     println!(
        //         "MageAI does not support decide_turn for {:?}",
        //         ctx.actor.class
        //     );

        //     return TurnPlan {
        //         main: MainAction::Skip,
        //         secondary: SecondaryAction::Skip,
        //     };
        // }

        // TurnPlan {
        //     main: MainAction::Skip,
        //     secondary: SecondaryAction::Skip,
        // }
    }
}
