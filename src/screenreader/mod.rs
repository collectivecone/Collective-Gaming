use std::sync::Mutex;
use std::time::Duration;
use std::thread::{spawn,sleep};
use std::time::Instant;
use std::ops::DerefMut;
use crate::networking::websockets;

use crate::settings::GLOBAL_SETTINGS;

use captrs::Capturer;

pub mod reader;
pub mod compression;

//pub static FULL_SCREEN_REFRESH_CYCLE;

pub static PREVIOUSSCREENBYTES: Mutex<Vec<u8>> =  Mutex::new(Vec::new());


pub fn monitor_scanner() {
    spawn (|| {
        let mut guard = PREVIOUSSCREENBYTES.lock().unwrap(); let previous_bytes = guard.deref_mut();
        previous_bytes.clear(); previous_bytes.extend_from_slice(&reader::makebase());
        let mut capture = Capturer::new(0).unwrap();
        drop(guard);

        loop {
           let before = Instant::now();
           let bytes = reader::getmonitordata(&mut capture);
           match bytes {
              Some(bytes) => {
                 let mut guard = PREVIOUSSCREENBYTES.lock().unwrap(); let previous_bytes = guard.deref_mut();
                 let compressed_bytes = compression::compress_frame(bytes.clone(),previous_bytes, false);
                 previous_bytes.clear();
                 previous_bytes.extend_from_slice(&bytes);
                 drop(guard);

                 let msg = tungstenite::Message::Binary(compressed_bytes);
                 websockets::send_to_all_users(msg,Some(websockets::WebsocketDataTypes::SendMonitorData));

            
               
              }
              None => {} 
           }
           let after = Instant::now();
           let time_taken_for_read_frame = after.duration_since(before);

           sleep(
              Duration::from_secs_f32(1f32 / (GLOBAL_SETTINGS.read().unwrap().fps)  ).saturating_sub(time_taken_for_read_frame)
           );

        }
    });
 }