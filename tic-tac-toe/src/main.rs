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
    fn it_creates_a_3_by_3_board_of_tiles() {
        let board = Board::new(3);
        assert_eq!(board.tiles.len(), 9)
    }

    #[test]
    fn it_creates_a_4_by_4_board_of_tiles() {
        let board = Board::new(4);
        assert_eq!(board.tiles.len(), 16)
    }
}