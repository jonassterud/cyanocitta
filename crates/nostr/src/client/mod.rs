//! Nostr client.

mod relay;

pub use relay::{Relay, RelayUrl};

use crate::types::{ClientMessage, RelayMessage};
use anyhow::{anyhow, Result};
use secp256k1::{rand, KeyPair, Secp256k1, SecretKey};
use std::collections::HashMap;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinSet;

/// Nostr client to interact with relays.
pub struct Client {
    pub keys: KeyPair,
    pub relays: HashMap<RelayUrl, Relay>,
    pool: JoinSet<Result<()>>,
}

impl Client {
    /// Create [`Client`] from randomly generated keys.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::new(&secp, &mut rand::thread_rng());

        Self::from_keys(keys)
    }

    /// Create [`Client`] from secret key.
    pub fn from_secret_key(sk: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::from_secret_key(&secp, &sk);

        Self::from_keys(keys)
    }

    /// Create [`Client`] from keys.
    pub fn from_keys(keys: KeyPair) -> Self {
        Self { keys, relays: HashMap::new(), pool: JoinSet::new() }
    }

    /// Add relay to client.
    pub fn add_relay(&mut self, relay: Relay) {
        self.relays.insert(relay.url.clone(), relay);
    }

    /// Create a listener for all relay messages.
    pub async fn listen(&mut self, buffer: usize) -> Result<Receiver<RelayMessage>> {
        let (client_sender, client_receiver) = channel::<RelayMessage>(buffer);

        for relay in self.relays.values_mut() {
            let client_sender = client_sender.clone();
            let mut relay_incoming_receiver = relay.incoming_sender.subscribe();
            self.pool.spawn(async move {
                while let Ok(message) = relay_incoming_receiver.recv().await {
                    client_sender.send(message)?;
                }

                Err(anyhow!("closed or lagged behind"))
            });
        }

        Ok(client_receiver)
    }

    /// Send message to relay.
    pub async fn send_message(&mut self, url: RelayUrl, message: ClientMessage) -> Result<()> {
        let relay = self.relays.get_mut(&url).ok_or_else(|| anyhow!("missing relay"))?;
        relay.send(message)?;

        Ok(())
    }

    /// Broadcast message to all relays.
    pub async fn broadcast_message(&mut self, message: ClientMessage) -> Result<()> {
        for relay in self.relays.values_mut() {
            relay.send(message.clone())?;
        }

        Ok(())
    }
}
