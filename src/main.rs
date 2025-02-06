#[path = "menus/menus.rs"] mod menus;
#[path = "utils/utils.rs"] mod utils;

fn main() {
    println!("Welcome to the command line game corner!");
    println!("This program is currently under construction using Rust!");
    // calls the user name function and stores it as a variable
    let player = utils::user_name();
    println!("Welcome {}!", player);

    menus::main_menu();
}
