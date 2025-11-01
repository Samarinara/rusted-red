use std::fs::{File, read_to_string};
use std::io::{self, Write, BufWriter};

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

fn create_location() -> io::Result<()> {
    print!("{}[2J", 27 as char);
    println!("What is the name of the new locaion?");
    let name = crate::input_value(">> ").trim().to_owned();

    println!("Creating template scene file...");
    let mut file = File::create("src/scenes/".to_owned() + &name + ".rs")?;
    let template = r#"pub fn load() {
    print!("This is a template");
    println!("-- Long beautiful description --");
    println!(" - Title - \n");
    println!("1) Option 1");
    println!("2) Option 2");
    println!("3) Option 3");
    println!("4) Option 4");
    println!("0) Return to Main Menu");
    let choice = crate::input_value(">> ");
    match choice.as_str().trim() {
        "1" => {option_1();},
        "2" => {option_2();},
        "3" => {option_3();},
        "4" => {option_4();},
        "0" => {crate::main_menu();},
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


}"#;
    file.write_all(template.as_bytes())?;
    println!("File created successfully");

    println!("Adding new scene to module tree...");
    let scene_manager = read_to_string("src/scenes.rs")?;

    let mut output = Vec::new();
    let target_line = "// Add scenes here";
    output.push(format!("pub mod {};", name));

    for line in scene_manager.lines() {
        output.push(line.to_string());
        if line == target_line {
            output.push(format!(r#"        "{0}" => {0}::load(),"#, name));
        }
    }
    
    let scene_manager_file = File::create("src/scenes.rs")?;
    let mut writer = BufWriter::new(scene_manager_file);
    for line in output {
        writeln!(writer, "{}", line);
    }

    Ok(())
}
