use crate::board::Board;
use crate::players::Player;
use crate::board::Tile;
use crate::board::format_board;
use std::cell::RefCell;
use crate::board::print_tiles;
use core::borrow::BorrowMut;

pub struct Game {
    board: Board,
    status: String,
    current_player: Box<Player>,
    opponent: Box<Player>,
    current_player_move: u32,
}

impl Game {
    pub fn new(game_board: Board, player_one: Box<Player>, player_two: Box<Player>) -> Self {
        let board = game_board;
        let status = String::from("IN_PROGRESS");
        let current_player = player_one;
        let opponent = player_two;
        let current_player_move = 0;

        Self { board, status, current_player, opponent, current_player_move }
    }

    fn swap_players(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.opponent);
    }

    pub fn run(&mut self) {
        while self.status == "IN_PROGRESS" {
            print!("{}", format_board(&self.board));
            self.play_turn();
        }
    }

    fn play_turn(&mut self) {
        let player_symbol = self.current_player.get_symbol().to_string();
        let player_move = self.current_player.get_move();
        self.board.mark_with_symbol(player_symbol, player_move);
        self.swap_players();
    }

    pub fn are_symbols_aligned<'board>(&self, board_section: &mut impl Iterator<Item=&'board Tile>, symbol: String) -> bool {
        board_section.all(|tile| tile.symbol.borrow_mut().to_string() == RefCell::new(symbol.clone()).borrow_mut().to_string())
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

    #[test]
    fn it_returns_true_if_three_symbols_are_the_same_in_a_section_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol("X".to_string(), 1);
        board.mark_with_symbol("X".to_string(), 5);
        board.mark_with_symbol("X".to_string(), 9);
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game = Game::new(board, Box::new(player_one), Box::new(player_two));
        let right_diagonal = &mut game.board.get_right_diagonal();

        assert_eq!(game.are_symbols_aligned(right_diagonal, "[X] ".to_string()), true)
    }

    #[test]
    fn it_returns_false_if_three_symbols_are_not_the_same_in_a_section_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol("X".to_string(), 1);
        board.mark_with_symbol("X".to_string(), 5);
        board.mark_with_symbol("O".to_string(), 9);
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game = Game::new(board, Box::new(player_one), Box::new(player_two));
        let right_diagonal = &mut game.board.get_right_diagonal();

        assert_eq!(game.are_symbols_aligned(right_diagonal, "[X] ".to_string()), false)
    }
}