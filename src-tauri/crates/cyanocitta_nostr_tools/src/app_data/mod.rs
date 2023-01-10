use std::path::PathBuf;

use crate::*;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// AppData.
#[derive(Default, Deserialize, Serialize)]
pub struct AppData {
    /// Information about this user.
    pub profiles: Vec<Profile>,
    /// Current profile index.
    pub current_profile: usize,
    /// List of relays.
    pub relays: Vec<Relay>,
    /// Messages to be sent to relays.
    pub message_pool: Vec<Message>,
}

impl AppData {
    /// Get path to where [`AppData`] is stored.
    fn get_path() -> Result<PathBuf> {
        let mut path =
            dirs::data_local_dir().ok_or_else(|| anyhow!("failed getting local data dir"))?;
        path.push("cyanocitta.app/data.json");

        Ok(path)
    }

    /// Load [`AppData`] from path.
    pub fn load() -> Result<AppData> {
        let path = Self::get_path()?;
        let app_data = serde_json::from_slice(&std::fs::read(&path)?)?;

        Ok(app_data)
    }

    /// Save [`AppData`] to path.
    pub fn save(&self) -> Result<()> {
        let path = Self::get_path()?;
        let mut dir_path = path.clone();
        dir_path.pop();

        std::fs::create_dir_all(&dir_path)?;
        std::fs::write(&path, serde_json::to_string(self)?)?;

        Ok(())
    }

    /// Create [`AppData`] with default relays.
    pub fn new_default_relays() -> Self {
        Self {
            profiles: vec![],
            current_profile: 0,
            relays: vec![Relay {
                id: "wss://relay.damus.io".to_owned(),
                ..Default::default()
            }],
            message_pool: vec![],
        }
    }
}