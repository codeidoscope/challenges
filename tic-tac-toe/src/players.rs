use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub trait Player {
    fn get_move(&self) -> u32;
    fn get_symbol(&self) -> &String;
}

pub fn get_player_move<T: Player>(player: T) -> u32 {
    player.get_move()
}

pub struct Human {
    pub symbol: String,
}

pub struct Computer {
    pub symbol: String,
}

impl Player for Human {
    fn get_move(&self) -> u32 {
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

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Player for Computer {
    fn get_move(&self) -> u32 {
        // TODO: Implement computer player
        4
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl Human {
    pub fn new(sym: String) -> Self {
        Human { symbol: sym }
    }
}

impl Computer {
    pub fn new(sym: String) -> Self {
        Computer { symbol: sym }
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

    #[test]
    fn it_create_a_player_with_a_given_symbol() {
        let player = Human::new("X".to_string());
        assert_eq!(player.symbol, "X")
    }
}