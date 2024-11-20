use std::thread;
use std::time;

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
        local_server: true,
    } );
    pub struct SettingsStruct {
        pub keys: Vec<&'static str>,
        pub fps: f32,
        pub keyboard_update_rate: f64,

        pub skip_client_keyboard_cycle: u32,
        pub ratio_for_press: f64,
        pub screen_size: (u32,u32),

        pub ignore_multiple_connections_per_ip: bool,
        pub local_server: bool
    }
}



fn main() {
    screenreader::monitor_scanner();
    networking::http_handler();
    userinputs::check_inputs();

    loop {
        thread::sleep(time::Duration::from_secs(100)); // temp so the main doesn't die.
    }
}