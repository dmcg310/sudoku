// import board.rs
mod board;

fn main() {
    let mut board = board::Board::new();
    board.read_from_file("board.txt");
    board.print_board();
}
