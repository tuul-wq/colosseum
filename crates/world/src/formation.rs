use domain::{HeroId, Position};

use crate::errors::WorldError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formation {
    slots: [Option<HeroId>; Position::COUNT],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lineup {
    frontline: HeroId,
    midline: HeroId,
    backline: HeroId,
}

impl Lineup {
    pub fn new(frontline: HeroId, midline: HeroId, backline: HeroId) -> Self {
        Self {
            frontline,
            midline,
            backline,
        }
    }
}

impl From<[HeroId; Position::COUNT]> for Lineup {
    fn from([frontline, midline, backline]: [HeroId; Position::COUNT]) -> Self {
        Self::new(frontline, midline, backline)
    }
}

impl Formation {
    pub fn new(lineup: Lineup) -> Self {
        Self {
            slots: [
                Some(lineup.frontline),
                Some(lineup.midline),
                Some(lineup.backline),
            ],
        }
    }

    pub fn all_heroes(&self) -> Vec<&HeroId> {
        self.slots.iter().filter_map(Option::as_ref).collect()
    }

    pub fn position_of(&self, hero_id: &HeroId) -> Option<Position> {
        Position::ordered()
            .into_iter()
            .zip(&self.slots)
            .find_map(|(position, id)| (id.as_ref() == Some(hero_id)).then_some(position))
    }

    pub fn hero_at(&self, position: Position) -> Option<&HeroId> {
        self.slots[position.index()].as_ref()
    }

    pub fn place(&mut self, hero_id: &HeroId, position: Position) -> Result<(), WorldError> {
        if self.position_of(hero_id).is_some() {
            return Err(WorldError::PositionOccupied);
        }

        let slot = &mut self.slots[position.index()];

        if slot.is_some() {
            return Err(WorldError::PositionOccupied);
        }

        *slot = Some(hero_id.clone());

        Ok(())
    }

    pub fn remove(&mut self, hero_id: &HeroId) -> Result<(), WorldError> {
        let pos = self.position_of(hero_id).ok_or(WorldError::HeroNotFound)?;

        self.slots[pos.index()] = None;
        self.compact_forward();

        Ok(())
    }

    pub fn move_to(&mut self, hero_id: &HeroId, new_position: Position) -> Result<(), WorldError> {
        let curr_pos = self.position_of(hero_id).ok_or(WorldError::HeroNotFound)?;

        if curr_pos == new_position {
            return Ok(());
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
        let heroes = self
            .slots
            .iter_mut()
            .filter_map(Option::take)
            .collect::<Vec<_>>();

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

    fn formation() -> Formation {
        Formation::new(Lineup::new(
            hero_id("Front"),
            hero_id("Mid"),
            hero_id("Back"),
        ))
    }

    #[test]
    fn new_places_heroes_in_all_positions() {
        let formation = formation();
        let front_hero_id = hero_id("Front");
        let mid_hero_id = hero_id("Mid");
        let back_hero_id = hero_id("Back");

        assert_eq!(formation.hero_at(Position::Frontline), Some(&front_hero_id));
        assert_eq!(formation.hero_at(Position::Midline), Some(&mid_hero_id));
        assert_eq!(formation.hero_at(Position::Backline), Some(&back_hero_id));
    }

    #[test]
    fn place_stores_hero_at_empty_position() {
        let mut formation = formation();
        let old_back_hero_id = hero_id("Back");
        let new_back_hero_id = hero_id("NewBack");

        formation
            .remove(&old_back_hero_id)
            .expect("back hero removal should succeed");
        let result = formation.place(&new_back_hero_id, Position::Backline);

        assert!(result.is_ok());
        assert_eq!(
            formation.hero_at(Position::Backline),
            Some(&new_back_hero_id)
        );
        assert_eq!(
            formation.position_of(&new_back_hero_id),
            Some(Position::Backline)
        );
    }

    #[test]
    fn place_rejects_occupied_position_and_preserves_existing_hero() {
        let mut formation = formation();
        let first_hero_id = hero_id("Front");
        let second_hero_id = hero_id("Second");

        let result = formation.place(&second_hero_id, Position::Frontline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&first_hero_id));
        assert_eq!(formation.position_of(&second_hero_id), None);
    }

    #[test]
    fn place_rejects_same_hero_twice_and_preserves_original_position() {
        let mut formation = formation();
        let hero_id = hero_id("Front");

        let result = formation.place(&hero_id, Position::Backline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::Frontline));
    }

    #[test]
    fn remove_compacts_midline_and_backline_forward_when_frontline_is_removed() {
        let mut formation = formation();
        let front_hero_id = hero_id("Front");
        let mid_hero_id = hero_id("Mid");
        let back_hero_id = hero_id("Back");

        let result = formation.remove(&front_hero_id);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), Some(&mid_hero_id));
        assert_eq!(formation.hero_at(Position::Midline), Some(&back_hero_id));
        assert_eq!(formation.hero_at(Position::Backline), None);
        assert_eq!(formation.position_of(&front_hero_id), None);
    }

    #[test]
    fn remove_returns_hero_not_found_for_unknown_hero() {
        let mut formation = formation();
        let hero_id = hero_id("Missing");

        let result = formation.remove(&hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
    }

    #[test]
    fn move_to_moves_hero_to_empty_position_and_clears_old_position() {
        let mut formation = formation();
        let back_hero_id = hero_id("Back");
        let mid_hero_id = hero_id("Mid");

        formation
            .remove(&back_hero_id)
            .expect("back hero removal should succeed");
        let result = formation.move_to(&mid_hero_id, Position::Backline);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Midline), None);
        assert_eq!(formation.hero_at(Position::Backline), Some(&mid_hero_id));
        assert_eq!(
            formation.position_of(&mid_hero_id),
            Some(Position::Backline)
        );
    }

    #[test]
    fn move_to_same_position_is_noop() {
        let mut formation = formation();
        let hero_id = hero_id("Front");

        let result = formation.move_to(&hero_id, Position::Frontline);

        assert!(result.is_ok());
        assert_eq!(formation.hero_at(Position::Frontline), Some(&hero_id));
        assert_eq!(formation.position_of(&hero_id), Some(Position::Frontline));
    }

    #[test]
    fn move_to_rejects_occupied_position_and_preserves_both_heroes() {
        let mut formation = formation();
        let first_hero_id = hero_id("Front");
        let second_hero_id = hero_id("Mid");

        let result = formation.move_to(&first_hero_id, Position::Midline);

        assert!(matches!(result, Err(WorldError::PositionOccupied)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&first_hero_id));
        assert_eq!(formation.hero_at(Position::Midline), Some(&second_hero_id));
    }

    #[test]
    fn move_to_returns_hero_not_found_for_unknown_hero() {
        let mut formation = formation();
        let missing_hero_id = hero_id("Missing");
        let front_hero_id = hero_id("Front");

        let result = formation.move_to(&missing_hero_id, Position::Frontline);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.hero_at(Position::Frontline), Some(&front_hero_id));
    }

    #[test]
    fn swap_with_swaps_two_heroes() {
        let mut formation = formation();
        let first_hero_id = hero_id("Front");
        let second_hero_id = hero_id("Mid");

        let result = formation.swap_with(&first_hero_id, &second_hero_id);

        assert!(result.is_ok());
        assert_eq!(
            formation.hero_at(Position::Frontline),
            Some(&second_hero_id)
        );
        assert_eq!(formation.hero_at(Position::Midline), Some(&first_hero_id));
        assert_eq!(
            formation.position_of(&first_hero_id),
            Some(Position::Midline)
        );
        assert_eq!(
            formation.position_of(&second_hero_id),
            Some(Position::Frontline)
        );
    }

    #[test]
    fn swap_with_returns_hero_not_found_when_first_hero_is_missing() {
        let mut formation = formation();
        let missing_hero_id = hero_id("Missing");
        let present_hero_id = hero_id("Mid");

        let result = formation.swap_with(&missing_hero_id, &present_hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(formation.hero_at(Position::Midline), Some(&present_hero_id));
        assert_eq!(formation.position_of(&missing_hero_id), None);
    }

    #[test]
    fn swap_with_returns_hero_not_found_when_second_hero_is_missing() {
        let mut formation = formation();
        let present_hero_id = hero_id("Front");
        let missing_hero_id = hero_id("Missing");

        let result = formation.swap_with(&present_hero_id, &missing_hero_id);

        assert!(matches!(result, Err(WorldError::HeroNotFound)));
        assert_eq!(
            formation.hero_at(Position::Frontline),
            Some(&present_hero_id)
        );
        assert_eq!(formation.position_of(&missing_hero_id), None);
    }
}
