use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Frontline,
    Midline,
    Backline,
}

impl Position {
    pub const FRONTLINE: Self = Self::Frontline;
    pub const MIDLINE: Self = Self::Midline;
    pub const BACKLINE: Self = Self::Backline;

    pub fn index(self) -> usize {
        match self {
            Self::Frontline => 0,
            Self::Midline => 1,
            Self::Backline => 2,
        }
    }

    pub fn ordered() -> [Self; 3] {
        [Self::Frontline, Self::Midline, Self::Backline]
    }

    /// Returns Frontline, Midline, and Backline positions.
    pub fn all() -> HashSet<Self> {
        HashSet::from(Self::ordered())
    }

    /// Returns Frontline and Midline positions.
    pub fn front() -> HashSet<Self> {
        HashSet::from([Self::FRONTLINE, Self::MIDLINE])
    }

    /// Returns Midline position.
    pub fn mid() -> HashSet<Self> {
        HashSet::from([Self::MIDLINE])
    }

    /// Returns Midline and Backline positions.
    pub fn back() -> HashSet<Self> {
        HashSet::from([Self::MIDLINE, Self::BACKLINE])
    }
}
