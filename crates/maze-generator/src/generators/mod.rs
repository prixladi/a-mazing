use maze_core::MazeConfiguration;

use crate::GeneratorError;

mod helpers;

pub(crate) mod vanilla;
pub(crate) mod waterfall;

pub trait MazeGenerator {
    fn generate(&self) -> Result<MazeConfiguration, GeneratorError>;
}
