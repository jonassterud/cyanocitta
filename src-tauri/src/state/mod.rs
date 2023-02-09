use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use nostr_sdk::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct State {
    /// Bech32 public key.
    pub pk: String,
    /// Bech32 secret key.
    pub sk: String,
    /// Nostr client.
    #[serde(skip)]
    client: Option<Client>,
}

impl State {
    pub fn get_path() -> Result<PathBuf> {
        let mut path = dirs::data_local_dir().ok_or_else(|| anyhow!("missing data local dir"))?;
        path.push("cyanocitta.app/data.json");

        Ok(path)
    }

    pub fn load() -> Result<State> {
        let path = Self::get_path()?;
        let bytes = std::fs::read(path)?;
        let mut state = serde_json::from_slice::<State>(&bytes)?;

        let keys = Keys::from_pk_str(&state.pk)?;
        state.client = Some(Client::new(&keys));

        Ok(state)
    }

    pub async fn new() -> Result<State> {
        let keys: Keys = Keys::generate();
        let pk = keys.public_key().to_bech32()?;
        let sk = keys.secret_key()?.to_bech32()?;
        let client = Client::new(&keys);

        client.add_relay("wss://relay.damus.io", None).await?;

        Ok(State {
            pk,
            sk,
            client: Some(client),
        })
    }
}

impl Drop for State {
    fn drop(&mut self) {
        let path = Self::get_path().expect("failed getting path");

        let mut dirs = path.clone();
        dirs.pop();
        std::fs::create_dir_all(dirs).expect("failed creating dirs");

        let contents = serde_json::to_string(self).expect("failed serializing");
        std::fs::write(&path, contents).expect("failed writing");
    }
}
