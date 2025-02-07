use crate::games;
use inquire::Select;

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
            "Guessing Game".to_string(),
            "Rock, Paper, Scissors".to_string(),
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
                println!("Welcome to Rock, paper, Scissors!");
            } else if menu == "Exit Game" {
                println!("Thanks for playing!");
            }
        }
        Err(_) => {
            println!("There was an error!");
        }
    }
}