use std::collections::HashMap;
use tungstenite::accept_with_config;
use tungstenite;
use std::net::TcpStream;
use std::ops::Deref;
use serde_json::{self};

use crate::settings::GLOBAL_SETTINGS;

use super::User;
use super::WebsocketDataTypes;
use super::USERS;
use super::STAND_WEB_CONFIG;

fn initalise_data(websocket: &mut tungstenite::WebSocket<TcpStream>) {
    let json = serde_json::json!({
        "Keys" : (GLOBAL_SETTINGS.read().unwrap()).keys.clone(),
    });

    let string = json.to_string();
    let mut bytes = (*string.as_bytes()).to_vec();
    bytes.insert(0, WebsocketDataTypes::Initialing as u8);
   
    _ = websocket.send(tungstenite::Message::binary(bytes));
}

fn send_inital_monitor_data(websocket: &mut tungstenite::WebSocket<TcpStream>) {
    let guard = crate::screenreader::PREVIOUSSCREENBYTES.lock().unwrap(); println!("b2"); let previous_bytes = guard.deref().clone();
    drop(guard);

    if previous_bytes.len() != 0 {
        let mut bytes = crate::screenreader::compression::compress_frame(previous_bytes, &mut Vec::new(), true);
        bytes.insert(0, WebsocketDataTypes::SendMonitorData as u8);
    
       
        _ = websocket.send(tungstenite::Message::binary(bytes));
    }
}

fn get_ip(headers: &HashMap<String,String>, stream : &TcpStream) -> Option<String> {
    let ip = headers.get("CF-Connecting-IP");
    if let Some(ip) = ip {
        return Some(ip.clone());
    } else {
        match  stream.local_addr() {
            Ok(ip) =>   return  Some(ip.ip().to_string()),
            Err(_) => return None
        }
    }
    
}

fn is_multi_connecting(user_vec: &Vec<User>, ip_string: &String) -> bool {
    if GLOBAL_SETTINGS.read().unwrap().ignore_multiple_connections_per_ip {return false}

    for user in user_vec {
        if user.true_ip == *ip_string {
            return true
        }
    }
    return false

}

pub fn add_new_user(stream: TcpStream,headers: HashMap<String,String>) {
    let mut guard = USERS.lock().unwrap();
    let user_vec = guard.deref();

    if let Some(ip_string) = get_ip(&headers,&stream) {
        if !is_multi_connecting(&user_vec,&ip_string) {
            _ = stream.set_nonblocking(true);
            let mut websocket = accept_with_config(stream,Some(STAND_WEB_CONFIG)).unwrap();
            initalise_data(&mut websocket);
            send_inital_monitor_data(&mut websocket);

    
            let user = User{
                websocket: websocket,
                commands: Vec::new(),
                true_ip: ip_string,
            };
            guard.push(user);
        }
    };

    drop(guard);

   
}