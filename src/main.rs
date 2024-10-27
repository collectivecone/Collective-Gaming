use std::thread;
use std::time;

mod networking;
mod screenreader;
mod userinputs;

pub mod settings{
    pub const KEYS: &[&'static str] = &["w","s","d","a"];
    pub static FPS: f32 = 24f32;
    pub static RATIO_FOR_PRESS: f64 = 0.3f64;
    pub static SCREEN_SIZE: (u32,u32) = (350,200);
    pub static IGNORE_MULTIPLE_CONNECTIONS_PER_IP: bool = true;
}


fn main() {
    networking::http_handler();
    screenreader::monitor_scanner();
    userinputs::check_inputs();


    loop {
        thread::sleep(time::Duration::from_secs(100)); // temp so the main doesn't die.
    }
}


// TODOS
// Check if disabling cloudflare proxy changes ping, possibly the cause of all the ping.
// Check the internet times for
// Make it so if you aren't using a cloudflare proxy, it'll still work (noteably checking if a user is using the same ip doesn't really work if non cloudflare.)
// Make compression algorthim actually good
// Add command stuff so you can choose what monitor or what window you want to be stream. Same with the screen size
// Try making it so the main() runs on it's own thread so you can adjust shit with command line while it runs
// Put some of the http requests on different threads so you can serve more than around 75 people at the same time
// Figure out to properly scale the html
// Figure out CSS so that website can look good.