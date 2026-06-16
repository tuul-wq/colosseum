use domain::HeroClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaSetup {
    TwoVsTwo {
        left: [HeroClass; 2],
        right: [HeroClass; 2],
    },
    ThreeVsThree {
        left: [HeroClass; 3],
        right: [HeroClass; 3],
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Two,
    Three,
}

impl Layout {
    pub fn position_count(self) -> usize {
        match self {
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}

pub struct TeamSetup<const N: usize> {
    pub layout: Layout,
    pub classes: [HeroClass; N],
}

impl<const N: usize> TeamSetup<N> {
    pub fn new(layout: Layout, classes: [HeroClass; N]) -> Self {
        Self { layout, classes }
    }
}
