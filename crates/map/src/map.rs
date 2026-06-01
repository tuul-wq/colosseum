use domain::{HeroID, Position};

pub struct Map {
    /// The map data, stored as a flat vector of cells.
    data: Vec<Cell>,
    /// The width of the map (X axis).
    width: u8,
    /// The height of the map (Y axis).
    height: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Player(HeroID),
    Blocked,
    Empty,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Self {
        let size = usize::from(width) * usize::from(height);

        Self {
            data: (0..size).map(|_| Cell::Empty).collect(),
            width,
            height,
        }
    }

    pub fn get(&self, position: Position) -> Option<&Cell> {
        let index = self.index(position)?;

        self.data.get(index)
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut Cell> {
        let index = self.index(position)?;

        self.data.get_mut(index)
    }

    fn index(&self, position: Position) -> Option<usize> {
        if position.x >= self.width || position.y >= self.height {
            return None;
        }

        Some(usize::from(position.y) * usize::from(self.width) + usize::from(position.x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_index() {
        let map = Map::new(4, 4);

        assert_eq!(map.index(Position::new(8, 8)), None);
        assert_eq!(map.index(Position::new(0, 0)), Some(0));
        assert_eq!(map.index(Position::new(1, 0)), Some(1));
        assert_eq!(map.index(Position::new(0, 1)), Some(4));
        assert_eq!(map.index(Position::new(3, 3)), Some(15));
    }

    #[test]
    fn test_map_get_returns_cell_for_valid_position() {
        let map = Map::new(4, 4);

        assert_eq!(map.get(Position::new(0, 0)), Some(&Cell::Empty));
        assert_eq!(map.get(Position::new(3, 3)), Some(&Cell::Empty));
    }

    #[test]
    fn test_map_get_returns_none_for_out_of_bounds_position() {
        let map = Map::new(4, 4);

        assert_eq!(map.get(Position::new(4, 0)), None);
        assert_eq!(map.get(Position::new(0, 4)), None);
        assert_eq!(map.get(Position::new(8, 8)), None);
    }

    #[test]
    fn test_map_get_mut_updates_cell() {
        let mut map = Map::new(4, 4);

        assert_eq!(map.get(Position::new(2, 1)), Some(&Cell::Empty));

        let cell = map.get_mut(Position::new(2, 1)).unwrap();
        *cell = Cell::Blocked;

        assert_eq!(map.get(Position::new(2, 1)), Some(&Cell::Blocked));
    }

    #[test]
    fn test_map_get_mut_returns_none_for_out_of_bounds_position() {
        let mut map = Map::new(4, 4);

        assert_eq!(map.get_mut(Position::new(4, 0)), None);
        assert_eq!(map.get_mut(Position::new(0, 4)), None);
    }
}
