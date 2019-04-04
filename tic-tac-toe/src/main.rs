use crate::players::Human;
use crate::players::Player;

mod board;
mod players;
mod ui;
mod game;

fn main() {
    let board = board::Board::new(3);
    let player_one = Human::new("X".to_string());
    player_one.get_move();
}