use std::io::{self, Write};
use crate::scene_manager;

pub fn begin() {    
    print!("{}[2J", 27 as char);
    println!("\t\tOak");
    println!("Hello there!");
    println!("Welcome to the world of Pokémon!\nMy name is Oak!\nPeople call me the Pokémon Professor!");
    println!("\nThis world is inhabited by creatures called Pokémon!\nFor some people, Pokémon are pets. Other use them for fights.\nMyself… I study Pokémon as a profession.");

    crate::press_to_continue();

    println!("\n\n\n");
    println!("\t\tOak");
    println!("My apologies...\nCould you remind me of your name?");

    'name: loop {
        print!("\nPlease enter your name: ");
        io::stdout().flush().expect("Failed to flush");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Oops");
        
        'sure: loop{
            println!("\nYour name is {}? [y/n]", name.trim());
            print!(">> ");
            io::stdout().flush().expect("Failed to flush");

            match crate::y_n() {
               Some(true) => {break 'name},
               Some(false) => {break 'sure},
               None => println!("Try inputting either a 'y' or an 'n'"),
            }
        }
    }
    
    println!("\nGreat! It's good that we cleared that up. ");
    println!("Now I have a grandson... But I can't for the life of me remember his name.");
    let mut rival_name = String::new();
    'name: loop {
        print!("\nWhat is my grandson's name?\n>> ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut rival_name)
            .expect("Oops");
        rival_name = rival_name.trim().to_string();
        'sure: loop{
            println!("\nHis name is {}? [y/n]", rival_name.trim());
            print!(">> ");
            io::stdout().flush().expect("Failed to flush");

            match crate::y_n() {
               Some(true) => {break 'name},
               Some(false) => {break 'sure},
               None => println!("Try inputting either a 'y' or an 'n'"),
            }
        }
    }
    println!("Ah of course I remember {} now.\nI must've taken a couple too many rock smashes to the head. \nHa Ha!", rival_name);
    println!("\nWelcome to the world of Pokemon! See you soon.");
    scene_manager::load_scene("playerHouse");
}


