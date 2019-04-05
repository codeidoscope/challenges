use crate::players::Human;
use crate::game::Game;

mod board;
mod players;
mod ui;
mod game;

fn main() {
    let board = board::Board::new(3);
    let player_one = Human::new("X".to_string());
    let player_two = Human::new("O".to_string());
    let mut game = Game::new(board, Box::new(player_one), Box::new(player_two));
    game.run();
}