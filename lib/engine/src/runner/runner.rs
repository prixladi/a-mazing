use crate::{
    core::{maze::Maze, tile::TileKind},
    Position,
};

use super::run::Run;

#[derive(Debug)]
pub enum RunnerError {
    TooManySoftWalls { limit: u32 },
    WallOutOfBounds { position: Position },
    OverlappingWall { position: Position },
}

pub struct Runner<'a> {
    maze: &'a Maze,
    entrypoints: Vec<Position>,
    asc_checkpoint_levels: Vec<i32>,
}

impl<'a> Runner<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        let entrypoints = maze
            .get_tiles()
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, kind)| **kind == TileKind::Entrypoint)
                    .map(move |(y, _)| (x, y))
            })
            .collect();

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
            entrypoints,
            asc_checkpoint_levels: checkpoint_levels,
        }
    }

    pub fn run(
        &self,
        soft_walls: &Vec<Position>,
    ) -> Result<Option<(u32, Vec<Position>)>, RunnerError> {
        let tiles = self.get_tiles(soft_walls)?;
        let mut best_run: Option<Run> = None;

        for entrypoint in self.entrypoints.iter() {
            let current_run = Run::execute(&tiles, &self.asc_checkpoint_levels, *entrypoint);

            if let Some(new) = current_run {
                best_run = match best_run {
                    Some(old) if old.get_distance() <= new.get_distance() => Some(old),
                    _ => Some(new),
                };
            }
        }

        Ok(best_run.map(|run| (run.get_distance(), run.get_solved_path())))
    }

    fn get_tiles(
        &self,
        soft_walls: &Vec<Position>,
    ) -> Result<Vec<Vec<TileKind>>, RunnerError> {
        let max_soft_wall_count = self.maze.get_max_soft_wall_count();
        if max_soft_wall_count < soft_walls.len() as u32 {
            return Err(RunnerError::TooManySoftWalls {
                limit: max_soft_wall_count,
            });
        }

        let mut tiles: Vec<Vec<TileKind>> = self.maze.get_tiles().clone();
        for (x, y) in soft_walls {
            if *x >= tiles.len() {
                return Err(RunnerError::WallOutOfBounds { position: (*x, *y) });
            }
            if *y >= tiles[*x].len() {
                return Err(RunnerError::WallOutOfBounds { position: (*x, *y) });
            }
            if tiles[*x][*y] != TileKind::Empty {
                return Err(RunnerError::OverlappingWall { position: (*x, *y) });
            }

            tiles[*x as usize][*y as usize] = TileKind::Wall
        }

        return Ok(tiles);
    }
}

#[cfg(test)]
mod tests {
    use crate::core::maze::MazeOptions;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_basic() {
        let maze = Maze::new(&MazeOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap();

        assert_eq!(
            result,
            Some((
                14,
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (5, 0),
                    (6, 0),
                    (7, 0),
                    (7, 1),
                    (7, 2),
                    (7, 3),
                    (7, 4),
                    (7, 5),
                    (7, 6),
                    (7, 7)
                ]
            ))
        )
    }

    #[test]
    fn test_run_basic_with_many_walls() {
        let maze = Maze::new(&MazeOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (4, 7),
                (4, 6),
                (4, 5),
                (4, 4),
                (4, 3),
                (4, 2),
            ])
            .unwrap();

