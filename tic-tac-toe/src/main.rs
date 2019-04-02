use crate::human_player::HumanPlayer;

mod board;
mod human_player;
mod ui;
mod game;

fn main() {
    let board = board::Board::new(3);
    let player_one: HumanPlayer = human_player::HumanPlayer::new("X".to_string());
    player_one.get_move();
}