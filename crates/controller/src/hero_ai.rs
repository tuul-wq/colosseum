use domain::{Hero, HeroID, Position};
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

pub enum Action {
    /// Basic weapon attack against a target hero.
    Attack(HeroID),
    /// Class spell or special ability against a target hero.
    Spell(HeroID),
    /// Move to a target arena position.
    Move(Position),
    /// Skip turn
    Skip,
}

pub enum MainAction {
    /// Basic weapon attack. Range is defined by the actor's weapon.
    Attack(HeroID),
    /// Class spell or special ability.
    ///
    /// Spell definitions should decide whether the spell is ranged,
    /// melee-safe, self-targeted, or otherwise usable while engaged.
    Spell(HeroID),
    Skip,
}

pub enum SecondaryAction {
    /// Move to a target arena position, subject to speed, collision, and
    /// engagement rules.
    MoveTo(Position),
    Skip,
}

pub trait HeroAI {
    fn supports(&self, hero: &Hero) -> bool;

    fn decide_turn(&self, ctx: &DesicionContext) -> TurnPlan;
}
