use std::{ops::DerefMut, sync::Mutex};
use tungstenite::{protocol::WebSocketConfig, WebSocket};
use std::net::TcpStream;

pub mod adduser;
pub mod websocketinputprocess;

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
    true_ip: String,
    pub commands: Vec<String>,
    pub mouse_position: Option<(u16,u16)>
}

static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());


pub fn update_websockets_and_get_users() -> &'static Mutex<Vec<User>> {
    websocketinputprocess::read_all_user_sends();

    return &USERS;
}

pub fn send_to_all_users(mut msg: tungstenite::Message,websocket_data_type_op: Option<WebsocketDataTypes>) {
    let mut guard = USERS.lock().unwrap();
    let user_vec = guard.deref_mut();

    if let Some(websocket_data_type) = websocket_data_type_op {
        if let tungstenite::Message::Binary(mut vec) = msg {
            vec.insert( 0,websocket_data_type as u8);
            msg = tungstenite::Message::Binary(vec);
        }
    }
    for user in user_vec.iter_mut() {
        _ = user.websocket.send(msg.clone());
    }
}

