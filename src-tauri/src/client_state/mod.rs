use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct ClientState {
    /// Bech32 public key.
    pub pk: String,
    /// Bech32 secret key.
    pub sk: String,
    /// Client metadata.
    pub metadata: Metadata,
    /// Nostr client.
    #[serde(skip)]
    pub client: Option<Client>,
}

impl ClientState {
    pub async fn initialize_client(&mut self) -> Result<()> {
        let client = self
            .client
            .as_mut()
            .ok_or_else(|| anyhow!("missing client"))?;

        client.add_relay("wss://relay.damus.io", None).await?;
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
        let mut client_state = serde_json::from_slice::<Self>(&bytes)?;

        let keys = Keys::from_pk_str(&client_state.pk)?;
        let client = Client::new(&keys);
        client_state.client = Some(client);

        Ok(client_state)
    }

    pub fn new() -> Result<Self> {
        let keys: Keys = Keys::generate();
        let client_state = Self {
            pk: keys.public_key().to_bech32()?,
            sk: keys.secret_key()?.to_bech32()?,
            metadata: Metadata::new(),
            client: Some(Client::new(&keys)),
        };

        Ok(client_state)
    }
}

impl Drop for ClientState {
    fn drop(&mut self) {
        let path = Self::get_path().expect("failed getting path");

        let mut dirs = path.clone();
        dirs.pop();
        std::fs::create_dir_all(dirs).expect("failed creating dirs");

        let contents = serde_json::to_string(self).expect("failed serializing");
        std::fs::write(&path, contents).expect("failed writing");
    }
}
