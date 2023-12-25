use std::fmt::Display;

use eyre::{bail, eyre, Context};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::{Sink, Stream};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite::Message;

pub struct WsClient {
    sender: MessageSender,
    receiver: Receiver<eyre::Result<String>>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> eyre::Result<Self> {
        let ws = tokio_tungstenite::accept_async(stream)
            .await
            .wrap_err("Failed to initiate websocket")?;

        let (ws_send, ws_recv) = ws.split();
        let sender = spawn_tx_task(ws_send);
        let receiver = spawn_rx_task(ws_recv, sender.clone());

        Ok(Self { sender, receiver })
    }

    /// Serialize something as JSON and send it on the WebSocket.
    pub async fn send_message(&mut self, message: &impl Serialize) -> eyre::Result<()> {
        let serialized = serde_json::to_string(message).wrap_err("Failed to serialize message")?;
        self.sender
            .send(Message::text(serialized))
            .await
            .wrap_err("failed to send message to client")
    }

    /// Receive a message on the WebSocket and deserialize it from JSON into the provided type `M`.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. A message is either received completely or not at all.
    pub async fn receive_message<M: DeserializeOwned>(&mut self) -> eyre::Result<M> {
        let message = match self.receiver.recv().await {
            Some(r) => r?,
            None => bail!("client closed connection"),
        };

        serde_json::from_str(&message)
            .wrap_err_with(|| format!("Failed to parse message {message:?}"))
    }
}

/// Spawn a task to handle sending to a websocket sink.
fn spawn_tx_task<S, E>(mut sink: S) -> MessageSender
where
    S: Sink<Message, Error = E>,
    S: Send + Unpin + 'static,
    eyre::Report: From<E>,
{
    let (tx_send, mut tx_recv) = mpsc::channel::<(Message, oneshot::Sender<eyre::Result<()>>)>(1);
    let sender = MessageSender { tx: tx_send };

    spawn(async move {
        // loop and send messages until the Sender is dropped
        loop {
            let Some((message, out)) = tx_recv.recv().await else {
                return;
            };

            let result = sink.send(message).await.map_err(Into::into).map(drop);
            let _ = out.send(result);
        }
    });

    sender
}

/// Spawn a task to handle receiving from a websocket stream.
///
/// Data messages received on the socket are expected to be strings, i.e. a binary message is
/// treated as an error. Ping/pong messages are automatically responded to.
fn spawn_rx_task<S, E>(mut stream: S, sender: MessageSender) -> Receiver<eyre::Result<String>>
where
    S: Stream<Item = Result<Message, E>>,
    S: Send + Unpin + 'static,
    E: Display,
{
    let (rx_send, rx_recv) = mpsc::channel::<eyre::Result<String>>(1);

    spawn(async move {
        // loop and receive messages until:
        // - the stream returns EOF,
        // - or the stream returns an error,
        // - or the sender returns an error when we respond to a ping.
        // - or the receiving half of the channel is dropped.
        let err = loop {
            let message = match stream.next().await {
                Some(Ok(r)) => r,
                Some(Err(e)) => break eyre!("Failed to read message, error: {e}"),
                None => return, // EOF
            };

            let text_message = match message {
                Message::Ping(data) => match sender.send(Message::Pong(data)).await {
                    Ok(_) => continue,
                    Err(e) => break e,
                },
                Message::Pong(..) => continue,
                Message::Close(..) => return, // EOF
                Message::Frame(..) => break eyre!("got raw websocket frame"),
                Message::Binary(..) => break eyre!("client sent us binary data"),
                Message::Text(text) => text,
            };

            log::debug!("got message ¿{text_message}?");

            if rx_send.send(Ok(text_message)).await.is_err() {
                return; // receiver is closed, nothing more to do.
            }
        };

        let _ = rx_send.send(Err(err)).await;
    });

    rx_recv
}

#[derive(Clone)]
struct MessageSender {
    tx: Sender<(Message, oneshot::Sender<eyre::Result<()>>)>,
}

impl MessageSender {
    pub async fn send(&self, message: Message) -> eyre::Result<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.tx
            .send((message, response_tx))
            .await
            .map_err(|_| eyre!("can't send ws message: channel closed"))?;
        response_rx.await?
    }
}
