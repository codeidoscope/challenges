use crate::game::Game;
use crate::game_rules::GameRules;
use crate::players::Computer;
use crate::players::Human;
use crate::board::format_board;
use crate::test_helpers::populate_board;
use crate::board::Board;
use std::collections::HashMap;
use crate::players::UnbeatableComputer;

mod board;
mod game;
mod game_rules;
mod players;
mod ui;
mod test_helpers;

fn main() {
    let board = Board::new(3);

    let player_one = Human::new("X".to_string());
    let player_two = UnbeatableComputer::new("O".to_string(), "X".to_string());
    let game_rules = GameRules::new();
    let mut game = Game::new(
        board,
        game_rules,
        Box::new(player_one),
        Box::new(player_two),
    );
    game.run();
}
