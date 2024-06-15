use std::ops::Range;

use maze_core::MazeConfig;

use crate::GeneratorError;

use super::{
    helpers::{
        get_checkpoints, get_empty_positions_with_padding, get_left_wall_positions,
        get_random_number_in_range, get_random_positions, get_random_solvable_walls,
        get_right_wall_positions,
    },
    MazeGenerator,
};

pub(crate) struct VanillaGenerator;

const COL_COUNT: usize = 20;
const ROW_COUNT: usize = 10;
const ENTRYPOINT_RANGE: Range<usize> = 1..4;
const CHECKPOINT_RANGE: Range<usize> = 2..4;
const EXIT_RANGE: Range<usize> = 1..4;
const WALL_RANGE: Range<usize> = 10..21;
const MAX_SOFT_WALL_RANGE: Range<u32> = 15..26;

impl VanillaGenerator {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl MazeGenerator for VanillaGenerator {
    fn generate(&self) -> Result<MazeConfig, GeneratorError> {
        let entrypoint_count = get_random_number_in_range(ENTRYPOINT_RANGE);
        let checkpoint_count = get_random_number_in_range(CHECKPOINT_RANGE);
        let exit_count = get_random_number_in_range(EXIT_RANGE);
        let wall_count = get_random_number_in_range(WALL_RANGE);
        let max_soft_wall_count = get_random_number_in_range(MAX_SOFT_WALL_RANGE);

        let entrypoint_positions =
            get_random_positions(&get_left_wall_positions(ROW_COUNT), entrypoint_count);
        let exit_positions =
            get_random_positions(&get_right_wall_positions(COL_COUNT, ROW_COUNT), exit_count);

        let empty_positions = get_empty_positions_with_padding(
            COL_COUNT,
            ROW_COUNT,
            2,
            0,
            &[&entrypoint_positions, &exit_positions][..],
        );

        let checkpoint_positions = get_random_positions(&empty_positions, checkpoint_count);

        let empty_positions = get_empty_positions_with_padding(
            COL_COUNT,
            ROW_COUNT,
            1,
            0,
            &[
                &entrypoint_positions,
                &exit_positions,
                &checkpoint_positions,
            ][..],
        );

        let checkpoints = get_checkpoints(&checkpoint_positions, &exit_positions);

        let mut config = MazeConfig {
            col_count: COL_COUNT,
            row_count: ROW_COUNT,
            max_soft_wall_count: wall_count as u32,
            entrypoints: entrypoint_positions,
            checkpoints,
            walls: vec![],
        };

        let walls = get_random_solvable_walls(&config, &empty_positions, wall_count)?;

        config.max_soft_wall_count = max_soft_wall_count;
        config.walls = walls;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vanilla_generator() {
        let generator = VanillaGenerator::new();
        for _ in 0..5 {
            let config = generator.generate();
            assert!(config.is_ok());
        }
    }
}
