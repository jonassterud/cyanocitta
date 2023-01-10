use async_std::sync::{Arc, Mutex};
use anyhow::Result;
use cyanocitta_nostr_tools::AppData;
use tauri::Manager;

#[tauri::command]
pub async fn set_current_profile(index: usize, handle: tauri::AppHandle) -> Result<(), String> {
    let app_data = handle.state::<Arc<Mutex<AppData>>>();
    app_data.lock().await.current_profile = index;

    Ok(())
}
