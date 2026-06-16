use domain::HeroClass;

use crate::lineup::ArenaLineup;

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

impl ArenaSetup {
    pub fn into_lineup(self) -> ArenaLineup {
        match self {
            Self::TwoVsTwo { left, right } => ArenaLineup::new(
                TeamSetup::new(Layout::Two, left),
                TeamSetup::new(Layout::Two, right),
            ),
            Self::ThreeVsThree { left, right } => ArenaLineup::new(
                TeamSetup::new(Layout::Three, left),
                TeamSetup::new(Layout::Three, right),
            ),
        }
    }
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
    layout: Layout,
    classes: [HeroClass; N],
}

impl<const N: usize> TeamSetup<N> {
    fn new(layout: Layout, classes: [HeroClass; N]) -> Self {
        Self { layout, classes }
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }

    pub fn classes(self) -> [HeroClass; N] {
        self.classes
    }
}
