//! Nostr client relay.

use crate::types::{ClientMessage, RelayMessage};
use anyhow::{anyhow, Result};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::{channel, Sender};
use tokio::task::JoinSet;
use tokio_tungstenite::{self as ws, tungstenite::Message as wsMessage};

/// Nostr relay URL.
pub type RelayUrl = String;

/// Nostr relay.
pub struct Relay {
    /// Websocket URL.
    pub url: String,
    /// Used for sending messages TO the relay.
    pub outgoing_sender: Sender<ClientMessage>,
    /// Used for sending messages FROM the relay.
    pub incoming_sender: Sender<RelayMessage>,
    /// Thread pool.
    pool: JoinSet<Result<()>>,
}

impl Relay {
    /// Connect to relay and start tasks to send/receive messages.
    pub async fn listen(&mut self) -> Result<()> {
        let (mut ws_outgoing, mut ws_incoming) = ws::connect_async(&self.url).await?.0.split();

        // Listen for messages from "self.outgoing_sender" and send them to web socket
        let mut outgoing_receiver = self.outgoing_sender.subscribe();
        self.pool.spawn(async move {
            while let Ok(message) = outgoing_receiver.recv().await {
                let json = serde_json::to_string(&message)?;
                ws_outgoing.send(wsMessage::text(json)).await?;
            }

            Err(anyhow!("closed or lagged behind"))
        });

        // Listen for messages from web socket and send them to "self.incoming_sender"
        let incoming_sender = self.incoming_sender.clone();
        self.pool.spawn(async move {
            while let Some(ws_message) = ws_incoming.next().await {
                let ws_message = ws_message?;
                let ws_message = ws_message.to_text()?;
                let message = serde_json::from_str::<RelayMessage>(ws_message)?;
                incoming_sender.send(message)?;
            }

            Err(anyhow!("closed or lagged behind"))
        });

        Ok(())
    }

    /// Create [`Relay`].
    pub fn new(url: &str, buffer: usize) -> Self {
        Self {
            url: url.to_string(),
            outgoing_sender: channel::<ClientMessage>(buffer).0,
            incoming_sender: channel::<RelayMessage>(buffer).0,
            pool: JoinSet::new(),
        }
    }

    /// Send a message to the relay.
    pub fn send(&mut self, message: ClientMessage) -> Result<()> {
        self.outgoing_sender.send(message)?;

        Ok(())
    }
}
