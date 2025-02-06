#[path = "menus/menus.rs"] mod menus;
#[path = "utils/utils.rs"] mod utils;

fn main() {
    // Welcomes the user
    println!("Welcome to the command line game corner!");
    println!("This program is currently under construction using Rust!");

    // Sets the username variable to be passed onto something later
    let player = utils::user_name();
    println!("Welcome {}!", player);

    // Calls the main menu
    menus::main_menu();
}
