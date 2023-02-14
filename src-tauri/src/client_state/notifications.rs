use super::ClientState;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use tokio::task::JoinHandle;

impl ClientState {
    pub async fn start_notifications_loop(&self) -> Result<JoinHandle<()>> {
        let inner = self.0.lock().await;
        let client = inner
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("missing client"))?;
        let mut notifications_receiver = client.notifications();

        let client_state_clone = self.0.clone();
        let handle = tokio::spawn(async move {
            loop {
                while let Ok(notification) = notifications_receiver.recv().await {
                    if let RelayPoolNotification::Event(_, event) = notification {
                        match event.kind {
                            Kind::Metadata => {
                                if let Ok(metadata) =
                                    serde_json::from_str::<Metadata>(&event.content)
                                {
                                    let mut inner = client_state_clone.lock().await;
                                    inner.metadata.insert(event.pubkey.to_string(), metadata);
                                } else {
                                    eprintln!("failed deserializing recieved metadata");
                                }
                            },
                            Kind::TextNote => {
                                let mut inner = client_state_clone.lock().await;
                                inner.notes.insert(event.id.to_hex(), event);
                            },
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(handle)
    }
}
