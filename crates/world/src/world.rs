use domain::{HeroId, Position};

use crate::{WorldError, formation::Formation};

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

impl World {
    pub fn new() -> Self {
        Self {
            left: Formation::new(),
            right: Formation::new(),
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

    pub fn all_heroes(&self, side: Side) -> Vec<HeroId> {
        self.formation(side).all_heroes()
    }

    pub fn position_of(&self, side: Side, hero_id: HeroId) -> Option<Position> {
        self.formation(side).position_of(hero_id)
    }

    pub fn hero_at(&self, side: Side, position: Position) -> Option<HeroId> {
        self.formation(side).hero_at(position)
    }

    pub fn place(
        &mut self,
        side: Side,
        hero_id: HeroId,
        position: Position,
    ) -> Result<(), WorldError> {
        self.formation_mut(side).place(hero_id, position)
    }

    pub fn remove(&mut self, side: Side, hero_id: HeroId) -> Result<(), WorldError> {
        self.formation_mut(side).remove(hero_id)
    }

    pub fn move_to(
        &mut self,
        side: Side,
        hero_id: HeroId,
        new_position: Position,
    ) -> Result<(), WorldError> {
        self.formation_mut(side).move_to(hero_id, new_position)
    }

    pub fn swap_with(
        &mut self,
        side: Side,
        first_hero_id: HeroId,
        second_hero_id: HeroId,
    ) -> Result<(), WorldError> {
        self.formation_mut(side)
            .swap_with(first_hero_id, second_hero_id)
    }
}

#[cfg(test)]
mod tests {
    use domain::{Hero, HeroId, Position};

    use super::*;

    fn hero_id(name: &str) -> HeroId {
        Hero::warrior(name.into()).id
    }

    #[test]
    fn new_creates_empty_formations_for_both_sides() {
        let world = World::new();

        for position in Position::all() {
            assert_eq!(world.hero_at(Side::Left, position), None);
            assert_eq!(world.hero_at(Side::Right, position), None);
        }
    }

    #[test]
    fn place_stores_hero_on_selected_side_only() {
        let mut world = World::new();
        let hero_id = hero_id("Warrior");

        let result = world.place(Side::Left, hero_id, Position::Frontline);

        assert!(result.is_ok());
        assert_eq!(
            world.hero_at(Side::Left, Position::Frontline),
            Some(hero_id)
        );
        assert_eq!(
            world.position_of(Side::Left, hero_id),
            Some(Position::Frontline)
        );
        assert_eq!(world.hero_at(Side::Right, Position::Frontline), None);
        assert_eq!(world.position_of(Side::Right, hero_id), None);
    }

    #[test]
    fn remove_clears_hero_from_selected_side_only() {
        let mut world = World::new();
        let left_hero_id = hero_id("Left");
        let right_hero_id = hero_id("Right");

        world
            .place(Side::Left, left_hero_id, Position::Frontline)
            .expect("left placement should succeed");
        world
            .place(Side::Right, right_hero_id, Position::Frontline)
            .expect("right placement should succeed");

        let result = world.remove(Side::Left, left_hero_id);

        assert!(result.is_ok());
        assert_eq!(world.hero_at(Side::Left, Position::Frontline), None);
        assert_eq!(world.position_of(Side::Left, left_hero_id), None);
        assert_eq!(
            world.hero_at(Side::Right, Position::Frontline),
            Some(right_hero_id)
        );
        assert_eq!(
            world.position_of(Side::Right, right_hero_id),
            Some(Position::Frontline)
        );
    }

    #[test]
    fn move_to_moves_hero_on_selected_side_only() {
        let mut world = World::new();
        let left_hero_id = hero_id("Left");
        let right_hero_id = hero_id("Right");

        world
            .place(Side::Left, left_hero_id, Position::Frontline)
            .expect("left placement should succeed");
        world
            .place(Side::Right, right_hero_id, Position::Frontline)
            .expect("right placement should succeed");

        let result = world.move_to(Side::Left, left_hero_id, Position::Backline);

        assert!(result.is_ok());
        assert_eq!(world.hero_at(Side::Left, Position::Frontline), None);
        assert_eq!(
            world.hero_at(Side::Left, Position::Backline),
            Some(left_hero_id)
        );
        assert_eq!(
            world.position_of(Side::Left, left_hero_id),
            Some(Position::Backline)
        );
        assert_eq!(
            world.hero_at(Side::Right, Position::Frontline),
            Some(right_hero_id)
        );
        assert_eq!(world.hero_at(Side::Right, Position::Backline), None);
    }

    #[test]
    fn swap_with_swaps_heroes_on_selected_side() {
        let mut world = World::new();
        let first_hero_id = hero_id("First");
        let second_hero_id = hero_id("Second");
        let right_hero_id = hero_id("Right");

        world
            .place(Side::Left, first_hero_id, Position::Frontline)
            .expect("first placement should succeed");
        world
            .place(Side::Left, second_hero_id, Position::Backline)
            .expect("second placement should succeed");
        world
            .place(Side::Right, right_hero_id, Position::Frontline)
            .expect("right placement should succeed");

        let result = world.swap_with(Side::Left, first_hero_id, second_hero_id);

        assert!(result.is_ok());
        assert_eq!(
            world.hero_at(Side::Left, Position::Frontline),
            Some(second_hero_id)
        );
        assert_eq!(
            world.hero_at(Side::Left, Position::Backline),
            Some(first_hero_id)
        );
        assert_eq!(
            world.position_of(Side::Left, first_hero_id),
            Some(Position::Backline)
        );
        assert_eq!(
            world.position_of(Side::Left, second_hero_id),
            Some(Position::Frontline)
        );
        assert_eq!(
            world.hero_at(Side::Right, Position::Frontline),
            Some(right_hero_id)
        );
    }

    #[test]
    fn failed_place_preserves_existing_world_state() {
        let mut world = World::new();
        let first_hero_id = hero_id("First");
        let second_hero_id = hero_id("Second");

        world
            .place(Side::Left, first_hero_id, Position::Frontline)
            .expect("initial placement should succeed");

        let result = world.place(Side::Left, second_hero_id, Position::Frontline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(
            world.hero_at(Side::Left, Position::Frontline),
            Some(first_hero_id)
        );
        assert_eq!(world.position_of(Side::Left, second_hero_id), None);
        assert_eq!(world.hero_at(Side::Right, Position::Frontline), None);
    }
}
