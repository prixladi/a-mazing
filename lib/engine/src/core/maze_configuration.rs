use crate::Position;

use super::{maze_error::{MazeError, TileDescriptor}, tile::TileKind};


pub struct MazeConfiguration {
    pub col_count: usize,
    pub row_count: usize,
    pub max_soft_wall_count: u32,
    pub walls: Vec<Position>,
    pub entrypoints: Vec<Position>,
    pub checkpoints: Vec<(Position, i32)>,
}

impl MazeConfiguration {
    pub fn validate(&self) -> Result<(), MazeError> {
        let Self {
            col_count,
            row_count,
            walls,
            entrypoints,
            checkpoints,
            max_soft_wall_count: _,
        } = self;

        let maze_size = col_count * row_count;
        if maze_size < 4 {
            return Err(MazeError::InvalidMazeSize { size: maze_size });
        }

        if entrypoints.len() == 0 {
            return Err(MazeError::NoEntrypoint);
        }

        if checkpoints.len() == 0 {
            return Err(MazeError::NoCheckpoint);
        }

        let out_of_bounds_entrypoints = entrypoints
            .iter()
            .filter(|(x, y)| x >= col_count || y >= row_count)
            .map(|position| TileDescriptor {
                position: *position,
                kind: TileKind::Entrypoint,
            });
        let out_of_bounds_checkpoints = checkpoints
            .iter()
            .filter(|((x, y), _)| x >= col_count || y >= row_count)
            .map(|(position, priority)| TileDescriptor {
                position: *position,
                kind: TileKind::Checkpoint { level: *priority },
            });
        let out_of_bounds_walls = walls
            .iter()
            .filter(|(x, y)| x >= col_count || y >= row_count)
            .map(|position| TileDescriptor {
                position: *position,
                kind: TileKind::Wall,
            });

        let out_of_bounds_tiles: Vec<TileDescriptor> = out_of_bounds_entrypoints
            .chain(out_of_bounds_checkpoints)
            .chain(out_of_bounds_walls)
            .collect();

        match out_of_bounds_tiles.len() {
            0 => Ok(()),
            _ => Err(MazeError::TileOutOfBounds {
                tiles: out_of_bounds_tiles,
            }),
        }
    }
}