use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn read_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> { // read file line by line
    let file: File = File::open(filepath)?;
    let reader:BufReader<File> = BufReader::new(file);
    let mut v: Vec<String> = Vec::new();
    let mut number_of_rows: i32 = 0;
    let mut number_of_cols: i32 = 0;

    for line in reader.lines() {
        v.push(line?);
        number_of_rows = number_of_rows + 1;
    }

    get_column_range(&v, &mut number_of_cols, number_of_rows);

    println!("nro filas: {}", &number_of_rows);
    println!("nro columnas: {}", &number_of_cols);

    let mut grid_raw: Vec<i32> = vec![0; (number_of_rows * number_of_cols).try_into().unwrap()];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(number_of_cols.try_into().unwrap()).collect();
    let grid = grid_base.as_mut_slice();

    setting_bombs(&v, grid);

    let mut row_index: usize = 0;

    let fila: usize = number_of_rows.try_into().unwrap();
    let col: usize = number_of_cols.try_into().unwrap();
    for i in 0..fila {
        for j in 0..col {
            //println!("indice fila: {} indice columna: {} valor: {}", &i, &j, &grid[i][j]);
            if !hay_bomba(grid, i, j) {
                let cantidad_minas = get_close_mines(&i, &j, grid);
                grid[i][j] = cantidad_minas;
            }
        }
        row_index = row_index + 1;
    }
    println!("{:?}", map_grid(grid, fila, col));

    Ok(())
}

fn hay_bomba(grid: &mut [&mut [i32]], indice_fila: usize, indice_columna: usize) -> bool {
    grid[indice_fila][indice_columna] == -1
}

fn setting_bombs(v: &Vec<String>, grid: &mut [&mut [i32]]) {
    let mut indice_fila: usize = 0;
    for element in v {
        for (index, caracter) in element.chars().enumerate() {
            if caracter == '*' {
                grid[indice_fila][index] = -1;
            }
        }
        indice_fila = indice_fila + 1;
    }
}

fn map_grid(grid: &mut [&mut [i32]], fila: usize, col: usize) -> Vec<Vec<String>> {
    let mut columna: Vec<Vec<String>> = Vec::new();
    for i in 0..fila {
        let mut fila: Vec<String> = Vec::new();
        for j in 0..col {
            if grid[i][j] == -1 {
                fila.push("*".to_string());   
            }
            else {
                fila.push(grid[i][j].to_string()); 
            }
        }
        columna.push(fila);
    }
    columna
}

fn get_column_range(v: &Vec<String>, number_of_cols: &mut i32, number_of_rows: i32) {
    for e in v  {
        for _ in e.chars().enumerate() {
            *number_of_cols = *number_of_cols + 1;
        }   
    }
    *number_of_cols = *number_of_cols / number_of_rows;
}


fn get_close_mines(fila: &usize, columna: &usize,  grid: &mut [&mut [i32]]) -> i32 {
    let mut fila_inicio = 0;
    let mut fila_fin = 0;
    let mut columna_inicio = 0;
    let mut columna_fin = 0;
    let mut conteo = 0;
    if fila <= &0 {
        fila_inicio = 0;
    } else {
        fila_inicio = fila - 1;
    }
    if fila + 1 >= 4 {
        fila_fin = 4 - 1;
    } else {
        fila_fin = fila + 1;
    }
    if columna <= &0 {
        columna_inicio = 0;
    } else {
        columna_inicio = columna - 1;
    }
    if columna + 1 >= 5 {
        columna_fin = 5 - 1;
    } else {
        columna_fin = columna + 1;
    }
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
    let filepath: &str = "./buscaminas.txt";
    let _result: Result<(), Box<dyn Error>> = read_file(&filepath);
    // Los 0 van a representar que no toca ninguna bomba
    // Los -1 van a representar cada bomba
    // Despues podria mappear cada valor numerico al string correspondiente
}
