pub mod hero_ai;
pub mod mage_ai;
// pub mod warrior_ai;

pub use hero_ai::{
    DecisionContext, HeroAi, ScoreWeights, ScoredAction, ScoringAi, TargetSelection, TurnAction,
};
pub use mage_ai::MageAI;
// pub use warrior_ai::*;
