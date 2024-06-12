use std::fmt::Display;

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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Checkpoint {
    pub position: Position,
    pub level: i32,
}
