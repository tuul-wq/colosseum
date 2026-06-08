use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Frontline,
    Midline,
    Backline,
    Rearguard,
}

impl Position {
    pub fn all() -> HashSet<Self> {
        HashSet::from([
            Self::Frontline,
            Self::Midline,
            Self::Backline,
            Self::Rearguard,
        ])
    }

    pub fn front() -> HashSet<Self> {
        HashSet::from([Self::Frontline, Self::Midline])
    }

    pub fn mid() -> HashSet<Self> {
        HashSet::from([Self::Midline, Self::Backline])
    }

    pub fn back() -> HashSet<Self> {
        HashSet::from([Self::Backline, Self::Rearguard])
    }
}
