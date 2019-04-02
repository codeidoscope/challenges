use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct HumanPlayer {
    pub symbol: String,
}

impl HumanPlayer {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }

    pub fn get_move(&self) -> u32 {
        loop {
            print!("Select a position between 1 and 9: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let input = read_input();
            match numeric(&input) {
                Ok(input) if in_range(input) => break input,
                _ => eprintln!(
                    "Invalid position. Please select a position between 1 and 9.",
                )
            }
        }
    }
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input.");
    input
}

fn numeric(input: &str) -> Result<u32, ParseIntError> {
    let input = input.trim().parse::<u32>()?;
    Ok(input)
}

fn in_range(input: u32) -> bool {
    if input >= 1 && input <= 9 { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn it_create_a_human_player_with_a_given_symbol() {
        let human_player = HumanPlayer::new("X".to_string());
        assert_eq!(human_player.symbol, "X")
    }
}