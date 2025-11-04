
pub fn load() {
    print!("{}[2J", 27 as char);
    println!("This is a template");
    println!("-- Long beautiful description --");
    println!(" - Title - \n");
    println!("1) Option 1");
    println!("2) Option 2");
    println!("3) Option 3");
    println!("4) Option 4");
    println!("0) Return to Main Menu");
    loop{
        let choice = crate::input_value(">> ");
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
    println!("option_1");
}

fn option_2() {
    println!("option_2");
}

fn option_3() {
    println!("option_3");
}

fn option_4() {
    println!("option_4");
}


