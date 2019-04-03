use crate::human_player::HumanPlayer;
use crate::board::format_board;
use crate::board::Board;
use std::mem;

pub struct Game {
    board: Board,
    status: String,
    current_player: HumanPlayer,
    opponent: HumanPlayer,
}

impl Game {
    pub fn new(game_board: Board, player_one: HumanPlayer, player_two: HumanPlayer) -> Self {
        let board = game_board;
        let status = String::from("IN_PROGRESS");
        let current_player = player_one;
        let opponent = player_two;

        Self { board, status, current_player, opponent }
    }

    fn swap_players(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.opponent);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::human_player::HumanPlayer;

    #[test]
    fn it_swaps_two_players() {
        let board = Board::new(3);
        let player_one = HumanPlayer::new("X".to_string());
        let player_two = HumanPlayer::new("O".to_string());
        let mut game = Game::new(board, player_one, player_two);

        assert_eq!(game.current_player.symbol, "X");
        assert_eq!(game.opponent.symbol, "O");

        game.swap_players();

        assert_eq!(game.current_player.symbol, "O");
        assert_eq!(game.opponent.symbol, "X");
    }
}