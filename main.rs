use lazy_static::lazy_static;
use std::thread;
use std::time;

mod networking;
mod screenreader;
mod userinputs;
mod setup;

/*
pub mod settings {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref KEYS: Vec<&'static str> = vec!("w","a","s","d");
        static ref FPS: f32 = 24f32;
        static ref KEYBOARD_UPDATE_RATE: f64 = 160f64;
        
        static ref SKIP_CLIENT_KEYBOARD_CYCLE: u32 = 7;
        static ref RATIO_FOR_PRESS: f64 = 0.3f64;
        static ref SCREEN_SIZE: (u32,u32) = (300,168);
        static ref IGNORE_MULTIPLE_CONNECTIONS_PER_IP: bool = false;
    }
    

}

lazy_static! {
    static ref KEYS: Vec<&'static str> = vec!("w","a","s","d");
    static ref FPS: f32 = 24f32;
    static ref KEYBOARD_UPDATE_RATE: f64 = 160f64;
    
    static ref SKIP_CLIENT_KEYBOARD_CYCLE: u32 = 7;
    static ref RATIO_FOR_PRESS: f64 = 0.3f64;
    static ref SCREEN_SIZE: (u32,u32) = (300,168);
    static ref IGNORE_MULTIPLE_CONNECTIONS_PER_IP: bool = false;
}
     */


pub mod settings{
    pub const  KEYS: &[&'static str] = &["w","a","s","d","q","e"];
    pub static FPS: f32 = 24f32;
    pub static KEYBOARD_UPDATE_RATE: f64 = 160f64;
    
    pub static SKIP_CLIENT_KEYBOARD_CYCLE: u32 = 7;
    pub static RATIO_FOR_PRESS: f64 = 0.3f64;
    pub static SCREEN_SIZE: (u32,u32) = (300,168);
    pub static IGNORE_MULTIPLE_CONNECTIONS_PER_IP: bool = false;
}

static INCLUDE_OPTION_TO_CHANGE_SETTINGS_ON_START_UP: bool = false;


fn main() {

    
    //if INCLUDE_OPTION_TO_CHANGE_SETTINGS_ON_START_UP {setup::setup();} else {setup::use_defaults();}

    screenreader::monitor_scanner();
    networking::http_handler();
    userinputs::check_inputs();


    loop {
        thread::sleep(time::Duration::from_secs(100)); // temp so the main doesn't die.
    }
}


// TODOS
// Check if disabling cloudflare proxy changes ping, possibly the cause of all the ping.
// Make it so if you aren't using a cloudflare proxy, it'll still work (noteably checking if a user is using the same ip doesn't really work if non cloudflare.)
// Add command stuff so you can choose what monitor or what window you want to be stream. Same with the screen size
// Try making it so the main() runs on it's own thread so you can adjust shit with command line while it runs
// Put some of the http requests on different threads so you can serve more than around 75 people at the same time