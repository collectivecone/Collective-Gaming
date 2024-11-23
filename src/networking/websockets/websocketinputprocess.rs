use tungstenite::Message;
use std::ops::DerefMut;
use serde_json::Value;

use super::User;
use super::USERS;

fn register_mouse_change(user: &mut User, dict: serde_json::Map<String,Value>) {
    let mousex;
    let mousey;

    user.mouse_position = None;

    if let Some(value) = dict.get("X") {
        if let Value::Number(num) = value {
            if let Some(number) = num.as_u64() {
                mousex = number as u16;
            } else {return;}
        } else {return;}
    } else { return;}

    if let Some(value) = dict.get("Y") {
        if let Value::Number(num) = value {
            if let Some(number) = num.as_u64() {
                mousey = number as u16;
            } else {return;}
        } else {return;}
    } else {return;}



    user.mouse_position = Some((mousex,mousey));
}

fn register_user_controls(user: &mut User, dict: serde_json::Map<String,Value>) {
    let key: &String;
    let is_pressing: &bool;

    if let Some(value) = dict.get("Key") {
        if let Value::String(str) = value {
            key = str;
        } else {return;}
    } else {return;}

    if let Some(value) = dict.get("Pressing") {
        if let Value::Bool(bool) = value {
           is_pressing = bool;
        } else {return;}
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
                        } else if input_type == "Mouse" {
                            register_mouse_change(user, dict);
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
                     to_delete.push(i);
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
