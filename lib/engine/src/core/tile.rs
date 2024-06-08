#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Entrance,
    Empty,
    Wall,
    Checkpoint { level: i32 },
}