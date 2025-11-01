pub mod placeholder; 
pub mod template;

pub fn load_scene(scene: &str) {
    println!("Loading {}...", scene);
    match scene {
// Add scenes here
        _ =>  placeholder::load(),
    }
}

