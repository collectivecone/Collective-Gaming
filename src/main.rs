use std::thread;
use std::time;

use device_query::{DeviceQuery, DeviceState, Keycode};

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
        mouse_input_enabled: false,

        cloud_input_enabled: true,
    } );
    pub struct SettingsStruct {
        pub keys: Vec<String>,
        pub fps: f32,
        pub keyboard_update_rate: f32,

        pub skip_client_keyboard_cycle: u32,
        pub ratio_for_press: f32,
        pub screen_size: (u16,u16),

        pub ignore_multiple_connections_per_ip: bool,
        pub local_server: bool,

        pub keyboard_input_enabled: bool,
        pub mouse_input_enabled: bool,

        pub cloud_input_enabled: bool,
    }
}


fn main() {

    setup::setup(); // pauses main loop till set up is done

    screenreader::monitor_scanner();
    networking::http_handler();
    userinputs::check_inputs();

    println!("set up done, server now running");

    loop {
        thread::sleep(time::Duration::from_millis(25));
        let device_state = DeviceState::new();
        let keys = device_state.get_keys(); 
        if keys.contains(&Keycode::BackSlash) {break}
        if keys.contains(&Keycode::Comma) {
            settings::GLOBAL_SETTINGS.write().as_deref_mut().unwrap().cloud_input_enabled = false;
        } 
        if keys.contains(&Keycode::Dot) {
            settings::GLOBAL_SETTINGS.write().as_deref_mut().unwrap().cloud_input_enabled = true;
        }
        if keys.contains(&Keycode::Semicolon) {
            setup::setup();
        }
    }
}