use std::error::Error;

use maze_core::MazeError;
use maze_runner::MazeRunnerError;

#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Aggregated error occurred: {0}")]
    AggregatedError(Box<dyn Error>),
}

impl GeneratorError {
    pub(crate) fn from_maze_error(maze_error: MazeError) -> Self {
        Self::AggregatedError(Box::new(maze_error))
    }

    pub(crate) fn from_runner_error(runner_error: MazeRunnerError) -> Self {
        Self::AggregatedError(Box::new(runner_error))
    }
}
