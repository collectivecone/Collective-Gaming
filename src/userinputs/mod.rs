use std::thread::{spawn,sleep};
use std::time::Duration;
use enigo::{self, Direction, Keyboard, Mouse,Coordinate};
use std::ops::Deref;

use crate::settings::GLOBAL_SETTINGS;
use crate::networking::websockets;
use crate::networking::websockets::User;

fn user_inputs_mouse_positions(users: &Vec<User>) -> (u16,u16) {
    let mut total_mouse_position: (u32,u32) = (0,0);
    let mut total_users_with_mouses: u32 = 0;
    for user in users {
        if let Some((mousex,mousey)) = user.mouse_position {
            total_users_with_mouses += 1;
        

            total_mouse_position.0 += mousex as u32 ;
            total_mouse_position.1 += mousey as u32;
        }
    }
    if total_users_with_mouses == 0 {return (0,0)}

    return ((total_mouse_position.0 / total_users_with_mouses) as u16,(total_mouse_position.1 / total_users_with_mouses) as u16 ) ;
}

fn user_inputs_into_keyboard_inputs(users: &Vec<User>) -> (Vec<(String,bool)>,Vec<(String,f64)>) {
    let total_users = users.len() as f64;

    let settings = GLOBAL_SETTINGS.read().unwrap();
    let (valid_keys,ratio_for_press) = ((*settings).keys.clone(),(*settings).ratio_for_press);
    drop(settings);

    let mut amounts: Vec<f64> = vec!(); for _ in valid_keys.clone() {amounts.push(0f64);}
    for user in users {
        for command in user.commands.iter() {
            for (i, entry) in valid_keys.clone() .iter().enumerate()  {
                if entry == command {
                    amounts[i] += 1f64;
                }
            }
        }
    }

    let mut keys_pressing: Vec<(String,bool)> = vec!();
    let mut keys_ratios: Vec<(String,f64)> = vec!();

    for (i,amount) in amounts.iter().enumerate() {
        let key = valid_keys[i].clone() ;
        keys_pressing.push((key.clone(), amount / total_users >= ratio_for_press ));
        keys_ratios.push((key.clone(), amount / total_users ));

    }

    return (keys_pressing,keys_ratios)
}

fn set_mouse_inputs(mouse_position: (u16,u16) ,controller: &mut enigo::Enigo) {
    let screen_size = controller.main_display().unwrap();

    let x = ((mouse_position.0 as f32) / (65536 as f32) * (screen_size.0 as f32)) as i32;
    let y = ((mouse_position.1 as f32) / (65536 as f32) * (screen_size.1 as f32)) as i32;
    
    _ = controller.move_mouse(x, y, Coordinate::Abs);
}

fn set_keyboard_inputs(keyboard_inputs: &Vec<(String,bool)>,previous_keyboard_inputs: Vec<(String,bool)>,controller: &mut enigo::Enigo) {
    for (i,(key,bool)) in keyboard_inputs.iter().enumerate() {
        let (_,previous) = previous_keyboard_inputs.get(i).unwrap();
        if *previous == *bool {continue}

        let command_type: Direction;

        if *bool {command_type = Direction::Press} 
        else {command_type = Direction::Release}
       
        if key == "Space" {
         _ = controller.key(enigo::Key::Unicode(' '), command_type);
        } else if key == "M1" {
            controller.button(Button::Left, command_type)
        } else if key == "M2" {
            controller.button(Button::Right, command_type)
        } else if key == "M3" {
            controller.button(Button::Middle, command_type)
        } else {
            let char = key.to_lowercase().as_str().chars().next().unwrap();

            _ = controller.key(enigo::Key::Unicode(char), command_type);
        }
    }
}

fn send_keyboard_data_to_client(user_count: usize,key_ratios: Vec<(String,f64)>) {
    let settings = GLOBAL_SETTINGS.read().unwrap();
    let ratio_for_press = (*settings).ratio_for_press;
    drop(settings);

    let mut bit_array: Vec<u8> = vec![];
    bit_array.push(((user_count / (256 * 256 * 256)) % 256) as u8) ;
    bit_array.push(((user_count / (256 * 256 )) % 256) as u8);
    bit_array.push(((user_count / (256)) % 256) as u8);
    bit_array.push((user_count % 256) as u8);

    for (_,i) in key_ratios {
        if i < ratio_for_press {
            bit_array.push(((i / ratio_for_press)  * 128f64) as u8);
        } else {
            bit_array.push(((((i - ratio_for_press) / (1f64 - ratio_for_press))  * 127f64) as u8) + 128u8);
        }
    }

    let msg = tungstenite::Message::Binary(bit_array);
    websockets::send_to_all_users(msg,websockets::WebsocketDataTypes::SendingGlobalUserStates);
}

pub fn check_inputs() {
    spawn (|| {
        let mut controller = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
        let settings = GLOBAL_SETTINGS.read().unwrap();
        let mut previous: Vec<(String,bool)> = vec!(); for entry in ((*settings).keys).iter()  {previous.push((entry.clone(),false)); }
        drop(settings);
        loop {
            let users_global = super::networking::websockets::update_websockets_and_get_users();
            let guard = users_global.lock().unwrap();
            let user_vec = guard.deref();

            let user_count = user_vec.len();
            if user_count > 0 {
                let (commands,key_ratios,) = user_inputs_into_keyboard_inputs(user_vec);
                let mouseposition = user_inputs_mouse_positions(user_vec);
                drop(guard);
                if (*GLOBAL_SETTINGS.read().unwrap()).cloud_input_enabled {
                    if (*GLOBAL_SETTINGS.read().unwrap()).mouse_input_enabled {
                        set_mouse_inputs(mouseposition,&mut controller);
                    }
                    if (*GLOBAL_SETTINGS.read().unwrap()).keyboard_input_enabled {
                        set_keyboard_inputs(&commands,previous,&mut controller);
                    }
                 
                }
                send_keyboard_data_to_client(user_count,key_ratios);
               
                previous = commands;
            } else {drop(guard);}

            sleep(Duration::from_secs_f64(1f64 / (*GLOBAL_SETTINGS.read().unwrap()).keyboard_update_rate ));
        }
    });
}