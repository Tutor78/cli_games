use inquire::Text;

pub fn hangman () {
    let word = "Word";

    let word_display: Vec<_> = word.chars().collect();

    let mut display_vector: Vec<char> = vec!['_'; word_display.len()];

    let player_guess = Text::new("What letter would you like to guess?")
        .prompt()
        .unwrap();

    let player_letter = player_guess
        .chars()
        .next()
        .unwrap();

    println!("{}", player_letter);

    let mut i = 0;

    for char in word_display {
        let letter_match = char.to_uppercase().next().unwrap();
        let player_letter_match = player_letter.to_uppercase().next().unwrap();

        if letter_match == player_letter_match {
            display_vector[i] = char;
        }

        print!(" {} ", display_vector[i]);

        i += 1;
    }
}