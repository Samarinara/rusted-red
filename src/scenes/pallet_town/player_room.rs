
pub fn load() {
    println!("loading a new location...");
    menu()
}
pub fn menu() {
    print!("{}[2J", 27 as char);
    println!(" - Your Room - \n");
    println!("It's a bit messy, but it's home.\n");
    println!("  1) Go downstairs");
    println!("  2) Fiddle with the clock");
    println!("  3) Clean your room");
    println!("  4) Watch something on TV");
    println!("  0) Return to Main Menu");
    loop{
        let choice = crate::input_value("\n>> ");
        match choice.as_str().trim() {
            "1" => {option_1(); break},
            "2" => {option_2(); break},
            "3" => {option_3(); break},
            "4" => {option_4(); break},
            "0" => {crate::main_menu(); break},
            _ => {println!("Please print a valid option");}
        }

    }
}   

fn option_1() {
    crate::scenes::load_scene("player_house");
}

fn option_2() {
    print!("{}[2J", 27 as char);
    println!("Wow... that's a clock\nWhat was supposed to be interesting here?\n");
    crate::press_to_continue();
    menu();
}

fn option_3() {
    print!("{}[2J", 27 as char);
    println!("*Cleaning noises*\n");
    crate::press_to_continue();
    print!("{}[2J", 27 as char);
    println!("The room looks a little bit cleaner?\nYou still wouldn't call it clean");
    crate::press_to_continue();
    menu();
}

fn option_4() {
    print!("{}[2J", 27 as char);
    println!("So violent...\nNintendo would never approve this programming\n");
    crate::press_to_continue();
    print!("{}[2J", 27 as char);
    println!("Now off to the regular PG adventure\n");
    crate::press_to_continue();
    menu();
}


