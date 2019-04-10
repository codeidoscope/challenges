use crate::board::Board;

pub fn populate_board(board: &Board, board_string: String) {
    let split_board_string = board_string.split_whitespace();

    for (i, symbol) in split_board_string.enumerate() {
        board.mark_with_symbol(&symbol.to_string(), i + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_populates_a_board_when_given_a_string() {
        let board = Board::new(3);
        let board_string = String::from(
            "X 2 O \
             4 5 6 \
             7 8 9",
        );

        populate_board(&board, board_string);
        assert_eq!(board.tiles[0].symbol.borrow_mut().to_string(), "X");
        assert_eq!(board.tiles[1].symbol.borrow_mut().to_string(), "2");
        assert_eq!(board.tiles[2].symbol.borrow_mut().to_string(), "O");
    }
}
