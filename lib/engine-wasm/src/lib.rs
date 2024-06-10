mod models;
mod utils;

use engine::{
    core::{maze::Maze, maze_configuration::MazeConfiguration},
    runner::runner::Runner,
};
use models::{MazerOptions, MazerPosition, MazerRunResult};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mazer {
    maze: Maze,
}

#[wasm_bindgen]
impl Mazer {
    pub fn new(options: MazerOptions) -> Mazer {
        set_panic_hook();

        let maze = Maze::new(&MazeConfiguration {
            col_count: options.get_col_count(),
            row_count: options.get_row_count(),
            max_soft_wall_count: options.get_max_soft_wall_count(),
            walls: options
                .get_walls()
                .iter()
                .map(|pos| (pos.get_x(), pos.get_y()))
                .collect(),
            entrypoints: options
                .get_entrypoints()
                .iter()
                .map(|pos| (pos.get_x(), pos.get_y()))
                .collect(),
            checkpoints: options
                .get_checkpoints()
                .iter()
                .map(|checkpoint| {
                    let position = checkpoint.get_position();
                    ((position.get_x(), position.get_y()), checkpoint.get_level())
                })
                .collect(),
        })
        .unwrap();

        Self { maze }
    }

    #[wasm_bindgen]
    pub fn run(&self, soft_walls: Vec<MazerPosition>) -> Option<MazerRunResult> {
        let walls = soft_walls
            .iter()
            .map(|pos| (pos.get_x(), pos.get_y()))
            .collect();

        let runner = Runner::new(&self.maze);
        let result = runner.run(&walls).unwrap();

        return result.map(|run| {
            MazerRunResult::new(
                run.get_score(),
                run.get_solved_path()
                    .iter()
                    .cloned()
                    .map(|(x, y)| MazerPosition::new(x, y))
                    .collect(),
            )
        });
    }
}
