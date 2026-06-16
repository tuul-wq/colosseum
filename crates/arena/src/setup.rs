use domain::HeroClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArenaSetup {
    pub left: [HeroClass; 3],
    pub right: [HeroClass; 3],
}
