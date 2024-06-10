use super::{maze_configuration::MazeConfiguration, maze_error::MazeError, tile::TileKind};

#[derive(Debug, PartialEq)]
pub struct Maze {
    tiles: Vec<Vec<TileKind>>,
    max_soft_wall_count: u32,
}

impl Maze {
    pub fn new(options: &MazeConfiguration) -> Result<Self, MazeError> {
        options.validate()?;

        let mut tiles: Vec<Vec<TileKind>> =
            vec![vec![TileKind::Empty; options.row_count]; options.col_count];

        for (x, y) in options.walls.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (tiles[*x][*y], TileKind::Wall),
                });
            }
            tiles[*x][*y] = TileKind::Wall;
        }

        for (x, y) in options.entrypoints.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
                    position: (*x, *y),
                    kinds: (tiles[*x][*y], TileKind::Entrypoint),
                });
            }
            tiles[*x][*y] = TileKind::Entrypoint;
        }

        for ((x, y), priority) in options.checkpoints.iter() {
            if tiles[*x][*y] != TileKind::Empty {
                return Err(MazeError::OverlappingTiles {
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
    use crate::core::maze_error::TileDescriptor;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_with_invalid_size() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 1,
            row_count: 0,
            max_soft_wall_count: 5,
            walls: vec![],
            entrypoints: vec![],
            checkpoints: vec![],
        });

        assert_eq!(maze, Err(MazeError::InvalidMazeSize { size: 0 }))
    }

    #[test]
    fn test_create_without_any_entrypoint() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrypoints: vec![],
            checkpoints: vec![],
        });

        assert_eq!(maze, Err(MazeError::NoEntrypoint))
    }

    #[test]
    fn test_create_without_any_checkpoint() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![],
        });

        assert_eq!(maze, Err(MazeError::NoCheckpoint))
    }

    #[test]
    fn test_create_with_wall_out_of_bounds() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(5, 5)],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            maze,
            Err(MazeError::TileOutOfBounds {
                tiles: vec![TileDescriptor {
                    position: (5, 5),
                    kind: TileKind::Wall
                }]
            })
        )
    }

    #[test]
    fn test_create_with_entrypoint_out_of_bounds() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrypoints: vec![(3, 3)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            maze,
            Err(MazeError::TileOutOfBounds {
                tiles: vec![TileDescriptor {
                    position: (3, 3),
                    kind: TileKind::Entrypoint
                }]
            })
        )
    }

    #[test]
    fn test_create_with_checkpoint_out_of_bounds() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 0)],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((77, 77), 1)],
        });

        assert_eq!(
            maze,
            Err(MazeError::TileOutOfBounds {
                tiles: vec![TileDescriptor {
                    position: (77, 77),
                    kind: TileKind::Checkpoint { level: 1 }
                }]
            })
        )
    }

    #[test]
    fn test_create_with_overlapping_wall_and_entrypoint() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 0)],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            maze,
            Err(MazeError::OverlappingTiles {
                position: (0, 0),
                kinds: (TileKind::Wall, TileKind::Entrypoint)
            })
        )
    }

    #[test]
    fn test_create_with_overlapping_wall_and_checkpoint() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(1, 1)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            maze,
            Err(MazeError::OverlappingTiles {
                position: (1, 1),
                kinds: (TileKind::Wall, TileKind::Checkpoint { level: 1 })
            })
        )
    }

    #[test]
    fn test_create_basic() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 2,
            row_count: 2,
            max_soft_wall_count: 5,
            walls: vec![(0, 1)],
            entrypoints: vec![(1, 0)],
            checkpoints: vec![((1, 1), 1)],
        });

        assert_eq!(
            maze,
            Ok(Maze {
                tiles: vec![
                    vec![TileKind::Empty, TileKind::Wall],
                    vec![TileKind::Entrypoint, TileKind::Checkpoint { level: 1 }]
                ],
                max_soft_wall_count: 5
            })
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
            maze,
            Ok(Maze {
                tiles: vec![
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
                ],
                max_soft_wall_count: 5
            })
        )
    }
}
