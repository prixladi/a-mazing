use std::collections::HashSet;

use maze_core::{Checkpoint, Maze, MazeConfig, Position};
use maze_runner::MazeRunner;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::GeneratorError;

pub(super) fn get_top_wall_positions(col_count: usize, row_count: usize) -> Vec<Position> {
    let x_line_range = 0..col_count;
    x_line_range
        .map(|x| Position {
            x,
            y: row_count - 1,
        })
        .collect()
}

pub(super) fn get_bottom_wall_positions(col_count: usize) -> Vec<Position> {
    let x_line_range = 0..col_count;
    x_line_range.map(|x| Position { x, y: 0 }).collect()
}

pub(super) fn get_right_wall_positions(col_count: usize, row_count: usize) -> Vec<Position> {
    let y_line_range = 0..row_count;
    y_line_range
        .map(|y| Position {
            x: col_count - 1,
            y,
        })
        .collect()
}

pub(super) fn get_left_wall_positions(row_count: usize) -> Vec<Position> {
    let y_line_range = 0..row_count;
    y_line_range.map(|y| Position { x: 0, y }).collect()
}

pub(super) fn get_empty_positions_in_rectangle(
    top_left: Position,
    bottom_right: Position,
    used_positions: &[&Vec<Position>],
) -> Vec<Position> {
    let used: HashSet<&Position> = used_positions.iter().cloned().flatten().collect();

    let mut empty_positions = vec![];

    for x in top_left.x..bottom_right.x {
        for y in bottom_right.y..top_left.y {
            let position = Position { x, y };
            if !used.contains(&position) {
                empty_positions.push(position);
            }
        }
    }

    empty_positions
}

pub(super) fn get_empty_positions_with_padding(
    col_count: usize,
    row_count: usize,
    padding_x: usize,
    padding_y: usize,
    used_positions: &[&Vec<Position>],
) -> Vec<Position> {
    get_empty_positions_in_rectangle(
        Position {
            x: padding_x,
            y: row_count - padding_y,
        },
        Position {
            x: col_count - padding_x,
            y: padding_y,
        },
        used_positions,
    )
}

pub(super) fn get_checkpoints(
    checkpoint_positions: &Vec<Position>,
    exit_positions: &Vec<Position>,
) -> Vec<Checkpoint> {
    checkpoint_positions
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, position)| Checkpoint {
            position,
            level: i as i32 + 1,
        })
        .chain(exit_positions.iter().cloned().map(|position| Checkpoint {
            position,
            level: checkpoint_positions.len() as i32 + 1,
        }))
        .collect()
}

pub(super) fn is_solvable(
    config: &MazeConfig,
    walls: &Vec<Position>,
) -> Result<bool, GeneratorError> {
    let maze = Maze::new(config).map_err(GeneratorError::from_maze_error)?;
    let run = MazeRunner::new(&maze)
        .run(&walls)
        .map_err(GeneratorError::from_runner_error)?;

    Ok(run.is_some())
}

pub(super) fn get_random_number_in_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    thread_rng().gen_range(range)
}

pub(super) fn get_random_shuffle(positions: &Vec<Position>) -> Vec<Position> {
    let mut copy: Vec<Position> = positions.clone();
    copy.shuffle(&mut thread_rng());
    copy.into_iter().collect()
}

pub(super) fn get_random_positions(positions: &Vec<Position>, n: usize) -> Vec<Position> {
    get_random_shuffle(positions).into_iter().take(n).collect()
}

pub(super) fn get_random_solvable_walls(
    config: &MazeConfig,
    empty_positions: &Vec<Position>,
    wall_count: usize,
) -> Result<Vec<Position>, GeneratorError> {
    let mut walls = vec![];
    for position in get_random_shuffle(empty_positions).into_iter() {
        if walls.len() >= wall_count {
            break;
        }

        walls.push(position);
        if !is_solvable(&config, &walls)? {
            walls.pop();
        }
    }

    Ok(walls)
}
