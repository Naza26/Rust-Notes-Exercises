use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn read_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> { // read file line by line
    let file: File = File::open(filepath)?;
    let reader:BufReader<File> = BufReader::new(file);
    let mut v: Vec<String> = Vec::new();
    let mut number_of_rows: i32 = 0;
    let mut number_of_cols: i32 = 0;

    for line in reader.lines() {
        // println!("{}", line?);
        v.push(line?);
        number_of_rows = number_of_rows + 1;
    }

    obtener_rango_columna(&v, &mut number_of_cols, number_of_rows);

    println!("nro filas: {}", &number_of_rows);
    println!("nro columnas: {}", &number_of_cols);

    let mut grid_raw: Vec<i32> = vec![0; (number_of_rows * number_of_cols).try_into().unwrap()];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(number_of_cols.try_into().unwrap()).collect();
    let grid = grid_base.as_mut_slice();
    // grid[0][0] = 4;
    // println!("{:?}", &grid);

    setting_bombs(&v, grid);


    let mut row_index: usize = 0;
    for element in v {
        for (col_index, _caracter) in element.chars().enumerate() {
            println!("indice fila: {} indice columna: {} valor: {}", &row_index, &col_index, &grid[row_index][col_index]);
            println!("\n{:?}", &grid);

            if !hay_bomba(grid, row_index, col_index) && col_index == 0 && row_index == 0 {
                actualizo_cercania_bombas(grid, row_index, col_index -1);
            }
            if col_index + 1 >= number_of_cols.try_into().unwrap() {
                actualizo_cercania_bombas(grid, row_index, col_index -1);
            }
            if row_index + 1 >= number_of_rows.try_into().unwrap() {
                actualizo_cercania_bombas(grid, row_index - 1, col_index -1);
            }
            if !hay_bomba(grid, row_index, col_index) {
                actualizo_cercania_bombas(grid, row_index, col_index);
            }
            
        }
        // println!("{:?}", &grid);
        row_index = row_index + 1;
    }

    println!("{:?}", &grid);

    Ok(())
}

fn hay_bomba(grid: &mut [&mut [i32]], indice_fila: usize, indice_columna: usize) -> bool {
    grid[indice_fila][indice_columna] == -1
}

fn actualizo_cercania_bombas(grid: &mut [&mut [i32]], row_index: usize, col_index: usize) {
    if grid[row_index - 1][col_index] == -1 { // arriba
        grid[row_index][col_index] += 1;
    }
    if grid[row_index - 1][col_index + 1] == -1 { // diagonal derecha superior
        grid[row_index][col_index] += 1;
    }
    if grid[row_index][col_index + 1] == -1 { // derecha 
        grid[row_index][col_index] += 1;
    }
    if grid[row_index + 1][col_index + 1] == -1 { // diagonal derecha inferior
        grid[row_index][col_index] += 1;
    }
    if grid[row_index + 1][col_index] == -1 { // abajo
        grid[row_index][col_index] += 1;
    }
    if grid[row_index + 1][col_index - 1] == -1 { // diagonal izquierda superior
        grid[row_index][col_index] += 1;
    }
    if grid[row_index][col_index - 1] == -1 { // izquierda
        grid[row_index][col_index] += 1;
    }
    if grid[row_index - 1][col_index - 1] == -1 { // diagonal izquierda superior
        grid[row_index][col_index] += 1;
    }
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

fn obtener_rango_columna(v: &Vec<String>, number_of_cols: &mut i32, number_of_rows: i32) {
    for e in v  {
        for _ in e.chars().enumerate() {
            *number_of_cols = *number_of_cols + 1;
        }   
    }
    *number_of_cols = *number_of_cols / number_of_rows;
}


fn main() {
    let filepath: &str = "./buscaminas.txt";
    let _result: Result<(), Box<dyn Error>> = read_file(&filepath);
    // Los 0 van a representar que no toca ninguna bomba
    // Los -1 van a representar cada bomba
    // Despues podria mappear cada valor numerico al string correspondiente
}
