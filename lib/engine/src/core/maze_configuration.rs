use crate::Position;

use super::{
    maze_error::{MazeError, TileDescriptor},
    tile::{TileBoard, TileKind},
};

#[derive(Debug, PartialEq)]
pub struct MazeConfiguration {
    pub col_count: usize,
    pub row_count: usize,
    pub max_soft_wall_count: u32,
    pub walls: Vec<Position>,
    pub entrypoints: Vec<Position>,
    pub checkpoints: Vec<(Position, i32)>,
}

impl MazeConfiguration {
    pub fn validate_and_convert_to_board(&self) -> Result<TileBoard, MazeError> {
        let maze_size = self.col_count * self.row_count;
        if maze_size < 4 {
            return Err(MazeError::InvalidMazeSize { size: maze_size });
        }

        if self.entrypoints.len() == 0 {
            return Err(MazeError::NoEntrypoint);
        }

        if self.checkpoints.len() == 0 {
            return Err(MazeError::NoCheckpoint);
        }

        let mut board: TileBoard = vec![vec![TileKind::Empty; self.row_count]; self.col_count];

        for (x, y) in self.walls.iter() {
            if *x >= self.col_count || *y >= self.row_count {
                return Err(MazeError::TileOutOfBounds(TileDescriptor {
                    position: (*x, *y),
                    kind: TileKind::Wall,
                }));
            }

            if board[*x][*y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (board[*x][*y], TileKind::Wall),
                });
            }
            board[*x][*y] = TileKind::Wall;
        }

        for (x, y) in self.entrypoints.iter() {
            if *x >= self.col_count || *y >= self.row_count {
                return Err(MazeError::TileOutOfBounds(TileDescriptor {
                    position: (*x, *y),
                    kind: TileKind::Entrypoint,
                }));
            }

            if board[*x][*y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (board[*x][*y], TileKind::Entrypoint),
                });
            }

            board[*x][*y] = TileKind::Entrypoint;
        }

        for ((x, y), priority) in self.checkpoints.iter() {
            if *x >= self.col_count || *y >= self.row_count {
                return Err(MazeError::TileOutOfBounds(TileDescriptor {
                    position: (*x, *y),
                    kind: TileKind::Checkpoint { level: *priority },
                }));
            }

            if board[*x][*y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (board[*x][*y], TileKind::Checkpoint { level: *priority }),
                });
            }
            board[*x][*y] = TileKind::Checkpoint { level: *priority };
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::tile::TileKind;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate_and_convert_to_board_with_invalid_size() {
        let configuration = MazeConfiguration {
            col_count: 1,
            row_count: 0,
            max_soft_wall_count: 5,
            walls: vec![],
            entrypoints: vec![],
            checkpoints: vec![],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(board, Err(MazeError::InvalidMazeSize { size: 0 }))
    }

    #[test]
    fn test_validate_and_convert_to_board_without_any_entrypoint() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrypoints: vec![],
            checkpoints: vec![],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(board, Err(MazeError::NoEntrypoint))
    }

    #[test]
    fn test_validate_and_convert_to_board_without_any_checkpoint() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(board, Err(MazeError::NoCheckpoint))
    }

    #[test]
    fn test_validate_and_convert_to_board_with_wall_out_of_bounds() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(5, 5)],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((1, 1), 1)],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::TileOutOfBounds(TileDescriptor {
                position: (5, 5),
                kind: TileKind::Wall
            }))
        )
    }

    #[test]
    fn test_create_with_entrypoint_out_of_bounds() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrypoints: vec![(3, 3)],
            checkpoints: vec![((1, 1), 1)],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::TileOutOfBounds(TileDescriptor {
                position: (3, 3),
                kind: TileKind::Entrypoint
            }))
        )
    }

    #[test]
    fn test_validate_and_convert_to_board_with_checkpoint_out_of_bounds() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((77, 77), 1)],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::TileOutOfBounds(TileDescriptor {
                position: (77, 77),
                kind: TileKind::Checkpoint { level: 1 }
            }))
        )
    }

    #[test]
    fn test_validate_and_convert_to_board_with_overlapping_wall_and_entrypoint() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 0)],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((1, 1), 1)],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::OverlappingTiles {
                position: (0, 0),
                kinds: (TileKind::Wall, TileKind::Entrypoint)
            })
        )
    }

    #[test]
    fn test_validate_and_convert_to_board_with_overlapping_wall_and_checkpoint() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 1)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::OverlappingTiles {
                position: (1, 1),
                kinds: (TileKind::Wall, TileKind::Checkpoint { level: 1 })
            })
        )
    }

    #[test]
    fn test_validate_and_convert_to_board() {
        let configuration = MazeConfiguration {
            col_count: 3,
            row_count: 3,
            max_soft_wall_count: 5,
            walls: vec![(0, 1)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1), ((2, 2), 2)],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Ok(vec![
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
