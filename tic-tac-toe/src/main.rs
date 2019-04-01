fn display_prompt(prompt: &str) -> &str {
    prompt
}

#[derive(Debug)]
struct Tile {
    symbol: String,
}

struct Board {
    size: usize,
    tiles: Vec<Tile>,
}

impl Board {
    fn new(size: usize) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();

        for i in 0..size * size {
            tiles.push(Tile { symbol: format!("[{}] ", i + 1) })
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
}

fn main() {
    println!("{:?}", Board::new(3).tiles)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_displays_a_prompt_to_the_user() {
        let mut prompts = HashMap::new();
        prompts.insert("welcome", "Welcome to Tic Tac Toe! You are player X");
        prompts.insert("select_position", "Please select a position between 1 and 9: ");

        let welcome_prompt = prompts.get(&"welcome").unwrap();
        let select_position_prompt = prompts.get(&"select_position").unwrap();

        assert_eq!(display_prompt(welcome_prompt),
                   "Welcome to Tic Tac Toe! You are player X");
        assert_eq!(display_prompt(select_position_prompt),
                   "Please select a position between 1 and 9: ");
    }

    #[test]
    fn it_creates_a_3_by_3_board() {
        let board = Board::new(3);
        assert_eq!(board.tiles.len(), 9)
    }

    #[test]
    fn it_creates_a_4_by_4_board() {
        let board = Board::new(4);
        assert_eq!(board.tiles.len(), 16)
    }

    #[test]
    fn returns_the_length_of_the_board() {
        let board = Board::new(3);
        assert_eq!(board.len(), 9)
    }

    #[test]
    fn it_returns_the_row_of_a_3_by_3_board() {
        let board = Board::new(3);
        assert_eq!(board.get_rows().count(), 3)
    }

    #[test]
    fn it_returns_the_row_of_a_4_by_4_board() {
        let board = Board::new(4);
        assert_eq!(board.get_rows().count(), 4)
    }
}