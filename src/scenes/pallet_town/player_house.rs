
pub fn load() {
    println!("loading a new location...");
    menu()
}
pub fn menu() {
    print!("{}[2J", 27 as char);
    println!(" - Your House - \n");
    println!("Your living room is much nicer than your bedroom.\nYour mom sits at the table reading a book.\n");
    println!("  1) Go outside");
    println!("  2) Talk to your Mom");
    println!("  3) Go back upstairs");
    println!("  4) Check the fridge");
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
    crate::scenes::load_scene("pallet_town_outside");
}

fn option_2() {
    print!("{}[2J", 27 as char);
    println!("\t- Mom -");
    println!("\nOh honey, you look tired. Have a rest");
    crate::press_to_continue();
    menu();
}

fn option_3() {
    crate::scenes::load_scene("player_room");
}

fn option_4() {
    print!("{}[2J", 27 as char);
    println!("Darn\nEmpty\n");
    crate::press_to_continue();
    menu();
}


