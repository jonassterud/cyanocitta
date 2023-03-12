//! Nostr client.

mod relay;

pub use relay::{Relay, RelayUrl};

use crate::types::{ClientMessage, Metadata, RelayMessage};
use anyhow::{anyhow, Result};
use secp256k1::{rand, KeyPair, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast::{channel, Receiver};
use tokio::task::JoinSet;

/// Nostr client to interact with relays.
#[derive(Deserialize, Serialize)]
pub struct Client {
    pub keys: KeyPair,
    pub metadata: Metadata,
    pub relays: HashMap<RelayUrl, Relay>,
    #[serde(skip)]
    pool: JoinSet<Result<()>>,
}

impl Client {
    /// Add relay to client.
    pub fn add_relay(&mut self, relay: Relay) {
        self.relays.insert(relay.url.clone(), relay);
    }

    /// Broadcast message to all relays.
    pub async fn broadcast_message(&mut self, message: ClientMessage) -> Result<()> {
        for relay in self.relays.values_mut() {
            relay.send(message.clone())?;
        }

        Ok(())
    }

    /// Create [`Client`] from keys.
    pub fn from_keys(keys: KeyPair) -> Self {
        Self {
            keys,
            metadata: Metadata::default(),
            relays: HashMap::new(),
            pool: JoinSet::new(),
        }
    }

    /// Create [`Client`] from secret key.
    pub fn from_secret_key(sk: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::from_secret_key(&secp, &sk);

        Self::from_keys(keys)
    }

    /// Create a listener for all relay messages.
    pub async fn listen(&mut self, buffer: usize) -> Result<Receiver<RelayMessage>> {
        let (client_sender, client_receiver) = channel::<RelayMessage>(buffer);

        for relay in self.relays.values_mut() {
            let client_sender = client_sender.clone();
            let relay_incoming_sender = relay.incoming_sender.as_ref().ok_or_else(|| anyhow!("missing incoming sender"))?;
            let mut relay_incoming_receiver = relay_incoming_sender.subscribe();

            self.pool.spawn(async move {
                while let Ok(message) = relay_incoming_receiver.recv().await {
                    client_sender.send(message)?;
                }

                Err(anyhow!("closed or lagged behind"))
            });
        }

        Ok(client_receiver)
    }

    /// Create [`Client`] from randomly generated keys.
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::new(&secp, &mut rand::thread_rng());

        Self::from_keys(keys)
    }

    /// Send message to relay.
    pub async fn send_message(&mut self, url: RelayUrl, message: ClientMessage) -> Result<()> {
        let relay = self.relays.get_mut(&url).ok_or_else(|| anyhow!("missing relay"))?;
        relay.send(message)?;

        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
