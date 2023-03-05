//! Nostr client relay.

use crate::types::{ClientMessage, RelayMessage};
use std::collections::VecDeque;
use anyhow::Result;
use tokio::net::TcpStream;
use tokio_tungstenite::{self as ws, WebSocketStream, MaybeTlsStream};

/// Nostr relay URL.
pub type RelayUrl = String;

/// Nostr relay.
pub struct Relay {
    pub url: String,
    pub send_pool: VecDeque<ClientMessage>,
    pub ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Relay {
    pub async fn new(url: &str) -> Result<Self> {
        Ok(Self {
            url: url.to_string(),
            send_pool: VecDeque::new(),
            ws_stream: ws::connect_async(url).await?.0,
        })
    }
}