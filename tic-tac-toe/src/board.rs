use std::cell::RefCell;

#[derive(Debug)]
pub struct Tile {
    pub symbol: RefCell<String>,
    position: usize,
}

pub struct Board {
    size: usize,
    pub tiles: Vec<Tile>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();

        for i in 0..size * size {
            tiles.push(Tile {
                symbol: RefCell::new(format!("{}", i + 1)),
                position: i,
            })
        }

        Self { size, tiles }
    }

    pub fn get_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Tile>> {
        (0..self.size).map(move |row_index| {
            let row_start = row_index * self.size;
            let row_end = row_start + self.size;
            self.tiles[row_start..row_end].iter()
        })
    }

    pub fn get_columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Tile>> {
        (0..self.size).map(move |column_index| {
            (0..self.size).map(move |row_index| &self.tiles[row_index * self.size + column_index])
        })
    }

    pub fn get_right_diagonal(&self) -> impl Iterator<Item = &Tile> {
        (0..self.size).map(move |index| &self.tiles[index * self.size + index])
    }

    pub fn get_left_diagonal(&self) -> impl Iterator<Item = &Tile> {
        (0..self.size).map(move |index| &self.tiles[index * self.size + self.size - 1 - index])
    }

    pub fn mark_with_symbol(&self, symbol: &str, position: usize) {
        let position_index = position - 1;
        self.tiles[position_index]
            .symbol
            .replace(symbol.to_string());
    }

    pub fn is_position_occupied(&self, position: usize) -> bool {
        let position_index = position;
        let tile_symbol = self.tiles[position_index].symbol.borrow_mut().to_string();

        tile_symbol == "X" || tile_symbol == "O"
    }

    pub fn get_empty_tiles_by_user_position(&self) -> Vec<usize> {
        let tiles = self.tiles.iter();
        let mut empty_tiles = Vec::new();

        for tile in tiles {
            if !self.is_position_occupied(tile.position) {
                empty_tiles.push(tile.position + 1);
            } else {
            }
        }
        empty_tiles
    }
}

pub fn format_board(board: &Board) -> String {
    let rows = board.get_rows();
    rows.map(|row| format!("{}{}", tiles_to_string(row), "\n"))
        .collect::<String>()
}

fn tiles_to_string<'board>(tiles: impl Iterator<Item = &'board Tile>) -> String {
    tiles
        .map(|tile| format!("[{}] ", tile.symbol.borrow_mut().to_string()))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::players::Human;
    use crate::test_helpers::populate_board;

    fn len(board: Board) -> usize {
        board.tiles.len()
    }

    fn section_to_string<'board>(
        board_section: impl Iterator<Item = impl Iterator<Item = &'board Tile>>,
    ) -> String {
        let mut section_to_string: String = String::new();
        for section in board_section {
            section_to_string.push_str(tiles_to_string(section).as_str());
        }
        section_to_string
    }

    #[test]
    fn it_creates_a_3_by_3_board() {
        let board = Board::new(3);

        assert_eq!(len(board), 9)
    }

    #[test]
    fn returns_the_length_of_the_board() {
        let board = Board::new(3);

        assert_eq!(len(board), 9)
    }

    #[test]
    fn it_returns_the_row_of_a_3_by_3_board() {
        let board = Board::new(3);
        let rows = board.get_rows();

        assert_eq!(board.get_rows().count(), 3);
        assert_eq!(
            section_to_string(rows),
            "[1] [2] [3] \
             [4] [5] [6] \
             [7] [8] [9] "
        )
    }

    #[test]
    fn it_returns_the_columns_of_a_3_by_3_board() {
        let board = Board::new(3);
        let columns = board.get_columns();

        assert_eq!(board.get_columns().count(), 3);
        assert_eq!(
            section_to_string(columns),
            "[1] [4] [7] \
             [2] [5] [8] \
             [3] [6] [9] "
        )
    }

    #[test]
    fn it_returns_the_right_diagonal_of_a_3_by_3_board() {
        let board = Board::new(3);
        let right_diagonal = board.get_right_diagonal();

        assert_eq!(board.get_right_diagonal().count(), 3);
        assert_eq!(tiles_to_string(right_diagonal), "[1] [5] [9] ")
    }

    #[test]
    fn it_returns_the_left_diagonal_of_a_3_by_3_board() {
        let board = Board::new(3);
        let left_diagonal = board.get_left_diagonal();

        assert_eq!(board.get_left_diagonal().count(), 3);
        assert_eq!(tiles_to_string(left_diagonal), "[3] [5] [7] ")
    }

    #[test]
    fn it_formats_a_3_by_3_board() {
        let board = Board::new(3);

        assert_eq!(
            format_board(&board),
            "[1] [2] [3] \n\
             [4] [5] [6] \n\
             [7] [8] [9] \n"
        );
    }

    #[test]
    fn it_marks_the_board_with_the_player_symbol() {
        let board = Board::new(3);
        let player = Human::new("X".to_string());

        assert_eq!(board.tiles[2].symbol.borrow_mut().to_string(), "3");

        board.mark_with_symbol(&player.symbol, 3);

        assert_eq!(board.tiles[2].symbol.borrow_mut().to_string(), "X");
        assert_eq!(
            format_board(&board),
            "[1] [2] [X] \n\
             [4] [5] [6] \n\
             [7] [8] [9] \n"
        );
    }

    #[test]
    fn it_return_true_if_the_position_is_occupied() {
        let board = Board::new(3);
        board.mark_with_symbol(&"X".to_string(), 5);
        assert_eq!(board.is_position_occupied(4), true);
    }

    #[test]
    fn it_return_false_if_the_position_is_not_occupied() {
        let board = Board::new(3);
        assert_eq!(board.is_position_occupied(6), false);
    }

    #[test]
    fn it_returns_a_list_of_empty_tiles_by_user_positions() {
        let board = Board::new(3);
        populate_board(&board, "X X X \
                                X X 6 \
                                7 8 9".to_string());

        assert_eq!(board.get_empty_tiles_by_user_position(), [6, 7, 8, 9])
    }
}
