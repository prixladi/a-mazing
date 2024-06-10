use crate::Position;

use super::{maze_configuration::MazeConfiguration, maze_error::MazeError, tile::TileBoard};

pub struct Maze {
    board: TileBoard,
    entrypoints: Vec<Position>,
    max_soft_wall_count: u32,
}

impl Maze {
    pub fn new(config: &MazeConfiguration) -> Result<Self, MazeError> {
        let board = config.validate_and_convert_to_board()?;
        let max_soft_wall_count = config.max_soft_wall_count;

        Ok(Self {
            board,
            entrypoints: config.entrypoints.clone(),
            max_soft_wall_count,
        })
    }

    pub fn get_tiles(&self) -> &TileBoard {
        &self.board
    }

    pub fn get_entrypoints(&self) -> &Vec<Position> {
        &self.entrypoints
    }

    pub fn get_max_soft_wall_count(&self) -> u32 {
        self.max_soft_wall_count
    }
}

#[cfg(test)]
mod tests {
    use crate::core::tile::TileKind;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_invalid() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 7,
            walls: vec![(0, 10)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert!(maze.is_err());
    }

    #[test]
    fn test_create_basic() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 7,
            walls: vec![(0, 1)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            maze.as_ref().map(|maze| maze.get_max_soft_wall_count()),
            Ok(7)
        );

        assert_eq!(
            maze.as_ref().map(|maze| maze.get_tiles()),
            Ok(&vec![
                vec![TileKind::Empty, TileKind::Wall],
                vec![TileKind::Entrypoint, TileKind::Checkpoint { level: 1 }]
            ])
        )
    }

    #[test]
    fn test_create_with_multiple_checkpoints() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 3,
            row_count: 3,
            max_soft_wall_count: 5,
            walls: vec![(0, 1)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1), ((2, 2), 2)],
        });

        assert_eq!(
            maze.as_ref().map(|maze| maze.get_max_soft_wall_count()),
            Ok(5)
        );

        assert_eq!(
            maze.as_ref().map(|maze| maze.get_tiles()),
            Ok(&vec![
                vec![TileKind::Empty, TileKind::Wall, TileKind::Empty],
                vec![
                    TileKind::Entrypoint,
                    TileKind::Checkpoint { level: 1 },
                    TileKind::Empty
                ],
                vec![
                    TileKind::Empty,
                    TileKind::Empty,
                    TileKind::Checkpoint { level: 2 }
                ]
            ])
        )
    }
}
