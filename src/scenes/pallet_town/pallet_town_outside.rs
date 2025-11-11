
pub fn load() {
    println!("loading a new location...");
    menu()
}
pub fn menu() {
    print!("{}[2J", 27 as char);
    println!(" - Pallet Town - \n");
    println!("This is a small town with a cozy vibe\nYou hear the pidgeys chirp overhead\nIt feels like home\n");
    println!("  1) Head to route 1");
    println!("  2) Enter rival's house");
    println!("  3) Enter Oak's Lab");
    println!("  4) Go Home");
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
    crate::routes::load_route("route_1".to_string());
}

fn option_2() {
    crate::scenes::load_scene("rival_house");
}

fn option_3() {
    crate::scenes::load_scene("oak_lab");
}

fn option_4() {
    crate::scenes::load_scene("player_house");
}


