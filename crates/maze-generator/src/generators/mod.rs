use maze_core::MazeConfig;

use crate::GeneratorError;

mod helpers;

pub(crate) mod vanilla;
pub(crate) mod waterfall;

pub trait MazeGenerator {
    fn generate(&self) -> Result<MazeConfig, GeneratorError>;
}
