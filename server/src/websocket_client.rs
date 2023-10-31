use crate::ws_message::WsMessageOut;
use eyre::{bail, Context};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

#[derive(Debug)]
pub struct WsClient {
    conn: WebSocketStream<TcpStream>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> Self {
        let ws = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Failed to initiate websocket");
        Self { conn: ws }
    }

    pub async fn send_message(&mut self, message: &WsMessageOut) -> eyre::Result<()> {
        let serialized = serde_json::to_string(message).wrap_err("Failed to serialize message")?;
        self.conn
            .send(Message::text(serialized))
            .await
            .wrap_err("failed to send message to client")
    }

    pub async fn receive_message<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> eyre::Result<ResponseMessage> {
        let response = match self.conn.next().await {
            Some(Ok(r)) => r,
            Some(Err(e)) => {
                bail!("Failed to read message, error {e}");
            }
            None => {
                bail!("EOF")
            }
        };

        let text_response = response
            .to_text()
            .wrap_err("Failed to read text from response")?;

        log::debug!("got message ¿{text_response}?");

        serde_json::from_str(text_response)
            .wrap_err_with(|| format!("Failed to parse message {text_response:?}"))
    }
}
