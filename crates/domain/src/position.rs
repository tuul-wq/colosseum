use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Frontline,
    Midline,
    Backline,
}

impl Position {
    /// Amount of positions per side
    pub const COUNT: usize = 3;

    pub fn index(self) -> usize {
        match self {
            Self::Frontline => 0,
            Self::Midline => 1,
            Self::Backline => 2,
        }
    }

    pub fn ordered() -> [Self; Self::COUNT] {
        [Self::Frontline, Self::Midline, Self::Backline]
    }

    /// Returns Frontline, Midline, and Backline positions.
    pub fn all() -> HashSet<Self> {
        HashSet::from(Self::ordered())
    }

    /// Returns Frontline and Midline positions.
    pub fn front() -> HashSet<Self> {
        HashSet::from([Self::Frontline, Self::Midline])
    }

    /// Returns Midline position.
    pub fn mid() -> HashSet<Self> {
        HashSet::from([Self::Midline])
    }

    /// Returns Midline and Backline positions.
    pub fn back() -> HashSet<Self> {
        HashSet::from([Self::Midline, Self::Backline])
    }
}
