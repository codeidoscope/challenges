mod board;

fn main() {
    let board = board::Board::new(3);
    println!("{}", board::format_board(board))
}