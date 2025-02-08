use inquire::Text;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use rand::Rng;
use crate::menus::menus::main_menu;
use crate::utils::utils::play_again;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Word {
    id: i32,
    word: String,
    definition: String,
}


pub fn hangman () {
    loop {
        let word = choose_word();

        let word_display: Vec<_> = word.chars().collect();

        let mut display_vector: Vec<char> = vec!['_'; word_display.len()];

        println!("Your word is {} letters long", display_vector.clone().len());

        loop {
            let player_guess = Text::new("What letter would you like to guess?")
                .prompt()
                .unwrap();

            let player_letter = player_guess
                .chars()
                .next()
                .unwrap();

            println!("You're letter: {}", player_letter);

            let mut i = 0;

            for char in word_display.clone() {
                let letter_match = char.to_uppercase().next().unwrap();
                let player_letter_match = player_letter.to_uppercase().next().unwrap();

                if letter_match == player_letter_match {
                    display_vector[i] = char;
                }

                print!(" {} ", display_vector[i]);

                i += 1;
            }

            println!("");

            if display_vector == word_display {
                println!("You win!");
                break;
            }
        }

        if play_again() == false {
            println!("Thanks for playing");
            break;
        }
    }

    main_menu();
}

fn choose_word() -> String {
    let json_file = Path::new("./src/games/hangman/text.json");
    let file = File::open(json_file).unwrap();

    let dictionary: Vec<Word> = serde_json::from_reader(file)
        .expect("Error while reading file");

    let random_num = rand::rng().random_range(1..=dictionary.len()) as i32;

    let mut secret_word = String::new();

    for word in dictionary {
        if word.id == random_num {
            secret_word = word.word
        }
    }

    secret_word
}