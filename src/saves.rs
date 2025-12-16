use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{read_to_string, File};
use std::io::{self, BufWriter, Write};

#[derive(Serialize, Deserialize)]
struct Save {
    top: String,
    prev_scene: String,
    current_scene: String,
    bottom: String
}

pub fn get_prev_scene() -> Result<String, io::Error> {
    let file_string = read_to_string("game_data/save_data.json")?;
    let save: Save = serde_json::from_str(&file_string.as_str())?;
    Ok(save.prev_scene)
}

pub fn save_game(current_scene: &str) -> Result<String, io::Error> {
    let file_string = read_to_string("game_data/save_data.json")?;
    let mut save: Save = serde_json::from_str(&file_string.as_str())?;
    
    save.prev_scene = save.current_scene;
    save.current_scene = current_scene.to_string();

    let new_save = serde_json::to_string_pretty(&save);
    let save_file = File::create("game_data/save_data.json")?;
    let mut writer = BufWriter::new(save_file);

    let mut output = Vec::new();
    let binding = new_save?;
    for line in binding.lines() {
        output.push(line);
    }

    for line in output {
        writeln!(writer, "{}\n", line);
    }

    Ok(current_scene.to_string())
}

pub fn refresh_pokeapi_data() {
    println!("refreshing...");
}
