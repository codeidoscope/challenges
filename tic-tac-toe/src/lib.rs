fn display_prompt(prompt: &str) -> &str {
    prompt
}

fn create_board(size: u32) -> Vec<String>{
    let board_size: u32 = size * size;
    let mut board: Vec<String> = Vec::new();

    for i in 0..board_size {
        board.push(format!("[{}] ", i+1))
    }
    board
}

#[cfg(test)]
mod tests {
    use crate::display_prompt;
    use std::collections::HashMap;
    use crate::create_board;

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
        assert_eq!(create_board(3), ["[1] ", "[2] ", "[3] ",
                                     "[4] ", "[5] ", "[6] ",
                                     "[7] ", "[8] ", "[9] "])
    }

    #[test]
    fn it_creates_a_4_by_4_board() {
        assert_eq!(create_board(4), ["[1] ", "[2] ", "[3] ", "[4] ",
                                           "[5] ", "[6] ", "[7] ", "[8] ",
                                           "[9] ", "[10] ", "[11] ", "[12] ",
                                           "[13] ", "[14] ", "[15] ", "[16] "])
    }

}
