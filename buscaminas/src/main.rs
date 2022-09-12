use buscaminas::mine_sweeper::MineSweeper;

fn main() {
    let filepath: &str = "./buscaminas.txt";
    let mine_sweeper: MineSweeper =
        MineSweeper::new(filepath).expect("Program could not be executed");
    println!("Initial grid given file: ");
    println!("{}", mine_sweeper.get_current_grid_state());
    println!("Grid filled in with nearby bombs: ");
    println!("{}", mine_sweeper.mines_counting());
}
