
use std::net::TcpListener;
use std::thread::spawn;
use crate::settings::GLOBAL_SETTINGS;
pub mod https;
pub mod websockets;


pub fn http_handler() {
   spawn (|| {
      let listener = if !(GLOBAL_SETTINGS.read().unwrap()).local_server 
            {TcpListener::bind("0.0.0.0:80").unwrap()}
      else  {TcpListener::bind("127.0.0.1:80").unwrap()};
      for stream in listener.incoming() { 
         let mut stream = stream.unwrap();
         if let Some((_,_,headers)) = https::get_body_and_headers(&mut stream) {
            if headers.get("Upgrade").unwrap_or(&String::new()) == "websocket" {
            //   println!("{:?}",headers);
               websockets::adduser::add_new_user(stream,headers);
            } else {
               https::handle_connection(stream);
            }
         };
       }
   });
 }