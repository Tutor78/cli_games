use inquire::Text;
use crate::menus::menus::main_menu;

pub fn hangman () {
    let word = "Word";

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

    main_menu();
}