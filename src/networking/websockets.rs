use std::{collections::HashMap, ops::DerefMut, sync::Mutex};
use tungstenite::{accept_with_config, protocol::WebSocketConfig, Message, WebSocket};
use std::net::TcpStream;
use std::ops::Deref;
use serde_json::{self, Value};

pub enum WebsocketDataTypes {
    SendMonitorData  = 1,
    Initialing = 0,
    SendingGlobalUserStates = 2,
}

#[allow(deprecated)]
pub const STAND_WEB_CONFIG: WebSocketConfig = WebSocketConfig{
    max_send_queue: None,
    write_buffer_size: 0,
    max_write_buffer_size: usize::MAX,
    max_message_size: None,
    max_frame_size: None,
    accept_unmasked_frames: false,
};

#[derive(Debug)]
pub struct User{
    websocket: WebSocket<TcpStream>,
    true_ip: String, // Change to enum later.
    pub commands: Vec<String>,
}

static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());


fn register_user_controls(user: &mut User, dict: serde_json::Map<String,Value>) {
    let key: &String;
    let is_pressing: &bool;

    if let Some(value) = dict.get("Key") {
        if let Value::String(str) = value {
            key = str;
        } else {return}
    } else {return;}

    if let Some(value) = dict.get("Pressing") {
        if let Value::Bool(bool) = value {
           is_pressing = bool;
        } else {return}
    } else {return;}

    let mut already_holding = false;
    let mut index = 0;
    for (i,command) in user.commands.iter().enumerate() {
        if command == key {
            already_holding = true;
            index = i;
            break
        }
    }

    if !already_holding && *is_pressing {
        user.commands.push(key.clone());
    } else if already_holding && !(*is_pressing) {
        user.commands.remove(index);
    }
}

fn process_user_web_input(user:&mut User,msg: tungstenite::Message ) {
    if let Message::Text(text) = msg {
        let str = text.as_str();
        let json: Result<Value,serde_json::Error> = serde_json::from_str(str);
        if let Ok(Value::Object(dict)) = json {
            let input_type_option =  dict.get("Invoke_Type");
            match input_type_option {
                Some(input_type) => {
                    if let Value::String(input_type) = input_type {
                        if input_type == "Control" {
                            register_user_controls(user,dict);
                        }
                    }
                }
                None => {}
            }
        }
    }
}

pub fn read_all_user_sends() {
    let mut guard = USERS.lock().unwrap();
    let user_vec = guard.deref_mut();
    let mut to_delete: Vec<usize> = vec!();
    for (i,user) in user_vec.iter_mut().enumerate() {
        loop {
            match user.websocket.read() {
               Ok(msg) => {
                  _ = process_user_web_input(user,msg);
               }
               Err(e) => {
                 if e.to_string() == "Trying to work with closed connection" {
                     to_delete.push(i); println!("os error");
                } 
                break;
            }
            }
        }
    }

    for (i,delete_index) in to_delete.iter().enumerate() {
       user_vec.swap_remove(delete_index - i);
    }
}

pub fn update_websockets_and_get_users() -> &'static Mutex<Vec<User>> {
    read_all_user_sends();

    return &USERS;
}

pub fn send_to_all_users(mut msg: tungstenite::Message,websocket_data_type: WebsocketDataTypes) {

    let mut guard = USERS.lock().unwrap();
    let user_vec = guard.deref_mut();

    if let tungstenite::Message::Binary(mut vec) = msg {
        vec.insert( 0,websocket_data_type as u8);
        msg = tungstenite::Message::Binary(vec);
    }

    for user in user_vec.iter_mut() {
        
        _ = user.websocket.send(msg.clone());
    }
}

pub fn initalise_data() -> tungstenite::Message {


    let json = serde_json::json!({
        "Keys" : crate::settings::KEYS,
    });
 

    let string = json.to_string();
    let mut bytes = (*string.as_bytes()).to_vec();
    bytes.insert(0, WebsocketDataTypes::Initialing as u8);
   
    return tungstenite::Message::binary(bytes)
}

fn get_ip(headers: &HashMap<String,String>, _: &TcpStream) -> Option<String> {
    let ip = headers.get("CF-Connecting-IP");
    println!("{:?}",ip);
    if let Some(ip) = ip {
        return Some(ip.clone());
    } else {
        
    }
    return None;
}

fn is_multi_connecting(user_vec: &Vec<User>, ip_string: &String) -> bool {
    if crate::settings::IGNORE_MULTIPLE_CONNECTIONS_PER_IP {return false}

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
            _ = websocket.send(initalise_data());
    
            let user = User{
                websocket: websocket,
                commands: Vec::new(),
                true_ip: ip_string,
            };
            guard.push(user);
        }
    };

   
}