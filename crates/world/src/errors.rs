#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    #[error("Formation not found")]
    FormationNotFound,

    #[error("Hero not found")]
    HeroNotFound,

    #[error("Position occupied")]
    PositionOccupied,

    #[error("Position not found")]
    PositionNotFound,
}
