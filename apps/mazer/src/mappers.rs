use maze_core::{Checkpoint, MazeConfig, Position};
use maze_generator::GeneratorType;
use maze_runner::MazeRunResult;

use crate::models::{
    MazerCheckpoint, MazerConfig, MazerGeneratorType, MazerPosition, MazerRunResult,
};

pub(crate) fn to_mazer_position(position: &Position) -> MazerPosition {
    MazerPosition::new(position.x, position.y)
}

pub(crate) fn from_mazer_position(position: &MazerPosition) -> Position {
    Position {
        x: position.get_x(),
        y: position.get_y(),
    }
}

pub(crate) fn to_mazer_checkpoint(checkpoint: &Checkpoint) -> MazerCheckpoint {
    MazerCheckpoint::new(to_mazer_position(&checkpoint.position), checkpoint.level)
}

pub(crate) fn from_mazer_checkpoint(checkpoint: &MazerCheckpoint) -> Checkpoint {
    Checkpoint {
        position: from_mazer_position(&checkpoint.get_position()),
        level: checkpoint.get_level(),
    }
}

pub(crate) fn to_mazer_config(config: &MazeConfig) -> MazerConfig {
    MazerConfig::new(
        config.col_count,
        config.row_count,
        config.max_soft_wall_count,
        config.entrypoints.iter().map(to_mazer_position).collect(),
        config.checkpoints.iter().map(to_mazer_checkpoint).collect(),
        config.walls.iter().map(to_mazer_position).collect(),
    )
}

pub(crate) fn from_mazer_config(config: &MazerConfig) -> MazeConfig {
    MazeConfig {
        col_count: config.get_col_count(),
        row_count: config.get_row_count(),
        max_soft_wall_count: config.get_max_soft_wall_count(),
        entrypoints: config
            .get_entrypoints()
            .iter()
            .map(from_mazer_position)
            .collect(),
        checkpoints: config
            .get_checkpoints()
            .iter()
            .map(from_mazer_checkpoint)
            .collect(),
        walls: config.get_walls().iter().map(from_mazer_position).collect(),
    }
}

pub(crate) fn to_mazer_run_result(run_result: &MazeRunResult) -> MazerRunResult {
    MazerRunResult::new(
        run_result.get_score(),
        run_result
            .get_solved_path()
            .iter()
            .map(to_mazer_position)
            .collect(),
    )
}

pub(crate) fn from_mazer_generator_type(generator_type: MazerGeneratorType) -> GeneratorType {
    match generator_type {
        MazerGeneratorType::Vanilla => GeneratorType::Vanilla,
        MazerGeneratorType::Waterfall => GeneratorType::Waterfall,
    }
}
