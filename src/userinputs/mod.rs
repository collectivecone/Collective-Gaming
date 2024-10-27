
use std::thread::{spawn,sleep};
use std::time::Duration;
use enigo::{self, Direction, Keyboard};
use std::ops::Deref;

use crate::settings::RATIO_FOR_PRESS;
use crate::networking::websockets;
use crate::networking::websockets::User;

fn user_inputs_into_keyboard_inputs(users: &Vec<User>) -> (Vec<(String,bool)>,Vec<(String,f64)>) {
    let total_users = users.len() as f64;

    let valid_keys = crate::settings::KEYS;
    let mut amounts: Vec<f64> = vec!(0f64,0f64,0f64,0f64);

    for user in users {
        for command in user.commands.iter() {
            for (i, entry) in valid_keys.iter().enumerate()  {
                if entry == command {
                    amounts[i] += 1f64;
                }
            }
        }
    }

    let mut keys_pressing: Vec<(String,bool)> = vec!();
    let mut keys_ratios: Vec<(String,f64)> = vec!();

    for (i,amount) in amounts.iter().enumerate() {
        let key = valid_keys[i];
        keys_pressing.push((String::from(key), amount / total_users >= RATIO_FOR_PRESS ));
        keys_ratios.push((String::from(key), amount / total_users ));

    }

    return (keys_pressing,keys_ratios)
}

fn set_keyboard_inputs(keyboard_inputs: &Vec<(String,bool)>,previous_keyboard_inputs: Vec<(String,bool)>,controller: &mut enigo::Enigo) {
    for (i,(key,bool)) in keyboard_inputs.iter().enumerate() {
        let (_,previous) = previous_keyboard_inputs.get(i).unwrap();
        if *previous == *bool {continue}

        let command_type: Direction;

        if *bool {command_type = Direction::Press} 
        else {command_type = Direction::Release}
       
        let char = key.to_lowercase().as_str().chars().next().unwrap();

        _ = controller.key(enigo::Key::Unicode(char), command_type);
    }
}

fn send_keybaord_data_to_client(user_count: usize,key_ratios: Vec<(String,f64)>) {

    let mut bit_array: Vec<u8> = vec![];
    bit_array.push(((user_count / (256 * 256 * 256)) % 256) as u8) ;
    bit_array.push(((user_count / (256 * 256 )) % 256) as u8);
    bit_array.push(((user_count / (256)) % 256) as u8);
    bit_array.push((user_count % 256) as u8);

    for (_,i) in key_ratios {
        if i < crate::settings::RATIO_FOR_PRESS {
            bit_array.push(((i / RATIO_FOR_PRESS)  * 128f64) as u8);
        } else {
            bit_array.push(((((i - RATIO_FOR_PRESS) / (1f64 - RATIO_FOR_PRESS))  * 127f64) as u8) + 128u8);
        }
    }

    let msg = tungstenite::Message::Binary(bit_array);
    websockets::send_to_all_users(msg,websockets::WebsocketDataTypes::SendingGlobalUserStates);
}

pub fn check_inputs() {
    spawn (|| {
        let mut controller = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
        let mut previous: Vec<(String,bool)> = vec!(); for entry in crate::settings::KEYS.iter()  {previous.push((String::from(*entry),false)); }
        loop {
            let users_global = super::networking::websockets::update_websockets_and_get_users();
            let guard = users_global.lock().unwrap();
            let user_vec = guard.deref();

            let user_count = user_vec.len();
            if user_count > 0 {
                let (commands,key_ratios,) = user_inputs_into_keyboard_inputs(user_vec);
                drop(guard);
                set_keyboard_inputs(&commands,previous,&mut controller);
                send_keybaord_data_to_client(user_count,key_ratios);
               
                previous = commands;
            }

            sleep(Duration::from_millis(15));
        }
    });
}