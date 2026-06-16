use domain::{HeroClass, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArenaSetup {
    pub left: [HeroClass; Position::COUNT],
    pub right: [HeroClass; Position::COUNT],
}
