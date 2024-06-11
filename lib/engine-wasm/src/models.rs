use crate::utils::set_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MazerConfiguration {
    col_count: usize,
    row_count: usize,
    max_soft_wall_count: u32,
    checkpoints: Vec<MazerCheckpoint>,
    entrypoints: Vec<MazerPosition>,
    walls: Vec<MazerPosition>,
}

#[wasm_bindgen]
impl MazerConfiguration {
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
    pub fn get_col_count(&self) -> usize {
        self.col_count
    }

    #[wasm_bindgen(getter, js_name = rowCount)]
    pub fn get_row_count(&self) -> usize {
        self.row_count
    }

    #[wasm_bindgen(getter, js_name = maxSoftWallCount)]
    pub fn get_max_soft_wall_count(&self) -> u32 {
        self.max_soft_wall_count
    }

    #[wasm_bindgen(getter, js_name = entrypoints)]
    pub fn get_entrypoints(&self) -> Vec<MazerPosition> {
        self.entrypoints.clone()
    }

    #[wasm_bindgen(getter, js_name = checkpoints)]
    pub fn get_checkpoints(&self) -> Vec<MazerCheckpoint> {
        self.checkpoints.clone()
    }

    #[wasm_bindgen(getter, js_name = walls)]
    pub fn get_walls(&self) -> Vec<MazerPosition> {
        self.walls.clone()
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

    #[wasm_bindgen(getter, js_name = score)]
    pub fn get_score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen(getter, js_name = path)]
    pub fn get_path(&self) -> Vec<MazerPosition> {
        self.path.clone()
    }
}

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

    #[wasm_bindgen(getter, js_name = x)]
    pub fn get_x(&self) -> usize {
        self.x
    }

    #[wasm_bindgen(getter, js_name = y)]
    pub fn get_y(&self) -> usize {
        self.y
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

    #[wasm_bindgen(getter, js_name = position)]
    pub fn get_position(&self) -> MazerPosition {
        self.position.clone()
    }

    #[wasm_bindgen(getter, js_name = level)]
    pub fn get_level(&self) -> i32 {
        self.level
    }
}
