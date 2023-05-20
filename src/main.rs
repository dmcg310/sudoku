use std::time::Instant;
mod board;

fn main() {
    let start_time = Instant::now();

    let mut board = board::Board::new();
    board.read_from_file("board.txt");
    board.print_board();
    board.solver();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("\nTime: {:?}", elapsed_time);
}
