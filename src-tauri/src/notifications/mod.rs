use super::ClientState;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use tokio::task::JoinHandle;

pub async fn start_loop(client_state: &ClientState) -> Result<JoinHandle<()>> {
    let temp = client_state.0.clone();
    let inner = temp.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client"))?;
    let mut notifications_receiver = client.notifications();

    let temp = temp.clone();
    let handle = tokio::spawn(async move {
        loop {
            while let Ok(notification) = notifications_receiver.recv().await {
                println!("{:?}\n", notification);

                match notification {
                    RelayPoolNotification::Event(_, event) => match event.kind {
                        Kind::Metadata => {
                            if let Ok(metadata) = serde_json::from_str(&event.content) {
                                let mut inner = temp.lock().await;
                                inner.metadata.insert(event.pubkey.to_string(), metadata);
                            }
                        }
                        Kind::TextNote => {
                            let mut inner = temp.lock().await;
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
                                let mut inner = temp.lock().await;
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
    });

    Ok(handle)
}
