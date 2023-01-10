use async_std::sync::{Arc, Mutex};

use anyhow::Result;
use cyanocitta_nostr_tools::AppData;
use tauri::Manager;

#[tauri::command]
pub async fn get_app_data(handle: tauri::AppHandle) -> Result<String, String> {
    let app_data = handle.state::<Arc<Mutex<AppData>>>();
    let json = serde_json::to_string(&*app_data.lock().await).map_err(|x| x.to_string())?;

    Ok(json)
}
