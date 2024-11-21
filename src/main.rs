use std::thread;
use std::time;
use prompted::input;
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};


mod networking;
mod screenreader;
mod userinputs;
mod setup;




pub mod settings{
    use std::sync::RwLock;

    pub static GLOBAL_SETTINGS: RwLock<SettingsStruct> = RwLock::new(SettingsStruct{
        keys: Vec::new(),
        fps: 24f32,
        keyboard_update_rate: 160f64,

        skip_client_keyboard_cycle: 7,
        ratio_for_press: 0.3f64,
        screen_size: (300,168),

        ignore_multiple_connections_per_ip: false,
        local_server: false,

        keyboard_input_enabled: true,
    } );
    pub struct SettingsStruct {
        pub keys: Vec<String>,
        pub fps: f32,
        pub keyboard_update_rate: f64,

        pub skip_client_keyboard_cycle: u32,
        pub ratio_for_press: f64,
        pub screen_size: (u32,u32),

        pub ignore_multiple_connections_per_ip: bool,
        pub local_server: bool,

        pub keyboard_input_enabled: bool
    }
}

fn setup() {

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

    if input!("is local server (Y/N)") == "Y" {
        settings.local_server = true;
    }

    if input!("ignore multiple connections per ip (Y/N)") == "Y" {
        settings.ignore_multiple_connections_per_ip = true;
    }
    drop(settings_guard);
}

fn main() {

    setup();

    screenreader::monitor_scanner();
    networking::http_handler();
    userinputs::check_inputs();

    println!("set up done, server now running");
    println!("hit \\ to emergency stop the program");
    println!("hit , and . to toggle between input being off or on");
    println!("hit ; to toggle between input being off or on");

    loop {
        thread::sleep(time::Duration::from_millis(100));
        let device_state = DeviceState::new();
        let keys = device_state.get_keys(); 
        if keys.contains(&Keycode.BackSlash) {break}
        if keys.contains(&Keycode.Comma) {
            settings::GLOBAL_SETTINGS.write().as_deref_mut().unwrap().keyboard_input_enabled = false;
        } 
        if keys.contains(&Keycode.Dot) {
            settings::GLOBAL_SETTINGS.write().as_deref_mut().unwrap().keyboard_input_enabled = true;
        }
        if keys.contains(&Keycode.Semicolon) {
            setup();
        }



    }
}