use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::board::Board;
use crate::game_rules::GameRules;

pub trait Player {
    fn get_move(&self) -> usize;
    fn get_symbol(&self) -> &String;
}

pub fn get_player_move<T: Player>(player: T) -> usize {
    player.get_move()
}

pub struct Human {
    pub symbol: String,
}

pub struct Computer {
    pub symbol: String,
    pub other_player_symbol: String,
    pub game_rules: GameRules,
}

impl Player for Human {
    fn get_move(&self) -> usize {
        loop {
            println!("Select a position between 1 and 9: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let input = read_input();
            match is_numeric(&input) {
                Ok(input) if is_in_range(input) => break input,
                _ => eprintln!(
                    "Invalid position. Please select a position between 1 and 9.",
                )
            }
        }
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Player for Computer {
    fn get_move(&self) -> usize {
        // TODO: Implement computer player
        4
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Human {
    pub fn new(player_symbol: String) -> Self {
        Human { symbol: player_symbol }
    }
}

impl Computer {
    pub fn new(player_symbol: String, other_symbol: String, rules: GameRules) -> Self {
        Computer { symbol: player_symbol, other_player_symbol: other_symbol, game_rules: rules }
    }

    fn calculate_score(self, board: &Board, depth: isize) -> isize {
        let status = self.game_rules.get_status(&board, &self.symbol, &self.other_player_symbol);
        let draw = String::from("DRAW");
        let computer_wins = format!("PLAYER_{}_WINS", &self.symbol);
        let computer_loses = format!("PLAYER_{}_WINS", &self.other_player_symbol);

        if status == draw {
            0
        } else if status == computer_wins {
            10 + depth
        } else if status == computer_loses {
            -10 + depth
        } else {
            0
        }
    }

    fn is_game_draw(&self, board: &Board) -> bool {
        &self.game_rules.get_status(&board, &self.symbol, &self.other_player_symbol) == "DRAW"
    }
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input.");
    input
}

fn is_numeric(input: &str) -> Result<usize, ParseIntError> {
    let input = input.trim().parse::<usize>()?;
    Ok(input)
}

fn is_in_range(input: usize) -> bool {
    if input >= 1 && input <= 9 { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn it_creates_a_player_with_a_given_symbol() {
        let player = Human::new("X".to_string());
        assert_eq!(player.symbol, "X")
    }

    #[test]
    fn it_returns_true_when_the_game_is_a_draw() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);
        board.mark_with_symbol(o, 1);
        board.mark_with_symbol(o, 2);
        board.mark_with_symbol(x, 3);
        board.mark_with_symbol(x, 4);
        board.mark_with_symbol(x, 5);
        board.mark_with_symbol(o, 6);
        board.mark_with_symbol(o, 7);
        board.mark_with_symbol(x, 8);
        board.mark_with_symbol(x, 9);

        assert_eq!(player_two.is_game_draw(&board), true);
    }

    #[test]
    fn it_returns_false_when_the_game_is_not_a_draw() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);

        assert_eq!(player_two.is_game_draw(&board), false);
    }

    #[test]
    fn a_winning_position_returns_a_positive_score_for_the_computer_player() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        board.mark_with_symbol(x, 1);
        board.mark_with_symbol(x, 2);
        board.mark_with_symbol(o, 3);
        board.mark_with_symbol(o, 4);
        board.mark_with_symbol(o, 5);
        board.mark_with_symbol(o, 6);
        board.mark_with_symbol(o, 7);
        board.mark_with_symbol(x, 8);
        board.mark_with_symbol(x, 9);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);
        assert_eq!(player_two.calculate_score(&board, 0), 10);
    }

    #[test]
    fn a_losing_position_returns_a_negative_score_for_the_computer_player() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        board.mark_with_symbol(o, 1);
        board.mark_with_symbol(o, 2);
        board.mark_with_symbol(x, 3);
        board.mark_with_symbol(x, 4);
        board.mark_with_symbol(x, 5);
        board.mark_with_symbol(x, 6);
        board.mark_with_symbol(x, 7);
        board.mark_with_symbol(o, 8);
        board.mark_with_symbol(o, 9);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);
        assert_eq!(player_two.calculate_score(&board, 0), -10);
    }

    #[test]
    fn it_returns_0_for_a_tie_position() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        board.mark_with_symbol(o, 1);
        board.mark_with_symbol(o, 2);
        board.mark_with_symbol(x, 3);
        board.mark_with_symbol(x, 4);
        board.mark_with_symbol(x, 5);
        board.mark_with_symbol(o, 6);
        board.mark_with_symbol(o, 7);
        board.mark_with_symbol(x, 8);
        board.mark_with_symbol(x, 9);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);
        assert_eq!(player_two.calculate_score(&board, 0), 0);
    }

    #[test]
    fn it_returns_a_negative_score_when_the_opponent_can_win_with_one_spot_let_on_the_board() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        board.mark_with_symbol(x, 1);
        board.mark_with_symbol(o, 3);
        board.mark_with_symbol(o, 4);
        board.mark_with_symbol(x, 5);
        board.mark_with_symbol(o, 6);
        board.mark_with_symbol(o, 7);
        board.mark_with_symbol(x, 8);
        board.mark_with_symbol(x, 9);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);
        assert_eq!(player_two.calculate_score(&board, 0), -10);
    }

    #[test]
    fn it_returns_a_positive_score_when_current_player_can_win_with_one_spot_left_on_the_board() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        board.mark_with_symbol(x, 1);
        board.mark_with_symbol(x, 2);
        board.mark_with_symbol(o, 3);
        board.mark_with_symbol(o, 4);
        board.mark_with_symbol(o, 6);
        board.mark_with_symbol(o, 7);
        board.mark_with_symbol(x, 8);
        board.mark_with_symbol(x, 9);

        let player_one = Human::new("X".to_string());
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);
        board.mark_with_symbol(&player_two.symbol, 5);
        assert_eq!(player_two.calculate_score(&board, 0), 10);
    }

    #[test]
    fn it_returns_a_negative_score_when_opponent_can_win_with_two_spots_left_on_the_board() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        let x = &"X".to_string();
        let o = &"O".to_string();
        board.mark_with_symbol(x, 1);
        board.mark_with_symbol(o, 3);
        board.mark_with_symbol(o, 4);
        board.mark_with_symbol(o, 6);
        board.mark_with_symbol(o, 7);
        board.mark_with_symbol(o, 8);
        board.mark_with_symbol(x, 9);

        let player_one = Human::new("X".to_string());
        board.mark_with_symbol(&player_one.symbol, 5);
        let player_two = Computer::new("O".to_string(), player_one.symbol, game_rules);

        assert_eq!(player_two.calculate_score(&board, 0), -10);
    }
}