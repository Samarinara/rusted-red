pub mod player_room;
pub mod placeholder; 
pub mod template;

pub fn load_scene(scene: &str) {
    println!("Loading {}...", scene);
    match scene {
// Add scenes here
        "player_room" => player_room::load(),
        _ =>  placeholder::load(),
    }
}

