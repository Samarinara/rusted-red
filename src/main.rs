use std::io::{self, Write};

mod data_management;
mod intro;
mod routes;
mod saves;
mod scenes;

// All the scenes

fn main() {
    main_menu();
}

pub fn main_menu() {
    print!("{}[2J", 27 as char);

    println!("  *    P O K E M O N :  R U S T E D   R E D    *");
    println!("");
    println!(" ___  ___               ___  ___                   ");
    println!(" |  \\/  |     ( )       |  \\/  |                   ");
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
        let input = input_value("Enter the number of the menu option you want: ");

        match input.trim() {
            "1" => {
                new_game();
                break;
            }
            "2" => {
                load_game();
                break;
            }
            "3" => {
                display_options();
                break;
            }
            "0" => {
                exit_game();
                break;
            }
            "edit" => {
                scenes::template::edit_scenes();
                break;
            }
            _ => println!("Invalid choice. Please try again"),
        }
    }
}

fn new_game() {
    println!("Initializing new game...");
    println!("Overwriting save with defaults... (placeholder)");
    intro::begin();
}

fn load_game() {
    println!("Loading saved game...");
    let prev_scene = saves::get_prev_scene();
    match prev_scene {
        Ok(prev_scene) => {
            scenes::load_scene(&prev_scene.as_str());
        }
        Err(e) => {
            println!("No scene Found: {}", e);
            exit_game();
        }
    }
}

fn display_options() {
    print!("{}[2J", 27 as char);
    println!("\tWELCOME TO THE SETTINGS");
    println!("More options coming soon!\n");

    println!("1: Update pokemon data from PokeAPI");
    println!("0: Return to Main Menu");

    loop {
        let player_input = input_value("\nEnter the number of the menu option you want: ");
        match player_input.trim() {
            "1" => {
                refresh_pokeapi_data();
                break;
            }
            "0" => break,

            _ => {
                println!(
                    "\nPlease enter one of the numbers in the menu: {}",
                    &player_input
                );
            }
        }
    }
    main_menu();
}

fn exit_game() {
    println!("Saving game... ");
    println!("Exiting...");
    std::process::exit(0)
}

pub fn y_n() -> Option<bool> {
    print!("\n[y/n]\n>> ");
    io::stdout().flush().expect("Failed to flush");

    let mut sure = String::new();
    io::stdin().read_line(&mut sure).expect("Oops");
    sure = sure.trim().to_uppercase();
    match sure.as_str() {
        "Y" => return Some(true),
        "N" => return Some(false),
        _ => {
            println!("\nSorry that wasn't one of the allowed inputs\nPlease input a Y or an N");
            return y_n();
        }
    }
}

pub fn press_to_continue() {
    println!("\n<Press any key to continue>");

    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).expect("Oops");
}

pub fn input_value(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Oops");
    println!("{}", value);
    return value;
}

pub fn clear() {
    print!("{}[2J", 27 as char);
}

pub fn refresh_pokeapi_data() {
    clear();
    println!("Would you like to redownload all the pokemon data?");
    println!("It may take a while, and requires 100Mb free space");
    println!("\nThis will update all the data to the latest version and cannot be reversed");
    println!("\n\tDo you wish to continue?");
    match y_n() {
        Some(true) => println!("\nAlright, starting now."),
        Some(false) => {
            println!("Ok then, let's go back to the Main Menu");
            press_to_continue();
            main_menu();
            return;
        }
        None => {
            println!("Sorry an error occured\nReturning to the Main Menu");
            press_to_continue();
            main_menu();
            return;
        }
    }
    println!("yes");
}
