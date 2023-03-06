//! Client functions for sending messages to relays.

use super::{relay::RelayUrl, Client};
use crate::types::RelayMessage;
use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver};

impl Client {
    pub async fn listen(&mut self, buffer: usize) -> Result<Receiver<RelayMessage>> {
        let (client_sender, client_receiver) = channel::<RelayMessage>(buffer);

        for relay in self.relays.values_mut() {
            let client_sender = client_sender.clone();
            let mut relay_incoming_receiver = relay.incoming_sender.subscribe();
            self.pool.spawn(async move {
                if let Ok(message) = relay_incoming_receiver.recv().await {
                    client_sender.send(message).await?;
                }

                anyhow::Ok(())
            });

            relay.connect_and_listen().await?;
        }

        Ok(client_receiver)
    }
}
