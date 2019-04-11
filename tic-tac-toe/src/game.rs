use crate::board::format_board;
use crate::board::Board;
use crate::game_rules::GameRules;
use crate::players::Player;
use std::cell::Cell;

pub struct Game {
    board: Board,
    game_rules: GameRules,
    current_player: Box<Player>,
    opponent: Box<Player>,
    current_player_move: Cell<usize>,
}

impl Game {
    pub fn new(
        game_board: Board,
        rules: GameRules,
        player_one: Box<Player>,
        player_two: Box<Player>,
    ) -> Self {
        let board = game_board;
        let game_rules = rules;
        let current_player = player_one;
        let opponent = player_two;
        let current_player_move = Cell::new(0);

        Self {
            board,
            game_rules,
            current_player,
            opponent,
            current_player_move,
        }
    }

    fn swap_players(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.opponent);
    }

    pub fn run(&mut self) {
        print!("\n{}\n", format_board(&self.board));
        while self.game_rules.get_status(
            &self.board,
            self.current_player.get_symbol(),
            self.opponent.get_symbol(),
        ) == "IN_PROGRESS"
        {
            self.play_turn();
        }
    }

    fn play_turn(&mut self) {
        let empty_positions = self.board.get_empty_tiles_by_user_position();
        let player_symbol = self.current_player.get_symbol().to_string();
        let opponent_symbol = self.opponent.get_symbol().to_string();
        let player_move = self.current_player.get_move(
            self.board.clone(),
            &self.game_rules,
            &player_symbol,
            &opponent_symbol,
            1,
        );
        let opponent_symbol = self.opponent.get_symbol().to_string();

        self.set_current_player_move(player_move);
        self.board.mark_with_symbol(&player_symbol, &player_move);
        print!("\n{}\n", format_board(&self.board));
        println!(
            "\n{}",
            self.game_rules.get_status_string(
                &self.board,
                &player_symbol,
                opponent_symbol,
                self.current_player_move.get()
            )
        );
        self.swap_players();
    }

    fn set_current_player_move(&mut self, player_move: usize) {
        self.current_player_move = Cell::new(player_move);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::players::Computer;
    use crate::players::Human;

    #[test]
    fn it_swaps_two_players() {
        let board = Board::new(3);
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();
        let mut game = Game::new(
            board,
            game_rules,
            Box::new(player_one),
            Box::new(player_two),
        );

        assert_eq!(game.current_player.get_symbol(), "X");
        assert_eq!(game.opponent.get_symbol(), "O");

        game.swap_players();

        assert_eq!(game.current_player.get_symbol(), "O");
        assert_eq!(game.opponent.get_symbol(), "X");
    }
}
