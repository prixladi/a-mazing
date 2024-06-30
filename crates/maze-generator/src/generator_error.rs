use std::error::Error;

use maze_core::MazeError;
use maze_runner::MazeRunnerError;

#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Aggregated error occurred: {0}")]
    AggregatedError(Box<dyn Error>),
}

impl From<MazeError> for GeneratorError {
    fn from(value: MazeError) -> Self {
        Self::AggregatedError(Box::new(value))
    }
}

impl From<MazeRunnerError> for GeneratorError {
    fn from(value: MazeRunnerError) -> Self {
        Self::AggregatedError(Box::new(value))
    }
}
