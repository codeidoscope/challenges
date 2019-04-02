fn print_tiles<'board>(tiles: impl Iterator<Item=&'board Tile>) {
    let formatted_tiles = tiles.map(|tile| tile.symbol.as_str())
        .collect::<String>();
    println!("{}", formatted_tiles)
}

fn tiles_to_string<'board>(tiles: impl Iterator<Item=&'board Tile>) -> String {
    let tiles_as_string = tiles.map(|tile| tile.symbol.as_str())
        .collect::<String>();
    tiles_as_string
}

fn section_to_string<'board>(board_section: impl Iterator<Item=impl Iterator<Item=&'board Tile>>) -> String {
    let mut section_to_string: String = String::new();
    for section in board_section {
        section_to_string.push_str(tiles_to_string(section).as_str());
    }
    section_to_string
}

pub fn format_board(board: Board) -> String {
    let rows = board.get_rows();
    let formatted_rows = rows.map(|row| format!("{}{}", tiles_to_string(row), "\n"))
        .collect::<String>();
    formatted_rows
}

#[derive(Debug)]
struct Tile {
    symbol: String,
    position: usize,
}

pub struct Board {
    size: usize,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();

        for i in 0..size * size {
            tiles.push(Tile { symbol: format!("[{}] ", i + 1), position: i })
        }

        Self { size, tiles }
    }

    fn len(&self) -> usize {
        self.tiles.len()
    }

    fn get_rows(&self) -> impl Iterator<Item=impl Iterator<Item=&Tile>> {
        (0..self.size).map(move |row_index| {
            let row_start = row_index * self.size;
            let row_end = row_start + self.size;
            self.tiles[row_start..row_end].iter()
        })
    }

    fn get_columns(&self) -> impl Iterator<Item=impl Iterator<Item=&Tile>> {
        (0..self.size).map(move |column_index| {
            (0..self.size)
                .map(move |row_index| &self.tiles[row_index * self.size + column_index])
        })
    }

    fn get_right_diagonal(&self) -> impl Iterator<Item=&Tile> {
        (0..self.size).map(move |idx| &self.tiles[idx * self.size + idx])
    }

    fn get_left_diagonal(&self) -> impl Iterator<Item=&Tile> {
        (0..self.size).map(move |idx| &self.tiles[idx * self.size + self.size - 1 - idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_3_by_3_board() {
        let board = Board::new(3);

        assert_eq!(board.tiles.len(), 9)
    }

    #[test]
    fn returns_the_length_of_the_board() {
        let board = Board::new(3);

        assert_eq!(board.len(), 9)
    }

    #[test]
    fn it_returns_the_row_of_a_3_by_3_board() {
        let board = Board::new(3);
        let rows = board.get_rows();

        assert_eq!(board.get_rows().count(), 3);
        assert_eq!(section_to_string(rows), "[1] [2] [3] \
                                             [4] [5] [6] \
                                             [7] [8] [9] ")
    }

    #[test]
    fn it_returns_the_columns_of_a_3_by_3_board() {
        let board = Board::new(3);
        let columns = board.get_columns();

        assert_eq!(board.get_columns().count(), 3);
        assert_eq!(section_to_string(columns), "[1] [4] [7] \
                                                [2] [5] [8] \
                                                [3] [6] [9] ")
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
        assert_eq!(format_board(board), "[1] [2] [3] \n\
                                         [4] [5] [6] \n\
                                         [7] [8] [9] \n")
    }
}