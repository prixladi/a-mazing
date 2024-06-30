mod generator_error;
mod generator_type;
mod generators;

use generators::{vanilla::VanillaGenerator, waterfall::WaterfallGenerator, MazeGenerator};

pub use generator_error::*;
pub use generator_type::*;

pub fn create_generator(generator_type: MazeGeneratorType) -> Box<dyn MazeGenerator> {
    match generator_type {
        MazeGeneratorType::Vanilla => Box::new(VanillaGenerator::new()),
        MazeGeneratorType::Waterfall => Box::new(WaterfallGenerator::new()),
    }
}
