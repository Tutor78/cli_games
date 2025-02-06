use crate::libs::{main_menu, user_name};

mod libs;

fn main() {
    println!("Welcome to the command line game corner!");
    println!("This program is currently under construction using Rust!");
    // calls the user name function and stores it as a variable
    let player = user_name();
    println!("Welcome {}!", player);

    main_menu();
}
