use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tauri::api::path;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct AppState(Arc<Mutex<AppStateData>>);

#[derive(Default, Deserialize, Serialize)]
pub struct AppStateData {}

impl AppState {
    /// Create [`AppState`] from [`AppStateData`].
    pub fn from_data(data: AppStateData) -> Self {
        Self(Arc::new(Mutex::new(data)))
    }

    /// Get lock on inner [`AppStateData`].
    pub async fn inner(&self) -> MutexGuard<AppStateData> {
        self.0.lock().await
    }

    /// Get directory for where to save [`AppStateData`].
    fn local_dir_path() -> Result<PathBuf> {
        let mut path = path::local_data_dir().ok_or_else(|| anyhow!("failed finding local dir"))?;
        path.push("cyanocitta/");

        Ok(path)
    }

    /// Get file path for where to save [`AppStateData`].
    fn local_file_path() -> Result<PathBuf> {
        let mut path = Self::local_dir_path()?;
        path.push("app_state.json");

        Ok(path)
    }

    /// Create [`AppState`].
    pub fn new() -> Self {
        if let Ok(app_state) = Self::try_load() {
            app_state
        } else {
            Self::from_data(AppStateData::default())
        }
    }

    /// Try creating [`AppState`] from local storage data.
    pub fn try_load() -> Result<Self> {
        let path = Self::local_file_path()?;
        let bytes = std::fs::read(path)?;
        let data = serde_json::from_slice(&bytes)?;

        Ok(Self::from_data(data))
    }

    /// Try saving [`AppStateData`] to local storage data.
    pub async fn try_save(&self) -> Result<()> {
        let dir_path = Self::local_dir_path()?;
        let file_path = Self::local_file_path()?;
        let data = serde_json::to_string(&*self.inner().await)?;

        std::fs::create_dir_all(dir_path)?;
        std::fs::write(file_path, data)?;

        Ok(())
    }
}
