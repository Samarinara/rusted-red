use std::io::{self, Write};

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
    println!("")
}


