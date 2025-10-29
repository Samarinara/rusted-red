pub fn edit_scenes() { 
    print!("{}[2J", 27 as char);
    println!("\tWelcome to the scene editing wizard");
    println!("1) Create Location");
    println!("2) Create Route -TBD");
    println!("3) Delete Scene -TBD");
    println!("0) Return to main menu");
    
    let choice = crate::input_value(">> ");
    println!("{}", choice);
    match choice.as_str().trim() {
        "1" => {create_location();},
        _ => {crate::main_menu();},
    }
}

fn create_location() {
    print!("{}[2J", 27 as char);
    println!("\tWhat is the name of the new locaion?");
    name = crate::input_value(">> ");
    
}
