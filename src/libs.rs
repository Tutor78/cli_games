use inquire::{Text, Select};

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
pub fn main_menu() {
    let menu_options: Vec<&str> = vec!["Guessing Game", "Exit"];

    loop {
        let ans = Select::new("What would you like to play?", menu_options.clone())
            .prompt();

        match ans {
            Ok(answer) => {
                if answer.trim() == "Guessing Game" {
                    println!("That's under construction!");
                } else if answer.trim() == "Exit" {
                    println!("Thanks for playing!");
                    break;
                }
            }
            Err(_) => {
                println!("An error occurred :(");
            }
        }
    }
}