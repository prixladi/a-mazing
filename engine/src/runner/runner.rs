use crate::{
    core::{board::Board, tile::TileKind},
    Position,
};

use super::run::Run;

#[derive(Debug)]
pub enum RunnerCreationError {
    TooManySoftWalls { limit: u32 },
    WallOutOfBounds { position: Position },
    OverlappingWall { position: Position },
}

pub struct Runner {
    tiles: Vec<Vec<TileKind>>,
    entrances: Vec<Position>,
    asc_checkpoint_levels: Vec<i32>,
}

impl Runner {
    pub fn new(board: &Board, soft_walls: &Vec<Position>) -> Result<Self, RunnerCreationError> {
        let tiles = board.get_tiles();
        let max_soft_wall_count = board.get_max_soft_wall_count();

        if max_soft_wall_count < soft_walls.len() as u32 {
            return Err(RunnerCreationError::TooManySoftWalls {
                limit: max_soft_wall_count,
            });
        }

        let mut tiles: Vec<Vec<TileKind>> = tiles.clone();
        for (x, y) in soft_walls {
            if *x >= tiles.len() {
                return Err(RunnerCreationError::WallOutOfBounds { position: (*x, *y) });
            }
            if *y >= tiles[*x].len() {
                return Err(RunnerCreationError::WallOutOfBounds { position: (*x, *y) });
            }
            if tiles[*x][*y] != TileKind::Empty {
                return Err(RunnerCreationError::OverlappingWall { position: (*x, *y) });
            }

            tiles[*x as usize][*y as usize] = TileKind::Wall
        }

        let entrances = tiles
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, kind)| **kind == TileKind::Entrance)
                    .map(move |(y, _)| (x, y))
            })
            .collect();

        let mut checkpoint_levels: Vec<i32> = tiles
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

        Ok(Self {
            tiles,
            entrances,
            asc_checkpoint_levels: checkpoint_levels,
        })
    }

    pub fn run(&self) -> Option<(u32, Vec<Position>)> {
        let mut best_run: Option<Run> = None;
        for entrance in self.entrances.iter() {
            let current_run = Run::execute(&self.tiles, &self.asc_checkpoint_levels, *entrance);

            if let Some(new) = current_run {
                best_run = match best_run {
                    Some(old) if old.get_distance() <= new.get_distance() => Some(old),
                    _ => Some(new),
                };
            }
        }

        best_run.map(|run| (run.get_distance(), run.get_solved_path()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::board::BoardCreationOptions;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_basic() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(&board, &vec![]).unwrap();
        let result = runner.run();

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
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(
            &board,
            &vec![
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
            ],
        )
        .unwrap();
        let result = runner.run();

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
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(
            &board,
            &vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (2, 7),
            ],
        )
        .unwrap();
        let result = runner.run();

        assert_eq!(result, None);
    }

    #[test]
    fn test_run_basic_with_multiple_entrances() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0), (5, 5)],
            checkpoints: vec![((7, 7), 1)],
        })
        .unwrap();

        let runner = Runner::new(
            &board,
            &vec![
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
            ],
        )
        .unwrap();
        let result = runner.run();

        assert_eq!(
            result,
            Some((4, vec![(5, 5), (6, 5), (7, 5), (7, 6), (7, 7)]))
        )
    }

    #[test]
    fn test_run_leveled() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 6,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![((5, 5), 1), ((1, 1), 2)],
        })
        .unwrap();

        let runner = Runner::new(&board, &vec![]).unwrap();
        let result = runner.run();

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
    fn test_run_leveled_with_multiple_entrances() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 6,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0), (4, 4)],
            checkpoints: vec![((5, 5), 1), ((1, 1), 2)],
        })
        .unwrap();

        let runner = Runner::new(&board, &vec![]).unwrap();
        let result = runner.run();

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
        let board = Board::new(&BoardCreationOptions {
            col_count: 7,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![((5, 5), 1), ((3, 3), 1), ((1, 1), 2)],
        })
        .unwrap();

        let runner = Runner::new(&board, &vec![]).unwrap();
        let result = runner.run();

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
        let board = Board::new(&BoardCreationOptions {
            col_count: 7,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![((0, 5), 1), ((4, 4), 1), ((5, 0), 2)],
        })
        .unwrap();

        let runner = Runner::new(&board, &vec![]).unwrap();
        let result = runner.run();

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
    fn test_run_leveled_many_entrances_checkpoints_and_walls() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![(0, 7), (1, 7), (1, 4)],
            entrances: vec![(0, 0), (0, 8)],
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

        let runner = Runner::new(&board, &vec![(1, 6), (1, 5)]).unwrap();
        let result = runner.run();

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
        let board = Board::new(&BoardCreationOptions {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![(0, 7), (1, 7), (1, 4)],
            entrances: vec![(0, 0), (0, 8)],
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

        let runner = Runner::new(
            &board,
            &vec![(1, 6), (1, 5), (5, 4), (3, 4), (4, 5), (4, 3)],
        )
        .unwrap();
        let result = runner.run();

        assert_eq!(result, None)
    }

    #[test]
    fn test_run_leveled_inaccessible_checkpoint_but_it_has_duplicate() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 9,
            row_count: 9,
            max_soft_wall_count: 200,
            walls: vec![(0, 7), (1, 7), (1, 4)],
            entrances: vec![(0, 0), (0, 8)],
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

        let runner = Runner::new(
            &board,
            &vec![(1, 6), (1, 5), (5, 4), (3, 4), (4, 5), (4, 3)],
        )
        .unwrap();
        let result = runner.run();

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
}
