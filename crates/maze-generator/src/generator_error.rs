use std::error::Error;

use maze_core::MazeError;
use maze_runner::RunnerError;

#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Internal error occurred: {0}")]
    InternalError(Box<dyn Error>),
}

impl GeneratorError {
    pub(crate) fn from_maze_error(maze_error: MazeError) -> Self {
        Self::InternalError(Box::new(maze_error))
    }

    pub(crate) fn from_runner_error(runner_error: RunnerError) -> Self {
        Self::InternalError(Box::new(runner_error))
    }
}
