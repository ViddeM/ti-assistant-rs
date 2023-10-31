#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use eyre::bail;
use game::GameState;
use tokio::{
    net::{TcpListener, TcpStream},
    select, spawn,
    sync::{broadcast, RwLock},
};
use ws_message::{GameId, WsMessageOut};

use crate::{game::Game, websocket_client::WsClient, ws_message::WsMessageIn};

#[macro_use]
extern crate rocket;

pub mod api;
pub mod data;
pub mod example_game;
pub mod game;
pub mod lobby;
pub mod phases;
pub mod player;
pub mod websocket_client;
pub mod ws_message;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let host = format!("0.0.0.0:5555");
    log::info!("Setting up websocket on host {host}");

    let server = TcpListener::bind(host)
        .await
        .expect("Failed to setup TCP listener");

    let lobbies = Arc::new(Lobbies::default());

    loop {
        let (stream, from) = server.accept().await.expect("Failed to accept client");
        spawn(handle_client(stream, from, Arc::clone(&lobbies)));
    }
}

#[derive(Default)]
pub struct Lobbies {
    list: RwLock<HashMap<GameId, Arc<Lobby>>>,
}

pub struct Lobby {
    game: RwLock<Game>,
    state_updates: broadcast::Sender<GameState>,
}

pub async fn handle_client(stream: TcpStream, from: SocketAddr, lobbies: Arc<Lobbies>) {
    async fn inner(stream: TcpStream, from: SocketAddr, lobbies: Arc<Lobbies>) -> eyre::Result<()> {
        let mut ws_client = WsClient::accept(stream).await;

        let message = ws_client.receive_message::<WsMessageIn>().await?;

        let lobby = match message {
            WsMessageIn::NewGame => {
                let id: GameId = "asdf".to_string(); //TODO: randomize id
                let lobby = Arc::new(Lobby {
                    game: RwLock::new(Game::default()),
                    state_updates: broadcast::Sender::new(100),
                });
                let mut lobbies = lobbies.list.write().await;

                if lobbies.contains_key(&id) {
                    bail!("new game id collision: {id:?}");
                }

                lobbies.insert(id, Arc::clone(&lobby));

                lobby
            }
            WsMessageIn::JoinGame(id) => {
                let list = lobbies.list.read().await;
                let Some(lobby) = list.get(&id) else {
                    bail!("no lobby with id {id:?}");
                };
                Arc::clone(lobby)
            }
            _ => {
                bail!("got unexpected initial message: {message:?}")
            }
        };

        ws_client
            .send_message(&WsMessageOut::game_options())
            .await?;

        let mut state_updates = {
            let game = lobby.game.read().await;
            ws_client
                .send_message(&WsMessageOut::GameState(game.current.clone()))
                .await?;

            // make sure we subscribe while we are holding the game state lock to avoid silly races
            lobby.state_updates.subscribe()
        };

        loop {
            select! {
                update = state_updates.recv() => {
                    log::debug!("sending state update to {from:?}");
                    ws_client.send_message(&WsMessageOut::GameState(update?)).await?;
                }
                message = ws_client.receive_message::<WsMessageIn>() => {
                    let message = message?;
                    let WsMessageIn::Event(event) = message else {
                        bail!("got unexpected event: {message:?}");
                    };

                    let mut game = lobby.game.write().await;

                    log::debug!("applying event {event:?}");
                    // TODO: propagate errors back over the socket?
                    game.apply(event);
                    lobby.state_updates.send(game.current.clone())?;

                    drop(game);
                }
            }
        }
    }

    if let Err(e) = inner(stream, from, lobbies).await {
        log::warn!("disconnecting {from} because of error: {e:#}");
    } else {
        log::info!("{from} disconnected");
    }
}
