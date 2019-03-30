fn greet_user() -> String {
    String::from("Welcome to Tic Tac Toe! You are player X")
}

fn prompt_user() -> String {
    String::from("Please select a position between 1 and 9: ")
}


#[cfg(test)]
mod tests {
    use crate::greet_user;
    use crate::prompt_user;

    #[test]
    fn it_greets_a_user_when_game_starts() {
        assert_eq!(greet_user(), "Welcome to Tic Tac Toe! You are player X")
    }

    #[test]
    fn it_prompts_a_user_to_choose_a_position() {
        assert_eq!(prompt_user(), "Please select a position between 1 and 9: ")
    }
}
