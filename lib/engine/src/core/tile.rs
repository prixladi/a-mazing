#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Entrypoint,
    Empty,
    Wall,
    Checkpoint { level: i32 },
}