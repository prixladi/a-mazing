use crate::{
    core::{board::Board, tile::TileKind},
    Position,
};

use super::{
    node::{Node, Nodes},
    run::Run,
};

#[derive(Debug)]
pub enum RunnerCreationError {
    TooManySoftWalls { limit: u32 },
    WallOutOfBounds(usize, usize),
    WallOnInvalidTile(usize, usize),
}

pub struct Runner {
    tiles: Vec<Vec<TileKind>>,
    entrances: Vec<(usize, usize)>,
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
                return Err(RunnerCreationError::WallOutOfBounds(*x, *y));
            }
            if *y >= tiles[*x].len() {
                return Err(RunnerCreationError::WallOutOfBounds(*x, *y));
            }
            if tiles[*x][*y] != TileKind::Empty {
                return Err(RunnerCreationError::WallOnInvalidTile(*x, *y));
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
                    .map(|(y, _)| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        Ok(Self { tiles, entrances })
    }

    pub fn run(&self) -> Option<(u32, Vec<Position>)> {
        let mut best_run: Option<Run> = None;

        let nodes: Vec<Vec<Node>> = self
            .tiles
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, kind)| Node::new(*kind, (x, y)))
                    .collect()
            })
            .collect();

        for entrance in self.entrances.iter() {
            let current_run = Run::new(Nodes::new(nodes.clone()), *entrance);

            if let Some(new) = current_run {
                best_run = match best_run {
                    Some(old) if old.get_distance() < new.get_distance() => Some(old),
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
    fn test_run_1() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            exits: vec![(7, 7)],
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
    fn test_run_2() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            exits: vec![(7, 7)],
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
    fn test_run_3() {
        let board = Board::new(&BoardCreationOptions {
            col_count: 8,
            row_count: 8,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            exits: vec![(7, 7)],
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
}
