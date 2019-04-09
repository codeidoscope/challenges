extern crate rand;

use self::rand::Rng;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub trait Player {
    fn get_move(&self, empty_positions: Vec<usize>) -> usize;
    fn get_symbol(&self) -> &String;
}

pub struct Human {
    pub symbol: String,
}

pub struct Computer {
    pub symbol: String,
}

impl Player for Human {
    fn get_move(&self, empty_positions: Vec<usize>) -> usize {
        loop {
            println!("Select a position between 1 and 9: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let input = read_input();
            match is_numeric(&input) {
                Ok(input) if is_in_range(input) && is_empty_position(&empty_positions, input) => {
                    break input;
                }
                _ => eprintln!("Invalid position. Please select a position between 1 and 9.", ),
            }
        }
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Player for Computer {
    fn get_move(&self, empty_positions: Vec<usize>) -> usize {
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

    #[test]
    fn it_creates_a_player_with_a_given_symbol() {
        let player = Human::new("X".to_string());
        assert_eq!(player.symbol, "X")
    }
}
