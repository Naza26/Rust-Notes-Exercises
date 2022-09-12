use buscaminas::{grid::Grid, mine_sweeper::MineSweeper};

// importing common module.
mod common;

#[test]
fn mine_sweeper_gets_created_with_initial_grid() {
    let mine_sweeper: MineSweeper = common::setup();

    let expected_grid: Vec<&str> = common::initial_grid_state();

    let current_grid: &Grid = mine_sweeper.get_current_grid_state();

    let flattened_current_grid: Vec<String> = common::flatten_grid(current_grid);

    assert_eq!(expected_grid, flattened_current_grid)
}

#[test]
fn mine_sweeper_can_count_all_near_mines() {
    let mine_sweeper: MineSweeper = common::setup();

    let expected_grid: Vec<&str> = common::final_grid_state();

    let current_grid: Grid = mine_sweeper.mines_counting();

    let flattened_current_grid: Vec<String> = common::flatten_grid(&current_grid);

    assert_eq!(expected_grid, flattened_current_grid)
}
