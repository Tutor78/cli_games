use inquire::CustomType;
use rand::Rng;
use std::cmp::Ordering;
use crate::menus::menus::main_menu;
use crate::utils::utils::{ play_again, difficulty };

pub fn guessing_game() {
    loop {
        let mut num_of_guesses = 0;

        let secret_num: i32;

        let difficulty = difficulty("test", "test2", "test3");

        if difficulty == "Easy" {
            secret_num = rand::rng().random_range(1..=25);
        } else if difficulty == "Normal" {
            secret_num = rand::rng().random_range(1..=50).into();
        } else {
            secret_num = rand::rng().random_range(1..=100).into();
        };

        loop {
            println!("You've guessed {} times!", num_of_guesses);

            let player_guess = CustomType::<i32>::new("Try to guess: ")
                .with_error_message("Please pick a valid number!")
                .prompt();

            match player_guess.unwrap().cmp(&secret_num) {
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

        if play_again() == false {
            println!("Thanks for guessing!");
            break;
        }
    }

    main_menu();
}
