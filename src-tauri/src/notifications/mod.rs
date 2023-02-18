use crate::client_state::ClientState;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use tokio::{sync::broadcast::Receiver, task::JoinHandle};

/// Spawns a tokio task that listens for and handles relay notifications.
///
/// # Errors
///
/// The `JoinHandle` will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
pub async fn start_loop(client_state: &ClientState) -> JoinHandle<Result<()>> {
    let client_state_unit = client_state.0.clone();
    let handle = tokio::spawn(async move {
        let mut notifications_receiver: Receiver<RelayPoolNotification> = {
            let temp = client_state_unit.lock().await;
            let client = temp
                .client
                .as_ref()
                .ok_or_else(|| anyhow!("missing client"))?;

            anyhow::Ok(client.notifications())
        }?;

        loop {
            while let Ok(notification) = notifications_receiver.recv().await {
                println!("{:?}", notification);
                match notification {
                    RelayPoolNotification::Event(_, event) => match event.kind {
                        Kind::Metadata => {
                            if let Ok(metadata) = serde_json::from_str(&event.content) {
                                let mut inner = client_state_unit.lock().await;
                                inner.metadata.insert(event.pubkey.to_string(), metadata);
                            }
                        }
                        Kind::TextNote => {
                            let mut inner = client_state_unit.lock().await;
                            inner.notes.insert(event.id.to_hex(), event);
                        }
                        _ => {}
                    },
                    RelayPoolNotification::Message(_, message) => match message {
                        RelayMessage::Event {
                            subscription_id: _,
                            event,
                        } => {
                            if let Ok(metadata) = serde_json::from_str(&event.content) {
                                let mut inner = client_state_unit.lock().await;
                                inner.metadata.insert(event.pubkey.to_string(), metadata);
                            }
                        }
                        RelayMessage::Notice { message } => {}
                        RelayMessage::EndOfStoredEvents(_) => {}
                        RelayMessage::Ok {
                            event_id,
                            status,
                            message,
                        } => {}
                        RelayMessage::Auth { challenge } => {}
                        RelayMessage::Empty => {}
                    },
                    RelayPoolNotification::Shutdown => {}
                }
            }
        }

        #[allow(unreachable_code)]
        Ok(())
    });

    handle
}
