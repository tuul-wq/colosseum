use domain::heroes::HeroId;
use domain::position::Position;

use crate::errors::WorldError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formation {
    slots: Vec<Option<HeroId>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lineup {
    heroes: Vec<HeroId>,
    position_count: usize,
}

impl Lineup {
    pub fn new(heroes: Vec<HeroId>) -> Self {
        let position_count = heroes.len();

        Self {
            heroes,
            position_count,
        }
    }

    pub fn with_position_count(heroes: Vec<HeroId>, position_count: usize) -> Self {
        assert!(
            position_count >= heroes.len(),
            "lineup position count must fit all heroes"
        );

        Self {
            heroes,
            position_count,
        }
    }

    pub fn len(&self) -> usize {
        self.heroes.len()
    }

    pub fn position_count(&self) -> usize {
        self.position_count
    }
}

impl<const N: usize> From<[HeroId; N]> for Lineup {
    fn from(heroes: [HeroId; N]) -> Self {
        Self::new(heroes.into())
    }
}

impl Formation {
    pub fn new(lineup: Lineup) -> Self {
        let mut slots = vec![None; lineup.position_count];

        for (slot, hero_id) in slots.iter_mut().zip(lineup.heroes) {
            *slot = Some(hero_id);
        }

        Self { slots }
    }

    pub fn all_heroes(&self) -> Vec<&HeroId> {
        self.slots.iter().filter_map(Option::as_ref).collect()
    }

    pub fn position_of(&self, hero_id: &HeroId) -> Option<Position> {
        self.slots
            .iter()
            .enumerate()
            .find_map(|(index, id)| (id.as_ref() == Some(hero_id)).then_some(Position::new(index)))
    }

    pub fn hero_at(&self, position: Position) -> Option<&HeroId> {
        self.slots.get(position.index()).and_then(Option::as_ref)
    }

    pub fn has_position(&self, position: Position) -> bool {
        self.slots.get(position.index()).is_some()
    }

    pub fn place(&mut self, hero_id: &HeroId, position: Position) -> Result<(), WorldError> {
        if self.position_of(hero_id).is_some() {
            return Err(WorldError::PositionOccupied);
        }

        let slot = self
            .slots
            .get_mut(position.index())
            .ok_or(WorldError::PositionNotFound)?;

        if slot.is_some() {
            return Err(WorldError::PositionOccupied);
        }

        *slot = Some(hero_id.clone());

        Ok(())
    }

    pub fn remove(&mut self, hero_id: &HeroId) -> Result<(), WorldError> {
        let pos = self.position_of(hero_id).ok_or(WorldError::HeroNotFound)?;

        let slot = self
            .slots
            .get_mut(pos.index())
            .ok_or(WorldError::PositionNotFound)?;
        *slot = None;

        self.compact_forward();

        Ok(())
    }

    pub fn move_to(&mut self, hero_id: &HeroId, new_position: Position) -> Result<(), WorldError> {
        let curr_pos = self.position_of(hero_id).ok_or(WorldError::HeroNotFound)?;

        if curr_pos == new_position {
            return Ok(());
        }

        if !self.has_position(new_position) {
            return Err(WorldError::PositionNotFound);
        }

        if self.hero_at(new_position).is_some() {
            return Err(WorldError::PositionOccupied);
        }

        self.slots[new_position.index()] = Some(hero_id.clone());
        self.slots[curr_pos.index()] = None;

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

        self.slots[first_hero_pos.index()] = Some(second_hero_id.clone());
        self.slots[second_hero_pos.index()] = Some(first_hero_id.clone());

        Ok(())
    }

    fn compact_forward(&mut self) {
        let heroes: Vec<_> = self.slots.iter_mut().filter_map(Option::take).collect();
        let mut heroes = heroes.into_iter();

        for slot in &mut self.slots {
            *slot = heroes.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::{HeroId, Position};

    use super::*;

    fn hero_id(name: &str) -> HeroId {
        HeroId::new(name)
    }

    fn two_hero_formation() -> Formation {
        Formation::new(Lineup::with_position_count(
            vec![hero_id("Front"), hero_id("Mid")],
            Position::all().len(),
        ))
    }

    fn three_hero_formation() -> Formation {
        Formation::new(Lineup::new(vec![
            hero_id("Front"),
            hero_id("Mid"),
            hero_id("Back"),
        ]))
    }

    #[test]
    fn new_with_two_places_heroes_in_front_and_midline() {
        let formation = two_hero_formation();
        let front_hero_id = hero_id("Front");
        let mid_hero_id = hero_id("Mid");

        for position in Position::all() {
            assert!(formation.has_position(position));
        }

        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&front_hero_id));
        assert_eq!(formation.hero_at(Position::MIDLINE), Some(&mid_hero_id));
        assert_eq!(formation.hero_at(Position::BACKLINE), None);
        assert_eq!(formation.slots.len(), Position::all().len());
    }

    #[test]
    fn new_with_three_places_heroes_in_all_positions() {
        let formation = three_hero_formation();
        let front_hero_id = hero_id("Front");
        let mid_hero_id = hero_id("Mid");
        let back_hero_id = hero_id("Back");

        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&front_hero_id));
        assert_eq!(formation.hero_at(Position::MIDLINE), Some(&mid_hero_id));
        assert_eq!(formation.hero_at(Position::BACKLINE), Some(&back_hero_id));
        assert_eq!(formation.slots.len(), Position::all().len());
    }

    #[test]
    fn new_supports_more_than_three_ordered_positions() {
        let formation = Formation::new(Lineup::new(vec![
            hero_id("One"),
            hero_id("Two"),
            hero_id("Three"),
            hero_id("Four"),
            hero_id("Five"),
        ]));

        assert_eq!(formation.hero_at(Position::new(0)), Some(&hero_id("One")));
        assert_eq!(formation.hero_at(Position::new(3)), Some(&hero_id("Four")));
        assert_eq!(formation.hero_at(Position::new(4)), Some(&hero_id("Five")));
        assert_eq!(formation.hero_at(Position::new(5)), None);
    }

    #[test]
    fn place_stores_hero_at_position() {
        let mut formation = two_hero_formation();
        let hero_id = hero_id("Back");

        let result = formation.place(&hero_id, Position::BACKLINE);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::BACKLINE), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::BACKLINE));
    }

    #[test]
    fn place_rejects_occupied_position_and_preserves_existing_hero() {
        let mut formation = two_hero_formation();
        let first_hero_id = hero_id("Front");
        let second_hero_id = hero_id("Second");

        let result = formation.place(&second_hero_id, Position::FRONTLINE);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&first_hero_id));
        assert_eq!(formation.position_of(&second_hero_id), None);
    }

    #[test]
    fn place_rejects_same_hero_twice_and_preserves_original_position() {
        let mut formation = two_hero_formation();
        let hero_id = hero_id("Front");

        let result = formation.place(&hero_id, Position::BACKLINE);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&hero_id));
        assert_eq!(formation.hero_at(Position::BACKLINE), None);
        assert_eq!(formation.position_of(&hero_id), Some(Position::FRONTLINE));
    }

    #[test]
    fn remove_clears_hero_but_keeps_position_slot_available() {
        let mut formation = two_hero_formation();
        let front_hero_id = hero_id("Front");
        let hero_id = hero_id("Mid");

        let result = formation.remove(&hero_id);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&front_hero_id));
        assert_eq!(formation.hero_at(Position::MIDLINE), None);
        assert!(formation.has_position(Position::MIDLINE));
        assert_eq!(formation.position_of(&hero_id), None);
    }

    #[test]
    fn remove_returns_hero_not_found_for_unknown_hero() {
        let mut formation = two_hero_formation();
        let hero_id = hero_id("Missing");

        let result = formation.remove(&hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.slots.len(), Position::all().len());
    }

    #[test]
    fn remove_compacts_midline_and_backline_forward_when_frontline_is_removed() {
        let mut formation = three_hero_formation();
        let front_hero_id = hero_id("Front");
        let mid_hero_id = hero_id("Mid");
        let back_hero_id = hero_id("Back");

        let result = formation.remove(&front_hero_id);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&mid_hero_id));
        assert_eq!(formation.hero_at(Position::MIDLINE), Some(&back_hero_id));
        assert_eq!(formation.hero_at(Position::BACKLINE), None);
        assert_eq!(formation.position_of(&front_hero_id), None);
        assert_eq!(
            formation.position_of(&mid_hero_id),
            Some(Position::FRONTLINE)
        );
        assert_eq!(
            formation.position_of(&back_hero_id),
            Some(Position::MIDLINE)
        );
    }

    #[test]
    fn move_to_moves_hero_to_empty_position_and_clears_old_position() {
        let mut formation = two_hero_formation();
        let hero_id = hero_id("Mid");

        let result = formation.move_to(&hero_id, Position::BACKLINE);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::MIDLINE), None);
        assert_eq!(formation.hero_at(Position::BACKLINE), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::BACKLINE));
    }

    #[test]
    fn move_to_same_position_is_noop() {
        let mut formation = two_hero_formation();
        let hero_id = hero_id("Front");

        let result = formation.move_to(&hero_id, Position::FRONTLINE);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::FRONTLINE));
    }

    #[test]
    fn move_to_rejects_occupied_position_and_preserves_both_heroes() {
        let mut formation = two_hero_formation();
        let first_hero_id = hero_id("Front");
        let second_hero_id = hero_id("Mid");

        let result = formation.move_to(&first_hero_id, Position::MIDLINE);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&first_hero_id));
        assert_eq!(formation.hero_at(Position::MIDLINE), Some(&second_hero_id));
        assert_eq!(
            formation.position_of(&first_hero_id),
            Some(Position::FRONTLINE)
        );
        assert_eq!(
            formation.position_of(&second_hero_id),
            Some(Position::MIDLINE)
        );
    }

    #[test]
    fn move_to_returns_hero_not_found_for_unknown_hero() {
        let mut formation = two_hero_formation();
        let missing_hero_id = hero_id("Missing");
        let front_hero_id = hero_id("Front");

        let result = formation.move_to(&missing_hero_id, Position::FRONTLINE);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.hero_at(Position::FRONTLINE), Some(&front_hero_id));
    }

    #[test]
    fn swap_with_swaps_two_heroes() {
        let mut formation = two_hero_formation();
        let first_hero_id = hero_id("Front");
        let second_hero_id = hero_id("Mid");

        let result = formation.swap_with(&first_hero_id, &second_hero_id);

        assert!(result.is_ok());
        assert_eq!(
            formation.hero_at(Position::FRONTLINE),
            Some(&second_hero_id)
        );
        assert_eq!(formation.hero_at(Position::MIDLINE), Some(&first_hero_id));
        assert_eq!(
            formation.position_of(&first_hero_id),
            Some(Position::MIDLINE)
        );
        assert_eq!(
            formation.position_of(&second_hero_id),
            Some(Position::FRONTLINE)
        );
    }

    #[test]
    fn swap_with_returns_hero_not_found_when_first_hero_is_missing() {
        let mut formation = two_hero_formation();
        let missing_hero_id = hero_id("Missing");
        let present_hero_id = hero_id("Mid");

        let result = formation.swap_with(&missing_hero_id, &present_hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.hero_at(Position::MIDLINE), Some(&present_hero_id));
        assert_eq!(formation.position_of(&missing_hero_id), None);
    }

    #[test]
    fn swap_with_returns_hero_not_found_when_second_hero_is_missing() {
        let mut formation = two_hero_formation();
        let present_hero_id = hero_id("Front");
        let missing_hero_id = hero_id("Missing");

        let result = formation.swap_with(&present_hero_id, &missing_hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(
            formation.hero_at(Position::FRONTLINE),
            Some(&present_hero_id)
        );
        assert_eq!(formation.position_of(&missing_hero_id), None);
    }
}
