use maze_core::{Maze, Position, TileBoard, TileKind};

use crate::{run::run_maze, runner_error::MazeRunnerError};

use super::run::MazeRunResult;

pub struct MazeRunner<'a> {
    maze: &'a Maze,
    ascending_checkpoint_levels: Vec<i32>,
}

impl<'a> MazeRunner<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        let mut checkpoint_levels: Vec<i32> = maze
            .get_board()
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

    pub fn run(
        &self,
        soft_walls: &Vec<Position>,
    ) -> Result<Option<MazeRunResult>, MazeRunnerError> {
        let board = get_board_with_soft_walls(&self.maze, soft_walls)?;
        let mut best_result: Option<MazeRunResult> = None;

        for entrypoint in self.maze.get_entrypoints().iter() {
            let current_run = run_maze(&board, &self.ascending_checkpoint_levels, entrypoint);

            if let Some(new) = current_run {
                best_result = match best_result {
                    Some(old) if old.get_score() <= new.get_score() => Some(old),
                    _ => Some(new),
                };
            }
        }

        Ok(best_result)
    }
}

fn get_board_with_soft_walls(
    maze: &Maze,
    soft_walls: &Vec<Position>,
) -> Result<TileBoard, MazeRunnerError> {
    let max_soft_wall_count = maze.get_max_soft_wall_count();
    if max_soft_wall_count < soft_walls.len() as u32 {
        return Err(MazeRunnerError::TooManySoftWalls {
            limit: max_soft_wall_count,
        });
    }

    let mut tiles: TileBoard = maze.get_board().clone();
    for &Position { x, y } in soft_walls {
        if x >= tiles.len() {
            return Err(MazeRunnerError::WallOutOfBounds {
                position: Position { x, y },
            });
        }
        if y >= tiles[x].len() {
            return Err(MazeRunnerError::WallOutOfBounds {
                position: Position { x, y },
            });
        }
        if tiles[x][y] != TileKind::Empty {
            return Err(MazeRunnerError::OverlappingWall {
                position: Position { x, y },
            });
        }

        tiles[x][y] = TileKind::Wall
    }

    Ok(tiles)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use maze_core::{Checkpoint, MazeConfig};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_basic() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(14));

        assert_eq!(
            result.map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_basic_with_many_walls() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![
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
        ])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(26));

        assert_eq!(
            result.map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_basic_with_inaccessible_checkpoint() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![
            Position { x: 2, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },
            Position { x: 2, y: 3 },
            Position { x: 2, y: 4 },
            Position { x: 2, y: 5 },
            Position { x: 2, y: 6 },
            Position { x: 2, y: 7 },
        ])?;

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_run_basic_with_multiple_entrypoints() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 5, y: 5 }],
            checkpoints: vec![Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 1,
            }],
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![
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
        ])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(4));
        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
                Position { x: 5, y: 5 },
                Position { x: 6, y: 5 },
                Position { x: 7, y: 5 },
                Position { x: 7, y: 6 },
                Position { x: 7, y: 7 }
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(18));

        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled_with_multiple_entrypoints() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(10));

        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled_with_duplicate_checkpoints_0() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(10));

        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled_with_duplicate_checkpoints_1() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(13));

        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled_many_entrypoints_checkpoints_and_walls() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![Position { x: 1, y: 6 }, Position { x: 1, y: 5 }])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(20));

        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![
            Position { x: 1, y: 6 },
            Position { x: 1, y: 5 },
            Position { x: 5, y: 4 },
            Position { x: 3, y: 4 },
            Position { x: 4, y: 5 },
            Position { x: 4, y: 3 },
        ])?;

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint_but_it_has_duplicate() -> Result<(), Box<dyn Error>>
    {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![
            Position { x: 1, y: 6 },
            Position { x: 1, y: 5 },
            Position { x: 5, y: 4 },
            Position { x: 3, y: 4 },
            Position { x: 4, y: 5 },
            Position { x: 4, y: 3 },
        ])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(18));

        assert_eq!(
            result.as_ref().map(|res| res.get_solved_path()),
            Some(vec![
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
            ])
        );

        Ok(())
    }

    #[test]
    fn test_run_leveled_big_maze() -> Result<(), Box<dyn Error>> {
        let maze = Maze::new(&MazeConfig {
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
        })?;

        let runner = MazeRunner::new(&maze);
        let result = runner.run(&vec![
            Position { x: 205, y: 1 },
            Position { x: 207, y: 1 },
            Position { x: 206, y: 0 },
            Position { x: 205, y: 2 },
        ])?;

        assert_eq!(result.as_ref().map(|res| res.get_score()), Some(1985));

        Ok(())
    }
}
