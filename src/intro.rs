use std::io::{self, Write};

pub fn begin() {    
    print!("{}[2J", 27 as char);
    println!("\t\tOak");
    println!("Hello there!");
    println!("Welcome to the world of Pokémon!\nMy name is Oak!\nPeople call me the Pokémon Professor!");
    println!("\nThis world is inhabited by creatures called Pokémon!\nFor some people, Pokémon are pets. Other use them for fights.\nMyself… I study Pokémon as a profession.");
    println!("<Press any key to continue>");
    
    let mut dummy = String::new();
    io::stdin()
        .read_line(&mut dummy)
        .expect("Oops");
    println!("\n\n\n");
    println!("\t\tOak");
    println!("My apologies...\nCould you remind me of your name?");
    print!("\nPlease enter your name: ");
    io::stdout().flush().expect("Failed to flush");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Oops");
    println!("\nOh, your name is {}?", name.trim());
}
