mod utils;

use std::vec;

use engine::{
    core::maze::{Maze, MazeOptions},
    runner::runner::Runner,
};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen]
    pub fn new(x: usize, y: usize) -> Self {
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
pub struct RunResult {
    distance: u32,
    path: Vec<Position>,
}

#[wasm_bindgen]
impl RunResult {
    #[wasm_bindgen(getter, js_name = distance)]
    pub fn get_distance(&self) -> u32 {
        self.distance
    }

    #[wasm_bindgen(getter, js_name = path)]
    pub fn get_path(&self) -> Vec<Position> {
        self.path.clone()
    }
}

#[wasm_bindgen]
pub struct Mazer {
    maze: Maze,
}

#[wasm_bindgen]
impl Mazer {
    pub fn new() -> Option<Mazer> {
        set_panic_hook();
        let maze = Maze::new(&MazeOptions {
            col_count: 7,
            row_count: 2,
            max_soft_wall_count: 200,
            walls: vec![],
            entrances: vec![(0, 0)],
            checkpoints: vec![
                ((0, 1), 1),
                ((2, 0), 2),
                ((5, 0), 3),
                ((4, 0), 3),
                ((6, 0), 4),
            ],
        })
        .ok()?;

        Some(Self { maze })
    }

    #[wasm_bindgen]
    pub fn run(&self, soft_walls: Vec<Position>) -> Option<RunResult> {
        let walls = soft_walls
            .iter()
            .map(|pos| (pos.get_x(), pos.get_y()))
            .collect();
        
        let runner = Runner::new(&self.maze);
        let result = runner.run(&walls).unwrap();

        return result.map(|(distance, path)| RunResult {
            distance,
            path: path
                .iter()
                .cloned()
                .map(|(x, y)| Position::new(x, y))
                .collect(),
        });
    }
}
