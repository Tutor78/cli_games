use inquire::{CustomType, Confirm};
use rand::{Rng};
use std::cmp::Ordering;
use crate::menus;

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

    menus::main_menu();
}