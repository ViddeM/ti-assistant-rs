#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Websocket communication for ti-helper.

/// A connected game client.
pub mod websocket_client;
/// Messages that can be sent or received.
pub mod ws_message;
