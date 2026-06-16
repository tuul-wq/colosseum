use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize);

impl Position {
    pub const FRONTLINE: Self = Self(0);
    pub const MIDLINE: Self = Self(1);
    pub const BACKLINE: Self = Self(2);

    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(self) -> usize {
        self.0
    }

    /// Returns Frontline, Midline, and Backline positions.
    pub fn all() -> HashSet<Self> {
        HashSet::from([Self::FRONTLINE, Self::MIDLINE, Self::BACKLINE])
    }

    pub fn range(count: usize) -> impl Iterator<Item = Self> {
        (0..count).map(Self::new)
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
