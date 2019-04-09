use crate::game::Game;
use crate::game_rules::GameRules;
use crate::players::Computer;
use crate::players::Human;

mod board;
mod game;
mod game_rules;
mod players;
mod ui;
mod test_helpers;

fn main() {
    let board = board::Board::new(3);
    let player_one = Human::new("X".to_string());
    let player_two = Computer::new("O".to_string());
    let game_rules = GameRules::new();
    let mut game = Game::new(
        board,
        game_rules,
        Box::new(player_one),
        Box::new(player_two),
    );
    game.run();
}
