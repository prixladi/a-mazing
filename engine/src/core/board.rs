use crate::Position;

use super::tile::TileKind;

#[derive(Debug, PartialEq)]
pub enum BoardCreationError {
    InvalidBoardSize {
        size: usize,
    },
    NoEntrance,
    NoExit,
    TileOutOfBound {
        position: Position,
        kind: TileKind,
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
    pub exits: Vec<Position>,
}

impl BoardCreationOptions {
    fn ensure_valid(&self) -> Result<(), BoardCreationError> {
        let Self {
            col_count,
            row_count,
            walls,
            entrances,
            exits,
            max_soft_wall_count: _,
        } = self;

        let board_size = col_count * row_count;
        if board_size < 4 {
            return Err(BoardCreationError::InvalidBoardSize { size: board_size });
        }

        if entrances.len() == 0 {
            return Err(BoardCreationError::NoEntrance);
        }

        if exits.len() == 0 {
            return Err(BoardCreationError::NoExit);
        }

        for (x, y) in entrances.iter() {
            if x >= col_count || y >= row_count {
                return Err(BoardCreationError::TileOutOfBound {
                    position: (*x, *y),
                    kind: TileKind::Entrance,
                });
            }
        }

        for (x, y) in exits.iter() {
            if x >= col_count || y >= row_count {
                return Err(BoardCreationError::TileOutOfBound {
                    position: (*x, *y),
                    kind: TileKind::Exit,
                });
            }
        }

        for (x, y) in walls.iter() {
            if x >= col_count || y >= row_count {
                return Err(BoardCreationError::TileOutOfBound {
                    position: (*x, *y),
                    kind: TileKind::Wall,
                });
            }
        }

        Ok(())
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

        for (x, y) in options.exits.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(BoardCreationError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (tiles[*x][*y], TileKind::Exit),
                });
            }
            tiles[*x][*y] = TileKind::Exit;
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
    fn test_create_with_bad_data_0() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 1,
            row_count: 0,
            max_soft_wall_count: 5,
            walls: vec![],
            entrances: vec![],
            exits: vec![],
        });

        assert_eq!(board, Err(BoardCreationError::InvalidBoardSize { size: 0 }))
    }

    #[test]
    fn test_create_with_bad_data_1() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrances: vec![],
            exits: vec![],
        });

        assert_eq!(board, Err(BoardCreationError::NoEntrance))
    }

    #[test]
    fn test_create_with_bad_data_2() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrances: vec![(0, 0)],
            exits: vec![],
        });

        assert_eq!(board, Err(BoardCreationError::NoExit))
    }

    #[test]
    fn test_create_with_bad_data_3() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(5, 5)],
            entrances: vec![(0, 0)],
            exits: vec![(1, 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::TileOutOfBound {
                position: (5, 5),
                kind: TileKind::Wall
            })
        )
    }

    #[test]
    fn test_create_with_bad_data_4() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrances: vec![(3, 3)],
            exits: vec![(1, 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::TileOutOfBound {
                position: (3, 3),
                kind: TileKind::Entrance
            })
        )
    }

    #[test]
    fn test_create_with_bad_data_5() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrances: vec![(0, 0)],
            exits: vec![(77, 77)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::TileOutOfBound {
                position: (77, 77),
                kind: TileKind::Exit
            })
        )
    }

    #[test]
    fn test_create_with_bad_data_6() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 0)],
            entrances: vec![(0, 0)],
            exits: vec![(1, 1)],
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
    fn test_create_with_bad_data_7() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 1)],
            entrances: vec![(1, 0)],
            exits: vec![(1, 1)],
        });

        assert_eq!(
            board,
            Err(BoardCreationError::OverlappingTiles {
                position: (1, 1),
                kinds: (TileKind::Wall, TileKind::Exit)
            })
        )
    }

    #[test]
    fn test_create_with_correct_data() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 1)],
            entrances: vec![(1, 0)],
            exits: vec![(1, 1)],
        });

        assert_eq!(
            board,
            Ok(Board {
                tiles: vec![
                    vec![TileKind::Empty, TileKind::Wall],
                    vec![TileKind::Entrance, TileKind::Exit]
                ],
                max_soft_wall_count: 5
            })
        )
    }
}
