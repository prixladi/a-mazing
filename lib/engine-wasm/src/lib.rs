mod models;
mod utils;

use engine::{
    core::{
        maze::Maze,
        maze_configuration::MazeConfiguration,
        tile::{Checkpoint, Position},
    },
    runner::runner::Runner,
};
use models::{MazerConfiguration, MazerPosition, MazerRunResult};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mazer {
    maze: Maze,
}

#[wasm_bindgen]
impl Mazer {
    pub fn new(configuration: MazerConfiguration) -> Mazer {
        set_panic_hook();

        let maze = Maze::new(&MazeConfiguration {
            col_count: configuration.get_col_count(),
            row_count: configuration.get_row_count(),
            max_soft_wall_count: configuration.get_max_soft_wall_count(),
            walls: configuration
                .get_walls()
                .iter()
                .map(|pos| Position {
                    x: pos.get_x(),
                    y: pos.get_y(),
                })
                .collect(),
            entrypoints: configuration
                .get_entrypoints()
                .iter()
                .map(|pos| Position {
                    x: pos.get_x(),
                    y: pos.get_y(),
                })
                .collect(),
            checkpoints: configuration
                .get_checkpoints()
                .iter()
                .map(|checkpoint| {
                    let pos = checkpoint.get_position();
                    Checkpoint {
                        position: Position {
                            x: pos.get_x(),
                            y: pos.get_y(),
                        },
                        level: checkpoint.get_level(),
                    }
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
            .map(|pos| Position {
                x: pos.get_x(),
                y: pos.get_y(),
            })
            .collect();

        let runner = Runner::new(&self.maze);
        let result = runner.run(&walls).unwrap();

        return result.map(|run| {
            MazerRunResult::new(
                run.get_score(),
                run.get_solved_path()
                    .iter()
                    .cloned()
                    .map(|Position { x, y }| MazerPosition::new(x, y))
                    .collect(),
            )
        });
    }
}
