use domain::{Hero, HeroId, Position};
use world::World;

pub struct DesicionContext<'a> {
    pub actor: &'a Hero,
    pub world: &'a World,
    pub targets: Vec<&'a Hero>,
}

pub struct TurnPlan {
    /// Main combat choice for the turn: basic attack, spell, or skip.
    ///
    /// The arena should resolve movement before this action. If the actor starts
    /// adjacent to any living enemy, the actor is engaged: ranged attacks and
    /// ranged targeted spells are unavailable against every target, not just the
    /// adjacent enemy. Engaged actors may still use melee attacks, melee-safe
    /// spells, self spells, or choose to retreat.
    pub main: MainAction,
    /// Movement choice for the turn.
    ///
    /// Movement is resolved before the main action. Moving away from engagement
    /// should consume/prevent the main action, so ranged classes cannot retreat
    /// and attack in the same turn after melee has reached them.
    pub secondary: SecondaryAction,
}

pub enum MainAction {
    /// Basic weapon attack against a target hero.
    Attack(HeroId),
    /// Use special ability.
    Ability(HeroId),
    /// Disengage from adjacent enemies, enables move afterwards.
    Disengage,
    /// Skip action
    Skip,
}

pub enum SecondaryAction {
    /// Move to a target arena position. If engaged must Disengage first.
    MoveTo(Position),
    /// Skip action
    Skip,
}

pub trait HeroAI {
    fn supports(&self, hero: &Hero) -> bool;

    fn decide_turn(&self, ctx: &DesicionContext) -> TurnPlan;
}
