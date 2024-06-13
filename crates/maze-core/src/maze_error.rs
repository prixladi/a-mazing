use thiserror::Error;

use super::tile::{Position, TileKind};

#[derive(Debug, PartialEq, Error)]
pub enum MazeError {
    #[error("Maze must have at least {min} tiles, got {size}")]
    InvalidMazeSize { size: usize, min: usize },
    #[error("Maze required at least one entrypoint")]
    NoEntrypoint,
    #[error("Maze required at least one checkpoint")]
    NoCheckpoint,
    #[error("Tile is out of bounds at position {0}")]
    TileOutOfBounds(Position, TileKind),
    #[error("Tiles are overlapping at position {position}")]
    OverlappingTiles {
        position: Position,
        kinds: (TileKind, TileKind),
    },
}
