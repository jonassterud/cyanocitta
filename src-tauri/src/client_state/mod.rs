use crate::notifications;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A thread-safe unit struct storing [`InnerClientState`].
pub struct ClientState(pub Arc<Mutex<InnerClientState>>);

/// The inner part of [`ClientState`].
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
    pub metadata: HashMap<String, Metadata>,
    /// Notes
    #[serde(default)]
    pub notes: HashMap<String, Event>,
    /// Nostr client.
    #[serde(skip)]
    pub client: Option<Client>,
}

impl ClientState {
    /// Initializes this [`ClientState`] by:
    /// * Adding the default relays.
    /// * Connecting to relays.
    /// * Adding a default subscription.
    /// * Running [`notifications::start_loop`].
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * `client` in [`InnerClientState`] is `None`.
    /// * [`Client::add_relay`] fails.
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

    /// Get the absolute path for where to store persistent data.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * [`dirs::data_local_dir`] returns `None`.
    fn get_path() -> Result<PathBuf> {
        let mut path = dirs::data_local_dir().ok_or_else(|| anyhow!("missing data local dir"))?;
        path.push("cyanocitta.app/data.json");

        Ok(path)
    }

    /// Create [`ClientState`] from stored data at [`ClientState::get_path`].
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * [`ClientState::get_path`] fails.
    /// * File reading issues.
    /// * `serde_json` deserialization fails.
    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        let bytes = std::fs::read(path)?;
        let mut inner_client_state = serde_json::from_slice::<InnerClientState>(&bytes)?;

        let keys = Keys::new(inner_client_state.sk);
        let client = Client::new(&keys);
        inner_client_state.client = Some(client);

        Ok(ClientState(Arc::new(Mutex::new(inner_client_state))))
    }

    /// Create new [`ClientState`] and save it.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * [`Keys::secret_key`] fails.
    /// * [`InnerClientState::new`] fails.
    /// * [`InnerClientState::save`] fails.
    pub fn new(keys: &Keys) -> Result<Self> {
        let inner_client_state = InnerClientState::new(keys)?;
        inner_client_state.save()?;

        Ok(ClientState(Arc::new(Mutex::new(inner_client_state))))
    }
}

impl InnerClientState {
    /// Create new [`InnerClientState`].
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * [`Keys::secret_key`] fails.
    pub fn new(keys: &Keys) -> Result<Self> {
        Ok(Self {
            pk: keys.public_key(),
            sk: keys.secret_key()?,
            default_relays: vec![
                "wss://relay.nostr.wirednet.jp".to_string(),
                "wss://relay.damus.io".to_string(),
                "wss://relay.nostr.info".to_string(),
                "wss://offchain.pub".to_string(),
                "wss://relay.nostriches.org".to_string(),
                "wss://relay.nostr.org/ws".to_string(),
            ],
            metadata: HashMap::new(),
            notes: HashMap::new(),
            client: Some(Client::new(keys)),
        })
    }

    /// Save current state at [`ClientState::get_path`].
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * [`ClientState::get_path`] fails.
    /// * Failed creating directories.
    /// * Failed writing file.
    /// * `serde_json` serialization fails.
    pub fn save(&self) -> Result<()> {
        let path = ClientState::get_path()?;

        let mut dirs = path.clone();
        dirs.pop();
        std::fs::create_dir_all(dirs)?;

        let contents = serde_json::to_string(self)?;
        std::fs::write(&path, contents)?;

        Ok(())
    }

    /// Create [`InnerClientState`] from secret key.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * [`Keys::from_sk_str`] fails.
    /// * [`Keys::secret_key`] fails
    /// * [`InnerClientState::save`] fails
    pub fn from_sk(sk: &str) -> Result<Self> {
        let keys = Keys::from_sk_str(sk)?;
        let inner_client_state = InnerClientState::new(&keys)?;
        inner_client_state.save()?;

        Ok(inner_client_state)
    }
}
