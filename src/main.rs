mod games;
mod menus;
mod utils;

fn main() {
    // Creates Data folder
    utils::utils::create_data_dir().expect("Could not create data dir");
    // Creates db
    utils::utils::create_db().expect("Could not create database");

    // Welcomes the user
    println!("Welcome to the command line game corner!");
    println!("This program is currently under construction using Rust!");

    // Calls the main menu
    menus::menus::main_menu();
}