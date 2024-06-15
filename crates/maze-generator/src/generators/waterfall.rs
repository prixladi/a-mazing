use std::ops::Range;

use maze_core::MazeConfiguration;

use crate::GeneratorError;

use super::{
    helpers::{
        get_bottom_wall_positions, get_checkpoints, get_empty_positions_with_padding,
        get_random_number_in_range, get_random_positions, get_random_solvable_walls,
        get_top_wall_positions,
    },
    MazeGenerator,
};

pub(crate) struct WaterfallGenerator;

const COL_COUNT: usize = 10;
const ROW_COUNT: usize = 15;
const CHECKPOINT_RANGE: Range<usize> = 3..4;
const WALL_RANGE: Range<usize> = 10..16;
const MAX_SOFT_WALL_RANGE: Range<u32> = 10..21;

impl WaterfallGenerator {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl MazeGenerator for WaterfallGenerator {
    fn generate(&self) -> Result<MazeConfiguration, GeneratorError> {
        let checkpoint_count = get_random_number_in_range(CHECKPOINT_RANGE);
        let wall_count = get_random_number_in_range(WALL_RANGE);
        let max_soft_wall_count = get_random_number_in_range(MAX_SOFT_WALL_RANGE);

        let entrypoint_positions = get_top_wall_positions(COL_COUNT, ROW_COUNT);

        let exit_positions = get_bottom_wall_positions(COL_COUNT);

        let empty_positions = get_empty_positions_with_padding(
            COL_COUNT,
            ROW_COUNT,
            0,
            2,
            &[&entrypoint_positions, &exit_positions][..],
        );

        let checkpoint_positions = get_random_positions(&empty_positions, checkpoint_count);

        let empty_positions = get_empty_positions_with_padding(
            COL_COUNT,
            ROW_COUNT,
            0,
            1,
            &[
                &entrypoint_positions,
                &exit_positions,
                &checkpoint_positions,
            ][..],
        );

        let checkpoints = get_checkpoints(&checkpoint_positions, &exit_positions);

        let mut config = MazeConfiguration {
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
    fn test_waterfall_generator() {
        let generator = WaterfallGenerator::new();
        for _ in 0..5 {
            let configuration = generator.generate();
            assert!(configuration.is_ok());
        }
    }
}