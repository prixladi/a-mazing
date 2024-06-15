mod mappers;
mod models;
mod utils;

use maze_core::Maze;
use maze_generator::create_generator;
use maze_runner::MazeRunner;

use mappers::{
    from_mazer_config, from_mazer_generator_type, from_mazer_position, to_mazer_config,
    to_mazer_run_result,
};
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

        let maze = Maze::new(&from_mazer_config(&config)).unwrap();

        Self { maze }
    }

    #[wasm_bindgen]
    pub fn run(&self, soft_walls: Vec<MazerPosition>) -> Option<MazerRunResult> {
        let walls = soft_walls.iter().map(from_mazer_position).collect();

        let runner = MazeRunner::new(&self.maze);
        let result = runner.run(&walls).unwrap();

        result.map(|run| to_mazer_run_result(&run))
    }

    #[wasm_bindgen(js_name = generateConfig)]
    pub fn generate_config(generator_type: MazerGeneratorType) -> MazerConfig {
        set_panic_hook();

        let generator = create_generator(from_mazer_generator_type(generator_type));
        let config = generator.generate().unwrap();

        to_mazer_config(&config)
    }
}
