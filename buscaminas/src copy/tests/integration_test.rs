use super::Cell;
use super::MineSweeper;

// importing common module.
mod common;

// #[test]
// fn test_add() {
//     // using common code.
//     common::setup();
//     assert_eq!(adder::add(3, 2), 5);
// }

#[test]
fn mine_sweeper_gets_created_with_initial_grid() {
    let mine_sweeper = common::setup();

    let expected_grid: common::initial_grid_state();

    let current_grid = mine_sweeper;

    let flattened_current_grid: Vec<String> = common::flatten_grid(current_grid);

    assert_eq!(expected_grid, flattened_current_grid)
}

#[test]
fn mine_sweeper_can_count_all_near_mines() {
    let mine_sweeper = common::setup();

    let expected_grid: Vec<&str> = common::final_grid_state();

    let current_grid = mine_sweeper.mines_counting();

    let flattened_current_grid: Vec<String> = common::flatten_grid(current_grid.grid);

    assert_eq!(expected_grid, flattened_current_grid)
}