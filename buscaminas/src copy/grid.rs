use core::fmt;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::mine_sweeper::Cell;

/// A Grid is represented here.
pub struct Grid {
    /// This grid represents the gameboard of the MineSweeper.
    pub grid: Vec<Vec<Cell>>,
}

impl Grid {
    /// Returns a grid built as 2D vec of cells
    ///
    /// # Arguments
    ///
    /// * `raw_grid` - A raw grid representing the MineSweeper
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use buscaminas::grid::Grid;
    /// use buscaminas::mine_sweeper::Cell;
    /// let grid = Grid::new(vec![vec![Cell::Common(0), Cell::Bomb]]);
    /// ```
    pub fn new(raw_grid: Vec<Vec<Cell>>) -> Self {
        Self { grid: raw_grid }
    }
    /// Returns a grid built as 2D vec of cells with the initial state of the MineSweeper
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file from which the Grid will be built upon
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use buscaminas::grid::Grid;
    /// let grid = Grid::from_path("buscaminas.txt");
    /// ```
    pub fn from_path(path: &str) -> Result<Self, io::Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("{:?}", error);
                return Err(error);
            }
        };
        let reader: BufReader<File> = BufReader::new(file);

        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for line in reader.lines() {
            grid.push(
                line.unwrap()
                    .into_bytes()
                    .iter()
                    .map(|value| match value {
                        b'*' => Cell::Bomb,
                        _ => Cell::Common(0),
                    })
                    .collect(),
            );
        }

        Ok(Self { grid })
    }
    /// Returns the height of the grid
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use buscaminas::grid::Grid;
    /// use buscaminas::mine_sweeper::Cell;
    /// let grid = Grid::new(vec![vec![Cell::Common(0), Cell::Bomb]]);
    /// let grid_height = grid.height();
    /// ```
    pub fn height(&self) -> usize {
        self.grid[0].len()
    }

    /// Returns the width of the grid
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use buscaminas::grid::Grid;
    /// use buscaminas::mine_sweeper::Cell;
    /// let grid = Grid::new(vec![vec![Cell::Common(0), Cell::Bomb]]);
    /// let grid_width = grid.width();
    /// ```
    pub fn width(&self) -> usize {
        self.grid.len()
    }

    /// Returns cell of the grid given the row and col indexes
    ///
    /// # Arguments
    ///
    /// * `row_index` - An index representing the row position of the grid
    /// * `col_index` - An index representing the column position of the grid
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use buscaminas::grid::Grid;
    /// use buscaminas::mine_sweeper::Cell;
    /// let grid = Grid::new(vec![vec![Cell::Common(0), Cell::Bomb]]);
    /// let cell = grid.get_cell(0,0);
    /// ```
    pub fn get_cell(&self, row_index: usize, col_index: usize) -> &Cell {
        &self.grid[row_index][col_index]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Bomb => "*".to_string(),
                    Cell::Common(close_bombs) => close_bombs.to_string(),
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
