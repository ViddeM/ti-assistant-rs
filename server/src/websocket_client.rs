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
    pub async fn accept(stream: TcpStream) -> eyre::Result<Self> {
        let ws = tokio_tungstenite::accept_async(stream)
            .await
            .wrap_err("Failed to initiate websocket")?;

        Ok(Self { conn: ws })
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
        // loop to handle ping/pong messages
        loop {
            let message = match self.conn.next().await {
                Some(Ok(r)) => r,
                Some(Err(e)) => bail!("Failed to read message, error: {e}"),
                None => bail!("EOF"),
            };

            let text_message = match message {
                Message::Ping(data) => {
                    self.conn
                        .send(Message::Pong(data))
                        .await
                        .wrap_err("failed to send pong to client")?;
                    continue;
                }
                Message::Pong(..) => continue,
                Message::Frame(..) => bail!("got raw websocket frame"),
                Message::Close(..) => bail!("client closed connection"),
                Message::Binary(..) => bail!("client sent us binary data"),
                Message::Text(text) => text,
            };

            log::debug!("got message ¿{text_message}?");

            break serde_json::from_str(&text_message)
                .wrap_err_with(|| format!("Failed to parse message {text_message:?}"));
        }
    }
}
