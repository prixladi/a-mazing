use super::tile::{Position, TileKind};

#[derive(Debug, PartialEq)]
pub struct TileDescriptor {
    pub position: Position,
    pub kind: TileKind,
}

#[derive(Debug, PartialEq)]
pub enum MazeError {
    InvalidMazeSize {
        size: usize,
    },
    NoEntrypoint,
    NoCheckpoint,
    TileOutOfBounds(TileDescriptor),
    OverlappingTiles {
        position: Position,
        kinds: (TileKind, TileKind),
    },
}