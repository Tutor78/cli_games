mod games;
mod menus;
mod utils;

fn main() {
    // Welcomes the user
    println!("Welcome to the command line game corner!");
    println!("This program is currently under construction using Rust!");

    // Sets the username variable to be passed onto something later
    let player = utils::utils::user_name();
    println!("Welcome {}!", player);

    // Calls the main menu
    menus::menus::main_menu();
}