use std::fs::create_dir_all;
use std::path::{ PathBuf };
use inquire::{Text, Confirm, Select};
use directories::BaseDirs;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    username: String,
}

#[derive(Debug)]
struct Highscore {
    guessing_game: i32,
    rock_paper_scissor: i32,
    hangman: i32,
}

fn create_profile() -> User {
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

    let user = User { username: player_name };

    user
}

pub fn play_again() -> bool {
    let play_again = Confirm::new("Play again?").prompt().unwrap();

    play_again
}

pub fn difficulty(easy_desc: &str, normal_desc: &str, hard_desc: &str) -> String {
    let mut easy = String::new();
    easy.push_str("Easy: ");
    easy.push_str(&easy_desc);

    let mut normal = String::new();
    normal.push_str("Normal: ");
    normal.push_str(&normal_desc);

    let mut hard = String::new();
    hard.push_str("Hard: ");
    hard.push_str(&hard_desc);

    let difficulty_options = vec![easy, normal, hard];

    let difficulty = Select::new("Choose your difficulty:", difficulty_options)
        .prompt()
        .unwrap()
        .to_string();

    let difficulty: Vec<&str> = difficulty.split(":").collect();

    difficulty[0].to_string()
}

// The following functions all deal with the db file
pub fn create_db() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS User (username TEXT NOT NULL PRIMARY KEY)",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Highscores (username TEXT NOT NULL PRIMARY KEY, guessing_game INTEGER NOT NULL, rock_paper_scissor INTEGER NOT NULL, hangman INTEGER NOT NULL)",
                ()
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

pub fn create_new_profile() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    let user = create_profile();
    let highscores = Highscore {
        guessing_game: 0,
        rock_paper_scissor: 0,
        hangman: 0,
    };

    conn.execute(
        "INSERT INTO User (username) VALUES (?1)",
        (&user.username,),
    )?;

    conn.execute(
        "INSERT INTO Highscores (guessing_game, rock_paper_scissor, hangman, username) VALUES (?1, ?2, ?3, ?4)",
        (&highscores.guessing_game, &highscores.rock_paper_scissor, &highscores.hangman, &user.username),
    )?;

    Ok(())
}

pub fn recall_db_data() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT username FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
        })
    })?;

    for user in user_iter {
        println!("Found user: {:?}", user?);
    }

    Ok(())
}