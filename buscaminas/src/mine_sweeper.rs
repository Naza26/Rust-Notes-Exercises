use crate::grid::Grid;

/// A cell from a grid is represented here.
#[derive(Debug)]
pub enum Cell {
    /// A cell that contains a bomb.
    Bomb,
    /// A cell that contains the number of nearby bombs.
    Common(u32),
}

/// A MineSwepper grid is represented here.
pub struct MineSweeper {
    /// This grid represents the gameboard of the MineSweeper.
    grid: Grid,
}

impl MineSweeper {
    /// Returns a MineSweeper gameboard
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the path for the file containing the grid of the MineSweeper
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// ```
    pub fn new(path: &str) -> Self {
        let grid: Grid = Grid::from_path(path);

        Self { grid }
    }

    /// Returns a Grid representing the MineSweeper board containing all nearby bombs
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// let grid = mine_sweeper.mines_counting();
    /// ```
    pub fn mines_counting(&self) -> Grid {
        let mut raw_grid: Vec<Vec<Cell>> = Vec::new();
        for i in 0..self.row_range() {
            let mut r: Vec<Cell> = Vec::new();
            for j in 0..self.col_range() {
                let cell: Cell = match self.grid.get_cell(i, j) {
                    Cell::Bomb => Cell::Bomb,
                    Cell::Common(_) => Cell::Common(self.get_close_mines(i, j)),
                };
                r.push(cell);
            }
            raw_grid.push(r);
        }
        Grid::new(raw_grid)
    }

    /// Returns the row range of the grid
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// let row_range = mine_sweeper.row_range();
    /// ```
    pub fn row_range(&self) -> usize {
        self.grid.width()
    }

    /// Returns the col range of the grid
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// let col_range = mine_sweeper.col_range();
    /// ```
    pub fn col_range(&self) -> usize {
        self.grid.height()
    }

    /// Returns a boolean in case a cell contains a bomb
    ///
    /// # Arguments
    ///
    /// * `row_index` - An index representing the location of the row
    /// * `col_index` - An index representing the location of the column
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// let is_bomb_cell = mine_sweeper.is_bomb(0, 0);
    /// ```
    fn is_bomb(&self, row_index: usize, col_index: usize) -> bool {
        match self.grid.get_cell(row_index, col_index) {
            Cell::Bomb => true,
            Cell::Common(_) => false,
        }
    }

    /// Returns the amount of mines near a given cell
    ///
    /// # Arguments
    ///
    /// * `i` - An index representing the location of the row
    /// * `j` - An index representing the location of the column
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// let amount_of_mines = mine_sweeper.get_close_mines(0, 0);
    /// ```
    fn get_close_mines(&self, i: usize, j: usize) -> u32 {
        let mut counter = 0;

        let initial_row: usize = if i == 0 { 0 } else { i - 1 };
        let end_row: usize = if i + 1 >= self.row_range() {
            self.row_range() - 1
        } else {
            i + 1
        };
        let initial_col: usize = if j == 0 { 0 } else { j - 1 };
        let end_col: usize = if j + 1 >= self.col_range() {
            self.col_range() - 1
        } else {
            j + 1
        };

        for i in initial_row..=end_row {
            for j in initial_col..=end_col {
                if self.is_bomb(i, j) {
                    counter += 1;
                }
            }
        }
        counter
    }

    /// Returns an actual representation of the state of the grid
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use mine_sweeper::MineSweeper;
    /// let mine_sweeper = MineSweeper::new("buscaminas.txt");
    /// let grid = mine_sweeper.get_current_grid_state();
    /// ```
    pub fn get_current_grid_state(&self) -> &Grid {
        &self.grid
    }
}

#[cfg(test)]
mod mind_sweeper_tests {
    use super::Cell;
    use super::MineSweeper;

    // lo pase a test de integracion
    // #[test]
    // fn mine_sweping_gets_created_within_initial_grid() {
    //     let filepath: &str = "./buscaminas.txt";
    //     let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

    //     let expected_grid: Vec<&str> = vec![
    //         "0", "*", "0", "*", "0", "0", "0", "*", "0", "0", "0", "0", "*", "0", "0", "0", "0",
    //         "0", "0", "0",
    //     ];

    //     let current_grid = mine_sweeper.grid;

    //     let flattened_curent_grid: Vec<String> = current_grid
    //         .grid
    //         .iter()
    //         .flatten()
    //         .map(|element| match element {
    //             Cell::Bomb => "*".to_string(),
    //             Cell::Common(value) => value.to_string(),
    //         })
    //         .collect();
    //     assert_eq!(expected_grid, flattened_curent_grid)
    // }

    #[test]
    fn mine_sweeper_can_get_row_range() {
        let filepath: &str = "./buscaminas.txt";
        let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

        assert_eq!(mine_sweeper.row_range(), 4);
    }

    #[test]
    fn mine_sweeper_can_get_col_range() {
        let filepath: &str = "./buscaminas.txt";
        let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

        assert_eq!(mine_sweeper.col_range(), 5);
    }

    #[test]
    fn mine_sweeper_can_detect_bombs() {
        let filepath: &str = "./buscaminas.txt";
        let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

        assert!(mine_sweeper.is_bomb(0, 1));
    }

    #[test]
    fn mine_sweeper_can_detect_common_cells() {
        let filepath: &str = "./buscaminas.txt";
        let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

        assert!(!mine_sweeper.is_bomb(0, 0));
    }

    #[test]
    fn mine_sweeper_can_get_amount_of_close_mine() {
        let filepath: &str = "./buscaminas.txt";
        let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

        let amount_of_close_mines: u32 = mine_sweeper.get_close_mines(0, 0);

        assert_eq!(amount_of_close_mines, 1);
    }

    // lo mismo, lo pase a test de integracion
    // #[test]
    // fn mine_sweeper_can_count_all_near_mines() {
    //     let filepath: &str = "./buscaminas.txt";
    //     let mine_sweeper: MineSweeper = MineSweeper::new(filepath);

    //     let expected_grid: Vec<&str> = vec![
    //         "1", "*", "3", "*", "1", "1", "3", "*", "3", "1", "0", "2", "*", "2", "0", "0", "1",
    //         "1", "1", "0",
    //     ];

    //     let current_grid = mine_sweeper.mines_counting();

    //     let flattened_curent_grid: Vec<String> = current_grid
    //         .grid
    //         .iter()
    //         .flatten()
    //         .map(|element| match element {
    //             Cell::Bomb => "*".to_string(),
    //             Cell::Common(value) => value.to_string(),
    //         })
    //         .collect();
    //     assert_eq!(expected_grid, flattened_curent_grid)
    // }
}
