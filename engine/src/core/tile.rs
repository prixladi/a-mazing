#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Empty,
    Entrance,
    Exit,
    Wall,
}