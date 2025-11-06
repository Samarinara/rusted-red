use std::fs::{File, self, read_to_string};
use std::io::{self, Write, BufWriter};
use std::path::Path;

const INVALID_CHARS: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*', '.'];

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
    println!("What is the name of the new scene?");
    let name = crate::input_value(">> ").trim().to_owned();
    let mut location = String::new();
    loop {
        println!("What location is this scene in? (type '0' if no location)");
        location = crate::input_value(">> ").trim().to_owned();
        if is_valid_filename(&location.as_str()) {
            break
        }
        println!("That isn't a valid file name.\n Don't use special characters except for '-' and '_'");
    }

    

    println!("Creating template scene file...");

    if !Path::new(("src/scenes/".to_string() + &location.to_string() + "/mod.rs").as_str()).exists() {
        println!("New location");
        fs::create_dir("src/scenes/".to_owned() + &location.to_string())?;

    }else {
        println!("Location Exists");
    }

    let mut file = File::create("src/scenes/".to_owned() + &location + "/" + &name + ".rs")?;
    let template = r#"
pub fn load() {
    println!("loading a new location...");
    menu()
}
pub fn menu() {
    print!("{}[2J", 27 as char);
    println!(" - Title - \n");
    println!("-- Long beautiful description --\n");
    println!("  1) Option 1");
    println!("  2) Option 2");
    println!("  3) Option 3");
    println!("  4) Option 4");
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


"#;
    file.write_all(template.as_bytes())?;
    println!("File created successfully");

    println!("Adding new scene to module tree...");
    let scene_manager = read_to_string("src/scenes.rs")?;

    let mut output = Vec::new();
    let target_line = "// Add scenes here";

    if !Path::new(("src/scenes/".to_string() + &location.to_string() + "/mod.rs").as_str()).exists() {
        println!("New location");
        output.push("pub mod ".to_owned() + &location + ";");
        File::create("src/scenes/".to_owned() + &location + "/mod.rs");

    }else {
        println!("Location Exists");
    }

    for line in scene_manager.lines() {
        output.push(line.to_string());
        if line == target_line {
            output.push(format!(r#"        "{1}" => {0}::load_{1}(),"#, location, name));
        }
    }
    
    let scene_manager_file = File::create("src/scenes.rs")?;
    let mut writer = BufWriter::new(scene_manager_file);
    for line in output {
        writeln!(writer, "{}", line);
    }
    
    println!("Creating Location mod.rs file");
    let location_mod = read_to_string(("src/scenes/".to_string() + &location.to_string() + "/mod.rs").as_str())?;
    
    let mut mod_output = Vec::new();
    let i_hate_formatting = "mod ~; pub fn load_~() { ~::load(); }";
    let formatted_bullshit = &i_hate_formatting.replace("~", &name);
    mod_output.push(formatted_bullshit.to_owned());
    
    for line in location_mod.lines() {
        mod_output.push(line.to_string());
    }

    let location_mod_file = File::create(("src/scenes/".to_string() + &location.to_string() + "/mod.rs").as_str())?;
    let mut mod_writer = BufWriter::new(location_mod_file);
    for line in mod_output {
        writeln!(mod_writer, "{}", line);
    }

    println!("Finished :D");

    Ok(())
}

fn is_valid_filename(name: &str) -> bool {
    if name.is_empty() || name == "." || name == ".." {
        return false;
    }
    !name.chars().any(|c| INVALID_CHARS.contains(&c))
}
