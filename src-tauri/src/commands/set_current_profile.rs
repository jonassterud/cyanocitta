use std::sync::{Arc, Mutex};
use anyhow::Result;
use cyanocitta_nostr_tools::AppData;
use tauri::Manager;

#[tauri::command]
pub async fn set_current_profile(index: usize, handle: tauri::AppHandle) -> Result<(), String> {
    let app_data = handle.state::<Arc<Mutex<AppData>>>().clone();
    let mut app_data = app_data.lock().map_err(|x| x.to_string())?;
    app_data.current_profile = index;
    app_data.save().map_err(|x| x.to_string())?;

    Ok(())
}
