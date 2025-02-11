use crate::games;
use inquire::Select;
use crate::utils::utils::{create_new_profile, recall_db_data};

struct Menu {
    title: String,
    message: Box<str>,
    options: Vec<String>,
}

pub fn main_menu() {
    let main_menu = Menu {
        title: String::from("Main Menu"),
        message: String::from("What would you like to play?").into(),
        options: vec![
            "Profiles".to_string(),
            "Guessing Game".to_string(),
            "Rock, Paper, Scissors".to_string(),
            "Hangman".to_string(),
            "Exit".to_string()
        ]
    };

    println!("{}", main_menu.title);

    let menu_choice = Select::new(&main_menu.message, main_menu.options).prompt();

    match menu_choice {
        Ok(menu) => {
            if menu == "Guessing Game" {
                games::guessing_game::guessing_game();
            } else if menu == "Rock, Paper, Scissors" {
                games::rock_paper_scissors::rock_paper_scissors();
            } else if menu == "Hangman" {
                games::hangman::game::hangman();
            } else if menu == "Exit Game" {
                println!("Thanks for playing!");
            } else if menu == "Profiles" {
                profile_menu();
            }
        }
        Err(_) => {
            println!("There was an error!");
        }
    }
}

fn profile_menu() {
    let profile_menu = Menu {
        title: String::from("Profiles"),
        message: String::from("What would you like to do?").into(),
        options: vec![
            "Create Profile".to_string(),
            "Recall Profiles".to_string(),
        ]
    };

    let menu_choice = Select::new(&profile_menu.message, profile_menu.options).prompt();

    match menu_choice {
        Ok(menu) => {
            if menu == "Create Profile" {
                create_new_profile().expect("Unable to add user to db");
            } else if menu == "Recall Profiles" {
                recall_db_data().expect("Unable to retrieve db data");
            }
        }
        Err(_) => {
            println!("There was an error!");
        }
    }

    main_menu();
}