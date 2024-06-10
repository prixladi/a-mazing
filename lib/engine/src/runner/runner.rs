use crate::core::{Maze, Position, TileBoard, TileKind};

use super::run::Run;

#[derive(Debug)]
pub enum RunnerError {
    TooManySoftWalls { limit: u32 },
    WallOutOfBounds { position: Position },
    OverlappingWall { position: Position },
}

pub struct Runner<'a> {
    maze: &'a Maze,
    ascending_checkpoint_levels: Vec<i32>,
}

impl<'a> Runner<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        let mut checkpoint_levels: Vec<i32> = maze
            .get_tiles()
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|kind| match kind {
                        TileKind::Checkpoint { level: _ } => true,
                        _ => false,
                    })
                    .map(|kind| match kind {
                        TileKind::Checkpoint { level } => *level,
                        _ => todo!(),
                    })
            })
            .collect();
        checkpoint_levels.sort_by(|a, b| a.cmp(b));
        checkpoint_levels.dedup();

        Self {
            maze,
            ascending_checkpoint_levels: checkpoint_levels,
        }
    }

    pub fn run(&self, soft_walls: &Vec<Position>) -> Result<Option<Run>, RunnerError> {
        let board = self.get_board_with_soft_walls(soft_walls)?;
        let entrypoints = self.maze.get_entrypoints();
        let mut best_run: Option<Run> = None;

        for entrypoint in entrypoints.iter() {
            let current_run = Run::execute(&board, &self.ascending_checkpoint_levels, entrypoint);

            if let Some(new) = current_run {
                best_run = match best_run {
                    Some(old) if old.get_score() <= new.get_score() => Some(old),
                    _ => Some(new),
                };
            }
        }

        Ok(best_run)
    }

    fn get_board_with_soft_walls(
        &self,
        soft_walls: &Vec<Position>,
    ) -> Result<TileBoard, RunnerError> {
        let max_soft_wall_count = self.maze.get_max_soft_wall_count();
        if max_soft_wall_count < soft_walls.len() as u32 {
            return Err(RunnerError::TooManySoftWalls {
                limit: max_soft_wall_count,
            });
        }

        let mut tiles: TileBoard = self.maze.get_tiles().clone();
        for &Position { x, y } in soft_walls {
            if x >= tiles.len() {
                return Err(RunnerError::WallOutOfBounds {
                    position: Position { x, y },
                });
            }
            if y >= tiles[x].len() {
                return Err(RunnerError::WallOutOfBounds {
                    position: Position { x, y },
                });
            }
            if tiles[x][y] != TileKind::Empty {
                return Err(RunnerError::OverlappingWall {
                    position: Position { x, y },
                });
            }

            tiles[x][y] = TileKind::Wall
        }

        return Ok(tiles);
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Checkpoint, MazeConfiguration};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_basic() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap().unwrap();

        assert_eq!(result.get_score(), 14);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 4, y: 0 },
                Position { x: 5, y: 0 },
                Position { x: 6, y: 0 },
                Position { x: 7, y: 0 },
                Position { x: 7, y: 1 },
                Position { x: 7, y: 2 },
                Position { x: 7, y: 3 },
                Position { x: 7, y: 4 },
                Position { x: 7, y: 5 },
                Position { x: 7, y: 6 },
                Position { x: 7, y: 7 }
            ]
        )
    }

    #[test]
    fn test_run_basic_with_many_walls() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                Position { x: 2, y: 0 },
                Position { x: 2, y: 1 },
                Position { x: 2, y: 2 },
                Position { x: 2, y: 3 },
                Position { x: 2, y: 4 },
                Position { x: 2, y: 5 },
                Position { x: 2, y: 6 },
                Position { x: 4, y: 7 },
                Position { x: 4, y: 6 },
                Position { x: 4, y: 5 },
                Position { x: 4, y: 4 },
                Position { x: 4, y: 3 },
                Position { x: 4, y: 2 },
            ])
            .unwrap()
            .unwrap();

        assert_eq!(result.get_score(), 26);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 5 },
                Position { x: 1, y: 6 },
                Position { x: 1, y: 7 },
                Position { x: 2, y: 7 },
                Position { x: 3, y: 7 },
                Position { x: 3, y: 6 },
                Position { x: 3, y: 5 },
                Position { x: 3, y: 4 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 2 },
                Position { x: 3, y: 1 },
                Position { x: 4, y: 1 },
                Position { x: 5, y: 1 },
                Position { x: 6, y: 1 },
                Position { x: 7, y: 1 },
                Position { x: 7, y: 2 },
                Position { x: 7, y: 3 },
                Position { x: 7, y: 4 },
                Position { x: 7, y: 5 },
                Position { x: 7, y: 6 },
                Position { x: 7, y: 7 }
            ]
        );
    }

    #[test]
    fn test_run_basic_with_inaccessible_checkpoint() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                Position { x: 2, y: 0 },
                Position { x: 2, y: 1 },
                Position { x: 2, y: 2 },
                Position { x: 2, y: 3 },
                Position { x: 2, y: 4 },
                Position { x: 2, y: 5 },
                Position { x: 2, y: 6 },
                Position { x: 2, y: 7 },
            ])
            .unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_run_basic_with_multiple_entrypoints() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 5, y: 5 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                Position { x: 2, y: 0 },
                Position { x: 2, y: 1 },
                Position { x: 2, y: 2 },
                Position { x: 2, y: 3 },
                Position { x: 2, y: 4 },
                Position { x: 2, y: 5 },
                Position { x: 2, y: 6 },
                Position { x: 4, y: 7 },
                Position { x: 4, y: 6 },
                Position { x: 4, y: 5 },
                Position { x: 4, y: 4 },
                Position { x: 4, y: 3 },
                Position { x: 4, y: 2 },
            ])
            .unwrap()
            .unwrap();

        assert_eq!(result.get_score(), 4);
        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 5, y: 5 },
                Position { x: 6, y: 5 },
                Position { x: 7, y: 5 },
                Position { x: 7, y: 6 },
                Position { x: 7, y: 7 }
            ]
        )
    }

    #[test]
    fn test_run_leveled() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 6,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 5, y: 5 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 1, y: 1 },
                    level: 2,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap().unwrap();

        assert_eq!(result.get_score(), 18);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 4, y: 0 },
                Position { x: 5, y: 0 },
                Position { x: 5, y: 1 },
                Position { x: 5, y: 2 },
                Position { x: 5, y: 3 },
                Position { x: 5, y: 4 },
                Position { x: 5, y: 5 },
                Position { x: 4, y: 5 },
                Position { x: 3, y: 5 },
                Position { x: 2, y: 5 },
                Position { x: 1, y: 5 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 }
            ]
        );
    }

    #[test]
    fn test_run_leveled_with_multiple_entrypoints() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 6,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 4, y: 4 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 5, y: 5 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 1, y: 1 },
                    level: 2,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap().unwrap();

        assert_eq!(result.get_score(), 10);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 4, y: 4 },
                Position { x: 5, y: 4 },
                Position { x: 5, y: 5 },
                Position { x: 4, y: 5 },
                Position { x: 3, y: 5 },
                Position { x: 2, y: 5 },
                Position { x: 1, y: 5 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 }
            ]
        );
    }

    #[test]
    fn test_run_leveled_with_duplicate_checkpoints_0() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 7,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 5, y: 5 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 3, y: 3 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 1, y: 1 },
                    level: 2,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap().unwrap();

        assert_eq!(result.get_score(), 10);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 3, y: 1 },
                Position { x: 3, y: 2 },
                Position { x: 3, y: 3 },
                Position { x: 2, y: 3 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 }
            ]
        )
    }

    #[test]
    fn test_run_leveled_with_duplicate_checkpoints_1() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 7,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 0, y: 5 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 4, y: 4 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 5, y: 0 },
                    level: 2,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap().unwrap();

        assert_eq!(result.get_score(), 13);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 4, y: 0 },
                Position { x: 4, y: 1 },
                Position { x: 4, y: 2 },
                Position { x: 4, y: 3 },
                Position { x: 4, y: 4 },
                Position { x: 5, y: 4 },
                Position { x: 5, y: 3 },
                Position { x: 5, y: 2 },
                Position { x: 5, y: 1 },
                Position { x: 5, y: 0 }
            ]
        );
    }

    #[test]
    fn test_run_leveled_many_entrypoints_checkpoints_and_walls() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![
                Position { x: 0, y: 7 },
                Position { x: 1, y: 7 },
                Position { x: 1, y: 4 },
            ],
            entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 0, y: 8 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 0, y: 6 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 4, y: 4 },
                    level: 2,
                },
                Checkpoint {
                    position: Position { x: 5, y: 0 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 4, y: 0 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 6, y: 0 },
                    level: 4,
                },
                Checkpoint {
                    position: Position { x: 0, y: 1 },
                    level: 4,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![Position { x: 1, y: 6 }, Position { x: 1, y: 5 }])
            .unwrap()
            .unwrap();

        assert_eq!(result.get_score(), 20);

        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 0, y: 2 },
                Position { x: 0, y: 3 },
                Position { x: 0, y: 4 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 6 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 0, y: 3 },
                Position { x: 1, y: 3 },
                Position { x: 2, y: 3 },
                Position { x: 3, y: 3 },
                Position { x: 4, y: 3 },
                Position { x: 4, y: 4 },
                Position { x: 5, y: 4 },
                Position { x: 5, y: 3 },
                Position { x: 5, y: 2 },
                Position { x: 5, y: 1 },
                Position { x: 5, y: 0 },
                Position { x: 6, y: 0 }
            ]
        );
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![
                Position { x: 0, y: 7 },
                Position { x: 1, y: 7 },
                Position { x: 1, y: 4 },
            ],
            entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 0, y: 8 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 0, y: 6 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 4, y: 4 },
                    level: 2,
                },
                Checkpoint {
                    position: Position { x: 5, y: 0 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 4, y: 0 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 6, y: 0 },
                    level: 4,
                },
                Checkpoint {
                    position: Position { x: 0, y: 1 },
                    level: 4,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 5, y: 4 },
                Position { x: 3, y: 4 },
                Position { x: 4, y: 5 },
                Position { x: 4, y: 3 },
            ])
            .unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint_but_it_has_duplicate() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![
                Position { x: 0, y: 7 },
                Position { x: 1, y: 7 },
                Position { x: 1, y: 4 },
            ],
            entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 0, y: 8 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 0, y: 6 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 3, y: 0 },
                    level: 2,
                },
                Checkpoint {
                    position: Position { x: 4, y: 4 },
                    level: 2,
                },
                Checkpoint {
                    position: Position { x: 5, y: 0 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 4, y: 0 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 6, y: 0 },
                    level: 4,
                },
                Checkpoint {
                    position: Position { x: 0, y: 1 },
                    level: 4,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 5, y: 4 },
                Position { x: 3, y: 4 },
                Position { x: 4, y: 5 },
                Position { x: 4, y: 3 },
            ])
            .unwrap()
            .unwrap();

        assert_eq!(result.get_score(), 18);
        assert_eq!(
            result.get_solved_path(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 0, y: 2 },
                Position { x: 0, y: 3 },
                Position { x: 0, y: 4 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 6 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 0, y: 3 },
                Position { x: 1, y: 3 },
                Position { x: 2, y: 3 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 2 },
                Position { x: 3, y: 1 },
                Position { x: 3, y: 0 },
                Position { x: 4, y: 0 },
                Position { x: 5, y: 0 },
                Position { x: 6, y: 0 }
            ]
        );
    }

    #[test]
    fn test_run_leveled_big_maze() {
        let maze = Maze::new(&MazeConfiguration {
            col_count: 210,
            row_count: 26,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![
                Checkpoint {
                    position: Position { x: 4, y: 5 },
                    level: 1,
                },
                Checkpoint {
                    position: Position { x: 150, y: 20 },
                    level: 2,
                },
                Checkpoint {
                    position: Position { x: 1, y: 1 },
                    level: 3,
                },
                Checkpoint {
                    position: Position { x: 160, y: 20 },
                    level: 4,
                },
                Checkpoint {
                    position: Position { x: 1, y: 2 },
                    level: 5,
                },
                Checkpoint {
                    position: Position { x: 10, y: 25 },
                    level: 6,
                },
                Checkpoint {
                    position: Position { x: 10, y: 21 },
                    level: 6,
                },
                Checkpoint {
                    position: Position { x: 3, y: 3 },
                    level: 7,
                },
                Checkpoint {
                    position: Position { x: 120, y: 25 },
                    level: 8,
                },
                Checkpoint {
                    position: Position { x: 4, y: 4 },
                    level: 9,
                },
                Checkpoint {
                    position: Position { x: 130, y: 25 },
                    level: 10,
                },
                Checkpoint {
                    position: Position { x: 0, y: 1 },
                    level: 10,
                },
                Checkpoint {
                    position: Position { x: 200, y: 5 },
                    level: 11,
                },
                Checkpoint {
                    position: Position { x: 1, y: 21 },
                    level: 12,
                },
                Checkpoint {
                    position: Position { x: 6, y: 6 },
                    level: 13,
                },
                Checkpoint {
                    position: Position { x: 120, y: 24 },
                    level: 14,
                },
                Checkpoint {
                    position: Position { x: 7, y: 7 },
                    level: 15,
                },
                Checkpoint {
                    position: Position { x: 8, y: 19 },
                    level: 16,
                },
                Checkpoint {
                    position: Position { x: 8, y: 8 },
                    level: 17,
                },
                Checkpoint {
                    position: Position { x: 150, y: 19 },
                    level: 18,
                },
                Checkpoint {
                    position: Position { x: 200, y: 1 },
                    level: 19,
                },
                Checkpoint {
                    position: Position { x: 202, y: 1 },
                    level: 20,
                },
                Checkpoint {
                    position: Position { x: 1, y: 20 },
                    level: 20,
                },
                Checkpoint {
                    position: Position { x: 206, y: 1 },
                    level: 21,
                },
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let run = runner
            .run(&vec![
                Position { x: 205, y: 1 },
                Position { x: 207, y: 1 },
                Position { x: 206, y: 0 },
                Position { x: 205, y: 2 },
            ])
            .unwrap()
            .unwrap();

        assert_eq!(run.get_score(), 1985)
    }
}
