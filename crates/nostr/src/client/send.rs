//! Client functions for sending messages to relays.

use super::{relay::RelayUrl, Client};
use crate::types::ClientMessage;
use anyhow::{anyhow, Result};

impl Client {
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
