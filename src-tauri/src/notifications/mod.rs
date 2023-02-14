use super::{ClientState, InnerClientState};
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use tokio::task::JoinHandle;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn insert_metadata(inner: Arc<Mutex<InnerClientState>>, event: Event) -> Result<()> {
    let mut inner = inner.lock().await;
    let metadata = serde_json::from_str::<Metadata>(&event.content)?;

    inner.metadata.insert(event.pubkey.to_string(), metadata);

    Ok(())
}

pub async fn start_loop(client_state: &ClientState) -> Result<JoinHandle<()>> {
    let temp = client_state.0.clone();
    let inner = temp.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client"))?;
    let mut notifications_receiver = client.notifications();

    let temp = client_state.0.clone();
    let handle = tokio::spawn(async move {
        loop {
            while let Ok(notification) = notifications_receiver.recv().await {
                println!("{:?}\n", notification);

                match notification {
                    RelayPoolNotification::Event(_, event) => {
                        match event.kind {
                            Kind::Metadata => {
                                insert_metadata(temp.clone(), event).await;
                            },
                            Kind::TextNote => {
                                let mut inner = temp.lock().await;
                                inner.notes.insert(event.id.to_hex(), event);
                            },
                            _ => {}
                        }
                    },
                    RelayPoolNotification::Message(_, message) => {
                        match message {
                            RelayMessage::Event { subscription_id: _, event } => {
                                insert_metadata(temp.clone(), *event).await;
                            },
                            RelayMessage::Notice { message } => {},
                            RelayMessage::EndOfStoredEvents(_) => {},
                            RelayMessage::Ok { event_id, status, message } => {},
                            RelayMessage::Auth { challenge } => {},
                            RelayMessage::Empty => {},
                        }
                    },
                    RelayPoolNotification::Shutdown => {},
                }
            }
        }
    });

    Ok(handle)
}
