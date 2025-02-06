use inquire::Text;

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