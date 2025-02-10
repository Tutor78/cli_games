use inquire::Text;
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
        let mut max_guesses = 10;

        let word_to_guess = random_word();

        let word = word_to_guess.word;

        let word_display: Vec<_> = word.chars().collect();

        let mut display_vector: Vec<char> = vec!['_'; word_display.len()];

        println!("Your word is {} letters long", display_vector.clone().len());
        println!("You have a max of {} guesses", max_guesses);
        println!("The definition is: {}", word_to_guess.definition);

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

            println!();

            max_guesses -= 1;

            if max_guesses > 3 {
                println!("{} guesses left", max_guesses );
            } else if max_guesses == 1 {
                println!("{} guess left", max_guesses);
            } else if max_guesses == 0 {
                println!("You lose");
                break;
            }

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

fn dictionary() -> Vec<Word> {
    let json_file = include_str!("../../../data/dictionary.json");

    let dictionary: Vec<Word> = serde_json::from_str(json_file)
        .expect("Error while reading file");

    dictionary
}

fn random_word() -> Word {
    let dictionary = dictionary();
    let random_num = rand::rng().random_range(1..=dictionary.len() as i32);

    let mut random_word: Word = Word {
        id: 0,
        word: "No word!".to_string(),
        definition: "No definition!".to_string(),
    };

    for word in dictionary {
        if word.id == random_num {
            random_word = word
        }
    }

    random_word
}