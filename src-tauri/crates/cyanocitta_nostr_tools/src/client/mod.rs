use crate::*;
use anyhow::Result;
use async_std::sync::{Arc, Mutex};
use async_tungstenite::{
    async_std::{connect_async, ConnectStream},
    tungstenite::Message as WebSocketMessage,
    WebSocketStream,
};
use futures::{future::join_all, SinkExt, StreamExt};
use secp256k1::SecretKey;

/// Client.
pub struct Client {
    pub profile: Profile,
    pub relays: Vec<String>,
    pub connections: Vec<WebSocketStream<ConnectStream>>,
}

impl Client {
    /// Create [`Client`].
    pub fn new(secret_key: Option<SecretKey>, relays: Option<Vec<String>>) -> Self {
        let profile = secret_key.map_or(Profile::new_with_random_keypair(), |sk| {
            Profile::from_secret_key(sk)
        });
        let relays = relays.unwrap_or(vec!["wss://relay.damus.io".to_owned()]);

        Self {
            profile,
            relays,
            connections: vec![],
        }
    }

    /// Connect to relays.
    pub async fn connect_to_relays(&mut self) -> Result<()> {
        for relay in &self.relays {
            self.connections.push(connect_async(relay).await?.0);
        }

        Ok(())
    }

    /// Send [`Message`].
    pub async fn send_message(&mut self, message: Message) -> Result<()> {
        for connection in &mut self.connections {
            connection
                .send(WebSocketMessage::Text(message.serialize()?))
                .await?;
        }

        Ok(())
    }

    /// Listen for [`Message`] on all websockets, and send them to the `sender`.
    ///
    /// # Arguments
    ///
    /// * `sender` - [`channel::Sender`].
    pub async fn get_message(self, out: Arc<Mutex<Vec<Message>>>) -> Result<()> {
        let mut handles = vec![];

        for connection in self.connections {
            let (_, mut read) = connection.split();
            let out = out.clone();

            handles.push(async_std::task::spawn(async move {
                while let Some(Ok(data)) = read.next().await {
                    let json = data.into_text().expect("expected text");
                    out.lock()
                        .await
                        .push(Message::from_relay(json).expect("failed reading message"));
                }
            }));
        }

        join_all(handles).await;

        Ok(())
    }
}
