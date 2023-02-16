use crate::notifications;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ClientState(pub Arc<Mutex<InnerClientState>>);

#[derive(Deserialize, Serialize)]
pub struct InnerClientState {
    /// Public key
    pub pk: XOnlyPublicKey,
    /// Secret key.
    pub sk: SecretKey,
    /// Default relays
    #[serde(default)]
    pub default_relays: Vec<String>,
    /// Metadata
    #[serde(default)]
    pub metadata: BTreeMap<String, Metadata>,
    /// Notes
    #[serde(default)]
    pub notes: BTreeMap<String, Event>,
    /// Nostr client.
    #[serde(skip)]
    pub client: Option<Client>,
}

impl ClientState {
    pub async fn initialize_client(&self) -> Result<()> {
        let inner = self.0.lock().await;
        let client = inner
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("missing client"))?;

        for relay_url in &inner.default_relays {
            client.add_relay(relay_url, None).await?;
        }

        client.connect().await;
        client
            .subscribe(vec![SubscriptionFilter::new().author(inner.pk).limit(5000)])
            .await;

        notifications::start_loop(&self).await;

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
        let keys = Keys::generate();
        let inner_client_state = InnerClientState {
            pk: keys.public_key(),
            sk: keys.secret_key()?,
            default_relays: vec![
                String::from_str("wss://relay.nostr.wirednet.jp")?,
                String::from_str("wss://relay.damus.io")?,
                String::from_str("wss://relay.nostr.info")?,
                String::from_str("wss://offchain.pub")?,
                String::from_str("wss://relay.nostriches.org")?,
                String::from_str("wss://relay.nostr.org/ws")?,
            ],
            metadata: BTreeMap::new(),
            notes: BTreeMap::new(),
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

    pub fn from_sk(sk: &str) -> Result<Self> {
        let keys = Keys::from_sk_str(sk)?;
        let inner_client_state = InnerClientState {
            pk: keys.public_key(),
            sk: keys.secret_key()?,
            default_relays: vec![
                String::from_str("wss://relay.nostr.wirednet.jp")?,
                String::from_str("wss://relay.damus.io")?,
                String::from_str("wss://relay.nostr.info")?,
                String::from_str("wss://offchain.pub")?,
                String::from_str("wss://relay.nostriches.org")?,
                String::from_str("wss://relay.nostr.org/ws")?,
            ],
            metadata: BTreeMap::new(),
            notes: BTreeMap::new(),
            client: Some(Client::new(&keys)),
        };
        inner_client_state.save()?;

        Ok(inner_client_state)
    }
}
