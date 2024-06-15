use maze_core::{Checkpoint, Maze, MazeConfig, Position};
use maze_generator::{create_generator, GeneratorType};
use maze_runner::MazeRunner;

fn main() {
    generator();
    runner();
}

fn generator() {
    let generator = create_generator(GeneratorType::Vanilla);

    let config = generator.generate().unwrap();

    let maze = Maze::new(&config).unwrap();

    let runner = MazeRunner::new(&maze);

    let result = runner.run(&vec![]).unwrap().unwrap();

    println!("{:?}", result.get_solved_path());
}

#[allow(dead_code)]
fn runner() {
    let maze = Maze::new(&MazeConfig {
        col_count: 7,
        row_count: 2,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![Position { x: 0, y: 0 }],
        checkpoints: vec![
            Checkpoint {
                position: Position { x: 0, y: 1 },
                level: 1,
            },
            Checkpoint {
                position: Position { x: 2, y: 0 },
                level: 2,
            },
            Checkpoint {
                position: Position { x: 5, y: 0 },
                level: 3,
            },
            Checkpoint {
                position: Position { x: 4, y: 0 },
                level: 3,
            },
            Checkpoint {
                position: Position { x: 6, y: 0 },
                level: 4,
            },
        ],
    })
    .unwrap();

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();

    println!("{:?}", result.get_score());

    let maze = Maze::new(&MazeConfig {
        col_count: 23,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![Position { x: 0, y: 0 }],
        checkpoints: vec![
            Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 19,
            },
            Checkpoint {
                position: Position { x: 2, y: 1 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 10, y: 20 },
                level: 23,
            },
            Checkpoint {
                position: Position { x: 20, y: 1 },
                level: 21,
            },
        ],
    })
    .unwrap();

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner
        .run(&vec![
            Position { x: 19, y: 1 },
            Position { x: 21, y: 1 },
            Position { x: 19, y: 0 },
            Position { x: 20, y: 2 },
        ])
        .unwrap()
        .unwrap();
    println!("{:?}", result.get_score());

    let maze = Maze::new(&MazeConfig {
        col_count: 23,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![Position { x: 0, y: 0 }],
        checkpoints: vec![
            Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 19,
            },
            Checkpoint {
                position: Position { x: 2, y: 1 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 10, y: 20 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 20, y: 1 },
                level: 21,
            },
        ],
    })
    .unwrap();

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner
        .run(&vec![
            Position { x: 19, y: 1 },
            Position { x: 21, y: 1 },
            Position { x: 19, y: 0 },
            Position { x: 20, y: 2 },
        ])
        .unwrap()
        .unwrap();
    println!("{:?}", result.get_score());

    let maze = Maze::new(&MazeConfig {
        col_count: 210,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![Position { x: 0, y: 0 }],
        checkpoints: vec![
            Checkpoint {
                position: Position { x: 4, y: 5 },
                level: 1,
            },
            Checkpoint {
                position: Position { x: 150, y: 20 },
                level: 2,
            },
            Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 3,
            },
            Checkpoint {
                position: Position { x: 160, y: 20 },
                level: 4,
            },
            Checkpoint {
                position: Position { x: 1, y: 2 },
                level: 5,
            },
            Checkpoint {
                position: Position { x: 10, y: 25 },
                level: 6,
            },
            Checkpoint {
                position: Position { x: 10, y: 21 },
                level: 6,
            },
            Checkpoint {
                position: Position { x: 3, y: 3 },
                level: 7,
            },
            Checkpoint {
                position: Position { x: 120, y: 25 },
                level: 8,
            },
            Checkpoint {
                position: Position { x: 4, y: 4 },
                level: 9,
            },
            Checkpoint {
                position: Position { x: 130, y: 25 },
                level: 10,
            },
            Checkpoint {
                position: Position { x: 0, y: 1 },
                level: 10,
            },
            Checkpoint {
                position: Position { x: 200, y: 5 },
                level: 11,
            },
            Checkpoint {
                position: Position { x: 1, y: 21 },
                level: 12,
            },
            Checkpoint {
                position: Position { x: 6, y: 6 },
                level: 13,
            },
            Checkpoint {
                position: Position { x: 120, y: 24 },
                level: 14,
            },
            Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 15,
            },
            Checkpoint {
                position: Position { x: 8, y: 19 },
                level: 16,
            },
            Checkpoint {
                position: Position { x: 8, y: 8 },
                level: 17,
            },
            Checkpoint {
                position: Position { x: 150, y: 19 },
                level: 18,
            },
            Checkpoint {
                position: Position { x: 200, y: 1 },
                level: 19,
            },
            Checkpoint {
                position: Position { x: 202, y: 1 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 1, y: 20 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 206, y: 1 },
                level: 21,
            },
        ],
    })
    .unwrap();

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner
        .run(&vec![
            Position { x: 205, y: 1 },
            Position { x: 207, y: 1 },
            Position { x: 206, y: 0 },
            Position { x: 205, y: 2 },
        ])
        .unwrap()
        .unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![Position { x: 7, y: 6 }]).unwrap().unwrap();
    println!("{:?}", result.get_score());

    let maze = Maze::new(&MazeConfig {
        col_count: 210,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![Position { x: 0, y: 0 }, Position { x: 3, y: 4 }],
        checkpoints: vec![
            Checkpoint {
                position: Position { x: 4, y: 5 },
                level: 1,
            },
            Checkpoint {
                position: Position { x: 150, y: 20 },
                level: 2,
            },
            Checkpoint {
                position: Position { x: 1, y: 1 },
                level: 3,
            },
            Checkpoint {
                position: Position { x: 160, y: 20 },
                level: 4,
            },
            Checkpoint {
                position: Position { x: 1, y: 2 },
                level: 5,
            },
            Checkpoint {
                position: Position { x: 10, y: 25 },
                level: 6,
            },
            Checkpoint {
                position: Position { x: 10, y: 21 },
                level: 6,
            },
            Checkpoint {
                position: Position { x: 3, y: 3 },
                level: 7,
            },
            Checkpoint {
                position: Position { x: 120, y: 25 },
                level: 8,
            },
            Checkpoint {
                position: Position { x: 4, y: 4 },
                level: 9,
            },
            Checkpoint {
                position: Position { x: 130, y: 25 },
                level: 10,
            },
            Checkpoint {
                position: Position { x: 0, y: 1 },
                level: 10,
            },
            Checkpoint {
                position: Position { x: 200, y: 5 },
                level: 11,
            },
            Checkpoint {
                position: Position { x: 1, y: 21 },
                level: 12,
            },
            Checkpoint {
                position: Position { x: 6, y: 6 },
                level: 13,
            },
            Checkpoint {
                position: Position { x: 120, y: 24 },
                level: 14,
            },
            Checkpoint {
                position: Position { x: 7, y: 7 },
                level: 15,
            },
            Checkpoint {
                position: Position { x: 8, y: 19 },
                level: 16,
            },
            Checkpoint {
                position: Position { x: 8, y: 8 },
                level: 17,
            },
            Checkpoint {
                position: Position { x: 150, y: 19 },
                level: 18,
            },
            Checkpoint {
                position: Position { x: 200, y: 1 },
                level: 19,
            },
            Checkpoint {
                position: Position { x: 202, y: 1 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 1, y: 20 },
                level: 20,
            },
            Checkpoint {
                position: Position { x: 206, y: 1 },
                level: 21,
            },
        ],
    })
    .unwrap();

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner
        .run(&vec![
            Position { x: 205, y: 1 },
            Position { x: 207, y: 1 },
            Position { x: 206, y: 0 },
            Position { x: 205, y: 2 },
        ])
        .unwrap()
        .unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner.run(&vec![Position { x: 7, y: 6 }]).unwrap().unwrap();
    println!("{:?}", result.get_score());

    let runner = MazeRunner::new(&maze);
    let result = runner
        .run(&vec![
            Position { x: 2, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 3 },
            Position { x: 2, y: 4 },
            Position { x: 2, y: 5 },
            Position { x: 2, y: 6 },
            Position { x: 4, y: 7 },
            Position { x: 4, y: 6 },
            Position { x: 4, y: 3 },
            Position { x: 4, y: 2 },
        ])
        .unwrap()
        .unwrap();
    println!("{:?}", result.get_score());
}
