use crate::nostr_tools::*;
use anyhow::Result;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn notes(handle: tauri::AppHandle) -> Result<String, String> {
    let state = handle.state::<Arc<Mutex<Client>>>();
    let notes = &state.lock().await.notes;
    let notes_json = serde_json::to_string(notes).map_err(|x| x.to_string())?;

    Ok(notes_json)
}
