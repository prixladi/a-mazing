use std::cmp;

use itertools::Itertools;

use crate::{
    core::{board::Board, tile::TileKind},
    Position,
};

pub struct Solver<'a> {
    board: &'a Board,
    empty_tiles: Vec<Position>,
}

impl<'a> Solver<'a> {
    pub fn new(board: &'a Board) -> Self {
        let empty_tiles = board
            .get_tiles()
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, kind)| **kind == TileKind::Empty)
                    .map(move |(y, _)| (x, y))
            })
            .collect();

        Self { board, empty_tiles }
    }

    pub fn solve(&self) {
        let max_wall_count = cmp::min(
            self.board.get_max_soft_wall_count() as usize,
            self.empty_tiles.len(),
        );

        let mut cnt: u64 = 0;
        for i in self.empty_tiles.iter().combinations(max_wall_count) {
            // println!("{:?}", i)

            cnt +=1;
        }

        println!("{}", cnt)
    }
}
