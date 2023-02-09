use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Deserialize, Serialize)]
pub struct State {}

impl State {
    pub fn get_path() -> Result<PathBuf> {
        let mut path = dirs::data_local_dir().ok_or_else(|| anyhow!("missing data local dir"))?;
        path.push("cyanocitta.app/data.json");

        Ok(path)
    }

    pub fn load() -> Result<State> {
        let path = Self::get_path()?;
        let bytes = std::fs::read(path)?;
        let state = serde_json::from_slice(&bytes)?;

        Ok(state)
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
