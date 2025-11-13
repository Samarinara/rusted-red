use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;
use std::io;

#[derive(Serialize, Deserialize)]
struct Save {
    prev_scene: String,
}

pub fn get_prev_scene() -> Result<String, io::Error> {
    let file_string = read_to_string("game_data/save_data.json")?;
    let save: Save = serde_json::from_str(&file_string.as_str())?;
    Ok(save.prev_scene)
}


