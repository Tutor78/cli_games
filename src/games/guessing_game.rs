use inquire::{CustomType, Confirm, Select};
use rand::{Rng};
use std::cmp::Ordering;
use crate::menus;

pub fn guessing_game() {
    loop {
        let mut num_of_guesses = 0;

        let difficulty_options = vec!["Easy", "Normal", "Hard"];

        let difficulty_select = Select::new("Choose your difficulty!", difficulty_options)
            .prompt()
            .unwrap()
            .to_string();

        let secret_number = difficulty(difficulty_select);

        loop {
            println!("You've guessed {} times!", num_of_guesses);

            let player_guess = CustomType::<i32>::new("Try to guess: ")
                .with_error_message("Please pick a valid number!")
                .prompt();

            match player_guess.unwrap().cmp(&secret_number) {
                Ordering::Greater => println!("That number is too big!"),
                Ordering::Less => println!("That number is too small!"),
                Ordering::Equal => {
                    num_of_guesses += 1;

                    if num_of_guesses == 1 {
                        println!("You won with {} guess!", num_of_guesses);
                    } else {
                        println!("You won with {} guesses!", num_of_guesses);
                    }
                    break;
                },
            }

            num_of_guesses += 1;
        }

        let play_again = Confirm::new("Play again?").prompt();

        if play_again.unwrap() == false {
            println!("Thanks for guessing!");
            break;
        }
    }

    menus::menus::main_menu();
}

fn difficulty(difficulty: String) -> i32 {
    if difficulty == "Easy" {
        println!("Alright I am thinking of a number between 1 and 25");
        rand::rng().random_range(1..=25)
    } else if difficulty == "Normal" {
        println!("Alright I am thinking of a number between 1 and 50");
        rand::rng().random_range(1..=50)
    } else {
        println!("Alright I am thinking of a number between 1 and 100");
        rand::rng().random_range(1..=100)
    }
}

#[test]
fn easy_difficulty() {
    let secret_number = difficulty(String::from("Easy"));
    let in_range: bool;

    if secret_number < 1 {
        in_range = false;
    } else if secret_number > 25 {
        in_range = false;
    } else {
        in_range = true;
    }

    assert!(in_range);
}

#[test]
fn normal_difficulty() {
    let secret_number = difficulty(String::from("Normal"));
    let in_range: bool;

    if secret_number < 1 {
        in_range = false;
    } else if secret_number > 50 {
        in_range = false;
    } else {
        in_range = true;
    }

    assert!(in_range);
}

#[test]
fn hard_difficulty() {
    let secret_number = difficulty(String::from("Hard"));
    let in_range: bool;

    if secret_number < 1 {
        in_range = false;
    } else if secret_number > 100 {
        in_range = false;
    } else {
        in_range = true;
    }

    assert!(in_range);
}