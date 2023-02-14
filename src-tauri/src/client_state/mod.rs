mod notifications;

use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ClientState(pub Arc<Mutex<InnerClientState>>);

#[derive(Deserialize, Serialize)]
pub struct InnerClientState {
    /// Public key
    pub pk: XOnlyPublicKey,
    /// Secret key.
    pub sk: SecretKey,
    /// Metadata
    #[serde(default)]
    pub metadata: HashMap<String, Metadata>,
    /// Nostr client.
    #[serde(skip)]
    pub client: Option<Client>,
}

impl ClientState {
    pub async fn initialize_client(&mut self) -> Result<()> {
        let mut inner = self.0.lock().await;
        let pk = inner.pk.clone();
        let client = inner
            .client
            .as_mut()
            .ok_or_else(|| anyhow!("missing client"))?;

        client
            .add_relay("wss://relay.nostr.wirednet.jp", None)
            .await?;
        client.add_relay("wss://relay.damus.io", None).await?;
        client.add_relay("wss://relay.nostr.info/", None).await?;
        client
            .subscribe(vec![SubscriptionFilter::new().author(pk).limit(5000)])
            .await;
        client.connect().await;

        Ok(())
    }

    fn get_path() -> Result<PathBuf> {
        let mut path = dirs::data_local_dir().ok_or_else(|| anyhow!("missing data local dir"))?;
        path.push("cyanocitta.app/data.json");

        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        let bytes = std::fs::read(path)?;
        let mut inner_client_state = serde_json::from_slice::<InnerClientState>(&bytes)?;

        let keys = Keys::new(inner_client_state.sk);
        let client = Client::new(&keys);
        inner_client_state.client = Some(client);

        Ok(ClientState(Arc::new(Mutex::new(inner_client_state))))
    }

    pub fn new() -> Result<Self> {
        let keys: Keys = Keys::generate();
        let inner_client_state = InnerClientState {
            pk: keys.public_key(),
            sk: keys.secret_key()?,
            metadata: HashMap::new(),
            client: Some(Client::new(&keys)),
        };
        inner_client_state.save()?;

        Ok(ClientState(Arc::new(Mutex::new(inner_client_state))))
    }
}

impl InnerClientState {
    pub fn save(&self) -> Result<()> {
        let path = ClientState::get_path()?;

        let mut dirs = path.clone();
        dirs.pop();
        std::fs::create_dir_all(dirs)?;

        let contents = serde_json::to_string(self)?;
        std::fs::write(&path, contents)?;

        Ok(())
    }
}
