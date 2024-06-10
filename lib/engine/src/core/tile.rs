#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Entrypoint,
    Empty,
    Wall,
    Checkpoint { level: i32 },
}

pub type TileBoard = Vec<Vec<TileKind>>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Checkpoint {
    pub position: Position,
    pub level: i32,
}
