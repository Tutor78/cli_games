use std::fs::create_dir_all;
use std::path::{ PathBuf };
use inquire::{Text, Confirm, Select};
use directories::BaseDirs;
use rusqlite::{Connection, Result};
use rusqlite::fallible_iterator::FallibleIterator;

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
}

pub fn user_name() -> String {
    let player_name = loop {
        let name_input = Text::new("Before we begin, what is your name?")
            .prompt();

        let validate_name = name_input.unwrap();

        if validate_name.chars().all(char::is_whitespace) {
            println!("Please enter a valid name!");
        } else {
            break validate_name;
        }
    };

    player_name
}

pub fn play_again() -> bool {
    let play_again = Confirm::new("Play again?").prompt().unwrap();

    play_again
}

pub fn difficulty() -> String {
    let difficulty_options = vec!["Easy", "Normal", "Hard"];

    let difficulty = Select::new("Choose your difficulty:", difficulty_options)
        .prompt()
        .unwrap()
        .to_string();

    difficulty
}

// The following functions all deal with the db file
pub fn create_db() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (id INTEGER PRIMARY KEY, username TEXT NOT NULL)",
        (),
    )?;

    Ok(())
}

fn path_to_db() -> String {
    let dir_path = path_to_data_dir();

    let db_path = PathBuf::from(dir_path.join("rusty.db3"));

    db_path.to_str().unwrap().to_string()
}

pub fn create_data_dir() -> Result<()> {
    let dir_path = path_to_data_dir();

    create_dir_all(dir_path)
        .expect("Could not create Rusty Games Directory!");

    Ok(())
}

fn path_to_data_dir() -> PathBuf {
    let binding = BaseDirs::new().unwrap();
    let path = binding.data_local_dir();

    let dir_path = path.join("Rusty Games").join("data");

    dir_path
}