use crate::Position;

use super::tile::TileKind;

#[derive(Debug, PartialEq)]
pub struct TileDescriptor {
    position: Position,
    kind: TileKind,
}

#[derive(Debug, PartialEq)]
pub enum BoardCreationError {
    InvalidBoardSize {
        size: usize,
    },
    NoEntrance,
    NoCheckpoint,
    TileOutOfBounds {
        tiles: Vec<TileDescriptor>,
    },
    OverlappingTiles {
        position: Position,
        kinds: (TileKind, TileKind),
    },
}

pub struct BoardCreationOptions {
    pub col_count: usize,
    pub row_count: usize,
    pub max_soft_wall_count: u32,
    pub walls: Vec<Position>,
    pub entrances: Vec<Position>,
    pub checkpoints: Vec<(Position, i32)>,
}

impl BoardCreationOptions {
    fn ensure_valid(&self) -> Result<(), BoardCreationError> {
        let Self {
            col_count,
            row_count,
            walls,
            entrances,
            checkpoints,
            max_soft_wall_count: _,
        } = self;

        let board_size = col_count * row_count;
        if board_size < 4 {
            return Err(BoardCreationError::InvalidBoardSize { size: board_size });
        }

        if entrances.len() == 0 {
            return Err(BoardCreationError::NoEntrance);
        }

        if checkpoints.len() == 0 {
            return Err(BoardCreationError::NoCheckpoint);
        }

        let out_of_bounds_entrances = entrances
            .iter()
            .filter(|(x, y)| x >= col_count || y >= row_count)
            .map(|position| TileDescriptor {
                position: *position,
                kind: TileKind::Entrance,
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

        let mut out_of_bounds_tiles = out_of_bounds_entrances
            .chain(out_of_bounds_checkpoints)
            .chain(out_of_bounds_walls)
            .peekable();

        match out_of_bounds_tiles.peek() {
            Some(_) => Err(BoardCreationError::TileOutOfBounds {
                tiles: out_of_bounds_tiles.collect(),
            }),
            None => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Board {
    tiles: Vec<Vec<TileKind>>,
    max_soft_wall_count: u32,
}

impl Board {
    pub fn new(options: &BoardCreationOptions) -> Result<Self, BoardCreationError> {
        options.ensure_valid()?;

        let mut tiles: Vec<Vec<TileKind>> =
            vec![vec![TileKind::Empty; options.row_count]; options.col_count];

        for (x, y) in options.walls.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(BoardCreationError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (tiles[*x][*y], TileKind::Wall),
                });
            }
            tiles[*x][*y] = TileKind::Wall;
        }

        for (x, y) in options.entrances.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(BoardCreationError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (tiles[*x][*y], TileKind::Entrance),
                });
            }
            tiles[*x][*y] = TileKind::Entrance;
        }

        for ((x, y), priority) in options.checkpoints.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(BoardCreationError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (tiles[*x][*y], TileKind::Checkpoint { level: *priority }),
                });
            }
            tiles[*x][*y] = TileKind::Checkpoint { level: *priority };
        }

        Ok(Self {
            tiles,
            max_soft_wall_count: options.max_soft_wall_count,
        })
    }

    pub fn get_tiles(&self) -> &Vec<Vec<TileKind>> {
        &self.tiles
    }

    pub fn get_max_soft_wall_count(&self) -> u32 {
        self.max_soft_wall_count
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_with_invalid_board_size() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 1,
            row_count: 0,
            max_soft_wall_count: 5,
            walls: vec![],
            entrances: vec![],
            checkpoints: vec![],
        });

        assert_eq!(board, Err(BoardCreationError::InvalidBoardSize { size: 0 }))
    }

    #[test]
    fn test_create_without_any_entrance() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrances: vec![],
            checkpoints: vec![],
        });

        assert_eq!(board, Err(BoardCreationError::NoEntrance))
    }

    #[test]
    fn test_create_without_any_checkpoint() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![],
        });

        assert_eq!(board, Err(BoardCreationError::NoCheckpoint))
    }

    #[test]
    fn test_create_with_wall_out_of_bounds() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(5, 5)],
            entrances: vec![(0, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::TileOutOfBounds {
                tiles: vec![TileDescriptor {
                    position: (5, 5),
                    kind: TileKind::Wall
                }]
            })
        )
    }

    #[test]
    fn test_create_with_entrance_out_of_bounds() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrances: vec![(3, 3)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::TileOutOfBounds {
                tiles: vec![TileDescriptor {
                    position: (3, 3),
                    kind: TileKind::Entrance
                }]
            })
        )
    }

    #[test]
    fn test_create_with_checkpoint_out_of_bounds() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrances: vec![(0, 0)],
            checkpoints: vec![((77, 77), 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::TileOutOfBounds {
                tiles: vec![TileDescriptor {
                    position: (77, 77),
                    kind: TileKind::Checkpoint { level: 1 }
                }]
            })
        )
    }

    #[test]
    fn test_create_with_overlapping_wall_and_entrance() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 0)],
            entrances: vec![(0, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::OverlappingTiles {
                position: (0, 0),
                kinds: (TileKind::Wall, TileKind::Entrance)
            })
        )
    }

    #[test]
    fn test_create_with_overlapping_wall_and_checkpoint() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 1)],
            entrances: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::OverlappingTiles {
                position: (1, 1),
                kinds: (TileKind::Wall, TileKind::Checkpoint { level: 1 })
            })
        )
    }

    #[test]
    fn test_create_basic_board() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 1)],
            entrances: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            board,
            Ok(Board {
                tiles: vec![
                    vec![TileKind::Empty, TileKind::Wall],
                    vec![TileKind::Entrance, TileKind::Checkpoint { level: 1 }]
                ],
                max_soft_wall_count: 5
            })
        )
    }

    #[test]
    fn test_create_with_multiple_checkpoints() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 3,
            row_count: 3,
            max_soft_wall_count: 5,
            walls: vec![(0, 1)],
            entrances: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1), ((2, 2), 2)],
        });

        assert_eq!(
            board,
            Ok(Board {
                tiles: vec![
                    vec![TileKind::Empty, TileKind::Wall, TileKind::Empty],
                    vec![
                        TileKind::Entrance,
                        TileKind::Checkpoint { level: 1 },
                        TileKind::Empty
                    ],
                    vec![
                        TileKind::Empty,
                        TileKind::Empty,
                        TileKind::Checkpoint { level: 2 }
                    ]
                ],
                max_soft_wall_count: 5
            })
        )
    }
}
