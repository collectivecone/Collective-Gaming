use std::time::Duration;
use std::thread::{spawn,sleep};
use std::time::Instant;
use crate::networking::websockets;

use captrs::Capturer;

pub mod reader;
pub mod compression;


pub fn monitor_scanner() {
    spawn (|| {
        let mut capture = Capturer::new(0).unwrap();

        loop {
           let before = Instant::now();
           let bytes = reader::getmonitordata(&mut capture);
           match bytes {
              Some(mut bytes) => {
                 bytes = compression::compress_frame(bytes);
                 let msg = tungstenite::Message::Binary(bytes);
                 websockets::send_to_all_users(msg,websockets::WebsocketDataTypes::SendMonitorData);
              }
              None => {} 
           }
           let after = Instant::now();
           let time_taken_for_read_frame = after.duration_since(before);
           sleep(
              Duration::from_secs_f32(1f32/ crate::settings::FPS).saturating_sub(time_taken_for_read_frame)
           );
        }
    });
 }