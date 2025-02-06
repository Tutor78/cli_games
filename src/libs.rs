use inquire::{Text, Select, CustomType, Confirm};
use rand::{Rng};
use std::cmp::Ordering;

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

    let ans = Select::new("What would you like to play?", menu_options.clone())
        .prompt();

    match ans {
        Ok(answer) => {
            if answer.trim() == "Guessing Game" {
                guessing_game();
            } else if answer.trim() == "Exit" {
                println!("Thanks for playing!");
            }
        }
        Err(_) => {
            println!("An error occurred :(");
        }
    }

}

pub fn guessing_game() {
    loop {
        let secret_number = rand::rng().random_range(1..=100);

        println!("Alright I'm thinking of a number between 1 and 100");

        loop {
            let player_guess = CustomType::<i32>::new("Try to guess: ")
                .with_error_message("Please pick a valid number!")
                .prompt();

            let player_num = player_guess.unwrap();

            // println!("{}", player_num);
            // println!("{}", secret_number);

            match player_num.cmp(&secret_number) {
                Ordering::Greater => println!("That number is too big!"),
                Ordering::Less => println!("That number is too small!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }

        let play_again = Confirm::new("Play again?").prompt();

        if play_again.unwrap() == false {
            println!("Thanks for guessing!");
            break;
        }
    }

    main_menu();
}