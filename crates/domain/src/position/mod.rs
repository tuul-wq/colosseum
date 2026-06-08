use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Frontline,
    Midline,
    Backline,
    Rearguard,
}

impl Position {
    /// Returns Frontline, Midline, Backline, and Rearguard positions.
    pub fn all() -> HashSet<Self> {
        HashSet::from([
            Self::Frontline,
            Self::Midline,
            Self::Backline,
            Self::Rearguard,
        ])
    }

    /// Returns Frontline and Midline positions.
    pub fn front() -> HashSet<Self> {
        HashSet::from([Self::Frontline, Self::Midline])
    }

    /// Returns Midline and Backline positions.
    pub fn mid() -> HashSet<Self> {
        HashSet::from([Self::Midline, Self::Backline])
    }

    /// Returns Backline and Rearguard positions.
    pub fn back() -> HashSet<Self> {
        HashSet::from([Self::Backline, Self::Rearguard])
    }
}
