use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // read file line by line
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut v: Vec<String> = Vec::new();
    let mut number_of_rows: i32 = 0;
    let mut number_of_cols: i32 = 0;

    for line in reader.lines() {
        v.push(line?);
        number_of_rows += 1;
    }

    get_column_range(&v, &mut number_of_cols, number_of_rows);

    println!("nro filas: {}", &number_of_rows);
    println!("nro columnas: {}", &number_of_cols);

    let mut grid_raw: Vec<i32> = vec![0; (number_of_rows * number_of_cols).try_into().unwrap()];
    let mut grid_base: Vec<_> = grid_raw
        .as_mut_slice()
        .chunks_mut(number_of_cols.try_into().unwrap())
        .collect();
    let grid = grid_base.as_mut_slice();

    setting_bombs(&v, grid);

    let fila: usize = number_of_rows.try_into().unwrap();
    let col: usize = number_of_cols.try_into().unwrap();

    for i in 0..fila {
        for j in 0..col {
            if !hay_bomba(grid, i, j) {
                let amount_of_mines = get_close_mines(&i, &j, grid);
                grid[i][j] = amount_of_mines;
            }
        }
    }
    println!("{:?}", map_grid(grid, fila, col));

    Ok(())
}

fn hay_bomba(grid: &mut [&mut [i32]], indice_fila: usize, indice_columna: usize) -> bool {
    grid[indice_fila][indice_columna] == -1
}

fn setting_bombs(v: &[String], grid: &mut [&mut [i32]]) {
    for (row_index, element) in v.iter().enumerate() {
        for (col_index, character) in element.chars().enumerate() {
            if character == '*' {
                grid[row_index][col_index] = -1;
            }
        }
    }
}

fn map_grid(grid: &mut [&mut [i32]], fila: usize, col: usize) -> Vec<Vec<String>> {
    let mut columna: Vec<Vec<String>> = Vec::new();
    for grid_row in grid.iter().take(fila) {
        let mut fila: Vec<String> = Vec::new();
        for grid_element in grid_row.iter().take(col) {
            if *grid_element == -1 {
                fila.push("*".to_string());
            } else {
                fila.push(grid_element.to_string());
            }
        }
        columna.push(fila);
    }
    columna
}

fn get_column_range(v: &Vec<String>, number_of_cols: &mut i32, number_of_rows: i32) {
    for e in v {
        for _ in e.chars().enumerate() {
            *number_of_cols += 1;
        }
    }
    *number_of_cols /= number_of_rows;
}

fn get_close_mines(fila: &usize, columna: &usize, grid: &mut [&mut [i32]]) -> i32 {
    let mut conteo = 0;

    let fila_inicio: usize = if fila <= &0 { 0 } else { fila - 1 };
    let fila_fin: usize = if fila + 1 >= 4 { 4 - 1 } else { fila + 1 };
    let columna_inicio: usize = if columna <= &0 { 0 } else { columna - 1 };
    let columna_fin: usize = if columna + 1 >= 5 { 5 - 1 } else { columna + 1 };

    for i in fila_inicio..=fila_fin {
        for j in columna_inicio..=columna_fin {
            if hay_bomba(grid, i, j) {
                conteo += 1;
            }
        }
    }
    conteo
}

fn main() {
    let filepath = "./buscaminas.txt";
    let _result: Result<(), Box<dyn Error>> = read_file(filepath);
    // Los 0 van a representar que no toca ninguna bomba
    // Los -1 van a representar cada bomba
    // Despues podria mappear cada valor numerico al string correspondiente
}
