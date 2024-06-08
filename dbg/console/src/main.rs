use engine::{
    core::maze::{Maze, MazeOptions},
    runner::runner::Runner,
};

fn main() {
    let maze = Maze::new(&MazeOptions {
        col_count: 7,
        row_count: 2,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![(0, 0)],
        checkpoints: vec![
            ((0, 1), 1),
            ((2, 0), 2),
            ((5, 0), 3),
            ((4, 0), 3),
            ((6, 0), 4),
        ],
    })
    .unwrap();

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();

    println!("{:?}", result.0);

    let maze = Maze::new(&MazeOptions {
        col_count: 23,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![(0, 0)],
        checkpoints: vec![((1, 1), 19), ((2, 1), 20), ((10, 20), 23), ((20, 1), 21)],
    })
    .unwrap();

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner
        .run(&vec![(19, 1), (21, 1), (19, 0), (20, 2)])
        .unwrap()
        .unwrap();
    println!("{:?}", result.0);

    let maze = Maze::new(&MazeOptions {
        col_count: 23,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![(0, 0)],
        checkpoints: vec![((1, 1), 19), ((2, 1), 20), ((10, 20), 20), ((20, 1), 21)],
    })
    .unwrap();

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner
        .run(&vec![(19, 1), (21, 1), (19, 0), (20, 2)])
        .unwrap()
        .unwrap();
    println!("{:?}", result.0);

    let maze = Maze::new(&MazeOptions {
        col_count: 210,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![(0, 0)],
        checkpoints: vec![
            ((4, 5), 1),
            ((150, 20), 2),
            ((1, 1), 3),
            ((160, 20), 4),
            ((1, 2), 5),
            ((10, 25), 6),
            ((10, 21), 6),
            ((3, 3), 7),
            ((120, 25), 8),
            ((4, 4), 9),
            ((130, 25), 10),
            ((0, 1), 10),
            ((200, 5), 11),
            ((1, 21), 12),
            ((6, 6), 13),
            ((120, 24), 14),
            ((7, 7), 15),
            ((8, 19), 16),
            ((8, 8), 17),
            ((150, 19), 18),
            ((200, 1), 19),
            ((202, 1), 20),
            ((1, 20), 20),
            ((206, 1), 21),
        ],
    })
    .unwrap();

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner
        .run(&vec![(205, 1), (207, 1), (206, 0), (205, 2)])
        .unwrap()
        .unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![(7, 6)]).unwrap().unwrap();
    println!("{:?}", result.0);

    let maze = Maze::new(&MazeOptions {
        col_count: 210,
        row_count: 26,
        max_soft_wall_count: 200,
        walls: vec![],
        entrypoints: vec![(0, 0), (3, 4)],
        checkpoints: vec![
            ((4, 5), 1),
            ((150, 20), 2),
            ((1, 1), 3),
            ((160, 20), 4),
            ((1, 2), 5),
            ((10, 25), 6),
            ((10, 21), 6),
            ((3, 3), 7),
            ((120, 25), 8),
            ((4, 4), 9),
            ((130, 25), 10),
            ((0, 1), 10),
            ((200, 5), 11),
            ((1, 21), 12),
            ((6, 6), 13),
            ((120, 24), 14),
            ((7, 7), 15),
            ((8, 19), 16),
            ((8, 8), 17),
            ((150, 19), 18),
            ((200, 1), 19),
            ((202, 1), 20),
            ((1, 20), 20),
            ((206, 1), 21),
        ],
    })
    .unwrap();

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![]).unwrap().unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner
        .run(&vec![(205, 1), (207, 1), (206, 0), (205, 2)])
        .unwrap()
        .unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner.run(&vec![(7, 6)]).unwrap().unwrap();
    println!("{:?}", result.0);

    let runner = Runner::new(&maze);
    let result = runner
        .run(&vec![
            (2, 0),
            (2, 1),
            (2, 3),
            (2, 4),
            (2, 5),
            (2, 6),
            (4, 7),
            (4, 6),
            (4, 3),
            (4, 2),
        ])
        .unwrap()
        .unwrap();
    println!("{:?}", result.0);
}
