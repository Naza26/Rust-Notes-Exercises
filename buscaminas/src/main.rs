mod mine_sweeper;
use mine_sweeper::MineSweeper;

fn main() {
    let filepath: &str = "./buscaminas.txt";
    let mine_sweeper: MineSweeper = MineSweeper::new(filepath);
    println!("Initial grid given file: ");
    println!("{}", mine_sweeper.get_current_grid_state());
    println!("Grid filled in with near bombs: ");
    println!("{}", mine_sweeper.mines_counting()); 
}
