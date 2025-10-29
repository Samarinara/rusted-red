pub mod placeholder; 

pub fn load_scene(scene: &str) {
    println!("Loading {}...", scene);
    match scene {
        _ =>  placeholder::load(),
    }
}

