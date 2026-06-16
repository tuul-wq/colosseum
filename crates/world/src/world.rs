use domain::{HeroId, Position};

use crate::errors::WorldError;
use crate::formation::{Formation, Lineup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    left: Formation,
    right: Formation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn other_side(side: Side) -> Side {
        match side {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl World {
    pub fn new(left_team: Lineup, right_team: Lineup) -> Self {
        Self {
            left: Formation::new(left_team),
            right: Formation::new(right_team),
        }
    }

    fn formation(&self, side: Side) -> &Formation {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    fn formation_mut(&mut self, side: Side) -> &mut Formation {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    pub fn all_heroes(&self, side: Side) -> Vec<&HeroId> {
        self.formation(side).all_heroes()
    }

    pub fn position_of(&self, side: Side, hero_id: &HeroId) -> Option<Position> {
        self.formation(side).position_of(hero_id)
    }

    pub fn hero_at(&self, side: Side, position: Position) -> Option<&HeroId> {
        self.formation(side).hero_at(position)
    }

    pub fn place(
        &mut self,
        side: Side,
        hero_id: &HeroId,
        position: Position,
    ) -> Result<(), WorldError> {
        self.formation_mut(side).place(hero_id, position)
    }

    pub fn remove(&mut self, side: Side, hero_id: &HeroId) -> Result<(), WorldError> {
        self.formation_mut(side).remove(hero_id)
    }

    pub fn move_to(
        &mut self,
        side: Side,
        hero_id: &HeroId,
        new_position: Position,
    ) -> Result<(), WorldError> {
        self.formation_mut(side).move_to(hero_id, new_position)
    }

    pub fn swap_with(
        &mut self,
        side: Side,
        first_hero_id: &HeroId,
        second_hero_id: &HeroId,
    ) -> Result<(), WorldError> {
        self.formation_mut(side)
            .swap_with(first_hero_id, second_hero_id)
    }
}

#[cfg(test)]
mod tests {
    use domain::{HeroId, Position};

    use super::*;

    fn hero_id(name: &str) -> HeroId {
        HeroId::new(name)
    }

    fn lineup(first: &str, second: &str) -> Lineup {
        Lineup::with_position_count(vec![hero_id(first), hero_id(second)], Position::all().len())
    }

    fn world() -> World {
        World::new(
            lineup("LeftFront", "LeftMid"),
            lineup("RightFront", "RightMid"),
        )
    }

    #[test]
    fn new_creates_formations_for_both_sides() {
        let world = world();
        let left_front_hero_id = hero_id("LeftFront");
        let left_mid_hero_id = hero_id("LeftMid");
        let right_front_hero_id = hero_id("RightFront");
        let right_mid_hero_id = hero_id("RightMid");

        assert_eq!(
            world.hero_at(Side::Left, Position::FRONTLINE),
            Some(&left_front_hero_id)
        );
        assert_eq!(
            world.hero_at(Side::Left, Position::MIDLINE),
            Some(&left_mid_hero_id)
        );
        assert_eq!(world.hero_at(Side::Left, Position::BACKLINE), None);
        assert_eq!(
            world.hero_at(Side::Right, Position::FRONTLINE),
            Some(&right_front_hero_id)
        );
        assert_eq!(
            world.hero_at(Side::Right, Position::MIDLINE),
            Some(&right_mid_hero_id)
        );
        assert_eq!(world.hero_at(Side::Right, Position::BACKLINE), None);
    }

    #[test]
    fn place_stores_hero_on_selected_side_only() {
        let mut world = world();
        let hero_id = hero_id("LeftBack");

        let result = world.place(Side::Left, &hero_id, Position::BACKLINE);

        assert!(result.is_ok());
        assert_eq!(
            world.hero_at(Side::Left, Position::BACKLINE),
            Some(&hero_id)
        );
        assert_eq!(
            world.position_of(Side::Left, &hero_id),
            Some(Position::BACKLINE)
        );
        assert_eq!(world.hero_at(Side::Right, Position::BACKLINE), None);
        assert_eq!(world.position_of(Side::Right, &hero_id), None);
    }

    #[test]
    fn remove_clears_hero_from_selected_side_only() {
        let mut world = world();
        let left_hero_id = hero_id("LeftMid");
        let right_hero_id = hero_id("RightMid");

        let result = world.remove(Side::Left, &left_hero_id);

        assert!(result.is_ok());
        assert_eq!(world.hero_at(Side::Left, Position::MIDLINE), None);
        assert_eq!(world.position_of(Side::Left, &left_hero_id), None);
        assert_eq!(
            world.hero_at(Side::Right, Position::MIDLINE),
            Some(&right_hero_id)
        );
        assert_eq!(
            world.position_of(Side::Right, &right_hero_id),
            Some(Position::MIDLINE)
        );
    }

    #[test]
    fn move_to_moves_hero_on_selected_side_only() {
        let mut world = world();
        let left_hero_id = hero_id("LeftMid");
        let right_hero_id = hero_id("RightMid");

        let result = world.move_to(Side::Left, &left_hero_id, Position::BACKLINE);

        assert!(result.is_ok());
        assert_eq!(world.hero_at(Side::Left, Position::MIDLINE), None);
        assert_eq!(
            world.hero_at(Side::Left, Position::BACKLINE),
            Some(&left_hero_id)
        );
        assert_eq!(
            world.position_of(Side::Left, &left_hero_id),
            Some(Position::BACKLINE)
        );
        assert_eq!(
            world.hero_at(Side::Right, Position::MIDLINE),
            Some(&right_hero_id)
        );
        assert_eq!(world.hero_at(Side::Right, Position::BACKLINE), None);
    }

    #[test]
    fn swap_with_swaps_heroes_on_selected_side() {
        let mut world = world();
        let first_hero_id = hero_id("LeftFront");
        let second_hero_id = hero_id("LeftMid");
        let right_hero_id = hero_id("RightFront");

        let result = world.swap_with(Side::Left, &first_hero_id, &second_hero_id);

        assert!(result.is_ok());
        assert_eq!(
            world.hero_at(Side::Left, Position::FRONTLINE),
            Some(&second_hero_id)
        );
        assert_eq!(
            world.hero_at(Side::Left, Position::MIDLINE),
            Some(&first_hero_id)
        );
        assert_eq!(
            world.position_of(Side::Left, &first_hero_id),
            Some(Position::MIDLINE)
        );
        assert_eq!(
            world.position_of(Side::Left, &second_hero_id),
            Some(Position::FRONTLINE)
        );
        assert_eq!(
            world.hero_at(Side::Right, Position::FRONTLINE),
            Some(&right_hero_id)
        );
    }

    #[test]
    fn failed_place_preserves_existing_world_state() {
        let mut world = world();
        let first_hero_id = hero_id("LeftFront");
        let second_hero_id = hero_id("Second");

        let result = world.place(Side::Left, &second_hero_id, Position::FRONTLINE);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(
            world.hero_at(Side::Left, Position::FRONTLINE),
            Some(&first_hero_id)
        );
        assert_eq!(world.position_of(Side::Left, &second_hero_id), None);
        assert_eq!(
            world.hero_at(Side::Right, Position::FRONTLINE),
            Some(&hero_id("RightFront"))
        );
    }
}
