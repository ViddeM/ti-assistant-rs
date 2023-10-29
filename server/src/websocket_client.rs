use std::net::TcpStream;

use serde::de::DeserializeOwned;
use tungstenite::Message;
#[doc(hidden)]
use tungstenite::WebSocket;

use crate::ws_message::WsMessage;

#[derive(Debug)]
pub struct WsClient {
    conn: WebSocket<TcpStream>,
}

impl WsClient {
    pub fn accept(stream: TcpStream) -> Self {
        let ws = tungstenite::accept(stream).expect("Failed to initiate websocket");
        Self { conn: ws }
    }

    pub fn send_message(&mut self, message: &WsMessage) {
        if !self.conn.can_write() {
            panic!("Unable to write: Connection is closed");
        }

        let serialized = serde_json::to_string(message).expect("Failed to serialize message");
        if let Err(e) = self.conn.write_message(Message::text(serialized)) {
            eprintln!("Failed to send message to clients, err: {e}");
        }
    }

    pub fn receive_message<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> Option<ResponseMessage> {
        if !self.conn.can_read() {
            panic!("Unable to read: Connection is closed");
        }

        let response = match self.conn.read_message() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to read message, error {e}");
                return None;
            }
        };

        let text_response = response
            .to_text()
            .expect("Failed to read text from response");
        match serde_json::from_str(text_response) {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!("Failed to parse message [{text_response}], got err {e}");
                self.close();
                None
            }
        }
    }

    pub fn close(&mut self) {
        println!("Closing WS connection");

        if self.conn.can_write() {
            self.conn.close(None).expect("Failed to disconnect player");
        }
    }
}
