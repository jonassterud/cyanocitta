use crate::client_state::ClientState;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use std::collections::HashMap;
use tokio::{sync::broadcast::Receiver, task::JoinHandle};

/// Spawns a tokio task that listens for and handles relay notifications.
///
/// # Errors
///
/// The `JoinHandle` will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
pub async fn start_loop(client_state: &ClientState) -> JoinHandle<Result<()>> {
    let client_state = client_state.clone();
    let handle = tokio::spawn(async move {
        let mut notifications_receiver: Receiver<RelayPoolNotification> = {
            let temp = client_state.0.lock().await;
            let client = temp
                .client
                .as_ref()
                .ok_or_else(|| anyhow!("missing client"))?;

            anyhow::Ok(client.notifications())
        }?;

        while let Ok(notification) = notifications_receiver.recv().await {
            //println!("{:?}", notification);

            if let RelayPoolNotification::Message(_, message) = notification {
                match message {
                    RelayMessage::Event {
                        subscription_id,
                        event,
                    } => {
                        handle_event(&client_state, &subscription_id, *event).await;
                    }
                    RelayMessage::Notice { message } => {
                        eprintln!("Notice: {}", message);
                    }
                    _ => {}
                };
            }
        }

        Ok(())
    });

    handle
}

async fn handle_event(client_state: &ClientState, subscription_id: &SubscriptionId, event: Event) {
    println!("{:?} => {}", subscription_id, event.content);

    match event.kind {
        Kind::Metadata => {
            if let Ok(metadata) = serde_json::from_str(&event.content) {
                let mut inner = client_state.0.lock().await;
                inner.metadata.insert(event.pubkey.to_string(), metadata);
            }
        }
        Kind::TextNote => {
            let mut inner = client_state.0.lock().await;
            if let Some(map) = inner.notes.get_mut(&subscription_id.to_string()) {
                map.insert(event.id.to_string(), event);
            } else {
                let mut inner_map = HashMap::new();
                inner_map.insert(event.id.to_string(), event);
                inner.notes.insert(subscription_id.to_string(), inner_map);
            }
        }
        _ => {}
    }
}
