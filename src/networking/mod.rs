
use std::net::TcpListener;
use std::thread::spawn;
pub mod https;
pub mod websockets;


pub fn http_handler() {
   spawn (|| {
      let listener = TcpListener::bind("0.0.0.0:80").unwrap();
      for stream in listener.incoming() { 
         let mut stream = stream.unwrap();
         if let Some((_,_,headers)) = https::get_body_and_headers(&mut stream) {
            if headers.get("Upgrade").unwrap_or(&String::new()) == "websocket" {
               println!("get");
               println!("{:?}",stream.peer_addr());
            //   println!("{:?}",headers);
               websockets::add_new_user(stream,headers);
            } else {
               println!("{:?}",headers);
               https::handle_connection(stream);
            }
         };
       }
   });
 }