use engine_core::Position;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Too many soft walls provided, limit is {limit}")]
    TooManySoftWalls { limit: u32 },
    #[error("Wall out of bound at position {position}")]
    WallOutOfBounds { position: Position },
    #[error("Overlapping wall at position {position}")]
    OverlappingWall { position: Position },
}
