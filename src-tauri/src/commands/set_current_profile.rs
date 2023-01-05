use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::Client;
use tauri::State;

#[tauri::command]
pub fn set_current_profile(index: usize, client: State<Mutex<Client>>) -> Result<(), String> {
    client
        .lock()
        .map_err(|x| x.to_string())?
        .app_data
        .current_profile = index;

    Ok(())
}
