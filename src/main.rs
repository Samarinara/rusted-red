use std::io;

mod intro;
mod scenes;

// All the scenes

fn main() {
    print!("{}[2J", 27 as char);
    
    println!("  *    P O K E M O N :  R U S T E D   R E D    *");
    println!("");
    println!(" ___  ___      _        ___  ___                   ");
    println!(" |  \\/  |     (_)       |  \\/  |                   ");
    println!(" | .  . | __ _ _ _ __   | .  . | ___ _ __  _   _   ");
    println!(" | |\\/| |/ _` | | '_ \\  | |\\/| |/ _ \\ '_ \\| | | |  ");
    println!(" | |  | | (_| | | | | | | |  | |  __/ | | | |_| |  ");
    println!(" \\_|  |_/\\__,_|_|_| |_| \\_|  |_/\\___|_| |_|\\__,_|  ");
    println!("");

    println!("1: New game (Previous game will be deleted)");
    println!("2: Load Game");
    println!("3: Options");
    println!("0: Exit game");
    println!("");
   
    loop {
        println!("Enter the number of the menu option you want: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "1" => {new_game(); break},
            "2" => {load_game(); break},
            "3" => {display_options(); break},
            "0" => {exit_game(); break},
            _ => println!("Invalid choice. Please try again")
        }
    }
}

fn new_game(){
    println!("Initializing new game...");
    println!("Overwriting save with defaults... (placeholder)");
    intro::begin();
}

fn load_game() {
    println!("Loading saved game...");
    scenes::load_scene("placeholder");
}

fn display_options() {
    print!("{}[2J", 27 as char);
    println!("Options menu coming soon!");
    println!("Press any key to return to the main menu");
    let mut dummy = String::new();
    io::stdin()
        .read_line(&mut dummy)
        .expect("Oops");
    main();
}

fn exit_game() {
    println!("Saving game... (not real yet)");
    println!("Exiting...");
    std::process::exit(0)
}

pub fn y_n() -> Option<bool>{
    let mut sure = String::new();
    io::stdin()
        .read_line(&mut sure)
        .expect("Oops");
    sure = sure.trim().to_uppercase();
    match sure.as_str() {
        "Y" => return Some(true),
        "N" => return Some(false),
        _ => return None,
    }
}

pub fn press_to_continue() {
    println!("\n<Press any key to continue>");
    
    let mut dummy = String::new();
    io::stdin()
        .read_line(&mut dummy)
        .expect("Oops");   
}

