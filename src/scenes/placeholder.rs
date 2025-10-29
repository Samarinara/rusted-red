pub fn load() {
    print!("{}[2J", 27 as char);
    println!("This is the placeholder scene. If you are reading this, something may have gone wrong");
    
    crate::press_to_continue();
    crate::main_menu();
}


