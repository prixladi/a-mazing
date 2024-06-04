use engine::{
    core::board::{Board, BoardCreationOptions},
    runner::runner::Runner,
};

fn main() {
    let board = Board::new(&BoardCreationOptions {
        col_count: 8,
        row_count: 8,
        max_soft_wall_count: 200,
        walls: vec![],
        entrances: vec![(0, 0)],
        exits: vec![(7, 7)],
    })
    .unwrap();

    let runner = Runner::new(&board, &vec![]).unwrap();
    let result = runner.run().unwrap();
    println!("{:?}", result);

    let runner = Runner::new(&board, &vec![(7, 6)]).unwrap();
    let result = runner.run().unwrap();
    println!("{:?}", result);

    let runner = Runner::new(
        &board,
        &vec![
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (2, 6),
            (4, 7),
            (4, 6),
            (4, 5),
            (4, 4),
            (4, 3),
            (4, 2),
        ],
    )
    .unwrap();
    let result = runner.run().unwrap();
    println!("{:?}", result);
}
