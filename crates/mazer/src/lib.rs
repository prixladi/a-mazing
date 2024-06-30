mod models;
mod utils;

use maze_core::Maze;
use maze_generator::create_generator;
use maze_runner::MazeRunner;

use models::{MazerConfig, MazerGeneratorType, MazerPosition, MazerRunResult};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mazer {
    maze: Maze,
}

#[wasm_bindgen]
impl Mazer {
    #[wasm_bindgen]
    pub fn new(config: MazerConfig) -> Mazer {
        set_panic_hook();

        let maze = Maze::new(&config.into()).unwrap();

        Self { maze }
    }

    #[wasm_bindgen]
    pub fn run(&self, soft_walls: Vec<MazerPosition>) -> Option<MazerRunResult> {
        let walls = soft_walls.into_iter().map(|pos| pos.into()).collect();

        let runner = MazeRunner::new(&self.maze);
        let result = runner.run(&walls).unwrap();
        result.map(|run| run.into())
    }

    #[wasm_bindgen(js_name = generateConfig)]
    pub fn generate_config(generator_type: MazerGeneratorType) -> MazerConfig {
        set_panic_hook();

        let generator = create_generator(generator_type.into());
        generator.generate().unwrap().into()
    }
}
