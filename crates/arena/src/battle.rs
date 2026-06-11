use domain::{AbilityId, HeroId};

pub struct Battle {}

pub enum BattleEvents {
    GameStarted {
        left_side: Vec<HeroId>,
        right_side: Vec<HeroId>,
    },
    TurnStarted {
        hero_id: HeroId,
    },
    AbilityUsed {
        hero_id: HeroId,
        ability_id: AbilityId,
    },
    DamageDealt {
        hero_id: HeroId,
        amount: u8,
    },
    Healed {
        hero_id: HeroId,
        amount: u8,
    },
    HeroDied {
        hero_id: HeroId,
    },
    Swapped {
        first: HeroId,
        second: HeroId,
    },
    Bandaged {
        hero_id: HeroId,
        amount: u8,
    },
}
