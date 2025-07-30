use crate::utils::set_panic_hook;

use maze_core::{Checkpoint, MazeConfig, Position};
use maze_generator::MazeGeneratorType;
use maze_runner::MazeRunResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct MazerPosition {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl MazerPosition {
    #[wasm_bindgen]
    pub fn new(x: usize, y: usize) -> Self {
        set_panic_hook();
        Self { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> usize {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> usize {
        self.y
    }
}

impl From<Position> for MazerPosition {
    fn from(position: Position) -> Self {
        MazerPosition::new(position.x, position.y)
    }
}

impl From<MazerPosition> for Position {
    fn from(val: MazerPosition) -> Self {
        Position {
            x: val.x,
            y: val.y,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct MazerCheckpoint {
    position: MazerPosition,
    level: i32,
}

#[wasm_bindgen]
impl MazerCheckpoint {
    #[wasm_bindgen]
    pub fn new(position: MazerPosition, level: i32) -> Self {
        set_panic_hook();
        Self { position, level }
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> MazerPosition {
        self.position.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> i32 {
        self.level
    }
}

impl From<Checkpoint> for MazerCheckpoint {
    fn from(checkpoint: Checkpoint) -> Self {
        MazerCheckpoint::new(checkpoint.position.into(), checkpoint.level)
    }
}

impl From<MazerCheckpoint> for Checkpoint {
    fn from(val: MazerCheckpoint) -> Self {
        Checkpoint {
            position: val.position.into(),
            level: val.level,
        }
    }
}

#[wasm_bindgen]
pub struct MazerConfig {
    col_count: usize,
    row_count: usize,
    max_soft_wall_count: u32,
    entrypoints: Vec<MazerPosition>,
    checkpoints: Vec<MazerCheckpoint>,
    walls: Vec<MazerPosition>,
}

#[wasm_bindgen]
impl MazerConfig {
    #[wasm_bindgen]
    pub fn new(
        col_count: usize,
        row_count: usize,
        max_soft_wall_count: u32,
        entrypoints: Vec<MazerPosition>,
        checkpoints: Vec<MazerCheckpoint>,
        walls: Vec<MazerPosition>,
    ) -> Self {
        set_panic_hook();
        Self {
            col_count,
            row_count,
            max_soft_wall_count,
            walls,
            entrypoints,
            checkpoints,
        }
    }

    #[wasm_bindgen(getter, js_name = colCount)]
    pub fn col_count(&self) -> usize {
        self.col_count
    }

    #[wasm_bindgen(getter, js_name = rowCount)]
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    #[wasm_bindgen(getter, js_name = maxSoftWallCount)]
    pub fn max_soft_wall_count(&self) -> u32 {
        self.max_soft_wall_count
    }

    #[wasm_bindgen(getter, js_name = entrypoints)]
    pub fn entrypoints(&self) -> Vec<MazerPosition> {
        self.entrypoints.clone()
    }

    #[wasm_bindgen(getter, js_name = checkpoints)]
    pub fn checkpoints(&self) -> Vec<MazerCheckpoint> {
        self.checkpoints.clone()
    }

    #[wasm_bindgen(getter, js_name = walls)]
    pub fn walls(&self) -> Vec<MazerPosition> {
        self.walls.clone()
    }
}

impl From<MazeConfig> for MazerConfig {
    fn from(config: MazeConfig) -> Self {
        MazerConfig::new(
            config.col_count,
            config.row_count,
            config.max_soft_wall_count,
            config.entrypoints.into_iter().map(|x| x.into()).collect(),
            config.checkpoints.into_iter().map(|x| x.into()).collect(),
            config.walls.into_iter().map(|x| x.into()).collect(),
        )
    }
}

impl From<MazerConfig> for MazeConfig {
    fn from(val: MazerConfig) -> Self {
        MazeConfig {
            col_count: val.col_count,
            row_count: val.row_count,
            max_soft_wall_count: val.max_soft_wall_count,
            entrypoints: val.entrypoints().into_iter().map(|x| x.into()).collect(),
            checkpoints: val.checkpoints().into_iter().map(|x| x.into()).collect(),
            walls: val.walls().into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[wasm_bindgen]
pub struct MazerRunResult {
    score: u32,
    path: Vec<MazerPosition>,
}

#[wasm_bindgen]
impl MazerRunResult {
    pub fn new(score: u32, path: Vec<MazerPosition>) -> Self {
        Self { score, path }
    }

    #[wasm_bindgen(getter)]
    pub fn score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Vec<MazerPosition> {
        self.path.clone()
    }
}

impl From<MazeRunResult> for MazerRunResult {
    fn from(result: MazeRunResult) -> Self {
        MazerRunResult::new(
            result.score(),
            result.solved_path().into_iter().map(|x| x.into()).collect(),
        )
    }
}

#[wasm_bindgen]
pub enum MazerGeneratorType {
    Vanilla,
    Waterfall,
}

impl From<MazerGeneratorType> for MazeGeneratorType {
    fn from(val: MazerGeneratorType) -> Self {
        match val {
            MazerGeneratorType::Vanilla => MazeGeneratorType::Vanilla,
            MazerGeneratorType::Waterfall => MazeGeneratorType::Waterfall,
        }
    }
}
