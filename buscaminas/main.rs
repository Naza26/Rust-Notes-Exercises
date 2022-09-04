use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};


enum Cell {
    Bomb,
    Common(i32)
}

impl fmt::Display for MineSweeper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            write!(f, "{:?}\n", String::from_utf8(row.to_vec()).unwrap())?;
        }
        Ok(())
    }
}

struct MineSweeper {
    grid: Vec<Vec<u8>>
}

impl MineSweeper {
    pub fn new(path: &str) -> Self {
        let file: File = File::open(path).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        let mut grid: Vec<Vec<u8>> = Vec::new();

        for line in reader.lines() {
            grid.push(line.unwrap().into_bytes());
        }

        Self { grid }
    }

    pub fn mines_counting(&self) {
        for i in 0..self.get_row_range() {
            let mut r = Vec::new();
            for j in 0..self.get_col_range() {
                if !self.hay_bomba(i, j) {
                    let amount_of_mines = self.get_close_mines(i, j);
                    r.push(amount_of_mines.to_string());
                } else {
                    r.push("*".to_string());
                }
            }
            println!("{:?}", r.concat());
        }
    }

    pub fn get_row_range(&self) -> usize {
        self.grid.len()
    }

    pub fn get_col_range(&self) -> usize {
        self.grid[0].len()
    }

    fn hay_bomba(&self, indice_fila: usize, indice_columna: usize) -> bool {
        self.grid[indice_fila][indice_columna] == '*' as u8
    }

    fn get_close_mines(&self, i: usize, j:usize) -> i32 {
        let mut counter = 0;

        let fila_inicio: usize = if i <= 0 { 0 } else { i - 1 };
        let fila_fin: usize = if i + 1 >= 4 { 4 - 1 } else { i + 1 };
        let columna_inicio: usize = if j <= 0 { 0 } else { j - 1 };
        let columna_fin: usize = if j + 1 >= 5 { 5 - 1 } else { j + 1 };
    
        for i in fila_inicio..=fila_fin {
            for j in columna_inicio..=columna_fin {
                if self.hay_bomba(i, j) {
                    counter += 1;
                }
            }
        }
        counter
    }
}

fn main() {
    let filepath = "./buscaminas.txt";
    let mine_sweeper = MineSweeper::new(filepath);
    println!("{}", mine_sweeper);
    mine_sweeper.mines_counting()
}
