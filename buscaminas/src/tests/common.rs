pub fn setup() -> MineSweeper {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    let filepath: &str = "./buscaminas.txt";
    let mine_sweeper: MineSweeper = MineSweeper::new(filepath);
    mine_sweeper
}

pub fn initial_grid_state() -> Vec<&str> {
    vec![
        "0","*","0","*","0",
        "0","0","*","0","0",
        "0","0","*","0","0",
        "0","0","0","0","0"
    ];
}

pub fn final_grid_state() -> Vec<&str> {
    vec![
        "1","*","3","*","1",
        "1","3","*","3","1",
        "0","2","*","2","0",
        "0","1","1","1","0",
    ];
}

pub fn flatten_grid(grid: Grid) -> Vec<String> {
    grid.iter().flatten().map(|element| match element {
        Cell::Bomb => "*".to_string(),
        Cell::Common(value) => value.to_string(),
    }).collect();
}