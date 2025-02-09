use inquire::{Text, Confirm, Select};

pub fn user_name() -> String {
    let player_name = loop {
        let name_input = Text::new("Before we begin, what is your name?")
            .prompt();

        let validate_name = name_input.unwrap();

        if validate_name.chars().all(char::is_whitespace) {
            println!("Please enter a valid name!");
        } else {
            break validate_name;
        }
    };

    player_name
}

pub fn play_again() -> bool {
    let play_again = Confirm::new("Play again?").prompt().unwrap();

    play_again
}

pub fn difficulty() -> String {
    let difficulty_options = vec!["Easy", "Normal", "Hard"];

    let difficulty = Select::new("Choose your difficulty:", difficulty_options)
        .prompt()
        .unwrap()
        .to_string();

    difficulty
}