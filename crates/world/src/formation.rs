use std::collections::HashMap;

use domain::{HeroId, Position};

use crate::errors::WorldError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formation {
    slots: HashMap<Position, Option<HeroId>>,
}

impl Formation {
    pub fn new() -> Self {
        Self {
            slots: Position::all().into_iter().map(|pos| (pos, None)).collect(),
        }
    }

    pub fn all_heroes(&self) -> Vec<&HeroId> {
        self.slots.values().filter_map(|el| el.as_ref()).collect()
    }

    pub fn position_of(&self, hero_id: &HeroId) -> Option<Position> {
        self.slots
            .iter()
            .find_map(|(&pos, id)| (id.as_ref() == Some(hero_id)).then_some(pos))
    }

    pub fn hero_at(&self, position: Position) -> Option<&HeroId> {
        self.slots.get(&position).and_then(Option::as_ref)
    }

    pub fn place(&mut self, hero_id: &HeroId, position: Position) -> Result<(), WorldError> {
        if self.position_of(hero_id).is_some() {
            return Err(WorldError::PositionOccupied);
        }

        let slot = self
            .slots
            .get_mut(&position)
            .ok_or(WorldError::PositionNotFound)?;

        if slot.is_some() {
            return Err(WorldError::PositionOccupied);
        }

        *slot = Some(hero_id.clone());

        Ok(())
    }

    pub fn remove(&mut self, hero_id: &HeroId) -> Result<(), WorldError> {
        let pos = self.position_of(hero_id).ok_or(WorldError::HeroNotFound)?;

        self.slots
            .insert(pos, None)
            .ok_or(WorldError::PositionNotFound)?;

        self.compact_forward();

        Ok(())
    }

    pub fn move_to(&mut self, hero_id: &HeroId, new_position: Position) -> Result<(), WorldError> {
        let curr_pos = self.position_of(hero_id).ok_or(WorldError::HeroNotFound)?;

        if curr_pos == new_position {
            return Ok(());
        }

        if !self.slots.contains_key(&new_position) {
            return Err(WorldError::PositionNotFound);
        }

        if self.hero_at(new_position).is_some() {
            return Err(WorldError::PositionOccupied);
        }

        self.slots.insert(new_position, Some(hero_id.clone()));
        self.slots.insert(curr_pos, None);

        Ok(())
    }

    pub fn swap_with(
        &mut self,
        first_hero_id: &HeroId,
        second_hero_id: &HeroId,
    ) -> Result<(), WorldError> {
        let first_hero_pos = self
            .position_of(first_hero_id)
            .ok_or(WorldError::HeroNotFound)?;

        let second_hero_pos = self
            .position_of(second_hero_id)
            .ok_or(WorldError::HeroNotFound)?;

        self.slots
            .insert(first_hero_pos, Some(second_hero_id.clone()));
        self.slots
            .insert(second_hero_pos, Some(first_hero_id.clone()));

        Ok(())
    }

    fn compact_forward(&mut self) {
        let front_hero = self
            .slots
            .get_mut(&Position::Frontline)
            .and_then(Option::take);

        let mid_hero = self
            .slots
            .get_mut(&Position::Midline)
            .and_then(Option::take);

        let back_hero = self
            .slots
            .get_mut(&Position::Backline)
            .and_then(Option::take);

        let mut heroes = [front_hero, mid_hero, back_hero].into_iter().flatten();

        self.slots.insert(Position::Frontline, heroes.next());
        self.slots.insert(Position::Midline, heroes.next());
        self.slots.insert(Position::Backline, heroes.next());
    }
}

#[cfg(test)]
mod tests {
    use domain::{HeroId, Position};

    use super::*;

    #[test]
    fn new_creates_empty_slot_for_every_position() {
        let formation = Formation::new();

        for position in Position::all() {
            assert!(formation.slots.contains_key(&position));
            assert_eq!(formation.hero_at(position), None);
        }

        assert_eq!(formation.slots.len(), Position::all().len());
    }

    #[test]
    fn place_stores_hero_at_position() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Warrior");

