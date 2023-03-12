//! Nostr client relay.

use crate::types::{ClientMessage, RelayMessage};
use anyhow::{anyhow, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{channel, Sender};
use tokio::task::JoinSet;
use tokio_tungstenite::{self as ws, tungstenite::Message as wsMessage};

/// Nostr relay URL.
pub type RelayUrl = String;

/// Nostr relay.
#[derive(Deserialize, Serialize)]
pub struct Relay {
    /// Websocket URL.
    pub url: String,
    /// Whether the relay is active.
    pub active: bool,
    /// Used for sending messages TO the relay.
    #[serde(skip)]
    pub outgoing_sender: Option<Sender<ClientMessage>>,
    /// Used for sending messages FROM the relay.
    #[serde(skip)]
    pub incoming_sender: Option<Sender<RelayMessage>>,
    /// Thread pool.
    #[serde(skip)]
    pool: JoinSet<Result<()>>,
}

impl Relay {
    /// Connect to relay and start tasks to send/receive messages.
    pub async fn listen(&mut self, buffer: usize) -> Result<()> {
        let (mut ws_outgoing, mut ws_incoming) = ws::connect_async(&self.url).await?.0.split();

        // Create channels
        let (outgoing_sender, mut outgoing_receiver) = channel::<ClientMessage>(buffer);
        let (incoming_sender, _) = channel::<RelayMessage>(buffer);
        self.outgoing_sender = Some(outgoing_sender);
        self.incoming_sender = Some(incoming_sender.clone());

        // Listen for outgoing messages (client) and send them to web socket (relay)
        self.pool.spawn(async move {
            while let Ok(message) = outgoing_receiver.recv().await {
                let json = serde_json::to_string(&message)?;
                ws_outgoing.send(wsMessage::text(json)).await?;
            }

            Err(anyhow!("closed or lagged behind"))
        });

        // Listen for incoming messages (web socket) and send them trough incoming sender (client)
        self.pool.spawn(async move {
            while let Some(ws_message) = ws_incoming.next().await {
                let ws_message = ws_message?;
                let ws_message = ws_message.to_text()?;
                let message = serde_json::from_str::<RelayMessage>(ws_message)?;
                incoming_sender.send(message)?;
            }

            Err(anyhow!("closed or lagged behind"))
        });

        // Set to active
        self.active = true;

        Ok(())
    }

    /// Create [`Relay`].
    pub fn new(url: RelayUrl) -> Self {
        Self {
            url,
            active: false,
            outgoing_sender: None,
            incoming_sender: None,
            pool: JoinSet::new(),
        }
    }

    /// Send a message to the relay.
    pub fn send(&mut self, message: ClientMessage) -> Result<()> {
        let sender = self.outgoing_sender.as_ref().ok_or_else(|| anyhow!("missing outgoing sender"))?;
        sender.send(message)?;

        Ok(())
    }
}
