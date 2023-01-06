use std::path::PathBuf;

use crate::*;
use anyhow::{anyhow, Result};
use async_std::sync::{Arc, Mutex};
use async_tungstenite::{
    async_std::{connect_async, ConnectStream},
    tungstenite::Message as WebSocketMessage,
    WebSocketStream,
};
use futures::{future::join_all, SinkExt, StreamExt};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};

/// Client.
#[derive(Default)]
pub struct Client {
    /// App data.
    pub app_data: AppData,
    /// List of active connections.
    pub connections: Vec<WebSocketStream<ConnectStream>>,
}

/// AppData.
#[derive(Default, Deserialize, Serialize)]
pub struct AppData {
    /// Information about this user.
    pub profiles: Vec<Profile>,
    /// Current profile index.
    pub current_profile: usize,
    /// List of relays.
    pub relays: Vec<Relay>,
}

impl Client {
    /// Try to load existing [`Client`].
    ///
    /// # Arguments
    ///
    /// * `fallback` - fallback function if failing to load.
    pub fn load(fallback: &dyn Fn() -> Self) -> Result<Self> {
        if let Some(mut data_path) = dirs::data_local_dir() {
            data_path.push("cyanocitta.app/data.json");

            let client = if std::path::Path::exists(&data_path) {
                let app_data: AppData = serde_json::from_slice(&std::fs::read(&data_path)?)?;

                Client {
                    app_data,
                    ..Default::default()
                }
            } else {
                let mut data_dir_path = data_path.clone();
                data_dir_path.pop();

                std::fs::create_dir_all(&data_dir_path)?;
                let fallback_client = fallback();
                std::fs::write(
                    &data_path,
                    serde_json::to_string(&fallback_client.app_data)?,
                )?;

                fallback_client
            };

            Ok(client)
        } else {
            Err(anyhow!("failed getting local data dir"))
        }
    }

    /// Create [`Client`] with default relays.
    pub fn new_with_default_relays() -> Self {
        Self {
            app_data: AppData {
                relays: vec![Relay {
                    id: "wss://relay.damus.io".to_owned(),
                    ..Default::default()
                }],
                profiles: vec![],
                current_profile: 0,
            },
            connections: vec![],
        }
    }

    /// Connect to relays.
    pub async fn connect_to_relays(&mut self) -> Result<()> {
        for relay in &mut self.app_data.relays {
            self.connections
                .push(connect_async(relay.id.to_owned()).await?.0);
            *relay = Relay::new(&relay.id)?;
        }

        Ok(())
    }

    /// Send [`Message`].
    pub async fn send_message(&mut self, message: Message) -> Result<()> {
        for connection in &mut self.connections {
            connection
                .send(WebSocketMessage::Text(message.serialize()))
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
                    let message = Message::deserialize(&json).expect("failed reading message");

                    if matches!(message, Message::Notice(..)) {
                        println!("{:?}", message);
                    } else {
                        out.lock().await.push(message);
                    }
                }
            }));
        }

        join_all(handles).await;

        Ok(())
    }
}

impl AppData {
    pub fn save(&self) -> Result<()> {
        let mut path = dirs::data_local_dir().ok_or_else(|| anyhow!("failed getting local data dir"))?;

        path.push("cyanocitta.app");
        std::fs::create_dir_all(&path)?;

        path.push("data.json");
        std::fs::write(&path, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Drop for AppData {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}