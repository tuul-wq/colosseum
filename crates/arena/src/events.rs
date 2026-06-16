use domain::{AbilityId, HeroId};

pub enum BattleEvent {
    GameStarted {
        left_side: Vec<HeroId>,
        right_side: Vec<HeroId>,
    },
    TurnStarted {
        source: HeroId,
    },
    AbilityUsed {
        source: HeroId,
        ability_id: AbilityId,
    },
    DamageDealt {
        source: HeroId,
        target: HeroId,
        amount: u8,
    },
    Healed {
        source: HeroId,
        target: HeroId,
        amount: u8,
    },
    HeroDied {
        source: HeroId,
    },
    Swapped {
        source: HeroId,
        target: HeroId,
    },
    Bandaged {
        source: HeroId,
        amount: u8,
    },
}
