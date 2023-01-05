use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::Client;
use tauri::State;

#[tauri::command]
pub fn get_app_data(client: State<Mutex<Client>>) -> Result<String, String> {
    let app_data = &client.lock().map_err(|x| x.to_string())?.app_data;
    let json = serde_json::to_string(app_data).map_err(|x| x.to_string())?;

    Ok(json)
}
