use crate::players::Human;
use crate::game::Game;
use crate::game_rules::GameRules;

mod board;
mod players;
mod ui;
mod game;
mod game_rules;

fn main() {
    let board = board::Board::new(3);
    let player_one = Human::new("X".to_string());
    let player_two = Human::new("O".to_string());
    let game_rules = GameRules::new();
    let mut game = Game::new(board, game_rules, Box::new(player_one), Box::new(player_two));
    game.run();
}