        assert_eq!(
            result,
            Some((
                26,
                vec![
                    (0, 0),
                    (1, 0),
                    (1, 1),
                    (1, 2),
                    (1, 3),
                    (1, 4),
                    (1, 5),
                    (1, 6),
                    (1, 7),
                    (2, 7),
                    (3, 7),
                    (3, 6),
                    (3, 5),
                    (3, 4),
                    (3, 3),
                    (3, 2),
                    (3, 1),
                    (4, 1),
                    (5, 1),
                    (6, 1),
                    (7, 1),
                    (7, 2),
                    (7, 3),
                    (7, 4),
                    (7, 5),
                    (7, 6),
                    (7, 7)
                ]
            ))
        )
    }

    #[test]
    fn test_run_basic_with_inaccessible_checkpoint() {
        let maze = Maze::new(&MazeOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (2, 7),
            ])
            .unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_run_basic_with_multiple_entrypoints() {
        let maze = Maze::new(&MazeOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0), (5, 5)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (4, 7),
                (4, 6),
                (4, 5),
                (4, 4),
                (4, 3),
                (4, 2),
            ])
            .unwrap();

        assert_eq!(
            result,
            Some((4, vec![(5, 5), (6, 5), (7, 5), (7, 6), (7, 7)]))
        )
    }

    #[test]
    fn test_run_leveled() {
        let maze = Maze::new(&MazeOptions {
            col_count: 6,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((5, 5), 1), ((1, 1), 2)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap();

        assert_eq!(
            result,
            Some((
                18,
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (5, 0),
                    (5, 1),
                    (5, 2),
                    (5, 3),
                    (5, 4),
                    (5, 5),
                    (4, 5),
                    (3, 5),
                    (2, 5),
                    (1, 5),
                    (1, 4),
                    (1, 3),
                    (1, 2),
                    (1, 1)
                ]
            ))
        )
    }

    #[test]
    fn test_run_leveled_with_multiple_entrypoints() {
        let maze = Maze::new(&MazeOptions {
            col_count: 6,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0), (4, 4)],
            checkpoints: vec![((5, 5), 1), ((1, 1), 2)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap();

        assert_eq!(
            result,
            Some((
                10,
                vec![
                    (4, 4),
                    (5, 4),
                    (5, 5),
                    (4, 5),
                    (3, 5),
                    (2, 5),
                    (1, 5),
                    (1, 4),
                    (1, 3),
                    (1, 2),
                    (1, 1)
                ]
            ))
        )
    }

    #[test]
    fn test_run_leveled_with_duplicate_checkpoints_0() {
        let maze = Maze::new(&MazeOptions {
            col_count: 7,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((5, 5), 1), ((3, 3), 1), ((1, 1), 2)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap();

        assert_eq!(
            result,
            Some((
                10,
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (3, 1),
                    (3, 2),
                    (3, 3),
                    (2, 3),
                    (1, 3),
                    (1, 2),
                    (1, 1)
                ]
            ))
        )
    }

    #[test]
    fn test_run_leveled_with_duplicate_checkpoints_1() {
        let maze = Maze::new(&MazeOptions {
            col_count: 7,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![((0, 5), 1), ((4, 4), 1), ((5, 0), 2)],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![]).unwrap();

        assert_eq!(
            result,
            Some((
                13,
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (4, 1),
                    (4, 2),
                    (4, 3),
                    (4, 4),
                    (5, 4),
                    (5, 3),
                    (5, 2),
                    (5, 1),
                    (5, 0)
                ]
            ))
        )
    }

    #[test]
    fn test_run_leveled_many_entrypoints_checkpoints_and_walls() {
        let maze = Maze::new(&MazeOptions {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![(0, 7), (1, 7), (1, 4)],
            entrypoints: vec![(0, 0), (0, 8)],
            checkpoints: vec![
                ((0, 6), 1),
                ((4, 4), 2),
                ((5, 0), 3),
                ((4, 0), 3),
                ((6, 0), 4),
                ((0, 1), 4),
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner.run(&vec![(1, 6), (1, 5)]).unwrap();

        assert_eq!(
            result,
            Some((
                20,
                vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5),
                    (0, 6),
                    (0, 5),
                    (0, 4),
                    (0, 3),
                    (1, 3),
                    (2, 3),
                    (3, 3),
                    (4, 3),
                    (4, 4),
                    (5, 4),
                    (5, 3),
                    (5, 2),
                    (5, 1),
                    (5, 0),
                    (6, 0)
                ]
            ))
        )
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint() {
        let maze = Maze::new(&MazeOptions {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![(0, 7), (1, 7), (1, 4)],
            entrypoints: vec![(0, 0), (0, 8)],
            checkpoints: vec![
                ((0, 6), 1),
                ((4, 4), 2),
                ((5, 0), 3),
                ((4, 0), 3),
                ((6, 0), 4),
                ((0, 1), 4),
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![(1, 6), (1, 5), (5, 4), (3, 4), (4, 5), (4, 3)])
            .unwrap();

        assert_eq!(result, None)
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint_but_it_has_duplicate() {
        let maze = Maze::new(&MazeOptions {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![(0, 7), (1, 7), (1, 4)],
            entrypoints: vec![(0, 0), (0, 8)],
            checkpoints: vec![
                ((0, 6), 1),
                ((3, 0), 2),
                ((4, 4), 2),
                ((5, 0), 3),
                ((4, 0), 3),
                ((6, 0), 4),
                ((0, 1), 4),
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let result = runner
            .run(&vec![(1, 6), (1, 5), (5, 4), (3, 4), (4, 5), (4, 3)])
            .unwrap();

        assert_eq!(
            result,
            Some((
                18,
                vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5),
                    (0, 6),
                    (0, 5),
                    (0, 4),
                    (0, 3),
                    (1, 3),
                    (2, 3),
                    (3, 3),
                    (3, 2),
                    (3, 1),
                    (3, 0),
                    (4, 0),
                    (5, 0),
                    (6, 0)
                ]
            ))
        )
    }

    #[test]
    fn test_run_leveled_big_maze() {
        let maze = Maze::new(&MazeOptions {
            col_count: 210,
            row_count: 26,
            max_soft_wall_count: 200,
            walls: vec![],
            entrypoints: vec![(0, 0)],
            checkpoints: vec![
                ((4, 5), 1),
                ((150, 20), 2),
                ((1, 1), 3),
                ((160, 20), 4),
                ((1, 2), 5),
                ((10, 25), 6),
                ((10, 21), 6),
                ((3, 3), 7),
                ((120, 25), 8),
                ((4, 4), 9),
                ((130, 25), 10),
                ((0, 1), 10),
                ((200, 5), 11),
                ((1, 21), 12),
                ((6, 6), 13),
                ((120, 24), 14),
                ((7, 7), 15),
                ((8, 19), 16),
                ((8, 8), 17),
                ((150, 19), 18),
                ((200, 1), 19),
                ((202, 1), 20),
                ((1, 20), 20),
                ((206, 1), 21),
            ],
        })
        .unwrap();

        let runner = Runner::new(&maze);
        let (distance, _) = runner
            .run(&vec![(205, 1), (207, 1), (206, 0), (205, 2)])
            .unwrap()
            .unwrap();

        assert_eq!(distance, 1985)
    }
}
