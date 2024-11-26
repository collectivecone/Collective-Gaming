pub fn setup() {
    let mut settings_guard = settings::GLOBAL_SETTINGS.write();
    let settings = (settings_guard.as_deref_mut()).unwrap();
    
    println!("Input a key you want to be pressable, enter nothing to break loop");
    println!("A Space is represented by the word 'Space' ");
    loop {
        let key = input!("");
        let trim: String  = key.clone();

        if key.trim() == "" { break;};
        settings.keys.push(trim);
    }



    if let Ok(x) = input!("x screen size").parse() {
        if let Ok(y) = input!("y screen size").parse() {
            settings.screen_size = (x,y);
        }
    } else {println!("default set")}


    if let Ok(fps) = input!("fps").parse() { 
        settings.fps = fps;
    }

    if let Ok(ratio_for_press) = input!("ratio for press").parse() { 
        settings.ratio_for_press = ratio_for_press;
    }

    if let Ok(keyboard_update_rate) = input!("keyboard update rate").parse() {
        settings.keyboard_update_rate = keyboard_update_rate
    }

    match (input!("Is local server (Y/N)").to_lowercase()).as_str() {
        "y" => {settings.local_server = true},
        "n" => {settings.local_server = false},
        _ => {},
    }

    match (input!("Ignore multiple connections per ip (Y/N)").to_lowercase()).as_str() {
        "y" => {settings.ignore_multiple_connections_per_ip = true},
        "n" => {settings.ignore_multiple_connections_per_ip = false},
        _ => {},
    }

    match (input!("Keyboard Inputs (Y/N)").to_lowercase()).as_str() {
        "y" => {settings.keyboard_input_enabled = true},
        "n" => {settings.keyboard_input_enabled = false},
        _ => {},
    }

    match (input!("Mouse Input (Y/N)").to_lowercase()).as_str() {
        "y" => {settings.mouse_input_enabled = true},
        "n" => {settings.mouse_input_enabled = false},
        _ => {},
    }

    networking::websockets::send_to_all_users(networking::websockets::adduser::initalise_data_message(),None);
    drop(settings_guard);

    println!("");
    println!("hit \\ to emergency stop the program");
    println!("hit , and . to toggle between input being off or on");
    println!("hit ; to edit settings (note that this doesn't work for keys being changed or anything like that yet, server settings can be changed however including ratio to press)");
}