pub mod pewter_city;
pub mod pallet_town;
pub mod placeholder; 
pub mod template;

pub fn load_scene(scene: &str) {
    println!("Loading {}...", scene);
    match scene {
// Add scenes here
        "pewter_city_outside" => pewter_city::load_pewter_city_outside(),
        "player_house" => pallet_town::load_player_house(),
        "player_room" => pallet_town::load_player_room(),
        _ =>  placeholder::load(),
    }
}