        let result = formation.place(&hero_id, Position::Frontline);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::Frontline));
    }

    #[test]
    fn place_rejects_occupied_position_and_preserves_existing_hero() {
        let mut formation = Formation::new();
        let first_hero_id = HeroId::new("First");
        let second_hero_id = HeroId::new("Second");

        formation
            .place(&first_hero_id, Position::Frontline)
            .expect("initial placement should succeed");

        let result = formation.place(&second_hero_id, Position::Frontline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&first_hero_id));
        assert_eq!(formation.position_of(&second_hero_id), None);
    }

    #[test]
    fn place_rejects_same_hero_twice_and_preserves_original_position() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Warrior");

        formation
            .place(&hero_id, Position::Frontline)
            .expect("initial placement should succeed");

        let result = formation.place(&hero_id, Position::Backline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&hero_id));
        assert_eq!(formation.hero_at(Position::Backline), None);
        assert_eq!(formation.position_of(&hero_id), Some(Position::Frontline));
    }

    #[test]
    fn remove_clears_hero_but_keeps_position_slot_available() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Warrior");

        formation
            .place(&hero_id, Position::Frontline)
            .expect("placement should succeed");

        let result = formation.remove(&hero_id);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), None);
        assert!(formation.slots.contains_key(&Position::Frontline));
        assert_eq!(formation.position_of(&hero_id), None);
    }

    #[test]
    fn remove_returns_hero_not_found_for_unknown_hero() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Missing");

        let result = formation.remove(&hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.slots.len(), Position::all().len());
    }

    #[test]
    fn remove_compacts_midline_and_backline_forward_when_frontline_is_removed() {
        let mut formation = Formation::new();
        let front_hero_id = HeroId::new("Front");
        let mid_hero_id = HeroId::new("Mid");
        let back_hero_id = HeroId::new("Back");

        formation
            .place(&front_hero_id, Position::Frontline)
            .expect("front placement should succeed");
        formation
            .place(&mid_hero_id, Position::Midline)
            .expect("mid placement should succeed");
        formation
            .place(&back_hero_id, Position::Backline)
            .expect("back placement should succeed");

        let result = formation.remove(&front_hero_id);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), Some(&mid_hero_id));
        assert_eq!(formation.hero_at(Position::Midline), Some(&back_hero_id));
        assert_eq!(formation.hero_at(Position::Backline), None);
        assert_eq!(formation.position_of(&front_hero_id), None);
        assert_eq!(
            formation.position_of(&mid_hero_id),
            Some(Position::Frontline)
        );
        assert_eq!(
            formation.position_of(&back_hero_id),
            Some(Position::Midline)
        );
    }

    #[test]
    fn move_to_moves_hero_to_empty_position_and_clears_old_position() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Warrior");

        formation
            .place(&hero_id, Position::Frontline)
            .expect("placement should succeed");

        let result = formation.move_to(&hero_id, Position::Backline);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), None);
        assert_eq!(formation.hero_at(Position::Backline), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::Backline));
    }

    #[test]
    fn move_to_same_position_is_noop() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Warrior");

        formation
            .place(&hero_id, Position::Frontline)
            .expect("placement should succeed");

        let result = formation.move_to(&hero_id, Position::Frontline);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::Frontline));
    }

    #[test]
    fn move_to_rejects_occupied_position_and_preserves_both_heroes() {
        let mut formation = Formation::new();
        let first_hero_id = HeroId::new("First");
        let second_hero_id = HeroId::new("Second");

        formation
            .place(&first_hero_id, Position::Frontline)
            .expect("first placement should succeed");
        formation
            .place(&second_hero_id, Position::Backline)
            .expect("second placement should succeed");

        let result = formation.move_to(&first_hero_id, Position::Backline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&first_hero_id));
        assert_eq!(formation.hero_at(Position::Backline), Some(&second_hero_id));
        assert_eq!(
            formation.position_of(&first_hero_id),
            Some(Position::Frontline)
        );
        assert_eq!(
            formation.position_of(&second_hero_id),
            Some(Position::Backline)
        );
    }

    #[test]
    fn move_to_returns_hero_not_found_for_unknown_hero() {
        let mut formation = Formation::new();
        let hero_id = HeroId::new("Missing");

        let result = formation.move_to(&hero_id, Position::Frontline);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.hero_at(Position::Frontline), None);
    }

    #[test]
    fn swap_with_swaps_two_heroes() {
        let mut formation = Formation::new();
        let first_hero_id = HeroId::new("First");
        let second_hero_id = HeroId::new("Second");

        formation
            .place(&first_hero_id, Position::Frontline)
            .expect("first placement should succeed");
        formation
            .place(&second_hero_id, Position::Backline)
            .expect("second placement should succeed");

        let result = formation.swap_with(&first_hero_id, &second_hero_id);

        assert!(result.is_ok());
        assert_eq!(
            formation.hero_at(Position::Frontline),
            Some(&second_hero_id)
        );
        assert_eq!(formation.hero_at(Position::Backline), Some(&first_hero_id));
        assert_eq!(
            formation.position_of(&first_hero_id),
            Some(Position::Backline)
        );
        assert_eq!(
            formation.position_of(&second_hero_id),
            Some(Position::Frontline)
        );
    }

    #[test]
    fn swap_with_returns_hero_not_found_when_first_hero_is_missing() {
        let mut formation = Formation::new();
        let missing_hero_id = HeroId::new("Missing");
        let present_hero_id = HeroId::new("Present");

        formation
            .place(&present_hero_id, Position::Backline)
            .expect("placement should succeed");

        let result = formation.swap_with(&missing_hero_id, &present_hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(
            formation.hero_at(Position::Backline),
            Some(&present_hero_id)
        );
        assert_eq!(formation.position_of(&missing_hero_id), None);
    }

    #[test]
    fn swap_with_returns_hero_not_found_when_second_hero_is_missing() {
        let mut formation = Formation::new();
        let present_hero_id = HeroId::new("Present");
        let missing_hero_id = HeroId::new("Missing");

        formation
            .place(&present_hero_id, Position::Frontline)
            .expect("placement should succeed");

        let result = formation.swap_with(&present_hero_id, &missing_hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(
            formation.hero_at(Position::Frontline),
            Some(&present_hero_id)
        );
        assert_eq!(formation.position_of(&missing_hero_id), None);
    }
}
