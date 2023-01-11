mod start;

use anyhow::{anyhow, Result};
use dirs_next::data_local_dir;
use secp256k1::{rand::rngs::OsRng, Secp256k1};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;

/// A Nostr client.
#[derive(Deserialize, Serialize)]
pub struct Client {
    /// Public key of the user associated with this client.
    pub pubkey: Vec<u8>,
    /// Secret key of the user associated with this client.
    pub seckey: Vec<u8>,
    /// JSON messages to be sent to relays.
    pub pool: VecDeque<String>,
    /// Messages recieved from relays.
    pub notes: Vec<super::message::Event>,
    /// Relays.
    pub relays: Vec<String>,
}

impl Client {
    /// Create [`Client`] with random keypair.
    pub fn new_random() -> Arc<Mutex<Self>> {
        let secp = Secp256k1::new();
        let (seckey, pubkey) = secp.generate_keypair(&mut OsRng);

        Arc::new(Mutex::new(Self {
            pubkey: pubkey.serialize().to_vec(),
            seckey: seckey.secret_bytes().to_vec(),
            pool: VecDeque::new(),
            notes: vec![],
            relays: vec!["wss://relay.damus.io".to_string()],
        }))
    }

    /// Create [`Client`] from local storage.
    pub fn load() -> Result<Arc<Mutex<Self>>> {
        let mut path = data_local_dir().ok_or_else(|| anyhow!("failed getting local dir"))?;
        path.push("cyanocitta.app/data.json");

        let client = serde_json::from_slice(&std::fs::read(&path)?)?;
        Ok(Arc::new(Mutex::new(client)))
    }

    /// Save [`Client`] to local storage.
    pub fn save(&self) -> Result<()> {
        let mut path = data_local_dir().ok_or_else(|| anyhow!("failed getting local dir"))?;
        path.push("cyanocitta.app/");

        if !std::path::Path::exists(&path) {
            std::fs::create_dir_all(&path)?;
        }

        path.push("data.json");
        std::fs::write(&path, serde_json::to_string(self)?)?;

        Ok(())
    }
}
