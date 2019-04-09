use crate::board::Board;
use crate::board::Tile;
use std::cell::RefCell;

pub struct GameRules {}

impl GameRules {
    pub fn new() -> Self {
        Self {}
    }

    fn are_symbols_aligned<'board>(
        &self,
        board_section: &mut impl Iterator<Item = &'board Tile>,
        symbol: String,
    ) -> bool {
        board_section.all(|tile| {
            tile.symbol.borrow_mut().to_string()
                == RefCell::new(symbol.clone()).borrow_mut().to_string()
        })
    }

    fn is_winning_row_or_column<'board>(
        &self,
        sections: impl Iterator<Item = impl Iterator<Item = &'board Tile>>,
        symbol: String,
    ) -> bool {
        let mut result = Vec::new();
        for mut section in sections {
            result.push(self.are_symbols_aligned(&mut section, symbol.clone()));
        }

        result.contains(&true)
    }

    fn is_winning_diagonal<'board>(
        &self,
        right_diagonal: &mut impl Iterator<Item = &'board Tile>,
        left_diagonal: &mut impl Iterator<Item = &'board Tile>,
        symbol: String,
    ) -> bool {
        let right_diagonal_is_winning = self.are_symbols_aligned(right_diagonal, symbol.clone());
        let left_diagonal_is_winning = self.are_symbols_aligned(left_diagonal, symbol.clone());

        right_diagonal_is_winning || left_diagonal_is_winning
    }

    fn is_winner(&self, board: &Board, symbol: &String) -> bool {
        let row_is_winning = self.is_winning_row_or_column(board.get_rows(), symbol.clone());
        let column_is_winning = self.is_winning_row_or_column(board.get_columns(), symbol.clone());
        let diagonal_is_winning = self.is_winning_diagonal(
            &mut board.get_right_diagonal(),
            &mut board.get_left_diagonal(),
            symbol.clone(),
        );

        row_is_winning || column_is_winning || diagonal_is_winning
    }

    fn is_full(&self, board: &Board) -> bool {
        let mut result = Vec::new();
        for tile in &board.tiles {
            let tile_symbol = tile.symbol.borrow_mut().to_string();

            if tile_symbol == "X" || tile_symbol == "O" {
                result.push(true)
            } else {
                result.push(false)
            }
        }
        result.iter().all(|item| item == &true)
    }

    pub fn get_status(
        &self,
        board: &Board,
        current_player_symbol: &String,
        opponent_symbol: &String,
    ) -> String {
        let game_board = board;
        if self.is_winner(&game_board, &current_player_symbol) {
            format!("PLAYER_{}_WINS", current_player_symbol)
        } else if self.is_winner(&game_board, &opponent_symbol) {
            format!("PLAYER_{}_WINS", opponent_symbol)
        } else if self.is_full(&game_board) {
            "DRAW".to_string()
        } else {
            "IN_PROGRESS".to_string()
        }
    }

    pub fn get_status_string(
        &self,
        board: &Board,
        current_player_symbol: &String,
        opponent_symbol: String,
        current_player_move: usize,
    ) -> String {
        let status = self.get_status(&board, &current_player_symbol, &opponent_symbol);

        if status == format!("PLAYER_{}_WINS", &current_player_symbol) {
            format!("Player {} wins!", &current_player_symbol)
        } else if status == format!("PLAYER_{}_WINS", &opponent_symbol) {
            format!("Player {} wins!", &opponent_symbol)
        } else if status == "DRAW" {
            "It's a draw :(".to_string()
        } else {
            format!(
                "Player {} played in position {}",
                current_player_symbol, current_player_move
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::players::Computer;
    use crate::players::Human;

    #[test]
    fn it_returns_true_if_three_symbols_are_the_same_in_a_section_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"X".to_string(), 9);
        let right_diagonal = &mut board.get_right_diagonal();
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.are_symbols_aligned(right_diagonal, "X".to_string()),
            true
        )
    }

    #[test]
    fn it_returns_false_if_three_symbols_are_not_the_same_in_a_section_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 9);
        let right_diagonal = &mut board.get_right_diagonal();
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.are_symbols_aligned(right_diagonal, "X".to_string()),
            false
        )
    }

    #[test]
    fn it_returns_true_if_three_symbols_are_aligned_on_the_board_by_rows() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 2);
        board.mark_with_symbol(&"X".to_string(), 3);
        let board_rows = board.get_rows();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_row_or_column(board_rows, "X".to_string()),
            true
        )
    }

    #[test]
    fn it_returns_false_if_three_symbols_are_not_aligned_on_the_board_by_rows() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 2);
        board.mark_with_symbol(&"O".to_string(), 3);
        let board_rows = board.get_rows();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_row_or_column(board_rows, "X".to_string()),
            false
        )
    }

    #[test]
    fn it_returns_true_if_three_symbols_are_aligned_on_the_board_by_columns() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 4);
        board.mark_with_symbol(&"X".to_string(), 7);
        let board_columns = board.get_columns();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_row_or_column(board_columns, "X".to_string()),
            true
        )
    }

    #[test]
    fn it_returns_false_if_three_symbols_are_not_aligned_on_the_board_by_columns() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 4);
        board.mark_with_symbol(&"O".to_string(), 7);
        let board_columns = board.get_columns();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_row_or_column(board_columns, "X".to_string()),
            false
        )
    }

    #[test]
    fn it_returns_true_if_three_symbols_are_aligned_on_the_right_diagonal_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"X".to_string(), 9);
        let mut right_board_diagonal = board.get_right_diagonal();
        let mut left_board_diagonal = board.get_left_diagonal();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_diagonal(
                &mut right_board_diagonal,
                &mut left_board_diagonal,
                "X".to_string()
            ),
            true
        )
    }

    #[test]
    fn it_returns_false_if_three_symbols_are_not_aligned_on_the_right_diagonal_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 9);
        let mut right_board_diagonal = board.get_right_diagonal();
        let mut left_board_diagonal = board.get_left_diagonal();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_diagonal(
                &mut right_board_diagonal,
                &mut left_board_diagonal,
                "X".to_string()
            ),
            false
        )
    }

    #[test]
    fn it_returns_true_if_three_symbols_are_aligned_on_the_left_diagonal_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"X".to_string(), 7);
        let mut right_board_diagonal = board.get_right_diagonal();
        let mut left_board_diagonal = board.get_left_diagonal();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_diagonal(
                &mut right_board_diagonal,
                &mut left_board_diagonal,
                "X".to_string()
            ),
            true
        )
    }

    #[test]
    fn it_returns_false_if_three_symbols_are_not_aligned_on_the_left_diagonal_of_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 7);
        let mut right_board_diagonal = board.get_right_diagonal();
        let mut left_board_diagonal = board.get_left_diagonal();

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.is_winning_diagonal(
                &mut right_board_diagonal,
                &mut left_board_diagonal,
                "X".to_string()
            ),
            false
        )
    }

    #[test]
    fn it_returns_true_if_there_if_three_symbols_are_aligned_on_the_board() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"X".to_string(), 7);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(game_rules.is_winner(&board, &"X".to_string()), true)
    }

    #[test]
    fn it_returns_true_if_there_if_three_symbols_are_not_aligned_on_the_board() {
        let board = Board::new(3);
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(game_rules.is_winner(&board, &"X".to_string()), false)
    }

    #[test]
    fn it_return_true_if_the_board_is_full() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 1);
        board.mark_with_symbol(&"O".to_string(), 2);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"O".to_string(), 4);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 6);
        board.mark_with_symbol(&"X".to_string(), 7);
        board.mark_with_symbol(&"O".to_string(), 8);
        board.mark_with_symbol(&"X".to_string(), 9);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(game_rules.is_full(&board), true);
    }

    #[test]
    fn it_return_false_if_the_board_is_not_full() {
        let board = Board::new(3);
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string());
        let game_rules = GameRules::new();

        assert_eq!(game_rules.is_full(&board), false);
    }

    #[test]
    fn it_returns_player_o_wins_status_if_player_o_has_won() {
        let board = Board::new(3);
        board.mark_with_symbol(&"O".to_string(), 3);
        board.mark_with_symbol(&"O".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 7);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.get_status(&board, &player_one_symbol, &player_two_symbol),
            "PLAYER_O_WINS".to_string()
        )
    }

    #[test]
    fn it_returns_player_x_wins_status_if_player_x_has_won() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"X".to_string(), 7);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.get_status(&board, &player_one_symbol, &player_two_symbol),
            "PLAYER_X_WINS".to_string()
        )
    }

    #[test]
    fn it_returns_in_progress_status_if_the_game_is_not_over() {
        let board = Board::new(3);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.get_status(&board, &player_one_symbol, &player_two_symbol),
            "IN_PROGRESS".to_string()
        )
    }

    #[test]
    fn it_returns_draw_status_if_the_board_is_full() {
        let board = Board::new(3);
        board.mark_with_symbol(&"O".to_string(), 1);
        board.mark_with_symbol(&"O".to_string(), 2);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 4);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 6);
        board.mark_with_symbol(&"O".to_string(), 7);
        board.mark_with_symbol(&"X".to_string(), 8);
        board.mark_with_symbol(&"X".to_string(), 9);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();

        assert_eq!(
            game_rules.get_status(&board, &player_one_symbol, &player_two_symbol),
            "DRAW".to_string()
        )
    }

    #[test]
    fn it_returns_player_x_wins_string_when_player_x_is_winner() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"X".to_string(), 7);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();
        let current_player_move = 7;

        assert_eq!(
            game_rules.get_status_string(
                &board,
                &player_one_symbol,
                player_two_symbol,
                current_player_move
            ),
            "Player X wins!"
        )
    }

    #[test]
    fn it_returns_player_o_wins_string_when_player_o_is_winner() {
        let board = Board::new(3);
        board.mark_with_symbol(&"O".to_string(), 3);
        board.mark_with_symbol(&"O".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 7);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();
        let current_player_move = 7;

        assert_eq!(
            game_rules.get_status_string(
                &board,
                &player_one_symbol,
                player_two_symbol,
                current_player_move
            ),
            "Player O wins!"
        )
    }

    #[test]
    fn it_returns_it_is_a_draw_string_when_there_is_no_winner() {
        let board = Board::new(3);
        board.mark_with_symbol(&"O".to_string(), 1);
        board.mark_with_symbol(&"O".to_string(), 2);
        board.mark_with_symbol(&"X".to_string(), 3);
        board.mark_with_symbol(&"X".to_string(), 4);
        board.mark_with_symbol(&"X".to_string(), 5);
        board.mark_with_symbol(&"O".to_string(), 6);
        board.mark_with_symbol(&"O".to_string(), 7);
        board.mark_with_symbol(&"X".to_string(), 8);
        board.mark_with_symbol(&"X".to_string(), 9);

        let player_one_symbol = Human::new("X".to_string()).symbol;
        let player_two_symbol = Computer::new("O".to_string()).symbol;
        let game_rules = GameRules::new();
        let current_player_move = 7;

        assert_eq!(
            game_rules.get_status_string(
                &board,
                &player_one_symbol,
                player_two_symbol,
                current_player_move
            ),
            "It's a draw :("
        )
    }
}
