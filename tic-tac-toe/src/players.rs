extern crate rand;

use self::rand::Rng;
use crate::board::Board;
use crate::game::Game;
use crate::game_rules::GameRules;
use std::cmp;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub trait Player {
    fn get_move(
        &self,
        board: Board,
        game_rules: &GameRules,
        current_player: &String,
        opponent: &String,
        depth: isize,
    ) -> usize;
    fn get_symbol(&self) -> &String;
}

pub struct Human {
    pub symbol: String,
}

pub struct Computer {
    pub symbol: String,
}

pub struct UnbeatableComputer {
    pub symbol: String,
    pub opponent_symbol: String,
}

impl Player for Human {
    fn get_move(
        &self,
        board: Board,
        game_rules: &GameRules,
        current_player: &String,
        opponent: &String,
        depth: isize,
    ) -> usize {
        let empty_positions = board.get_empty_tiles_by_user_position();
        loop {
            println!("Select a position between 1 and 9: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let input = read_input();
            match is_numeric(&input) {
                Ok(input) if is_in_range(input) && is_empty_position(&empty_positions, input) => {
                    break input;
                }
                _ => eprintln!("Invalid position. Please select a position between 1 and 9."),
            }
        }
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Player for Computer {
    fn get_move(
        &self,
        board: Board,
        game_rules: &GameRules,
        current_player: &String,
        opponent: &String,
        depth: isize,
    ) -> usize {
        let empty_positions = board.get_empty_tiles_by_user_position();
        let mut position = 0 as usize;
        while !empty_positions.contains(&position) {
            let mut rng = rand::thread_rng();
            position = rng.gen_range(1, 10);
        }
        position
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Player for UnbeatableComputer {
    fn get_move(
        &self,
        board: Board,
        game_rules: &GameRules,
        current_player: &String,
        opponent: &String,
        depth: isize,
    ) -> usize {
        let scored_positions =
            scored_positions(board, &game_rules, &current_player, &opponent, depth);
        select_best_position(scored_positions)
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Human {
    pub fn new(player_symbol: String) -> Self {
        Human {
            symbol: player_symbol,
        }
    }
}

impl Computer {
    pub fn new(player_symbol: String) -> Self {
        Computer {
            symbol: player_symbol,
        }
    }
}

impl UnbeatableComputer {
    pub fn new(player_symbol: String, other_player_symbol: String) -> Self {
        UnbeatableComputer {
            symbol: player_symbol,
            opponent_symbol: other_player_symbol,
        }
    }
}

fn negamax(
    board: Board,
    game_rules: &GameRules,
    maximising_player: &String,
    minimising_player: &String,
    depth: isize,
) -> isize {
    let mut highest_score = 0;
    if game_rules.get_status(&board.clone(), &maximising_player, &minimising_player)
        != "IN_PROGRESS"
        || depth == 0
    {
        calculate_score(&game_rules, board, depth);
    } else {
        highest_score = select_highest_score(scored_positions(
            board.clone(),
            game_rules,
            maximising_player,
            minimising_player,
            depth + 1,
        ));
        -1 * highest_score;
    }
    highest_score
}

fn calculate_score(game_rules: &GameRules, board: Board, depth: isize) -> isize {
    let computer_symbol = "O".to_string();
    let human_symbol = "X".to_string();
    let status = game_rules.get_status(&board, &computer_symbol, &human_symbol);
    let draw = "DRAW".to_string();
    let computer_wins = "PLAYER_O_WINS".to_string();
    let computer_loses = "PLAYER_X_WINS".to_string();

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

fn select_best_position(scored_positions: HashMap<isize, usize>) -> usize {
    let highest_score = scored_positions.keys().max();
    scored_positions
        .get(highest_score.unwrap())
        .unwrap()
        .clone()
}

fn select_highest_score(scored_positions: HashMap<isize, usize>) -> isize {
    let highest_score = scored_positions.keys().max();

    if highest_score.is_none() {
        return 0;
    } else {
        scored_positions
            .get(highest_score.unwrap())
            .unwrap_or(&0)
            .clone() as isize
    }
}

fn scored_positions(
    board: Board,
    game_rules: &GameRules,
    current_player: &String,
    opponent: &String,
    depth: isize,
) -> HashMap<isize, usize> {
    let mut scored_positions = HashMap::new();
    let empty_positions = board.get_empty_tiles_by_user_position();
    for position in empty_positions {
        let score = score_position(
            board.clone(),
            &game_rules,
            position,
            &current_player,
            &opponent,
            depth,
        );
        scored_positions.insert(score, position);
    }
    scored_positions
}

fn score_position(
    board: Board,
    game_rules: &GameRules,
    position: usize,
    current_player: &String,
    opponent: &String,
    depth: isize,
) -> isize {
    let score = negamax(
        board.mark_clone_with_symbol(&current_player, &position),
        game_rules,
        current_player,
        opponent,
        depth,
    );
    score
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
    input >= 1 && input <= 9
}

fn is_empty_position(empty_positions: &[usize], chosen_position: usize) -> bool {
    empty_positions.contains(&chosen_position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::game_rules::GameRules;
    use crate::test_helpers::populate_board;

    #[test]
    fn it_creates_a_player_with_a_given_symbol() {
        let player = Human::new("X".to_string());
        assert_eq!(player.symbol, "X")
    }

    #[test]
    fn a_winning_position_returns_a_positive_score_for_the_computer_player() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        populate_board(
            &board,
            "X X O \
             O O O \
             O X X"
                .to_string(),
        );
        let computer_player = UnbeatableComputer::new("O".to_string(), "X".to_string());
        assert_eq!(calculate_score(&game_rules, board, 0), 10);
    }

    #[test]
    fn a_losing_position_returns_a_negative_score_for_the_computer_player() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        populate_board(
            &board,
            "O O X \
             X X X \
             X O O"
                .to_string(),
        );
        let computer_player = UnbeatableComputer::new("O".to_string(), "X".to_string());
        assert_eq!(calculate_score(&game_rules, board, 0), -10);
    }

    #[test]
    fn it_returns_0_for_a_tie_position() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        populate_board(
            &board,
            "O O X \
             X X O \
             O X X"
                .to_string(),
        );
        let computer_player = UnbeatableComputer::new("O".to_string(), "X".to_string());
        assert_eq!(calculate_score(&game_rules, board, 0), 0);
    }

    #[test]
    fn it_returns_a_negative_score_when_the_opponent_can_win_with_one_spot_let_on_the_board() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        populate_board(
            &board,
            "X 2 O \
             O X O \
             O X X"
                .to_string(),
        );
        let computer_player = UnbeatableComputer::new("O".to_string(), "X".to_string());
        assert_eq!(calculate_score(&game_rules, board, 0), -10);
    }

    #[test]
    fn it_returns_a_positive_score_when_current_player_can_win_with_one_spot_left_on_the_board() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        populate_board(
            &board,
            "X X O \
             O 5 O \
             O X X"
                .to_string(),
        );
        let computer_player = UnbeatableComputer::new("O".to_string(), "X".to_string());
        board.mark_with_symbol(&computer_player.symbol, &5);
        assert_eq!(calculate_score(&game_rules, board, 0), 10);
    }

    #[test]
    fn it_returns_a_negative_score_when_opponent_can_win_with_two_spots_left_on_the_board() {
        let board = Board::new(3);
        let game_rules = GameRules::new();
        populate_board(
            &board,
            "X 2 O \
             O 5 O \
             O O X"
                .to_string(),
        );
        let human_player = Human::new("X".to_string());
        board.mark_with_symbol(&human_player.symbol, &5);
        let computer_player = UnbeatableComputer::new("O".to_string(), "X".to_string());

        assert_eq!(calculate_score(&game_rules, board, 0), -10);
    }
}
