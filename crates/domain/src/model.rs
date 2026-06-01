use uuid::Uuid;

use crate::class::HeroClass;

pub type HeroID = Uuid;

#[derive(Debug)]
pub struct Hero {
    pub id: HeroID,
    pub stats: Stats,
    pub class: HeroClass,
    pub weapon: Weapon,
    pub position: Position,
}

#[derive(Debug)]
pub struct Stats {
    /// The name of the hero.
    pub name: String,
    /// The health of the hero, in hit points.
    pub health: Health,
    /// Whether the hero is alive or not.
    pub alive: bool,
    /// The initiative of the hero, determines turn order.
    pub initiative: u8,
    /// The speed of the hero, in cells per turn.
    pub speed: u8,
}

#[derive(Debug)]
pub struct Health {
    /// The maximum health of the hero, in hit points.
    pub max: u16,
    /// The current health of the hero, in hit points.
    pub current: u16,
}

#[derive(Debug)]
pub struct Weapon {
    /// The damage of the weapon, in hit points.
    pub damage: u16,
    /// The range of the weapon, in cells.
    pub range: u8,
}

#[derive(Debug)]
pub struct Position {
    /// The x-coordinate of the position.
    pub x: u8,
    /// The y-coordinate of the position.
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
