use inquire::{ Select };
use rand::seq::IndexedRandom;
use crate::menus::menus::main_menu;
use crate::utils::utils::play_again;

pub fn rock_paper_scissors () {
    let mut player_wins = 0;
    let mut computer_wins = 0;
    let choices = vec!["Rock", "Paper", "Scissors"];

    loop {
        let player_choice = Select::new("What is your pick?", choices.clone())
            .prompt()
            .unwrap()
            .to_string();

        println!("You picked {}", player_choice);

        let computer_choice = choices.clone().choose(&mut rand::rng()).unwrap().to_string();

        println!("I chose {}.", computer_choice);

        match computer_choice.as_ref() {
            "Rock" => {
                if player_choice == "Scissors" {
                    println!("I win!");
                    computer_wins += 1;
                } else if player_choice == "Paper" {
                    println!("I lose!");
                    player_wins += 1;
                } else {
                    println!("It's a draw!");
                }
            }
            "Paper" => {
                if player_choice == "Rock" {
                    println!("I win!");
                    computer_wins += 1;
                } else if player_choice == "Scissors" {
                    println!("I lose!");
                    player_wins += 1;
                } else {
                    println!("It's a draw!");
                }
            }
            "Scissors" => {
                if player_choice == "Paper" {
                    println!("I win!");
                    computer_wins += 1;
                } else if player_choice == "Rock" {
                    println!("I lose!");
                    player_wins += 1;
                } else {
                    println!("It's a draw!");
                }
            }
            _ => println!("What's even happening?!"),
        }

        if computer_wins > player_wins {
            println!("I'm ahead {} to {}!", computer_wins, player_wins);
        } else if computer_wins < player_wins {
            println!("You're winning {} to {}!", player_wins, computer_wins);
        } else {
            println!("We're tied {} to {}", computer_wins, player_wins);
        }

        if play_again() == false {
            println!("Thanks for playing!");
            break;
        }
    }

    main_menu();
}