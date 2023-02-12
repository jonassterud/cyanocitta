mod notifications;

use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ClientState(pub Arc<Mutex<InnerClientState>>);

#[derive(Deserialize, Serialize)]
pub struct InnerClientState {
    /// Bech32 public key.
    pub pk: String,
    /// Bech32 secret key.
    pub sk: String,
    /// Client metadata.
    #[serde(default)]
    pub metadata: Metadata,
    /// Notes cache
    #[serde(default)]
    pub notes: Vec<Event>,
    /// Nostr client.
    #[serde(skip)]
    pub client: Option<Client>,
}

impl ClientState {
    pub async fn initialize_client(&mut self) -> Result<()> {
        let mut inner = self.0.lock().await;
        let pk = XOnlyPublicKey::from_bech32(&inner.pk)?;
        let client = inner
            .client
            .as_mut()
            .ok_or_else(|| anyhow!("missing client"))?;

        println!("{:?}", pk.to_string());
        //client.add_relay("wss://relay.damus.io", None).await?;
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

        let keys = Keys::from_sk_str(&inner_client_state.sk)?;
        let client = Client::new(&keys);
        inner_client_state.client = Some(client);

        Ok(ClientState(Arc::new(Mutex::new(inner_client_state))))
    }

    pub fn new() -> Result<Self> {
        let keys: Keys = Keys::generate();
        let inner_client_state = InnerClientState {
            pk: keys.public_key().to_bech32()?,
            sk: keys.secret_key()?.to_bech32()?,
            metadata: Metadata::new(),
            notes: vec![],
            client: Some(Client::new(&keys)),
        };

        Ok(ClientState(Arc::new(Mutex::new(inner_client_state))))
    }
}

impl Drop for ClientState {
    fn drop(&mut self) {
        let path = Self::get_path().expect("failed getting path");

        let mut dirs = path.clone();
        dirs.pop();
        std::fs::create_dir_all(dirs).expect("failed creating dirs");

        let inner = &*self.0.blocking_lock();
        let contents = serde_json::to_string(inner).expect("failed serializing");
        std::fs::write(&path, contents).expect("failed writing");
    }
}
