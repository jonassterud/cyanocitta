use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::AppData;
use tauri::State;

#[tauri::command]
pub fn set_current_profile(index: usize, app_data: State<Mutex<AppData>>) -> Result<(), String> {
    app_data.lock().map_err(|x| x.to_string())?.current_profile = index;

    Ok(())
}
