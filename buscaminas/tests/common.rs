use buscaminas::{
    grid::Grid,
    mine_sweeper::{Cell, MineSweeper},
};

pub fn setup() -> MineSweeper {
    let filepath: &str = "./buscaminas.txt";
    let mine_sweeper: MineSweeper = MineSweeper::new(filepath).unwrap();
    mine_sweeper
}

pub fn initial_grid_state() -> Vec<&'static str> {
    return vec![
        "0", "*", "0", "*", "0", "0", "0", "*", "0", "0", "0", "0", "*", "0", "0", "0", "0", "0",
        "0", "0",
    ];
}

pub fn final_grid_state() -> Vec<&'static str> {
    return vec![
        "1", "*", "3", "*", "1", "1", "3", "*", "3", "1", "0", "2", "*", "2", "0", "0", "1", "1",
        "1", "0",
    ];
}

pub fn flatten_grid(grid: &Grid) -> Vec<String> {
    grid.get_current_grid()
        .iter()
        .flatten()
        .map(|element| match element {
            Cell::Bomb => "*".to_string(),
            Cell::Common(value) => value.to_string(),
        })
        .collect()
}
