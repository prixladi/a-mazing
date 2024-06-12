use super::{
    maze_error::MazeError,
    tile::{Checkpoint, Position, TileBoard, TileKind},
};

#[derive(Debug, PartialEq)]
pub struct MazeConfiguration {
    pub col_count: usize,
    pub row_count: usize,
    pub max_soft_wall_count: u32,
    pub entrypoints: Vec<Position>,
    pub checkpoints: Vec<Checkpoint>,
    pub walls: Vec<Position>,
}

impl MazeConfiguration {
    pub(crate) fn validate_and_convert_to_board(&self) -> Result<TileBoard, MazeError> {
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

        for &Position { x, y } in self.entrypoints.iter() {
            if x >= self.col_count || y >= self.row_count {
                return Err(MazeError::TileOutOfBounds(
                    Position { x, y },
                    TileKind::Entrypoint,
                ));
            }

            if board[x][y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: Position { x, y },
                    kinds: (board[x][y], TileKind::Entrypoint),
                });
            }

            board[x][y] = TileKind::Entrypoint;
        }

        for Checkpoint { position, level } in self.checkpoints.iter().cloned() {
            let Position { x, y } = position;
            if x >= self.col_count || y >= self.row_count {
                return Err(MazeError::TileOutOfBounds(
                    Position { x, y },
                    TileKind::Checkpoint { level },
                ));
            }

            if board[x][y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: Position { x, y },
                    kinds: (board[x][y], TileKind::Checkpoint { level }),
                });
            }
            board[x][y] = TileKind::Checkpoint { level };
        }

        for &Position { x, y } in self.walls.iter() {
            if x >= self.col_count || y >= self.row_count {
                return Err(MazeError::TileOutOfBounds(
                    Position { x, y },
                    TileKind::Wall,
                ));
            }

            if board[x][y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: Position { x, y },
                    kinds: (board[x][y], TileKind::Wall),
                });
            }
            board[x][y] = TileKind::Wall;
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::TileKind;

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
            entrypoints: vec![Position { x: 0, y: 0 }],
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
            walls: vec![Position { x: 5, y: 5 }],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 1,
            }],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::TileOutOfBounds(
                Position { x: 5, y: 5 },
                TileKind::Wall
            ))
        )
    }

    #[test]
    fn test_create_with_entrypoint_out_of_bounds() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![Position { x: 1, y: 0 }],
            entrypoints: vec![Position { x: 3, y: 3 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 1,
            }],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::TileOutOfBounds(
                Position { x: 3, y: 3 },
                TileKind::Entrypoint
            ))
        )
    }

    #[test]
    fn test_validate_and_convert_to_board_with_checkpoint_out_of_bounds() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![Position { x: 1, y: 0 }],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 77, y: 77 },
                level: 1,
            }],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::TileOutOfBounds(
                Position { x: 77, y: 77 },
                TileKind::Checkpoint { level: 1 }
            ))
        )
    }

    #[test]
    fn test_validate_and_convert_to_board_with_overlapping_wall_and_entrypoint() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![Position { x: 0, y: 0 }],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 1,
            }],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::OverlappingTiles {
                position: Position { x: 0, y: 0 },
                kinds: (TileKind::Entrypoint, TileKind::Wall)
            })
        )
    }

    #[test]
    fn test_validate_and_convert_to_board_with_overlapping_wall_and_checkpoint() {
        let configuration = MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![Position { x: 1, y: 1 }],
            entrypoints: vec![Position { x: 1, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 1,
            }],
        };

        let board = configuration.validate_and_convert_to_board();

        assert_eq!(
            board,
            Err(MazeError::OverlappingTiles {
                position: Position { x: 1, y: 1 },
                kinds: (TileKind::Checkpoint { level: 1 }, TileKind::Wall)
            })
        )
    }

    #[test]
    fn test_validate_and_convert_to_board() {
        let configuration = MazeConfiguration {
            col_count: 3,
            row_count: 3,
            max_soft_wall_count: 5,
            walls: vec![Position { x: 0, y: 1 }],
            entrypoints: vec![Position { x: 1, y: 0 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 1, y: 1 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 2, y: 2 },
                    level: 2,
                },
            ],
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
