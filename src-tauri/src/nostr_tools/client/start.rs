use crate::nostr_tools::Message;
use anyhow::{anyhow, Result};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite};

impl super::Client {
    /// The start function does the following:
    /// * Connect to relays.
    /// * Send messages to relays (async loop).
    /// * Listen for messages from relays (async loop).
    pub fn start(client: Arc<Mutex<Self>>) {
        tokio::spawn(async move {
            let relays = client.lock().await.relays.clone();
            for relay in relays {
                // Connect
                let connection = connect_async(relay).await?.0;
                let (mut write, mut read) = connection.split();

                // Send messages
                let client_1 = client.clone();
                tokio::spawn(async move {
                    loop {
                        if let Some(message) = client_1.lock().await.pool.pop_front() {
                            write.send(tungstenite::Message::Text(message)).await?;
                        } else {
                            // TODO: Needed?
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }

                    #[allow(unreachable_code)]
                    Result::<(), anyhow::Error>::Ok(())
                });

                // Read messages
                let client_2 = client.clone();
                tokio::spawn(async move {
                    loop {
                        if let Some(Ok(message)) = read.next().await {
                            let message = message.to_text()?;
                            let nostr_message = serde_json::from_str::<Message>(message)?;

                            match nostr_message {
                                Message::Event(event) => client_2.lock().await.notes.push(event),
                                Message::Notice(notice) => return Err(anyhow!(notice)),
                                _ => return Err(anyhow!("unexpected message from relay")),
                            }
                        } else {
                            // TODO: Needed?
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }

                    #[allow(unreachable_code)]
                    Result::<(), anyhow::Error>::Ok(())
                });
            }

            Result::<(), anyhow::Error>::Ok(())
        });
    }
}
