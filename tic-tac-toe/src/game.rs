use crate::board::Board;
use crate::players::Player;

pub struct Game {
    board: Board,
    status: String,
    current_player: Box<Player>,
    opponent: Box<Player>,
}

impl Game {
    pub fn new(game_board: Board, player_one: Box<Player>, player_two: Box<Player>) -> Self {
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
    use crate::players::Human;
    use crate::players::Computer;

    #[test]
    fn it_swaps_two_players() {
        let board = Board::new(3);
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let mut game = Game::new(board, Box::new(player_one), Box::new(player_two));

        assert_eq!(game.current_player.get_symbol(), "X");
        assert_eq!(game.opponent.get_symbol(), "O");

        game.swap_players();

        assert_eq!(game.current_player.get_symbol(), "O");
        assert_eq!(game.opponent.get_symbol(), "X");
    }
}