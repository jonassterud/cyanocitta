use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::AppData;
use tauri::State;

#[tauri::command]
pub fn get_app_data(app_data: State<Mutex<AppData>>) -> Result<String, String> {
    let app_data = &*app_data.lock().map_err(|x| x.to_string())?;
    let json = serde_json::to_string(app_data).map_err(|x| x.to_string())?;

    Ok(json)
}
