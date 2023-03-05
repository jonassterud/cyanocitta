//! Client functions for sending messages to relays.

use crate::types::ClientMessage;
use anyhow::{anyhow, Result};
use super::{Client, relay::RelayUrl};

impl Client {
    pub fn send_message(&mut self, url: RelayUrl, message: ClientMessage) -> Result<()> {
        if let Some(relay) = self.relays.get_mut(&url) {
            relay.send_pool.push_back(message);

            Ok(())
        } else {
            Err(anyhow!("did not find relay"))
        }
    }

    pub fn broadcast_message(&mut self, message: ClientMessage) {
        for relay in self.relays.values_mut() {
            relay.send_pool.push_back(message.clone());
        }
    }
}